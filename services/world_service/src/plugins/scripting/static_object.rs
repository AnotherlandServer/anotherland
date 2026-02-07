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

use std::{ops::Deref, sync::Arc};

use mlua::{FromLua, IntoLua, Table, UserData};
use obj_params::EdnaAbility;
use realm_api::ObjectTemplate;
use scripting::LuaRuntime;

use crate::{error::WorldResult, plugins::{ParamValue, load_class_script}};

#[derive(Clone)]
pub struct StaticObject(pub Arc<ObjectTemplate>);

impl Deref for StaticObject {
    type Target = Arc<ObjectTemplate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserData for StaticObject {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Get", |lua, this, name: String| {
            let val = this.data.get_named::<obj_params::Value>(&name)
                .map_err(mlua::Error::external)?;
        
            ParamValue::new(val.clone())
                .into_lua(lua)
       });
    }
}

impl FromLua for StaticObject {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("object expected"))?;
        Ok(usr.borrow::<StaticObject>()?.clone())
    }
}

impl StaticObject {
    pub fn construct_lua_table(&self, runtime: &mut LuaRuntime) -> WorldResult<Table> {
        let base = load_class_script(runtime, 
            self.0.class, 
            self.0.data.get::<_, String>(EdnaAbility::LuaScript).ok().map(|s| s.as_str()))?;

        let metatable = runtime.vm().create_table()?;
        metatable.set("__index", base)?;

        let table = runtime.vm().create_table()?;
        table.set_metatable(Some(metatable))?;
        table.set("__static_object", self.clone())?;

        table.set("template_guid", self.id.to_string())?;
        table.set("name", self.name.clone())?;
        table.set("class", self.class.name().to_string())?;

        Ok(table)
    }
}