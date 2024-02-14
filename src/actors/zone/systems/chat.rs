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

use atlas::PlayerClass;
use bevy_ecs::{entity::Entity, event::{EventReader, EventWriter}, query::With, system::Query};

use crate::actors::{zone::zone_events::{AvatarEventFired, ProximityChatEvent}, AvatarEvent, Position};

pub fn send_proximity_chat(
    mut ev_chat: EventReader<ProximityChatEvent>,
    query: Query<(Entity, &Position), With<PlayerClass>>,
    mut ev_avatar_event: EventWriter<AvatarEventFired>,
) {
    for msg in ev_chat.read() {
        ev_avatar_event.send_batch(
            query.iter()
                .filter(|(_, pos)| pos.position.distance(msg.pos) <= msg.range.aware_dist())
                .map(|(entity, _)| {
                    AvatarEventFired(entity, AvatarEvent::ChatMessage { 
                        range: msg.range, 
                        sender: msg.sender.clone(),
                        message: msg.message.clone(),
                    })
                })
        );
    }
}