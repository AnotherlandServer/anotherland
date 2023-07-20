use chrono::{DateTime, Utc, NaiveDateTime};
use nom::{IResult, bits::{complete::{self, take, bool, tag}}, bits, sequence::{self, tuple, pair}, combinator::{self, fail, map, value}, error::{FromExternalError, ParseError, Error}, Parser, multi::{many0_count, length_count, self}, branch::alt, Slice, InputIter, InputLength, ToUsize, bytes};
use rand::Rng;
use tokio::{signal, net::{UdpSocket, TcpSocket}, io, time::Instant};
use std::{fmt::{Write, Debug}, time::{SystemTime, UNIX_EPOCH}, num::ParseIntError, ops::{RangeFrom, AddAssign, Shl, Shr}};

mod raknet;


fn GetTimeUS() -> u32 {
    (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % u32::MAX as u128) as u32
}

fn GenerateGUID() -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut guid = Vec::new();
    guid.resize(4, 0);
    guid[0] = GetTimeUS();
    guid[1] = rng.gen();
    guid[2] = rng.gen();
    guid[3] = rng.gen();

    guid
}

#[derive(Debug)]
struct PacketSequence {
    channel: u8,
    index: u32,
}

#[derive(Debug)]
enum Reliability {
    Unreliable,
    UnreliableSequenced(PacketSequence),
    Reliable,
    ReliableOrdered(PacketSequence),
    ReliableSequenced(PacketSequence),
}

#[derive(Debug)]
enum PacketSplit {
    NotSplit,
    Split{ index: u32, count: u32},
}

struct Acks {

}

struct Message {
    acks: Vec<Acks>,
    remote_time: Option<DateTime<Utc>>,
    received_time: DateTime<Utc>,
    packet: Packet,
}

struct Packet {
    number: u32,
    reliability: Reliability,
    split: PacketSplit,
    data: Vec<u8>,
}

enum MessageParseError {
    InvalidValue
}

type BitInput<'a> = (&'a [u8], usize);

fn parse_compressed_bytes<'a>(
    count: usize, 
    unsigned: bool
) -> impl Fn((&'a [u8], usize)) -> IResult<(&'a [u8], usize), Vec<u8>> 
{
    move |mut i| {
        print!("Input: {:#?}", i);

        let mut result = Vec::with_capacity(count);
        result.resize(count, 0);

        let (byte_match, nibble_match) = if unsigned {
            (0u8, 0u8)
        } else {
            (0xFFu8, 0xF0u8)
        };

        // Read upper bytes
        let upper_bytes = count - 1;
        for b in 0..upper_bytes {
            let (i2, is_compressed) = bool(i)?;
            i = i2;

            if is_compressed {
                result[upper_bytes - b] = byte_match;
            } else {
                let (i, uncompressed): (_, Vec<u8>) = multi::count(take(8usize), count - b)(i)?;
                result[0..count-b].copy_from_slice(&uncompressed);

                print!("Output 1: {:#?}", result);
                return Ok((i, result));
            }
        }

        // Uncompress first byte, if all upper bytes where compressed (equal to byte_match)
        let (i, is_negative) = bool(i)?;
        if is_negative {
            let (i, first_byte): (_, u8) = take(4usize)(i)?;
            result[0] = nibble_match | first_byte;

            print!("Output 2: {:#?}", result);
            return Ok((i, result));
        } else {
            let (i, first_byte): (_, u8) = take(8usize)(i)?;
            result[0] = first_byte;

            print!("Output 3: {:#?}", result);
            return Ok((i, result));
        }
    }
}

/*fn parse_compressed_bytes<I, C, E: ParseError<(I, usize)>>(
    count: usize, 
    unsigned: bool
) -> impl Fn((I, usize)) -> IResult<(I, usize), Vec<u8>, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength + Clone,
  C: ToUsize,
{
    move |i: (I, usize)| {
        let (byte_match, nibble_match) = if unsigned {
            (0u8, 0u8)
        } else {
            (0xFFu8, 0xF0u8)
        };

        // 

        let upper_bytes = count - 1;
        let (i, bytes) = multi::many0(
            value(byte_match, tag(0b1, 1usize))
        )(i)?;

        Ok((i, Vec::new()))
    }
}*/

fn parse_acks(i: BitInput) -> IResult<BitInput, Vec<Acks>> {
    let (i, hasAcks) = complete::tag(0b0, 1usize)(i)?;
    Ok((i, Vec::new()))
}

fn parse_time(i: BitInput) -> IResult<BitInput, DateTime<Utc>> {
    let (i, timestamp): (BitInput, u32) = complete::take(32usize)(i)?;
    Ok((i, DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(), Utc)))
}

