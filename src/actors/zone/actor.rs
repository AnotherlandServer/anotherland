// Copyright (C) 2024 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{sync::Arc, time::{Duration, Instant}};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{ oaPktServerAction, raknet::Message, AvatarId, NonClientBaseComponent, NonClientBaseParams, NpcOtherlandAttribute, NpcOtherlandClass, NpcOtherlandComponent, OaZoneConfigParams, ParamBox, ParamClass, ParamSet, ParamSetBox, PlayerAttribute, PlayerClass, PlayerParams, PortalClass, PortalParams, SpawnNodeClass, StartingPointClass, Uuid};
use bevy::{app::{App, Update}, utils::HashMap, MinimalPlugins};
use glam::{Vec3, Quat};
use log::{debug, info, warn};
use mongodb::Database;
use tokio::{runtime::Handle, select, sync::{broadcast, mpsc, OnceCell}, task::JoinHandle, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use crate::{actors::{get_player_height, zone::{behavior::AvatarBehaviorPlugin, behaviors::BehaviorsPlugin, resources::{EventInfo, EventInfos}, systems::{respawn, send_messages, send_proximity_chat, sepcial_event_controller, surf_spline, update_interests}}, Spawned}, cluster::{actor::Actor, ActorRef}, components::{SpecialEvents, ZoneFactory}, db::{Character, FlightTube}, util::{AnotherlandError, AnotherlandResult, OtherlandQuatExt}};
use crate::db::DatabaseRecord;

use super::{behavior::BehaviorExt, components::{AvatarComponent, AvatarEvent, EntityType, InterestList, Position}, resources::{Broadcaster, Tasks}, zone_events::{AvatarEventFired, ProximityChatEvent}, AvatarEventSender, Movement, PlayerSpawnMode, ProximityChatRange, SpawnerState, ZoneEvent};

pub enum ServerAction {
    DirectTravel(AvatarId, Option<Position>),
    NonPortalTravel(AvatarId, Option<Position>),
    Teleport(AvatarId, Position)
}

impl ServerAction {
    pub fn into_message(self) -> Message {
        let (instigator, action, version, teleport_override) = match self {
            Self::DirectTravel(instigator, teleport_override) => (
                instigator,
                "TRAVEL:DirectTravel|DirectTravelDefault".to_owned(),
                4,
                teleport_override
            ),
            Self::NonPortalTravel(instigator, teleport_override) => (
                instigator,
                "TRAVEL:NonPortalTravel|NonPortalTravelDefault".to_owned(),
                4,
                teleport_override
            ),
            Self::Teleport(instigator, position) => (
                instigator,
                "TELEPORT:TeleportTravel|TeleportTravelDefault".to_owned(),
                4,
                Some(position)
            ),
        };

        if let Some(teleport_override) = teleport_override {
            oaPktServerAction {
                instigator: instigator.as_u64(),
                action,
                version,
                override_teleport: true,
                pos: teleport_override.position.into(),
                rot: teleport_override.rotation.into(),
                ..Default::default()
            }.into_message()
        } else {
            oaPktServerAction {
                instigator: instigator.as_u64(),
                action,
                version,
                ..Default::default()
            }.into_message()
        }
    }
}

pub(super) static SPECIAL_EVENTS: OnceCell<SpecialEvents> = OnceCell::const_new();
pub(in crate::actors::zone) static FLIGHT_TUBES: OnceCell<HashMap<Uuid, Arc<FlightTube>>> = OnceCell::const_new();

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone)]
struct SlowUpdate;

#[derive(Resource)]
pub struct DefaultPos {
    pub pos: Vec3,
    pub rot: Vec3,
}

pub struct Zone {
    pub(super) realm_db: Database,
    pub(super) factory: ZoneFactory,

    default_pos: Vec3,
    default_rot: Vec3,

    pub(super) app: App,

    cancellation_token: CancellationToken,
    update_task: Option<JoinHandle<()>>,

    event_sender: broadcast::Sender<Arc<ZoneEvent>>,
    avatar_id_to_entity_lookup: HashMap<AvatarId, Entity>,
    pub(super) uuid_to_entity_lookup: HashMap<Uuid, Entity>,
}

