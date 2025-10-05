// Copyright (C) 2025 AnotherlandServer
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

use std::{sync::Arc, time::Instant};

use bevy::{app::{First, Last, Plugin, PreUpdate, Update}, ecs::{component::{Component, HookContext}, event::{Event, EventReader, EventWriter}, hierarchy::ChildOf, query::{Changed, Or, With, Without}, resource::Resource, schedule::IntoScheduleConfigs, system::{In, Res, SystemId}, world::World}, math::Vec3, platform::collections::HashMap, prelude::{Added, Commands, Entity, NextState, Query, ResMut}, time::{Time, Virtual}};
use futures_util::{future::join_all, TryStreamExt};
use log::{debug, error, info, trace, warn};
use mlua::{Function, Lua, Table};
use obj_params::{tag_gameobject_entity, tags::{NpcBaseTag, NpcOtherlandTag, PlayerTag, StructureBaseTag}, Class, ContentRefList, EdnaFunction, GameObjectData, ItemEdna, NonClientBase, NpcOtherland, Player};
use realm_api::{ObjectPlacement, RealmApi};
use scripting::{EntityScriptCommandsExt, LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptResult};
use tokio::runtime;
use toolkit::{types::{AvatarId, AvatarType, Uuid}, NativeParam};
use anyhow::anyhow;

use crate::{error::{WorldError, WorldResult}, instance::{InstanceState, ZoneInstance}, object_cache::{CacheEntry, ObjectCache}, plugins::{navigation, CachedObject, CommandExtPriv, CooldownGroups, ForceSyncPositionUpdate, FutureCommands, Inventory, ItemAbilities, ItemEdnaAbilities, Movement, NpcAbilities, ParamValue, PlayerController, SpawnCallback}, OBJECT_CACHE};

use super::{AvatarIdManager, AvatarInfo, Factions, FutureTaskComponent, HealthUpdateEvent, MessageType, PlayerLocalSets};

#[derive(Component)]
pub struct ContentInfo {
    pub placement_id: Uuid,
    pub template: Arc<CacheEntry>,
}

#[derive(Component)]
pub struct DynamicInstance;

#[derive(Resource)]
pub struct LoaderSystems {
    spawn_instance: SystemId<In<(Option<Entity>, WorldResult<(GameObjectData, Arc<CacheEntry>, Abilities, Items)>, String, Table, Option<Function>)>>,
}

#[derive(Default, Resource)]
pub struct InstanceManager(HashMap<Uuid, Entity>);

impl InstanceManager {
    #[allow(dead_code)]
    pub fn find_instance(&self, id: Uuid) -> Option<Entity> {
        self.0.get(&id).cloned()
    }
}

#[derive(Event)]
pub struct DespawnAvatar(pub Entity);

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        //let (content_sender, content_receiver) = tokio::sync::mpsc::channel::<Content>(100);

        //app.insert_resource(ForeignResource(content_receiver));
        app.insert_resource(InstanceManager::default());
        app.world_mut().register_component_hooks::<ContentInfo>()
            .on_insert(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.insert(id, entity);
            })
            .on_remove(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.remove(&id);
            });

        app.add_systems(First, init_gameobjects);
        app.add_systems(PreUpdate, (
            update_spawn_state,
            spawn_init_entity
        ).chain());
        app.add_systems(Update, (
            sync_debug_pos.after(navigation::update),
            avatar_despawner,
        ));
        app.add_systems(Last, cleanup_dynamic_instances);

        app.add_event::<DespawnAvatar>();

        app.register_command("get_avatar_info", command_get_avatar_info);

        insert_loader_api(app.world_mut()).expect("Failed to insert loader API");

        let systems = LoaderSystems {
            spawn_instance: app.register_system(spawn_instance),
        };
        app.insert_resource(systems);

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let realm_api = instance.realm_api.clone();
        let zone = instance.zone.clone();
        let object_cache = instance.object_cache.clone();

        let init_task = FutureTaskComponent::new(
            instance.spawn_task(async move {
                // Query
                let mut query = realm_api.query_object_placements()
                    .zone_guid(*zone.guid())
                    .query().await.unwrap();
        
                let mut content = vec![];
                
                while let Some(mut placement) = query.try_next().await.unwrap() {
                    if let Some(template) = object_cache.get_object_by_guid(placement.content_guid).await.unwrap() {
                        placement.data.set_parent(Some(template.data.clone()));
                        let (abilities, items) = load_additional_content(&placement.data).await.unwrap();
                        
                        content.push((placement, template, abilities, items));
                    } else {
                        warn!("Template '{}' not found for placement '{}'", placement.content_guid, placement.id);
                    }
                }
        
                info!("Instance {} load completed.", zone.guid());
                content
            }), 
            app.world_mut().register_system(ingest_content)
        );

        app.world_mut().spawn(init_task);
    }
}

