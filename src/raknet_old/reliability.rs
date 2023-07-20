use std::{time::Duration, collections::VecDeque, ops::Range, io};

use bitstream_io::{BitWriter, LittleEndian, BitWrite, BigEndian, BitQueue, Endianness, Numeric};
use chrono::{DateTime, Utc, NaiveDateTime};
use nom::{bits, bits::complete as bit_parser, sequence::tuple, combinator::{map, fail, opt, value, flat_map, success, cond, map_res, map_parser, into, eof, rest_len}, IResult, number::complete as num_parser, multi::{length_count, self, count}, bytes::{complete::take}, error::{context, VerboseError, ParseError, ErrorKind}, branch::alt};
use nom::error::convert_error;

use super::{Message};

pub type MessageNumber = u32;
pub type AckRange = Range<MessageNumber>;
pub type SystemTime = Duration;

/*#[derive(Debug)]
pub struct Acks {
    own_time: SystemTime,
    acks: Vec<AckRange>
}

impl Acks {
    pub fn new(own_time: SystemTime, acks: Vec<AckRange>) -> Self {
        Acks {
            own_time,
            acks
        }
    }

    pub fn get_own_time(&self) -> SystemTime { self.own_time }
    pub fn get_acks(&self) -> &Vec<AckRange> { &self.acks }
    pub fn has_acks(&self) -> bool { !self.acks.is_empty() }
}*/


#[derive(Debug)]
pub enum ReliableMessage {
    Ack{time: SystemTime, id_ranges: Vec<AckRange>},
    MessageFrame{
        id: u32,
        reliability: Reliability,
        split: PacketSplit,
        message: Message,
    }
}
/*pub struct ReliableMessage {
    pub number: u32,
    pub date: DateTime<Utc>,
    pub reliability: Reliability,
    pub split: PacketSplit,
    pub message: Message,
}*/

#[derive(Debug)]
pub enum PacketSplit {
    NotSplit,
    Split{ id: u16, index: u32, count: u32},
}

#[derive(Debug)]
pub enum Reliability {
    Unreliable,
    UnreliableSequenced(PacketSequence),
    Reliable,
    ReliableOrdered(PacketSequence),
    ReliableSequenced(PacketSequence),
}



type BitInput<'a> = (&'a [u8], usize);

#[derive(Debug)]
struct DebugError {
    message: String,
}

/*
Ack bit 
    System time (u32)
    Elements Count
        Min=Max Flag
        Min
            Max
System Time bit
    System Time
Message Number
Reliability (3 bits)
    Ordering Channel (5 bits)
    Ordering index (u32)
Is split bit
    Split packet id
    Split packet index
    Split packet count
Data bit length
(Aligned) Message bytes




 */

