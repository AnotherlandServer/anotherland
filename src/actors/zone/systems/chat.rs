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

use bevy_ecs::{event::EventReader, system::Query};

use crate::actors::{zone::zone_events::ProximityChatEvent, AvatarEvent, AvatarEventServer, Position};

pub fn send_proximity_chat(
    mut ev_chat: EventReader<ProximityChatEvent>,
    query: Query<(&AvatarEventServer, &Position)>,
) {
    for msg in ev_chat.read() {
        for (event_sender, position) in query.iter() {
            if position.position.distance(msg.pos) <= msg.range.aware_dist() {
                let event_sender = event_sender.sender.clone();
                let range = msg.range;
                let sender = msg.sender.clone();
                let message = msg.message.clone();

                tokio::spawn(async move {
                    let _ = event_sender.send(AvatarEvent::ChatMessage { 
                        range, 
                        sender, 
                        message 
                    }).await;
                });
            }
        }
    }
}