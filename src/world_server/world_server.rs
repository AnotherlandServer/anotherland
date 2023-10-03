use std::{time::{Duration, SystemTime, UNIX_EPOCH}, collections::HashMap, fs, sync::Arc};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use glam::Vec3;
use log::{info, debug, trace, error, warn};
use mongodb::Database;
use nom::{multi::length_count, number::complete::le_u8};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::JoinHandle, time::{Interval, self, Instant}};

use crate::{CONF, cluster::{ServerInstance, ClusterMessage}, WORLD_SERVER_IDS, db::{WorldDef, DatabaseRecord, realm_database, WorldServerEntry, Account, Session, cluster_database, Character}, ARGS, util::{AnotherlandError, AnotherlandErrorKind::ApplicationError}};
use raknet::*;
use atlas::*;
use crate::util::AnotherlandResult;

#[derive(Clone)]
struct ClientState {
    account: Account,
    session: Session,
    character: Character,
}

pub struct WorldServer {
    listener: RakNetListener,
    worlddef: WorldDef,

    cluster_db: Database,
    realm_db: Database,

    client_state: HashMap<Uuid, ClientState>,

    /*client_loadstate: u32,
    last_loadstate: u32,
    last_update: Instant,

    hp_test: i32,
    move_mgr: Option<Vec<u8>>,

    clients: HashMap<Uuid, RakNetPeerHandle>,*/
}

impl WorldServer {
    async fn authenticate_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<(Uuid, ClientState)> {
        let peer_id = request.peer().read().await.guid().to_owned();

        // Do we have a client state?
        if self.client_state.contains_key(&peer_id) {
            return Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()));
        }

        // Does the message contain a session id?
        use Message::*;
        let session_id = match request.message() {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => Ok(pkt.session_id.clone()),
            _ => Err(AnotherlandError::new(ApplicationError, "message without session id"))
        }?;

        // Lookup session
        match Session::get(self.cluster_db.clone(), &session_id).await? {
            Some(session) => {
                // validate world id
                if session.world_id.is_none() {
                    return Err(AnotherlandError::new(ApplicationError, "no world selected"));
                }

                if session.world_id.unwrap() != self.worlddef.id {
                    return Err(AnotherlandError::new(ApplicationError, "client connected to wrong world server"));
                }

                // validate a character is selected
                if session.character_id.is_none() {
                    return Err(AnotherlandError::new(ApplicationError, "no character selected"));
                }

                // Lookup character
                match Character::get(self.realm_db.clone(), &session.character_id.unwrap()).await? {
                    Some(character) => {
                        self.client_state.insert(peer_id.clone(), ClientState { 
                            account: Account::get_by_id(self.cluster_db.clone(), &session.account).await?.unwrap(), 
                            session,
                            character,
                        });
        
                        Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()))
                    },
                    None => {
                        Err(AnotherlandError::new(ApplicationError, "selected character not found"))
                    }
                }
            },
            None => {
                Err(AnotherlandError::new(ApplicationError, "unknown session"))
            }
        }
    }
}