fn parse_remote_time(i: BitInput) -> IResult<BitInput, Option<DateTime<Utc>>> {
    let (i, has_remote_time) = bool(i)?;
    combinator::cond(has_remote_time, parse_time)(i)
}

fn parse_sequence(i: BitInput) -> IResult<BitInput, PacketSequence> {
    let (i, (channel, index)) : (_, (u8, u32)) = tuple((take(5usize), take(32usize)))(i)?;
    Ok((i, PacketSequence { channel, index }))
}

fn parse_reliability(i: BitInput) -> IResult<BitInput, Reliability> {
    let (i, reliability) = take(3usize)(i)?;
    match reliability {
        0 => Ok((i, Reliability::Unreliable)),
        1 => {
            let (i, seq) = parse_sequence(i)?;
            Ok((i, Reliability::UnreliableSequenced(seq)))
        },
        2 => Ok((i, Reliability::Reliable)),
        3 => {
            let (i, seq) = parse_sequence(i)?;
            Ok((i, Reliability::ReliableOrdered(seq)))
        },
        4 => {
            let (i, seq) = parse_sequence(i)?;
            Ok((i, Reliability::ReliableSequenced(seq)))
        }
        _ => fail(i)
    }
}

fn parse_packet_split(i: BitInput) -> IResult<BitInput, PacketSplit> {
    let (i, is_split) = bool(i)?;

    if is_split {
        let (i, (index, count)) : (_, (u32, u32)) = 
            map(
                tuple((
                    parse_compressed_bytes(4usize, true), 
                    parse_compressed_bytes(4usize, true)
                )),
                |r| (
                    u32::from_le_bytes(r.0[0..4].try_into().unwrap()), 
                    u32::from_le_bytes(r.1[0..4].try_into().unwrap())
                )
            )(i)?;
        Ok((i, PacketSplit::Split { index, count }))
    } else {
        Ok((i, PacketSplit::NotSplit))
    }
}

fn parse_msg(i: &[u8]) -> IResult<&[u8], Message> {
    //bits(parse_msg_internal)(i)
    // Read header
    let (i, (acks, remote_time, packet_number, reliability, split, data)) : 
        (_, (_, _, u32, _, _, _))
        = bits(tuple((
            parse_acks, 
            parse_remote_time,
            take(32usize),
            parse_reliability,
            parse_packet_split,
            length_count(
                map(
                    parse_compressed_bytes(2usize, true),
                    |r| (u16::from_le_bytes(r[0..2].try_into().unwrap()) + 7) >> 3
                ), 
                map(
                    bits::bytes::<_, _, nom::error::Error<&[u8]>, _, _>(bytes::complete::take(1usize)),
                    |r: &[u8]| r[0]
                )
            )
        )))(i)?;

    Ok((i, Message {
        acks,
        remote_time,
        received_time: Utc::now(),
        packet: Packet {
            number: packet_number,
            reliability,
            split,
            data
        }
    }))
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:6112").await?;

    let que_sock = TcpSocket::new_v4()?;
    que_sock.bind("0.0.0.0:53292".parse().unwrap())?;
    let que_listener = que_sock.listen(10)?;

    tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            let (len, addr) = sock.recv_from(&mut buf).await.unwrap();
    
            let mut dbg = String::new();
            for  i in 0..len {
                write!(&mut dbg, "{:02X} ", buf[i]).expect("Unable to write");
            }
    
            println!("Got: {}", dbg);
    
            match buf[0] {
                0x09 => {
                    let mut response = Vec::<u8>::new();
                    let guid = GenerateGUID();
    
                    response.push(10);
                    response.push(0);
    
                    sock.send_to(response.as_slice(), addr).await.unwrap();
                }
                _ => {
                    // 0000   40 e7 7b c1 c0 00 00 00 13 80 05
                    // 0000   7d 28 3b c1 c0 00 00 00 13 80 05
                    // 0000   72 6a 3b c1 c0 00 00 00 13 80 05
                    // 0000   5b ee 3b c1 c0 00 00 00 13 80 05

                    match nom::error::dbg_dmp(parse_msg, "RakNet")(&buf[0..len]) {
                        Ok((i, message)) => {
                            println!("Got Message Num {} - {} {:#?} {:#?} {:#?}", 
                            message.packet.number, 
                            if message.remote_time.is_some() { 
                                message.received_time.format("%Y-%m-%d %H:%M:%S").to_string()
                            } else {
                                "".to_string()
                            },
                            message.packet.reliability,
                            message.packet.split,
                            message.packet.data);
                        },
                        Err(_) => {

                        }
                    }

                },
            }
        }
    });
    
    tokio::spawn(async move {
        loop {
            let (con, adr) = que_listener.accept().await.unwrap();

        }
    });

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    Ok(())
}
