use std::{sync::{Arc}, time::{Duration, SystemTime, UNIX_EPOCH}, collections::HashMap, fs};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use glam::Vec3;
use log::{info, debug, trace, error};
use nom::{multi::length_count, number::complete::le_u8};
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};
use tokio::{sync::RwLock, task::JoinHandle, time::{Interval, self, Instant}};

use crate::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest, RakNetPeerHandle, State}, util::AnotherlandResult, CONF, atlas::{CPktLogin, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktS2XConnectionState, Uuid, oaPktRealmStatusList, CPktResourceNotify, CpktResourceNotifyResourceType, oaPktFactionResponse, CParamClass_player, CParam, CPktBlob, CPktServerNotify, CPktStream_167_0, CPktAvatarUpdate, oaPktUIConfigUpdate, CParamClass_npcOtherland, oaPktServerAction, CPktStackedAvatarUpdate, oaPktClusterNodeToClient, NativeParam, FactionRelationList, oaPktMoveManagerPosUpdate}, db::{AccountRecord}, DB, login_server::Session};
use crate::atlas::FactionRelation;

pub struct WorldServer {
    internal: Arc<RwLock<WorldServerInternal>>
}

impl WorldServer {
    pub async fn init() -> AnotherlandResult<Self> {
        Ok(WorldServer {
            internal: WorldServerInternal::init().await?
        })
    }
}

pub struct WorldServerInternal {
    listener: RakNetListener,
    task: Option<JoinHandle<()>>,

    client_loadstate: u32,
    last_loadstate: u32,
    last_update: Instant,

    clients: HashMap<Uuid, RakNetPeerHandle>,
}

