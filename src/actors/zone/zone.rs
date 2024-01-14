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

use std::{time::{Duration, Instant}, cell::Cell, collections::HashMap, sync::Arc, io, ops::DerefMut};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{ParamClassContainer, AvatarId, Player, ParamEntity, PlayerComponent, PlayerParam, NpcOtherlandParam, PortalParam, SpawnNodeParam, StructureParam, TriggerParam, StartingPointParam, AvatarType, BoundParamClass, ParamError, ParamClass, NonClientBase, SpawnerParam, ChessPieceParam, ShipParam, InteractObjectParam, PatrolNodeParam, MinigameInfoParam, EdnaContainerParam, MinigameScoreBoardParam, PresetPointParam, DoorParam, MyLandSettingsParam, QuestBeaconParam, ServerGatewayParam, Door, ServerGatewayExitPhaseParam, NonSpawnPlacementParam, PlanetParam, ChessMetaGameLogicParam, OtherlandStructureParam, ServerGatewayExitPhase, MypadRoomDoorParam, BilliardBallParam, WorldDisplayParam, CustomTriggerParam, NonClientBaseComponent, Uuid, OaZoneConfigParam, OaZoneConfig, CtfGameFlagParam, StartingPointComponent};
use futures::Future;
use glam::{Vec3, Quat};
use legion::{World, WorldOptions, Schedule, Resources, Entity, storage::IntoComponentSource, IntoQuery};
use log::{info, warn, as_serde, debug, trace};
use mongodb::Database;
use nom::character;
use rand::{thread_rng, Rng};
use tokio::{time::{Interval, self}, sync::{mpsc, broadcast}, select, task::JoinHandle, runtime::Handle};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{cluster::actor::Actor, db::{ZoneDef, realm_database, Character, Instance, Content, NpcContent, StructureContent, SpawnerContent, WorldDef, MiscContent}, util::{AnotherlandResult, AnotherlandError}, NODE, actors::zone::components::AvatarComponent, components::ZoneFactory};
use crate::db::DatabaseRecord;

use super::{ZoneEvent, components::{EntityType, InterestEvent, InterestList, Position}, systems::{update_interests, update_interests_system}, PlayerSpawnMode, Movement, ProximityChatRange};

pub struct Zone {
    pub(super) realm_db: Database,
    pub(super) factory: ZoneFactory,

    default_pos: Vec3,
    default_rot: Vec3,

    pub(super) world: World,
    main_update: Schedule,
    resources: Resources,

    cancellation_token: CancellationToken,
    update_task: Option<JoinHandle<()>>,

    event_sender: broadcast::Sender<Arc<ZoneEvent>>,
    avatar_id_to_entity_lookup: HashMap<AvatarId, Entity>,
}

