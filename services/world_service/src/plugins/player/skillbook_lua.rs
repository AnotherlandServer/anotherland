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
use bevy::ecs::{system::{In, Query, ResMut}, world::World};
use mlua::{Lua, Table};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::types::Uuid;

use crate::{error::WorldResult, plugins::Skillbook};

pub(super) fn insert_skillbook_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let skillbook_api = lua.create_table().unwrap();
    runtime.register_native("skillbook", skillbook_api.clone()).unwrap();

    skillbook_api.set("GetSkill", lua.create_bevy_function(world, 
        |
            In((player, skill_id)): In<(Table, String)>,
            query: Query<&Skillbook>,
            mut runtime: ResMut<LuaRuntime>,
        | -> WorldResult<Option<Table>> {
            let skillbook = query.get(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let skill_id = skill_id.parse::<Uuid>()?;

            skillbook.0.iter()
                .find(|s| s.id == skill_id)
                .map(|s| s.construct_lua_table(&mut runtime))
                .transpose()
        })?)?;

    Ok(())
}