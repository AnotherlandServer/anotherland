use std::{sync::Arc, fs};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, CPkt, CPktStream_126_5, oaCharacter, CPktStream_126_1, oaPktResponseSelectWorld, oaPktRequestEnterGame, oaPktLoginQueueUpdate}};

pub struct WorldServer {
    listener: RakNetListener,
}

struct WorldServerMessageHandler {
    temp_field_count: u32,
}

#[async_trait]
impl RequestHandler for WorldServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::AtlasPkt(pkt) => {
                println!("World pkt: \n{:#?}", pkt);

                match &pkt {
                    CPkt::oaPktRequestEnterGame(pkt) => {
                        let mut queue_update = oaPktLoginQueueUpdate::default();
                                        
                        queue_update.channel = 1;
                        queue_update.number = 1;
                        queue_update.field38_0x2c = self.temp_field_count;

                        response.add_message(Reliability::Reliable, queue_update.to_message());

                        self.temp_field_count += 1;
                    }
                    _ => (),
                }
            }
            _ => {},
        }

        Ok(())
    }

    async fn update_client<'a>(&'a mut self, peer: &RakNetPeer, update: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        let mut queue_update = oaPktLoginQueueUpdate::default();
                                        
        queue_update.channel = 1;
        queue_update.number = 1;
        queue_update.field38_0x2c = self.temp_field_count;

        update.add_message(Reliability::Reliable, queue_update.to_message());

        self.temp_field_count += 1;

        Ok(())
    }
}

impl WorldServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<WorldServer> {
        let mut listener = RakNetListener::bind(Box::new(WorldServerMessageHandler {
            temp_field_count: 0
        }), addr).await?;

        Ok(Self {listener})
    }
}
