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

use bevy::prelude::World;
use mlua::{AnyUserData, IntoLua, Lua, Table, Value};
use obj_params::GameObjectData;

use crate::{LuaTableExt, ParamValue, ScriptResult, REG_WORLD};

pub(crate) fn create_gameobject_table(lua: &Lua) -> ScriptResult<Table> {
    let tbl = lua.create_table()?;
    tbl.set("__index", &tbl)?;

    tbl.set("Get", lua.create_function(gameobject_get)?)?;
    tbl.set("Set", lua.create_function(gameobject_set)?)?;
    tbl.set("Reset", lua.create_function(gameobject_reset)?)?;

    Ok(tbl)
}

fn gameobject_get(lua: &Lua, (this, index): (Table, String)) -> Result<Value, mlua::Error> {
    lua.named_registry_value::<AnyUserData>(REG_WORLD)?
        .borrow_mut_scoped(move |world: &mut World| {
            let ent = this.entity()?;
            let gameobject = world.entity(ent).get::<GameObjectData>()
                .ok_or(mlua::Error::runtime("Gameobject not found"))?;

            let val = gameobject.get_named::<obj_params::Value>(&index)
                .map_err(mlua::Error::external)?;

            ParamValue::new(val.clone()).into_lua(lua)
        })?
}

fn gameobject_set(lua: &Lua, (this, index, value): (Table, String, Value)) -> Result<Value, mlua::Error> {
    lua.named_registry_value::<AnyUserData>(REG_WORLD)?
        .borrow_mut_scoped(move |world: &mut World| {
            let mut ent = world.entity_mut(this.entity()?);
            let mut gameobject = ent.get_mut::<GameObjectData>()
                .ok_or(mlua::Error::runtime("Gameobject not found"))?;

            let attr = gameobject.class().get_attribute(&index)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            let value = ParamValue::from_lua(attr, value, lua)?;

            if let Some(prev_val) = gameobject.set_named(&index, value) {
                ParamValue::new(prev_val).into_lua(lua)
            } else {
                ParamValue::new(attr.default().clone()).into_lua(lua)
            }
        })?
}

fn gameobject_reset(lua: &Lua, (this, index): (Table, String)) -> Result<Value, mlua::Error> {
    lua.named_registry_value::<AnyUserData>(REG_WORLD)?
        .borrow_mut_scoped(move |world: &mut World| {
            let mut ent = world.entity_mut(this.entity()?);
            let mut gameobject = ent.get_mut::<GameObjectData>()
                .ok_or(mlua::Error::runtime("Gameobject not found"))?;

            let attr = gameobject.class().get_attribute(&index)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            if let Some(prev_val) = gameobject.set_named(&index, attr.default().clone()) {
                ParamValue::new(prev_val).into_lua(lua)
            } else {
                ParamValue::new(attr.default().clone()).into_lua(lua)
            }
        })?
}