use nom::{number::complete::{le_u8, le_u32, le_u16, le_u64}, combinator::{flat_map, fail, map, rest_len, peek}, error::{context, VerboseError}, IResult, sequence::tuple, bytes::complete::{take, tag}, multi::{many0, count}};

use super::{pkt_parse_cstr, pkt_parse_wstr, PktBody};

#[derive(Debug)]
pub struct PktLogin {
    pub username: String,
    pub password: String,
    pub steamid: String,
    pub magic: [u8; 16],
    pub os_info: [String; 30],
}

impl PktLogin {
    pub fn from_bytes<'a>(data: &'a [u8]) -> IResult<&'a [u8], super::PktBody, VerboseError<&'a [u8]>> {
        context("pkt_login", map(
            tuple((
                pkt_parse_cstr,
                pkt_parse_cstr,
                pkt_parse_cstr,
                le_u32,
                le_u8,
                take(16usize),
                count(pkt_parse_wstr, 30usize),
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u16,
                le_u32,
                le_u64,
                le_u16,
                le_u32,
                pkt_parse_cstr,
                le_u32,
                tuple((
                    pkt_parse_cstr,
                    pkt_parse_cstr,
                    pkt_parse_cstr,
                )),
            )),
            |(username, password, steamid, _, _, magic, client_info, _, _, _, _, _, _, _, _, _, _, _, _)| PktBody::Login(
                Box::new(PktLogin {
                    username,
                    password,
                    steamid,
                    magic: magic.try_into().unwrap(),
                    os_info: client_info.try_into().unwrap(),
                })
            )
        ))(data)
    }
}