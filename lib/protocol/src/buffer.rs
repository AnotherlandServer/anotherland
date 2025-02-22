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

use std::ops::{Deref, DerefMut};

use bitstream_io::{ByteWrite, ByteWriter, LittleEndian};
use nom::{bytes::complete::take, combinator::flat_map, error::{context, VerboseError}, number::complete::le_u32, IResult};

#[derive(Default, Clone, Debug)]
pub struct OABuffer(Vec<u8>);

impl OABuffer {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
    
        writer.write(self.0.len() as u32).unwrap();
        writer.write_bytes(&self.0).unwrap();

        buf
    } 

    pub(crate) fn from_bytes(i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> { 
        let (i, data) = context("OABuffer",
            flat_map(le_u32, take))(i)?;
        
        Ok((i, OABuffer(data.to_vec())))
    }
}

impl Deref for OABuffer {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OABuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<OABuffer> for Vec<u8> {
    fn from(value: OABuffer) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for OABuffer {
    fn from(value: Vec<u8>) -> Self {
        OABuffer(value)
    }
}