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

use anyhow::anyhow;
use bevy::ecs::{component::Component, lifecycle::HookContext, world::{DeferredWorld, World}};
use mlua::LuaSerdeExt;
use obj_params::GameObjectData;
use log::error;
use scripting::{EntityScriptCommandsExt, LuaRuntime, ScriptApi, ScriptCommandsExt, ScriptObject};

use crate::{error::WorldError, plugins::{SpawnCallback, insert_object_info, load_class_script}};

#[derive(Component)]
#[component(on_insert = on_scripted_inserted)]
pub struct Scripted;

fn on_scripted_inserted(mut world: DeferredWorld<'_>, ctx: HookContext) {
    world
        .commands()
        .queue(move |world: &mut World| {
            let spawn_callback = world
                .entity_mut(ctx.entity)
                .take::<SpawnCallback>();

            let script = world.resource_scope::<LuaRuntime, _>(|world, mut runtime| {
                let Some(object) = world
                    .entity(ctx.entity)
                    .get::<GameObjectData>()
                else {
                    return Err(WorldError::Other(anyhow!("Scripted component added to entity {:?} without GameObjectData", ctx.entity)));
                };

                load_class_script(runtime.as_mut(), object.class(), object.get_named::<String>("LuaScript").ok().map(|s| s.as_str()))
            });

            let runtime = world
                .get_resource::<LuaRuntime>()
                .unwrap()
                .vm()
                .clone();

            match script {
                    Ok(lua_class) => {
                        world
                            .commands()
                            .entity(ctx.entity)
                            .insert(ScriptObject::new(&runtime, Some(lua_class.clone())).unwrap())
                            .queue(insert_object_info)
                            .call_named_lua_method(ScriptApi::Attach, ());

                        if let Some(SpawnCallback(callback)) = spawn_callback {
                            world
                                .commands()
                                .call_lua_method(callback.clone(), (runtime.null(), lua_class));

                            world
                                .commands()
                                .entity(ctx.entity)
                                .remove::<SpawnCallback>();
                        }
                    },
                    Err(e) => {
                        error!("Failed to load script: {e}");
                    },
                }
        });
}