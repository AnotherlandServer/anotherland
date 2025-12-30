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

use bevy::{app::{First, Last, Plugin, PreStartup}, ecs::{message::Message, schedule::IntoScheduleConfigs}, prelude::{App, resource_exists}};

use crate::{clean_hot_reload, create_script_object_hooks, hot_reload, prepare_hot_reload, HotReloadEnabled};

#[derive(Message)]
pub struct LuaScriptReloaded;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, prepare_hot_reload);
        app.add_systems(First, hot_reload.run_if(resource_exists::<HotReloadEnabled>));
        app.add_systems(Last, clean_hot_reload);

        app.add_message::<LuaScriptReloaded>();

        create_script_object_hooks(app);
    }
}

