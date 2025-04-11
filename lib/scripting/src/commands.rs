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

use anyhow::anyhow;
use bevy::prelude::{Entity, EntityCommand, EntityCommands, World};
use log::{debug, error};
use mlua::{Function, IntoLua, IntoLuaMulti, Value};

use crate::{LuaRuntime, ScriptObject, REG_WORLD};

pub trait IntoLuaApiName {
    fn name(&self) -> &str;
}

pub trait ScriptCommandsExt {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, name: impl IntoLuaApiName, args: T) -> &mut Self;
    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T) -> &mut Self;
    fn fire_lua_event<T: IntoLuaMulti + Send + 'static>(&mut self, event: &'static str, args: T) -> &mut Self;
}

impl ScriptCommandsExt for EntityCommands<'_> {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, name: impl IntoLuaApiName, args: T) -> &mut Self {
        self.queue(LuaMethodCall {
            method: MethodRef::Name(name.name().to_string()),
            args: Box::new(args),
        })
    }

    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T) -> &mut Self {
        self.queue(LuaMethodCall {
            method: MethodRef::Function(func),
            args: Box::new(args),
        })
    }

    fn fire_lua_event<T: IntoLuaMulti + Send + 'static>(&mut self, event: &'static str, args: T) -> &mut Self {
        self.queue(LuaMethodCall {
            method: MethodRef::Event(event),
            args: Box::new(args),
        })
    }
}

impl IntoLuaApiName for &str {
    fn name(&self) -> &str { self }
}

impl IntoLuaApiName for String {
    fn name(&self) -> &str { self }
}

enum MethodRef {
    Name(String),
    Function(Function),
    Event(&'static str),
}

struct LuaMethodCall<T: IntoLuaMulti + Send> {
    method: MethodRef,
    args: Box<T>,
}

impl <T: IntoLuaMulti + Send + 'static> EntityCommand for LuaMethodCall<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let lua = world.get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        if let Err(e) = lua.scope(|scope| {
            // Entity might've been despawned in a previously queued command. 
            // So we ignore missing entities here.
            if let Ok(ent) = world.get_entity(entity) {
                let obj = ent
                    .get::<ScriptObject>().ok_or(anyhow!("not a script object"))?.object.clone();

                // We have to borrow the world to the lua vm,
                // so it can be accessed within api functions.
                lua.set_named_registry_value(REG_WORLD,
                    scope.create_any_userdata_ref_mut(world)?
                )?;

                if let Some((method, mut args)) = match self.method {
                    MethodRef::Name(name) => {
                        if let Ok(method) = obj.get::<Function>(name.as_str()) {
                            Some((method, self.args.into_lua_multi(&lua)?))
                        } else {
                            debug!("Method '{}' not found!", name);
                            None
                        }
                    },
                    MethodRef::Function(function) => Some((function, self.args.into_lua_multi(&lua)?)),
                    MethodRef::Event(event) => {
                        if let Ok(method) = obj.get::<Function>("Emit") {
                            let mut args = self.args.into_lua_multi(&lua)?;
                            args.push_front(event.into_lua(&lua)?);

                            Some((method, args))
                        } else {
                            debug!("Method 'Emit' not found!");
                            None
                        }
                    },
                } {
                    args.push_front(mlua::Value::Table(obj));

                    let _ = method.call::<Value>(args)?;
                }
            }

            Ok(())
        }) {
            error!("Script error: {}", e);
        }
    }
}