impl Zone {
    pub fn initialize(factory: ZoneFactory) -> Self {
        Self {
            realm_db: factory.db().clone(),
            factory,
            default_pos: Vec3::default(),
            default_rot: Vec3::default(),
            world: World::new(WorldOptions::default()),
            main_update: Schedule::builder()
                .add_system(update_interests_system())
                .build(),
            resources: Resources::default(),
            cancellation_token: CancellationToken::new(),
            update_task: None,
            event_sender: broadcast::channel(10).0,
            avatar_id_to_entity_lookup: HashMap::new(),
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
        self.resources.insert(self.event_sender.clone());
        self.resources.insert(Handle::current());
        self.resources.insert(TaskTracker::new());

        self.load_content().await?;

        info!("Spawned zone {}...", self.factory.zone_def().guid);

        Ok(()) 
    }

    async fn started(&mut self, mut handle: ActorRef<Self>) -> AnotherlandResult<()> { 
        let token = self.cancellation_token.clone();
        self.update_task = Some(tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(40)); // Aim for 25 cycles/sec

            loop {
                select! {
                    _ = interval.tick() => handle.update().await,
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
    pub(super) fn spawn_non_player_avatar<T>(&mut self, avatar_id: AvatarId, entity_type: EntityType, name: &str, phase_tag: &str, components: T) -> AvatarId
        where Option<T>: IntoComponentSource {

        // spawn entity
        let entity = {
            let entity = self.world.push(components);
            let mut entry = self.world.entry(entity).unwrap();
            entry.add_component(entity_type);
            entry.add_component(AvatarComponent {
                id: avatar_id.clone(),
                name: name.to_owned(),
                phase_tag: phase_tag.to_owned(),
            });

            if let Ok(base) = entry.get_component::<NonClientBaseComponent>() {
                entry.add_component(Position {
                    position: base.pos().unwrap().to_owned(),
                    rotation: Quat::from_euler(glam::EulerRot::XYZ, base.rot().unwrap().x,  base.rot().unwrap().y, base.rot().unwrap().z),
                    velocity: Vec3::default(),
                });
            }

            self.avatar_id_to_entity_lookup.insert(avatar_id.clone(), entity.clone());
            entity
        };

        // notify clients
        let params = self.get_entity_params(entity).unwrap();
        let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
            avatar_id: avatar_id.clone(), 
            params,
        }));

        avatar_id
    }

    pub(super) fn get_entity_params(&mut self, entity: Entity) -> Option<ParamClassContainer> {
        if let Some(entry) = self.world.entry(entity) {
            let params = match entry.get_component::<EntityType>().unwrap() {
                EntityType::Player => PlayerParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Spawner => SpawnerParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::NpcOtherland => NpcOtherlandParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Structure => StructureParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Portal => PortalParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::StartingPoint => StartingPointParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Trigger => TriggerParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::ChessPiece => ChessPieceParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Ship => ShipParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Planet => PlanetParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::InteractObject => InteractObjectParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::PatrolNode => PatrolNodeParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::SpawnNode => SpawnNodeParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::MinigameInfo => MinigameInfoParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::ChessMetaGameLogic => ChessMetaGameLogicParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::EDNAContainer => EdnaContainerParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::BilliardBall => BilliardBallParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::OtherlandStructure => OtherlandStructureParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::MinigameScoreBoard => MinigameScoreBoardParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::PresetPoint => PresetPointParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::Door => DoorParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::CTFGameFlag => CtfGameFlagParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::ServerGateway => ServerGatewayParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::ServerGatewayExitPhase => ServerGatewayExitPhaseParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::NonSpawnPlacement => NonSpawnPlacementParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::MyLandSettings => MyLandSettingsParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::WorldDisplay => WorldDisplayParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::MypadRoomDoor => MypadRoomDoorParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::QuestBeacon => QuestBeaconParam::from_component(&self.world, entity).unwrap().into(),
                EntityType::CustomTrigger => CustomTriggerParam::from_component(&self.world, entity).unwrap().into(),
            };
            
            Some(params)
        } else {
            None
        }
    }
}

#[actor_actions]
impl Zone {
    pub fn update(&mut self) {
        let start_time = Instant::now();
        self.main_update.execute(&mut self.world, &mut self.resources);

        let cycle_duration = Instant::now().duration_since(start_time);
        if cycle_duration.as_millis() >= 30 {
            warn!(zone = self.factory.zone_def().guid.to_string(); "Zone update cycle can't keep up! Took {}ms", cycle_duration.as_millis());
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<ZoneEvent>> {
        self.event_sender.subscribe()
    }

    pub async fn spawn_player(&mut self, spawn_mode: PlayerSpawnMode, avatar_id: AvatarId, character_id: u32, interest_event_sender: mpsc::Sender<InterestEvent>) -> AnotherlandResult<Character> {
        let mut spawn_mode = spawn_mode;
        
        if let Some(mut character) = Character::get(self.realm_db.clone(), &character_id).await? {
            // do some first time spawn setup
            if *character.data.first_time_spawn().unwrap_or(&true) {
                character.data.set_spawn_mode(PlayerSpawnMode::LoginFirstTime.into());
                character.data.set_first_time_spawn(false);

                spawn_mode = PlayerSpawnMode::LoginFirstTime;
            }

            // update zone if stored zone differs or we force spawn to entry point
            if spawn_mode == PlayerSpawnMode::TravelDirect || spawn_mode == PlayerSpawnMode::LoginFirstTime || *self.factory.config().only_spawn_to_entry_point().unwrap() {
                // special case if the player comes from class selection,
                // perform some setup in that case.
                if character.data.zone_guid().unwrap_or(&Uuid::default()) == &Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap() {
                    character.data.set_hp_cur(character.data.hp_max().unwrap_or_default());
                }

                debug!("Updating player avatar zone");

                // lookup entry point and copy position
                let mut entry_point_query = <(&StartingPointComponent, &NonClientBaseComponent)>::query();
                if let Some(entry_point) = entry_point_query.iter(&self.world).next() {
                    debug!("Found entrypoint");

                    character.data.set_pos(entry_point.1.pos().unwrap().to_owned());
                    character.data.set_rot(entry_point.1.rot().unwrap().to_owned());
                }

                character.data.set_zone(&self.factory.zone_def().zone);
                character.data.set_zone_guid(self.factory.zone_def().guid.clone());
                character.data.set_world_map_guid(&self.factory.world_def().umap_guid.to_string());
                character.world_id = self.factory.world_def().id as u32;
            }

            character.data.set_spawn_mode(spawn_mode.into());
            character.data.set_client_ready(false);
            character.data.set_player_loading(true);
            character.data.set_player_node_state(2);

            // save character changes
            character.save(self.realm_db.clone()).await?;

            let entity = self.world.push(character.data.clone().to_entity());
            let mut entry = self.world.entry(entity).unwrap();
            entry.add_component(AvatarComponent {
                id: avatar_id.clone(),
                name: character.name.clone(),
                phase_tag: "".to_owned(),
            });
            entry.add_component(Position {
                position: character.data.pos().unwrap().to_owned(),
                rotation: Quat::from_euler(glam::EulerRot::XYZ, character.data.rot().unwrap().x, character.data.rot().unwrap().y, character.data.rot().unwrap().z),
                velocity: Vec3::default(),
            });
            entry.add_component(InterestList {
                interests: Vec::new(),
                update_sender: interest_event_sender,
            });
            entry.add_component(EntityType::Player);

            self.avatar_id_to_entity_lookup.insert(avatar_id.clone(), entity);

            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
                avatar_id, 
                params: character.data.clone().into(),
            }));

            Ok(character)
        } else {
            Err(AnotherlandError::app_err("character not found"))
        }
    }

