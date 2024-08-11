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

use std::sync::Arc;

use atlas::Uuid;
use atlas::{AvatarId, raknet::Message};

use log::{error, warn, trace};
use tokio::select;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::actors::ZoneRegistry;
use crate::cluster::RemoteActorRef;
use crate::util::{AnotherlandErrorKind, AnotherlandError};
use crate::{util::AnotherlandResult, NODE};
use crate::frontends::{TravelType, ZoneDownstreamMessage, ZoneServerClient, ZoneUpstreamMessage};

enum ZoneRouterCommand {
    ConnectZone { zone_id: Uuid, session_id: Uuid, avatar_id: AvatarId, retval: oneshot::Sender<AnotherlandResult<ZoneRouterConnection>> },
    ForwardMessageToZone { zone_id: Uuid, session_id: Uuid, message: Vec<u8>, retval: oneshot::Sender<AnotherlandResult<()>> },
    ForwardToClient { session_id: Uuid, message: Vec<u8> },
    RequestTravel { session_id: Uuid, zone_id: Uuid, travel: TravelType },
    NotifyTravel { zone_id: Uuid, session_id: Uuid, destination: TravelType, retval: oneshot::Sender<AnotherlandResult<()>> },
    DropConnection { zone_id: Uuid, session_id: Uuid },
    DropZone { zone_id: Uuid },
    IngameCommand { zone_id: Uuid, session_id: Uuid, command: String },
}

#[derive(Clone)]
pub(super) struct ZoneRouter {
    token: CancellationToken,
    tasks: TaskTracker,
    command_sender: mpsc::Sender<ZoneRouterCommand>,
}