impl Zone {
    pub fn initialize(factory: ZoneFactory) -> Self {
        Self {
            realm_db: factory.db().clone(),
            factory,
            default_pos: Vec3::default(),
            default_rot: Vec3::default(),
            app: App::new(),
            cancellation_token: CancellationToken::new(),
            update_task: None,
            event_sender: broadcast::channel(500).0,
            avatar_id_to_entity_lookup: HashMap::new(),
            uuid_to_entity_lookup: HashMap::new(),
        }
    }
}

// Trust me, Zone is Send + Sync!
unsafe impl Send for Zone {}
unsafe impl Sync for Zone {}

#[async_trait]
impl Actor for Zone {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { None }

    async fn starting(&mut self) -> AnotherlandResult<()> {
        // load special event config
        let special_events = SPECIAL_EVENTS.get_or_try_init(SpecialEvents::load).await.unwrap();
        FLIGHT_TUBES.get_or_try_init(Zone::load_flight_tubes).await.unwrap();

        // setup bevy app
        self.app
            .add_plugins(MinimalPlugins)
            .add_plugins(AvatarBehaviorPlugin)
            .add_plugins(BehaviorsPlugin)
            .add_plugins(SubjectivityPlugin)
            .add_plugins(SubjectiveLensesPlugin)
            .add_systems(Update, (
                send_proximity_chat,
                update_interests,
                send_messages,
                surf_spline,
            ))
            .add_systems(SlowUpdate, (
                respawn,
                sepcial_event_controller,
            ))
            .add_event::<ProximityChatEvent>()
            .add_event::<AvatarEventFired>()
            .insert_resource(Broadcaster {
                sender: self.event_sender.clone()
            })
            .insert_resource(Tasks {
                handle: Handle::current(),
                tasks: TaskTracker::new(),
            })
            .insert_resource(EventInfos(
                special_events.get_events_for_map(&self.factory.world_def().name).await?
                .into_iter()
                .map(|v| EventInfo {
                    event: v,
                    active: None
                })
                .collect::<Vec<_>>()
            ))
            .insert_resource(DefaultPos {
                pos: self.default_pos,
                rot: self.default_rot,
            });

        // load in content
        self.load_content_instances().await?;

        // lookup starting point
        {
            let mut query = self.app.world.query::<&StartingPointClass>();
            if let Some(entry_point) = query.iter(&self.app.world).next() {
                debug!("Found entrypoint");

                self.default_pos = entry_point.pos().to_owned();
                self.default_rot = entry_point.rot().to_owned();
            }
        }

        info!("Spawned zone {}...", self.factory.zone_def().guid);

        Ok(()) 
    }

    async fn started(&mut self, mut handle: ActorRef<Self>) -> AnotherlandResult<()> { 
        let token = self.cancellation_token.clone();
        self.update_task = Some(tokio::spawn(async move {
            let mut update_interval = time::interval(Duration::from_millis(40)); // Aim for 25 cycles/sec
            let mut respawn_interval = time::interval(Duration::from_secs(1));
            loop {
                select! {
                    _ = update_interval.tick() => handle.fast_update().await,
                    _ = respawn_interval.tick() => handle.slow_update().await,
                    _ = token.cancelled() => break,
                }
            }
        }));

        Ok(()) 
    }

