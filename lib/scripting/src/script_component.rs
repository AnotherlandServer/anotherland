// Copyright (C) 2024 AnotherlandServer
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

use bevy::{app::App, prelude::Component};
use mlua::{Lua, Table};

use crate::{ApiType, LuaRuntime, ScriptResult};

pub(crate) fn create_script_object_hooks(app: &mut App) {
    app.world_mut().register_component_hooks::<Scripted>()
        .on_add(|world, entity, _| {
            let script = world.entity(entity).get::<Scripted>().unwrap();
            script.script.raw_set("__ent", script.lua.create_any_userdata(entity).unwrap())
                .expect("failed to attach entity to script object");
        });
}

#[derive(Component)]
pub struct Scripted {
    pub(crate) lua: Lua,
    pub(crate) script: Table,
    pub(crate) api: ApiType,
}

impl Scripted {
    pub fn vm(&self) -> &Lua { &self.lua }
    pub fn script(&self) -> &Table { &self.script }
    pub fn api_type(&self) -> ApiType { self.api }
}

impl LuaRuntime {
    pub fn create_scripted_player(&mut self) -> ScriptResult<Scripted> {
        let object = self.vm().create_table()?;
        object.set_metatable(Some(
            self.load_scripted_class(ApiType::Player, "_player")?
        ));

        Ok(Scripted {
            lua: self.vm().clone(),
            script: object,
            api: ApiType::Player,
        })
    }

    pub fn create_scripted_entity(&mut self, script_name: &str) -> ScriptResult<Scripted> {
        let object = self.vm().create_table()?;
        object.set_metatable(Some(
            self.load_script(script_name)?
        ));

        Ok(Scripted {
            lua: self.vm().clone(),
            script: object,
            api: ApiType::Script,
        })
    }
}