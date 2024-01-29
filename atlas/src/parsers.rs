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

use std::io;

use bitstream_io::{ByteWriter, ByteWrite, Endianness};
use nom::{IResult, error::{VerboseError, context}, multi::length_count, number::complete::{le_u16, le_u8}, combinator::map};

pub fn parse_pkt_cstring(data: &[u8]) -> IResult<&[u8], String, VerboseError<&[u8]>> {
    context("cstring", 
        map(
            length_count(le_u16, le_u8), 
            |data| String::from_utf8_lossy(&data).to_string()
        )
    )(data)
}

pub fn parse_pkt_wstring(data: &[u8]) -> IResult<&[u8], String, VerboseError<&[u8]>> {
    context("wstring",
        map(
            length_count(le_u16, le_u16), 
            |data| String::from_utf16_lossy(&data).to_string()
        )
    )(data)
}

pub fn write_pkt_cstring<W,E>(writer: &mut ByteWriter<W, E>, value: Option<&str>, maxlen: Option<usize>) -> io::Result<()>
where 
    W: io::Write, 
    E: Endianness, {
    if let Some(value) = value {
        let value = if let Some(maxlen) = maxlen {
            &value[..maxlen]
        } else {
            value
        };

        writer.write(value.len() as u16)?;
        writer.write_bytes(value.as_bytes())?;
    } else {
        writer.write(0u16)?;
    }

    Ok(())
} 

pub fn write_pkt_wstring<W,E>(writer: &mut ByteWriter<W, E>, value: Option<&str>, maxlen: Option<usize>) -> io::Result<()>
where 
    W: io::Write, 
    E: Endianness, {
    if let Some(value) = value {
        let value = if let Some(maxlen) = maxlen {
            &value[..maxlen]
        } else {
            value
        };

        writer.write(value.len() as u16)?;
        for v in value.encode_utf16() {
            writer.write(v)?;
        }
    } else {
        writer.write(0u16)?;
    }

    Ok(())
} 