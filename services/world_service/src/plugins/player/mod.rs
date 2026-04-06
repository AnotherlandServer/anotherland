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
mod stance;

use bevy::{app::{First, Last, Plugin, Update}, ecs::{entity::Entity, schedule::IntoScheduleConfigs, system::{In, Query}}, state::condition::in_state};
pub use controller::*;
pub use localset::*;
use obj_params::Class;
pub use systems::*;
pub use skillbook::*;
pub use initial_inventory_transfer::*;
use toolkit::NativeParam;

use crate::{instance::{InstanceShutdown, InstanceState}, plugins::{Avatar, BehaviorExt, CommandExtPriv, InitializeObject, Movement, NetworkExtPriv, ServerAction, clear_obj_changes, player::{bevariors::{behavior_flight_tube, behavior_loot_avatar}, loader::TransmitAsyncPlayerData, stance::sync_class_stance}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_message::<PlayerJoinRequested>();
        app.add_message::<TransmitAsyncPlayerData>();

        app.add_systems(First, 
            (
                (
                    process_join_requests,
                    handle_controller_events
                ).run_if(in_state(InstanceState::Running)),
                skillbook::network_sync_skillbook
            )
        );

        app.add_systems(Update, (
            spawn_player,
            sync_class_stance,
            network_sync_skill,
            initial_skill_sync
        ));

        app.add_systems(Last, (
            save_player_data.before(clear_obj_changes),
            cleanup_local_sets,
            cleanup_player_controllers,
        ));

        app.add_systems(InstanceShutdown, close_connections);
        
        app.register_message_handler(handle_avatar_update);

        app.register_command("instantKill", cmd_instant_kill);
        app.register_command("travel_to_portal", cmd_travel_to_portal);
        app.register_command("play_cinematic", |
                In((ent, params)): In<(Entity, Vec<NativeParam>)>,
                query: Query<(&Avatar, &Movement, &PlayerController)>,
            | {
                if 
                    let Some(NativeParam::String(cinematic)) = params.first() &&
                    let Some(NativeParam::String(level)) = params.get(1) &&
                    let Ok((avatar, movement, controller)) = query.get(ent)
                {
                    controller.send_packet(ServerAction::Cinematic { 
                        player: avatar.id,
                        name: cinematic.clone(), 
                        level: Some(level.clone()), 
                        position: Some((movement.position, movement.rotation))
                    }.into_pkt());
                }
            });

        app.register_command("trigger_remote_event", |
                In((ent, params)): In<(Entity, Vec<NativeParam>)>,
                query: Query<(&Movement, &PlayerController)>,
            | {
                if 
                    let Some(NativeParam::String(event)) = params.first() &&
                    let Ok((movement, controller)) = query.get(ent)
                {
                    controller.send_packet(ServerAction::RemoteEvent(event.clone(), (movement.position, movement.rotation)).into_pkt());
                }
            });


        app.register_string_behavior(Class::Player, "FlightTube", behavior_flight_tube);
        app.register_binary_behavior(Class::Player, "lootavatar", behavior_loot_avatar);

        app.add_observer(on_initialize_player);

        skillbook_lua::insert_skillbook_api(app);
        player_lua::insert_player_api(app);
        portalbook_lua::insert_portalbook_api(app);
    }
}