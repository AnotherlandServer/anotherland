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

use std::{cell::Cell, collections::HashMap, f32::consts::PI, io, ops::DerefMut, sync::Arc, time::{Duration, Instant}};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{oaPktConfirmTravel, oaPktServerAction, raknet::Message, setup_atlas, AvatarId, NonClientBaseComponent, NonClientBaseParams, OaZoneConfigParams, ParamBox, ParamClass, ParamSetBox, PlayerClass, PlayerParams, PortalClass, PortalParams, SpawnNodeClass, StartingPointComponent, Uuid};
use glam::{Vec3, Quat};
use log::{as_serde, debug, error, info, trace, warn};
use mongodb::Database;
use specs::{Dispatcher, DispatcherBuilder, Entity, Join, World, WriteStorage};
use tokio::{time, sync::{mpsc, broadcast}, select, task::JoinHandle, runtime::Handle};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use specs::{WorldExt, Builder};

use crate::{actors::Spawned, cluster::{actor::Actor, ActorRef}, components::ZoneFactory, db::{Character, Instance, RawInstance}, frontends::TravelType, util::{AnotherlandError, AnotherlandResult, OtherlandQuatExt}, NODE};
use crate::db::DatabaseRecord;

use super::{components::{AvatarComponent, EntityType, AvatarEvent, InterestList, Position}, systems::{RespawnEntities, UpdateInterests}, AvatarEventServer, Movement, PlayerSpawnMode, ProximityChatRange, SpawnerState, ZoneEvent};

pub enum ServerAction {
    DirectTravel(AvatarId, Option<Position>),
    NonPortalTravel(AvatarId, Option<Position>),
    TeleportTravel(AvatarId, Option<Position>),
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
            Self::TeleportTravel(instigator, teleport_override) => (
                instigator,
                "TRAVEL:TeleportTravel|TeleportTravelDefault".to_owned(),
                4,
                teleport_override
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
        setup_atlas(&mut self.world);

        self.world.register::<AvatarComponent>();
        self.world.register::<EntityType>();
        self.world.register::<InterestList>();
        self.world.register::<Position>();
        self.world.register::<Spawned>();
        self.world.register::<SpawnerState>();
        self.world.register::<AvatarEventServer>();

        self.world.insert(self.event_sender.clone());
        self.world.insert(Handle::current());
        self.world.insert(TaskTracker::new());

        self.load_content().await?;

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
                    _ = update_interval.tick() => handle.update().await,
                    _ = respawn_interval.tick() => handle.check_respawn().await,
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
    pub(super) fn spawn_non_player_avatar<T>(&mut self, avatar_id: AvatarId, entity_type: EntityType, name: &str, phase_tag: &str, id: Uuid, entity_params: T) -> AvatarId 
        where T: ParamClass + Clone + ?Sized
    {
        // spawn entity
        let entity = entity_params
            .append_to_entity(self.world.create_entity())
            .with(entity_type)
            .with(AvatarComponent {
                id: avatar_id.clone(),
                name: name.to_owned(),
                phase_tag: phase_tag.to_owned(),
            })
            .build();

        // insert position component for npcs & structures
        let base_storage = self.world.read_storage::<NonClientBaseComponent>();
        if let Some(base) = base_storage.get(entity) {
            let _ = self.world.write_storage::<SpawnerState>().insert(entity, SpawnerState { 
                despawn_instant: None, 
                respawn_instant: None 
            });

            debug!("Rot: {:?}", base.rot());

            let _ = self.world.write_storage::<Position>().insert(entity, Position {
                position: base.pos().to_owned(),
                rotation: Quat::from_unit_vector(base.rot().to_owned()),
                velocity: Vec3::default(),
            });
        } else {
            // assume the entity is always spawned
            let _ = self.world.write_storage::<Spawned>().insert(entity, Spawned);
        }

        // update lookup map
        self.avatar_id_to_entity_lookup.insert(avatar_id.clone(), entity.clone());
        self.uuid_to_entity_lookup.insert(id, entity.clone());

        
        // notify clients
        let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
            avatar_id: avatar_id.clone(), 
            params: entity_params.clone().into_box(),
        }));

        avatar_id
    }

    pub(super) fn get_entity_params(&mut self, entity: Entity) -> Option<ParamBox> {
        if let Some(params) = self.world.read_storage::<ParamBox>().get(entity) {
            Some(params.clone())
        } else {
            None
        }
    }
}

