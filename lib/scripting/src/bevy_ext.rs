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
use bevy::{ecs::{error::BevyError, system::{Command, Commands, IntoSystem, SystemInput}, world::{EntityMut, EntityWorldMut, World}}, prelude::{App, EntityCommand, EntityCommands}};
use mlua::{FromLuaMulti, Function, IntoLua, IntoLuaMulti, MultiValue, Table};

use crate::{LuaExt, LuaFunctionExt, LuaRuntime, ScriptError, ScriptObject, ScriptResult, ScriptResultExt};

pub trait IntoLuaApiName {
    fn name(&self) -> &str;
}

pub trait EntityScriptCommandsExt {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, name: impl IntoLuaApiName, args: T) -> &mut Self;
    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T) -> &mut Self;
    fn fire_lua_event<T: IntoLuaMulti + Send + 'static>(&mut self, event: &'static str, args: T) -> &mut Self;
}

pub trait ScriptCommandsExt {
    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T);
}

pub trait ScriptAppExt {
    fn add_lua_api
        <F: IntoSystem<In, Result<Out, E>, Marker> + 'static, In, Out, E, Marker>
        (&mut self, group: &'static str, name: &'static str, function: F) -> &mut Self
    where 
        In: SystemInput + 'static,
        <In as SystemInput>::Inner<'static>: FromLuaMulti,
        Out: IntoLuaMulti + 'static,
        E: std::error::Error + Send + Sync + 'static;
}

pub trait ScriptEntityWorldExt {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, name: impl IntoLuaApiName, args: T) -> ScriptResult<R>;
    fn call_lua_method<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, func: Function, args: T) -> ScriptResult<R>;
    fn fire_lua_event<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, event: &'static str, args: T) -> ScriptResult<R>;
}

pub trait PrivateWorldExt {
    fn call_lua_function<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, function: Function, args: T) -> ScriptResult<R>;
}

impl EntityScriptCommandsExt for EntityCommands<'_> {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, name: impl IntoLuaApiName, args: T) -> &mut Self {
        self.queue(LuaMethodCallCommand {
            method: MethodRef::Name(name.name().to_string()),
            args: Box::new(args),
        })
    }

    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T) -> &mut Self {
        self.queue(LuaMethodCallCommand {
            method: MethodRef::Function(func),
            args: Box::new(args),
        })
    }

    fn fire_lua_event<T: IntoLuaMulti + Send + 'static>(&mut self, event: &'static str, args: T) -> &mut Self {
        self.queue(LuaMethodCallCommand {
            method: MethodRef::Event(event),
            args: Box::new(args),
        })
    }
}

