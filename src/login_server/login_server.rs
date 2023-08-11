use std::{sync::Arc, fs};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, error::{VerboseError, convert_error}, sequence::tuple, combinator::map, multi::{length_data, count}, bytes::complete::take};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::{raknet::{RakNetListener, RequestHandler, RakNetRequest, RakNetResponse, Message, RakNetPeer, Packet, Reliability}, atlas::{self, PktLogin, PktBody}};

pub struct LoginServer {
    listener: RakNetListener,
}

struct LoginServerMessageHandler {

}

#[async_trait]
impl RequestHandler for LoginServerMessageHandler {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>> {
        match request.message() {
            Message::AtlasPkt(pkt) => {
                match &pkt.body {
                    PktBody::Login(login_pkt) => {
                        println!("{:#?}", pkt);

                        let mut buf = Vec::new();
                        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

                        let login_str = "Hey, it works!";

                        // Header
                        writer.write(0u8);
                        writer.write(0u64);

                        // flag
                        writer.write(1u8);
                        writer.write(0u32);

                        writer.write(0u8);
                        writer.write(0u32);
                        writer.write(0u32);
                        writer.write(0u16);
                        writer.write(0u32);
                        writer.write(login_str.len() as u16);
                        writer.write_bytes(login_str.as_bytes());
                        writer.write_bytes(login_pkt.magic.as_slice());
                        writer.write(0u32);
                        writer.write(login_pkt.username.len() as u16);
                        writer.write_bytes(login_pkt.username.as_bytes());
                        writer.write(0u32);
                        writer.write(0u16);
                        writer.write(0u16);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);
                        writer.write(0u8);

                        /*writer.write(ID_CONNECTION_REQUEST_ACCEPTED)?;
                        writer.write_bytes(peer_addr.to_bytes().as_slice())?;
                        writer.write(*index)?;
                        writer.write_bytes(own_addr.to_bytes().as_slice())?;
                        writer.write_bytes(guid.to_bytes().as_slice())?;*/

                        response.add_message(Reliability::Reliable, Message::User { number: 34, data: buf });
                    },
                    _ => (),
                }
            }
            /*Message::ReceivedStaticData { data } => {
                let result = PktLogin::from_bytes(data);

                if let Ok((_, pkt)) = result {
                    println!("{:#?}", pkt);
                } else if let Err(e) = result {
                    println!(
                        "verbose errors:\n{}",
                        convert_error(data.as_slice(), e)
                      );
                }
                /*fs::write("logindata.bin", data);

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
                }*/

            },*/
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
