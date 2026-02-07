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

use bevy::{app::App, ecs::lifecycle::HookContext, prelude::Component};
use mlua::{Lua, Table};

use crate::{LuaRuntime, ScriptResult};

pub(crate) fn create_script_object_hooks(app: &mut App) {
    app.world_mut().register_component_hooks::<ScriptObject>()
        .on_add(|world, HookContext { entity, .. }| {
            let script = world.entity(entity).get::<ScriptObject>().unwrap();
            script.object.raw_set("__ent", script.lua.create_any_userdata(entity).unwrap())
                .expect("failed to attach entity to script object");
        });
}

#[derive(Component)]
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
