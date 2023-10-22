use std::{net::{SocketAddrV4, SocketAddr, Ipv4Addr}, collections::HashMap, time::{Instant, Duration}};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian};
use bson::doc;
use log::{info, debug, warn, error};
use mongodb::{options::UpdateOptions, Database};
use std::fs;
use std::error::Error;

use crate::{util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind::{ApplicationError, self}}, CONF, ARGS, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue}, db::{WorldDef}};
use crate::db::{Account, cluster_database, Session, DatabaseRecord, realm_database, Character};
use atlas::{CPkt, Uuid, PlayerParam, oaCharacter, CPktStream_126_1, oaCharacterList, CPktStream_126_5, oaPktResponseSelectWorld, oaPktCharacterSelectSuccess, ParamClass, Player};
use atlas::raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest};
use atlas::oaPktCharacterFailure;
use atlas::BoundParamClass;

#[derive(Clone)]
struct ClientState {
    account: Account,
    session: Session,
    character: Option<Uuid>,
    world: Option<WorldDef>,
}

pub struct RealmServer {
    realm_id: u32,
    realm_name: String,
    external_ip: Ipv4Addr,
    external_port: u16,

    listener: RakNetListener,
    cluster_db: Database,
    realm_db: Database, 

    frontend_servers: Vec<(Instant, SocketAddrV4)>,

    client_state: HashMap<Uuid, ClientState>,
    cluster: MessageQueueProducer,
}

impl RealmServer {
    async fn authenticate_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<(Uuid, ClientState)> {
        let peer_id = request.peer().read().await.guid().to_owned();

        // Do we have a client state?
        if self.client_state.contains_key(&peer_id) {
            return Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()));
        }

        // Does the message contain a session id?
        use Message::*;
        let session_id = match request.message() {
            AtlasPkt(CPkt::oaPktRequestCharacterList(pkt)) => {
                Ok(pkt.session_id.clone())
            },
            _ => {
                Err(AnotherlandError::new(ApplicationError, "message without session id"))
            }
        }?;

        // Lookup session
        match Session::get(self.cluster_db.clone(), &session_id).await? {
            Some(session) => {
                self.client_state.insert(peer_id.clone(), ClientState { 
                    account: Account::get_by_id(self.cluster_db.clone(), &session.account).await?.unwrap(), 
                    session,
                    character: None,
                    world: None,
                });

                Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()))
            },
            None => {
                Err(AnotherlandError::new(ApplicationError, "unknown session"))
            }
        }
    }
}

#[async_trait]
impl ServerInstance for RealmServer {
    type ServerProperties = ();

    async fn init(properties: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        info!("Starting realm server [{}]...", CONF.realm.name);

        let mut listener = RakNetListener::new();
        listener.listen(&CONF.realm.listen_address).await?;

        let (cluster, _) = connect_queue(MessageChannel::ClusterChannel).await?;

        Ok(Box::new(Self {
            realm_id: CONF.realm.id.into(),
            realm_name: CONF.realm.name.clone(),
            external_ip: ARGS.external_ip.to_string().parse().unwrap(),
            external_port: listener.local_addr().await.unwrap().port(),
            listener,
            cluster_db: cluster_database().await,
            realm_db: realm_database().await,
            frontend_servers: Vec::new(),
            client_state: HashMap::new(),
            cluster
        }))
    }

    async fn close(&mut self) {

    }

    fn raknet_listener(&self) -> Option<&RakNetListener> {
        Some(&self.listener)
    }

    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        let (peer_id, mut state) = match self.authenticate_request(&request).await {
            Ok(state) => state,
            Err(e) => {
                warn!("Failed to authenticate client: {}", e);

                // Close client connection when we can't authenticate them
                request.peer().write().await.disconnect().await;

                return Ok(())
            }
        };

