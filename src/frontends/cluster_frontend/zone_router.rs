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

use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::DerefMut;

use atlas::{AvatarId, raknet::Message};
use futures::future::Remote;
use log::{error, warn, trace};
use tokio::select;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::Uuid;

use crate::cluster::RemoteActorRef;
use crate::util::{AnotherlandErrorKind, AnotherlandError};
use crate::{util::AnotherlandResult, components::ZoneRegistry, NODE};
use crate::frontends::{ZoneServerClient, ZoneMessage};

enum ZoneRouterCommand {
    ConnectZone { zone_id: Uuid, session_id: Uuid, avatar_id: AvatarId, retval: oneshot::Sender<AnotherlandResult<ZoneRouterConnection>> },
    ForwardMessageToZone { zone_id: Uuid, session_id: Uuid, message: Vec<u8>, retval: oneshot::Sender<AnotherlandResult<()>> },
    ForwardToClient { session_id: Uuid, message: Vec<u8> },
    NotifyTravel { zone_id: Uuid, session_id: Uuid, retval: oneshot::Sender<AnotherlandResult<()>> },
    DropConnection { zone_id: Uuid, session_id: Uuid },
    DropZone { zone_id: Uuid },
}

enum ZoneRouterEvent {
    
}

#[derive(Clone)]
pub(super) struct ZoneRouter {
    token: CancellationToken,
    tasks: TaskTracker,
    command_sender: mpsc::Sender<ZoneRouterCommand>,
}

