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

use bevy::{app::{Plugin, PreUpdate}, prelude::{Added, App, Commands, Entity, EntityWorldMut, Query, ResMut}};
use log::error;
use obj_params::{Class, GameObjectData, NonClientBase};
use scripting::{ApiType, LuaRuntime, ScriptApi, ScriptCommandsExt, Scripted};


use super::{AvatarInfo, ContentInfo};

pub struct ScriptObjectInfoPlugin;

impl Plugin for ScriptObjectInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, attach_scripts);
    }
}

pub fn attach_scripts(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut runtime: ResMut<LuaRuntime>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        if 
            let Ok(script) = obj.get::<_, String>(NonClientBase::LuaScript) &&
            !script.is_empty()
        {
            let api_type = match obj.class() {
                Class::NpcBase => ApiType::Npc,
                Class::NpcOtherland => ApiType::Npc,
                _ => ApiType::Script,
            };

            match runtime.create_scripted_entity(api_type, script) {
                Ok(script) => {
                    commands.entity(ent)
                        .insert(script)
                        .queue(insert_avatar_info)
                        .call_named_lua_method(ScriptApi::Attach, ());
                },
                Err(e) => { error!("Failed to create script '{}': {}", script, e); }
            }
        } else if obj.class() == Class::Player {
            match runtime.create_scripted_player() {
                Ok(script) => {
                    commands.entity(ent)
                        .insert(script)
                        .queue(insert_avatar_info)
                        .call_named_lua_method(ScriptApi::Attach, ());
                },
                Err(e) => { error!("Failed to create scripted player: {}", e); }
            }
        } else if obj.class() == Class::NpcOtherland { // Generic NPC script
            match runtime.create_scripted_entity(ApiType::Npc, ApiType::Npc.base()) {
                Ok(script) => {
                    commands.entity(ent)
                        .insert(script)
                        .queue(insert_avatar_info)
                        .call_named_lua_method(ScriptApi::Attach, ());
                },
                Err(e) => { error!("Failed to create script '{}': {}", ApiType::Npc.base(), e); }
            }
        }
    }
}

fn insert_avatar_info(entity: EntityWorldMut<'_>) {
    let script = entity.get::<Scripted>().unwrap();
    let avatar_info = entity.get::<AvatarInfo>().unwrap();
    let object = entity.get::<GameObjectData>().unwrap();
    let content_info = entity.get::<ContentInfo>();

    script.script().raw_set("avatar_id", avatar_info.id.as_u64()).unwrap();
    script.script().raw_set("name", avatar_info.name.clone()).unwrap();
    script.script().raw_set("class", object.class().name()).unwrap();

    if let Some(content_info) = content_info {
        script.script().raw_set("placement_guid", content_info.placement_id.to_string()).unwrap();
        script.script().raw_set("template_guid", content_info.template.id.to_string()).unwrap();
    }
}