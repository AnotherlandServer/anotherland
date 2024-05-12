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

use bevy::app::{Last, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;

use crate::actors::zone::plugins::{remove_old_items, send_param_update_events};

use super::{combat::send_hitpoint_updates, initialize_fog_of_war, inventory::{send_item_removals, send_item_updates, track_added_items, ItemTracker}, params::send_param_updates, positions::send_position_updates};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ItemTracker::new());
        app.add_systems(Last, (
            send_position_updates,
            send_param_updates.after(send_param_update_events),
            send_hitpoint_updates,
            send_item_updates,
            send_item_removals.after(remove_old_items),
            track_added_items,
            initialize_fog_of_war,
        ));
    }
}