    pub fn despawn_avatar(&mut self, avatar_id: AvatarId) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.remove(&avatar_id) {
            self.world.remove(entity);

            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarDespawned { avatar_id }));
        }
    }

    pub fn update_avatar(&mut self, avatar_id: AvatarId, params: ParamClassContainer) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            match &params {
                ParamClassContainer::Player(player) => {
                    if let Some(entry) = self.world.entry(*entity).as_mut() {
                        let (updated_player_component,) = player.clone().to_entity();

                        // apply updates
                        entry.get_component_mut::<PlayerComponent>().unwrap().apply(updated_player_component);
                    }
                },
                _ => todo!(),
            }

            // mirror update back to other clients
            // todo: check if params contain meaningful changes for other clients
            let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                avatar_id, 
                params,
            }));
        }
    }


    pub async fn move_player_avatar(&mut self, avatar_id: AvatarId, movement: Movement) {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            if let Some(mut entry) = self.world.entry(*entity) {
                let position = entry.get_component_mut::<Position>().unwrap();
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

    pub fn get_avatar_params(&mut self, avatar_id: AvatarId) -> Option<(String, ParamClassContainer)> {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            let name = {
                if let Some(entry) = self.world.entry(*entity) {
                    Some(entry.get_component::<AvatarComponent>().unwrap().name.clone())
                } else {
                    None
                }
            };

            self.get_entity_params(*entity).map(|v| (name.unwrap(), v))
        } else {
            None
        }
    }

    pub fn get_avatar_move_state(&mut self, avatar_id: AvatarId) -> Option<Position> {
        if let Some(entity) = self.avatar_id_to_entity_lookup.get(&avatar_id) {
            if let Some(entry) = self.world.entry(*entity) {
                Some(entry.get_component::<Position>().unwrap().clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn proximity_chat(&mut self, range: ProximityChatRange, avatar_id: AvatarId, message: String) {
        
    }
}