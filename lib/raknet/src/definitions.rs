// Copyright (C) 2024 AnotherlandServer
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

use crate::error::{RakNetError, Result};

pub const MAX_MTU_SIZE: usize = 1024;
pub const RECV_BUFFER_SIZE: usize = 2048;

#[derive(Debug, Clone, Copy)]
pub enum PacketID {
    InternalPing,
    PingOpenConnections,
    Ping,
    ConnectedPong,
    ConnectionRequest,
    SecuredConnectionResponse,
    SecuredConnectionConfirmation,
    OpenConnectionRequest,
    OpenConnectionReply,
    ConnectionRequestAccepted,
    ConnectionAttemptFailed,
    AlreadyConnected,
    NewIncomingConnection,
    NoFreeIncomingConnections,
    DisconnectionNotification,
    ConnectionLost,
    RsaPublicKeyMismatch,
    InvalidPassword,
    ModifiedPacket,
    Pong,
    ConnectionBanned,
    User(u8),
}

impl PacketID {
    pub fn from(val: u8) -> Self {
        match val {
            1 => PacketID::InternalPing,
            2 => PacketID::PingOpenConnections,
            3 => PacketID::Ping,
            4 => PacketID::ConnectedPong,
            5 => PacketID::ConnectionRequest,
            6 => PacketID::SecuredConnectionResponse,
            7 => PacketID::SecuredConnectionConfirmation,
            9 => PacketID::OpenConnectionRequest,
            10 => PacketID::OpenConnectionReply,
            11 => PacketID::ConnectionRequestAccepted,
            12 => PacketID::ConnectionAttemptFailed,
            13 => PacketID::AlreadyConnected,
            14 => PacketID::NewIncomingConnection,
            15 => PacketID::NoFreeIncomingConnections,
            16 => PacketID::DisconnectionNotification,
            17 => PacketID::ConnectionLost,
            18 => PacketID::RsaPublicKeyMismatch,
            19 => PacketID::InvalidPassword,
            20 => PacketID::ModifiedPacket,
            21 => PacketID::Pong,
            23 => PacketID::ConnectionBanned,
            _ => PacketID::User(val),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            PacketID::InternalPing => 1,
            PacketID::PingOpenConnections => 2,
            PacketID::Ping => 3,
            PacketID::ConnectedPong => 4,
            PacketID::ConnectionRequest => 5,
            PacketID::SecuredConnectionResponse => 6,
            PacketID::SecuredConnectionConfirmation => 7,
            PacketID::OpenConnectionRequest => 9,
            PacketID::OpenConnectionReply => 10,
            PacketID::ConnectionRequestAccepted => 11,
            PacketID::ConnectionAttemptFailed => 12,
            PacketID::AlreadyConnected => 13,
            PacketID::NewIncomingConnection => 14,
            PacketID::NoFreeIncomingConnections => 15,
            PacketID::DisconnectionNotification => 16,
            PacketID::ConnectionLost => 17,
            PacketID::RsaPublicKeyMismatch => 18,
            PacketID::InvalidPassword => 19,
            PacketID::ModifiedPacket => 20,
            PacketID::Pong => 21,
            PacketID::ConnectionBanned => 23,
            PacketID::User(id) => id,
        }
    }
}

// Message IDs named in a way similar to the RakNet sources for easier comparison
// All these IDs differ from stock RakNet and are unique to Otherland
/*pub const ID_INTERNAL_PING: u8 = 1;
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
pub const ID_RSA_PUBLIC_KEY_MISMATCH: u8 = 18;
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
const ID_USER_MESSAGE_START: u8 = 100;*/