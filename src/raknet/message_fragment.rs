use std::io;
use std::time::Duration;

use bitstream_io::{BitWriter, BitWrite};
use nom::bytes;
use nom::combinator::{flat_map, cond, fail};
use nom::error::context;
use nom::multi::{self, length_count};
use nom::sequence::tuple;
use nom::{IResult, error::VerboseError, combinator::map, multi::count};
use nom::bits::{complete as bit_parser, self};

use super::Message;

pub type MessageNumber = u32;
pub type AckRange = (MessageNumber, MessageNumber);
pub type BitInput<'a> = (&'a [u8], usize);

#[derive(Debug, Clone, Copy)]
pub enum PacketSplit {
    NotSplit,
    Split{ id: u16, index: u32, count: u32},
}

#[derive(Debug, Clone, Copy)]
pub enum Reliability {
    Unreliable,
    UnreliableSequenced(PacketSequence),
    Reliable,
    ReliableOrdered(PacketSequence),
    ReliableSequenced(PacketSequence),
}

#[derive(Debug, Clone, Copy)]
pub struct PacketSequence {
    pub channel: u8,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct OnlineMessage {
    pub number: MessageNumber,
    pub reliability: Reliability,
    pub split: PacketSplit,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum MessageFragment {
    Ack(Duration, Vec<AckRange>),
    SystemTime(Duration),
    OfflineMessage(Message),
    OnlineMessage(OnlineMessage),
    /*RawResponse {
        number: MessageNumber,
        reliability: Reliability,
        split: PacketSplit,
        message: Message,
    }*/
}

impl MessageFragment {
    fn parse_duration(i: BitInput) -> IResult<BitInput, Duration, VerboseError<BitInput>> {
        context("duration", map(
            count(bit_parser::take(8usize), 4), 
            |v: Vec<u8>| Duration::from_millis(
                u32::from_le_bytes(v.as_slice().try_into().unwrap()) as u64
            )
        ))(i)
    }

    fn parse_message_number(i: BitInput) -> IResult<BitInput, MessageNumber, VerboseError<BitInput>> {
        context("message_number", map(
            count(bit_parser::take(8usize), 4), 
            |v: Vec<u8>| {
                let r = u32::from_le_bytes(v.as_slice().try_into().unwrap());
                r
            }
        ))(i)
    }

    
    fn parse_sequence(i: BitInput) -> IResult<BitInput, PacketSequence, VerboseError<BitInput>> {
        context("sequence", map(tuple((
            bit_parser::take(5usize), 
            bit_parser::take(32usize)
        )), |(channel, index)| PacketSequence { channel, index }))(i)
    }

    fn parse_reliability(i: BitInput) -> IResult<BitInput, Reliability, VerboseError<BitInput>> {
        context("reliability", flat_map( bit_parser::take(3usize),
            |reliability| {
                match reliability {
                    0 => |i| Ok((i, Reliability::Unreliable)),
                    1 => |i| {
                        map(Self::parse_sequence, |seq| Reliability::UnreliableSequenced(seq))(i)
                    },
                    2 => |i| Ok((i, Reliability::Reliable)),
                    3 => |i| {
                        map(Self::parse_sequence, |seq| Reliability::ReliableOrdered(seq))(i)
                    },
                    4 => |i| {
                        map(Self::parse_sequence, |seq| Reliability::ReliableSequenced(seq))(i)
                    },
                    _ => |i| fail(i)
                }
            }))(i)

    }

    fn parse_packet_split(i: BitInput) -> IResult<BitInput, PacketSplit, VerboseError<BitInput>> {
        let (i, is_split) = bit_parser::bool(i)?;
    
        if is_split {
            let (i, (id, index, count)) = 
                map(
                    tuple((
                        Self::parse_compressed_bytes(2usize, true),
                        Self::parse_compressed_bytes(4usize, true), 
                        Self::parse_compressed_bytes(4usize, true)
                    )),
                    |r| (
                        u16::from_le_bytes(r.0[0..2].try_into().unwrap()),
                        u32::from_le_bytes(r.1[0..4].try_into().unwrap()), 
                        u32::from_le_bytes(r.2[0..4].try_into().unwrap())
                    )
                )(i)?;
            Ok((i, PacketSplit::Split { id, index, count }))
        } else {
            Ok((i, PacketSplit::NotSplit))
        }
    }

    fn parse_compressed_bytes(
        count: usize, 
        unsigned: bool
    ) -> impl Fn(BitInput) -> IResult<BitInput, Vec<u8>, VerboseError<BitInput>> 
    {
        move |mut i| {
            //print!("Input: {:#?}", i);
    
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
                let (i2, is_compressed) = bit_parser::bool(i)?;
                i = i2;
    
                if is_compressed {
                    result[upper_bytes - b] = byte_match;
                } else {
                    let (i, uncompressed): (_, Vec<u8>) = multi::count(bit_parser::take(8usize), count - b)(i)?;
                    result[0..count-b].copy_from_slice(&uncompressed);
    
                    //print!("Output 1: {:#?}", result);
                    return Ok((i, result));
                }
            }
    
            // Uncompress first byte, if all upper bytes where compressed (equal to byte_match)
            let (i, is_negative) = bit_parser::bool(i)?;
            if is_negative {
                let (i, first_byte): (_, u8) = bit_parser::take(4usize)(i)?;
                result[0] = nibble_match | first_byte;
    
                //print!("Output 2: {:#?}", result);
                return Ok((i, result));
            } else {
                let (i, first_byte): (_, u8) = bit_parser::take(8usize)(i)?;
                result[0] = first_byte;
    
                //print!("Output 3: {:#?}", result);
                return Ok((i, result));
            }
        }
    }

    fn parse_data(i: BitInput) -> IResult<BitInput, Vec<u8>, VerboseError<BitInput>> {
        let (i, message_len) = context("message length", Self::parse_compressed_bytes(2usize, true))(i)?;

        context("message data", map(bits::bytes(
            bytes::complete::take::<_, _, VerboseError<&[u8]>>(u16::from_le_bytes(message_len.try_into().unwrap()) / 8)
        ), |r| r.to_vec()))(i)
    }

    pub fn parse_ack<'a>(input: BitInput<'a>) -> IResult<BitInput<'a>, MessageFragment, VerboseError<BitInput<'a>>> {
        context("ack", map(tuple((
            Self::parse_duration,
            length_count(
                context("ack entries", map(
                    Self::parse_compressed_bytes(2, true),
                    |v| {
                        let r = u16::from_le_bytes(v.as_slice().try_into().unwrap());
                        //println!("Ack count {}", r);
                        r
                    }
                )), 
                //Pase range entry
                context("ack data", flat_map(bit_parser::bool, 
                    |max_equals_min| {
                        //println!("Max = Min: {}", max_equals_min);
                        map(
                            tuple((
                                context("min message id", Self::parse_message_number),
                                context("max message id", cond(
                                    !max_equals_min, 
                                    Self::parse_message_number
                                ))
                            )),
                            |(start, end)| (start, end.unwrap_or(start))
                        )
                    }))
            )
        )), |(system_time, ack_range)| Self::Ack(system_time, ack_range)))(input)
    }

