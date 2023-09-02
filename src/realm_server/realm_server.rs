use std::{sync::Arc, fs, net::Ipv4Addr};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, CPkt, CPktStream_126_5, oaCharacter, CPktStream_126_1, oaPktResponseSelectWorld, oaPktCharacterSelectSuccess, CParamClass_player, CParam, Uuid, oaCharacterList, CPktBlob}};

pub struct RealmServer {
    listener: RakNetListener,
}

struct RealmServerMessageHandler {

}

#[async_trait]
impl RequestHandler for RealmServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::AtlasPkt(pkt) => {
                //println!("Realm pkt: \n{:#?}", pkt);

                match &pkt {
                    CPkt::oaPktRequestCharacterList(pkt) => {

                        // Create params
                        let mut character = CParamClass_player::default();
                        character.alive = Some(CParam::Bool(true));
                        character.lvl = Some(CParam::Int32(1));
                        character.tutorial_mode = Some(CParam::Bool(true));

                        let character_data = character.to_bytes();

                        println!("{:#?}", character_data);

                        let character = oaCharacter {
                            field_0: 0,
                            name: "Peter".to_owned(),
                            world_id: 128,
                            length: character_data.len() as u32,
                            params: character_data,
                            field_5: 0,
                        };

                        let mut character_list = CPktStream_126_1::default();
                        character_list.list = oaCharacterList {
                            count: 1,
                            characters: vec![character],
                        };


                        response.add_message(Reliability::Reliable, character_list.as_message())
                    },
                    CPkt::oaPktCharacterCreate(pkt) => {
                        println!("Character create: {}", pkt.character_name);

                        // Create params
                        let mut player = CParamClass_player::default();
                        player.alive = Some(CParam::Bool(true));
                        player.lvl = Some(CParam::Int32(1));
                        player.world_map_guid = Some(CParam::CGuid(Uuid::from_str("02298957-0506-4a43-b4b0-5bde5acf4b0b").unwrap()));
                        player.zone_guid = Some(CParam::CGuid(Uuid::from_str("38282cff-3eab-475b-b684-f9ca86b6ca00").unwrap()));
                        player.tutorial_mode = Some(CParam::Bool(true));

                        let player_data = player.to_bytes();

                        println!("{:#?}", player_data);

                        let mut character_create_successful = CPktStream_126_5::default();
                        character_create_successful.character = oaCharacter {
                            field_0: 0,
                            name: pkt.character_name.clone(),
                            world_id: 65,
                            length: player_data.len() as u32,
                            params: player_data,
                            field_5: 0,
                        };

                        response.add_message(Reliability::Reliable, character_create_successful.as_message())
                    },
                    CPkt::oaPktRequestSelectWorld(pkt) => {
                        let mut response_select_world = oaPktResponseSelectWorld::default();
                        response_select_world.field_1 = true;
                        response_select_world.field_2 = 0;
                        response_select_world.field_3 = pkt.field_3.clone();
                        response.add_message(Reliability::Reliable, response_select_world.as_message())
                    },
                    CPkt::oaPktCharacterSelect(pkt) => {
                        let mut character_select_success = oaPktCharacterSelectSuccess::default();

                        character_select_success.world_ip = u32::from_be(Ipv4Addr::new(192, 168, 178, 45).into());
                        character_select_success.world_port = 6114;
                        character_select_success.magic_bytes = Vec::new();
                        character_select_success.magic_bytes.resize(16, 0);

                        response.add_message(Reliability::Reliable, character_select_success.as_message())
                    },
                    _ => (),
                }
            }
            _ => {},
        }

        Ok(())
    }

    async fn update_client<'a>(&'a mut self, peer: &RakNetPeer, update: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        Ok(())
    }
}

impl RealmServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<RealmServer> {
        let mut listener = RakNetListener::bind(Box::new(RealmServerMessageHandler {}), addr).await?;

        /*let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);*/

        Ok(Self {listener})
    }
}
