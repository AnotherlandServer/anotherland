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

use bevy::app::{First, Last, Plugin, PostUpdate};

use super::{combat::send_hitpoint_updates, inventory::{send_item_removals, send_item_updates, track_added_items, ItemTracker}, params::{prepare_param_updates, send_param_updates}, positions::send_position_updates};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ItemTracker::new());
        app.add_systems(First, prepare_param_updates);
        app.add_systems(Last, (
            send_position_updates,
            send_param_updates,
            send_hitpoint_updates,
            send_item_updates,
            send_item_removals,
            track_added_items,
        ));
    }
}