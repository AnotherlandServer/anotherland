mod cpkt;
mod parsers;
mod uuid;

use nom::{IResult, Err, error::{VerboseError, context, ErrorKind}, combinator::*, sequence::*, multi::*, number::complete::*};
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use parsers::*;
use std::io;
use std::cmp::min;
use super::raknet::Message;
use parsers::*;

include!(concat!(env!("OUT_DIR"), "/generated_packets.rs"));

pub use cpkt::*;
pub use self::uuid::*;