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

use bevy::{app::App, ecs::{entity::Entity, query::With, system::{In, Query, Res}}, time::{Time, Virtual}};
use mlua::Lua;
use scripting::{LuaEntity, LuaRuntime, ScriptAppExt};
use toolkit::types::AvatarId;
use anyhow::anyhow;

use crate::{error::WorldResult, instance::WorldController, plugins::{AvatarIdManager, Avatar, ContentInfo, InstanceManager}};

pub fn insert_world_api(app: &mut App) {
    let runtime = app.world_mut().get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    lua.globals().set("NULL_AVATAR_ID", AvatarId::default())
        .expect("Failed to set NULL_AVATAR_ID");

    app
        .add_lua_api("world", "GetWorld",
        |
            In(()): In<()>,
            query: Query<Entity, With<WorldController>>,
        | -> WorldResult<LuaEntity> {
            let obj = query.single()
                .map_err(|_| anyhow!("No world controller found"))?;
            Ok(LuaEntity(obj))
        })
        .add_lua_api("world", "GetEntityByAvatarId", 
        |
            In(id): In<AvatarId>,
            avatar_manager: Res<AvatarIdManager>,
        | -> WorldResult<Option<LuaEntity>> {
            if let Some(entity_id) = avatar_manager.resolve_avatar_id(id) {
                Ok(Some(LuaEntity(entity_id)))
            } else {
                Ok(None)
            }
        })
        .add_lua_api("world", "GetEntityById", 
        |
            In(id): In<String>,
            instance_manager: Res<InstanceManager>,
        | -> WorldResult<Option<LuaEntity>> {
            if let Some(entity_id) = instance_manager.find_instance(id.parse()?) {
                Ok(Some(LuaEntity(entity_id)))
            } else {
                Ok(None)
            }
        })
        .add_lua_api("world", "GetEntityByName", 
        |
            In(name): In<String>,
            query: Query<(Entity, &Avatar)>,
        | -> WorldResult<Option<LuaEntity>> {
            for (entity, avatar) in query.iter() {
                if avatar.name == name {
                    return Ok(Some(LuaEntity(entity)));
                }
            }

            Ok(None)
        })
        .add_lua_api("world", "FindEntitiesByTemplateId", 
        |
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
        })
        .add_lua_api("world", "FindEntitiesByClass", 
        |
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
        })
        .add_lua_api("world", "GetCurrentTime", 
        |
            timer: Res<Time<Virtual>>,
        | -> WorldResult<f32> {
            Ok(timer.elapsed_secs())
        });
}