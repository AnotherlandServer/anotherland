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

use bevy::app::{First, Last, Plugin};
use bevy_ecs::component::Component;

use super::{insert_new_items, prepare_player_change_detection, update_item_database, update_player_database};

pub struct PersistancePlugin;

impl Plugin for PersistancePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(First, prepare_player_change_detection);
        app.add_systems(Last, (update_player_database, insert_new_items, update_item_database));
    }
}

#[derive(Component)]
pub struct CreationPending;
