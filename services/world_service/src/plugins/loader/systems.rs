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

use std::sync::Arc;

use anyhow::anyhow;
use bevy::{ecs::{entity::Entity, event::{EventReader, EventWriter}, hierarchy::ChildOf, query::{Added, Changed, Or, With, Without}, system::{Commands, In, Query, Res, ResMut}, world::World}, math::Vec3, state::state::NextState, time::{Time, Virtual}};
use futures::future::join_all;
use log::{debug, trace, warn};
use mlua::{Function, Lua, Table};
use obj_params::{Class, ContentRefList, EdnaFunction, GameObjectData, ItemEdna, NonClientBase, NpcOtherland, Player, tag_gameobject_entity, tags::{NpcBaseTag, NpcOtherlandTag, PlayerTag, StructureBaseTag}};
use realm_api::{ObjectPlacement, ObjectTemplate};
use scripting::{EntityScriptCommandsExt, LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptResult};
use toolkit::{NativeParam, types::{AvatarId, AvatarType, Uuid}};

use crate::{error::{WorldError, WorldResult}, instance::InstanceState, plugins::{Active, AvatarIdManager, Avatar, StaticObject, ContentInfo, CooldownGroups, DebugPlayer, DespawnAvatar, DynamicInstance, Factions, ForceSyncPositionUpdate, FutureCommands, HealthUpdateEvent, Inventory, ItemAbilities, ItemEdnaAbilities, MessageType, Movement, NpcAbilities, ParamValue, PlayerController, PlayerLocalSets, SpawnCallback, SpawnState}};

pub(super) struct Abilities(Vec<Arc<ObjectTemplate>>);
pub(super) struct Item {
    item: Arc<ObjectTemplate>,
    abilities: Vec<Arc<ObjectTemplate>>,
}
pub(super) struct Items(Vec<Item>);


/*pub(super) fn ingest_content(
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
            Avatar {
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
}*/

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

                    let obj = GameObjectData::instantiate(obj.parent().unwrap());

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


pub(super) fn sync_debug_pos(
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

pub(super) fn command_get_avatar_info(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    player: Query<(&GameObjectData, &PlayerController), With<PlayerTag>>,
    avatars: Query<(&Avatar, &ContentInfo)>,
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
        //loader_systems: Res<LoaderSystems>,
        runtime: Res<LuaRuntime>,
        mut commands: Commands
    | -> WorldResult<()> {
        todo!()
        /*let lua = runtime.vm().clone();

        commands.run_system_async(async move {
            let res = try {
                let template = OBJECT_CACHE.wait()
                    .get_object_by_name(&template).await?
                    .ok_or(WorldError::Other(anyhow!("Template '{}' not found", template)))?;

                let mut instance = GameObjectData::instantiate(&template.data);
                for pair in params.pairs::<String, mlua::Value>() {
                    let (key, value) = pair.map_err(WorldError::LuaError)?;

                    let attr = instance.class().get_attribute(&key)
                        .ok_or(mlua::Error::runtime("attribute not found"))
                        .map_err(WorldError::LuaError)?;

                    let value = ParamValue::from_lua(attr, value, &lua)
                        .map_err(WorldError::LuaError)?;

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

        Ok(())*/
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

/*pub(super) async fn load_additional_content(instance: &GameObjectData) -> WorldResult<(Abilities, Items)> {
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
}*/

/*pub(super) fn spawn_instance(
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
        Avatar {
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
}*/

pub(super) fn cleanup_dynamic_instances(
    instances: Query<(Entity, &SpawnState), (Changed<SpawnState>, With<DynamicInstance>)>,
    mut commands: Commands,
) {
    for (ent, state) in instances.iter() {
        if matches!(*state, SpawnState::Despawned(_)) {
            commands.entity(ent).despawn();
        }
    }
}

pub(super) fn avatar_despawner(
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