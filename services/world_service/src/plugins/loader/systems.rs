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

use std::str::FromStr;

use anyhow::anyhow;
use bevy::{ecs::{entity::Entity, hierarchy::ChildOf, message::{MessageReader, MessageWriter}, query::{Added, Changed, Or, With, Without}, system::{Commands, In, Query, Res}, world::World}, math::Vec3,time::{Time, Virtual}};
use log::debug;
use mlua::{Function, Lua, Table};
use obj_params::{Class, GameObjectData, NonClientBase, Player, tag_gameobject_entity, tags::{NpcBaseTag, NpcOtherlandTag, PlayerTag, StructureBaseTag}};
use scripting::{EntityScriptCommandsExt, LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{NativeParam, types::{AvatarId,  Uuid}};

use crate::{error::{WorldError, WorldResult}, plugins::{Active, Avatar, AvatarIdManager, ComponentLoaderCommandsTrait, ContentCacheRef, ContentInfo, DebugPlayer, DespawnAvatar, DynamicInstance, ForceSyncPositionUpdate, HealthUpdateEvent, MessageType, Movement, NonPlayerGameObjectLoader, NonPlayerGameObjectLoaderParams, ParamValue, PlayerController, SpawnState}};

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
    mut health_events: MessageWriter<HealthUpdateEvent>,
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
                    commands.write_message(DespawnAvatar(ent));
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
        In((owner, class, template, name, params, callback)): In<(Option<Table>, String, String, String, Table, Option<Function>)>,
        runtime: Res<LuaRuntime>,
        mut commands: Commands
    | -> WorldResult<()> {
        let lua = runtime.vm().clone();
        let class = Class::from_str(&class)?;
        let mut instance = GameObjectData::new_for_class(class);

        for pair in params.pairs::<String, mlua::Value>() {
            let (key, value) = pair.map_err(WorldError::LuaError)?;

            let attr = instance.class().get_attribute(&key)
                .ok_or(mlua::Error::runtime("attribute not found"))
                .map_err(WorldError::LuaError)?;

            let value = ParamValue::from_lua(attr, value, &lua)
                .map_err(WorldError::LuaError)?;

            instance.set_named(&key, value);
        }

        let owner = match owner {
            Some(t) => Some(t.entity()?),
            None => None,
        };

        commands
            .spawn_empty()
            .load_component_with_error_handler::<NonPlayerGameObjectLoader, _>(
                NonPlayerGameObjectLoaderParams::Dynamic { 
                    id: Uuid::new(), 
                    owner, 
                    name, 
                    template: ContentCacheRef::Name(template), 
                    data: instance, 
                    callback: callback.clone(), 
                }, 
                move |e, commands| {
                    if let Some(callback) = callback {
                        commands
                            .call_lua_method(callback, e.to_string());
                    }
                });

        Ok(())
    })?)?;

    loader_api.set("DespawnAvatar", lua.create_bevy_function(world, |
        In(ent): In<Table>,
        mut commands: Commands,
    | -> WorldResult<()> {
        let Ok(entity) = ent.entity() else {
            return Err(WorldError::Other(anyhow!("Invalid entity")));
        };

        commands.write_message(DespawnAvatar(entity));

        Ok(())
    })?)?;

    Ok(())
}

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
    mut events: MessageReader<DespawnAvatar>,
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