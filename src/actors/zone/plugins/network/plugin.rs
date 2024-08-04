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

use atlas::{raknet::Message, CPkt};
use bevy::{app::{App, Last, Plugin}, utils::HashMap};
use bevy_ecs::{schedule::IntoSystemConfigs, system::{IntoSystem, Resource, SystemId}, world::{Mut, World}};

use crate::actors::zone::{plugins::{remove_old_items, send_param_update_events, send_subjective_param_update_events}, systems::update_interests};

use super::{combat::{send_hitpoint_updates, toggle_off_combat, toggle_on_combat}, initialize_fog_of_war, inventory::{send_item_removals, send_item_updates, track_added_items, ItemTracker}, params::{send_param_updates, send_subjective_param_updates}, positions::send_position_updates, quest::{handle_quest_debug_request, handle_quest_request, notify_quest_abandoned, notify_quest_accepted, update_quest_giver_status}};

#[derive(Resource)]
pub struct MessageHandlers(HashMap<(u8, u8), SystemId<CPkt, ()>>);

pub trait NetworkExt {
    fn register_message_handler<T: IntoSystem<CPkt, (), Marker> + 'static, Marker>(&mut self, id: (u8, u8), system: T);
    fn handle_message(&mut self, message: Message);
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MessageHandlers(HashMap::new()));
        app.insert_resource(ItemTracker::new());
        app.add_systems(Last, (
            send_position_updates,
            send_param_updates.after(send_param_update_events),
            send_subjective_param_updates.after(send_subjective_param_update_events),
            send_hitpoint_updates,
            send_item_updates,
            send_item_removals.after(remove_old_items),
            track_added_items,
            initialize_fog_of_war,
            toggle_on_combat,
            toggle_off_combat,
            notify_quest_accepted,
            notify_quest_abandoned,
            update_quest_giver_status.after(update_interests)
        ));

        app.register_message_handler((0xa5, 0x4), handle_quest_request);
        app.register_message_handler((0xa5, 0xa), handle_quest_debug_request);
    }
}

impl NetworkExt for App {
    fn register_message_handler<T: IntoSystem<CPkt, (), Marker> + 'static, Marker>(&mut self, id: (u8, u8), system: T) {
        let system = self.world.register_system(system);

        self.world.get_resource_mut::<MessageHandlers>()
            .unwrap()
            .0
            .insert(id, system);
    }

    fn handle_message(&mut self, message: Message) {
        if let Message::AtlasPkt(pkt) = message {
            self.world.resource_scope(|world: &mut World, res: Mut<MessageHandlers>| {
                if let Some(system) = res.0.get(&pkt.get_id()) {
                    let _ = world.run_system_with_input(*system, pkt);
                }
            });
        }
    }
}