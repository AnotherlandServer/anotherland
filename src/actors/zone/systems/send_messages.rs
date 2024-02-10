// Copyright (C) 2023 AnotherlandServer
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

use bevy_ecs::{event::Events, system::{Query, Res, ResMut}};

use crate::actors::{zone::{resources::Tasks, zone_events::AvatarEventFired}, AvatarEventSender};

pub fn send_messages(
    tasks: Res<Tasks>,
    mut ev_avatar_event: ResMut<Events<AvatarEventFired>>,
    query: Query<&AvatarEventSender>,
) {
    // pretty sure there is a more efficient way of doing this
    for AvatarEventFired(entity, event) in ev_avatar_event.drain() {
        let sender = query.get(entity).unwrap().0.clone();

        tasks.tasks.spawn(async move {
            let _ = sender.send(event).await;
        });
    }
}