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

use std::collections::HashMap;

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{AvatarId, PlayerParams, Uuid, UUID_NIL};
use log::info;
use mongodb::Database;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{cluster::actor::Actor, db::{realm_database, Character, DatabaseRecord}, util::AnotherlandResult, NODE};

use super::SessionManager;

#[derive(Clone)]
pub enum ChatChannel {
    Party,
    Clan,
    Officer,
    Whisper{ receiver: String },
    Generic(String),
}

#[derive(Clone)]
pub enum SocialEvent {
    Chat(ChatMessage)
}

#[derive(Clone)]
pub struct ChatMessage {
    pub channel: ChatChannel,
    pub sender: String,
    pub message: String,
}

struct SocialState {
    sender: Sender<SocialEvent>,
    character_name: Option<String>,
    clan_id: Option<Uuid>,
    zone_id: Option<Uuid>,
}

pub struct Social {
    db: Database,
    avatars: HashMap<AvatarId, SocialState>,
    session_manger: RemoteActorRef<SessionManager>,
}

impl Social {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Social {
            db: realm_database().await,
            avatars: HashMap::new(),
            session_manger: NODE.get_remote_actor("session_manager")
                .expect("Session manager not found"),
        })
    }
}

#[async_trait]
impl Actor for Social {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { Some("social") }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }
}

#[actor_actions]
impl Social {
    pub async fn register_avatar(&mut self, id: AvatarId) -> Receiver<SocialEvent> {
        let (sender, receiver) = mpsc::channel(10);

        self.avatars.insert(id, SocialState { 
            sender,
            character_name: None,
            clan_id: None,
            zone_id: None,
        });

        receiver
    }

    pub fn unregister_avatar(&mut self, id: AvatarId) {
        self.avatars.remove(&id);
    }

    pub async fn update_avatar(&mut self, id: AvatarId, session_id: Uuid) -> AnotherlandResult<()> {
        if let Some(state) = self.avatars.get_mut(&id) {
            let session = self.session_manger.get_session(session_id).await?;
            
            if let Some(character) = Character::get(self.db.clone(), &session.character_id.unwrap()).await? {
                state.zone_id = session.zone_guid;
                state.clan_id = if *character.data.clan_guid() != *UUID_NIL {
                    Some(*character.data.clan_guid())
                } else {
                    None
                };
                state.character_name = Some(character.name.clone());
            }
        }
        
        Ok(())
    }

    pub async fn chat(&self, sender: AvatarId, channel: ChatChannel, message: String) {
        if let Some(state) = self.avatars.get(&sender) {
            match channel {
                ChatChannel::Clan => {},
                ChatChannel::Officer => {},
                ChatChannel::Party => {},
                ChatChannel::Whisper { receiver } => {
                    info!(
                        channel = "whisper", 
                        receiver = receiver, 
                        sender = state.character_name.as_ref().unwrap(); 
                        "{}: {}", state.character_name.as_ref().unwrap(), message
                    );

                    if let Some(receiver) = self.avatars.values().find(|v| {
                        if let Some(name) = v.character_name.as_ref() {
                            name == receiver.as_str()
                        } else {
                            false
                        }
                    }) {
                        let message = SocialEvent::Chat(ChatMessage { 
                            channel: ChatChannel::Whisper { receiver: receiver.character_name.as_ref().unwrap().clone() }, 
                            sender: state.character_name.as_ref().unwrap().clone(),
                            message 
                        });

                        // send message to sender and recipient
                        let _ = receiver.sender.send(message.clone()).await;
                        let _ = state.sender.send(message).await;
                    }
                },
                ChatChannel::Generic(channel) => {
                    info!(
                        channel = "generic", 
                        channel_name = channel,
                        sender = state.character_name.as_ref().unwrap(); 
                        "{}: {}", state.character_name.as_ref().unwrap(), message
                    );

                    let message = SocialEvent::Chat(ChatMessage { 
                        channel: ChatChannel::Generic(channel), 
                        sender: state.character_name.as_ref().unwrap().clone(), 
                        message
                    });

                    // for now we assume all characters beeing in the all channels all the time
                    for receiver in self.avatars.values() {
                        let _ = receiver.sender.send(message.clone()).await;
                    }
                }
            }
        }
    }
}