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

use std::{sync::{LazyLock, RwLock}};

use bevy::{ecs::{entity::Entity, lifecycle::HookContext, world::DeferredWorld}, platform::collections::HashMap, prelude::Component};
use mlua::{FromLua, IntoLua, Lua, Table};

use crate::{LuaRuntime, LuaTableExt, ScriptResult};

#[derive(Component)]
#[component(on_add = on_script_object_added)]
#[component(on_remove = on_script_object_removed)]
pub struct ScriptObject {
    pub(crate) lua: Lua,
    pub(crate) object: Table,
}

impl ScriptObject {
    pub fn new(runtime: &LuaRuntime, base: Option<Table>) -> ScriptResult<ScriptObject> {
        let object = runtime.vm().create_table()?;

        if let Some(base) = base {
            let meta = runtime.vm().create_table()?;
            meta.set("__index", base.clone())?;

            object.set_metatable(Some(meta))?;
        }

        Ok(Self {
            lua: runtime.vm().clone(),
            object,
        })
    }

    pub fn vm(&self) -> &Lua { &self.lua }
    pub fn object(&self) -> &Table { &self.object }
}

static TABLE_MAP: LazyLock<RwLock<HashMap<Entity, Table>>> = LazyLock::new(|| {
    RwLock::new(HashMap::<Entity, Table>::new())
});

fn on_script_object_added(world: DeferredWorld<'_>, ctx: HookContext) {
    let script = world
        .entity(ctx.entity)
        .get::<ScriptObject>()
        .unwrap();
    
    script.object.raw_set("__ent", script.lua.create_any_userdata(ctx.entity).unwrap())
        .expect("failed to attach entity to script object");

    TABLE_MAP.write().unwrap().insert(ctx.entity, script.object.clone());
}

fn on_script_object_removed(_world: DeferredWorld<'_>, ctx: HookContext) {
    TABLE_MAP.write().unwrap().remove(&ctx.entity);
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
    fn into_lua(self, _lua: &Lua) -> mlua::Result<mlua::Value> {
        if let Some(table) = TABLE_MAP.read().unwrap().get(&self.0) {
            Ok(mlua::Value::Table(table.clone()))
        } else {
            Err(mlua::Error::runtime("entity does not have an associated script object"))
        }
    }
}