use std::{sync::Arc};

use async_trait::async_trait;
use nom::{IResult, error::VerboseError, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer};

pub struct LoginServer {
    listener: RakNetListener,
}

struct LoginServerMessageHandler {

}

#[async_trait]
impl RequestHandler for LoginServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::ReceivedStaticData { data } => {
                if let Ok((_, (_, email, password, _, strings))) 
                    = tuple((
                        take(9usize), 
                        Self::parse_utf8_string, // username
                        Self::parse_utf8_string, // password
                        take(23usize), // ??
                        count(Self::parse_utf16_string, 30usize)
                    ))(data) {
                    println!("Username: {}", email);
                    println!("Password: {}", password);
                    println!("Strings: {:#?}", strings);
                } else {
                    panic!("Static data parse failed");
                }

            },
            _ => {},
        }

        Ok(())
    }
}

impl LoginServerMessageHandler {
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

impl LoginServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<LoginServer> {
        let mut listener = RakNetListener::bind(Box::new(LoginServerMessageHandler {}), addr).await?;

        /*let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);*/

        Ok(Self {listener})
    }
}
