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

use bevy::ecs::{event::EntityEvent, observer::On, query::Has, system::{Commands, Query}};
use scripting::{EntityScriptCommandsExt, ScriptApi};

use crate::plugins::{InitializeObject, RemoveObject, Scripted};

pub fn on_init_script(
    event: On<InitializeObject>,
    has_scripted: Query<Has<Scripted>>,
    mut commands: Commands
) {
    if !has_scripted.get(event.event_target()).unwrap_or(false) {
        return;
    }

    commands
        .entity(event.event_target())
        .call_named_lua_method(ScriptApi::Attach, ());
}

pub fn on_remove_script(
    event: On<RemoveObject>,
    has_scripted: Query<Has<Scripted>>,
    mut commands: Commands
) {
    if !has_scripted.get(event.event_target()).unwrap_or(false) {
        return;
    }

    commands
        .entity(event.event_target())
        .call_named_lua_method(ScriptApi::Detach, ());
}