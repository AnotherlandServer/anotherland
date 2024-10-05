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

pub const MAX_MTU_SIZE: usize = 1492;
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
        // All these IDs differ from stock RakNet and are unique to Otherland
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
        // All these IDs differ from stock RakNet and are unique to Otherland
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
