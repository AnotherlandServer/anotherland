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

use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use bevy::{app::{App, Plugin, PostUpdate, PreUpdate, Update}, ecs::{component::Component, entity::Entity, error::Result, message::MessageReader, query::{Added, Changed, With, Without}, system::{Commands, In, Query, Res}, world::World}, time::{Real, Stopwatch, Time, Virtual}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{debug, warn};
use mlua::Lua;
use obj_params::{GameObjectData, OaBuff2, ParamWriter};
use protocol::{CPktBuffRequest, CPktBuffUpdate};
use realm_api::ObjectTemplate;
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaExt, LuaRuntime, ScriptObject, ScriptResult};
use toolkit::types::Uuid;

use crate::{error::WorldResult, plugins::{AsyncOperationEntityCommandsExt, CombatEvent, CombatEventType, ContentCache, ContentCacheRef, Interruption, Kind, Scripted, WeakCache, player_error_handler_system}};

use super::{Avatar, ContentInfo, Interests, PlayerController};

pub struct BuffsPlugin;

impl Plugin for BuffsPlugin {
    fn build(&self, app: &mut App) {
        insert_buff_api(app.world_mut()).unwrap();

        app.add_systems(PreUpdate, insert_buff_info);
        app.add_systems(PostUpdate, send_buff_update);
        app.add_systems(Update, (tick_buffs, interrupt_buffs, process_combat_events));
        app.add_systems(PostUpdate, remove_buffs);
    }
}

#[derive(Component)]
#[relationship(relationship_target = Buffs)]
pub struct Buffing(pub Entity);

impl Buffing {
    pub fn target(&self) -> Entity {
        self.0
    }
}

#[derive(Component)]
#[relationship_target(relationship = Buffing, linked_spawn)]
pub struct Buffs(Vec<Entity>);

#[allow(clippy::type_complexity)]
pub fn insert_buff_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("buffs", object_api.clone()).unwrap();

    object_api.set("AddBuff", lua.create_bevy_function(world, |
        In((owner, instigator, id, duration, delay, stacks)): In<(LuaEntity, Option<LuaEntity>, String, Option<f32>, Option<f32>, Option<i32>)>,
        mut commands: Commands
    | -> WorldResult<String> {
        let instigator = instigator.map(LuaEntity::take);
        let id = id.parse()?;
        let instance_id = Uuid::new();

        commands
            .entity(owner.entity())
            .perform_async_operation(async move {
                Ok((
                    instigator,
                    ContentCache::get(
                        &ContentCacheRef::Uuid(id)
                    ).await,
                    instance_id,
                    duration,
                    delay,
                    stacks
                ))
            })
            .on_finish_run_system(insert_buff)
            .on_error_run_system(player_error_handler_system);

        Ok(instance_id.to_string())
    })?)?;

    object_api.set("AddBuffByName", lua.create_bevy_function(world, |
        In((owner, instigator, name, duration, delay, stacks)): In<(LuaEntity, Option<LuaEntity>, String, Option<f32>, Option<f32>, Option<i32>)>,
        mut commands: Commands
    | -> WorldResult<String> {
        let instigator = instigator.map(LuaEntity::take);
        let instance_id = Uuid::new();

        commands
            .entity(owner.entity())
            .perform_async_operation(async move {
                Ok((
                    instigator,
                    ContentCache::get(
                        &ContentCacheRef::Name(name)
                    ).await,
                    instance_id,
                    duration,
                    delay,
                    stacks,
                ))
            })
            .on_finish_run_system(insert_buff)
            .on_error_run_system(player_error_handler_system);

        Ok(instance_id.to_string())
    })?)?;

    object_api.set("RemoveBuff", lua.create_bevy_function(world, |
        In((owner, reference_type, id)): In<(LuaEntity, String, String)>,
        buffs: Query<&Buffs>,
        buff: Query<(Entity, &ContentInfo), With<Buff>>,
        mut commands: Commands
    | -> WorldResult<bool> {
        let uuid = id.parse::<Uuid>().ok();

        for child in buffs.iter_descendants(owner.entity()) {
            if 
                let Ok((ent, content)) = buff.get(child) &&
                match reference_type.as_str() {
                    "Template" => Some(content.template.id) == uuid,
                    "Instance" => Some(content.placement_id) == uuid,
                    "Name" => content.template.name == id,
                    _ => return Err(anyhow!("Invalid reference type: {}", reference_type).into()),
                }
            {
                debug!("Removing buff {}:{} from {ent:?}", content.placement_id, content.template.id);

                commands
                    .entity(ent)
                    .insert(BuffExpired);

                return Ok(true);
            }
        }

        Ok(false)
    })?)?;

    object_api.set("HasBuff", lua.create_bevy_function(world, |
        In((owner, reference_type, id)): In<(LuaEntity, String, String)>,
        buffs: Query<&Buffs>,
        buff: Query<&ContentInfo, With<Buff>>,
    | -> WorldResult<bool> {
        let uuid = id.parse::<Uuid>().ok();

        for child in buffs.iter_descendants(owner.entity()) {
            if 
                let Ok(content) = buff.get(child) &&
                match reference_type.as_str() {
                    "Template" => Some(content.template.id) == uuid,
                    "Instance" => Some(content.placement_id) == uuid,
                    "Name" => content.template.name == id,
                    _ => return Err(anyhow!("Invalid reference type: {}", reference_type).into()),
                }
            {
                return Ok(true);
            }
        }

        Ok(false)
    })?)?;

    Ok(())
}