#[async_trait]
impl ServerInstance for WorldServer {
    async fn init() -> AnotherlandResult<Box<Self>> {
        let id = WORLD_SERVER_IDS.write().await
                .pop_front()
                .expect("Tried to spawn more world servers than available worlds");

        let db = realm_database().await;

        let worlddef = WorldDef::get(db.clone(), &id).await?
            .expect("Unable to find world definition");

        info!("Starting world server [{}]...", worlddef.name);

        let mut base_addr = CONF.world.base_listen_address;
        base_addr.set_port(base_addr.port() + id - 1);

        // Register our worldserver
        WorldServerEntry::create(db.clone(), WorldServerEntry {
            world_id: id,
            external_ip: ARGS.external_ip.to_string(),
            external_port: base_addr.port(),
        }).await?;

        // Start server
        let mut listener = RakNetListener::new();
        listener.listen(base_addr).await?;

        Ok(Box::new(Self {
            listener,
            worlddef,
            cluster_db: cluster_database().await,
            realm_db: realm_database().await,
            client_state: HashMap::new(),
            /*task: None,
            client_loadstate: 0,
            last_loadstate: 0,
            last_update: Instant::now(),
            hp_test: 0,
            clients: HashMap::new(),
            move_mgr: None,*/
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
            AtlasPkt(CPkt::oaPktRequestEnterGame(_)) => {
                // Transfer character to client
                let mut avatar_blob = CPktBlob::default();
                //avatar_blob.avatar_id = state.character.id;


                /*self.clients.insert(request.peer().read().await.guid().to_owned(), request.peer());

                let mut worlddef = CPktResourceNotify::default();
                worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                worlddef.field_2 = Uuid::from_str("58340efa-9495-47e2-b4e4-723a2978a6f1").unwrap();
                worlddef.field_3 = "Hello!".to_owned();

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, worlddef.as_message()).await?;

                let mut character = CParamClass_player::default();

                character.alive = Some(CParam::Bool(true));
                character.lvl = Some(CParam::Int32(1));
                character.tutorial_mode = Some(CParam::Bool(false));
                character.world_map_guid = Some(CParam::CGuid(Uuid::from_str("58340efa-9495-47e2-b4e4-723a2978a6f1").unwrap()));
                character.zone_guid = Some(CParam::CGuid(Uuid::from_str("b1bbd5c5-0990-454b-bcfa-5dfe176c6756").unwrap()));
                character.client_ready = Some(CParam::Bool(true));
                character.collision_extent = Some(CParam::Vector3(Vec3::new(21.0, 21.0, 44.0)));
                character.generate_interest_list = Some(CParam::Bool(true));
                character.host_ip = Some(CParam::String(request.peer().read().await.remote_address().ip.to_string()));
                character.in_game_session = Some(CParam::Bool(false));
                character.self_radius = Some(CParam::Float(20.0));
                character.team_id = Some(CParam::Int32(0));
                character.zone = Some(CParam::String("MeetingRoom".to_owned()));
                character.spawn_mode = Some(CParam::Int32(2));
                character.player_loading = Some(CParam::Bool(false));
                character.client_ready = Some(CParam::Bool(true));
                character.ue3class_id = Some(CParam::String("Engine.AtlasAvatar".to_owned()));
                character.player_node_state = Some(CParam::Int32(2));
                character.visible_item_info = Some(CParam::IntArray(vec![1527, 1531, 1649]));

                let character_data = character.to_bytes();

                let mut buf = Vec::new();
                let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
        
                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());

                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());

                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());
                writer.write_bytes(0f32.to_le_bytes().as_slice());

                writer.write(0u64);
                writer.write(0u8);
                writer.write(0u8);
                writer.write(0u8);
                writer.write(0u16);
                writer.write(1u16);
                writer.write(0u64);

                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_name = "Test".to_owned();
                avatar_blob.avatar_id = 1;
                avatar_blob.class_id = 77;
                avatar_blob.data_len1 = character_data.len() as u32;
                avatar_blob.data1 = character_data;
                avatar_blob.data_len2 = buf.len() as u32;
                avatar_blob.data2 = buf;
                avatar_blob.has_guid = true;
                avatar_blob.field_9 = Some(pkt.field_1.clone());

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatar_blob.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktFriendRequest(pkt)) => {
                /*let mut friend_list = CPktStream_167_0::default();
                friend_list.friend_list.count = 0;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, friend_list.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktClientServerPing(pkt)) => {
                /*let response = pkt.clone();
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                /*let mut action = pkt.clone();
                action.version = 2;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                /*self.client_loadstate = pkt.field_1;


                let mut response = pkt.clone();
                response.field_2 += 1;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                /*if pkt.avatar_id.is_none() {
                    if self.hp_test < 1000 {
                        self.hp_test += 1;
                    }

                    let mut response = CPktAvatarUpdate::default();
                    response.avatar_id = Some(1);
                    response.full_update = false;
                    response.data_len2 = pkt.data_len2;
                    response.field_16 = pkt.field_16.clone();
                    response.field_14 = 1;

                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                }*/
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunity(pkt)) => {
                /*let mut response = oaPktCommunityToClusterClient::default();
                response.field_1 = pkt.field_1.clone();
                response.field_2 = "Hi!".to_owned();
                response.field_3 = pkt.field_3.clone();
                response.field_4 = pkt.field_4;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunication(pkt)) => {
                /*let mut response = oaPktCommunicationToClusterClient::default();
                response.field_1 = pkt.field_1.clone();
                response.field_2 = "Hi!".to_owned();
                response.field_3 = NativeParam::Struct(vec![
                    NativeParam::Int(0),
                    NativeParam::String("Hi!".to_owned()),
                ]);
                response.field_4 = pkt.field_4;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktClienToClusterNode(pkt)) => {
                /*match pkt.field_2 {
                    5 => {
                        let mut response = oaPktClusterNodeToClient::default();
                        response.field_1 = Uuid::new_v4();
                        response.field_3 = NativeParam::Struct(vec![
                            NativeParam::Int(0xa8)
                        ]);
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                    },
                    _ => (),
                }*/

            },
            AtlasPkt(CPkt::oaPktFactionRequest(pkt)) => {
                /*let factions = vec![FactionRelation {
                    field_0: Uuid::from_str("be55863a-03a0-4f2a-807c-b794e84f537c").unwrap(),
                    field_1: "Player".to_owned(),
                    field_2: 6000.0,
                }];

                let faction_list = FactionRelationList {
                    count: factions.len() as u32,
                    factions,
                };

                let mut faction_response = oaPktFactionResponse::default();
                faction_response.field_1 = pkt.field_1;
                faction_response.field_2 = pkt.field_2;
                faction_response.field_3 = NativeParam::Struct(vec![
                    NativeParam::Buffer(faction_list.to_bytes())
                ]);

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, faction_response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                //println!("Routed pkt {:#?}", Message::from_bytes(&pkt.field_4).unwrap())
                /*match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                        println!("Pos update x: {} y: {} z: {}", pkt.field_1[0], pkt.field_1[1], pkt.field_1[2]);
                    },
                    _ => (),
                }*/
            }
            _ => (), //debug!("Unhandled request: {:#?}", request.message()),
        }

        self.client_state.insert(peer_id, state);

        Ok(())
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        Ok(())
    }

    /*async fn update_peers(&mut self) -> AnotherlandResult<()> {
        let mut remove_clients = Vec::new();

        for client in self.clients.iter() {
            if client.1.read().await.state() != State::Connected {
                remove_clients.push(client.0.to_owned());
                continue;
            }

            if Instant::now().duration_since(self.last_update).as_millis() > 10 && self.client_loadstate < 8 {
                self.last_update = Instant::now();
    
                if self.client_loadstate != self.last_loadstate {
                    self.last_loadstate = self.client_loadstate;
    
                    match self.client_loadstate {
                        5 => {
                            /*{
                                let character_data = fs::read("npc_lambda_bartender_lovenurseclinic.bin").unwrap();
    
                                let mut buf = Vec::new();
                                let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                        
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write(0u64);
                                writer.write(0u8);
                                writer.write(0u8);
                                writer.write(0u8);
                                writer.write(0u16);
                                writer.write(1u16);
                                writer.write(0u64);
    
                                let mut avatar_update = CPktAvatarUpdate::default();
                                avatar_update.full_update = true;
                                avatar_update.avatar_id = Some(0xf00);
                                avatar_update.field_2 = Some(true);
                                avatar_update.field_4 = Some("Test1".to_owned());
                                avatar_update.field_5 = Some(47);
                                avatar_update.field_6 = Some(Uuid::new_v4());
                                avatar_update.flags = Some(3);
                                avatar_update.field_10 = Some(Uuid::from_str("7a6aa750-5cab-42e3-899c-9b3d7ed0dc7a").unwrap());
                                avatar_update.data_len = Some(character_data.len() as u32);
                                avatar_update.field_12 = Some(character_data);
                                
                                avatar_update.field_14 = 0;
                                avatar_update.data_len2 = buf.len() as u32;
                                avatar_update.field_16 = buf;

                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;
                            }*/

                            {
                                let mut state = oaPktS2XConnectionState::default();
                                state.field_1 = 6;
                                state.field_2 = 0;
    
                                self.client_loadstate = 6;
    
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, state.as_message()).await?;
                            }
                        },
                        6 => {
                            /*{
                                let mut character = CParamClass_npcOtherland::default();
    
                                character.alive = Some(CParam::Bool(true));
                                character.lvl = Some(CParam::Int32(3));
                                //character.zone_guid = Some(CParam::CGuid(Uuid::from_str("3abb7eb1-662a-48ff-bdec-c1eac42c42d1").unwrap()));
                                //character.party_guid = Some(CParam::CGuid(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()));
                                character.collision_extent = Some(CParam::Vector3(Vec3::new(21.0, 21.0, 44.0)));
                                //character.death_info = Some(CParam::Any(0, Vec::new()));
                                character.generate_interest_list = Some(CParam::Bool(false));
                                //character.instance_zone_key = Some(CParam::String("".to_owned()));
                                //character.pos = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                                //character.rot = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                                character.self_radius = Some(CParam::Float(20.0));
                                character.team_id = Some(CParam::Int32(0));
                                character.zone = Some(CParam::String("MeetingRoom".to_owned()));
                                character.ue3class_id = Some(CParam::String("Otherland.OLAvatarNPC".to_owned()));
    
                                let character_data = character.to_bytes();
    
                                let mut buf = Vec::new();
                                let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                        
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
                                writer.write_bytes(0f32.to_le_bytes().as_slice());
    
                                writer.write(0u64);
                                writer.write(0u8);
                                writer.write(0u8);
                                writer.write(0u8);
                                writer.write(0u16);
                                writer.write(1u16);
                                writer.write(0u64);
    
                                let mut avatar_update = CPktAvatarUpdate::default();
                                avatar_update.full_update = true;
                                avatar_update.avatar_id = Some(3);
                                avatar_update.field_2 = Some(false);
                                avatar_update.field_4 = Some("Test2".to_owned());
                                avatar_update.field_5 = Some(47);
                                avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                                avatar_update.flags = Some(4);
                                avatar_update.field_9 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                                avatar_update.data_len = Some(character_data.len() as u32);
                                avatar_update.field_12 = Some(character_data);
                                
                                avatar_update.field_14 = 0;
                                avatar_update.data_len2 = buf.len() as u32;
                                avatar_update.field_16 = buf;
    
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;
                            }*/
    
                            /*{
                                let mut state = oaPktS2XConnectionState::default();
                                state.field_1 = 7;
                                state.field_2 = 0;
    
                                self.client_loadstate = 7;
    
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, state.as_message()).await?;   
                            }*/
                        },
                        7 => {
                            {
                                let mut game_time_sync = CPktServerNotify::default();
                                game_time_sync.notify_type = 0;
                                game_time_sync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                
                
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, game_time_sync.as_message()).await?;
                            }
            
                            {
                                let mut realm_time_sync = CPktServerNotify::default();
                                realm_time_sync.notify_type = 19;
                                realm_time_sync.field_4 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, realm_time_sync.as_message()).await?;
                            }

                            {
                                let ui_config = oaPktUIConfigUpdate::default();
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, ui_config.as_message()).await?;
                            }

                            {
                                let mut state = oaPktS2XConnectionState::default();
                                state.field_1 = 8;
                                state.field_2 = 0;
    
                                self.client_loadstate = 8;
    
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, state.as_message()).await?;
                            }

                            // Tell the client the player has arrived
                            {
                                let mut action = oaPktServerAction::default();
                                action.action = "TRAVEL:DirectTravel|DirectTravelDefault".to_owned();
                                action.version = 4;
                                action.override_teleport = false;
                                action.field_7 = vec![f32::INFINITY, f32::INFINITY, f32::INFINITY];
                                action.field_8 = vec![1.0f32, 0.0f32, 0.0f32, 0.0f32];
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await?;
                            }

                        },
                        _ => println!("Undefined load state: {}", self.client_loadstate),
                    }
                }
            }    
        }

        for r in &remove_clients {
            self.clients.remove(r);
        }

        Ok(())
    }*/

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
}