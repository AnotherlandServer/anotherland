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

use bevy::prelude::{Entity, IntoSystem, SystemInput, World};
use mlua::{AnyUserData, FromLuaMulti, Function, IntoLuaMulti, Lua, Table, UserDataRef};

use crate::{ScriptResult, REG_WORLD};

pub trait LuaExt {
    fn create_bevy_function<'i, F: IntoSystem<In, Result<Out, E>, Marker> + 'static, Marker, E, In, Out>(&self, world: &mut World, system: F) 
        -> ScriptResult<Function>
        where 
            In: SystemInput + 'static,
            <In as SystemInput>::Inner<'static>: FromLuaMulti,
            Out: IntoLuaMulti + 'static,
            E: std::error::Error + Send + Sync + 'static;
}

impl LuaExt for Lua {
    fn create_bevy_function<F: IntoSystem<In, Result<Out, E>, Marker> + 'static, Marker, E, In, Out>(&self, world: &mut World, system: F) 
        -> ScriptResult<Function>
        where 
            In: SystemInput + 'static,
            <In as SystemInput>::Inner<'static>: FromLuaMulti,
            Out: IntoLuaMulti + 'static,
            E: std::error::Error + Send + Sync + 'static
    {
        let system = world.register_system(system);

        Ok(
            self.create_function(move |lua: &Lua, args: In::Inner<'static>| {
                lua.named_registry_value::<AnyUserData>(REG_WORLD)?
                    .borrow_mut_scoped(move |world: &mut World| {
                        world.run_system_with(system, args).unwrap()
                            .map_err(|e| mlua::Error::external(e))
                    })?
            })?
        )
    }
}

pub trait LuaTableExt {
    fn entity(&self) -> Result<Entity, mlua::Error>;
}

impl LuaTableExt for Table {
    fn entity(&self) -> Result<Entity, mlua::Error> {
        Ok(*self.get::<UserDataRef<Entity>>("__ent")?)
    }
}

pub trait LuaFunctionExt {
    fn call_with_world<R: FromLuaMulti>(&self, lua: &mlua::Lua, world: &mut World, args: impl IntoLuaMulti) -> mlua::Result<R>;
}

impl LuaFunctionExt for Function {
    fn call_with_world<R: FromLuaMulti>(&self, lua: &mlua::Lua, world: &mut World, args: impl IntoLuaMulti) -> mlua::Result<R> {
        lua.scope(|scope| {
            // We have to borrow the world to the lua vm,
            // so it can be accessed within api functions.
            lua.set_named_registry_value(REG_WORLD,
                scope.create_any_userdata_ref_mut(world)?
            )?;

            self.call::<R>(args.into_lua_multi(lua)?)
        })
    }
}