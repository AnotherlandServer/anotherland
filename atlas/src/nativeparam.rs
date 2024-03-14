// Copyright (C) 2024 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::vec::IntoIter;

use glam::f32::Vec3;
use log::error;
use nom::{IResult, error::{VerboseError, context}, number::complete::{le_u8, le_f32, le_f64, le_i32, le_u64, le_u32, le_i64}, multi::count, bytes::complete::take, combinator::{map, fail}};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};

use crate::{ParamError, AvatarId, Uuid};

use super::parsers::parse_pkt_cstring;

#[derive(Debug, Clone, Default)]
pub enum NativeParam {
    #[default]
    Invalid,
    Byte(u8), // 1
    Float(f32), // 2
    Double(f64), // 3
    Int(i32), // 4
    String(String), // 5
    Struct(Vec<NativeParam>), // 6
    Guid(Uuid), // 7
    AvatarId(u64), // 8
    Vector3(Vec3), // 9
    Bool(bool), // 10
    JsonValue(String), // 11
    IntArray(Vec<i32>), // 12
    LongLong(i64), // 13
    Buffer(Vec<u8>), // 14
    UInt(u32), // 15
    GuidArray(Vec<Uuid>), // 16
    StringArray(Vec<String>), // 17
}

impl NativeParam {
    pub fn from_bytes(data: &[u8]) -> IResult<&[u8], NativeParam, VerboseError<&[u8]>> {
        context("nativeparam", |data| {
            let (data, r#type) = context("type", le_u8)(data)?;

            match r#type {
                1 => {
                    let (data, val) = context("byte", le_u8)(data)?;
                    Ok((data, NativeParam::Byte(val)))
                },
                2 => {
                    let (data, val) = context("float", le_f32)(data)?;
                    Ok((data, NativeParam::Float(val)))
                },
                3 => {
                    let (data, val) = context("double", le_f64)(data)?;
                    Ok((data, NativeParam::Double(val)))
                },
                4 => {
                    let (data, val) = context("int", le_i32)(data)?;
                    Ok((data, NativeParam::Int(val)))
                },
                5 => {
                    let (data, val) = context("string", parse_pkt_cstring)(data)?;
                    Ok((data, NativeParam::String(val)))
                },
                6 => {
                    let (data, val) = context("struct", Self::parse_struct)(data)?;
                    Ok((data, val))
                },
                7 => {
                    let (data, bytes) = context("uuid", take(16usize))(data)?;
                    Ok((data, NativeParam::Guid(uuid::Uuid::from_bytes_le(bytes.try_into().unwrap()).into())))
                },
                8 => {
                    let (data, val) = le_u64(data)?;
                    Ok((data, NativeParam::AvatarId(val)))
                },
                9 => {
                    let (data, val) = context("vec3", count(le_f32, 3))(data)?;
                    Ok((data, NativeParam::Vector3(Vec3::new(val[0], val[1], val[2]))))
                },
                10 => {
                    let (data, val) = context("bool", le_u8)(data)?;
                    Ok((data, NativeParam::Bool(val != 0)))
                },
                11 => {
                    let (data, val) = context("json", parse_pkt_cstring)(data)?;
                    Ok((data, NativeParam::JsonValue(val)))
                },
                12 => {
                    context("int_array", |data| {
                        let (data, len) = le_u32(data)?;
                        let (data, val) = count(le_i32, len as usize)(data)?;

                        Ok((data, NativeParam::IntArray(val)))
                    })(data)
                },
                13 => {
                    let (data, val) = context("longlong", le_i64)(data)?;
                    Ok((data, NativeParam::LongLong(val)))
                },
                14 => {
                    context("buffer", |data| {
                        let (data, len) = le_u32(data)?;
                        let (data, val) = count(le_u8, len as usize)(data)?;
                        Ok((data, NativeParam::Buffer(val)))
                    })(data)
                },
                15 => {
                    let (data, val) = context("uint", le_u32)(data)?;
                    Ok((data, NativeParam::UInt(val)))
                },
                16 => {
                    context("uuid_array", |data| {
                        let (data, len) = le_u32(data)?;
                        let (data, val) = count(
                            map(
                                take(16usize), 
                                    |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                                ), 
                            len as usize)(data)?;
                        Ok((data, NativeParam::GuidArray(val)))
                    })(data)
                },
                17 => {
                    context("string_array", |data| {
                        let (data, len) = le_u32(data)?;
                        let (data, val) = count(parse_pkt_cstring, len as usize)(data)?;
                        Ok((data, NativeParam::StringArray(val)))
                    })(data)
                },
                _ => {
                    error!("Unknown native param type {}", r#type);
                    context("unknown type", fail)(data)
                }
                //_ => panic!,
            }
        })(data)
    }

    pub fn parse_struct(data: &[u8]) -> IResult<&[u8], NativeParam, VerboseError<&[u8]>> {
        let (data, field_count) = le_u8(data)?;
        let (data, params) = count(Self::from_bytes, field_count as usize)(data)?;

        Ok((data, NativeParam::Struct(params)))
    }

    pub fn to_struct_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        match self {
            Self::Struct(val) => {
                let _ = writer.write(val.len() as u8);
                for val in val {
                    let _ = writer.write_bytes(&val.to_bytes());
                }
            },
            Self::Invalid => {
                let _ = writer.write(0u8);
            }
            _ => panic!(),
        }

        buf
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        match self {
            Self::Invalid => {},
            Self::Byte(val) => {
                let _ = writer.write(1u8);
                let _ = writer.write(*val);
            },
            Self::Float(val) => {
                let _ = writer.write(2u8);
                let _ = writer.write_bytes(val.to_le_bytes().as_slice());
            },
            Self::Double(val) => {
                let _ = writer.write(3u8);
                let _ = writer.write_bytes(val.to_le_bytes().as_slice());
            },
            Self::Int(val) => {
                let _ = writer.write(4u8);
                let _ = writer.write(*val);
            },
            Self::String(val) => {
                let _ = writer.write(5u8);
                let _ = writer.write(val.len() as u16);
                let _ = writer.write_bytes(val.as_bytes());
            },
            Self::Struct(val) => {
                let _ = writer.write(6u8);
                let _ = writer.write(val.len() as u8);
                for val in val {
                    let _ = writer.write_bytes(&val.to_bytes());
                }
            },
            Self::Guid(val) => {
                let _ = writer.write(7u8);
                let _ = writer.write_bytes(&val.to_uuid_1().to_bytes_le());
            },
            Self::AvatarId(val) => {
                let _ = writer.write(8u8);
                let _ = writer.write(*val);
            },
            Self::Vector3(val) => {
                let _ = writer.write(9u8);
                let _ = writer.write_bytes(val.x.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.y.to_le_bytes().as_slice());
                let _ = writer.write_bytes(val.z.to_le_bytes().as_slice());
            },
            Self::Bool(val) => {
                let _ = writer.write(10u8);
                let _ = writer.write(if *val { 1u8 } else { 0u8 });
            },
            Self::JsonValue(val) => {
                let _ = writer.write(11u8);
                let _ = writer.write(val.len() as u16);
                let _ = writer.write_bytes(val.as_bytes());
            },
            Self::IntArray(val) => {
                let _ = writer.write(12u8);
                let _ = writer.write(val.len() as u32);
                for val in val {
                    let _ = writer.write(*val);
                }
            },
            Self::LongLong(val) => {
                let _ = writer.write(13u8);
                let _ = writer.write(*val);
            },
            Self::Buffer(val) => {
                let _ = writer.write(14u8);
                let _ = writer.write(val.len() as u32);
                let _ = writer.write_bytes(val);
            },
            Self::UInt(val) => {
                let _ = writer.write(15u8);
                let _ = writer.write(*val);
            },
            Self::GuidArray(val) => {
                let _ = writer.write(16u8);
                let _ = writer.write(val.len() as u16);
                for val in val {
                    let _ = writer.write_bytes(&val.to_uuid_1().to_bytes_le());
                }
            },
            Self::StringArray(val) => {
                let _ = writer.write(17u8);
                let _ = writer.write(val.len() as u16);
                for val in val {
                    let _ = writer.write(val.len() as u16);
                    let _ = writer.write_bytes(val.as_bytes());
                }
            },
        }

        buf
    }

    pub fn to_f32(&self) -> Result<f32, ParamError> {
        match self {
            NativeParam::Float(val) => Ok(val.to_owned()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_i32(&self) -> Result<i32, ParamError> {
        match self {
            NativeParam::Int(val) => Ok(val.to_owned()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_string(&self) -> Result<String, ParamError> {
        match self {
            NativeParam::String(val) => Ok(val.to_owned()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_uuid(&self) -> Result<Uuid, ParamError> {
        match self {
            NativeParam::Guid(val) => Ok(val.to_owned()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_struct_iter(self) -> Result<IntoIter<NativeParam>, ParamError> {
        match self {
            NativeParam::Struct(var) => Ok(var.into_iter()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_avatar_id(self) -> Result<AvatarId, ParamError> {
        match self {
            NativeParam::AvatarId(val) => Ok(val.into()),
            _ => Err(ParamError(())),
        }
    }

    pub fn to_bool(self) -> Result<bool, ParamError> {
        match self {
            NativeParam::Bool(val) => Ok(val),
            _ => Err(ParamError(())),
        }
    }
}


