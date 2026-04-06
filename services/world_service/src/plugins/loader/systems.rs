// Copyright (C) 2026 AnotherlandServer
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
use bevy::{app::App, ecs::{entity::Entity, event::EntityEvent, hierarchy::ChildOf, message::MessageWriter, observer::On, query::{Changed, Has, Or, With, Without}, system::{Commands, In, Query, Res}, world::EntityWorldMut}, math::Vec3, time::{Time, Virtual}};
use log::debug;
use mlua::{Function, Table};
use obj_params::{Class, GameObjectData, NonClientBase, Player, tags::{NonClientBaseTag, NpcBaseTag, NpcOtherlandTag, PlayerTag, StructureBaseTag}};
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaRuntime, LuaTableExt, ScriptAppExt};
use toolkit::{NativeParam, types::{AvatarId,  Uuid}};

use crate::{error::{WorldError, WorldResult}, plugins::{Active, Avatar, AvatarIdManager, ComponentLoaderCommandsTrait, ContentCacheRef, ContentInfo, DebugNpc, DebugPlayer, DespawnAvatar, DynamicInstance, ForceSyncPositionUpdate, HealthUpdateRequest, MessageType, Movement, NonPlayerGameObjectLoader, NonPlayerGameObjectLoaderParams, ParamValue, PlayerController, RecalculateAttributes, RemoveObject, ScriptingEntityCommandsExt, SpawnAvatar, SpawnState}};

#[allow(clippy::type_complexity)]
pub fn update_spawn_state(
    mut entities: Query<(Entity, &GameObjectData, &mut SpawnState, &mut Movement), Or<(With<NpcBaseTag>, With<StructureBaseTag>)>>,
    mut health_events: MessageWriter<HealthUpdateRequest>,
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
                    commands
                        .entity(ent)
                        .trigger(DespawnAvatar);
                }
            },
            SpawnState::Despawned(instant) => {
                let respawn_delay = *obj.get::<_, f32>(NonClientBase::RespawnDelay).unwrap();

                if instant.elapsed().as_secs_f32() >= respawn_delay {
                    debug!("Respawning entity {ent}");

                    let obj = GameObjectData::instantiate(obj.parent().unwrap());

                    // This should be handled as event
                    movement.position = *obj.get::<_, Vec3>(NonClientBase::Pos).unwrap();

                    health_events.write(HealthUpdateRequest::revive(ent, HealthUpdateRequest::next_id(), None, None, None));
                    state.mark_alive();
                    commands.entity(ent)
                        .insert(obj)
                        .trigger(SpawnAvatar);
                }
            },
        }
    }
}

pub fn spawn_init_entity(
    event: On<SpawnAvatar>,
    has_tag: Query<Has<NonClientBaseTag>>,
    mut entities: Query<&mut GameObjectData>,
    mut commands: Commands,
) {
    if !has_tag.get(event.event_target()).unwrap_or(false) {
        return;
    }

    let Ok(mut obj) = entities.get_mut(event.event_target()) else {
        return;
    };

    let hp_mod = obj.get_named::<f32>("HpMod").cloned().unwrap_or(1.0);
    let hp_max = obj.get_named::<i32>("hpMax").cloned().unwrap_or(0);
    let bonus_hp = obj.get_named::<f32>("BonusHP").cloned().unwrap_or(0.0);

    obj.set_named("alive", true);
    obj.set_named("hpMax", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);
    obj.set_named("hpCur", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);

    commands.entity(event.event_target())
            .trigger(RecalculateAttributes)
            .insert(Active)
            .fire_lua_event("Spawned", ());
}

#[allow(clippy::type_complexity)]
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
    In((ent, _args)): In<(Entity, Vec<NativeParam>)>,
    player: Query<(&GameObjectData, &PlayerController), With<PlayerTag>>,
    avatars: Query<(&Avatar, &ContentInfo, Option<&DebugNpc>, Option<&ChildOf>)>,
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

    let Ok((avatar_info, content_info, debug_npc, child_of)) = avatars.get(target_ent) else {
        return;
    };

    let Ok((avatar_info, content_info, _, _)) = 
        (match (debug_npc, child_of) {
            (Some(_), Some(parent)) => avatars.get(parent.parent()),
            _ => Ok((avatar_info, content_info, None, None)),
        }) else {
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
pub fn insert_loader_api(app: &mut App) {
    app
        .add_lua_api("loader", "RequestSpawnInstance",
        |
        In((owner, class, template, name, params, callback)): In<(Option<LuaEntity>, String, String, String, Table, Option<Function>)>,
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

            commands
                .spawn_empty()
                .load_component_with_error_handler::<NonPlayerGameObjectLoader, _>(
                    NonPlayerGameObjectLoaderParams::Dynamic { 
                        id: Uuid::new(), 
                        owner: owner.map(LuaEntity::take), 
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
        })
        .add_lua_api("loader", "DespawnAvatar",
        |
            In(ent): In<Table>,
            mut commands: Commands,
        | -> WorldResult<()> {
            let Ok(entity) = ent.entity() else {
                return Err(WorldError::Other(anyhow!("Invalid entity")));
            };

            commands
                .entity(entity)
                .trigger(DespawnAvatar);

            Ok(())
        });
}

#[allow(clippy::type_complexity)]
pub(super) fn cleanup_dynamic_instances(
    instances: Query<(Entity, &SpawnState), (Changed<SpawnState>, With<DynamicInstance>)>,
    mut commands: Commands,
) {
    for (ent, state) in instances.iter() {
        if matches!(*state, SpawnState::Despawned(_)) {
            commands.entity(ent).deferred_despawn();
        }
    }
}

pub(super) fn avatar_despawner(
    event: On<DespawnAvatar>,
    mut query: Query<&mut SpawnState>,
    mut commands: Commands,
) {
    if 
        let Ok(mut state) = query.get_mut(event.event_target()) &&
        !matches!(*state, SpawnState::Despawned(_))
    {
        debug!("Despawning entity {}", event.event_target());

        state.mark_despawned();
        commands.entity(event.event_target())
            .fire_lua_event("Despawned", ())
            .remove::<Active>();
    }
}

pub fn on_remove_object(
    event: On<RemoveObject>,
    mut commands: Commands,
) {
    commands
        .entity(event.event_target())
        .remove::<Active>()
        .deferred_despawn();
}