#[actor_actions]
impl Zone {
    pub fn update(&mut self) {
        let start_time = Instant::now();

        let mut dispatcher = DispatcherBuilder::new()
            .with(UpdateInterests, "update_interests", &[])
            .build();

        dispatcher.dispatch(&self.world);
        self.world.maintain();

        let cycle_duration = Instant::now().duration_since(start_time);
        if cycle_duration.as_millis() >= 30 {
            warn!(zone = self.factory.zone_def().guid.to_string(); "Zone update cycle can't keep up! Took {}ms", cycle_duration.as_millis());
        }
    }

    pub fn check_respawn(&mut self) {
        let mut dispatcher = DispatcherBuilder::new()
            .with(RespawnEntities, "respawn_entities", &[])
            .build();

        dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<ZoneEvent>> {
        self.event_sender.subscribe()
    }

    pub async fn spawn_player(&mut self, spawn_mode: PlayerSpawnMode, avatar_id: AvatarId, character_id: u32, avatar_event_sender: mpsc::Sender<AvatarEvent>) -> AnotherlandResult<(Character, ServerAction)> {
        let mut spawn_mode = spawn_mode;
        let mut action;
        
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

                    // lookup entry point and copy position
                    let starting_point_storage = self.world.read_storage::<StartingPointComponent>();
                    let non_client_storage = self.world.read_storage::<NonClientBaseComponent>();

                    if let Some(entry_point) = (&starting_point_storage, &non_client_storage).join().next() {
                        debug!("Found entrypoint");

                        character.data.set_pos(entry_point.1.pos().to_owned());
                        character.data.set_rot(entry_point.1.rot().to_owned());
                    }

                    character.data.set_zone(&self.factory.zone_def().zone);
                    character.data.set_zone_guid(self.factory.zone_def().guid.clone());
                    character.data.set_world_map_guid(&self.factory.world_def().umap_guid.to_string());
                    character.world_id = self.factory.world_def().id as u32;

                    action = ServerAction::DirectTravel(AvatarId::default(), Some(Position { 
                        position: character.data.pos().to_owned(),
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    }));
                },
                PlayerSpawnMode::LoginNormal => {
                    action = ServerAction::DirectTravel(AvatarId::default(), Some(Position { 
                        position: character.data.pos().to_owned(),
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    }));
                },
                PlayerSpawnMode::TravelPortal(portal_uuid) => {
                    let portal_storage = self.world.read_storage::<PortalClass>();
                    let spawn_storage = self.world.read_storage::<SpawnNodeClass>();
                    let avatar_storage = self.world.read_storage::<AvatarComponent>();

                    // get portal node
                    let portal_entity = self.uuid_to_entity_lookup.get(&portal_uuid).unwrap();
                    let portal = portal_storage.get(*portal_entity).unwrap();
                    let portal_avatar = avatar_storage.get(*portal_entity).unwrap();

                    // get exit node
                    let exit_entity = self.uuid_to_entity_lookup.get(&Uuid::parse_str(&*portal.exit_point().unwrap()).unwrap()).unwrap();
                    let exit = spawn_storage.get(*exit_entity).unwrap();

                    // move to zone
                    character.data.set_zone(&self.factory.zone_def().zone);
                    character.data.set_zone_guid(self.factory.zone_def().guid.clone());
                    character.data.set_world_map_guid(&self.factory.world_def().umap_guid.to_string());
                    character.world_id = self.factory.world_def().id as u32;

                    // position player at exit
                    character.data.set_pos(exit.pos().to_owned());
                    character.data.set_rot(exit.rot().to_owned());

                    action = ServerAction::DirectTravel(portal_avatar.id, Some(Position { 
                        position: character.data.pos().to_owned(),
                        rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                        velocity: Vec3::default(),
                    }));
                },
                _ => unimplemented!(),
            }

