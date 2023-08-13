use nom::{IResult, error::{VerboseError, context}, Parser, multi::length_count, number::complete::{le_u16, le_u8}, combinator::map};

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