use std::{sync::Arc, fs};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self}};

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
                match &pkt {
                    _ => println!("Realm pkt: \n{:#?}", pkt),
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

impl RealmServerMessageHandler {
    fn parse_utf8_string<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a[u8]>> {
        map(length_data(nom::number::complete::le_u16), |b| String::from_utf8_lossy(b).to_string())(data)
    }

    fn parse_utf16_string<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a[u8]>> {
        map(length_data(map(nom::number::complete::le_u16, |i| i*2)), |b: &[u8]| {
            let (front, slice, back) = unsafe {
                b.align_to::<u16>()
            };
            if front.is_empty() && back.is_empty() {
                String::from_utf16_lossy(slice).to_string()
            } else {
                String::new()
            }
        })(data)
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