    pub fn parse_system_time<'a>(input: BitInput<'a>) -> IResult<BitInput<'a>, MessageFragment, VerboseError<BitInput<'a>>> {
        context("system_time", map(
            Self::parse_duration, 
            |duration| Self::SystemTime(duration))
        )(input)
    }

    pub fn parse_packet<'a>(input: BitInput<'a>) -> IResult<BitInput<'a>, MessageFragment, VerboseError<BitInput<'a>>> {
        context("message", map(tuple((
            // Message Number
            context("message number", Self::parse_message_number),
            context("reliability", Self::parse_reliability),
            context("split", Self::parse_packet_split),
            context("data", Self::parse_data),
        )), |(number, reliability, split, data)| {
            Self::OnlineMessage(OnlineMessage { number, reliability, split, data })
        }))(input)
    }
}

impl MessageFragment {
    pub fn serialize_to_bitwriter<E, W>(&self, writer: &mut BitWriter<E, W>) -> io::Result<()> 
    where
    E: io::Write,
    W: bitstream_io::Endianness
    {
        match self {
            Self::Ack(time, id_ranges) => {
                // Ack time
                writer.write(32, (time.as_millis() as u32).to_be())?;

                // Ack count
                Self::write_compressed(writer, true, &(id_ranges.len() as u16).to_le_bytes())?;
                for ack in id_ranges {
                    let id_count = (ack.1 + 1) - ack.0;

                    writer.write_bit(id_count == 1)?;
                    writer.write(32, ack.0.to_be())?;
                    if id_count > 1 {
                        writer.write(32, (ack.1).to_be())?;
                    }
                }
            },

            Self::SystemTime(time) => {
                writer.write(32, (time.as_millis() as u32).to_be())?; // Ack time
            },

            Self::OfflineMessage(message) => {
                writer.byte_align()?;
                writer.write_bytes(message.to_bytes().as_slice())?;
            },

            Self::OnlineMessage(message) => {
                writer.write(32, message.number.to_be())?; // Packet number

                // Reliability
                match message.reliability {
                    Reliability::Unreliable => writer.write(3, 0)?,
                    Reliability::UnreliableSequenced(sequence) => {
                        writer.write(3, 1)?;
                        writer.write(5, sequence.channel.to_be())?;
                        writer.write(32, sequence.index.to_be())?;
                    },
                    Reliability::Reliable => writer.write(3, 2)?,
                    Reliability::ReliableOrdered(sequence) => {
                        writer.write(3, 3)?;
                        writer.write(5, sequence.channel.to_be())?;
                        writer.write(32, sequence.index.to_be())?;
                    },
                    Reliability::ReliableSequenced(sequence) => {
                        writer.write(3, 4)?;
                        writer.write(5, sequence.channel.to_be())?;
                        writer.write(32, sequence.index.to_be())?;
                    },
                }
        
                // Split
                match message.split {
                    PacketSplit::NotSplit => writer.write_bit(false)?,
                    PacketSplit::Split { id, index, count } => {
                        writer.write_bit(true)?;
                        writer.write(16, id.to_be())?;
                        Self::write_compressed(writer, true, &index.to_le_bytes())?;
                        Self::write_compressed(writer, true, &count.to_le_bytes())?;
                    }
                }

                // Message
                Self::write_compressed(writer, true, &((message.data.len() * 8) as u16).to_le_bytes())?;
                writer.byte_align()?; // Align bytes to start writing payload
                writer.write_bytes(&message.data)?;
            },

            _ => panic!("Unsupported message type for serialization"),
        }

        Ok(())
    }

    fn write_compressed<E, W>(w: &mut BitWriter<E, W>, unsigned: bool, data: &[u8]) -> io::Result<()> 
    where
    E: io::Write,
    W: bitstream_io::Endianness
    {
        let byte_match = if unsigned { 0 } else { 0xFF };

        for b in (1..data.len()).rev() {
            if data[b] == byte_match {
                w.write_bit(true)?;
            } else {
                w.write_bit(false)?;
                return w.write_bytes(&data[0..b+1]);
            }
        }

        if (data[0] & 0xF0) == (byte_match & 0xF0) {
            w.write_bit(true)?;
            w.write(4, data[0] & 0x0F)?;
        } else {
            w.write_bit(false)?;
            w.write(8, data[0])?;
        }

        Ok(())
    }
}