use std::{net::{Ipv4Addr}, io};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use nom::{number::complete::{le_u8, le_u32, le_u16}, combinator::{flat_map, fail, map, rest_len, peek}, error::{context, VerboseError}, IResult, sequence::tuple, bytes::complete::{take, tag}, multi::{many0, count}};

use super::{Guid, PeerAddress};

// Message IDs named in a way similar to the RakNet sources for easier comparison
// All these IDs differ from stock RakNet and are unique to Otherland
const ID_INTERNAL_PING: u8 = 1;
const ID_PING_OPEN_CONNECTIONS: u8 = 2;
const ID_PING: u8 = 3;
const ID_CONNECTED_PONG: u8 = 4;
const ID_CONNECTION_REQUEST: u8 = 5;
const ID_SECURED_CONNECTION_RESPONSE: u8 = 6;
const ID_SECURED_CONNECTION_CONFIRMATION: u8 = 7;
const ID_OPEN_CONNECTION_REQUEST: u8 = 9;
const ID_OPEN_CONNECTION_REPLY: u8 = 10;
const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 11;
const ID_CONNECTION_ATTEMPT_FAILED: u8 = 12;
const ID_ALREADY_CONNECTED: u8 = 13;
const ID_NEW_INCOMING_CONNECTION: u8 = 14;
const ID_NO_FREE_INCOMING_CONNECTIONS: u8 = 15;
const ID_DISCONNECTION_NOTIFICATION: u8 = 16;
const ID_CONNECTION_LOST: u8 = 17;
const ID_INVALID_PASSWORD: u8 = 19;
const ID_MODIFIED_PACKET: u8 = 20;
const ID_CONNECTION_BANNED: u8 = 23;
//const ID_RSA_PUBLIC_KEY_MISMATCH: u8 = 121;
const ID_USER_MESSAGE_START: u8 = 100;

#[derive(Debug)]
pub enum Message {
    InternalPing,
    PingOpenConnections,
    Ping,
    ConnectedPong,
    ConnectionRequest{password: String},
    SecuredConnectionResponse{hash: [u8; 20], e: u32, modulus: [u8; 64]},
    SecuredConnectionConfirmation,
    OpenConnectionRequest{version: u8},
    OpenConnectionReply,
    ConnectionRequestAccepted{index: u16, peer_addr: PeerAddress, own_addr: PeerAddress, guid: Guid},
    ConnectionAttemptFailed,
    AlreadyConnected,
    NewIncomingConnection,
    NoFreeIncomingConnections,
    DisconnectionNotification,
    ConnectionList,
    InvalidPassword,
    ModifiedPacket,
    ConnectionBanned,
    RSAPublicKeyMismatch,
    User{number: u8, data: Vec<u8>},
}

impl Message {
    pub fn test_offline_message<'a>(data: &'a [u8]) -> bool {
        data[0] == ID_OPEN_CONNECTION_REQUEST || data[0] == ID_OPEN_CONNECTION_REPLY
    }
}

// Parsing
impl Message {
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
                count(le_u32, 4),
            )),
            |(_, peer_addr, index, own_addr, guid)| 
                Message::ConnectionRequestAccepted { 
                    index: index, 
                    peer_addr: PeerAddress::new(&Ipv4Addr::from(peer_addr.0), peer_addr.1), 
                    own_addr: PeerAddress::new(&Ipv4Addr::from(own_addr.0), own_addr.1),
                    guid: Guid { g: guid.as_slice().try_into().unwrap() }
                }))(data)
    }

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
                    ID_CONNECTION_REQUEST => Self::parse_connection_request,
                    ID_SECURED_CONNECTION_RESPONSE => Self::parse_secured_connection_response,
                    ID_OPEN_CONNECTION_REQUEST => Self::parse_open_connection_request,
                    ID_OPEN_CONNECTION_REPLY => Self::parse_open_connection_reply,
                    ID_CONNECTION_REQUEST_ACCEPTED => Self::parse_connection_request_accepted,
                    ID_USER_MESSAGE_START..=u8::MAX => Self::parse_user_message,
                    _ => Self::parse_unknown_message,
                }
            }))(data)
    }
}

// Writing
impl Message {
    fn write(&self, writer: &mut ByteWriter<&mut Vec<u8>, LittleEndian>) -> io::Result<()> {
        match self {
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
