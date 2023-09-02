use std::{sync::Arc, fs, time::{Instant, SystemTime, UNIX_EPOCH}};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use glam::f32::Vec3;

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, CPkt, CPktStream_126_5, oaCharacter, CPktStream_126_1, oaPktResponseSelectWorld, oaPktRequestEnterGame, oaPktLoginQueueUpdate, CParam, CParamClass_player, Uuid, CPktStream_167_0, CPktResourceNotify, CpktResourceNotifyResourceType, CPktStackedAvatarUpdate, CPktAggregated, oaCharacterList, CPktAvatarUpdate, CParamClass_npcOtherland, CPktServerNotify}};
use crate::atlas::CPktBlob;
use crate::atlas::oaPktS2XConnectionState;
use crate::atlas::oaPktClusterNodeToClient;

pub struct WorldServer {
    listener: RakNetListener,
}

struct WorldServerMessageHandler {
    client_loadstate: u32,
    last_loadstate: u32,
    last_update: Instant,
}

#[async_trait]
impl RequestHandler for WorldServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::AtlasPkt(pkt) => {
                //println!("World pkt: \n{:#?}", pkt);

                match &pkt {
                    CPkt::oaPktRequestEnterGame(pkt) => {
                        let mut worlddef = CPktResourceNotify::default();
                        worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                        worlddef.field_2 = Uuid::from_str("deb7a33c-4e3e-47a3-8ae9-2156d802f3d0").unwrap();
                        worlddef.field_3 = "Hello!".to_owned();

                        response.add_message(Reliability::Reliable, worlddef.as_message());

                        let mut character = CParamClass_player::default();

                        character.alive = Some(CParam::Bool(true));
                        character.lvl = Some(CParam::Int32(1));
                        character.tutorial_mode = Some(CParam::Bool(true));
                        character.world_map_guid = Some(CParam::CGuid(Uuid::from_str("deb7a33c-4e3e-47a3-8ae9-2156d802f3d0").unwrap()));
                        character.zone_guid = Some(CParam::CGuid(Uuid::from_str("3abb7eb1-662a-48ff-bdec-c1eac42c42d1").unwrap()));
                        //character.party_guid = Some(CParam::CGuid(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()));
                        character.client_ready = Some(CParam::Bool(false));
                        character.collision_extent = Some(CParam::Vector3(Vec3::new(21.0, 21.0, 44.0)));
                        //character.death_info = Some(CParam::Any(0, Vec::new()));
                        character.generate_interest_list = Some(CParam::Bool(true));
                        character.host_ip = Some(CParam::String(peer.remote_address().ip.to_string()));
                        character.in_game_session = Some(CParam::Bool(false));
                        //character.instance_zone_key = Some(CParam::String("".to_owned()));
                        //character.pos = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                        //character.rot = Some(CParam::Vector3(Vec3::new(0.0, 0.0, 0.0)));
                        character.self_radius = Some(CParam::Float(20.0));
                        character.team_id = Some(CParam::Int32(0));
                        character.zone = Some(CParam::String("MeetingRoom".to_owned()));
                        character.spawn_mode = Some(CParam::Int32(2));

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
                        avatarBlob.avatar_name = "Peter".to_owned();
                        avatarBlob.avatar_id = 1;
                        avatarBlob.class_id = 77;
                        avatarBlob.data_len1 = character_data.len() as u32;
                        avatarBlob.data1 = character_data;
                        avatarBlob.data_len2 = buf.len() as u32;
                        avatarBlob.data2 = buf;
                        avatarBlob.has_guid = true;
                        avatarBlob.field_9 = Some(pkt.field_1.clone());

                        response.add_message(Reliability::Reliable, avatarBlob.as_message());

                        let mut gameTimeSync = CPktServerNotify::default();
                        gameTimeSync.notify_type = 0;
                        gameTimeSync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

                        response.add_message(Reliability::Reliable, gameTimeSync.as_message());

                        let mut realmTimeSync = CPktServerNotify::default();
                        realmTimeSync.notify_type = 19;
                        realmTimeSync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                        response.add_message(Reliability::Reliable, realmTimeSync.as_message());
                    },
                    CPkt::oaPktFriendRequest(pkt) => {
                        let mut friendList = CPktStream_167_0::default();
                        friendList.friend_list.count = 0;

                        response.add_message(Reliability::Reliable, friendList.as_message());
                    },
                    CPkt::oaPktClientServerPing(pkt) => {
                        response.add_message(Reliability::Reliable, pkt.clone().as_message());
                    },
                    CPkt::oaPktServerAction(pkt) => {
                        let mut action = pkt.clone();
                        action.version = 3;
                        response.add_message(Reliability::Reliable, action.as_message());
                    },
                    /*CPkt::oaPktClienToClusterNode(pkt) => {
                        let mut node_to_client = oaPktClusterNodeToClient::default();
                        node_to_client.field_3 = pkt.field_3.clone();
                        response.add_message(Reliability::Reliable, node_to_client.to_message());
                    }*/
                    CPkt::oaPktC2SConnectionState(pkt) => {
                        self.client_loadstate = pkt.field_1;
                        /*let mut state = oaPktS2XConnectionState::default();
                        state.field_1 = 6;
                        state.field_2 = 0;

                        self.client_loadstate = 6;

                        response.add_message(Reliability::Reliable, state.as_message());*/


                        // oaPktClusterNodeToClient 0 = quest template?



                        /*let mut aggregated = CPktAggregated::default();
                        response.add_message(Reliability::Reliable, aggregated.to_message());*/

                    },
                    _ => (),
                }
            }
            _ => {},
        }

        Ok(())
    }

    async fn update_client<'a>(&'a mut self, peer: &RakNetPeer, update: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        if Instant::now().duration_since(self.last_update).as_secs() > 1 {
            self.last_update = Instant::now();

            if self.client_loadstate != self.last_loadstate {
                self.last_loadstate = self.client_loadstate;

                match self.client_loadstate {
                    5 => {
                        {
                            let mut character = CParamClass_npcOtherland::default();

                            character.alive = Some(CParam::Bool(true));
                            character.lvl = Some(CParam::Int32(1));
                            character.zone_guid = Some(CParam::CGuid(Uuid::from_str("3abb7eb1-662a-48ff-bdec-c1eac42c42d1").unwrap()));
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
                            avatar_update.avatar_id = Some(1);
                            avatar_update.field_2 = Some(false);
                            avatar_update.field_4 = Some("Max".to_owned());
                            avatar_update.field_5 = Some(47);
                            avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.flags = Some(4);
                            avatar_update.field_9 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.data_len = Some(character_data.len() as u32);
                            avatar_update.field_12 = Some(character_data);
                            
                            avatar_update.field_14 = 0;
                            avatar_update.data_len2 = buf.len() as u32;
                            avatar_update.field_16 = buf;

                            update.add_message(Reliability::Reliable, avatar_update.as_message());
                        }

                        {
                            let mut state = oaPktS2XConnectionState::default();
                            state.field_1 = 6;
                            state.field_2 = 0;

                            self.client_loadstate = 6;

                            update.add_message(Reliability::Reliable, state.as_message());
                        }
                    },
                    6 => {
                        {
                            let mut character = CParamClass_npcOtherland::default();

                            character.alive = Some(CParam::Bool(true));
                            character.lvl = Some(CParam::Int32(1));
                            character.zone_guid = Some(CParam::CGuid(Uuid::from_str("3abb7eb1-662a-48ff-bdec-c1eac42c42d1").unwrap()));
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
                            avatar_update.avatar_id = Some(2);
                            avatar_update.field_2 = Some(false);
                            avatar_update.field_4 = Some("Max2".to_owned());
                            avatar_update.field_5 = Some(47);
                            avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.flags = Some(4);
                            avatar_update.field_9 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.data_len = Some(character_data.len() as u32);
                            avatar_update.field_12 = Some(character_data);
                            
                            avatar_update.field_14 = 0;
                            avatar_update.data_len2 = buf.len() as u32;
                            avatar_update.field_16 = buf;

                            update.add_message(Reliability::Reliable, avatar_update.as_message());
                        }

                        {
                            let mut state = oaPktS2XConnectionState::default();
                            state.field_1 = 7;
                            state.field_2 = 0;

                            self.client_loadstate = 7;

                            update.add_message(Reliability::Reliable, state.as_message());        
                        }
                    },
                    7 => {
                        let mut state = oaPktS2XConnectionState::default();
                        state.field_1 = 8;
                        state.field_2 = 0;

                        self.client_loadstate = 8;

                        update.add_message(Reliability::Reliable, state.as_message());        
                    },
                    8 => (),
                    _ => panic!(),
                }
            }
        }

        /*let mut queue_update = oaPktLoginQueueUpdate::default();
                                        
        queue_update.channel = 1;
        queue_update.number = 1;
        queue_update.field38_0x2c = self.temp_field_count;

        update.add_message(Reliability::Reliable, queue_update.to_message());

        self.temp_field_count += 1;*/

        Ok(())
    }
}

impl WorldServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<WorldServer> {
        let mut listener = RakNetListener::bind(Box::new(WorldServerMessageHandler {
            client_loadstate: 0,
            last_loadstate: 0,
            last_update: Instant::now(),
        }), addr).await?;

        Ok(Self {listener})
    }
}
