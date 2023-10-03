use std::io;

use bitstream_io::{ByteWriter, ByteWrite, Endianness};
use nom::{IResult, error::{VerboseError, context}, multi::length_count, number::complete::{le_u16, le_u8}, combinator::map};

pub fn parse_pkt_cstring<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a [u8]>> {
    context("cstring", 
        map(
            length_count(le_u16, le_u8), 
            |data| String::from_utf8_lossy(&data).to_string()
        )
    )(data)
}

pub fn parse_pkt_wstring<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a [u8]>> {
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
            &value
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
            &value
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