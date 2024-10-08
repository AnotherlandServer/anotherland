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

use std::{ops::{Deref, DerefMut}, sync::Arc, time::{Duration, Instant}};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{ oaAbilityBarReferences, oaPktMoveManagerPosUpdate, oaPlayerClassData, raknet::Message, AbilityBarReference, AvatarId, ClassSkills, DynParamSet, NativeParam, NonClientBaseParams, OaZoneConfigParams, Param, ParamBox, ParamClass, ParamSetBox, PlayerAttribute, PlayerClass, PlayerComponent, PlayerParams, PortalParams, SpawnNodeParams, StartingPointComponent, StartingPointParams, Uuid};
use bevy::{app::{App, PreUpdate, Update}, utils::{hashbrown::HashMap, HashSet}, MinimalPlugins};
use glam::{Vec3, Quat};
use log::{debug, info, warn};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use tokio::{runtime::Handle, select, sync::{mpsc, OnceCell}, task::JoinHandle, time};
use tokio_stream::StreamExt;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel, system::{Command, RunSystemOnce}};

use crate::{actors::{get_player_height, register_commands, zone::{ behaviors::BehaviorsPlugin, plugins::{insert_player_inventory, AnimationPlugin, AvatarBehaviorPlugin, CombatPlugin, CommandsPlugin, CompletedDialogues, DialoguePlugin, FactionsPlugin, HitPointsPlugin, InventoryPlugin, ParamsPlugin, PersistancePlugin, PositionPlugin, PreviousParamBox, QuestLog, QuestsPlugin, SpawnPlugin, SubjectivityPlugin, ThreatList}, resources::{EventInfo, EventInfos, ZoneInfo}, subjective_lenses::SubjectiveLensesPlugin, systems::{send_proximity_chat, sepcial_event_controller, setup_combat_style, setup_combat_style_assassin, setup_combat_style_cyber, setup_combat_style_energizer, setup_combat_style_hacker, setup_combat_style_none, setup_combat_style_rage, setup_combat_style_tech, surf_spline, update_interests}}, Spawned}, cluster::actor::Actor, components::{SpecialEvents, ZoneFactory}, db::{get_cached_floor_maps, realm_database, Character, Content, FlightTube, FloorMapInfo, InventoryEntry, StructureContent}, util::{AnotherlandError, AnotherlandResult, OtherlandQuatExt}};
use crate::db::DatabaseRecord;

use super::{components::{self, AvatarComponent, EntityType, InterestList}, plugins::{award_start_equipment, AvatarEvent, BehaviorExt, CommandsExt, DamageEvent, InCombat, ItemPurchaseRequest, ItemSellRequest, NetworkExt, NetworkPlugin, PlayerController, Position, ServerAction, SubjectivityExt}, resources::Tasks, systems::set_heavy_skill_data, zone_events::ProximityChatEvent, Movement, PhysicsState, PlayerSpawnMode, PortalNodelink, ProximityChatRange, SpawnerState};

pub(super) static SPECIAL_EVENTS: OnceCell<SpecialEvents> = OnceCell::const_new();
pub(in crate::actors::zone) static FLIGHT_TUBES: OnceCell<HashMap<Uuid, Arc<FlightTube>>> = OnceCell::const_new();

pub struct PortalHiveDestination {
    pub name: String,
    pub world_name: String,
    pub display_name: Uuid,
    pub zone: Uuid,
    pub link: PortalNodelink,
}

pub static PORTAL_HIVE_DESTINATIONS: OnceCell<HashMap<String, PortalHiveDestination>> = OnceCell::const_new();
pub static DISPLAY_NAMES: OnceCell<HashMap<Uuid, String>> = OnceCell::const_new();
pub static ZONE_ID_LOOKUP: OnceCell<HashMap<String, Uuid>> = OnceCell::const_new();

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone)]
pub struct SlowUpdate;

#[derive(Resource)]
pub struct DefaultPos {
    pub pos: Vec3,
    pub rot: Vec3,
}

#[derive(Resource)]
pub struct RealmDatabase(pub Database);

#[derive(Resource, Default)]
pub struct UuidToEntityLookup(HashMap<Uuid, Entity>);

