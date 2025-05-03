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

use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use bevy::{app::{App, Plugin, PostUpdate, PreUpdate, Update}, ecs::{component::Component, entity::Entity, query::{Added, Changed, With, Without}, schedule::IntoSystemConfigs, system::{Commands, In, Query, Res, Resource, SystemId}, world::World}, hierarchy::{BuildChildren, Children, DespawnRecursiveExt, HierarchyQueryExt, Parent}, time::{Real, Stopwatch, Time, Virtual}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{debug, warn};
use mlua::{Lua, Table};
use obj_params::{GameObjectData, OaBuff2, ParamWriter};
use protocol::{CPktBuffRequest, CPktBuffUpdate};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use toolkit::types::Uuid;

use crate::{error::WorldResult, object_cache::CacheEntry, plugins::FutureCommands, OBJECT_CACHE};

use super::{attach_scripts, AvatarInfo, ContentInfo, Interests, PlayerController};

#[derive(Resource)]
#[allow(clippy::type_complexity)]
struct BuffSystems {
    insert_buff: SystemId<In<(Entity, Option<Entity>, WorldResult<Option<Arc<CacheEntry>>>, Uuid, Option<f32>, Option<f32>, Option<i32>)>>,
}

pub struct BuffsPlugin;

impl Plugin for BuffsPlugin {
    fn build(&self, app: &mut App) {
        let buff_systems = BuffSystems {
            insert_buff: app.register_system(insert_buff),
        };

        app.insert_resource(buff_systems);

        insert_buff_api(app.world_mut()).unwrap();

        app.add_systems(PreUpdate, insert_buff_info.after(attach_scripts));
        app.add_systems(PostUpdate, send_buff_update);
        app.add_systems(Update, update_buffs);
        app.add_systems(PostUpdate, remove_buffs);
    }
}

#[allow(clippy::type_complexity)]
pub fn insert_buff_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("buffs", object_api.clone()).unwrap();

    object_api.set("AddBuff", lua.create_bevy_function(world, |
        In((owner, instigator, id, duration, delay, stacks)): In<(Table, Option<Table>, String, Option<f32>, Option<f32>, Option<i32>)>,
        systems: Res<BuffSystems>,
        mut commands: Commands
    | -> WorldResult<String> {
        let ent = owner.entity()?;
        let instigator = instigator.map(|t| t.entity()).transpose()?;
        let id = id.parse()?;
        let instance_id = Uuid::new();

        commands.run_system_async(async move {
            (
                ent,
                instigator,
                OBJECT_CACHE.wait().get_object_by_guid(id).await,
                instance_id,
                duration,
                delay,
                stacks,
            )
        }, systems.insert_buff);

        Ok(instance_id.to_string())
    })?)?;

    object_api.set("AddBuffByName", lua.create_bevy_function(world, |
        In((owner, instigator, name, duration, delay, stacks)): In<(Table, Option<Table>, String, Option<f32>, Option<f32>, Option<i32>)>,
        systems: Res<BuffSystems>,
        mut commands: Commands
    | -> WorldResult<String> {
        let ent = owner.entity()?;
        let instigator = instigator.map(|t| t.entity()).transpose()?;
        let instance_id = Uuid::new();

        commands.run_system_async(async move {
            (
                ent,
                instigator,
                OBJECT_CACHE.wait().get_object_by_name(&name).await,
                instance_id,
                duration,
                delay,
                stacks,
            )
        }, systems.insert_buff);

        Ok(instance_id.to_string())
    })?)?;

    object_api.set("RemoveBuff", lua.create_bevy_function(world, |
        In((owner, reference_type, id)): In<(Table, String, String)>,
        children: Query<&Children>,
        buffs: Query<(Entity, &ContentInfo), With<Buff>>,
        mut commands: Commands
    | -> WorldResult<bool> {
        let uuid = id.parse::<Uuid>().ok();
        let owner = owner.entity()?;

        for child in children.iter_descendants(owner) {
            if let Ok((ent, content)) = buffs.get(child) {
                if match reference_type.as_str() {
                    "Template" => Some(content.template.id) == uuid,
                    "Instance" => Some(content.placement_id) == uuid,
                    "Name" => content.template.name == id,
                    _ => return Err(anyhow!("Invalid reference type: {}", reference_type).into()),
                } {
                    debug!("Removing buff {}:{} from {ent:?}", content.placement_id, content.template.id);

                    commands
                        .entity(ent)
                        .insert(BuffExpired);

                    return Ok(true);
                }
            }
        }

        Ok(false)
    })?)?;

    object_api.set("HasBuff", lua.create_bevy_function(world, |
        In((owner, reference_type, id)): In<(Table, String, String)>,
        children: Query<&Children>,
        buffs: Query<&ContentInfo, With<Buff>>,
    | -> WorldResult<bool> {
        let uuid = id.parse::<Uuid>().ok();
        let owner = owner.entity()?;

        for child in children.iter_descendants(owner) {
            if let Ok(content) = buffs.get(child) {
                if match reference_type.as_str() {
                    "Template" => Some(content.template.id) == uuid,
                    "Instance" => Some(content.placement_id) == uuid,
                    "Name" => content.template.name == id,
                    _ => return Err(anyhow!("Invalid reference type: {}", reference_type).into()),
                } {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    })?)?;

    Ok(())
}

#[allow(clippy::type_complexity)]
fn insert_buff(
    In((ent, instigator, res, instance_id, duration, _delay, stacks)): In<(Entity, Option<Entity>, WorldResult<Option<Arc<CacheEntry>>>, Uuid, Option<f32>, Option<f32>, Option<i32>)>,
    query: Query<&AvatarInfo>,
    time: Res<Time<Real>>,
    mut commands: Commands,
) {
    if let Ok(Some(template)) = res {
        debug!("Inserting buff {ent:?} with template {}", template.id);

        let mut data = GameObjectData::instantiate(&template.data);
        
        let mut buff = Buff {
            added: Stopwatch::new(),
            duration: None,
            interval: None,
            tick: Stopwatch::new(),
            instigator,
        };

        if let Ok(tick_period) = data.get::<_, f32>(OaBuff2::TickPeriod) {
            buff.interval = Some(Duration::from_secs_f32(*tick_period));
        }

        if let Some(duration) = duration {
            buff.duration = Some(Duration::from_secs_f32(duration));
            data.set(OaBuff2::Lifespan, duration);
        } else if let Ok(duration) = data.get::<_, f32>(OaBuff2::Lifespan) {
            buff.duration = Some(Duration::from_secs_f32(*duration));
        }

        if let Some(stacks) = stacks {
            data.set(OaBuff2::StackCount, stacks);
        }

        if 
            let Some(instigator) = instigator &&
            let Ok(avatar) = query.get(instigator) 
        {
            data.set(OaBuff2::Instigator, avatar.id);
            if ent == instigator {
                data.set(OaBuff2::InstigatorIsSource, true);
            }
        }

        data.set(OaBuff2::CreationTime, time.elapsed_secs());
        data.set(OaBuff2::DurationLeft, *data.get::<_, f32>(OaBuff2::Lifespan).unwrap());


        commands
            .spawn((
                data,
                buff,
                ContentInfo {
                    placement_id: instance_id,
                    template,
                }
            ))
            .set_parent(ent);
    } else {
        warn!("Buff template not found for entity {ent:?}");
    }
}

#[derive(Component)]
pub struct Buff {
    added: Stopwatch,
    duration: Option<Duration>,
    interval: Option<Duration>,
    tick: Stopwatch,
    instigator: Option<Entity>,
}

#[derive(Component)]
pub struct BuffExpired;

#[allow(clippy::type_complexity)]
fn send_buff_update(
    query: Query<(&ContentInfo, &GameObjectData, &Parent), (With<Buff>, Changed<GameObjectData>)>,
    avatar_query: Query<&AvatarInfo>,
    players: Query<(Entity, &Interests, &PlayerController)>,
) {
    for (content, obj, owner) in query.iter() {
        let mut param_buffer = Vec::new();
        let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

        obj//.as_set()
            //.client_params()
            .write_to_client(&mut writer)
            .expect("failed to serialize params");

        if let Ok(avatar) = avatar_query.get(owner.get()) {
            debug!("Buff update for {:?}", owner.get());

            for (ent, interests, controller) in players.iter() {
                debug!("Checking interests for {ent:?}");

                if owner.get() == ent || interests.contains_key(&owner.get()) {
                    debug!("Sending buff update to {ent:?}");

                    controller.send_packet(CPktBuffUpdate {
                        avatar_id: avatar.id,
                        instance_id: content.placement_id,
                        has_template: true,
                        content_id: Some(content.template.id),
                        class_id: content.template.class.id() as i32,
                        params: param_buffer.clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn insert_buff_info(
    query: Query<(&Buff, &Parent, &ScriptObject), (With<Buff>, Added<ScriptObject>)>,
    objects: Query<&ScriptObject>,
) {
    for (buff, owner, script) in query.iter() {
        debug!("Add buff");

        if let Ok(target_obj) = objects.get(owner.get()) {
            script.object().set("target", target_obj.object()).unwrap();
        }

        if let Some(instigator) = buff.instigator {
            if let Ok(target_obj) = objects.get(instigator) {
                script.object().set("instigator", target_obj.object()).unwrap();
            }
        } else if let Ok(target_obj) = objects.get(owner.get()) {
            script.object().set("target", target_obj.object()).unwrap();
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_buffs(
    mut query: Query<(Entity, &mut Buff, &mut GameObjectData), (With<Buff>, Without<BuffExpired>)>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
) {
    for (ent, mut buff, mut obj) in query.iter_mut() {
        buff.added.tick(time.delta());
        buff.tick.tick(time.delta());

        if 
            let Some(interval) = buff.interval &&
            buff.tick.elapsed() >= interval
        {
            buff.tick.reset();

            if let Some(duration) = buff.duration {
                obj.set(OaBuff2::DurationLeft, (duration.as_secs_f32() - buff.added.elapsed().as_secs_f32()).max(0.0));
            }

            commands
                .entity(ent)
                .call_named_lua_method("Tick", ());
        }
        
        if let Some(duration) = buff.duration {
            if buff.added.elapsed() >= duration {
                commands
                    .entity(ent)
                    .insert(BuffExpired);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn remove_buffs(
    query: Query<(Entity, &ContentInfo, &Parent), (With<Buff>, With<BuffExpired>)>,
    players: Query<(Entity, &Interests, &PlayerController)>,
    avatar_query: Query<&AvatarInfo>,
    mut commands: Commands,
) {
    for (ent, content, owner) in query.iter() {
        commands
            .entity(ent)
            .call_named_lua_method("Detach", ());
        
        commands
            .entity(ent)
            .despawn_recursive();

        if let Ok(avatar) = avatar_query.get(owner.get()) {
            for (ent, interests, controller) in players.iter() {
                if owner.get() == ent || interests.contains_key(&owner.get()) {
                    controller.send_packet(CPktBuffRequest {
                        avatar_id: avatar.id,
                        instance_id: content.placement_id,
                        ..Default::default()
                    });
                }
            }
        }
    }
}