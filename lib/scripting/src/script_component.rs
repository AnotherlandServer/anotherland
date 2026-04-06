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

use bevy::{ecs::{entity::Entity, lifecycle::HookContext, world::DeferredWorld}, prelude::Component};
use mlua::{FromLua, IntoLua, Lua, Table};

use crate::{LuaRuntime, LuaTableExt, ScriptResult};

#[derive(Component)]
#[component(on_remove = on_script_object_removed)]
pub struct ScriptObject {
    pub(crate) lua: Lua,
    pub(crate) object: Table,
}

impl ScriptObject {
    pub fn new(ent: Entity, runtime: &Lua, base: Option<Table>) -> ScriptResult<ScriptObject> {
        let registry = runtime.named_registry_value::<Table>("_ENTITIES").unwrap();

        let object = 
            if let Ok(table) = registry.raw_get::<Table>(ent.to_bits()) {
                table.clone()
            } else {
                let table = runtime.create_table()?;
                registry.raw_set(ent.to_bits(), table.clone())?;
                table
            };
                
        if let Some(base) = base {
            let meta = runtime.create_table()?;
            meta.set("__index", base.clone())?;

            object.set_metatable(Some(meta))?;
        } else {
            object.set_metatable(None)?;
        }

        object.raw_set("__ent", runtime.create_any_userdata(ent).unwrap())
            .expect("failed to attach entity to script object");

        Ok(Self {
            lua: runtime.clone(),
            object,
        })
    }

    pub fn vm(&self) -> &Lua { &self.lua }
    pub fn object(&self) -> &Table { &self.object }
}

fn on_script_object_removed(world: DeferredWorld<'_>, ctx: HookContext) {
    let lua = world.get_resource::<LuaRuntime>().unwrap().vm().clone();
    let registry = lua.named_registry_value::<Table>("_ENTITIES").unwrap();

    registry.set(ctx.entity.to_bits(), mlua::Value::Nil).unwrap();
}

#[derive(Clone, Copy)]
pub struct LuaEntity(pub Entity);

impl LuaEntity {
    pub fn take(self) -> Entity {
        self.0
    }

    pub fn entity(&self) -> Entity {
        self.0
    }
}

impl From<Entity> for LuaEntity {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<LuaEntity> for Entity {
    fn from(wrapper: LuaEntity) -> Self {
        wrapper.0
    }
}

impl FromLua for LuaEntity {
    fn from_lua(lua_value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = lua_value.as_table()
            .ok_or_else(|| mlua::Error::runtime("expected a table for Entity"))?;
        let ent = table.entity()?;
        Ok(Self(ent))
    }
}

impl IntoLua for LuaEntity {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let registry = lua.named_registry_value::<Table>("_ENTITIES").unwrap();

        if let Ok(table) = registry.raw_get::<Table>(self.0.to_bits()) {
            Ok(mlua::Value::Table(table.clone()))
        } else {
            let table = lua.create_table()?;

            let meta = lua.create_table()?;
            meta.set("__index", lua.create_function(object_not_ready)?)?;

            table.set_metatable(Some(meta))?;

            registry.raw_set(self.0.to_bits(), table.clone())?;
            Ok(mlua::Value::Table(table))
        }
    }
}

fn object_not_ready(_lua: &Lua, _args: mlua::MultiValue) -> mlua::Result<()> {
     Err(mlua::Error::runtime("script object is not ready yet"))
}