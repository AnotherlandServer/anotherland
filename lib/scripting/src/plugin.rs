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

use bevy::{app::{First, Plugin, PreStartup}, prelude::{resource_exists, App, IntoSystemConfigs}};

use crate::{create_script_object_hooks, hot_reload, prepare_hot_reload, HotReloadEnabled};

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, prepare_hot_reload);
        app.add_systems(First, hot_reload.run_if(resource_exists::<HotReloadEnabled>));

        create_script_object_hooks(app);
    }
}

