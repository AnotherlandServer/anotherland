use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

use bitstream_io::{Numeric, ByteWriter, LittleEndian, ByteWrite};
use nom::{IResult, bytes::complete::{self, take}, combinator::{map, flat_map, map_parser, fail, value, success}, branch::alt, number::complete::*, error::{VerboseError, context}, sequence::tuple, multi::count};
use tokio::io::AsyncWriteExt;

/*
Known message IDs
1  - ID_INTERNAL_PING
2  - ID_PING_OPEN_CONNECTIONS (!?)
3  - ID_PING (!?)
4  - ID_CONNECTED_PONG
5  - ID_CONNECTION_REQUEST (!)
6  - ID_SECURED_CONNECTION_RESPONSE
7  - ID_SECURED_CONNECTION_CONFIRMATION
9  - ID_OPEN_CONNECTION_REQUEST
10 - ID_OPEN_CONNECTION_REPLY (???) (!?)
11 - ID_CONNECTION_REQUEST_ACCEPTED (?)
12 - ID_CONNECTION_ATTEMPT_FAILED 
13 - ID_ALREADY_CONNECTED (!?)
14 - ID_NEW_INCOMING_CONNECTION
15 - ID_NO_FREE_INCOMING_CONNECTIONS
16 - ID_DISCONNECTION_NOTIFICATION
17 - ID_CONNECTION_LOST
19 - ID_INVALID_PASSWORD
20 - ID_MODIFIED_PACKET
121 - ID_RSA_PUBLIC_KEY_MISMATCH
23 - ID_CONNECTION_BANNED (!?)
 */

#[derive(Debug)]
pub enum RakNetVersion {
    Invalid,
    Version3
}

#[derive(Debug)]
pub enum Message {
    OpenConnectionRequest{version: RakNetVersion},
    OpenConnectionReply,
    ConnectionRequest,
    ConnectionRequestAccepted{index: u16, peer_addr: SocketAddr, own_addr: SocketAddr, guid: [u32; 4]},
    SecuredConnectionResponse{hash: [u8; 20], e: u32, modulus: [u8; 64]},
    UserData{data: Vec<u8>}
}

#[derive(Debug)]
pub enum ParseError {
    Unknown,
    UnexpectedEndOfData,
    InvalidData,
    InternalError,
}

impl Message {
    pub fn parse<'a>(b: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&[u8]>> {
        context("message_type", flat_map(le_u8, |msg_id| {
            match msg_id {
                /* ID_CONNECTION_REQUEST */ 
                5 => |i| Ok((i, Message::ConnectionRequest)),
                /* ID_SECURED_CONNECTION_RESPONSE */ 
                6 => |i| context("secured_connection_response", map(tuple((
                    take(20usize),
                    le_u32,
                    take(64usize),
                )),
                |(hash, exponent, modulus): (&[u8],_,&[u8])| 
                    Message::SecuredConnectionResponse { 
                        hash: hash.try_into().unwrap(), 
                        e: exponent, modulus: 
                        modulus.try_into().unwrap() 
                    }))(i),
                /* ID_OPEN_CONNECTION_REQUEST */ 
                9 => |i| {
                    let (i, version) = context("open_connection_request", map(
                            le_u8,
                            |r| match r {
                                3 => RakNetVersion::Version3,
                                _ => RakNetVersion::Invalid,
                            }))(i)?;
                    Ok((i, Message::OpenConnectionRequest{version}))
                },
                /*ID_OPEN_CONNECTION_REPLY*/ 
                10 => |i| Ok((i, Message::OpenConnectionReply)),
                /*ID_CONNECTION_REQUEST_ACCEPTED*/ 
                11 => |i| context("connection_request_accepted", map(tuple((
                    tuple((le_u32, le_u16)),
                    le_u16,
                    tuple((le_u32, le_u16)),
                    count(le_u32, 4),
                )),
                |(peer_addr, index, own_addr, guid)| 
                    Message::ConnectionRequestAccepted { 
                        index: index, 
                        peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(peer_addr.0), peer_addr.1)), 
                        own_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(own_addr.0), own_addr.1)),
                        guid: guid.as_slice().try_into().unwrap()
                    }))(i),
                _ => fail,
            }
        }))(b)
    }

    pub fn from_bytes(b: &[u8]) -> Result<Message, ParseError> {
        match Message::parse(b) {
            Ok((_, message)) => Ok(message),
            Err(e) => match e {
                nom::Err::Incomplete(_) => Err(ParseError::UnexpectedEndOfData),
                nom::Err::Error(_) => Err(ParseError::InvalidData),
                nom::Err::Failure(_) => Err(ParseError::InternalError),
            },
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        match self {
            Self::OpenConnectionRequest { version } => {
                writer.write(9u8);
                writer.write(3u8);
            },
            Self::OpenConnectionReply => { 
                writer.write(10u8); 
                writer.write(0u8); // Padding to avoid some packet filters
            },
            Self::ConnectionRequest => { 
                writer.write(5u8); 
                writer.write(0u8); // Padding to avoid some packet filters
            },
            Self::SecuredConnectionResponse { hash, e, modulus } => {
                writer.write(6u8);
                writer.write_bytes(hash);
                writer.write(*e);
                writer.write_bytes(modulus);
            },
            Self::ConnectionRequestAccepted { index, peer_addr, own_addr, guid } => {
                // MsgID
                writer.write(11u8);

                // Peer Address
                match peer_addr.ip() {
                    std::net::IpAddr::V4(adr) => {
                        writer.write::<u32>(adr.into());
                        writer.write(peer_addr.port());
                    },
                    std::net::IpAddr::V6(_) => panic!("IPv6 not supported!"),
                }

                // System index
                writer.write(*index);

                // Own Address
                match own_addr.ip() {
                    std::net::IpAddr::V4(adr) => {
                        writer.write::<u32>(adr.into());
                        writer.write(own_addr.port());
                    },
                    std::net::IpAddr::V6(_) => panic!("IPv6 not supported!"),
                }

                guid.map(|g| { writer.write(g) });
            },
            _ => panic!("Message ID not implemented in serializer!"),
        }
        
        buf
    }
}