impl UuidToEntityLookup {
    pub fn find_entity(&self, id: &Uuid) -> Option<&Entity> {
        self.0.get(id)
    }

    pub fn insert(&mut self, id: Uuid, entity: Entity) {
        self.0.insert(id, entity);
    }
}


#[derive(Resource, Default)]
pub struct AvatarIdToEntityLookup(HashMap<AvatarId, Entity>);

impl Deref for AvatarIdToEntityLookup {
    type Target = HashMap<AvatarId, Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AvatarIdToEntityLookup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct FloorMapInfos(pub Vec<&'static FloorMapInfo>);

#[derive(Resource)]
pub struct StructureContentTemplate(pub HashMap<Uuid, StructureContent>);

const STRUCTURE_TEMPLATES: &[&str] = &[
    "ed57bedb-64dd-425d-8da0-602fb3a13f8e",
    "755c666f-cc65-4e2d-990b-01229807ef11",
];

impl StructureContentTemplate {
    async fn load(db: Database) -> AnotherlandResult<StructureContentTemplate> {
        let mut templates = HashMap::new();

        for str_id in STRUCTURE_TEMPLATES {
            let id = Uuid::parse_str(str_id).unwrap();
            if let Some(content) = StructureContent::get(db.clone(), &id).await? {
                templates.insert(id, content);
            }
        }
        
        Ok(StructureContentTemplate(templates))
    }
}

#[derive(Component)]
pub struct CurrentTarget(pub Entity);

#[derive(Event)]
pub struct PlayerSpawned {
    pub player: Entity,
    pub mode: PlayerSpawnMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarState {
    pub id: AvatarId,
    pub instance_id: Option<Uuid>,
    pub record_id: Option<Uuid>,
    pub name: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub params: ParamBox,
}

pub struct Zone {
    pub(super) realm_db: Database,
    pub(super) factory: ZoneFactory,

    default_pos: Vec3,
    default_rot: Vec3,

    pub(super) app: App,

    cancellation_token: CancellationToken,
    update_task: Option<JoinHandle<()>>,
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

        // load display names
        DISPLAY_NAMES.get_or_try_init(Zone::load_display_names).await.unwrap();

        // fill zone lookup table
        ZONE_ID_LOOKUP.get_or_try_init(Zone::load_zone_names).await.unwrap();

        // setup bevy app
        self.app
            .add_plugins(MinimalPlugins)
            .add_plugins((
                ParamsPlugin,
                NetworkPlugin,
                PersistancePlugin,
                AvatarBehaviorPlugin,
                BehaviorsPlugin,
                SubjectivityPlugin,
                SubjectiveLensesPlugin,
                InventoryPlugin,
                HitPointsPlugin,
                PositionPlugin,
                CommandsPlugin,
                CombatPlugin,
                FactionsPlugin,
                QuestsPlugin,
                DialoguePlugin
            ))
            .add_plugins((
                AnimationPlugin,
                SpawnPlugin
            ))
            .add_systems(PreUpdate, setup_combat_style)
            .add_systems(Update, (
                send_proximity_chat,
                update_interests,
                surf_spline,
                (
                    setup_combat_style_none,
                    setup_combat_style_rage,
                    setup_combat_style_tech,
                    setup_combat_style_assassin,
                    setup_combat_style_energizer,
                    setup_combat_style_hacker,
                    setup_combat_style_cyber,
                ),
            ))
            .add_systems(SlowUpdate, (
                sepcial_event_controller,
            ))
            .add_event::<ProximityChatEvent>()
            .add_event::<PlayerSpawned>()
            .insert_resource(ZoneInfo(self.factory.clone()))
            .insert_resource(Tasks {
                handle: Handle::current(),
                tasks: TaskTracker::new(),
            })
            .insert_resource(RealmDatabase(realm_database().await))
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
            })
            .insert_resource(UuidToEntityLookup::default())
            .insert_resource(AvatarIdToEntityLookup::default())
            .insert_resource(FloorMapInfos(get_cached_floor_maps(self.factory.world_def().id)))
            .insert_resource(StructureContentTemplate::load(self.realm_db.clone()).await?);

        register_commands(&mut self.app);

        // load in content
        self.load_content_instances().await?;

        // load portal hive destinations
        PORTAL_HIVE_DESTINATIONS.get_or_try_init(Zone::load_portal_hive_destinations).await?;

        // lookup starting point
        {
            let mut query = self.app.world.query_filtered::<&ParamBox, With<StartingPointComponent>>();
            if let Some(entry_point) = query.iter(&self.app.world).next() &&
                let Some(entry_point) = entry_point.get_impl::<dyn StartingPointParams>() {
                debug!("Found entrypoint");

                entry_point.pos().clone_into(&mut self.default_pos);
                entry_point.rot().clone_into(&mut self.default_rot);
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
    ) -> Entity
        where T: ParamClass + Clone + ?Sized
    {
        self.app.world.resource_scope(|world, mut uuid_to_entity: Mut<UuidToEntityLookup>| {
            // spawn entity
            let entity = world
                .spawn(entity_params.clone().into_bundle())
                .insert(entity_type)
                .insert(AvatarComponent {
                    id: avatar_id,
                    instance_id: Some(id),
                    record_id: Some(content_id),
                    name: name.to_owned(),
                    phase_tag: phase_tag.to_owned(),
                })
                .id();

            let mut entity_ref = world.get_entity_mut(entity).unwrap();
            let entity_params = entity_params.into_box();

            // add tags
            if let Some(non_client_base) = entity_params.get_impl::<dyn NonClientBaseParams>() {
                for tag in non_client_base.tags().split(' ') {
                    match tag {
                        "RespawnPoint" => { entity_ref.insert(components::RespawnPoint); }
                        "PortalHive" => { entity_ref.insert(components::PortalHive); }
                        "InteractionTell" => { entity_ref.insert(components::InteractionTell); }
                        _ => (),
                    };
                }
            }

            // insert position component for npcs & structures
            if entity_params.get_impl::<dyn NonClientBaseParams>().is_some() {
                entity_ref
                    .insert(SpawnerState { 
                        despawn_instant: None, 
                        respawn_instant: None 
                    });
            } else {
                // assume the entity is always spawned
                entity_ref.insert(Spawned);
            }

            // update lookup map
            world.get_resource_mut::<AvatarIdToEntityLookup>().unwrap().insert(avatar_id, entity);
            uuid_to_entity.insert(id, entity);

            entity
        })
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

    pub async fn spawn_player(&mut self, spawn_mode: PlayerSpawnMode, avatar_id: AvatarId, character_id: u32, avatar_event_sender: mpsc::UnboundedSender<AvatarEvent>) -> AnotherlandResult<(String, PlayerClass, ServerAction)> {
        let mut spawn_mode = spawn_mode;
        
        if let Some(mut character) = Character::get(self.realm_db.clone(), &character_id).await? {
            let mut inventory_entries = Vec::new();

            // load inventory from database
            {
                let mut cursor = InventoryEntry::get_player_inventory(realm_database().await, character.guid).await?;
                while let Ok(Some(entry)) = cursor.try_next().await {
                    inventory_entries.push(entry);
                }
            }

            let (entity, action) = self.app.world.resource_scope(|world, uuid_to_entity: Mut<UuidToEntityLookup>| {
                world.resource_scope(|world, mut ev: Mut<Events<PlayerSpawned>>| {
                    let action;
                    let position;

                    let mut original_params = character.data.clone();

                    // do some first time spawn setup
                    if character.data.first_time_spawn() {
                        character.data.set_spawn_mode(PlayerSpawnMode::LoginFirstTime.into());

                        spawn_mode = PlayerSpawnMode::LoginFirstTime;
                    }

                    if self.factory.config().only_spawn_to_entry_point() {
                        spawn_mode = PlayerSpawnMode::TravelDirect;
                    }

                    // bling is stored outside of params in database
                    character.data.set_bling(character.bling.unwrap_or_default());
                    character.data.set_cooldown_passed(true);
                    character.data.set_current_ability_bar_references(oaAbilityBarReferences {
                        class_hash: 0xFE0D0DC2,
                        version: 1,
                        count: 9,
                        main_skill_bar: [
                            AbilityBarReference {
                                id: 0,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 1,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 2,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 3,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 4,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 5,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 6,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 7,
                                ..Default::default()
                            },
                            AbilityBarReference {
                                id: 8,
                                ..Default::default()
                            },
                        ].to_vec(),
                        ..Default::default()
                    }.to_bytes());
                    character.data.set_class_data(oaPlayerClassData {
                        class_hash: 0x9D35021A,
                        field_2: 1,
                        ..Default::default()
                    }.to_bytes());

                    // update zone if stored zone differs or we force spawn to entry point
                    match spawn_mode {
                        PlayerSpawnMode::LoginFirstTime |
                        PlayerSpawnMode::TravelDirect => {
                            // special case if the player comes from class selection,
                            // perform some setup in that case.
                            if *character.data.zone_guid() == Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap() {
                                character.data.set_hp_cur(character.data.hp_max());
                            }

                            character.data.set_first_time_spawn(false);

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
                                seconds: 0.0,
                                physics_state: PhysicsState::Walking,
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
                                seconds: 0.0,
                                physics_state: PhysicsState::Walking,
                                position: character.data.pos().to_owned().1,
                                rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                                velocity: Vec3::default(),
                            };

                            action = ServerAction::DirectTravel(AvatarId::default(), Some(position.clone()));
                        },
                        PlayerSpawnMode::TravelPortal(portal_uuid) => {
                            let (portal, portal_avatar) = uuid_to_entity.find_entity(&portal_uuid).map(|entity| {
                                (
                                    world.get::<ParamBox>(*entity).and_then(|p| p.get_impl::<dyn PortalParams>()).unwrap(), 
                                    world.get::<AvatarComponent>(*entity).unwrap()
                                )
                            }).unwrap();

                            // get exit node
                            if !portal.exit_point().is_empty() {
                                let exit = uuid_to_entity.find_entity(&Uuid::parse_str(portal.exit_point()).unwrap())
                                    .and_then(|entity| world.get::<ParamBox>(*entity))
                                    .and_then(|p| p.get_impl::<dyn SpawnNodeParams>())
                                    .unwrap();

                                character.data.set_pos((0, exit.pos().to_owned() + Vec3::new(0.0, 0.0, get_player_height(&character.data) / 2.0)));
                                character.data.set_rot(exit.rot().to_owned());
                            } else {
                                warn!("No exit node set for portal {}", portal_uuid);

                                character.data.set_pos((0, self.default_pos));
                                character.data.set_rot(self.default_rot);
                            }

                            // move to zone
                            let source_world = character.data.world_map_guid().to_string();

                            character.data.set_zone(&self.factory.zone_def().zone);
                            character.data.set_zone_guid(self.factory.zone_def().guid);
                            character.data.set_world_map_guid(&self.factory.world_def().umap_guid.to_string());
                            character.world_id = self.factory.world_def().id as u32;

                            position = Position { 
                                mover_key: 0,
                                replica: 7,
                                version: 1,
                                seconds: 0.0,
                                physics_state: PhysicsState::Walking,
                                position: character.data.pos().to_owned().1,
                                rotation: Quat::from_unit_vector(character.data.rot().to_owned()),
                                velocity: Vec3::default(),
                            };

                            // if we are still on the same map, use local travel
                            if source_world == *character.data.world_map_guid() {
                                action = ServerAction::LocalPortal(portal_avatar.id, position.clone());
                            } else {
                                action = ServerAction::Portal(portal_avatar.id, Some(position.clone()));
                            }
                        },
                        _ => unimplemented!(),
                    }

                    character.data.set_spawn_mode(spawn_mode.into());
                    character.data.set_client_ready(false);
                    character.data.set_player_loading(true);
                    character.data.set_player_node_state(2);

                    // init empty class data
                    set_heavy_skill_data(&mut character.data);

                    character.data.set_current_class_skills(ClassSkills {
                        class_hash: 0x81E0A735,
                        ..Default::default()
                    }.to_bytes());

                    // spawn player into the world
                    let entity = world.spawn(character.data.clone().into_bundle())
                        .insert(AvatarComponent {
                            id: avatar_id,
                            instance_id: None,
                            record_id: Some(character.guid),
                            name: character.name.clone(),
                            phase_tag: "".to_owned(),
                        })
                        .insert(InterestList::new())
                        .insert(EntityType::Player)
                        .insert(PlayerController::new(avatar_id, avatar_event_sender))
                        .insert(position.clone())
                        .insert(CompletedDialogues::new(character.completed_dialogues.clone()))
                        .insert(QuestLog::load_from_character(&character))
                        .insert(ThreatList(HashSet::new()))
                        .id();

                    debug!("Spawn mode: {:?}", spawn_mode);

                    // insert inventory
                    world.run_system_once_with((entity, inventory_entries), insert_player_inventory);

                    // give start equipment to player
                    if matches!(spawn_mode, PlayerSpawnMode::LoginFirstTime) {
                        world.run_system_once_with(entity, award_start_equipment);
                    }

                    // HACK: Add previous parambox here, so we can transfer the changes made in this method 
                    // to the client after travel, instead of having to transfer the whole set again, which
                    // is not expected by the client and leads to unexpected behavor.
                    if !matches!(spawn_mode, PlayerSpawnMode::LoginNormal) && !matches!(spawn_mode, PlayerSpawnMode::LoginFirstTime) {
                        let mut entity = world.get_entity_mut(entity).unwrap();

                        // update visible_item_info in original_params to prevent it from 
                        // beeing detected as change.
                        original_params.set_visible_item_info(
                            entity.get::<ParamBox>()
                                .unwrap()
                                .get_impl::<dyn PlayerParams>()
                                .unwrap()
                                .visible_item_info()
                                .to_vec()
                        );

                        entity.insert(PreviousParamBox(original_params.into_box()));

                        // send position update too!
                        entity.get::<PlayerController>().unwrap().send_message(oaPktMoveManagerPosUpdate {
                            avatar_id,
                            pos: position.position.into(),
                            rot: position.rotation.into(),
                            vel: position.velocity.into(),
                            physics: position.physics_state.into(),
                            mover_key: position.mover_key,
                            seconds: position.seconds,
                            ..Default::default()
                        }.into_message());
                    }

                    ev.send(PlayerSpawned { 
                        player: entity, 
                        mode: spawn_mode,
                    });

                    (
                        entity,
                        action
                    )
                })
            });

            // save character changes
            character.save(self.realm_db.clone()).await?;

            self.app.world.get_resource_mut::<AvatarIdToEntityLookup>().unwrap().insert(avatar_id, entity);

            self.get_subjective_avatar_params(avatar_id, avatar_id)
                .map(|(name, character)| (name, character.take::<PlayerClass>().unwrap(), action))
                .ok_or(AnotherlandError::app_err("character deleted while spawning"))
        } else {
            Err(AnotherlandError::app_err("character not found"))
        }
    }

    pub fn despawn_player(&mut self, avatar_id: AvatarId) -> Option<PlayerClass> {
        self.app.world.resource_scope(|world, mut lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(entity) = lookup_table.get(&avatar_id) {
                let mut query = world.query_filtered::<(&Position, &mut ParamBox), With<PlayerComponent>>();
                if let Ok((position, player)) = query.get(&world, *entity) {
                    let mut player = player.to_owned().take::<PlayerClass>().unwrap();
    
                    // save player position
                    player.set_pos((0, position.position));
                    player.set_rot(position.rotation.as_unit_vector());
    
                    // drop references to world
                    drop(query);
    
                    world.despawn(*entity);
    
                    lookup_table.remove(&avatar_id);
    
                    Some(player)
                } else {
                    warn!("Avatar  {:?} is not a player!", avatar_id);
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn notify_player_ready(&mut self, avatar_id: AvatarId) {
        self.app.world.resource_scope(|mut world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(entity) = lookup_table.get(&avatar_id) {
                let mut query = world.query_filtered::<(&mut ParamBox, &AvatarComponent, &PlayerController), With<PlayerComponent>>();
                if let Ok((mut params, avatar, controller)) = query.get_mut(&mut world, *entity) {
                    let player = params.get_impl_mut::<dyn PlayerParams>().unwrap();
                    player.set_client_ready(true);
                    player.set_player_loading(false);

                    if player.spawn_mode() == 2 {
                        controller.send_game_message(super::plugins::GameMessage::Normal(format!(
                            "Welcome to Anotherland, {}!\n\
                            Please note that the server currently does not host the full game.\n\
                            Join the otherland Discord server to stay updated!",
                            avatar.name
                        )));
                    }
                }

                if let Some(mut player) = world.get_entity_mut(*entity) {
                    player.insert(Spawned);
                    player.insert(InCombat);
                }
            }
        })
    }

    pub fn update_avatar(&mut self, avatar_id: AvatarId, update_set: ParamSetBox) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(mut params) = lookup_table.get(&avatar_id)
                .and_then(|ent| world.get_mut::<ParamBox>(*ent)) {

                if let Ok(diff) = params.get::<PlayerClass>()
                    .map(|p| p.as_set())
                    .map(|s| update_set.get::<PlayerAttribute>().unwrap().diff(s))
                {
                    if !diff.is_empty() {
                        debug!("{:?}", diff);

                        if let Some(Param::Any(ability_bar_data)) = diff.get(&PlayerAttribute::CurrentAbilityBarReferences) {
                            println!("{:#?}", oaAbilityBarReferences::from_bytes(&ability_bar_data).unwrap());
                        }

                        if let Ok(player) = params.get_mut::<PlayerClass>() {
                            player.apply(diff);
                        }
                    }
                }
            }
        })
    }

    pub fn update_avatar_named_params(&mut self, avatar_id: AvatarId, update_set: HashMap<String, Param>) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(mut params) = lookup_table.get(&avatar_id)
                .and_then(|ent| world.get_mut::<ParamBox>(*ent)) {

                for (name, param) in update_set {
                    params.set_param(&name, param);
                }
            }
        })
    }

    pub fn move_player_avatar(&mut self, avatar_id: AvatarId, movement: Movement) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(mut position) = lookup_table.get(&avatar_id)
                .and_then(|ent| world.get_mut::<Position>(*ent)) {

                position.physics_state = movement.physics_state;
                position.mover_key = movement.mover_key;
                position.position = movement.position;
                position.rotation = movement.rotation;
                position.velocity = movement.velocity;
                position.seconds = movement.seconds;
            }
        })
    }

    pub fn get_avatar(&mut self, avatar_id: AvatarId) -> Option<AvatarState> {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            lookup_table.get(&avatar_id)
                .and_then(|ent| world.get_entity(*ent))
                .map(|ent| {
                    let component = ent.get::<AvatarComponent>().unwrap();
                    let position = ent.get::<Position>().unwrap();

                    AvatarState { 
                        id: avatar_id, 
                        instance_id: component.instance_id, 
                        record_id: component.record_id, 
                        name: component.name.clone(), 
                        position: position.position,
                        rotation: position.rotation.as_unit_vector(),
                        params: ent.get::<ParamBox>().unwrap().clone()
                    }
                })
        })
    }

    pub fn get_target_avatar(&mut self, avatar_id: AvatarId) -> Option<AvatarId> {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            lookup_table.get(&avatar_id)
                .and_then(|ent| world.get_entity(*ent))
                .and_then(|ent| ent.get::<CurrentTarget>())
                .and_then(|target| world.get_entity(target.0))
                .and_then(|ent| ent.get::<AvatarComponent>())
                .map(|avatar| avatar.id)
        })
    }

    pub fn get_subjective_avatar_params(&mut self, player_id: AvatarId, avatar_id: AvatarId) -> Option<(String, ParamBox)> {
        if let Some(player_id) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&player_id).cloned() &&
            let Some(target_id) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id).cloned() &&
            let Some(avatar_component) = self.app.world.get::<AvatarComponent>(target_id) 
        {
            let name = avatar_component.name.to_owned();
            
            self.app.get_subjective_params(player_id, target_id)
                .map(|p| (name, p))
        } else {
            None
        }
    }

    pub fn get_avatar_params_by_uuid(&mut self, uuid: Uuid) -> Option<(String, ParamBox)> {
        self.app.world.get_resource::<UuidToEntityLookup>().unwrap().find_entity(&uuid)
            .and_then(|ent| self.app.world.get_entity(*ent))
            .map(|ent| (ent.get::<AvatarComponent>().unwrap().name.clone(), ent.get::<ParamBox>().unwrap().clone()))
    }

    pub fn get_avatar_move_state(&mut self, avatar_id: AvatarId) -> Option<Position> {
        self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id)
            .and_then(|ent| self.app.world.get::<Position>(*ent))
            .cloned()
    }

    pub fn request_behavior(&mut self, avatar: AvatarId, behavior: String, data: String) {
        if let Some(target) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar) {    
            self.app.request_behavior(*target, behavior, data);
        }
    }

    pub fn tell_behavior(&mut self, instigator: AvatarId, target: AvatarId, behavior: String) {
        if let Some(instigator) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&instigator).cloned() && 
            let Some(target) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&target).cloned() {
            
            self.app.tell_behavior(instigator, target, behavior);
        }
    }

    pub fn tell_behavior_binary(&mut self, instigator: AvatarId, target: AvatarId, behavior: String, data: NativeParam) {
        if let Some(instigator) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&instigator).cloned() && 
            let Some(target) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&target).cloned() {
                
            self.app.tell_behavior_binary(instigator, target, behavior, data);
        }
    }

    pub fn proximity_chat(&mut self, range: ProximityChatRange, avatar_id: AvatarId, message: String) {
        if let Some(entity) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id)
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
        if let Some(entity) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id).cloned() {

            let mut ev_sender = self.app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
            ev_sender.send(DamageEvent(entity, i32::MAX));
        }
    }

    pub fn item_purchase_request(&mut self, avatar_id: AvatarId, item: Uuid, count: u32) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(mut ev_purchase_request) = world.get_resource_mut::<Events<ItemPurchaseRequest>>() &&
                let Some(entity) = lookup_table.get(&avatar_id) 
            {
                ev_purchase_request.send(ItemPurchaseRequest(*entity, item, count as i32));
            }
        })
    }

    pub fn item_sell_request(&mut self, avatar_id: AvatarId, item: Uuid, count: u32) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if let Some(mut ev_sell_request) = world.get_resource_mut::<Events<ItemSellRequest>>() &&
                let Some(entity) = lookup_table.get(&avatar_id) 
            {
                ev_sell_request.send(ItemSellRequest(*entity, item, count));
            }
        })
    }

    pub fn transfer_bling(&mut self, avatar_id: AvatarId, amount: i32) {
        if 
            let Some(entity) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id) &&
            let Some(mut params) = self.app.world.get_mut::<ParamBox>(*entity) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>()
        {
            params.set_bling(params.bling().saturating_add(amount));
        }
    }

    pub fn transfer_game_cash(&mut self, avatar_id: AvatarId, amount: i32) {
        if 
            let Some(entity) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&avatar_id) &&
            let Some(mut params) = self.app.world.get_mut::<ParamBox>(*entity) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>()
        {
            params.set_game_cash(params.game_cash().saturating_add(amount));
        }
    }

    pub fn update_player_target(&mut self, player_id: AvatarId, target_id: AvatarId) {
        self.app.world.resource_scope(|world, lookup_table: Mut<AvatarIdToEntityLookup>| {
            if 
                let Some(player_ent) = lookup_table.get(&player_id) &&
                let Some(mut player) = world.get_entity_mut(*player_ent)
            {
                if let Some(target_ent) = lookup_table.get(&target_id) {
                    player.insert(CurrentTarget(*target_ent));
                } else {
                    player.remove::<CurrentTarget>();
                }
            }
        })
    }

    pub fn exec_command(&mut self, player_id: AvatarId, command: String) {
        if let Some(entity) = self.app.world.get_resource::<AvatarIdToEntityLookup>().unwrap().get(&player_id) {
            self.app.execute_command(*entity, &command);
        }
    }

    pub fn handle_message(&mut self, message: Message) {
        self.app.handle_message(message);
    }
}

pub fn get_display_name(id: Uuid) -> &'static str {
    match DISPLAY_NAMES.get().unwrap().get(&id) {
        Some(name) => name.as_str(),
        None => "- Translation missing -",
    }
}