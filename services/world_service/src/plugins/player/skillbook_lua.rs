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

use bevy::{app::App, ecs::{relationship::RelationshipTarget, system::{In, Query}}};
use scripting::{LuaEntity, ScriptAppExt};
use toolkit::types::Uuid;

use crate::{error::WorldResult, plugins::{Abilities, AbilityOf, AbilityType}};

pub(super) fn insert_skillbook_api(app: &mut App,) {
    app
        .add_lua_api("skillbook", "GetSkill",
        |
            In((player, skill_id)): In<(LuaEntity, String)>,
            query: Query<&Abilities>,
            abilities: Query<&AbilityOf>,
        | -> WorldResult<Option<LuaEntity>> {
            let Ok(skillbook) = query.get(player.entity()) else {
                return Ok(None);
            };

            let skill_id = skill_id.parse::<Uuid>()?;

            for ability_ent in skillbook.collection() {
                if 
                    let Ok(ability_of) = abilities.get(*ability_ent) &&
                    let AbilityType::ClassSkill { id, .. } = ability_of.kind() &&
                    *id == skill_id
                {
                    return Ok(Some(LuaEntity(*ability_ent)));
                }
            }

            Ok(None)
        });
}