    async fn stopping(&mut self) -> AnotherlandResult<()> { 
        self.cancellation_token.cancel();

        if let Some(update_task) = self.update_task.as_mut() {
            let _ = update_task.await;
        }

        Ok(()) 
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> {
        Ok(())
    }
}

impl Zone {
    pub(super) fn spawn_non_player_avatar<T>(
        &mut self, 
        avatar_id: AvatarId, 
        entity_type: EntityType, 
        name: &str, 
        phase_tag: &str, 
        id: Uuid, 
        content_id: Uuid, 
        entity_params: T
    ) -> EntityWorldMut<'_> 
        where T: ParamClass + Clone + ?Sized
    {
        // spawn entity
        let entity = self.app.world
            .spawn(entity_params.as_bundle())
            .insert(entity_type)
            .insert(AvatarComponent {
                id: avatar_id,
                instance_id: Some(id),
                content_id: Some(content_id),
                name: name.to_owned(),
                phase_tag: phase_tag.to_owned(),
            })
            .id();

        let mut entity_ref = self.app.world.get_entity_mut(entity).unwrap();

        // insert position component for npcs & structures
        if let Some(base) = entity_ref.get::<NonClientBaseComponent>() {
            let position = base.pos().to_owned();
            let rotation = base.rot().to_owned();

            entity_ref
                .insert(SpawnerState { 
                    despawn_instant: None, 
                    respawn_instant: None 
                })
                .insert(Position {
                    mover_key: 0,
                    replica: 7,
                    version: 1,
                    position,
                    rotation: Quat::from_unit_vector(rotation),
                    velocity: Vec3::default(),
                });
        } else {
            // assume the entity is always spawned
            entity_ref.insert(Spawned);

            // notify clients
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
                avatar_id, 
                params: entity_params.clone().into_box(),
            }));
        }

        // update lookup map
        self.avatar_id_to_entity_lookup.insert(avatar_id, entity);
        self.uuid_to_entity_lookup.insert(id, entity);

        entity_ref
    }
}

#[actor_actions]
impl Zone {
    pub fn fast_update(&mut self) {
        let start_time = Instant::now();

        self.app.update();

        let cycle_duration = Instant::now().duration_since(start_time);
        if cycle_duration.as_millis() >= 30 {
            warn!(zone = self.factory.zone_def().guid.to_string(); "Zone update cycle can't keep up! Took {}ms", cycle_duration.as_millis());
        }
    }

    pub fn slow_update(&mut self) {
        self.app.world.run_schedule(SlowUpdate);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<ZoneEvent>> {
        self.event_sender.subscribe()
    }

    pub async fn spawn_player(&mut self, spawn_mode: PlayerSpawnMode, avatar_id: AvatarId, character_id: u32, avatar_event_sender: mpsc::Sender<AvatarEvent>) -> AnotherlandResult<(Character, ServerAction)> {
        let mut spawn_mode = spawn_mode;
        let action;

        let position;
        
        if let Some(mut character) = Character::get(self.realm_db.clone(), &character_id).await? {
            // do some first time spawn setup
            if character.data.first_time_spawn() {
                character.data.set_spawn_mode(PlayerSpawnMode::LoginFirstTime.into());
                character.data.set_first_time_spawn(false);

                spawn_mode = PlayerSpawnMode::LoginFirstTime;
            }

            if self.factory.config().only_spawn_to_entry_point() {
                spawn_mode = PlayerSpawnMode::TravelDirect;
            }

            // update zone if stored zone differs or we force spawn to entry point
            match spawn_mode {
                PlayerSpawnMode::LoginFirstTime |
                PlayerSpawnMode::TravelDirect => {
                    // special case if the player comes from class selection,
                    // perform some setup in that case.
                    if *character.data.zone_guid() == Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap() {
                        character.data.set_hp_cur(character.data.hp_max());
                    }

                    debug!("Updating player avatar zone");

                    character.data.set_pos((0, self.default_pos));
                    character.data.set_rot(self.default_rot);

                    character.data.set_zone(&self.factory.zone_def().zone);
                    character.data.set_zone_guid(self.factory.zone_def().guid);
                    character.data.set_world_map_guid(&self.factory.world_def().guid.to_string());
                    character.world_id = self.factory.world_def().id as u32;

                    position = Position { 
                        mover_key: 0,
                        replica: 7,
                        version: 1,
                        position: character.data.pos().to_owned().1,
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    };

                    action = ServerAction::DirectTravel(AvatarId::default(), Some(position.clone()));
                },
                PlayerSpawnMode::LoginNormal => {
                    position = Position { 
                        mover_key: 0,
                        replica: 7,
                        version: 1,
                        position: character.data.pos().to_owned().1,
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    };

                    action = ServerAction::DirectTravel(AvatarId::default(), Some(position.clone()));
                },
                PlayerSpawnMode::TravelPortal(portal_uuid) => {
                    let (portal, portal_avatar) = self.uuid_to_entity_lookup.get(&portal_uuid).map(|entity| {
                        (
                            self.app.world.get::<PortalClass>(*entity).unwrap(), 
                            self.app.world.get::<AvatarComponent>(*entity).unwrap()
                        )
                    }).unwrap();

                    // get exit node
                    if let Some(exit_point) = portal.exit_point() {
                        let exit = self.uuid_to_entity_lookup.get(&Uuid::parse_str(&*exit_point).unwrap())
                            .and_then(|entity| self.app.world.get::<SpawnNodeClass>(*entity))
                            .unwrap();

                        character.data.set_pos((0, exit.pos().to_owned() + Vec3::new(0.0, 0.0, get_player_height(&character.data) / 2.0)));
                        character.data.set_rot(exit.rot().to_owned());
                    } else {
                        warn!("Exit node not found on portal {}", portal_uuid);

                        character.data.set_pos((0, self.default_pos));
                        character.data.set_rot(self.default_rot);
                    }

                    // move to zone
                    character.data.set_zone(&self.factory.zone_def().zone);
                    character.data.set_zone_guid(self.factory.zone_def().guid);
                    character.data.set_world_map_guid(&self.factory.world_def().umap_guid.to_string());
                    character.world_id = self.factory.world_def().id as u32;

                    position = Position { 
                        mover_key: 0,
                        replica: 7,
                        version: 1,
                        position: character.data.pos().to_owned().1,
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    };

                    action = ServerAction::DirectTravel(portal_avatar.id, Some(position.clone()));
                },
                _ => unimplemented!(),
            }

            character.data.set_spawn_mode(spawn_mode.into());
            character.data.set_client_ready(false);
            character.data.set_player_loading(true);
            character.data.set_player_node_state(2);

            // save character changes
            character.save(self.realm_db.clone()).await?;

            let entity = self.app.world.spawn(character.data.as_bundle())
                .insert(AvatarComponent {
                    id: avatar_id,
                    instance_id: None,
                    content_id: None,
                    name: character.name.clone(),
                    phase_tag: "".to_owned(),
                })
                .insert(position)
                .insert(InterestList {
                    interests: Vec::new(),
                })
                .insert(AvatarEventSender(avatar_event_sender))
                .insert(EntityType::Player)
                .insert(Spawned)
                .id();

            self.avatar_id_to_entity_lookup.insert(avatar_id, entity);

            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
                avatar_id, 
                params: character.data.clone().into_box(),
            }));

