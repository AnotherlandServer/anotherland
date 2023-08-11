use nom::{IResult, error::VerboseError, error::context, combinator::{map, flat_map}, multi::length_count, number::complete::{le_u16, le_u8, le_u64}, Parser, sequence::tuple};

use super::pkt_login::*;

#[derive(Debug)]

pub struct Pkt {
    pub unknownByte: u8,
    pub unknownQWord: u64,
    pub body: PktBody,
}

#[derive(Debug)]
pub enum PktBody {
    Login(Box<PktLogin>)
}

impl Pkt {
    pub fn from_bytes<'a>(data: &'a [u8]) -> IResult<&'a [u8], Pkt, VerboseError<&'a [u8]>> {
        context("pkt_header", flat_map(
            tuple((
                le_u8,
                le_u8,
                le_u64,
            )),
            |(pktType, unknownByte, unknownQWord)| {
                move |data: &'a [u8]| -> IResult<&'a [u8], Pkt, VerboseError<&'a [u8]>> {
                    let body = match pktType {
                        ID_ATLAS_PKT_LOGIN => PktLogin::from_bytes(data)?,
                        _ => panic!(),
                    };

                    Ok((body.0, Pkt {
                        unknownByte,
                        unknownQWord,
                        body: body.1,
                    }))
                }
            } 
        ))(data)
    }
}

pub fn pkt_parse_cstr<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a [u8]>> {
    context("c_str", 
        map(
            length_count(le_u16, le_u8), 
            |data| String::from_utf8_lossy(&data).to_string()
        )
    )(data)
}

pub fn pkt_parse_wstr<'a>(data: &'a [u8]) -> IResult<&'a [u8], String, VerboseError<&'a [u8]>> {
    context("w_str",
        map(
            length_count(le_u16, le_u16), 
            |data| String::from_utf16_lossy(&data).to_string()
        )
    )(data)
}