impl ScriptCommandsExt for Commands<'_, '_> {
    fn call_lua_method<T: IntoLuaMulti + Send + 'static>(&mut self, func: Function, args: T) {
        self.queue(LuaMethodCallCommand {
            method: MethodRef::Function(func),
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

struct LuaMethodCallCommand<T: IntoLuaMulti + Send> {
    method: MethodRef,
    args: Box<T>,
}

impl <T: IntoLuaMulti + Send + 'static> EntityCommand for LuaMethodCallCommand<T> {
    fn apply(self, mut entity_world: EntityWorldMut<'_>) {
        match self.method {
            MethodRef::Name(name) => entity_world.call_named_lua_method::<_, ()>(name, *self.args),
            MethodRef::Function(function) => entity_world.call_lua_method::<_, ()>(function, *self.args),
            MethodRef::Event(event) => entity_world.fire_lua_event::<_, ()>(event, *self.args)
        }
        .handle();
    }
}

impl <T: IntoLuaMulti + Send + 'static> Command for LuaMethodCallCommand<T> {
    fn apply(self, world: &mut World) {
        let lua = world.get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        let MethodRef::Function(fnc) = self.method else {
            unimplemented!("LuaMethodCallCommand with non-function method reference cannot be applied to World");
        };

        self.args.into_lua_multi(&lua)
            .map_err(ScriptError::LuaError)
            .and_then(|args| world.call_lua_function::<_, ()>(fnc, args))
            .handle();
    }
}

impl ScriptAppExt for App {
    fn add_lua_api
        <
            F: IntoSystem<In, Result<Out, E>, Marker> + 'static, 
            In, 
            Out, 
            E, 
            Marker
        >(&mut self, group: &'static str, name: &'static str, function: F) -> &mut Self
    where 
        In: SystemInput + 'static,
        <In as SystemInput>::Inner<'static>: FromLuaMulti,
        Out: IntoLuaMulti + 'static,
        E: std::error::Error + Send + Sync + 'static
    {
        let runtime = self.world_mut().get_resource::<LuaRuntime>()
            .expect("lua runtime not created");
        let lua = runtime.vm().clone();
        let globals = lua.globals();
        let api_table = globals.get::<Table>("__engine")
            .or_else(|_| {
                let table = lua.create_table()?;
                globals.set("__engine", table.clone())?;
                Ok::<_, mlua::Error>(table)
            })
            .expect("failed to create __engine table");

        let group_table = api_table.get::<Table>(group)
            .or_else(|_| {
                let table = lua.create_table()?;
                api_table.set(group, table.clone())?;
                Ok::<_, mlua::Error>(table)
            })
            .expect("failed to create api group table");

        let func = lua.create_bevy_function(self.world_mut(), function)
            .expect("failed to create lua function from bevy system");

        group_table.set(name, func)
            .expect("failed to set api function in lua");

        self
    }
}

impl ScriptEntityWorldExt for EntityWorldMut<'_> {
    fn call_named_lua_method<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, name: impl IntoLuaApiName, args: T) -> ScriptResult<R> {
        if self.is_despawned() {
            return Err(ScriptError::Other(anyhow!("entity is despawned")));
        }
        
        let lua = self.world().get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        let obj = self
            .get::<ScriptObject>().ok_or(anyhow!("not a script object"))?.object.clone();

        let method = obj.get::<Function>(name.name())
            .map_err(ScriptError::LuaError)?;

        let mut args = args.into_lua_multi(&lua)?;
        args.push_front(mlua::Value::Table(obj));

        let res = 
            self.world_scope(|world| {
                method.call_with_world::<MultiValue>(&lua, world, args)
            })
            .map_err(ScriptError::LuaError)?;

        R::from_lua_multi(res, &lua).map_err(ScriptError::LuaError)
    }

    fn call_lua_method<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, method: Function, args: T) -> ScriptResult<R> {
        if self.is_despawned() {
            return Err(ScriptError::Other(anyhow!("entity is despawned")));
        }

        let lua = self.world().get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        let obj = self
            .get::<ScriptObject>().ok_or(anyhow!("not a script object"))?.object.clone();

        let mut args = args.into_lua_multi(&lua)?;
        args.push_front(mlua::Value::Table(obj));

        let res = 
            self.world_scope(|world| {
                method.call_with_world::<MultiValue>(&lua, world, args)
            })
            .map_err(ScriptError::LuaError)?;

        R::from_lua_multi(res, &lua).map_err(ScriptError::LuaError)
    }

    fn fire_lua_event<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, event: &'static str, args: T) -> ScriptResult<R> {
        if self.is_despawned() {
            return Err(ScriptError::Other(anyhow!("entity is despawned")));
        }
        
        let lua = self.world().get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        let obj = self
            .get::<ScriptObject>().ok_or(anyhow!("not a script object"))?.object.clone();

        let method = obj.get::<Function>("Emit")
            .map_err(ScriptError::LuaError)?;

        let mut args = args.into_lua_multi(&lua)?;
        args.push_front(event.into_lua(&lua)?);
        args.push_front(mlua::Value::Table(obj));

        let res = 
            self.world_scope(|world| {
                method.call_with_world::<MultiValue>(&lua, world, args)
            })
            .map_err(ScriptError::LuaError)?;

        R::from_lua_multi(res, &lua).map_err(ScriptError::LuaError)
    }
}

impl PrivateWorldExt for World {
    fn call_lua_function<T: IntoLuaMulti + Send + 'static, R: FromLuaMulti>(&mut self, function: Function, args: T) -> ScriptResult<R> {
        let lua = self.get_resource::<LuaRuntime>()
            .expect("lua runtime not created")
            .vm().clone();

        match function.call_with_world::<MultiValue>(&lua, self, args) {
            Ok(value) => {
                let res = R::from_lua_multi(value, &lua)?;
                Ok(res)
            },
            Err(e) => {
                Err(ScriptError::LuaError(e))
            },
        }
    }
}
