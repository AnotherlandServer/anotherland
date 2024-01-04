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
use atlas::{ParamClassContainer, AvatarId, Player, ParamEntity, PlayerComponent, PlayerParam, NpcOtherlandParam, PortalParam, SpawnNodeParam, StructureParam, TriggerParam, StartingPointParam, AvatarType, BoundParamClass, ParamError, ParamClass, NonClientBase, SpawnerParam, ChessPieceParam, ShipParam, InteractObjectParam, PatrolNodeParam, MinigameInfoParam, EdnaContainerParam, MinigameScoreBoardParam, PresetPointParam, DoorParam, MyLandSettingsParam, QuestBeaconParam, ServerGatewayParam, Door, ServerGatewayExitPhaseParam, NonSpawnPlacementParam, PlanetParam, ChessMetaGameLogicParam, OtherlandStructureParam, ServerGatewayExitPhase, MypadRoomDoorParam, BilliardBallParam, WorldDisplayParam, CustomTriggerParam, NonClientBaseComponent};
use futures::Future;
use glam::{Vec3, Quat};
use legion::{World, WorldOptions, Schedule, Resources, Entity, storage::IntoComponentSource};
use log::{info, warn, as_serde, debug, trace};
use mongodb::Database;
use rand::{thread_rng, Rng};
use tokio::{time::{Interval, self}, sync::{mpsc, broadcast}, select, task::JoinHandle, runtime::Handle};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::Uuid;

use crate::{cluster::actor::Actor, db::{ZoneDef, realm_database, Character, Instance, Content, NpcContent, StructureContent, SpawnerContent, WorldDef}, util::{AnotherlandResult, AnotherlandError}, NODE, components::zone::components::AvatarComponent};
use crate::db::DatabaseRecord;

use super::{ZoneEvent, components::{EntityType, InterestEvent, InterestList, Position}, systems::{update_interests, update_interests_system}, PlayerSpawnMode, Movement};

pub struct Zone {
    name: String,
    pub(super) zone_def: ZoneDef,
    pub(super) world_def: WorldDef,
    pub(super) realm_db: Database,

    default_pos: Vec3,
    default_rot: Vec3,

    pub(super) world: World,
    main_update: Schedule,
    resources: Resources,

    cancellation_token: CancellationToken,
    update_task: Option<JoinHandle<()>>,

    event_sender: broadcast::Sender<Arc<ZoneEvent>>,
    avatar_id_to_entity_lookup: HashMap<AvatarId, Entity>,

    pub(super) instance_template: HashMap<Uuid, (Instance, Option<AvatarId>)>,
}

impl Zone {
    pub async fn initialize(world_def: WorldDef, zone_def: ZoneDef) -> AnotherlandResult<Self> {
        Ok(Self {
            name: format!("zone_{}", zone_def.guid),
            zone_def,
            world_def,
            realm_db: realm_database().await,
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
            instance_template: HashMap::new(),
        })
    }
}

// Trust me, Zone is Send + Sync!
unsafe impl Send for Zone {}
unsafe impl Sync for Zone {}

#[async_trait]
impl Actor for Zone {
    fn name(&self) -> &str { self.name.as_str() }

    async fn starting(&mut self) -> AnotherlandResult<()> {
        self.resources.insert(self.event_sender.clone());
        self.resources.insert(Handle::current());
        self.resources.insert(TaskTracker::new());

        self.load_content().await?;

        info!("Loaded zone {}...", self.zone_def.guid);

        Ok(()) 
    }

    async fn started(&mut self) -> AnotherlandResult<()> { 
        let actor_name = self.name().to_owned();

        let token = self.cancellation_token.clone();
        self.update_task = Some(tokio::spawn(async move {
            let mut local_actor = NODE.get_actor(&actor_name).unwrap();
            let mut interval = time::interval(Duration::from_millis(40)); // Aim for 25 cycles/sec

            loop {
                select! {
                    _ = interval.tick() => local_actor.update().await,
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
    pub(super) fn spawn_non_player_avatar<T>(&mut self, entity_type: EntityType, name: &str, phase_tag: &str, components: T) -> AvatarId
        where Option<T>: IntoComponentSource
    {
        // Avatar IDs are prefixed with the avatar type
        let avatar_type = match entity_type {
            EntityType::Player => unimplemented!(),
            EntityType::NpcOtherland => AvatarType::Npc,
            _ => AvatarType::Other,
        };

        // generate avatar id
        let id = {
            let mut rng = thread_rng();
            loop {
                let id = AvatarId::new(rng.gen_range(1..1<<56) << 0xF, avatar_type.clone());
                if !self.avatar_id_to_entity_lookup.contains_key(&id) {
                    break id;
                }
            }
        };

        // spawn entity
        let entity = {
            let entity = self.world.push(components);
            let mut entry = self.world.entry(entity).unwrap();
            entry.add_component(entity_type);
            entry.add_component(AvatarComponent {
                id: id.clone(),
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

            self.avatar_id_to_entity_lookup.insert(id.clone(), entity.clone());
            entity
        };

        // notify clients
        let params = self.get_entity_params(entity).unwrap();
        let _ = self.event_sender.send(Arc::new(ZoneEvent::AvatarSpawned { 
            avatar_id: id.clone(), 
            params,
        }));

        id
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
            warn!(zone = self.zone_def.guid.to_string(); "Zone update cycle can't keep up! Took {}ms", cycle_duration.as_millis());
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<ZoneEvent>> {
        self.event_sender.subscribe()
    }

    pub async fn spawn_player(&mut self, avatar_id: AvatarId, character_id: u32, interest_event_sender: mpsc::Sender<InterestEvent>) -> AnotherlandResult<Character> {
        if let Some(mut character) = Character::get(self.realm_db.clone(), &character_id).await? {
            character.data.set_spawn_mode(PlayerSpawnMode::LoginNormal.into());
            character.data.set_client_ready(false);
            character.data.set_player_loading(true);
            character.data.set_player_node_state(2);

            character.data.set_zone(&self.zone_def.zone);
            character.data.set_zone_guid(self.zone_def.guid.clone().into());
            character.data.set_world_map_guid(&self.world_def.umap_guid.to_string());
            
            // do some first time spawn setup
            if *character.data.first_time_spawn().unwrap_or(&true) {
                character.data.set_pos(self.default_pos.clone());
                character.data.set_rot(self.default_rot.clone());
                character.data.set_first_time_spawn(false);

                character.save(self.realm_db.clone()).await?;
            }

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
}