impl ZoneRouter {
    pub fn new() -> Self {
        let token = CancellationToken::new();
        let tasks = TaskTracker::new();

        let (command_sender, mut command_receiver) = mpsc::channel(10);

        tokio::spawn({
            let token = token.clone();
            let tasks = tasks.clone();
            let command_sender = command_sender.clone();

            let mut connections = ZoneConnectionRegistry::new();
            let mut session_connections = HashMap::<Uuid, (Uuid, mpsc::Sender<Message>)>::new();

            async move {
                'event_loop: loop {
                    select! {
                        result = command_receiver.recv() => {
                            match result {
                                Some(ZoneRouterCommand::ConnectZone { zone_id, session_id, avatar_id, retval }) => {
                                    match connections.get_or_create_zone_server_client(&zone_id, &token, &tasks, &command_sender).await {
                                        Ok(connection) => {
                                            trace!(
                                                session_id = session_id.to_string(), 
                                                avatar_id = avatar_id.to_string(); 
                                                "Entering zone {}", zone_id);
            
                                            // Notify zone server that a client is entering
                                            if let Err(_) = connection.send(ZoneMessage::EnterZone { 
                                                    session_id: session_id.clone(), 
                                                    avatar_id: avatar_id.clone() 
                                                }).await {
                                                
                                                let _ = retval.send(Err(AnotherlandError::app_err("failed to enter zone")));
                                            } else {
                                                let (message_sender, message_receiver) = mpsc::channel(10);
            
                                                trace!(
                                                    session_id = session_id.to_string(), 
                                                    avatar_id = avatar_id.to_string(); 
                                                    "Returning connection handle for zone {}", zone_id);
            
                                                let _ = retval.send(Ok(ZoneRouterConnection {
                                                    zone_id: zone_id.clone(),
                                                    session_id: session_id.clone(),
                                                    avatar_id,
                                                    command_sender: command_sender.clone(),
                                                    message_receiver: Mutex::new(message_receiver),
                                                }));
            
                                                // A client can only ever be connected to one zone at a time.
                                                session_connections.insert(session_id, (zone_id, message_sender));
                                            }
                                        },
                                        Err(e) => {
                                            let _ = retval.send(Err(e));
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::ForwardMessageToZone { zone_id, session_id, message, retval}) => {
                                    match connections.get_zone_server_client(&zone_id) {
                                        Ok(connection) => {
                                            let _ = retval.send(connection.send(ZoneMessage::Message { 
                                                session_id, 
                                                message
                                            }).await.map_err(|_| AnotherlandErrorKind::IOError.into()));
                                        },
                                        Err(e) => {
                                            let _ = retval.send(Err(e));
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::ForwardToClient { session_id, message }) => {
                                    if let Some(client) = session_connections.get(&session_id) {
                                        match Message::from_bytes(&message) {
                                            Ok((_, message)) => {
                                                if client.1.send(message).await.is_err() {
                                                    session_connections.remove(&session_id);
                                                }
                                            },
                                            Err(e) => {
                                                warn!("Received invalid message from zone!");
                                            }
                                        }
                                        
                                    }
                                },
                                Some(ZoneRouterCommand::NotifyTravel { zone_id, session_id, retval }) => {
                                    match connections.get_zone_server_client(&zone_id) {
                                        Ok(connection) => {
                                            let _ = retval.send(connection.send(ZoneMessage::Travel {
                                                session_id
                                            }).await.map_err(|_| AnotherlandErrorKind::IOError.into()));
                                        },
                                        Err(e) => {
                                            let _ = retval.send(Err(e));
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::DropConnection { zone_id, session_id }) => {
                                    // tell zone the client left
                                    if let Ok(connection) = connections.get_zone_server_client(&zone_id) {
                                        let _ = connection.send(ZoneMessage::LeaveZone { 
                                            session_id: session_id.clone() 
                                        }).await;
                                    }
            
                                    // Remove connection when zone_id matches
                                    if session_connections.contains_key(&session_id) {
                                        if session_connections.get(&session_id).unwrap().0 == zone_id {
                                            session_connections.remove(&session_id);
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::DropZone { zone_id }) => {
                                    // collect all sessions connected to affected zone
                                    let session_ids: Vec<_> = session_connections.iter().filter(|(_, v)| v.0 == zone_id).map(|(k, _)| k.to_owned()).collect();

                                    // drop sessions
                                    for session_id in session_ids {
                                        session_connections.remove(&session_id);
                                    }
                                },
                                None => break 'event_loop,
                            }
                        },
                        _ = token.cancelled() => {
                            break 'event_loop;
                        }
                    }
                }
            }
        });

        ZoneRouter { 
            token, 
            tasks,
            command_sender,
        }
    }

    pub async fn connect_zone(&self, zone_id: &Uuid, session_id: &Uuid, avatar_id: &AvatarId) -> AnotherlandResult<ZoneRouterConnection> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::ConnectZone { 
            zone_id: zone_id.clone(), 
            session_id: session_id.clone(), 
            avatar_id: avatar_id.clone(), 
            retval: retval_sender 
        }).await.map_err(|_| AnotherlandErrorKind::IOError)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IOError)?
    }
}

struct ZoneConnectionRegistry {
    connections: HashMap<Uuid, mpsc::Sender<ZoneMessage>>,
    zone_registry: RemoteActorRef<ZoneRegistry>,
}

impl ZoneConnectionRegistry {
    fn new() -> Self {
        Self { 
            connections: HashMap::new(), 
            zone_registry: NODE.get_remote_actor("zone_registry").expect("Zone registry not available!"),
        }
    }

    async fn get_or_create_zone_server_client(&mut self, zone_id: &Uuid, token: &CancellationToken, tasks: &TaskTracker, command_sender: &mpsc::Sender<ZoneRouterCommand>) -> AnotherlandResult<mpsc::Sender<ZoneMessage>> {
        if let Some(connection) = self.connections.get(zone_id) {
            Ok(connection.to_owned())
        } else {
            trace!("Resolving server address for zone {}", zone_id);

            if let Some(addr) = self.zone_registry.resolve_zone_address(zone_id.clone()).await {
                let mut client = ZoneServerClient::connect(addr).await?;
                let (zone_message_sender, mut zone_message_receiver) = mpsc::channel(10);

                self.connections.insert(zone_id.clone(), zone_message_sender.clone());

                tasks.spawn({
                    let zone_id = zone_id.clone();
                    let token = token.to_owned();
                    let command_sender = command_sender.to_owned();

                    trace!("Starting net task for zone {}", zone_id);

                    async move {
                        'message_loop: loop {
                            select! {
                                message = client.recv() => {
                                       match message {
                                        Some(ZoneMessage::Message { session_id, message }) => {
                                            let _ = command_sender.send(ZoneRouterCommand::ForwardToClient { session_id, message }).await;
                                        },
                                        None => {
                                            zone_message_receiver.close();
                                            client.close().await;

                                            break 'message_loop;
                                        },
                                        _ => {},
                                    }
                                },
                                Some(message) = zone_message_receiver.recv() => {
                                    if let Err(e) = client.send(&message).await {
                                        error!(
                                            zone_id = zone_id.to_string(); 
                                            "Zone connection closed: {:#?}", e);

                                        zone_message_receiver.close();
                                    }
                                },
                                _ = token.cancelled() => {
                                    zone_message_receiver.close();
                                    client.close().await;
                                    break 'message_loop;
                                }
                            }
                        }

                        trace!("Stopping net task for zone {}", zone_id);

                        zone_message_receiver.close();
                        client.close().await;
                        
                        let _ = command_sender.send(ZoneRouterCommand::DropZone { zone_id: zone_id.clone() }).await;

                        trace!("Stopped net task for zone {}", zone_id);

                    }
                });
    
                Ok(zone_message_sender)
            } else {
                Err(AnotherlandError::app_err("zone not found"))
            }
        }
    }

    fn get_zone_server_client(&self, zone_id: &Uuid) -> AnotherlandResult<mpsc::Sender<ZoneMessage>> {
        self.connections.get(zone_id).map(|v| v.to_owned()).ok_or(AnotherlandError::app_err("zone not found"))
    }
}

pub(super) struct ZoneRouterConnection {
    zone_id: Uuid,
    session_id: Uuid,
    avatar_id: AvatarId,
    command_sender: mpsc::Sender<ZoneRouterCommand>,
    message_receiver: Mutex<mpsc::Receiver<Message>>,
}

// this is a lie, please don't use this accross threads
//unsafe impl Sync for ZoneRouterConnection {}

impl ZoneRouterConnection {
    pub async fn send(&self, message: &Message) -> AnotherlandResult<()> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::ForwardMessageToZone { 
            zone_id: self.zone_id.clone(), 
            session_id: self.session_id.clone(), 
            message: message.to_bytes(), 
            retval: retval_sender,
        }).await.map_err(|_| AnotherlandErrorKind::IOError)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IOError)?
    }

    pub async fn receive(&self) -> Option<Message> {
        let mut receiver = self.message_receiver.lock().await;
        receiver.recv().await
    }

    pub async fn notify_travel(&self) -> AnotherlandResult<()> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::NotifyTravel { 
            zone_id: self.zone_id.clone(), 
            session_id: self.session_id.clone(), 
            retval: retval_sender 
        }).await.map_err(|_| AnotherlandErrorKind::IOError)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IOError)?
    }
}

impl Drop for ZoneRouterConnection {
    fn drop(&mut self) {
        let cmd = ZoneRouterCommand::DropConnection { 
            zone_id: self.zone_id.clone(), 
            session_id: self.session_id.clone() 
        };

        let sender = self.command_sender.clone();

        tokio::spawn(async move {
            let _ = sender.send(cmd).await;
        });
    }
}