#[derive(Component)]
pub struct DebugPlayer;

fn ingest_content(
    In(content): In<Vec<(ObjectPlacement, Arc<CacheEntry>, Abilities, Items)>>,
    mut next_state: ResMut<NextState<InstanceState>>,
    mut avatar_manager: ResMut<AvatarIdManager>,
    cooldown_groups: Res<CooldownGroups>,
    mut commands: Commands,
) {
    for (placement, template, abilities, items) in content {
        // Skip disabled objects
        if !*placement.data.get::<_, bool>(NonClientBase::EnableInGame).unwrap_or(&false) {
            trace!("Skipping {}", placement.id);
            continue;
        } else {
            trace!("Spawning {}", placement.id);
        }

        let entry = avatar_manager.new_avatar_entry(AvatarType::Npc);

        let factions = if let Ok(faction) = placement.data.get_named::<ContentRefList>("Faction") {
            let mut factions = Factions::default();

            for faction in faction.iter() {
                factions.add_faction(faction.id);
            }

            factions
        } else {
            Factions::default()
        };

        let instance = GameObjectData::instantiate(&Arc::new(placement.data));

        let entity = commands.spawn((
            AvatarInfo {
                id: *entry.key(),
                name: placement.editor_name.clone(),
            },
            ContentInfo {
                placement_id: placement.id,
                template: template.clone(),
            },
            instance,
            Active,
            SpawnState::default(),
            PlayerLocalSets::default(),
            factions,
        )).id();

        entry.insert(entity);

        if template.class == Class::NpcOtherland {
            let mut inventory = Inventory::default();

            items.0.into_iter()
                .for_each(|item| {
                    let instance = GameObjectData::instantiate(&item.item.data);

                    let ent = commands
                        .spawn((
                            ContentInfo {
                                placement_id: item.item.id,
                                template: item.item.clone(),
                            },
                            ItemAbilities(
                                item.abilities.into_iter()
                                    .map(CachedObject)
                                    .collect()
                            ),
                            instance,
                            ChildOf(entity),
                        ))
                        .id(); 

                    inventory.items.insert(item.item.id, ent);
                });

            commands.entity(entity)
                .insert((
                    NpcAbilities(
                        abilities.0.into_iter()
                            .map(|entry| (GameObjectData::instantiate(&entry.data), entry))
                            .collect()
                    ),
                    cooldown_groups.create_cooldowns(),
                    inventory,
                ));
        }
    }

    next_state.set(InstanceState::Initializing);
}

pub fn init_gameobjects(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        tag_gameobject_entity(obj, &mut commands.entity(ent));
    }
}

