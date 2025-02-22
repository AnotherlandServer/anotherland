// Copyright (C) 2025 AnotherlandServer
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

use std::{collections::HashMap, sync::Arc};

use database::DatabaseRecord;
use log::debug;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use toolkit::types::Uuid;

use crate::{db::Character, proto::{NodeType, RealmResponse, RealmServer}, NODE_REGISTRY, SESSION_MANAGER};

#[derive(Clone)]
pub struct ChatRouter(Sender<Message>);

#[derive(Serialize, Deserialize, Clone)]
pub enum Destination {
    Broadcast,
    Whisper(String),
    Clan(Uuid),
    ClanOfficer(Uuid),
    Party(Uuid),
}

enum Message {
    ConnectSession(Uuid),
    DisconnectSession(Uuid),
    Forward {
        session_id: Option<Uuid>,
        destination: Destination,
        message: String,
    },
}

impl ChatRouter {
    pub fn new(db: Database, server: Arc<RealmServer>) -> Self {
        let (sender, mut receiver) = mpsc::channel(100);

        let mut sessions = HashMap::new();
        let mut name_lookup = HashMap::new();

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Some(Message::ConnectSession(id)) => {
                        if 
                            let Some(state) = SESSION_MANAGER.get().unwrap().get_state(id).await &&
                            let Ok(Some(character)) = Character::get(&db, &state.character_id).await
                        {
                            name_lookup.insert(character.name.clone(), state.clone());
                            sessions.insert(state.id, (character.name, state));
                        }
                    },
                    Some(Message::DisconnectSession(id)) => {
                        if let Some((name, _)) = sessions.remove(&id) {
                            name_lookup.remove(&name);
                        }
                    },
                    Some(Message::Forward { session_id, destination, message }) => {
                        let (sender_id, sender_name) = if let Some(id) = session_id {
                            // Prepend character name if sent from a valid session
                            if 
                                let Some(state) = SESSION_MANAGER.get().unwrap().get_state(id).await &&
                                let Ok(Some(character)) = Character::get(&db, &state.character_id).await
                            {
                                (Some(state.avatar_id), character.name.clone())
                            } else {
                                // Session or character not found, drop message
                                return;
                            }
                        } else {
                            (None, "System".to_string())
                        };
                
                        let msg = match &destination {
                            Destination::Broadcast => { // Forward message to all cluster nodes
                                for (peer, node) in NODE_REGISTRY.get().unwrap().nodes().await {
                                    if matches!(node.ty, NodeType::Cluster) {
                                        let _ = server.send(&peer, RealmResponse::ChatMessage {
                                            recipients: vec![],
                                            sender_id,
                                            sender_name: sender_name.clone(),
                                            destination: Destination::Broadcast, 
                                            message: message.clone()
                                        }).await;
                                    }
                                }
                                
                                None
                            },
                            Destination::Whisper(character_name) => {
                                let mut sessions = vec![];
                                if let Some(sender) = session_id {
                                    sessions.push(sender);
                                }

                                // Find the cluster node the recipient is connected to
                                // and route message there.
                                if 
                                    let Some(state) = name_lookup.get(character_name)
                                {
                                    sessions.push(state.id);

                                    Some((sessions, RealmResponse::ChatMessage {
                                        recipients: vec![],
                                        sender_id, 
                                        sender_name, 
                                        destination, 
                                        message 
                                    }))
                                } else {
                                    None
                                }
                            },
                            Destination::Clan(_) => { 
                                debug!("Clan chat not implemented!"); 
                                None
                            },
                            Destination::ClanOfficer(_) => { 
                                debug!("Clan officer chat not implemented!"); 
                                None
                            },
                            Destination::Party(_) => { 
                                debug!("Party chat not implemented!"); 
                                None
                            },
                        };

                        if let Some((mut sessions, msg)) = msg {
                            sessions.dedup();

                            let mut messages = HashMap::new();

                            // Group messages before sending to cluster nodes
                            for session in sessions {
                                if 
                                    let Some(state) = SESSION_MANAGER.get().unwrap().get_state(session).await &&
                                    let Some(cluster_node) = state.cluster_node
                                {
                                    if let Some(RealmResponse::ChatMessage { recipients, .. }) = messages.get_mut(&cluster_node) {
                                        recipients.push(state.id);
                                    } else {
                                        let mut msg = msg.clone();
                                        if let RealmResponse::ChatMessage { recipients, .. } = &mut msg {
                                            recipients.push(session);
                                        } else {
                                            unreachable!()
                                        }

                                        messages.insert(cluster_node, msg);
                                    }
                                }
                            }

                            // Forward chat messages to cluster nodes
                            for (node, msg) in messages {
                                if let Some((peer, _)) = NODE_REGISTRY.get().unwrap().node(node).await {
                                    let _ = server.send(&peer, msg).await;
                                }
                            }
                        }
                    },
                    None => break,
                }
            }
        });

        Self(sender)
    }

    pub async fn connect_session(&self, session_id: Uuid) {
        let _ = self.0.send(Message::ConnectSession(session_id)).await;
    }

    pub async fn disconnect_session(&self, session_id: Uuid) {
        let _ = self.0.send(Message::DisconnectSession(session_id)).await;
    }

    pub async fn forward_message(&self, session_id: Option<Uuid>, destination: Destination, message: String) {
        let _ = self.0.send(Message::Forward { session_id, destination, message }).await;
    }

    
}