#[allow(clippy::type_complexity)]
fn insert_buff(
    In((ent, (instigator, res, instance_id, duration, _delay, stacks))): In<(Entity, (Option<Entity>, Result<Option<Arc<ObjectTemplate>>>, Uuid, Option<f32>, Option<f32>, Option<i32>))>,
    query: Query<&Avatar>,
    time: Res<Time<Real>>,
    mut commands: Commands,
) {
    if let Ok(Some(template)) = res {
        debug!("Inserting buff {ent:?} with template {}", template.id);

        let mut data = GameObjectData::instantiate(template.clone());
        
        let mut buff = Buff {
            added: Stopwatch::new(),
            duration: None,
            interval: None,
            tick: Stopwatch::new(),
            instigator,
        };

        if 
            let Ok(tick_period) = data.get::<_, f32>(OaBuff2::TickPeriod).cloned() && 
            tick_period > 0.0 
        {
            buff.interval = Some(Duration::from_secs_f32(tick_period));
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
                },
                Buffing(ent),
                Scripted,
            ));
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
    query: Query<(&ContentInfo, &GameObjectData, &Buffing), (With<Buff>, Changed<GameObjectData>)>,
    avatar_query: Query<&Avatar>,
    players: Query<(Entity, &Interests, &PlayerController)>,
) {
    for (content, obj, buff_of) in query.iter() {
        let mut param_buffer = Vec::new();
        let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

        obj//.as_set()
            //.client_params()
            .write_to_client(&mut writer)
            .expect("failed to serialize params");

        if let Ok(avatar) = avatar_query.get(buff_of.target()) {
            debug!("Buff update for {:?}", buff_of.target());

            for (ent, interests, controller) in players.iter() {
                debug!("Checking interests for {ent:?}");

                if buff_of.target() == ent || interests.contains(&buff_of.target()) {
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
    query: Query<(&Buff, &Buffing, &ScriptObject), Added<ScriptObject>>,
) {
    for (buff, buff_of, script) in query.iter() {
        debug!("Add buff");

        script.object().set("target", LuaEntity(buff_of.target())).unwrap();
        script.object().set("instigator", buff.instigator.map(LuaEntity)).unwrap();
    }
}

#[allow(clippy::type_complexity)]
fn tick_buffs(
    mut query: Query<(Entity, &mut Buff, &mut GameObjectData), Without<BuffExpired>>,
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
        
        if 
            let Some(duration) = buff.duration &&
            buff.added.elapsed() >= duration 
        {
            commands
                .entity(ent)
                .insert(BuffExpired);
        }
    }
}

#[allow(clippy::type_complexity)]
fn remove_buffs(
    query: Query<(Entity, &ContentInfo, &Buffing), With<BuffExpired>>,
    players: Query<(Entity, &Interests, &PlayerController)>,
    avatar_query: Query<&Avatar>,
    mut commands: Commands,
) {
    for (ent, content, buff_of) in query.iter() {
        commands
            .entity(ent)
            .call_named_lua_method("Detach", ());
        
        commands
            .entity(ent)
            .despawn();

        if let Ok(avatar) = avatar_query.get(buff_of.target()) {
            for (ent, interests, controller) in players.iter() {
                if buff_of.target() == ent || interests.contains(&buff_of.target()) {
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

fn interrupt_buffs(
    mut messages: MessageReader<Interruption>,
    buffs: Query<&Buffs>,
    buff: Query<(Entity, &GameObjectData), With<Buff>>,
    mut commands: Commands,
) {
    for msg in messages.read() {
        for buff_ent in buffs.iter_descendants(msg.target) {
            let Ok((ent, content)) = buff.get(buff_ent) else {
                continue;
            };

            let destroy_on_get_hit = content.get::<_, bool>(OaBuff2::DestroyOnGetHit).unwrap();
            let destroy_on_critical_hit = content.get::<_, bool>(OaBuff2::DestroyOnCriticalHit).unwrap();
            let destroy_on_move = content.get::<_, bool>(OaBuff2::DestroyOnMove).unwrap();
            let destroy_on_owner_die = content.get::<_, bool>(OaBuff2::DestroyOnOwnerDied).unwrap();

            if 
                *destroy_on_get_hit && matches!(msg.kind, Kind::Damage) ||
                *destroy_on_critical_hit && matches!(msg.kind, Kind::DamageCritical) ||
                *destroy_on_move && matches!(msg.kind, Kind::Movement) ||
                *destroy_on_owner_die && matches!(msg.kind, Kind::Death)
            {
                commands
                    .entity(ent)
                    .insert(BuffExpired);
            }
        }
    }
}

fn process_combat_events(
    mut message: MessageReader<CombatEvent>,
    buffs: Query<&Buffs>,
    buff_iter: Query<(Entity, &Buff)>,
    mut commands: Commands,
) {
    for event in message.read() {
        for buff_ent in buffs.iter_descendants(event.target) {
            match event.update {
                CombatEventType::Damaged(amount) => 
                    commands
                        .entity(buff_ent)
                        .call_named_lua_method("OnOwnerDamaged", (event.instigator.map(LuaEntity), amount)),
                CombatEventType::Healed(amount) => 
                    commands
                        .entity(buff_ent)
                        .call_named_lua_method("OnOwnerHealed", (event.instigator.map(LuaEntity), amount)),
                CombatEventType::Death =>
                    commands
                        .entity(buff_ent)
                        .call_named_lua_method("OnOwnerDeath", event.instigator.map(LuaEntity)),
                CombatEventType::Revived =>
                    commands
                        .entity(buff_ent)
                        .call_named_lua_method("OnOwnerRevived", event.instigator.map(LuaEntity)),
                _ => continue,
            };
        }

        if matches!(event.update, CombatEventType::Death) {
            for (ent, buff) in buff_iter.iter() {
                if buff.instigator != Some(event.target) {
                    continue;
                }
                
                commands
                    .entity(ent)
                    .call_named_lua_method("OnInstigatorDeath", event.instigator.map(LuaEntity));
            }
        }
    }
}