            character.data.set_spawn_mode(spawn_mode.into());
            character.data.set_client_ready(false);
            character.data.set_player_loading(true);
            character.data.set_player_node_state(2);

            // save character changes
            character.save(self.realm_db.clone()).await?;

            let entity = character.data.append_to_entity(self.world.create_entity())
                .with(AvatarComponent {
                    id: avatar_id.clone(),
                    name: character.name.clone(),
                    phase_tag: "".to_owned(),
                })
                .with(Position {
                    position: character.data.pos().to_owned(),
                    rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                    velocity: Vec3::default(),
                })
                .with(InterestList {
                    interests: Vec::new(),
                })
                .with(AvatarEventServer {
                    sender: avatar_event_sender,
                })
                .with(EntityType::Player)
                .with(Spawned)
                .build();

            self.avatar_id_to_entity_lookup.insert(avatar_id.clone(), entity);

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
            let _ = self.world.entities_mut().delete(entity);
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarDespawned { avatar_id }));
        }
    }

    pub fn update_avatar(&mut self, avatar_id: AvatarId, update_set: ParamSetBox) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            if let Some(params) = self.world.write_storage::<ParamBox>().get_mut(*entity) {
                if let Ok(player) = params.get_mut::<PlayerClass>() {
                    player.apply(update_set.get().unwrap().to_owned())
                }
            }

            // mirror update back to other clients
            // todo: check if params contain meaningful changes for other clients
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                avatar_id, 
                params: update_set,
            }));
        }
    }


    pub async fn move_player_avatar(&mut self, avatar_id: AvatarId, movement: Movement) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            if let Some(position) = self.world.write_storage::<Position>().get_mut(*entity) {
                position.position = movement.position.clone().into();
                position.rotation = movement.rotation.clone().into();
                position.velocity = movement.velocity.clone().into();

                // update clients
                let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                    avatar_id, 
                    movement,
                }));
            }
        }
    }

    pub fn get_avatar_params(&mut self, avatar_id: AvatarId) -> Option<(String, ParamBox)> {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            let name = self.world.read_component::<AvatarComponent>().get(*entity).map(|v| v.name.clone());
            self.get_entity_params(*entity).map(|v| (name.unwrap(), v))
        } else {
            None
        }
    }

    pub fn get_avatar_params_by_uuid(&mut self, uuid: Uuid) -> Option<(String, ParamBox)> {
        if let Some(entity) = self.uuid_to_entity_lookup.get(&uuid) {
            let name = self.world.read_component::<AvatarComponent>().get(*entity).map(|v| v.name.clone());
            self.get_entity_params(*entity).map(|v| (name.unwrap(), v))
        } else {
            None
        }
    }

    pub fn get_avatar_move_state(&mut self, avatar_id: AvatarId) -> Option<Position> {
       if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            if let Some(entry) = self.world.read_component::<Position>().get(*entity) {
                Some(entry.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn tell_behavior(&mut self, instigator: AvatarId, target: AvatarId, behavior: String) {
        // todo: move behaviors into scripts
        if let Some(player_entity) = self.avatar_id_to_entity_lookup.get(&instigator) && 
            let Some(target_entity) = self.avatar_id_to_entity_lookup.get(&target) {
            let sender_storage = self.world.write_storage::<AvatarEventServer>();
            let sender = sender_storage.get(*player_entity).unwrap().sender.clone();

            // check target entity type
            let entity_type = *self.world.read_storage::<EntityType>().get(*target_entity).unwrap();

            match entity_type {
                EntityType::Portal => {
                    match behavior.as_str() {
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
                }
                _ => warn!("Behavior '{}' not implemented for entity type {:?}", behavior, entity_type),
            }
        }
    }

    pub fn proximity_chat(&mut self, range: ProximityChatRange, avatar_id: AvatarId, message: String) {
        
    }
}