#[allow(clippy::type_complexity)]
pub fn update_spawn_state(
    mut entities: Query<(Entity, &GameObjectData, &mut SpawnState, &mut Movement), Or<(With<NpcBaseTag>, With<StructureBaseTag>)>>,
    mut health_events: EventWriter<HealthUpdateEvent>,
    mut commands: Commands,
) {
    for (ent, obj, mut state, mut movement) in entities.iter_mut() {
        match *state {
            SpawnState::Alive => {
                if !*obj.get_named::<bool>("alive").unwrap() {
                    debug!("Entity {ent} is dead");
                    state.mark_killed();

                    // This should be handled as event
                    movement.velocity = Vec3::ZERO;

                    commands.entity(ent)
                        .fire_lua_event("Killed", ());
                }
            }
            SpawnState::Killed(instant) => {
                let despawn_delay = *obj.get::<_, f32>(NonClientBase::DespawnDelay).unwrap();

                if instant.elapsed().as_secs_f32() >= despawn_delay {
                    commands.send_event(DespawnAvatar(ent));
                }
            },
            SpawnState::Despawned(instant) => {
                let despawn_delay = *obj.get::<_, f32>(NonClientBase::DespawnDelay).unwrap();

                if instant.elapsed().as_secs_f32() >= despawn_delay {
                    debug!("Respawning entity {ent}");

                    let obj = GameObjectData::instantiate(&obj.parent().unwrap());

                    // This should be handled as event
                    movement.position = *obj.get::<_, Vec3>(NonClientBase::Pos).unwrap();

                    state.mark_alive();
                    health_events.write(HealthUpdateEvent::revive(ent, None, None));
                    commands.entity(ent)
                        .insert(obj)
                        .insert(Active)
                        .fire_lua_event("Spawned", ());
                }
            },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn spawn_init_entity(
    mut entities: Query<(Entity, &mut GameObjectData), Or<(Added<NpcBaseTag>, Added<StructureBaseTag>)>>,
    mut commands: Commands,
) {
    for (ent, mut obj) in entities.iter_mut() {
        let hp_mod = *obj.get_named::<f32>("HpMod").unwrap_or(&1.0);
        let hp_max = *obj.get_named::<i32>("hpMax").unwrap();
        let bonus_hp = *obj.get_named::<f32>("BonusHP").unwrap_or(&0.0);

        obj.set_named("alive", true);
        obj.set_named("hpMax", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);
        obj.set_named("hpCur", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);

        commands.entity(ent)
                .fire_lua_event("Spawned", ());
    }
}

struct Abilities(Vec<Arc<CacheEntry>>);
struct Item {
    item: Arc<CacheEntry>,
    abilities: Vec<Arc<CacheEntry>>,
}
struct Items(Vec<Item>);

#[derive(Component)]
pub struct Active;

#[derive(Component, Default, Clone, Copy)]
pub enum SpawnState {
    #[default]
    Alive,
    Killed(Instant),
    Despawned(Instant),
}

impl SpawnState {
    pub fn mark_killed(&mut self) {
        *self = SpawnState::Killed(Instant::now());
    }

    pub fn mark_despawned(&mut self) {
        *self = SpawnState::Despawned(Instant::now());
    }

    pub fn mark_alive(&mut self) {
        *self = SpawnState::Alive;
    }
}

fn sync_debug_pos(
    query: Query<&Movement, (Changed<Movement>, With<NpcOtherlandTag>, Without<DebugPlayer>)>,
    mut debug_pos: Query<(Entity, &mut Movement, &ChildOf), (With<DebugPlayer>, Without<NpcOtherlandTag>)>,
    mut commands: Commands,
    time: Res<Time<Virtual>>,
) {
    for (ent, mut debug_pos, child_of) in debug_pos.iter_mut() {
        if let Ok(pos) = query.get(child_of.parent()) {
            debug_pos.position = pos.position;
            debug_pos.rotation = pos.rotation;
            debug_pos.velocity = pos.velocity;
            debug_pos.radius = pos.radius;
            debug_pos.mode = pos.mode;
            //debug_pos.mover_type = pos.mover_type;
            //debug_pos.mover_replication_policy = pos.mover_replication_policy;
            //debug_pos.version = pos.version;
            //debug_pos.mover_key = pos.mover_key;
            debug_pos.seconds = time.elapsed_secs_f64();

            commands.entity(ent).insert(ForceSyncPositionUpdate);
        }
    }
}

fn command_get_avatar_info(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    player: Query<(&GameObjectData, &PlayerController), With<PlayerTag>>,
    avatars: Query<(&AvatarInfo, &ContentInfo)>,
    avatar_manager: Res<AvatarIdManager>,
) {
    let Ok((player_data, controller)) = player.get(ent) else {
        return;
    };

    let Ok(&target) = player_data.get::<_, AvatarId>(Player::Target) else {
        return;
    };

    let Some(target_ent) = avatar_manager.resolve_avatar_id(target) else {
        return;
    };

    let Ok((avatar_info, content_info)) = avatars.get(target_ent) else {
        return;
    };

    controller.send_message(MessageType::Normal, format!(
        "Avatar Info:\nID: {} ({})\nName: {}\nTemplate: {} ({})",
        avatar_info.id,
        content_info.placement_id,
        avatar_info.name,
        content_info.template.name,
        content_info.template.id,
    ));
}


#[allow(clippy::type_complexity)]
pub fn insert_loader_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let loader_api = lua.create_table().unwrap();
    runtime.register_native("loader", loader_api.clone()).unwrap();

    loader_api.set("RequestSpawnInstance", lua.create_bevy_function(world, |
        In((owner, template, name, params, callback)): In<(Option<Table>, String, String, Table, Option<Function>)>,
        loader_systems: Res<LoaderSystems>,
        runtime: Res<LuaRuntime>,
        mut commands: Commands
    | -> WorldResult<()> {
        let lua = runtime.vm().clone();

        commands.run_system_async(async move {
            let res = try {
                let template = OBJECT_CACHE.wait()
                    .get_object_by_name(&template).await?
                    .ok_or(WorldError::Other(anyhow!("Template '{}' not found", template)))?;

                let mut instance = GameObjectData::instantiate(&template.data);
                for pair in params.pairs::<String, mlua::Value>() {
                    let (key, value) = pair?;

                    let attr = instance.class().get_attribute(&key)
                        .ok_or(mlua::Error::runtime("attribute not found"))?;

                    let value = ParamValue::from_lua(attr, value, &lua)?;

                    instance.set_named(&key, value);
                }

                let (abilities, items) = load_additional_content(&instance).await?;

                (instance, template, abilities, items)
            };

            (
                owner.and_then(|t| t.entity().ok()),
                res,
                name,
                params,
                callback,
            )
        }, loader_systems.spawn_instance);

        Ok(())
    })?)?;

    loader_api.set("DespawnAvatar", lua.create_bevy_function(world, |
        In(ent): In<Table>,
        mut commands: Commands,
    | -> WorldResult<()> {
        let Ok(entity) = ent.entity() else {
            return Err(WorldError::Other(anyhow!("Invalid entity")));
        };

        commands.send_event(DespawnAvatar(entity));

        Ok(())
    })?)?;

    Ok(())
}

async fn load_additional_content(instance: &GameObjectData) -> WorldResult<(Abilities, Items)> {
    let mut items: Vec<obj_params::ContentRef> = vec![];

    if let Ok(weapons) = instance.get::<_, ContentRefList>(NpcOtherland::DefaultWeapon) {
        items.extend_from_slice(weapons);
    }

    if let Ok(def_items) = instance.get::<_, ContentRefList>(NpcOtherland::DefaultItems) {
        items.extend_from_slice(def_items);
    }

    let items = join_all(
        items.into_iter()
            .map(async |weapon| {
                let item = OBJECT_CACHE.wait().get_object_by_guid(weapon.id).await?
                    .ok_or(WorldError::Other(anyhow!("Item with GUID {} not found", weapon.id)))?;
                
                let mut item_abilities = vec![];

                if 
                    let Ok(abilities) = item.data.get::<_, serde_json::Value>(ItemEdna::Abilities) &&
                    let Ok(abilities) = serde_json::from_value::<ItemEdnaAbilities>(abilities.to_owned())
                {
                    for ability in abilities.0 {
                        if let Some(ability) = OBJECT_CACHE.wait().get_object_by_name(&ability.ability_name).await? {
                            item_abilities.push(ability);
                        }
                    }
                }

                if 
                    let Ok(skills) = item.data.get::<_, ContentRefList>(EdnaFunction::DefaultSkills)
                {
                    for skill in skills.iter() {
                        if let Some(ability) = OBJECT_CACHE.wait().get_object_by_guid(skill.id).await? {
                            item_abilities.push(ability);
                        }
                    }
                }

                Ok::<_, WorldError>(Item {
                    item,
                    abilities: item_abilities,
                })
            })
        ).await
        .into_iter()
        .filter_map(|result| {
            match result {
                Ok(item) => Some(item),
                Err(e) => {
                    warn!("Failed to fetch item: {e}");
                    None
                }
            }
        })
        .collect();

    // Collect ability GUIDs from the placement data
    let ability_guids = instance.get::<_, ContentRefList>(NpcOtherland::Abilities)
        .map(|abilities| {
            abilities
                .iter()
                .map(|content_ref| content_ref.id)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    // Fetch all abilities concurrently
    let ability_futures = ability_guids.iter().map(|guid| {
        OBJECT_CACHE.wait().get_object_by_guid(*guid)
    });

    let ability_results = join_all(ability_futures).await;
    
    // Filter out failed lookups and collect successful ones
    let abilities: Vec<Arc<CacheEntry>> = ability_results
        .into_iter()
        .filter_map(|result| result.unwrap_or(None))
        .collect();
    
    Ok((Abilities(abilities), Items(items)))
}

fn spawn_instance(
    In((owner, instance, name, params, callback)): In<(Option<Entity>, WorldResult<(GameObjectData, Arc<CacheEntry>, Abilities, Items)>, String, Table, Option<Function>)>,
    mut avatar_manager: ResMut<AvatarIdManager>,
    cooldown_groups: Res<CooldownGroups>,
    mut commands: Commands,
) {
    let (instance, template, abilities, items) = match instance {
        Ok(r) => r,
        Err(e) => {
            if let Some(callback) = callback {
                commands.call_lua_method(callback, e.to_string());
            }

            return;
        }
    };

    let entry = avatar_manager.new_avatar_entry(AvatarType::Npc);

    let factions = if let Ok(faction) = instance.get_named::<ContentRefList>("Faction") {
        let mut factions = Factions::default();

        for faction in faction.iter() {
            factions.add_faction(faction.id);
        }

        factions
    } else {
        Factions::default()
    };

    let ent = commands.spawn((
        DynamicInstance,
        AvatarInfo {
            id: *entry.key(),
            name,
        },
        ContentInfo {
            placement_id: Uuid::new(),
            template: template.clone(),
        },
        instance,
        factions,
        Active,
        SpawnState::default(),
        PlayerLocalSets::default(),
    )).id();

    entry.insert(ent);

    if let Some(callback) = callback {
        commands.entity(ent).insert(SpawnCallback(callback));
    }

    if let Some(owner) = owner {
        commands.entity(ent).insert(ChildOf(owner));
    }

    if template.class == Class::NpcOtherland {
        let mut inventory = Inventory::default();

        items.0.into_iter()
            .for_each(|item| {
                let instance = GameObjectData::instantiate(&item.item.data);

                let ent = commands
                    .spawn((
                        ContentInfo {
                            placement_id: item.item.id,
                            template: item.item.clone(),
                        },
                        ItemAbilities(
                            item.abilities.into_iter()
                                .map(CachedObject)
                                .collect()
                        ),
                        instance,
                        ChildOf(ent),
                    ))
                    .id(); 

                inventory.items.insert(item.item.id, ent);
            });

        commands.entity(ent)
            .insert((
                NpcAbilities(
                    abilities.0.into_iter()
                        .map(|entry| (GameObjectData::instantiate(&entry.data), entry))
                        .collect()
                ),
                cooldown_groups.create_cooldowns(),
                inventory,
            ));
    }
}

fn cleanup_dynamic_instances(
    instances: Query<(Entity, &SpawnState), (Changed<SpawnState>, With<DynamicInstance>)>,
    mut commands: Commands,
) {
    for (ent, state) in instances.iter() {
        if matches!(*state, SpawnState::Despawned(_)) {
            commands.entity(ent).despawn();
        }
    }
}

fn avatar_despawner(
    mut events: EventReader<DespawnAvatar>,
    mut query: Query<&mut SpawnState>,
    mut commands: Commands,
) {
    for DespawnAvatar(ent) in events.read() {
        if 
            let Ok(mut state) = query.get_mut(*ent) &&
            !matches!(*state, SpawnState::Despawned(_))
        {
            debug!("Despawning entity {ent}");

            state.mark_despawned();
            commands.entity(*ent)
                .fire_lua_event("Despawned", ())
                .remove::<Active>();
        }
    }
}