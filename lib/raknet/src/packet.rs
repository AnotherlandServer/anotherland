// Copyright (C) 2025 AnotherlandServer
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

use std::net::SocketAddr;

use rsa::{traits::PublicKeyParts, RsaPublicKey};
use uuid::Uuid;

use crate::{buffer::{RakNetReader, RakNetWriter}, error::Result, util::BinaryAddress, PacketID};

#[derive(Debug)]
pub struct ConnectionRequest {
    pub version: u8
}

pub fn read_open_connection_request(buf: &[u8]) -> Result<ConnectionRequest> {
    let mut buf = RakNetReader::new(buf);

    Ok(ConnectionRequest { version: buf.read_u8()? })
}

pub fn write_secured_connection_response(writer: &mut RakNetWriter, cookie: &[u8], key: RsaPublicKey) -> Result<()> {
    writer.write_u8(PacketID::SecuredConnectionResponse.to_u8());
    writer.write(cookie);
    writer.write_u32(key.e().get_limb(0) as u32);
    writer.write(&key.n().to_bytes_le());

    Ok(())
}

pub fn write_connection_request_accepted(writer: &mut RakNetWriter, peer_addr: SocketAddr, local_addr: SocketAddr, guid: Uuid) -> Result<()> {
    writer.write_u8(PacketID::ConnectionRequestAccepted.to_u8());
    writer.write(&peer_addr.ip().to_bytes());
    writer.write_u16(peer_addr.port());
    writer.write_u16(0);
    writer.write(&local_addr.ip().to_bytes());
    writer.write_u16(local_addr.port());
    writer.write(guid.as_bytes());

    Ok(())
}