use std::{sync::Arc, fs, net::Ipv4Addr};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, CPkt, CPktStream_126_5, oaCharacter, CPktStream_126_1, oaPktResponseSelectWorld, oaPktCharacterSelectSuccess}};

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
                println!("Realm pkt: \n{:#?}", pkt);

                match &pkt {
                    CPkt::oaPktRequestCharacterList(pkt) => {
                        let mut character_list = CPktStream_126_1::default();
                        response.add_message(Reliability::Reliable, character_list.to_message())
                    },
                    CPkt::oaPktCharacterCreate(pkt) => {
                        println!("Character create: {}", pkt.character_name);

                        let mut character_create_successful = CPktStream_126_5::default();
                        character_create_successful.character = oaCharacter {
                            field_0: 1,
                            field_1: "CharName".to_owned(),
                            field_2: 2,
                            field_3: 0,
                        };
                        response.add_message(Reliability::Reliable, character_create_successful.to_message())
                    },
                    CPkt::oaPktRequestSelectWorld(pkt) => {
                        let mut response_select_world = oaPktResponseSelectWorld::default();
                        response_select_world.field_1 = true;
                        response_select_world.field_2 = 0;
                        response_select_world.field_3 = pkt.field_3.clone();
                        response.add_message(Reliability::Reliable, response_select_world.to_message())
                    },
                    CPkt::oaPktCharacterSelect(pkt) => {
                        let mut character_select_success = oaPktCharacterSelectSuccess::default();

                        character_select_success.world_ip = u32::from_be(Ipv4Addr::new(172, 22, 96, 1).into());
                        character_select_success.world_port = 6114;
                        character_select_success.magic_bytes = Vec::new();
                        character_select_success.magic_bytes.resize(10, 0);

                        response.add_message(Reliability::Reliable, character_select_success.to_message())
                    }
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