        println!("Message: {:#?}", request.message());
        match request.message() {
            AtlasPkt(CPkt::oaPktRequestCharacterList(_)) => {
                let characters: Vec<oaCharacter> = Character::list(self.realm_db.clone(), &state.account.id).await?.into_iter().map(|c| {
                    let mut serialized = Vec::new();
                    let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);
                    c.data.write(&mut writer).expect("Serialization failed");

                    fs::write("chardata2.bin", serialized.clone());
                    
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

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_list.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktCharacterCreate(pkt)) => {
                debug!("Character create: {}", pkt.character_name);

                match Character::create(self.realm_db.clone(), &state.account.id, &pkt.character_name).await {
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
                        
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_create_successful.as_message()).await?;
                    },
                    Err(e) => match e.kind() {
                        AnotherlandErrorKind::DBError => {
                            // Todo: Check for duplicated key errors
                            let mut failure = oaPktCharacterFailure::default();
                            failure.field_1 = 1;
                            let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, failure.as_message()).await?;
                        },
                        _ => {
                            let mut failure = oaPktCharacterFailure::default();
                            failure.field_1 = 0; // not sure what 0 means
                            let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, failure.as_message()).await?;
                        }
                    }
                }
            },
            AtlasPkt(CPkt::oaPktRequestSelectWorld(pkt)) => {
                // Load selected world
                match WorldDef::get(self.realm_db.clone(), &pkt.world_id).await? {
                    Some(world) => {
                        // update session
                        state.session.select_world(self.cluster_db.clone(), pkt.world_id).await?;

                        debug!("Select world: {}", pkt.field_3.to_string());

                        // send response
                        let mut response_select_world = oaPktResponseSelectWorld::default();
                        response_select_world.field_1 = true;
                        response_select_world.field_2 = 0;
                        response_select_world.field_3 = pkt.field_3.clone();
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response_select_world.as_message()).await?;

                        state.world = Some(world);
                    },
                    None => {
                        // World not found. 
                        let mut response_select_world = oaPktResponseSelectWorld::default();
                        response_select_world.field_1 = false;
                        response_select_world.field_2 = 0;
                        response_select_world.field_3 = pkt.field_3.clone();
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response_select_world.as_message()).await?;
                    }
                }
            },
            AtlasPkt(CPkt::oaPktCharacterSelect(pkt)) => {
                if state.world.is_none() {
                    error!("Client tried to select character before selecting a world!");
                    let _ = request.peer().write().await.disconnect().await;
                } else {
                    // Load selected character
                    match Character::get(self.realm_db.clone(), &pkt.field_1).await? {
                        Some(character) => {
                            // validate, that this account owns the character
                            if character.account == state.account.id {

                                if self.frontend_servers.is_empty() {
                                    warn!("No frontend server available. Closing conneciton");
                                    request.peer().write().await.disconnect().await;
                                } else {
                                    let frontend_server = self.frontend_servers[0].1;

                                    info!("Character {} selected! Routing frontend server {}...", character.id, frontend_server);

                                    // select character
                                    state.session.select_character(self.cluster_db.clone(), character.id).await?;
                                    
                                    // select zone
                                    state.session.select_zone(self.cluster_db.clone(), character.data.zone_guid().unwrap().to_owned()).await?;

                                    // send response
                                    let mut character_select_success = oaPktCharacterSelectSuccess::default();
                                    character_select_success.world_ip = u32::from_be(frontend_server.ip().clone().into());
                                    character_select_success.world_port = frontend_server.port();
                                    character_select_success.session_id = state.session.id.clone();
                    
                                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_select_success.as_message()).await?;
                                }

                                // determine world server
                                /*match WorldServerEntry::get(self.realm_db.clone(), &state.world.as_ref().unwrap().id).await? {
                                    Some(worldserver) => {
                                        info!("Character {} selected! Routing to world {}...", character.id, state.world.as_ref().unwrap().id);

                                        // select character
                                        state.session.select_character(self.cluster_db.clone(), character.id).await?;

                                        // send response
                                        let mut character_select_success = oaPktCharacterSelectSuccess::default();
                                        character_select_success.world_ip = u32::from_be(worldserver.external_ip.parse::<Ipv4Addr>().unwrap().into());
                                        character_select_success.world_port = worldserver.external_port;
                                        character_select_success.session_id = state.session.id.clone();
                        
                                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_select_success.as_message()).await?;
                                    },
                                    None => {
                                        error!("Client tried to load into unavailable world {}", state.world.as_ref().unwrap().id);
                                        let _ = request.peer().write().await.disconnect().await;
                                    }
                                }*/
                            } else {
                                error!("Account {} tried to select character {} that is owned by {}!", state.account.id, pkt.field_1, character.account);
                                let _ = request.peer().write().await.disconnect().await;
                            }
                        },
                        None => {
                            error!("Tried to load unknown character: {}", pkt.field_1);
                            let _ = request.peer().write().await.disconnect().await;
                        }
                    }
                }
            },
            _ => debug!("Unhandled request: {:#?}", request.message()),
        }

        self.client_state.insert(peer_id, state);

        Ok(())
    }

    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> {
        match message {
            ClusterMessage::InvalidateSession{session_id} => {
                // Is the session id registered with us?
                match self.client_state.iter().find(|v| v.1.session.id == session_id).map(|v| v.0.clone()) {
                    Some(peer_id) => {
                        // Remove state and close connection
                        if let Some(peer) = self.listener.peer(&peer_id).await {
                            peer.write().await.disconnect().await;
                        }

                        self.client_state.remove(&peer_id);

                        Ok(())
                    },
                    None => Ok(()),
                }
            },
            ClusterMessage::FrontendServerHearthbeat { realm_id, address } => {
                // Only look for frontend servers set to our realm
                if realm_id == self.realm_id {
                    match self.frontend_servers.iter_mut().find(|i| i.1 == address) {
                        Some(frontend_server) => {
                            frontend_server.0 = Instant::now();
                            Ok(())
                        },
                        None => {
                            self.frontend_servers.push((Instant::now(), address));
                            Ok(())
                        }
                    }
                } else {
                    Ok(())
                }
            },
            _ => Ok(())
        }
    }

    fn get_subscribed_channels(&self) -> Vec<MessageChannel> {
        vec![
            MessageChannel::ClusterChannel, 
            MessageChannel::RealmChannel { 
                realm_id: self.realm_id, 
                channel: RealmChannel::GlobalChannel,
            }
        ]
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        // announce our presence
        self.cluster.send(ClusterMessage::RealmServerHearthbeat { 
            realm_id: self.realm_id, 
            name: self.realm_name.clone(), 
            population: self.client_state.len(),
            address: SocketAddrV4::new(self.external_ip, self.external_port), 
        }).await?;

        self.frontend_servers.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Ok(())
    }
}