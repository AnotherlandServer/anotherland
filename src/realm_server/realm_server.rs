use std::{net::{SocketAddrV4, SocketAddr, Ipv4Addr}, collections::HashMap};

use async_trait::async_trait;
use bson::doc;
use log::{info, debug, warn, error};
use mongodb::{options::UpdateOptions, Database};
use std::error::Error;

use crate::{util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind::{ApplicationError, self}}, CONF, ARGS, cluster::{ServerInstance, ClusterMessage}, db::{WorldDef, WorldServerEntry}};
use crate::db::{Account, Realm, cluster_database, Session, DatabaseRecord, realm_database, Character};
use atlas::{CPkt, Uuid, CParamClass_player, CParam, oaCharacter, CPktStream_126_1, oaCharacterList, CPktStream_126_5, oaPktResponseSelectWorld, oaPktCharacterSelectSuccess};
use atlas::raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest};
use atlas::oaPktCharacterFailure;

#[derive(Clone)]
struct ClientState {
    account: Account,
    session: Session,
    character: Option<Uuid>,
    world: Option<WorldDef>,
}

pub struct RealmServer {
    listener: RakNetListener,
    realm: Realm,
    cluster_db: Database,
    realm_db: Database,

    client_state: HashMap<Uuid, ClientState>,
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
    async fn init() -> AnotherlandResult<Box<Self>> {
        info!("Starting realm server [{}]...", CONF.realm.name);

        let mut listener = RakNetListener::new();
        listener.listen(&CONF.realm.listen_address).await?;

        let realm = Realm {
            id: CONF.realm.id.into(),
            name: CONF.realm.name.clone(),
            population: 1.0,
            external_ip: ARGS.external_ip.to_string(),
            external_port: listener.local_addr().await.unwrap().port(),
        };

        info!("Registering realm server [{}]...", realm.name);

        let collection = cluster_database().await.collection::<Realm>("realms");
        collection.update_one(
            doc!{"id": {"$eq": realm.id} }, 
            doc!{"$set": &bson::to_bson(&realm).unwrap().as_document()}, 
            UpdateOptions::builder().upsert(true).build()
        ).await?;

        Ok(Box::new(Self {
            listener,
            realm,
            cluster_db: cluster_database().await,
            realm_db: realm_database().await,
            client_state: HashMap::new(),
        }))
    }

    async fn close(&mut self) {

    }

    async fn next_request(&mut self) -> AnotherlandResult<Option<RakNetRequest>> {
        Ok(self.listener.next_request().await)
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
                    let serialized = c.data.to_bytes();
                    
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
                        let serialized = character.data.to_bytes();
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
                                // determine world server
                                match WorldServerEntry::get(self.realm_db.clone(), &state.world.as_ref().unwrap().id).await? {
                                    Some(worldserver) => {
                                        info!("Character {} selected! Joining routing to world {}...", character.id, state.world.as_ref().unwrap().id);

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
                                }
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
            ClusterMessage::InvalidateSession(id) => {
                // Is the session id registered with us?
                match self.client_state.iter().find(|v| v.1.session.id == id).map(|v| v.0.clone()) {
                    Some(peer_id) => {
                        // Remove state and close connection
                        if let Some(peer) = self.listener.peer(&peer_id) {
                            peer.write().await.disconnect().await;
                        }

                        self.client_state.remove(&peer_id);

                        Ok(())
                    },
                    None => Ok(()),
                }
            }
            _ => Ok(())
        }
    }

    async fn tick(&mut self) -> Result<(), AnotherlandError> {
        Ok(())
    }
}