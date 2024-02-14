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

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use serde::Serialize;

use super::RakNetErrorKind;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PeerAddress {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl PeerAddress {
    pub fn new(ip: &Ipv4Addr, port: u16) -> PeerAddress {
        PeerAddress { ip: *ip, port }
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(self.ip, self.port))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        writer.write::<u32>(self.ip.into()).expect("Failed to write ip");
        writer.write(self.port).expect("Failed to write port");

        buf
    }
}

impl TryFrom<SocketAddr> for PeerAddress {
    type Error = super::RakNetError;

    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        match value {
            SocketAddr::V4(addr) => {
                Ok(PeerAddress { ip: addr.ip().to_owned(), port: addr.port() })
            },
            _ => Err(Self::Error::from_kind(RakNetErrorKind::InvalidAddressFormat)),
        }
    }
}

impl Serialize for PeerAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        serializer.serialize_str(format!("{}:{}", self.ip, self.port).as_str())
    }
}
