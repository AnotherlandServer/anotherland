// Copyright (C) 2023 AnotherlandServer
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

use std::{collections::HashMap, ops::Deref, sync::Arc, time::{Duration, Instant}};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{oaPktConfirmTravel, oaPktMoveManagerStateChanged, oaPktServerAction, raknet::Message, AvatarId, NonClientBaseComponent, NonClientBaseParams, NpcBaseComponent, NpcOtherlandAttribute, NpcOtherlandClass, NpcOtherlandComponent, NpcOtherlandParams, OaZoneConfigParams, Param, ParamBox, ParamClass, ParamSet, ParamSetBox, PlayerAttribute, PlayerClass, PlayerParams, PortalClass, PortalParams, SpawnNodeClass, StartingPointClass, StartingPointComponent, TriggerClass, Uuid};
use glam::{Vec3, Quat};
use log::{debug, error, info, warn};
use mongodb::Database;
use tokio::{runtime::Handle, select, sync::{broadcast, mpsc, OnceCell}, task::JoinHandle, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use bevy_ecs::{prelude::*, system::RunSystemOnce};

use crate::{actors::{get_player_height, zone::{resources::{EventInfo, EventInfos}, systems::{respawn, send_proximity_chat, sepcial_event_controller, update_interests}}, Spawned}, cluster::{actor::Actor, ActorRef}, components::{SpecialEvents, ZoneFactory}, db::{Character, RawInstance}, frontends::TravelType, util::{AnotherlandError, AnotherlandResult, OtherlandQuatExt}};
use crate::db::DatabaseRecord;

use super::{components::{AvatarComponent, AvatarEvent, EntityType, InterestList, Position}, resources::{Broadcaster, Tasks}, zone_events::ProximityChatEvent, AvatarEventServer, Movement, PlayerSpawnMode, ProximityChatRange, SpawnerState, ZoneEvent};

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

static SPECIAL_EVENTS: OnceCell<SpecialEvents> = OnceCell::const_new();

pub struct Zone {
    pub(super) realm_db: Database,
    pub(super) factory: ZoneFactory,

    default_pos: Vec3,
    default_rot: Vec3,

    pub(super) world: World,

    cancellation_token: CancellationToken,
    update_task: Option<JoinHandle<()>>,

    event_sender: broadcast::Sender<Arc<ZoneEvent>>,
    avatar_id_to_entity_lookup: HashMap<AvatarId, Entity>,
    uuid_to_entity_lookup: HashMap<Uuid, Entity>,

    fast_update: Schedule,
    slow_update: Schedule,
}

impl Zone {
    pub fn initialize(factory: ZoneFactory) -> Self {
        Self {
            realm_db: factory.db().clone(),
            factory,
            default_pos: Vec3::default(),
            default_rot: Vec3::default(),
            world: World::new(),
            cancellation_token: CancellationToken::new(),
            update_task: None,
            event_sender: broadcast::channel(500).0,
            avatar_id_to_entity_lookup: HashMap::new(),
            uuid_to_entity_lookup: HashMap::new(),
            fast_update: Schedule::default(),
            slow_update: Schedule::default(),
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
        self.world.insert_resource(Broadcaster {
            sender: self.event_sender.clone()
        });
        self.world.insert_resource(Tasks {
            handle: Handle::current(),
            tasks: TaskTracker::new(),
        });

        // load in content
        self.load_content().await?;

        // load special event config
        let special_events = SPECIAL_EVENTS.get_or_try_init(SpecialEvents::load).await.unwrap();
        self.world.insert_resource(EventInfos(
            special_events.get_events_for_map(&self.factory.world_def().name).await?
            .into_iter()
            .map(|v| EventInfo {
                event: v,
                active: None
            })
            .collect::<Vec<_>>()
        ));

        // setup schedules
        self.fast_update.add_systems(update_interests);

        self.slow_update.add_systems((
            respawn,
            sepcial_event_controller,
            send_proximity_chat
        ));

        // lookup starting point
        {
            let mut query = self.world.query::<&StartingPointClass>();
            if let Some(entry_point) = query.iter(&self.world).next() {
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
    pub(super) fn spawn_non_player_avatar<T>(&mut self, avatar_id: AvatarId, entity_type: EntityType, name: &str, phase_tag: &str, id: Uuid, content_id: Uuid, entity_params: T) -> AvatarId 
        where T: ParamClass + Clone + ?Sized
    {
        // spawn entity
        let entity = self.world
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

        let mut entity_ref = self.world.get_entity_mut(entity).unwrap();

        // insert position component for npcs & structures
        if let Some(base) = entity_ref.get::<NonClientBaseComponent>() {
            let position = base.pos().to_owned();
            let rotation = base.rot().to_owned();
            drop(base);

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

        avatar_id
    }

    pub(super) fn get_entity_params(&mut self, entity: Entity) -> Option<ParamBox> {
        self.world.get::<ParamBox>(entity).cloned()
    }
}

#[actor_actions]
impl Zone {
    pub fn fast_update(&mut self) {
        let start_time = Instant::now();

        self.fast_update.run(&mut self.world);
        /*let mut dispatcher = DispatcherBuilder::new()
            .with(UpdateInterests, "update_interests", &[])
            .build();

        dispatcher.dispatch(&self.world);
        self.world.maintain();*/

        let cycle_duration = Instant::now().duration_since(start_time);
        if cycle_duration.as_millis() >= 30 {
            warn!(zone = self.factory.zone_def().guid.to_string(); "Zone update cycle can't keep up! Took {}ms", cycle_duration.as_millis());
        }
    }

    pub fn slow_update(&mut self) {
        /*let mut dispatcher = DispatcherBuilder::new()
            .with(SpecialEventController, "special_event_controller", &[])
            .with(RespawnEntities, "respawn_entities", &[])
            .build();*/

        self.slow_update.run(&mut self.world);
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
                            self.world.get::<PortalClass>(*entity).unwrap(), 
                            self.world.get::<AvatarComponent>(*entity).unwrap()
                        )
                    }).unwrap();

                    // get exit node
                    if let Some(exit_point) = portal.exit_point() {
                        let exit = self.uuid_to_entity_lookup.get(&Uuid::parse_str(&*exit_point).unwrap())
                            .and_then(|entity| self.world.get::<SpawnNodeClass>(*entity))
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

            let entity = self.world.spawn(character.data.as_bundle())
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
                .insert(AvatarEventServer {
                    sender: avatar_event_sender,
                })
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

    pub fn despawn_avatar(&mut self, avatar_id: AvatarId) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.remove(&avatar_id) {
            self.world.despawn(entity);
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarDespawned { avatar_id }));
        }
    }

    pub fn update_avatar(&mut self, avatar_id: AvatarId, update_set: ParamSetBox) {
        if let Some(mut params) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.world.get_mut::<ParamBox>(*ent)) {

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


    pub async fn move_player_avatar(&mut self, avatar_id: AvatarId, movement: Movement) {
        if let Some(mut position) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.world.get_mut::<Position>(*ent)) {

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
            .and_then(|ent| self.world.get_entity(*ent))
            .map(|ent| (ent.get::<AvatarComponent>().unwrap().name.clone(), ent.get::<ParamBox>().unwrap().clone()))
    }

    pub fn get_avatar_params_by_uuid(&mut self, uuid: Uuid) -> Option<(String, ParamBox)> {
        self.uuid_to_entity_lookup.get(&uuid)
            .and_then(|ent| self.world.get_entity(*ent))
            .map(|ent| (ent.get::<AvatarComponent>().unwrap().name.clone(), ent.get::<ParamBox>().unwrap().clone()))
    }

    pub fn get_avatar_move_state(&mut self, avatar_id: AvatarId) -> Option<Position> {
        self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|ent| self.world.get::<Position>(*ent))
            .cloned()
    }

    pub fn request_behavior(&mut self, instigator: AvatarId, behavior: String, data: String) {
        todo!()
    }

    pub fn tell_behavior(&mut self, instigator: AvatarId, target: AvatarId, behavior: String) {
            /*if let Some(player_entity) = self.avatar_id_to_entity_lookup.get(&instigator) && 
            let Some(target_entity) = self.avatar_id_to_entity_lookup.get(&target) {

            let sender = self.world.get::<AvatarEventServer>(*player_entity).unwrap().sender.clone();
            let entity_type = *self.world.get::<EntityType>(*target_entity).unwrap();

            let cmd_args: Vec<_> = behavior.split(' ').collect();

            match entity_type {
                EntityType::Player => {
                    match cmd_args[0] {
                        "RespawnNow" => {
                            match cmd_args[1] {
                                "NearestPortal" => {
                                    /*let spawned_storage = self.world.read_storage::<Spawned>();
                                    let mut position_storage = self.world.write_storage::<Position>();
                                    let starting_point_storage = self.world.read_storage::<StartingPointClass>();
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();*/
        
                                    
                                    let mut player_ent = self.world.entity_mut(*player_entity);
                                    
        
                                    // find nearest starting point (most likely a portal exit node)
                                    let mut query = self.world.query::<(&StartingPointClass, With<Spawned>)>();

                                    let mut positions: Vec<_> = query.iter(&self.world)
                                        .map(|(starting_point, _)| (starting_point.pos(), starting_point.rot())).collect();
        
                                    positions.sort_by(|a, b| {
                                        a.0.distance_squared(player_ent.get::<Position>().unwrap().position)
                                            .total_cmp(&b.0.distance_squared(player_ent.get::<Position>().unwrap().position))
                                    });
        
                                    let (respawn_pos, respawn_rot) = if let Some((pos, rot)) = positions.first() {
                                        debug!("Respawn pos: {:?}", pos);
                                        (**pos + Vec3::new(0.0, 0.0, get_player_height(&*player_ent.get::<PlayerClass>().unwrap()) / 2.0), **rot)
                                    } else {
                                        warn!("No portal for respawning found. Moving to default location");
                                        (self.default_pos, self.default_rot)
                                    };
        
                                    // revive & teleport player
                                    let mut update = ParamSet::<PlayerAttribute>::new();
        
                                    {
                                        let mut player = player_ent.get_mut::<PlayerClass>().unwrap();

                                        update.insert(PlayerAttribute::Alive, true);
                                        update.insert(PlayerAttribute::HpCur, player.hp_max());
                                        update.insert(PlayerAttribute::Pos, Param::Vector3Uts((0, respawn_pos)));
                                        update.insert(PlayerAttribute::Rot, respawn_rot);
                                        update.insert(PlayerAttribute::IsUnAttackable, true); 
                            
                                        player.apply(update.clone());
                                    }

                                    {
                                        let mut player_pos = player_ent.get_mut::<Position>().unwrap();

                                        player_pos.version = player_pos.version.wrapping_add(1);
                                        player_pos.position = respawn_pos;
                                        player_pos.rotation = Quat::from_unit_vector(respawn_rot);
                                    }
                        
                                    // update clients
                                    let event_sender = self.event_sender.clone();
        
                                    let current_hp = player_ent.get::<PlayerClass>().unwrap().hp_cur();
                                    let send_pos = player_ent.get::<Position>().unwrap().clone();
        
                                    tokio::spawn(async move {
                                        let _ = sender.send(AvatarEvent::ServerAction(ServerAction::Teleport(instigator, send_pos))).await;

                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                                            avatar_id: instigator, 
                                            movement: Movement { 
                                                position: respawn_pos, 
                                                rotation: Quat::from_unit_vector(respawn_rot), 
                                                velocity: Vec3::default(), 
                                                physics_state: super::PhysicsState::Walking, 
                                                mover_key: 0, 
                                                seconds: 0.0 
                                            } 
                                        }));

                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
        
                                        let _ = event_sender.send(Arc::new(ZoneEvent::CombatHpUpdate { 
                                            avatar_id: instigator, 
                                            hp: current_hp,
                                        }));
                                    });
                                },
                                _ => todo!("Respawn mode {} not implemented!", cmd_args[1]),
                            }
                        },
                        "PromptCooldown" => {
                            // todo
                        },
                        "DisableInvulnerability" => {
                            let mut update = ParamSet::<PlayerAttribute>::new();
                            let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                            update.insert(PlayerAttribute::IsUnAttackable, false);
                            player.apply(update.clone());

                            let event_sender = self.event_sender.clone();
                
                            tokio::spawn(async move {
                                let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                    avatar_id: instigator, 
                                    params: update.into_box(),
                                }));
                            });
                        },
                        _ => warn!("Unimplemented behavior '{}' for player", behavior),
                    }
                },
                EntityType::Portal => {
                    match cmd_args[0] {
                        "ConfirmTravelRequest" => {
                            tokio::spawn(async move {
                                let _ = sender.send(AvatarEvent::Message(oaPktConfirmTravel {
                                    state: 1,
                                    ..Default::default()
                                }.into_message())).await;
                            });
                        },
                        "DoTravel" => {
                            let portal = self.world.get::<PortalClass>(*target_entity).unwrap();
                            if let Some(nodelink) = portal.nodelink().map(|v| v.to_owned()) {
                                let db = self.realm_db.clone();

                                // get the target zone based on the destination node
                                tokio::spawn(async move {
                                    // todo: lookup target zone during zone loading and store in entity
                                    if let Ok(Some(node)) = RawInstance::get(db, &Uuid::parse_str(&nodelink).unwrap()).await {
                                        let _ = sender.send(AvatarEvent::Travel { 
                                            zone: node.zone_guid, 
                                            destination: TravelType::Portal { 
                                                uuid: node.guid 
                                            }
                                        }).await;
                                    } else {
                                        error!("Failed to read node {}", nodelink)
                                    }
                                });
                            } else {
                                warn!("No nodelink for portal set")
                            }
                        },
                        _ => warn!("Unimplemented behavior '{}' for portal node", behavior),
                    }
                },
                EntityType::Trigger => {
                    if cmd_args[0] == "triggeraction" {
                        let trigger = self.world.get::<TriggerClass>(*target_entity).unwrap();

                        if let Some(script) = trigger.lua_script() {
                            match script.deref() {
                                "ClassClear" => {
                                    let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 6);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassEnergizer" => {
                                    let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 3);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassWarrior" => {
                                    let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 0);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassMarksman" => {
                                    let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 1);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassAssassin" => {
                                    let mut player = self.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 2);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                _ => warn!("Unimplemented lua script '{}' for trigger", script),
                            }
                        };
                    } else {
                        warn!("Unimplemented behavior '{}' for trigger", behavior);
                    }
                },
                _ => warn!("Behavior '{}' not implemented for entity type {:?}", behavior, entity_type),
            }
        }*/

        // todo: move behaviors into scripts
        /*if let Some(player_entity) = self.avatar_id_to_entity_lookup.get(&instigator) && 
            let Some(target_entity) = self.avatar_id_to_entity_lookup.get(&target) {
            let sender_storage = self.world.write_storage::<AvatarEventServer>();
            let sender = sender_storage.get(*player_entity).unwrap().sender.clone();

            // check target entity type
            let entity_type = *self.world.read_storage::<EntityType>().get(*target_entity).unwrap();
            let cmd_args: Vec<_> = behavior.split(' ').collect();

            match entity_type {
                EntityType::Player => {
                    match cmd_args[0] {
                        "RespawnNow" => {
                            match cmd_args[1] {
                                "NearestPortal" => {
                                    let spawned_storage = self.world.read_storage::<Spawned>();
                                    let mut position_storage = self.world.write_storage::<Position>();
                                    let starting_point_storage = self.world.read_storage::<StartingPointClass>();
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
        
                                    let player = player_storage.get_mut(*player_entity).unwrap();
                                    let player_pos = position_storage.get_mut(*player_entity).unwrap();
        
                                    // find nearest starting point (most likely a portal exit node)
                                    let mut positions: Vec<_> = (
                                        &starting_point_storage, 
                                        &spawned_storage
                                    ).join().map(|(starting_point, _)| (starting_point.pos(), starting_point.rot())).collect();
        
                                    positions.sort_by(|a, b| {
                                        a.0.distance_squared(player_pos.position)
                                            .total_cmp(&b.0.distance_squared(player_pos.position))
                                    });
        
                                    let (respawn_pos, respawn_rot) = if let Some((pos, rot)) = positions.first() {
                                        debug!("Respawn pos: {:?}", pos);
                                        (**pos + Vec3::new(0.0, 0.0, get_player_height(player) / 2.0), **rot)
                                    } else {
                                        warn!("No portal for respawning found. Moving to default location");
                                        (self.default_pos, self.default_rot)
                                    };
        
                                    // revive & teleport player
                                    let mut update = ParamSet::<PlayerAttribute>::new();
        
                                    update.insert(PlayerAttribute::Alive, true);
                                    update.insert(PlayerAttribute::HpCur, player.hp_max());
                                    update.insert(PlayerAttribute::Pos, Param::Vector3Uts((0, respawn_pos)));
                                    update.insert(PlayerAttribute::Rot, respawn_rot);
                                    update.insert(PlayerAttribute::IsUnAttackable, true); 
                        
                                    player.apply(update.clone());

                                    player_pos.version = player_pos.version.wrapping_add(1);
                                    player_pos.position = respawn_pos;
                                    player_pos.rotation = Quat::from_unit_vector(respawn_rot);
                        
                                    // update clients
                                    let event_sender = self.event_sender.clone();
        
                                    let current_hp = player.hp_cur();
                                    let send_pos = player_pos.clone();
        
                                    tokio::spawn(async move {
                                        let _ = sender.send(AvatarEvent::ServerAction(ServerAction::Teleport(instigator, send_pos))).await;

                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                                            avatar_id: instigator, 
                                            movement: Movement { 
                                                position: respawn_pos, 
                                                rotation: Quat::from_unit_vector(respawn_rot), 
                                                velocity: Vec3::default(), 
                                                physics_state: super::PhysicsState::Walking, 
                                                mover_key: 0, 
                                                seconds: 0.0 
                                            } 
                                        }));

                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
        
                                        let _ = event_sender.send(Arc::new(ZoneEvent::CombatHpUpdate { 
                                            avatar_id: instigator, 
                                            hp: current_hp,
                                        }));
                                    });
                                },
                                _ => todo!("Respawn mode {} not implemented!", cmd_args[1]),
                            }
                        },
                        "PromptCooldown" => {
                            // todo
                        },
                        "DisableInvulnerability" => {
                            let mut update = ParamSet::<PlayerAttribute>::new();
                            let mut player_storage = self.world.write_storage::<PlayerClass>();
                            let player = player_storage.get_mut(*player_entity).unwrap();

                            update.insert(PlayerAttribute::IsUnAttackable, false);
                            player.apply(update.clone());

                            let event_sender = self.event_sender.clone();
                
                            tokio::spawn(async move {
                                let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                    avatar_id: instigator, 
                                    params: update.into_box(),
                                }));
                            });
                        },
                        _ => warn!("Unimplemented behavior '{}' for player", behavior),
                    }
                },
                EntityType::Portal => {
                    match cmd_args[0] {
                        "ConfirmTravelRequest" => {
                            tokio::spawn(async move {
                                let _ = sender.send(AvatarEvent::Message(oaPktConfirmTravel {
                                    state: 1,
                                    ..Default::default()
                                }.into_message())).await;
                            });
                        },
                        "DoTravel" => {
                            let portal_storage = self.world.read_storage::<PortalClass>();
                            let portal = portal_storage.get(*target_entity).unwrap();
                            if let Some(nodelink) = portal.nodelink().map(|v| v.to_owned()) {
                                let db = self.realm_db.clone();

                                // get the target zone based on the destination node
                                tokio::spawn(async move {
                                    // todo: lookup target zone during zone loading and store in entity
                                    if let Ok(Some(node)) = RawInstance::get(db, &Uuid::parse_str(&nodelink).unwrap()).await {
                                        let _ = sender.send(AvatarEvent::Travel { 
                                            zone: node.zone_guid, 
                                            destination: TravelType::Portal { 
                                                uuid: node.guid 
                                            }
                                        }).await;
                                    } else {
                                        error!("Failed to read node {}", nodelink)
                                    }
                                });
                            } else {
                                warn!("No nodelink for portal set")
                            }
                        },
                        _ => warn!("Unimplemented behavior '{}' for portal node", behavior),
                    }
                },
                EntityType::Trigger => {
                    if cmd_args[0] == "triggeraction" {
                        let trigger_storage = self.world.read_storage::<TriggerClass>();
                        let trigger = trigger_storage.get(*target_entity).unwrap();

                        if let Some(script) = trigger.lua_script() {
                            match script.deref() {
                                "ClassClear" => {
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
                                    let player = player_storage.get_mut(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 6);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassEnergizer" => {
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
                                    let player = player_storage.get_mut(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 3);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassWarrior" => {
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
                                    let player = player_storage.get_mut(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 0);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassMarksman" => {
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
                                    let player = player_storage.get_mut(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 1);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                "ClassAssassin" => {
                                    let mut player_storage = self.world.write_storage::<PlayerClass>();
                                    let player = player_storage.get_mut(*player_entity).unwrap();

                                    let mut update = ParamSet::new();
                                    update.insert(PlayerAttribute::CombatStyle, 2);

                                    player.apply(update.clone());

                                    let event_sender = self.event_sender.clone();
                
                                    tokio::spawn(async move {
                                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                                            avatar_id: instigator, 
                                            params: update.into_box(),
                                        }));
                                    });
                                },
                                _ => warn!("Unimplemented lua script '{}' for trigger", script),
                            }
                        };
                    } else {
                        warn!("Unimplemented behavior '{}' for trigger", behavior);
                    }
                },
                _ => warn!("Behavior '{}' not implemented for entity type {:?}", behavior, entity_type),
            }
        }*/
    }

    pub fn proximity_chat(&mut self, range: ProximityChatRange, avatar_id: AvatarId, message: String) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|e| self.world.get_entity(*e)) {

            let pos: Vec3 = entity.get::<Position>().unwrap().position;
            let sender = entity.get::<AvatarComponent>().unwrap().name.clone();

            self.world.send_event(ProximityChatEvent {
                range,
                pos,
                sender,
                message,
            });
        }
    }

    pub fn kill_avatar(&mut self, avatar_id: AvatarId) {
        if let Some(mut entity) = self.avatar_id_to_entity_lookup.get(&avatar_id)
            .and_then(|e| self.world.get_entity_mut(*e)) {
            
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