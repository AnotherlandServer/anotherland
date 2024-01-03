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

use std::{time::Duration, net::{SocketAddrV4, SocketAddr}};

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetPeer}, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktRealmStatusList, RealmStatus, oaCharacter, BoundParamClass, CPktStream_126_1, oaCharacterList, oaPktCharacterFailure, OaPktCharacterFailureErrorCode, CPktStream_126_5, oaPktCharacterDeleteSuccess, oaPktResponseSelectWorld, OaPktResponseSelectWorldErrorCode, Player, oaPktCharacterSelectSuccess};
use bitstream_io::{ByteWriter, LittleEndian};
use futures::future::Remote;
use log::{error, debug, warn};
use mongodb::Database;
use tokio::{time::{self, Interval}, select};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{cluster::{frontend::Frontend, MessageQueueProducer, connect_queue, MessageChannel, ClusterMessage, RemoteActorRef, ActorRef}, util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind}, components::{Authenticator, LoginResult, SessionHandler, RealmList, Realm}, CONF, NODE, db::{Session, Character, WorldDef, realm_database, DatabaseRecord}, ARGS};

#[derive(Default)]
struct RealmFrontendData {
}

pub struct RealmFrontend {
    //listener: RakNetListener,
    realm_db: Database,
    realm: Option<RemoteActorRef<Realm>>,
    session_handler: Option<ActorRef<SessionHandler<RealmFrontendData>>>,
    heartbeat_interval: Interval,
    cluster_sender: MessageQueueProducer,
    tasks: TaskTracker,
}

impl RealmFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self { 
            //listener: RakNetListener::bind(CONF.realm.listen_address).await?,
            realm_db: realm_database().await,
            realm: None,
            session_handler: None,
            heartbeat_interval: time::interval(Duration::from_secs(1)),
            cluster_sender: connect_queue(MessageChannel::ClusterChannel).await?.0,
            tasks: TaskTracker::new(),
        })
    }

    async fn send_heartbeat(&self, addr: &SocketAddr) {
        let active_sessions = self.session_handler.as_ref().unwrap().active_sessions().await;

        let _ = self.cluster_sender.send(ClusterMessage::RealmServerHearthbeat { 
            realm_id: CONF.realm.id, 
            name: CONF.realm.name.clone(), 
            channels: vec![(0, active_sessions as f32)], 
            address: SocketAddrV4::new(ARGS.external_ip, addr.port()),
        }).await;
    }
}

#[async_trait]
impl Frontend for RealmFrontend {
    fn name(&self) -> &str { "realm" }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        //self.listener.listen(CONF.realm.listen_address).await?;
        self.realm = Some(NODE.get_remote_actor("realm").unwrap());
        self.session_handler = Some(NODE.add_actor(SessionHandler::initialize("realm_session_handler").await));

        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let mut listener = RakNetListener::bind(CONF.realm.listen_address).await?;

        loop {
            select! {
                Ok(mut peer) = listener.accept() => {
                    let mut client_session = RealmFrontendSession {
                        realm_db: self.realm_db.clone(),
                        realm: self.realm.as_ref().unwrap().clone(),
                        session_handler: self.session_handler.as_ref().unwrap().clone(),
                    };
        
                    self.tasks.spawn(async move {
                        'net_loop: loop {
                            match peer.recv().await {
                                Ok(message) => {
                                    if let Err(e) = client_session.handle_request(&mut peer, message).await {
                                        error!("Failed to handle client request: {:#?}", e);
                                        break 'net_loop;
                                    }
                                },
                                Err(_) => {
                                    break 'net_loop;
                                }
                            }
                        }
        
                        debug!("Stopping client netloop");
        
                        if let Ok(session_ref_s) = client_session.session_handler.get(peer.id().clone()).await {
                            let session_ref = session_ref_s.lock().await;

                            // destroy session if the client disconnects without selecting a zone,
                            // otherwiese keep it, as the client will transition to a cluster server
                            // with the active session.
                            if session_ref.session().zone_guid.is_some() {
                                drop(session_ref); // explicitly drop session_ref to avoid a deadlock

                                client_session.session_handler.forget_peer(peer.id().clone()).await;
                            } else {
                                let _ = client_session.session_handler.destroy_session(session_ref.session().id.clone()).await;
                            }
                        } else {
                            debug!("Client session not found during disconnect!");
                        }

                        // cleanup connection
                        peer.disconnect().await;

                        debug!("Stopped client netloop");
                    });
                },
                _ = self.heartbeat_interval.tick() => {
                    self.send_heartbeat(listener.local_addr()).await
                },
                _ = token.cancelled() => break Ok(()),
            }
        }
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> {
        //let _ = self.listener.shutdown().await;

        self.tasks.close();
        self.tasks.wait().await;

        Ok(()) 
    }
}

