use std::collections::HashMap;

use bitstream_io::ByteWriter;
use bitstream_io::LittleEndian;
use bitstream_io::ByteWrite;
use nom::IResult;
use nom::bytes::complete::take;
use nom::error::VerboseError;
use nom::multi;
use nom::multi::count;
use nom::number;
use nom::combinator::fail;

use glam::f32::{Vec3, Vec4};
use nom::number::complete::le_u32;
use super::generated::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CParam {
    String(String), // 1, 22, 23, 24, 25, 26, 43
    Int64(i64), // 19
    Bool(bool), // 7
    CAvatarID(u64), // 16
    CGuid(Uuid), // 5
    LocalizedString(Uuid), // 15
    Any(u32, Vec<u8>), // 41
    Positionable, // 21
    Vector3(Vec3), // 13
    Vector4(Vec4), // 14
    FloatArray2(Vec<f32>), // 12
    IntArray4(Vec<u32>), // 9
    JsonValue(String), // 18
    Quarternion, // 20
    Bitset, // 10
    CAvatarIDSet, // 35
    CGuidSet, // 37
    TStringSet, // 4
    Int64Array(Vec<u64>), // 30
    CAvatarIDArray, // 36
    CGuidArray, // 38, 42
    StringArray(Vec<String>), // 32
    FloatArray(Vec<f32>), // 31
    IntArray(Vec<u32>), // 29
    StringMap(HashMap<String, String>), // 40
    IntMap(HashMap<String, u32>), // 39
    Float(f32), // 11
    Int32(i32), // 8, 17
    CGuidPair((Uuid, Uuid)), // 6
    StringPair((String, String)), // 2
    StringFloatPair((String, f32)), // 3
}

impl CParam {
    pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], CParam, VerboseError<&'a [u8]>> {
        let (i, type_id) = number::complete::le_u8(i)?;
        match type_id & 0x7F {
            1 | 22 | 23 | 24 | 25 | 26 | 43 => {
                let (i, len) = number::complete::le_u16(i)?;
                let (i, bytes) = take(len as usize)(i)?;

                if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                    Ok((i, CParam::String(string)))
                } else {
                    println!("Failed to parse string: {:#?}", bytes);
                    fail(i)
                }
            }
            5 => {
                let (i, uuid) = Uuid::from_bytes(i)?;

                Ok((i, CParam::CGuid(uuid)))
            },
            7 => {
                let (i, val) = number::complete::le_u8(i)?;
                Ok((i, CParam::Bool(val != 0)))
            },
            8 | 17 => {
                let (i, val) = number::complete::le_i32(i)?;
                Ok((i, CParam::Int32(val)))
            },
            11 => {
                let (i, val) = number::complete::le_f32(i)?;
                Ok((i, CParam::Float(val)))
            },
            13 => {
                let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                Ok((i, CParam::Vector3(Vec3::new(val[0], val[1], val[2]))))
            },
            14 => {
                let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                Ok((i, CParam::Vector4(Vec4::new(val[0], val[1], val[2], val[3]))))
            },
            15 => {
                let (i, uuid) = Uuid::from_bytes(i)?;

                Ok((i, CParam::LocalizedString(uuid)))
            },
            16 => {
                let (i, id) = number::complete::le_u64(i)?;

                Ok((i, CParam::CAvatarID(id)))
            },
            18 => {
                let (i, len) = number::complete::le_u16(i)?;
                let (i, bytes) = take(len as usize)(i)?;

                if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                    Ok((i, CParam::JsonValue(string)))
                } else {
                    println!("Failed to parse json: {:#?}", bytes);
                    fail(i)
                }
            },
            19 => {
                let (i, val) = number::complete::le_i64(i)?;
                Ok((i, CParam::Int64(val)))
            },
            29 => {
                let (i, count) = number::complete::le_u32(i)?;
                let (i, data) = multi::count(le_u32, count as usize)(i)?;
                Ok((i, CParam::IntArray(data)))
            }
            41 => {
                let (i, len) = number::complete::le_u32(i)?;
                let (i, class) = number::complete::le_u32(i)?;
                let (i, data) = take(len as usize)(i)?;

                Ok((i, CParam::Any(class, data.to_vec())))
            }
            _ => {
                println!("Invalid type_id {:#?}", type_id & 0x7F);
                fail(i)
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        match self {
            Self::String(val) => {
                let _ = writer.write(1u8);
                let _ = writer.write(val.len() as u16);
                let _ = writer.write_bytes(val.as_bytes());
            },
            Self::Bool(val) => {
                let _ = writer.write(7u8);
                let _ = writer.write(if *val { 1u8 } else { 0u8 });
            },
            Self::Int32(val) => {
                let _ = writer.write(8u8);
                let _ = writer.write(*val);
            },
            Self::Float(val) => {
                let _ = writer.write(11u8);
                let _ = writer.write_bytes(val.to_le_bytes().as_slice());
            },
            Self::Vector3(val) => {
                let _ = writer.write(13u8);
                let _ = writer.write_bytes(val.x.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.y.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.z.to_le_bytes().as_slice());
            },
            Self::Vector4(val) => {
                let _ = writer.write(14u8);
                let _ = writer.write_bytes(val.x.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.y.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.z.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.w.to_le_bytes().as_slice());
            },
            Self::CGuid(val) => {
                let _ = writer.write(5u8);
                let _ = writer.write_bytes(val.to_bytes().as_slice());
            },
            Self::LocalizedString(val) => {
                let _ = writer.write(15u8);
                let _ = writer.write_bytes(val.to_bytes().as_slice());
            },
            Self::CAvatarID(val) => {
                let _ = writer.write(16u8);
                let _ = writer.write(*val);
            },
            Self::JsonValue(val) => {
                let _ = writer.write(18u8);
                let _ = writer.write(val.len() as u16);
                let _ = writer.write_bytes(val.as_bytes());
            },
            Self::Int64(val) => {
                let _ = writer.write(19u8);
                let _ = writer.write(*val);
            },
            Self::IntArray(val) => {
                let _ = writer.write(29u8);
                let _ = writer.write(val.len() as u32);
                for &i in val {
                    let _ = writer.write(i);
                }
            }
            Self::Any(class, val) => {
                let _ = writer.write(41u8);
                let _ = writer.write(*class);
                let _ = writer.write(val.len() as u32);
                let _ = writer.write_bytes(val);
            }
            _ => todo!(),
        }

        buf
    }
}