// Copyright (C) 2023 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{net::Ipv4Addr, io};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use std::time::Duration;
use nom::{number::complete::{le_u8, le_u32, le_u16}, combinator::{flat_map, fail, map, rest_len, peek}, error::{context, VerboseError}, IResult, sequence::tuple, bytes::complete::{take, tag}, multi::{many0, count}};

use crate::{CPkt, Uuid};

use super::PeerAddress;

// Message IDs named in a way similar to the RakNet sources for easier comparison
// All these IDs differ from stock RakNet and are unique to Otherland
pub const ID_INTERNAL_PING: u8 = 1;
#[allow(unused)]
pub const ID_PING_OPEN_CONNECTIONS: u8 = 2;
#[allow(unused)]
pub const ID_PING: u8 = 3;
pub const ID_CONNECTED_PONG: u8 = 4;
pub const ID_CONNECTION_REQUEST: u8 = 5;
#[allow(unused)]
pub const ID_SECURED_CONNECTION_RESPONSE: u8 = 6;
#[allow(unused)]
pub const ID_SECURED_CONNECTION_CONFIRMATION: u8 = 7;
pub const ID_OPEN_CONNECTION_REQUEST: u8 = 9;
pub const ID_OPEN_CONNECTION_REPLY: u8 = 10;
#[allow(unused)]
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 11;
#[allow(unused)]
pub const ID_CONNECTION_ATTEMPT_FAILED: u8 = 12;
#[allow(unused)]
pub const ID_ALREADY_CONNECTED: u8 = 13;
pub const ID_NEW_INCOMING_CONNECTION: u8 = 14;
#[allow(unused)]
pub const ID_NO_FREE_INCOMING_CONNECTIONS: u8 = 15;
pub const ID_DISCONNECTION_NOTIFICATION: u8 = 16;
#[allow(unused)]
pub const ID_CONNECTION_LOST: u8 = 17;
#[allow(unused)]
pub const ID_INVALID_PASSWORD: u8 = 19;
#[allow(unused)]
pub const ID_MODIFIED_PACKET: u8 = 20;
#[allow(unused)]
pub const ID_PONG: u8 = 21;
#[allow(unused)]
pub const ID_CONNECTION_BANNED: u8 = 23;

//const ID_RSA_PUBLIC_KEY_MISMATCH: u8 = 121;
#[allow(unused)]
const ID_USER_MESSAGE_START: u8 = 100;

#[allow(unused)]
#[derive(Debug)]
pub enum Message {
    InternalPing{time: Duration},
    PingOpenConnections,
    Ping,
    ConnectedPong{remote_time: Duration, local_time: Duration},
    ConnectionRequest{password: String},
    SecuredConnectionResponse{hash: [u8; 20], e: u32, modulus: [u8; 64]},
    SecuredConnectionConfirmation,
    OpenConnectionRequest{version: u8},
    OpenConnectionReply,
    ConnectionRequestAccepted{index: u16, peer_addr: PeerAddress, own_addr: PeerAddress, guid: Uuid},
    ConnectionAttemptFailed,
    AlreadyConnected,
    NewIncomingConnection{primary_address: PeerAddress, secondary_addresses: Vec<PeerAddress>},
    NoFreeIncomingConnections,
    DisconnectionNotification,
    ConnectionList,
    InvalidPassword,
    ModifiedPacket,
    ConnectionBanned,
    RSAPublicKeyMismatch,
    //ReceivedStaticData{data: Vec<u8>},
    AtlasPkt(CPkt),
    User{number: u8, data: Vec<u8>},
}

impl Message {
    pub fn test_offline_message<'a>(data: &'a [u8]) -> bool {
        data[0] == ID_OPEN_CONNECTION_REQUEST || data[0] == ID_OPEN_CONNECTION_REPLY
    }
}

