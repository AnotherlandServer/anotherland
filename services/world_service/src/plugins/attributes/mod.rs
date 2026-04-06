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

use bevy::{app::Plugin, ecs::{entity::Entity, event::EntityEvent, observer::On, system::{Commands, In}}, prelude::App};
use scripting::{EntityScriptCommandsExt, LuaEntity, ScriptAppExt};

use crate::error::WorldResult;

pub struct AttributesPlugin;

impl Plugin for AttributesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_recalculate_attributes);
        
        app
            .add_lua_api("attributes", "RecalculateAttributes", 
            |
                In(ent): In<LuaEntity>,
                mut commands: Commands,
            | -> WorldResult<()> {
                commands
                    .entity(ent.entity())
                    .trigger(RecalculateAttributes);
                
                Ok(())
            });
    }
}

#[derive(EntityEvent)]
pub struct RecalculateAttributes(pub Entity);

fn on_recalculate_attributes(
    event: On<RecalculateAttributes>,
    mut commands: Commands,
) {
    commands
        .entity(event.event_target())
        .fire_lua_event("RecalculateAttributes", ());
}
