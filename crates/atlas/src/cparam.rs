use core::fmt;
use std::collections::HashMap;
use std::error::Error;

use bitstream_io::ByteWriter;
use bitstream_io::LittleEndian;
use bitstream_io::ByteWrite;
use nom::IResult;
use nom::bytes::complete::take;
use nom::error::VerboseError;
use nom::error::context;
use nom::multi;
use nom::multi::count;
use nom::number;
use nom::combinator::fail;
use serde_hex::{SerHex, StrictPfx};

use glam::f32::{Vec3, Vec4};
use nom::number::complete::le_u32;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use super::generated::*;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CParamError(());

impl fmt::Display for CParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cparam error")
    }
}

impl Error for CParamError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "v")]
pub enum CParam {
    String(String), // 1, 22, 23, 24, 25, 26, 43
    Int64(i64), // 19
    Bool(bool), // 7
    CAvatarID(
        #[serde(with = "SerHex::<StrictPfx>")]
        u64
    ), // 16
    CGuid(Uuid), // 5
    LocalizedString(Uuid), // 15
    Any(u32, Vec<u8>), // 41
    Positionable, // 21
    Vector3(Vec3), // 13
    Vector4(Vec4), // 14
    FloatArray2([f32;2]), // 12
    IntArray4(Vec<u32>), // 9
    JsonValue(Value), // 18
    Quarternion, // 20
    Bitset, // 10
    CAvatarIDSet, // 35
    CGuidSet, // 37
    TStringSet, // 4
    Int64Array(Vec<u64>), // 30
    CAvatarIDArray, // 36
    CGuidArray(Vec<Uuid>), // 38, 42
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
                context("String", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, CParam::String(string)))
                    } else {
                        println!("Failed to parse string: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            }
            5 => {
                context("Uuid", |i| {
                    let (i, uuid) = Uuid::from_bytes(i)?;

                    Ok((i, CParam::CGuid(uuid)))
                })(i)
            },
            7 => {
                context("Bool", |i| {
                    let (i, val) = number::complete::le_u8(i)?;
                    Ok((i, CParam::Bool(val != 0)))
                })(i)
            },
            8 | 17 => {
                context("Int32", |i| {
                    let (i, val) = number::complete::le_i32(i)?;
                    Ok((i, CParam::Int32(val)))
                })(i)
            },
            11 => {
                context("Float", |i| {
                    let (i, val) = number::complete::le_f32(i)?;
                    Ok((i, CParam::Float(val)))
                })(i)
            },
            12 => {
                context("FloatArray2", |i| {
                    let (i, val) = count(number::complete::le_f32, 2usize)(i)?;
                    Ok((i, CParam::FloatArray2([val[0], val[1]])))
                })(i)
            },
            13 => {
                context("Vector3", |i| {
                    let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                    Ok((i, CParam::Vector3(Vec3::new(val[0], val[1], val[2]))))
                })(i)
            },
            14 => {
                context("Vector4", |i| {
                    let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                    Ok((i, CParam::Vector4(Vec4::new(val[0], val[1], val[2], val[3]))))
                })(i)
            },
            15 => {
                context("LocalizedString", |i| {
                    let (i, uuid) = Uuid::from_bytes(i)?;

                    Ok((i, CParam::LocalizedString(uuid)))
                })(i)
            },
            16 => {
                context("AvatarID", |i| {
                    let (i, id) = number::complete::le_u64(i)?;

                    Ok((i, CParam::CAvatarID(id)))
                })(i)
            },
            18 => {
                context("JsonValue", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;

                    if bytes.is_empty() {
                        Ok((i, CParam::JsonValue(Value::Null)))
                    } else {
                        if let Ok(json) = serde_json::from_slice(bytes) {
                            Ok((i, CParam::JsonValue(json)))
                        } else {
                            // Try to fixup the json, as serde_json seems to be more strict than whatever parser otherland 
                            // is using. Or they just ignored those errors...
                            let fixed_json = String::from_utf8_lossy(bytes)
                                .replace("},]", "}]")
                                .replace("\":.", "\":0.")
                                .replace("},,{", "},{")
                                .replace("\"QuestID\":,\"QuestTag\":,", "\"QuestID\":null,\"QuestTag\":null,");

                            if let Ok(json) = serde_json::from_str(&fixed_json) {
                                Ok((i, CParam::JsonValue(json)))
                            } else {
                                println!("Failed to parse json: \"{}\"", String::from_utf8_lossy(bytes));
                                fail(i)
                            }
                        }
                    }
                })(i)
            },
            19 => {
                context("Int64", |i| {
                    let (i, val) = number::complete::le_i64(i)?;
                    Ok((i, CParam::Int64(val)))
                })(i)
            },
            29 => {
                context("IntArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_u32, count as usize)(i)?;
                    Ok((i, CParam::IntArray(data)))
                })(i)
            }
            32 => {
                context("StringArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(|i: &'a [u8]| -> Result<(&'a [u8], String), nom::Err<VerboseError<&'a [u8]>>> {
                        let (i, len) = number::complete::le_u16(i)?;
                        let (i, bytes) = take(len as usize)(i)?;
        
                        if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                            Ok((i, string))
                        } else {
                            println!("Failed to parse string: {:#?}", bytes);
                            fail(i)
                        }
                    }, count as usize)(i)?;
                    Ok((i, CParam::StringArray(data)))
                })(i)
            },
            41 => {
                context("Array", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u32(i)?;
                    let (i, class) = number::complete::le_u32(i)?;
                    let (i, data) = take(len as usize)(i)?;

                    Ok((i, CParam::Any(class, data.to_vec())))
                })(i)
            }
            38 | 42 => {
                context("GuidArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(Uuid::from_bytes, count as usize)(i)?;

                    Ok((i, CParam::CGuidArray(data)))
                })(i)
            },
            _ => {
                context("InvalidType", |i| {
                    println!("Invalid type_id {:#?}", type_id & 0x7F);
                    fail(i)
                })(i)
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
                let json = serde_json::to_string(val).unwrap();

                let _ = writer.write(18u8);
                let _ = writer.write(json.len() as u16);
                let _ = writer.write_bytes(json.as_bytes());
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

impl From<bool> for CParam {
    fn from(value: bool) -> Self {
        CParam::Bool(value)
    }
}

impl From<Vec3> for CParam {
    fn from(value: Vec3) -> Self {
        CParam::Vector3(value)
    }
}

impl From<Vec4> for CParam {
    fn from(value: Vec4) -> Self {
        CParam::Vector4(value)
    }
}

impl From<i32> for CParam {
    fn from(value: i32) -> Self {
        CParam::Int32(value)
    }
}

impl From<String> for CParam {
    fn from(value: String) -> Self {
        CParam::String(value)
    }
}

impl From<&str> for CParam {
    fn from(value: &str) -> Self {
        CParam::String(value.to_owned())
    }
}

impl From<Uuid> for CParam {
    fn from(value: Uuid) -> Self {
        CParam::CGuid(value)
    }
}

impl TryInto<Vec3> for CParam {
    type Error = CParamError;

    fn try_into(self) -> Result<Vec3, Self::Error> {
        match self {
            CParam::Vector3(val) => Ok(val),
            _ => Err(CParamError(()))
        }
    }
}

impl TryInto<Vec4> for CParam {
    type Error = CParamError;

    fn try_into(self) -> Result<Vec4, Self::Error> {
        match self {
            CParam::Vector4(val) => Ok(val),
            _ => Err(CParamError(()))
        }
    }
}