// Parsing
#[allow(unused)]
impl Message {
    fn parse_peer_address<'a>(data: &'a [u8]) -> IResult<&'a [u8], PeerAddress, VerboseError<&'a[u8]>> {
        map(tuple((le_u32, le_u16)), |(address, port)| PeerAddress::new(&Ipv4Addr::from(address), port))(data)
    }

    fn parse_internal_ping<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("internal_ping", map(
            tuple((
                tag([ID_INTERNAL_PING]),
                le_u32,
            )),
            |(_, timestamp)| {
                Message::InternalPing { time: Duration::from_millis(timestamp.into()) }
            }))(data)
    }

    fn parse_connected_pong<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("connected_pong", map(
            tuple((
                tag([ID_CONNECTED_PONG]),
                le_u32,
                le_u32,
            )),
            |(_, remote_time, local_time)| {
                Message::ConnectedPong { remote_time: Duration::from_millis(remote_time.into()), local_time: Duration::from_millis(local_time.into()) }
            }))(data)
    }

    fn parse_new_incoming_connection<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("new_incoming_connection", map(
            tuple((
                tag([ID_NEW_INCOMING_CONNECTION]),
                Self::parse_peer_address,
                many0(Self::parse_peer_address),
            )),
            |(_, primary_address, secondary_addresses)| {
                Message::NewIncomingConnection { primary_address, secondary_addresses }
            }))(data)
    }

    fn parse_disconnection_notification<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("disconnection_notification", map(
            tag([ID_DISCONNECTION_NOTIFICATION]),
            |_| {
                Message::DisconnectionNotification { }
            }))(data)
    }

    fn parse_connection_request<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("conenction_request", map(
            tuple((
                tag([ID_CONNECTION_REQUEST]),
                many0(le_u8),
            )),
            |(_, password)| {
                Message::ConnectionRequest{
                    password: String::from_utf8_lossy(password.as_slice()).into()
                }
            }))(data)
    }

    fn parse_secured_connection_response<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("secured_connection_response", map(tuple((
            tag([ID_SECURED_CONNECTION_RESPONSE]),
            take(20usize),
            le_u32,
            take(64usize),
        )),
        |(_, hash, exponent, modulus): (_, &[u8],_,&[u8])| 
            Message::SecuredConnectionResponse { 
                hash: hash.try_into().unwrap(), 
                e: exponent, modulus: 
                modulus.try_into().unwrap() 
            }))(data)
    }

    fn parse_open_connection_request<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("open_connection_request", 
            map(tuple((tag([ID_OPEN_CONNECTION_REQUEST]), le_u8)), |(_, version)| {
                Message::OpenConnectionRequest{version}
            }))(data)
    }

    fn parse_open_connection_reply<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("open_connection_reply", 
            map(tag([ID_OPEN_CONNECTION_REQUEST]), |_| Message::OpenConnectionReply))(data)
    }

    fn parse_connection_request_accepted<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("connection_request_accepted", 
            map(tuple((
                tag([ID_CONNECTION_REQUEST_ACCEPTED]),
                tuple((le_u32, le_u16)),
                le_u16,
                tuple((le_u32, le_u16)),
                count(le_u8, 16),
            )),
            |(_, peer_addr, index, own_addr, guid)| 
                Message::ConnectionRequestAccepted { 
                    index: index, 
                    peer_addr: PeerAddress::new(&Ipv4Addr::from(peer_addr.0), peer_addr.1), 
                    own_addr: PeerAddress::new(&Ipv4Addr::from(own_addr.0), own_addr.1),
                    guid: Uuid::from_bytes(&guid).unwrap().1,
                }))(data)
    }

    fn parse_atlas_pkt<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        /*let (_, (id, sub_id)) = tuple((le_u8, le_u8))(data)?;
        fs::write(format!("dump/{:#02x}_{:#02x}.bin", id, sub_id), data);*/

        context("atlas_pkt", map(CPkt::from_bytes, |pkt| Message::AtlasPkt(pkt)))(data)
    }

    /*fn parse_received_static_data<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("static_data", map(tuple((
            tag::<[u8;1],&'a [u8],_>([ID_RECEIVED_STATIC_DATA]),
            flat_map(rest_len, take)
        )), |(_, data)| Message::ReceivedStaticData { data: data.to_vec() }))(data)
    }*/

    fn parse_user_message<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("user_message", map(tuple((
            le_u8::<&[u8], _>, 
            flat_map(rest_len, take)
        )), |(number, data)| Message::User { number, data: data.to_vec() }))(data)
    }

    fn parse_unknown_message<'a>(data: &'a [u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("unknown_message", fail)(data)
    }

    pub fn from_bytes<'a>(data: &'a[u8]) -> IResult<&'a [u8], Message, VerboseError<&'a[u8]>> {
        context("message_type",
            flat_map(peek(le_u8),
            |msg_id| {
                match msg_id {
                    ID_INTERNAL_PING => Self::parse_internal_ping,
                    ID_CONNECTED_PONG => Self::parse_connected_pong,
                    ID_CONNECTION_REQUEST => Self::parse_connection_request,
                    ID_SECURED_CONNECTION_RESPONSE => Self::parse_secured_connection_response,
                    ID_OPEN_CONNECTION_REQUEST => Self::parse_open_connection_request,
                    ID_OPEN_CONNECTION_REPLY => Self::parse_open_connection_reply,
                    ID_CONNECTION_REQUEST_ACCEPTED => Self::parse_connection_request_accepted,
                    ID_NEW_INCOMING_CONNECTION => Self::parse_new_incoming_connection,
                    ID_DISCONNECTION_NOTIFICATION => Self::parse_disconnection_notification,
                    _ => Self::parse_atlas_pkt,
                }
            }))(data)
    }
}

