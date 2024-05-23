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

use crate::actors::zone::plugins::{remove_old_items, send_param_update_events};

use super::{combat::{send_hitpoint_updates, toggle_off_combat, toggle_on_combat}, dialog::{handle_dialog_choice, handle_dialog_request}, initialize_fog_of_war, inventory::{send_item_removals, send_item_updates, track_added_items, ItemTracker}, params::send_param_updates, positions::send_position_updates, quest::{handle_quest_request, initialize_quest_progress}};

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
            send_hitpoint_updates,
            send_item_updates,
            send_item_removals.after(remove_old_items),
            track_added_items,
            initialize_fog_of_war,
            initialize_quest_progress,
            toggle_on_combat,
            toggle_off_combat,
        ));

        app.register_message_handler((0xa5, 0x4), handle_quest_request);
        app.register_message_handler((0xa6, 0x0), handle_dialog_request);
        app.register_message_handler((0xa6, 0x1), handle_dialog_choice);
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