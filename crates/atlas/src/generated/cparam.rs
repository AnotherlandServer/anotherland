use nom::IResult;
use nom::error::VerboseError;
use nom::error::context;
use nom::multi::count;
use nom::number;
use nom::combinator::fail;
use nom::combinator::success;
use nom::sequence::tuple;
use crate::CParam;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use serde::{Serialize, Deserialize};

include!(concat!(env!("OUT_DIR"), "/generated_params.rs"));