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

use std::sync::Arc;

use core_api::{CoreApi, Session};
use log::{debug, error};
use obj_params::Player;
use protocol::CPkt;
use raknet::RakNetSocket;
use realm_api::{proto::{RealmClient, RealmRequest}, RealmApi};
use tokio::{select, sync::mpsc::{self, Sender}};
use toolkit::types::Uuid;
use world_service::{ClusterMessage, TravelMode, TravelRejectReason, WorldMessage, WorldRequest};

use crate::{error::ClusterFrontendResult, router::Router};

pub enum Message {
    Sidechannel(CPkt),
}

pub struct ClusterContext;

impl ClusterContext {
    pub async fn create_and_start(
        _core_api: CoreApi, 
        realm_api: RealmApi, 
        router: Router,
        socket: RakNetSocket,
        session: Session,
        realm_client: Arc<RealmClient>,
    ) -> ClusterFrontendResult<Sender<Message>> {
        let (sender, mut receiver) = mpsc::channel(10);

        // Get extended session from realm
        let session_state = realm_api.get_session_state(*session.id()).await?
            .ok_or(anyhow::Error::msg("session ext not found"))?;

        // Get selected character
        let character = realm_api.get_character(
                session_state.character()
            ).await?
            .ok_or(anyhow::Error::msg("character not found"))?;

        let mut channel = router.open_instance_channel(
            *session.id(), 
            *character.data().get(Player::ZoneGuid)?, 
            character.data().get::<_, String>(Player::InstanceZoneKey)?
                .parse::<Uuid>()
                .ok()
            ).await?;

        drop(character);

        // Notify realm
        let _ = realm_client.send(RealmRequest::ClientConnected { 
            session_id: *session.id()
        }).await;

        // Notify world
        let _ = channel.send(ClusterMessage::ClientArrived { 
            session: *session.id(),
            zone: channel.instance().zone,
            instance: channel.instance().key,
            mode: TravelMode::Login
        }).await;

        let (next_instance_send, mut next_instance_recv) = mpsc::channel(1);

        tokio::spawn(async move {
            loop {
                select! {
                    res = receiver.recv() => {
                        if let Some(msg) = res {
                            match msg {
                                Message::Sidechannel(pkt) => {
                                    let _ = socket.send(&pkt.to_bytes(), raknet::Reliability::ReliableOrdered).await;
                                }
                            }
                        } else {
                            break;
                        }
                    },
                    res = channel.recv() => {
                        match res {
                            Some(res) => {
                                match res {
                                    WorldMessage::ServerMessage { data } => {
                                        let _ = socket.send(&data, raknet::Reliability::ReliableOrdered).await;
                                    },
                                    WorldMessage::TravelRequest { zone, instance, mode } => {
                                        let channel_id = channel.id();
                                        let session_id = *session.id();
                                        let next_instance_send = next_instance_send.clone();
                                        let router = router.clone();
                                        let world_sender = channel.detached_sender();

                                        tokio::spawn(async move {
                                            match router.open_instance_channel(session_id, zone, instance).await {
                                                    Ok(res) => {
                                                        // Queue next instance
                                                        let _ = next_instance_send.send((res, mode)).await; 

                                                        // Tell currently connected world we're ready
                                                        let _ = world_sender.send(WorldRequest::RouterChannel {
                                                            id: channel_id,
                                                            msg: ClusterMessage::TravelAccepted
                                                        }).await;
                                                    },
                                                    Err(_) => {
                                                        let _ = world_sender.send(WorldRequest::RouterChannel {
                                                            id: channel_id,
                                                            msg: ClusterMessage::TravelRejected {
                                                                reason: TravelRejectReason::ZoneOffline
                                                            }
                                                        }).await;
                                                    }
                                                }
                                        });
                                    },
                                    WorldMessage::TravelCommited => {
                                        debug!("Travel committed! Waiting for instance to connect...");

                                        // Get queued instance
                                        match next_instance_recv.recv().await {
                                            Some((new_channel, mode)) => {
                                                debug!("Switching session '{}' to instance {}:{:?}", 
                                                    socket.id(), 
                                                    new_channel.instance().zone, 
                                                    new_channel.instance().key
                                                );
                
                                                // Notify world we're leaving
                                                let _ = channel.send(ClusterMessage::ClientLeft).await;

                                                channel = new_channel;

                                                // Tell new world about our arrival
                                                let _ = channel.send(ClusterMessage::ClientArrived { 
                                                    session: *session.id(),
                                                    zone: channel.instance().zone,
                                                    instance: channel.instance().key,
                                                    mode
                                                }).await;
                                            }
                                            None => {
                                                error!("Instance switcher channel closed unexpectedly!");
                                                break;
                                            }
                                        }
                                    },
                                    WorldMessage::Close => {
                                        error!("World node closed connection.");
                                        socket.close().await;
                                        break;
                                    }
                                }
                            }
                            None => {
                                error!("World node closed connection.");
                                socket.close().await;
                                break;
                            }
                        }
                    }
                    req = socket.recv() => {
                        match req {
                            Ok(pkt) => {
                                if let Err(e) = channel.send(ClusterMessage::Forward { data: pkt }).await {
                                    error!("Failed to handle client request: {e:#?}");
                                    socket.close().await;
                                    break;
                                }
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    },
                }
            }

            // Notify world and realm that client has disconnected
            let _ = channel.send(ClusterMessage::ClientLeft).await;

            let _ = realm_client.send(RealmRequest::ClientDisconnected { 
                session_id: *session.id()
            }).await;
        });

        Ok(sender)
    }
}