impl ZoneRouter {
    pub fn new() -> Arc<Self> {
        let token = CancellationToken::new();
        let tasks = TaskTracker::new();

        let (command_sender, mut command_receiver) = mpsc::channel(100);

        tokio::spawn({
            let token = token.clone();
            let tasks = tasks.clone();
            let command_sender = command_sender.clone();

            let mut connections = ZoneConnectionRegistry::new();
            let mut session_connections = HashMap::<Uuid, (Uuid, mpsc::Sender<ZoneRouterMessage>)>::new();

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
                                            if connection.send(ZoneUpstreamMessage::EnterZone { 
                                                    session_id, 
                                                    avatar_id 
                                                }).await.is_err() {
                                                
                                                let _ = retval.send(Err(AnotherlandError::app_err("failed to enter zone")));
                                            } else {
                                                let (message_sender, message_receiver) = mpsc::channel(100);
            
                                                trace!(
                                                    session_id = session_id.to_string(), 
                                                    avatar_id = avatar_id.to_string(); 
                                                    "Returning connection handle for zone {}", zone_id);
            
                                                let _ = retval.send(Ok(ZoneRouterConnection {
                                                    zone_id,
                                                    session_id,
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
                                            let _ = retval.send(connection.send(ZoneUpstreamMessage::Message { 
                                                session_id, 
                                                message
                                            }).await.map_err(|_| AnotherlandErrorKind::IO.into()));
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
                                                if client.1.send(ZoneRouterMessage::Message(message)).await.is_err() {
                                                    session_connections.remove(&session_id);
                                                }
                                            },
                                            Err(_e) => {
                                                warn!("Received invalid message from zone!");
                                            }
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::NotifyTravel { zone_id, session_id, destination, retval }) => {
                                    match connections.get_zone_server_client(&zone_id) {
                                        Ok(connection) => {
                                            let _ = retval.send(connection.send(ZoneUpstreamMessage::Travel {
                                                session_id,
                                                destination
                                            }).await.map_err(|_| AnotherlandErrorKind::IO.into()));
                                        },
                                        Err(e) => {
                                            let _ = retval.send(Err(e));
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::DropConnection { zone_id, session_id }) => {
                                    // tell zone the client left
                                    if let Ok(connection) = connections.get_zone_server_client(&zone_id) {
                                        let _ = connection.send(ZoneUpstreamMessage::LeaveZone { 
                                            session_id 
                                        }).await;
                                    }
            
                                    // Remove connection when zone_id matches
                                    if session_connections.contains_key(&session_id) && session_connections.get(&session_id).unwrap().0 == zone_id {
                                        session_connections.remove(&session_id);
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
                                Some(ZoneRouterCommand::RequestTravel { session_id, zone_id, travel }) => {
                                    if let Some(client) = session_connections.get(&session_id) {
                                        if client.1.send(ZoneRouterMessage::TravelRequest { zone: zone_id, travel }).await.is_err() {
                                            session_connections.remove(&session_id);
                                        }
                                    }
                                },
                                Some(ZoneRouterCommand::IngameCommand { zone_id, session_id, command }) => {
                                    if let Ok(connection) = connections.get_zone_server_client(&zone_id) {
                                        let _ = connection.send(ZoneUpstreamMessage::IngameCommand { 
                                            session_id, 
                                            command
                                        }).await;
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

        Arc::new(ZoneRouter { 
            token, 
            tasks,
            command_sender,
        })
    }

    pub async fn connect_zone(&self, zone_id: &Uuid, session_id: &Uuid, avatar_id: &AvatarId) -> AnotherlandResult<ZoneRouterConnection> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::ConnectZone { 
            zone_id: *zone_id, 
            session_id: *session_id, 
            avatar_id: *avatar_id, 
            retval: retval_sender 
        }).await.map_err(|_| AnotherlandErrorKind::IO)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IO)?
    }
}

impl Drop for ZoneRouter {
    fn drop(&mut self) {
        // close tasks but don't wait for them, because drop can't wait on async methods
        // and there is no sync function for that purpose.
        self.token.cancel();
        self.tasks.close();
    }
}

struct ZoneConnectionRegistry {
    connections: HashMap<Uuid, mpsc::Sender<ZoneUpstreamMessage>>,
    zone_registry: RemoteActorRef<ZoneRegistry>,
}

impl ZoneConnectionRegistry {
    fn new() -> Self {
        Self { 
            connections: HashMap::new(), 
            zone_registry: NODE.get_remote_actor("zone_registry").expect("Zone registry not available!"),
        }
    }

    async fn get_or_create_zone_server_client(&mut self, zone_id: &Uuid, token: &CancellationToken, tasks: &TaskTracker, command_sender: &mpsc::Sender<ZoneRouterCommand>) -> AnotherlandResult<mpsc::Sender<ZoneUpstreamMessage>> {
        if let Some(connection) = self.connections.get(zone_id) {
            Ok(connection.to_owned())
        } else {
            trace!("Resolving server address for zone {}", zone_id);

            if let Some(addr) = self.zone_registry.resolve_zone_address(*zone_id).await {
                let mut client = ZoneServerClient::connect(addr).await?;
                let (zone_message_sender, mut zone_message_receiver) = mpsc::channel(100);

                self.connections.insert(*zone_id, zone_message_sender.clone());

                tasks.spawn({
                    let zone_id = *zone_id;
                    let token = token.to_owned();
                    let command_sender = command_sender.to_owned();

                    trace!("Starting net task for zone {}", zone_id);

                    async move {
                        'message_loop: loop {
                            select! {
                                message = client.recv() => {
                                       match message {
                                        Some(ZoneDownstreamMessage::Message { session_id, message }) => {
                                            let _ = command_sender.send(ZoneRouterCommand::ForwardToClient { session_id, message }).await;
                                        },
                                        Some(ZoneDownstreamMessage::RequestTravel { session_id, zone, travel }) => {
                                            let _ = command_sender.send(ZoneRouterCommand::RequestTravel { session_id, zone_id: zone, travel }).await;
                                        },
                                        Some(ZoneDownstreamMessage::ApiResult(_)) => unreachable!("cluster is not receiving api results!"),
                                        None => {
                                            zone_message_receiver.close();
                                            client.close().await;

                                            break 'message_loop;
                                        },
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
                        
                        let _ = command_sender.send(ZoneRouterCommand::DropZone { zone_id }).await;

                        trace!("Stopped net task for zone {}", zone_id);

                    }
                });
    
                Ok(zone_message_sender)
            } else {
                Err(AnotherlandError::app_err("zone not found"))
            }
        }
    }

    fn get_zone_server_client(&self, zone_id: &Uuid) -> AnotherlandResult<mpsc::Sender<ZoneUpstreamMessage>> {
        self.connections.get(zone_id).map(|v| v.to_owned()).ok_or(AnotherlandError::app_err("zone not found"))
    }
}

pub enum ZoneRouterMessage {
    Message(Message),
    TravelRequest{ zone: Uuid, travel: TravelType },
}

pub(super) struct ZoneRouterConnection {
    zone_id: Uuid,
    session_id: Uuid,
    avatar_id: AvatarId,
    command_sender: mpsc::Sender<ZoneRouterCommand>,
    message_receiver: Mutex<mpsc::Receiver<ZoneRouterMessage>>,
}

// this is a lie, please don't use this accross threads
//unsafe impl Sync for ZoneRouterConnection {}

impl ZoneRouterConnection {
    pub async fn send(&self, message: &Message) -> AnotherlandResult<()> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::ForwardMessageToZone { 
            zone_id: self.zone_id, 
            session_id: self.session_id, 
            message: message.to_bytes(), 
            retval: retval_sender,
        }).await.map_err(|_| AnotherlandErrorKind::IO)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IO)?
    }

    pub async fn ingame_command(&self, command: String) -> AnotherlandResult<()> {
        self.command_sender.send(ZoneRouterCommand::IngameCommand { 
            zone_id: self.zone_id, 
            session_id: self.session_id, 
            command,
        }).await.map_err(|_| AnotherlandErrorKind::IO)?;

        Ok(())
    }

    pub async fn receive(&self) -> Option<ZoneRouterMessage> {
        let mut receiver = self.message_receiver.lock().await;
        receiver.recv().await
    }

    pub async fn notify_travel(&self, destination: TravelType) -> AnotherlandResult<()> {
        let (retval_sender, retval_receiver) = oneshot::channel();

        self.command_sender.send(ZoneRouterCommand::NotifyTravel { 
            zone_id: self.zone_id, 
            session_id: self.session_id, 
            destination,
            retval: retval_sender 
        }).await.map_err(|_| AnotherlandErrorKind::IO)?;

        retval_receiver.await.map_err(|_| AnotherlandErrorKind::IO)?
    }
}

impl Drop for ZoneRouterConnection {
    fn drop(&mut self) {
        let cmd = ZoneRouterCommand::DropConnection { 
            zone_id: self.zone_id, 
            session_id: self.session_id 
        };

        let sender = self.command_sender.clone();

        tokio::spawn(async move {
            let _ = sender.send(cmd).await;
        });
    }
}