struct RealmFrontendSession {
    realm_db: Database,
    realm: RemoteActorRef<Realm>,
    session_handler: ActorRef<SessionHandler<RealmFrontendData>>,
}

impl RealmFrontendSession {
    async fn handle_request(&mut self, peer: &mut RakNetPeer, message: Message) -> AnotherlandResult<()> {
        use Message::*;

        match message {
            AtlasPkt(CPkt::oaPktRequestCharacterList(pkt)) => {
                let session_ref_s = self.session_handler.initiate(peer.id().clone(), pkt.session_id, pkt.magic_bytes).await?;
                let session_ref = session_ref_s.lock().await;

                let characters: Vec<_> = self.realm.get_characters(session_ref.session().clone()).await?.into_iter().map(|c| {
                    let mut serialized = Vec::new();
                    let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);
                    c.data.write_to_client(&mut writer).expect("Serialization failed");

                    oaCharacter {
                        id: c.id,
                        name: c.name,
                        world_id: c.world_id,
                        length: serialized.len() as u32,
                        params: serialized,
                        field_5: 0
                    }
                }).collect();

                let mut character_list = CPktStream_126_1::default();
                character_list.list = oaCharacterList {
                    count: characters.len() as u32,
                    characters,
                };

                peer.send(Priority::High, Reliability::Reliable, character_list.into_message()).await?;

                Ok(())
            },
            AtlasPkt(CPkt::oaPktCharacterCreate(pkt)) => {
                let session_ref_s = self.session_handler.get(peer.id().clone()).await?;
                let session_ref = session_ref_s.lock().await;

                match self.realm.create_character(session_ref.session().clone(), pkt.character_name).await {
                    Ok(character) => {
                        let mut serialized = Vec::new();
                        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);
                        character.data.write(&mut writer).expect("Serialization failed");

                        let mut character_create_successful = CPktStream_126_5::default();
                        character_create_successful.character = oaCharacter {
                            id: character.id,
                            name: character.name.to_owned(),
                            world_id: character.world_id,
                            length: serialized.len() as u32,
                            params: serialized,
                            field_5: 0,
                        };

                        let _ = peer.send(Priority::High, Reliability::Reliable, character_create_successful.into_message()).await?;
                    },
                    Err(e) => match e.kind() {
                        AnotherlandErrorKind::DBError => {
                            // Todo: Check for duplicated key errors
                            let mut failure = oaPktCharacterFailure::default();
                            failure.error_code = OaPktCharacterFailureErrorCode::NameExists;

                            peer.send(Priority::High, Reliability::Reliable, failure.into_message()).await?;
                        },
                        _ => {
                            let mut failure = oaPktCharacterFailure::default();
                            failure.error_code = OaPktCharacterFailureErrorCode::DatabaseError;

                            peer.send(Priority::High, Reliability::Reliable, failure.into_message()).await?;
                        }
                    }
                }

                Ok(())
            },
            AtlasPkt(CPkt::oaPktCharacterDelete(pkt)) => {
                let session_ref_s = self.session_handler.get(peer.id().clone()).await?;
                let session_ref = session_ref_s.lock().await;

                if let Ok(_) = self.realm.delete_character(session_ref.session().clone(), pkt.character_id).await {
                    let mut response_character_delete = oaPktCharacterDeleteSuccess::default();
                    response_character_delete.character_id = pkt.character_id;

                    peer.send(Priority::High, Reliability::Reliable, response_character_delete.into_message()).await?;
                } else {
                    let mut failure = oaPktCharacterFailure::default();
                    failure.error_code = OaPktCharacterFailureErrorCode::DatabaseError;
    
                    peer.send(Priority::High, Reliability::Reliable, failure.into_message()).await?;
                }

                Ok(())
            },
            AtlasPkt(CPkt::oaPktRequestSelectWorld(pkt)) => {
                let session_ref_s = self.session_handler.get(peer.id().clone()).await?;
                let mut session_ref = session_ref_s.lock().await;

                match WorldDef::get(self.realm_db.clone(), &pkt.world_id).await? {
                    Some(_) => {
                        if  self.realm.get_cluster_frontend_address().await.is_none() {
                            let mut response_select_world = oaPktResponseSelectWorld::default();
                            response_select_world.success = false;
                            response_select_world.error_code = OaPktResponseSelectWorldErrorCode::ServerOffline;
                            response_select_world.field_3 = pkt.field_3.clone();
            
                            peer.send(Priority::High, Reliability::Reliable, response_select_world.into_message()).await?;
                        } else {
                            session_ref.select_world(pkt.world_id).await?;

                            let mut response_select_world = oaPktResponseSelectWorld::default();
                            response_select_world.success = true;
                            response_select_world.error_code = OaPktResponseSelectWorldErrorCode::NoError;
                            response_select_world.field_3 = pkt.field_3.clone();

                            peer.send(Priority::High, Reliability::Reliable, response_select_world.into_message()).await?;
                        }
                    },
                    None => {
                        // world not found
                        let mut response_select_world = oaPktResponseSelectWorld::default();
                        response_select_world.success = false;
                        response_select_world.error_code = OaPktResponseSelectWorldErrorCode::ServerOffline;
                        response_select_world.field_3 = pkt.field_3.clone();
        
                        peer.send(Priority::High, Reliability::Reliable, response_select_world.into_message()).await?;
                    }
                }

                Ok(())
            },
            AtlasPkt(CPkt::oaPktCharacterSelect(pkt)) => {
                let session_ref_s = self.session_handler.get(peer.id().clone()).await?;
                let mut session_ref = session_ref_s.lock().await;

                if session_ref.session().world_id.is_none() {
                    Err(AnotherlandError::app_err("no world selected"))
                } else {
                    if let Some(character) = self.realm.get_character(session_ref.session().clone(), pkt.field_1).await? {
                        // check if cluster server is online
                        if let Some(cluster_server) = self.realm.get_cluster_frontend_address().await {
                            session_ref.select_character(character.id).await?;
                            session_ref.select_zone(character.data.zone_guid().unwrap().to_owned()).await?;

                            let mut character_select_success = oaPktCharacterSelectSuccess::default();
                            character_select_success.world_ip = u32::from_be(cluster_server.ip().clone().into());
                            character_select_success.world_port = cluster_server.port();
                            character_select_success.session_id = session_ref.session().id.clone();
    
                            peer.send(Priority::High, Reliability::Reliable, character_select_success.into_message()).await?;
                        } else {
                            error!(peer = peer.id(), session = session_ref.session().id, character = character.guid; "Character select failed, cluster server is offline");
                            peer.disconnect().await;
                        }

                    } else {
                        error!(peer = peer.id(), session = session_ref.session().id; "Character select failed, character not found: {}", pkt.field_1);
                        peer.disconnect().await;
                    }

                    Ok(())
                }
            },
            AtlasPkt(CPkt::oaPktSendMsgToRealm(pkt)) => {
                debug!(peer = peer.id(); "Client Message: {}", pkt.message);
                Ok(())
            }
            _ => {
                warn!(peer = peer.id(); "Unhandled message: {:#?}", message);
                Err(AnotherlandError::app_err("unknown message"))
            },
        }
    }
}