// Writing
impl Message {
    fn write(&self, writer: &mut ByteWriter<&mut Vec<u8>, LittleEndian>) -> io::Result<()> {
        match self {
            Self::InternalPing { time } => {
                writer.write(ID_INTERNAL_PING)?;
                writer.write(time.as_millis() as u32)?;
            }
            Self::ConnectedPong { remote_time, local_time } => {
                writer.write(ID_CONNECTED_PONG)?;
                writer.write(remote_time.as_millis() as u32)?;
                writer.write(local_time.as_millis() as u32)?;
            },
            Self::ConnectionRequest { password } => {
                writer.write(ID_CONNECTION_REQUEST)?;
                writer.write_bytes(password.as_bytes())?;
            },
            Self::SecuredConnectionResponse { hash, e, modulus } => {
                writer.write(ID_SECURED_CONNECTION_RESPONSE)?;
                writer.write_bytes(hash)?;
                writer.write(*e)?;
                writer.write_bytes(modulus)?;
            },
            Self::OpenConnectionRequest { version } => {
                writer.write(ID_OPEN_CONNECTION_REQUEST)?;
                writer.write(*version)?;
            },
            Self::OpenConnectionReply => {
                writer.write(ID_OPEN_CONNECTION_REPLY)?;
            },
            Self::ConnectionRequestAccepted { index, peer_addr, own_addr, guid } => {
                writer.write(ID_CONNECTION_REQUEST_ACCEPTED)?;
                writer.write_bytes(peer_addr.to_bytes().as_slice())?;
                writer.write(*index)?;
                writer.write_bytes(own_addr.to_bytes().as_slice())?;
                writer.write_bytes(guid.to_bytes().as_slice())?;
            },
            Self::DisconnectionNotification => {
                writer.write(ID_DISCONNECTION_NOTIFICATION)?;
            }
            Self::AtlasPkt(pkt) => {
                let (id, subid) = pkt.get_id();
                writer.write(id)?;
                writer.write(subid)?;
                writer.write_bytes(pkt.to_bytes().as_slice())?;
            }
            Self::User { number, data } => {
                writer.write(*number)?;
                writer.write_bytes(data)?;
            }
            _ => panic!("Packet writer unimplemented!"),
        }

        Ok(())
    } 

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        self.write(&mut writer).expect("Message serialization failed");

        buf
    }
}