            Ok((character, action))
        } else {
            Err(AnotherlandError::app_err("character not found"))
        }
    }

    pub fn despawn_player(&mut self, avatar_id: AvatarId) -> Option<PlayerClass> {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            let mut query = self.app.world.query::<(&Position, &mut PlayerClass)>();
            if let Ok((position, player)) = query.get(&self.app.world, *entity) {
                let mut player = player.to_owned();

                // save player position
                player.set_pos((0, position.position));
                player.set_rot(position.rotation.as_unit_vector());

                // drop references to world
                drop(query);

                self.app.world.despawn(*entity);

                self.avatar_id_to_entity_lookup.remove(&avatar_id);
                let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarDespawned { avatar_id }));

                Some(player)
            } else {
                warn!("Avatar  {:?} is not a player!", avatar_id);
                None
            }
        } else {
            None
        }
    }

    pub fn update_avatar(&mut self, avatar_id: AvatarId, update_set: ParamSetBox) {
        if let Some(mut params) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.app.world.get_mut::<ParamBox>(*ent)) {

            if let Ok(player) = params.get_mut::<PlayerClass>() {
                player.apply(update_set.get().unwrap().to_owned());
            }
        }

         // mirror update back to other clients
        // todo: check if params contain meaningful changes for other clients
        let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
            avatar_id, 
            params: update_set,
        }));
    }


    pub fn move_player_avatar(&mut self, avatar_id: AvatarId, movement: Movement) {
        if let Some(mut position) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.app.world.get_mut::<Position>(*ent)) {

            position.position = movement.position;
            position.rotation = movement.rotation;
            position.velocity = movement.velocity;

            // update clients
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                avatar_id, 
                movement,
            }));
        }
    }

    pub fn get_avatar_params(&mut self, avatar_id: AvatarId) -> Option<(String, ParamBox)> {
        self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.app.world.get_entity(*ent))
            .map(|ent| (ent.get::<AvatarComponent>().unwrap().name.clone(), ent.get::<ParamBox>().unwrap().clone()))
    }

    pub fn get_subjective_avatar_params(&mut self, player_id: AvatarId, avatar_id: AvatarId) -> Option<(String, ParamBox)> {
        if let Some(player_id) = self.avatar_id_to_entity_lookup.get(&player_id) &&
            let Some(target_id) = self.avatar_id_to_entity_lookup.get(&avatar_id) &&
            let Some(avatar_component) = self.app.world.get::<AvatarComponent>(*target_id) 
        {
            let name = avatar_component.name.to_owned();
            drop(avatar_component);
            
            self.app.get_subjective_params(*player_id, *target_id)
                .map(|p| (name, p))
        } else {
            None
        }
    }

    pub fn get_avatar_params_by_uuid(&mut self, uuid: Uuid) -> Option<(String, ParamBox)> {
        self.uuid_to_entity_lookup.get(&uuid)
            .and_then(|ent| self.app.world.get_entity(*ent))
            .map(|ent| (ent.get::<AvatarComponent>().unwrap().name.clone(), ent.get::<ParamBox>().unwrap().clone()))
    }

    pub fn get_avatar_move_state(&mut self, avatar_id: AvatarId) -> Option<Position> {
        self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.app.world.get::<Position>(*ent))
            .cloned()
    }

    pub fn request_behavior(&mut self, avatar: AvatarId, behavior: String, data: String) {
        if let Some(target) = self.avatar_id_to_entity_lookup.get(&avatar) {    
            self.app.request_behavior(*target, behavior, data);
        }
    }

    pub fn tell_behavior(&mut self, instigator: AvatarId, target: AvatarId, behavior: String) {
        if let Some(instigator) = self.avatar_id_to_entity_lookup.get(&instigator) && 
            let Some(target) = self.avatar_id_to_entity_lookup.get(&target) {
                
            self.app.tell_behavior(*instigator, *target, behavior);
        }
    }

    pub fn proximity_chat(&mut self, range: ProximityChatRange, avatar_id: AvatarId, message: String) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|e| self.app.world.get_entity(*e)) {

            let pos: Vec3 = entity.get::<Position>().unwrap().position;
            let sender = entity.get::<AvatarComponent>().unwrap().name.clone();

            info!(
                channel = "speak", 
                range = range,
                sender = sender; 
                "{}: {}", sender, message
            );

            self.app.world.send_event(ProximityChatEvent {
                range,
                pos,
                sender,
                message,
            });
        }
    }

    pub fn kill_avatar(&mut self, avatar_id: AvatarId) {
        if let Some(mut entity) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|e| self.app.world.get_entity_mut(*e)) {
            
            if entity.contains::<PlayerClass>() {
                let mut update = ParamSet::<PlayerAttribute>::new();
                let mut player = entity.get_mut::<PlayerClass>().unwrap();

                update.insert(PlayerAttribute::Alive, false);
                update.insert(PlayerAttribute::HpCur, 0);
                
                player.apply(update.clone());

                let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                    avatar_id, 
                    params: update.into_box(),
                }));
    
                let _ = self.event_sender.send(Arc::new(ZoneEvent::CombatHpUpdate { 
                    avatar_id, 
                    hp: 0,
                }));
            } else if entity.contains::<NpcOtherlandComponent>() {
                let mut update = ParamSet::<NpcOtherlandAttribute>::new();
                let mut npc = entity.get_mut::<NpcOtherlandClass>().unwrap();

                update.insert(NpcOtherlandAttribute::Alive, false);
                update.insert(NpcOtherlandAttribute::HpCur, 0);
                
                npc.apply(update.clone());

                let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                    avatar_id, 
                    params: update.into_box(),
                }));
    
                let _ = self.event_sender.send(Arc::new(ZoneEvent::CombatHpUpdate { 
                    avatar_id, 
                    hp: 0,
                }));
            }
        }
    }
}