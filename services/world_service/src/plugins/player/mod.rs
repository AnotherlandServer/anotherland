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

use bevy::{app::{First, Last, Plugin, Update}, ecs::{entity::Entity, schedule::IntoScheduleConfigs, system::{In, Query}}, math::Vec3, state::condition::in_state};
pub use controller::*;
pub use localset::*;
use log::{debug, error};
use obj_params::Class;
pub use systems::*;
use protocol::{oaPkt_SplineSurfing_Acknowledge, oaPkt_SplineSurfing_Exit};
use regex::Regex;
pub use skillbook::*;
pub use initial_inventory_transfer::*;

use crate::{instance::{InstanceShutdown, InstanceState}, plugins::{BehaviorExt, CommandExtPriv, NetworkExtPriv, StringBehavior, clear_obj_changes}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let player_systems = PlayerSystems {
            apply_class_item_result: app.register_system(apply_class_item_result),
            travel_to_portal: app.register_system(travel_to_portal),
        };

        app.insert_resource(player_systems);

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

        app.register_string_behavior(Class::Player, "FlightTube", 
            |
                In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
                query: Query<&PlayerController>,
            | {
                debug!("FlightTube beahavior: {:?}", behavior.args);

                let re = Regex::new(r"SplineID=([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}) InverseTravel=([0-1]) Loc=\[ -?(\d+\.?\d*) -?(\d+\.?\d*) -?(\d+\.?\d*) \]").unwrap();
                if let Some(captures) = re.captures(&behavior.args.join(" ")) {
                    let spline_id = captures[1].parse().unwrap();
                    let inverse_travel = &captures[2] == "1";
                    let loc = Vec3::new(
                        captures[3].parse().unwrap(), 
                        captures[4].parse().unwrap(), 
                        captures[5].parse().unwrap(),
                    );
        

                    if let Ok(controller) = query.get(ent) {
                        controller.send_packet(
                            oaPkt_SplineSurfing_Acknowledge {
                                avatar_id: controller.avatar_id(),
                                spline_id,
                                acknowledged: true,
                                inverse_travel,
                                loc: loc.into(),
                                ..Default::default()
                            }
                        );

                        controller.send_packet(
                            oaPkt_SplineSurfing_Exit {
                                avatar_id: controller.avatar_id(),
                                spline_id,
                                ..Default::default()
                            }
                        );
                    }
                } else {
                    error!("Failed to parse FlightTube behavior: {:?}", behavior.args);
                }
            }
        );

        skillbook_lua::insert_skillbook_api(app.world_mut())
            .expect("failed to insert skillbook api");
        player_lua::insert_player_api(app.world_mut())
            .expect("failed to insert player api");
        portalbook_lua::insert_portalbook_api(app.world_mut())
            .expect("failed to insert portalbook api");
    }
}