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

// based on https://github.com/rlidwka/bevy_mod_physx/blob/3baa1ea0701aaea8faedf3281e488b6f369cfe7b/src/events.rs#L3

use bevy::prelude::*;
use bevy_ecs::system::Resource;
use parking_lot::Mutex;
use std::sync::mpsc::Receiver;

#[derive(Resource, Deref, DerefMut)]
struct ChannelReceiver<T>(Mutex<Receiver<T>>);

pub trait EventChannelExtension {
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self;
}

impl EventChannelExtension for App {
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self {
        assert!(
            !self.world.contains_resource::<ChannelReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>();
        self.add_systems(PreUpdate,channel_to_event::<T>);
        self.insert_resource(ChannelReceiver(Mutex::new(receiver)));
        self
    }
}

fn channel_to_event<T: Event + Send + Sync>(
    receiver: Res<ChannelReceiver<T>>,
    mut writer: EventWriter<T>,
) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.try_lock().expect("unable to acquire mutex lock");

    writer.send_batch((*events).try_iter());
}
