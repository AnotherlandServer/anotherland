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

use bevy::{ecs::{query::With, system::{In, Query, Res}, world::World}, time::{Time, Virtual}};
use mlua::{IntoLua, Lua, Table, Value};
use scripting::{LuaExt, LuaRuntime, ScriptObject, ScriptResult};
use toolkit::types::AvatarId;
use anyhow::anyhow;

use crate::{error::WorldResult, instance::WorldController, plugins::{AvatarIdManager, Avatar, ContentInfo, InstanceManager}};

pub fn insert_world_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("world", object_api.clone()).unwrap();

    lua.globals().set("NULL_AVATAR_ID", AvatarId::default())?;

    object_api.set("GetWorld", lua.create_bevy_function(world, |
        In(()): In<()>,
        query: Query<&ScriptObject, With<WorldController>>,
    | -> WorldResult<Table> {
        let obj = query.single()
            .map_err(|_| anyhow!("No world controller found"))?;
        Ok(obj.object().clone())
    })?)?;

    object_api.set("GetEntityByAvatarId", lua.create_bevy_function(world, |
        In(id): In<AvatarId>,
        runtime: Res<LuaRuntime>,
        avatar_manager: Res<AvatarIdManager>,
        query: Query<&ScriptObject>,
    | -> WorldResult<Value> {
        if 
            let Some(entity_id) = avatar_manager.resolve_avatar_id(id) &&
            let Ok(obj) = query.get(entity_id)
        {
            Ok(obj.object().clone().into_lua(runtime.vm())?)
        } else {
            Ok(Value::Nil)
        }
    })?)?;

    object_api.set("GetEntityById", lua.create_bevy_function(world, |
        In(id): In<String>,
        runtime: Res<LuaRuntime>,
        instance_manager: Res<InstanceManager>,
        query: Query<&ScriptObject>,
    | -> WorldResult<Value> {
        if 
            let Some(entity_id) = instance_manager.find_instance(id.parse()?) &&
            let Ok(obj) = query.get(entity_id)
        {
            Ok(obj.object().clone().into_lua(runtime.vm())?)
        } else {
            Ok(Value::Nil)
        }
    })?)?;

    object_api.set("GetEntityByName", lua.create_bevy_function(world, |
        In(name): In<String>,
        runtime: Res<LuaRuntime>,
        query: Query<(&Avatar, &ScriptObject)>,
    | -> WorldResult<Value> {
        for (obj, entity) in query.iter() {
            if obj.name == name {
                return Ok(entity.object().clone().into_lua(runtime.vm())?);
            }
        }

        Ok(Value::Nil)
    })?)?;

    object_api.set("FindEntitiesByTemplateId", lua.create_bevy_function(world, |
        In(template_id): In<String>,
        runtime: Res<LuaRuntime>,
        query: Query<(&ContentInfo, &ScriptObject)>,
    | -> WorldResult<Table> {
        let result = runtime.vm().create_table()?;
        let template_id = template_id.parse()?;

        for (info, obj) in query.iter() {
            if info.template.id == template_id {
                result.push(obj.object().clone())?;
            }
        }

        Ok(result)
    })?)?;

    object_api.set("FindEntitiesByClass", lua.create_bevy_function(world, |
        In(class): In<String>,
        runtime: Res<LuaRuntime>,
        query: Query<(&ContentInfo, &ScriptObject)>,
    | -> WorldResult<Table> {
        let result: Table = runtime.vm().create_table()?;
        let class = class.parse()?;

        for (info, obj) in query.iter() {
            if info.template.class == class {
                result.push(obj.object().clone())?;
            }
        }

        Ok(result)
    })?)?;
    
    object_api.set("GetCurrentTime", lua.create_bevy_function(world, |
        timer: Res<Time<Virtual>>,
    | -> WorldResult<f32> {
        Ok(timer.elapsed_secs())
    })?)?;

    Ok(())
}