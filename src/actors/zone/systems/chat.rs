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

use atlas::{CPktChat, CpktChatChatType, PlayerClass, PlayerComponent};
use bevy_ecs::{entity::Entity, event::{EventReader, EventWriter}, query::With, system::Query};

use crate::actors::{zone::{plugins::{PlayerController, Position}, zone_events::ProximityChatEvent}, ProximityChatRange};

pub fn send_proximity_chat(
    mut ev_chat: EventReader<ProximityChatEvent>,
    query: Query<(&Position, &PlayerController), With<PlayerComponent>>,
    //mut ev_avatar_event: EventWriter<AvatarEventFired>,
) {
    for msg in ev_chat.read() {
        for (_, controller) in query.iter()
            .filter(|(pos, _)| pos.position.distance(msg.pos) <= msg.range.aware_dist()) {

            controller.send_message(CPktChat {
                chat_type: match msg.range {
                    ProximityChatRange::Say => CpktChatChatType::Say,
                    ProximityChatRange::TeamSay => CpktChatChatType::Say,
                    ProximityChatRange::Shout => CpktChatChatType::Shout,
                },
                message: msg.message.clone(),
                sender: msg.sender.clone(),
                ..Default::default()
            }.into_message());
        }
    }
}