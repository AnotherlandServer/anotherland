use nom::{IResult, Err, error::{VerboseError, context, ErrorKind}, combinator::*, sequence::*, multi::*, number::complete::*};
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use crate::atlas::parsers::*;
use std::io;
use std::cmp::min;
use crate::raknet::Message;
use crate::atlas::NativeParam;

include!(concat!(env!("OUT_DIR"), "/generated_packets.rs"));