impl WorldServerInternal {
    async fn handle_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        //println!("Message: {:#?}", request.message());
        match request.message() {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => {
                self.clients.insert(request.peer().read().await.guid().to_owned(), request.peer());

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
                //character.party_guid = Some(CParam::CGuid(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()));
                character.client_ready = Some(CParam::Bool(true));
                character.collision_extent = Some(CParam::Vector3(Vec3::new(21.0, 21.0, 44.0)));
                //character.death_info = Some(CParam::Any(0, Vec::new()));
                character.generate_interest_list = Some(CParam::Bool(true));
                character.host_ip = Some(CParam::String(request.peer().read().await.remote_address().ip.to_string()));
                character.in_game_session = Some(CParam::Bool(false));
                //character.instance_zone_key = Some(CParam::String("".to_owned()));
                //character.pos = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                //character.rot = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                character.self_radius = Some(CParam::Float(20.0));
                character.team_id = Some(CParam::Int32(0));
                character.zone = Some(CParam::String("MeetingRoom".to_owned()));
                character.spawn_mode = Some(CParam::Int32(2));
                character.player_loading = Some(CParam::Bool(false));
                character.client_ready = Some(CParam::Bool(true));
                character.ue3class_id = Some(CParam::String("Engine.AtlasAvatar".to_owned()));
                character.player_node_state = Some(CParam::Int32(2));
                //character.content_class = Some(CParam::String("CharCustInfoPlayer.CharCustPlayer.Meshes.Player".to_string()));
                //character.current_skin = Some(CParam::String("CharCustInfoPlayer.CharCustPlayerCharSkinHuman0001.Units.PlayerCharSkinHuman0005".to_string()));
                character.visible_item_info = Some(CParam::IntArray(vec![1527, 1531, 1649]));
                //character.content_class = Some(CParam::String("CharCustInfoPlayer.CharCustPlayerCharSkinHuman0001.Units.PlayerCharSkinHuman0005".to_owned()));
                //character.current_skin = Some(CParam::String("CharCustInfoPlayer.CharCustPlayerCharSkinHuman0001.Units.PlayerCharSkinHuman0005".to_owned()));


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

                let mut avatarBlob = CPktBlob::default();
                avatarBlob.avatar_name = "Test".to_owned();
                avatarBlob.avatar_id = 1;
                avatarBlob.class_id = 77;
                avatarBlob.data_len1 = character_data.len() as u32;
                avatarBlob.data1 = character_data;
                avatarBlob.data_len2 = buf.len() as u32;
                avatarBlob.data2 = buf;
                avatarBlob.has_guid = true;
                avatarBlob.field_9 = Some(pkt.field_1.clone());

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatarBlob.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktFriendRequest(pkt)) => {
                let mut friendList = CPktStream_167_0::default();
                friendList.friend_list.count = 0;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, friendList.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClientServerPing(pkt)) => {
                let mut response = pkt.clone();
                response.field_1 += 1;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;

                println!("Ping! {:#?}", pkt);
                //let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, pkt.clone().as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                let mut action = pkt.clone();
                action.version = 2;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await?;
            },
            /*CPkt::oaPktClienToClusterNode(pkt) => {
                let mut node_to_client = oaPktClusterNodeToClient::default();
                node_to_client.field_3 = pkt.field_3.clone();
                response.add_message(Reliability::Reliable, node_to_client.to_message());
            }*/
            AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                self.client_loadstate = pkt.field_1;


                let mut response = pkt.clone();
                response.field_2 += 1;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                /*let mut state = oaPktS2XConnectionState::default();
                state.field_1 = 6;
                state.field_2 = 0;

                self.client_loadstate = 6;

                response.add_message(Reliability::Reliable, state.as_message());*/


                // oaPktClusterNodeToClient 0 = quest template?



                /*let mut aggregated = CPktAggregated::default();
                response.add_message(Reliability::Reliable, aggregated.to_message());*/

            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                let mut update = pkt.clone();
                update.avatar_id = Some(1);
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, update.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClienToClusterNode(pkt)) => {
                
            },
            AtlasPkt(CPkt::oaPktFactionRequest(pkt)) => {
                let factions = vec![FactionRelation {
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

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, faction_response.as_message()).await?;
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                //println!("Routed pkt {:#?}", Message::from_bytes(&pkt.field_4).unwrap())
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                        println!("Pos update x: {} y: {} z: {}", pkt.field_1[0], pkt.field_1[1], pkt.field_1[2]);
                    },
                    _ => (),
                }
            }
            _ => (), //debug!("Unhandled request: {:#?}", request.message()),
        }

        Ok(())
    }

    async fn update_peers(&mut self) -> AnotherlandResult<()> {
        let mut remove_clients = Vec::new();

        for client in self.clients.iter() {
            if client.1.read().await.state() != State::Connected {
                remove_clients.push(client.0.to_owned());
            }

            if Instant::now().duration_since(self.last_update).as_millis() > 200 {
                self.last_update = Instant::now();
    
                if self.client_loadstate != self.last_loadstate {
                    self.last_loadstate = self.client_loadstate;
    
                    match self.client_loadstate {
                        5 => {
                            {
                                /*let mut character = CParamClass_npcOtherland::default();
    
                                character.alive = Some(CParam::Bool(true));
                                character.lvl = Some(CParam::Int32(1));
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
    
                                let character_data = character.to_bytes();*/
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
                                avatar_update.avatar_id = Some(2);
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
                            }

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
                                let mut gameTimeSync = CPktServerNotify::default();
                                gameTimeSync.notify_type = 0;
                                gameTimeSync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                
                
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, gameTimeSync.as_message()).await?;
                            }
            
                            {
                                let mut realmTimeSync = CPktServerNotify::default();
                                realmTimeSync.notify_type = 19;
                                realmTimeSync.field_4 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, realmTimeSync.as_message()).await?;
                            }

                            {
                                let uiConfig = oaPktUIConfigUpdate::default();
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, uiConfig.as_message()).await?;
                            }

                            {
                                let mut state = oaPktS2XConnectionState::default();
                                state.field_1 = 8;
                                state.field_2 = 0;
    
                                self.client_loadstate = 8;
    
                                let _ = client.1.write().await.send(Priority::High, Reliability::Reliable, state.as_message()).await?;
                            }

                        },
                        8 => {
                            {
                                let mut action = oaPktServerAction::default();
                                //action.action = "LMPlatform_P".to_owned();
                                action.action = "TRAVEL:DirectTravel|DirectTravelDefault".to_owned();
                                //action.field_5 = "DirectTravelDefault".to_owned();
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
    }

    pub async fn init() -> AnotherlandResult<Arc<RwLock<Self>>> {
        info!("Starting world server...");

        let mut inst = Arc::new(RwLock::new(Self {
            listener: RakNetListener::new(),
            task: None,
            client_loadstate: 0,
            last_loadstate: 0,
            last_update: Instant::now(),
            clients: HashMap::new(),
        }));

        inst.write().await.listener.listen(&CONF.world.listen_address).await?;

        let task_handle = {
            let inst = inst.clone();

            Some(tokio::spawn(async move {
                let listener = inst.read().await.listener.clone();
                let mut game_ticks = time::interval(Duration::from_millis((1000.0 / 30.0) as u64));

                loop {
                    while let Some(request) = listener.try_next_request().await {
                        if let Err(e) = inst.write().await.handle_request(&request).await {
                            error!("Error handling request from peer {}: {:#?}", request.peer().read().await.guid(), e);
                        }
                    }

                    inst.write().await.update_peers().await;

                    game_ticks.tick().await;
                }

                trace!("Stopping world server loop...");
            }))
        };

        inst.write().await.task = task_handle;

        Ok(inst)
    }
}