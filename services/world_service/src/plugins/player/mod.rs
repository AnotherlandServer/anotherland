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

mod systems;
mod loader;
mod localset;
mod player_lua;
mod skillbook;
mod skillbook_lua;
mod controller;
mod portalbook_lua;
mod initial_inventory_transfer;
mod bevariors;

use bevy::{app::{First, Last, Plugin, Update}, ecs::schedule::IntoScheduleConfigs, state::condition::in_state};
pub use controller::*;
pub use localset::*;
use obj_params::Class;
pub use systems::*;
pub use skillbook::*;
pub use initial_inventory_transfer::*;

use crate::{instance::{InstanceShutdown, InstanceState}, plugins::{BehaviorExt, CommandExtPriv, NetworkExtPriv, clear_obj_changes, player::bevariors::{behavior_flight_tube, behavior_loot_avatar}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_message::<PlayerJoinRequested>();

        app.add_systems(First, 
            (
                (
                    process_join_requests,
                    handle_controller_events
                ).run_if(in_state(InstanceState::Running)),
                skillbook::network_sync_skillbook
            )
        );

        app.add_systems(Update, spawn_player);

        app.add_systems(Last, (
            save_player_data.before(clear_obj_changes),
            cleanup_local_sets,
            cleanup_player_controllers,
        ));

        app.add_systems(InstanceShutdown, close_connections);
        
        app.register_message_handler(handle_avatar_update);

        app.register_command("instantKill", cmd_instant_kill);

        app.register_string_behavior(Class::Player, "FlightTube", behavior_flight_tube);
        app.register_binary_behavior(Class::Player, "lootavatar", behavior_loot_avatar);

        skillbook_lua::insert_skillbook_api(app.world_mut())
            .expect("failed to insert skillbook api");
        player_lua::insert_player_api(app.world_mut())
            .expect("failed to insert player api");
        portalbook_lua::insert_portalbook_api(app.world_mut())
            .expect("failed to insert portalbook api");
    }
}