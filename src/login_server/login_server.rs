use std::{sync::Arc, fs, net::Ipv4Addr};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, CPkt, CPktLoginResult, oaPktLoginQueueUpdate, oaPktRealmStatusList, CpktLoginResultUiState, Uuid}};

pub struct LoginServer {
    listener: RakNetListener,
}

struct LoginServerMessageHandler {
    temp_field_count: u32,
}

#[async_trait]
impl RequestHandler for LoginServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::AtlasPkt(pkt) => {
                println!("{:#?}", pkt);

                match &pkt {
                    CPkt::CPktLogin(login_pkt) => {
                        let mut result = CPktLoginResult::default();

                        result.login_success = true;
                        result.ui_state = CpktLoginResultUiState::CharacterSelection;
                        result.user_id = Some(1234);
                        result.username = Some(login_pkt.username.to_owned());
                        result.magic_bytes = Some(login_pkt.magic_bytes.clone());
                        result.field_0x4 = Some(true);
                        result.field29_0x24 = Some(123);
                        result.realm_ip = Some(u32::from_be(Ipv4Addr::new(172, 22, 96, 1).into()));
                        result.realm_port = Some(6113);
                        result.field38_0x34 = Some(0xBEEF);
                        result.unknown_string = Some("Test String".to_owned());
                        result.session_id = Some(Uuid::new_v4());

                        println!("{:#?}", login_pkt);
                        println!("{:#?}", result);
                        println!("{:#?}", result.to_bytes());

                        response.add_message(Reliability::Reliable, result.to_message());

                        let mut realm_status = oaPktRealmStatusList::default();
                        
                        realm_status.realm_count = 1;
                        realm_status.realm_id = 1;
                        realm_status.realm_name = "Test server".to_owned();
                        realm_status.channel_count = 2;
                        realm_status.field_5.push(1);
                        realm_status.field_5.push(2);
                        realm_status.channel_flag_count = 2;
                        realm_status.field_7.push(0);
                        realm_status.field_7.push(0);
    
                        
                        response.add_message(Reliability::Reliable, realm_status.to_message());
                    },
                    CPkt::oaPktRealmStatusList(pkt) => {
                        println!("{:?}", pkt);
                    }
                    _ => (),
                }
            }
            _ => {},
        }

        Ok(())
    }

    async fn update_client<'a>(&'a mut self, peer: &RakNetPeer, update: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        /*let mut queue_update = oaPktLoginQueueUpdate::default();
                        
        queue_update.channel = 1;
        queue_update.number = 10;
        queue_update.field38_0x2c = self.temp_field_count;

        update.add_message(Reliability::Reliable, queue_update.to_message());

        self.temp_field_count += 1;*/

        Ok(())
    }
}

impl LoginServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<LoginServer> {
        let mut listener = RakNetListener::bind(Box::new(LoginServerMessageHandler { temp_field_count: 0 }), addr).await?;

        /*let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);*/

        Ok(Self {listener})
    }
}