impl ReliableMessage { 
    fn parse<'a>(b: &'a [u8]) -> IResult<&'a [u8], (Option<Duration>, ReliableMessage), VerboseError<&'a [u8]>> {
        bits(map(
            tuple((
                // Parse acks
                context("acks", flat_map(
                    bit_parser::bool, 
                    |has_acks| cond(has_acks, map(
                            tuple((
                                // Parse own time
                                context("other_time", ReliableMessage::parse_duration),

                                // Prase ack list
                                length_count(
                                    context("ack entries", map(
                                        ReliableMessage::parse_compressed_bytes(2, true),
                                        |v| {
                                            let r = u16::from_le_bytes(v.as_slice().try_into().unwrap());
                                            println!("Ack count {}", r);
                                            r
                                        }
                                    )), 
                                    //Pase range entry
                                    context("ack data", flat_map(bit_parser::bool, 
                                    |max_equals_min| {
                                        println!("Max = Min: {}", max_equals_min);
                                        map(
                                            tuple((
                                                context("min message id", ReliableMessage::parse_message_number),
                                                context("max message id", cond(
                                                    !max_equals_min, 
                                                    ReliableMessage::parse_message_number
                                                ))
                                            )),
                                            |(start, end)| Range { 
                                                start, 
                                                end: end.unwrap_or(start) + 1
                                            }
                                        )
                                    }))
                                )
                            )), 
                            |(time, id_ranges)| 
                                ReliableMessage::Ack { time, id_ranges }
                        )
                    )
                )),

                // Parse time
                context("system_time", map(
                    opt(
                        flat_map(
                            bit_parser::bool,
                            |has_time|
                                cond(has_time, ReliableMessage::parse_duration)
                        )),
                    |r| r.unwrap_or(None)
                )),

                // Main packet
                context("message", flat_map(rest_len,
                    |r| {
                        println!("Remaining bits: {}", r);
                        cond(r >= 8, tuple((
                            // Message Number
                            context("message number", ReliableMessage::parse_message_number),

                            context("reliability", ReliableMessage::parse_reliability),
                            context("split", ReliableMessage::parse_packet_split),
                            context("message", ReliableMessage::parse_message),
                        )))
                    }
                )),
            )),
            |(ack, system_time, message)| {
                if let Some(ack) = ack {
                    (system_time, ack)
                }else if let Some((id, reliability, split, message)) = message {
                    (system_time, ReliableMessage::MessageFrame { id, reliability, split, message })
                } else {
                    panic!("Got neither acks nor a message. Did you try parsing an empty message?");
                }
            }
        ))(b)
    }

    pub fn from_bytes(b: &[u8]) -> IResult<&[u8], (Option<Duration>, ReliableMessage), VerboseError<&[u8]>> {
        nom::error::dbg_dmp(ReliableMessage::parse, "ReliableMessage")(b)

        /*match nom::error::dbg_dmp(ReliableMessage::parse, "ReliableMessage")(b) {
            Ok((_, message)) => Ok(message),
            Err(e) => match e {
                nom::Err::Incomplete(e) => {
                    println!("E: {:#?}", e.to_owned());
                    Err(ParseError::UnexpectedEndOfData)
                },
                nom::Err::Error(e) => {
                    println!("E: {:#?}", e.to_owned());
                    Err(ParseError::InvalidData)
                },
                nom::Err::Failure(e) => {
                    println!("E: {:#?}", e.to_owned());
                    Err(ParseError::InternalError)
                },
            },
        }*/
    }

    pub fn to_bytes(&self, peer_time: Option<Duration>) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        match self {
            Self::Ack { time, id_ranges } => {
                writer.write_bit(true)?; // Enable ack preset bit
                writer.write(32, (time.as_millis() as u32).to_be())?; // Ack time
                
                // Ack count
                Self::write_compressed(&mut writer, true, &(id_ranges.len() as u16).to_le_bytes())?;
                for ack in id_ranges {
                    let id_count = ack.clone().count();

                    writer.write_bit(id_count == 1)?;
                    writer.write(32, ack.start.to_be())?;
                    if id_count > 1 {
                        writer.write(32, (ack.end - 1).to_be())?;
                    }
                }

                // Write optional system time
                match peer_time {
                    Some(time) => {
                        writer.write_bit(true)?;
                        writer.write(32, (time.as_millis() as u32).to_be())?; // Ack time
                    },
                    None => writer.write_bit(false)?,
                }
            },
            Self::MessageFrame { id, reliability, split, message } => {
                writer.write_bit(false)?; // Disable ack preset bit

                // Write optional system time
                match peer_time {
                    Some(system_time) => {
                        writer.write_bit(true)?;
                        writer.write(32, (system_time.as_millis() as u32).to_be())?; // Ack time
                    },
                    None => writer.write_bit(false)?,
                }

                writer.write(32, id.to_be())?; // Packet number

                // Reliability
                match reliability {
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
                match split {
                    PacketSplit::NotSplit => writer.write_bit(false)?,
                    PacketSplit::Split { id, index, count } => {
                        writer.write_bit(true)?;
                        writer.write(16, id.to_be())?;
                        Self::write_compressed(&mut writer, true, &index.to_le_bytes())?;
                        Self::write_compressed(&mut writer, true, &count.to_le_bytes())?;
                    }
                }

                // Message
                let msg = message.to_bytes();
                Self::write_compressed(&mut writer, true, &((msg.len() * 8) as u16).to_le_bytes())?;
                writer.byte_align()?; // Align bytes to start writing payload
                writer.write_bytes(&msg)?;
            }
        }

        Ok(buf)
    }

    fn write_compressed<E, W>(w: &mut BitWriter<E, W>, unsigned: bool, data: &[u8]) -> io::Result<()> 
    where
    E: io::Write,
    W: bitstream_io::Endianness
    {
        let byte_match = if unsigned { 0 } else { 0xFF };

        for b in (1..data.len()).rev() {
            println!("Byte [{}] {:02X}", b, data[b]);

            if data[b] == byte_match {
                w.write_bit(true)?;
            } else {
                w.write_bit(false)?;
                return w.write_bytes(&data[0..b+1]);
            }
        }

        println!("Byte [{}] {:02X}", 0, data[0]);

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

// Private parsers
impl ReliableMessage {
    fn parse_duration(i: BitInput) -> IResult<BitInput, Duration, VerboseError<BitInput>> {
        map(
            count(bit_parser::take(8usize), 4), 
            |v: Vec<u8>| Duration::from_millis(
                u32::from_le_bytes(v.as_slice().try_into().unwrap()) as u64
            )
        )(i)
    }

    fn parse_message_number(i: BitInput) -> IResult<BitInput, MessageNumber, VerboseError<BitInput>> {
        map(
            count(bit_parser::take(8usize), 4), 
            |v: Vec<u8>| {
                let r = u32::from_le_bytes(v.as_slice().try_into().unwrap());
                r
            }
        )(i)
    }

    fn parse_message(i: BitInput) -> IResult<BitInput, Message, VerboseError<BitInput>> {
        flat_map(
            ReliableMessage::parse_compressed_bytes(2usize, true),
            |len| bits::bytes(Message::parse)
        )(i)
    }

    fn parse_sequence(i: BitInput) -> IResult<BitInput, PacketSequence, VerboseError<BitInput>> {
        let (i, (channel, index)) : (_, (u8, u32)) = tuple((bit_parser::take(5usize), bit_parser::take(32usize)))(i)?;
        Ok((i, PacketSequence { channel, index }))
    }

    fn parse_reliability(i: BitInput) -> IResult<BitInput, Reliability, VerboseError<BitInput>> {
        let (i, reliability) = bit_parser::take(3usize)(i)?;
        match reliability {
            0 => Ok((i, Reliability::Unreliable)),
            1 => {
                let (i, seq) = ReliableMessage::parse_sequence(i)?;
                Ok((i, Reliability::UnreliableSequenced(seq)))
            },
            2 => Ok((i, Reliability::Reliable)),
            3 => {
                let (i, seq) = ReliableMessage::parse_sequence(i)?;
                Ok((i, Reliability::ReliableOrdered(seq)))
            },
            4 => {
                let (i, seq) = ReliableMessage::parse_sequence(i)?;
                Ok((i, Reliability::ReliableSequenced(seq)))
            }
            _ => fail(i)
        }
    }

    fn parse_packet_split(i: BitInput) -> IResult<BitInput, PacketSplit, VerboseError<BitInput>> {
        let (i, is_split) = bit_parser::bool(i)?;
    
        if is_split {
            let (i, (id, index, count)) = 
                map(
                    tuple((
                        ReliableMessage::parse_compressed_bytes(2usize, true),
                        ReliableMessage::parse_compressed_bytes(4usize, true), 
                        ReliableMessage::parse_compressed_bytes(4usize, true)
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
}

#[cfg(test)]
mod tests {
    use bitstream_io::{BitWriter, BitWrite, BigEndian};
    use nom::{bits, error::VerboseError, IResult};

    use super::ReliableMessage;

    fn parse_compressed_bytes_jig(b: &[u8]) -> IResult<&[u8], (Vec<u8>), VerboseError<&[u8]>> {
        bits(ReliableMessage::parse_compressed_bytes(4, true))(b)
    }

    fn compare_result_arrays(a: &[u8], b: &[u8]) -> bool {
        let mut r = true;

        for i in 0..a.len() {
            if a[i] != b[i] {
                println!("! {:02X} - {:02X} {:08b} - {:08b}", a[i], b[i], a[i], b[i]);
                r = false;
            } else {
                println!("  {:02X} - {:02X} {:08b} - {:08b}", a[i], b[i], a[i], b[i]);
            }
        }

        r
    }

    #[test]
    fn compressed_bytes_1() {
        let mut buf = Vec::<u8>::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        let original = 12345678u32.to_le_bytes();
        
        ReliableMessage::write_compressed(&mut writer, true, original.as_slice()).unwrap();
        writer.byte_align().unwrap();

        let (_, result) = parse_compressed_bytes_jig(buf.as_slice()).unwrap();

        assert!(compare_result_arrays(&original, result.as_slice()))
    }

    #[test]
    fn compressed_bytes_2() {
        let mut buf = Vec::<u8>::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        let original = 0u32.to_le_bytes();
        
        ReliableMessage::write_compressed(&mut writer, true, original.as_slice()).unwrap();
        writer.byte_align().unwrap();

        let (_, result) = parse_compressed_bytes_jig(buf.as_slice()).unwrap();

        assert!(compare_result_arrays(&original, result.as_slice()))
    }

    #[test]
    fn compressed_bytes_3() {
        let mut buf = Vec::<u8>::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        let original = 1u32.to_le_bytes();
        
        ReliableMessage::write_compressed(&mut writer, true, original.as_slice()).unwrap();
        writer.byte_align().unwrap();

        let (_, result) = parse_compressed_bytes_jig(buf.as_slice()).unwrap();

        assert!(compare_result_arrays(&original, result.as_slice()))
    }

    #[test]
    fn compressed_bytes_4() {
        let mut buf = Vec::<u8>::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        let original = 8u32.to_le_bytes();
        
        ReliableMessage::write_compressed(&mut writer, true, original.as_slice()).unwrap();
        writer.byte_align().unwrap();

        let (_, result) = parse_compressed_bytes_jig(buf.as_slice()).unwrap();

        assert!(compare_result_arrays(&original, result.as_slice()))
    }
}