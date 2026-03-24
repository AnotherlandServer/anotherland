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

use bevy::{ecs::{entity::Entity, query::With, system::{In, Query, Res}, world::World}, time::{Time, Virtual}};
use mlua::Lua;
use scripting::{LuaEntity, LuaExt, LuaRuntime, ScriptResult};
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
        query: Query<Entity, With<WorldController>>,
    | -> WorldResult<LuaEntity> {
        let obj = query.single()
            .map_err(|_| anyhow!("No world controller found"))?;
        Ok(LuaEntity(obj))
    })?)?;

    object_api.set("GetEntityByAvatarId", lua.create_bevy_function(world, |
        In(id): In<AvatarId>,
        avatar_manager: Res<AvatarIdManager>,
    | -> WorldResult<Option<LuaEntity>> {
        if let Some(entity_id) = avatar_manager.resolve_avatar_id(id) {
            Ok(Some(LuaEntity(entity_id)))
        } else {
            Ok(None)
        }
    })?)?;

    object_api.set("GetEntityById", lua.create_bevy_function(world, |
        In(id): In<String>,
        instance_manager: Res<InstanceManager>,
    | -> WorldResult<Option<LuaEntity>> {
        if let Some(entity_id) = instance_manager.find_instance(id.parse()?) {
            Ok(Some(LuaEntity(entity_id)))
        } else {
            Ok(None)
        }
    })?)?;

    object_api.set("GetEntityByName", lua.create_bevy_function(world, |
        In(name): In<String>,
        query: Query<(Entity, &Avatar)>,
    | -> WorldResult<Option<LuaEntity>> {
        for (entity, avatar) in query.iter() {
            if avatar.name == name {
                return Ok(Some(LuaEntity(entity)));
            }
        }

        Ok(None)
    })?)?;

    object_api.set("FindEntitiesByTemplateId", lua.create_bevy_function(world, |
        In(template_id): In<String>,
        query: Query<(Entity, &ContentInfo)>,
    | -> WorldResult<Vec<LuaEntity>> {
        let mut result = vec![];
        let template_id = template_id.parse()?;

        for (entity, info) in query.iter() {
            if info.template.id == template_id {
                result.push(LuaEntity(entity));
            }
        }

        Ok(result)
    })?)?;

    object_api.set("FindEntitiesByClass", lua.create_bevy_function(world, |
        In(class): In<String>,
        query: Query<(Entity, &ContentInfo)>,
    | -> WorldResult<Vec<LuaEntity>> {
        let mut result = vec![];
        let class = class.parse()?;

        for (entity, info) in query.iter() {
            if info.template.class == class {
                result.push(LuaEntity(entity));

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