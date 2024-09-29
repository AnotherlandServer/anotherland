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

use crate::{buffer::RakNetReader, error::{RakNetError, Result}, PacketID};

#[derive(Debug)]
pub struct ConnectionRequest {
    pub version: u8
}

pub struct InternalPing {
    pub duration: u32,
}

fn verify_packet_id(buf: &mut RakNetReader, id: u8) -> Result<()> {
    if buf.read_u8()? != id {
        Err(RakNetError::ParserMismatchError)
    } else {
        Ok(())
    }
}

pub fn read_open_connection_request(buf: &[u8]) -> Result<ConnectionRequest> {
    let mut buf = RakNetReader::new(buf);

    Ok(ConnectionRequest { version: buf.read_u8()? })
}

pub fn read_connection_request(buf: &[u8]) -> Result<ConnectionRequest> {
    let mut buf = RakNetReader::new(buf);
    
    todo!()
}