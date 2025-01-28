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

use bevy::{app::Plugin, prelude::{App, Entity, In, Query, Res, With}};
use obj_params::{tags::PlayerTag, GameObjectData, Player};
use protocol::{CPktChat, CpktChatChatType};
use realm_api::proto::{Destination, RealmRequest};
use toolkit::types::Uuid;

use crate::instance::ZoneInstance;

use super::{AvatarInfo, Movement, NetworkExtPriv, PlayerController};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_chat_msg);
    }
}

fn handle_chat_msg(
    In((ent, pkt)): In<(Entity, CPktChat)>,
    instance: Res<ZoneInstance>,
    query: Query<(&PlayerController, &AvatarInfo, &GameObjectData, &Movement), With<PlayerTag>>,
) {
    if let Ok((send_controller, avatar, sender_data, sender_movement)) = query.get(ent) {
        // Local messages are directly handled by this world node,
        // all other messages are relayed via the realm service to
        // cluster nodes.
        if 
            matches!(pkt.chat_type, CpktChatChatType::Local) || 
            matches!(pkt.chat_type, CpktChatChatType::LocalYell) || 
            matches!(pkt.chat_type, CpktChatChatType::Shout)
        {
        
            let pkt = CPktChat {
                chat_type: pkt.chat_type,
                field_2: avatar.id,
                sender: avatar.name.clone(),
                message: pkt.message.clone(),
                ..Default::default()
            };
            
            for (controller, _, data, movement) in query.iter() {
                let awarenes = data.get::<_, f32>(Player::AwareDist)
                    .unwrap() * match pkt.chat_type {
                        CpktChatChatType::LocalYell => 2.0,
                        CpktChatChatType::Shout => 2.0,
                        _ => 1.0
                    };
    
                if sender_movement.position.distance(movement.position) <= awarenes {
                    controller.send_packet(pkt.clone());
                }
            }
        } else {
            let client = instance.realm_client.clone();
            let session = *send_controller.session().id();
            let party_id = *sender_data.get::<_, Uuid>(Player::PartyGuid).unwrap();
            let clan_id = *sender_data.get::<_, Uuid>(Player::ClanGuid).unwrap();

            instance.spawn_task(async move {
                let _ = client.send(RealmRequest::ChatMessage { 
                    sender_id: Some(session), 
                    destination: match pkt.chat_type {
                        CpktChatChatType::Party => Destination::Party(party_id),
                        CpktChatChatType::Local => unreachable!(),
                        CpktChatChatType::LocalYell => unreachable!(),
                        CpktChatChatType::Clan => Destination::Clan(clan_id),
                        CpktChatChatType::ClanOfficer => Destination::ClanOfficer(clan_id),
                        CpktChatChatType::Whisper => Destination::Whisper(pkt.receiver),
                        CpktChatChatType::Shout => unreachable!(),
                        CpktChatChatType::Broadcast => Destination::Broadcast,
                        CpktChatChatType::Bubble => todo!(),
                    }, 
                    message: pkt.message 
                }).await;
            });
        }
    }
}