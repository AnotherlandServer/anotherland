// Copyright (C) 2025 AnotherlandServer
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

use base64::prelude::*;
use std::{collections::{HashMap, HashSet}, io};

use bitstream_io::ByteWrite;
use glam::{Quat, Vec3, Vec4};
use nom::{bytes::complete::take, combinator::{fail, map}, error::{context, VerboseError}, multi::{self, count}, number::{self, complete::{le_f32, le_i32, le_i64}}, IResult};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use toolkit::types::{AvatarId, Uuid};

use crate::{ParamError, ParamFlag, ParamType};

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String), // 1
    StringPair((String, String)), // 2
    StringFloatPair((String, f32)), // 3
    StringSet(HashSet<String>), // 4
    Guid(Uuid), // 5
    GuidPair((Uuid, Uuid)), // 6
    Bool(bool), // 7
    Int(i32), // 8
    BitField128(u128), // 9
    BitSetFilter(u32), // 10
    Float(f32), // 11
    FloatRange((f32, f32)), // 12
    Vector3(Vec3), // 13
    Vector3Uts((u32, Vec3)), // 13
    Vector4(Vec4), // 14
    LocalizedString(Uuid), // 15
    AvatarId(AvatarId), // 16
    UniqueId(i32), // 17
    JsonValue(JsonValue), // 18
    Int64(i64), // 19
    Quarternion(Quat), // 20
    Positionable(Quat, Vec3), // 21
    ContentRef(String), // 22
    ContentRefAndInt(String), // 23
    ContentRefAndFloat(String), // 24
    ContentRefList(String), // 25
    ClassRefPowerRangeList(String), // 26
    VectorInt(Vec<i32>), // 29
    VectorInt64(Vec<i64>), // 30
    VectorFloat(Vec<f32>), // 31
    VectorString(Vec<String>), // 32
    AvatarIdSet(HashSet<AvatarId>), // 35
    VectorAvatarId(Vec<AvatarId>), // 36
    GuidSet(HashSet<Uuid>), // 37
    VectorGuid(Vec<Uuid>), // 38
    HashmapStringInt(HashMap<String, i32>), // 39
    HashmapStringString(HashMap<String, String>), // 40
    Any(Vec<u8>), // 41
    VectorLocalizedString(Vec<Uuid>), // 42
    InstanceGroup(String), // 43
}

impl Value {
    pub fn from_slice<'a>(i: &'a [u8], flags: &[ParamFlag]) -> IResult<&'a [u8], Value, VerboseError<&'a [u8]>> {
        let (i, type_id) = number::complete::le_u8(i)?;
        match type_id & 0x7F {
            1 => {
                context("String", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::String(string)))
                    } else {
                        println!("Failed to parse string: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            2 => {
                // todo: validate the binary layout
                context("StringPair", |i: &'a [u8]| {
                    let (i, len_a) = number::complete::le_u16(i)?;
                    let (i, bytes_a) = take(len_a as usize)(i)?;
    
                    let (i, len_b) = number::complete::le_u16(i)?;
                    let (i, bytes_b) = take(len_b as usize)(i)?;

                    if let Ok(string_a) = String::from_utf8(bytes_a.to_vec()) && 
                        let Ok(string_b) = String::from_utf8(bytes_b.to_vec()) 
                    {
                        Ok((i, Value::StringPair((string_a, string_b))))
                    } else {
                        println!("Failed to parse string: {:#?} / {:#?}", bytes_a, bytes_b);
                        fail(i)
                    }
                })(i)
            },
            3 => {
                // todo: validate the binary layout
                context("StringFloatPair", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    let (i, val) = number::complete::le_f32(i)?;

                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::StringFloatPair((string, val))))
                    } else {
                        println!("Failed to parse string: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            4 => {
                // todo: validate the binary layout
                context("StringSet", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(|i: &'a [u8]| {
                        let (i, len) = number::complete::le_u16(i)?;
                        let (i, bytes) = take(len as usize)(i)?;
        
                        if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                            Ok((i, string))
                        } else {
                            println!("Failed to parse string: {:#?}", bytes);
                            fail(i)
                        }
                    }, count as usize)(i)?;
                    Ok((i, Value::StringSet(data.into_iter().collect())))
                })(i)
            },
            5 => {
                context("Guid", |i| {
                    let (i, uuid) = map(
                        take(16usize), 
                        |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                    )(i)?;

                    Ok((i, Value::Guid(uuid)))
                })(i)
            },
            6 => {
                context("GuidPair", |i| {
                    let (i, uuid_a) = map(
                        take(16usize), 
                        |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                    )(i)?;

                    let (i, uuid_b) = map(
                        take(16usize), 
                        |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                    )(i)?;

                    Ok((i, Value::GuidPair((uuid_a, uuid_b))))
                })(i)
            },
            7 => {
                context("Bool", |i| {
                    let (i, val) = number::complete::le_u8(i)?;
                    Ok((i, Value::Bool(val != 0)))
                })(i)
            },
            8 => {
                context("Int", |i| {
                    let (i, val) = number::complete::le_i32(i)?;
                    Ok((i, Value::Int(val)))
                })(i)
            },
            9 => {
                context("BitField128", |i| {
                    let (i, val) = number::complete::le_u128(i)?;
                    Ok((i, Value::BitField128(val)))
                })(i)
            },
            10 => {
                context("BitSetFilter", |i| {
                    let (i, val) = number::complete::le_u32(i)?;
                    Ok((i, Value::BitSetFilter(val)))
                })(i)
            },
            11 => {
                context("Float", |i| {
                    let (i, val) = number::complete::le_f32(i)?;
                    Ok((i, Value::Float(val)))
                })(i)
            },
            12 => {
                context("FloatRange", |i| {
                    let (i, val) = count(number::complete::le_f32, 2usize)(i)?;
                    Ok((i, Value::FloatRange((val[0], val[1]))))
                })(i)
            },
            13 => {
                if flags.contains(&ParamFlag::Uts) {
                    context("Vector3Uts", |i| {
                        let (i, uts) = number::complete::le_u32(i)?;
                        let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                        Ok((i, Value::Vector3Uts((uts, Vec3::new(val[0], val[1], val[2])))))
                    })(i)
                } else {
                    context("Vector3", |i| {
                        let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                        Ok((i, Value::Vector3(Vec3::new(val[0], val[1], val[2]))))
                    })(i)
                }
            },
            14 => {
                context("Vector4", |i| {
                    let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                    Ok((i, Value::Vector4(Vec4::new(val[0], val[1], val[2], val[3]))))
                })(i)
            },
            15 => {
                context("LocalizedString", |i| {
                    let (i, uuid) = map(
                        take(16usize), 
                        |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                    )(i)?;

                    Ok((i, Value::LocalizedString(uuid)))
                })(i)
            },
            16 => {
                context("AvatarID", |i| {
                    let (i, id) = number::complete::le_u64(i)?;

                    Ok((i, Value::AvatarId(id.into())))
                })(i)
            },
            17 => {
                context("UniqueId", |i| {
                    let (i, id) = number::complete::le_i32(i)?;

                    Ok((i, Value::UniqueId(id)))
                })(i)
            },
            18 => {
                context("JsonValue", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;

                    if bytes.is_empty() {
                        Ok((i, Value::JsonValue(JsonValue::Null)))
                    } else if let Ok(json) = serde_json::from_slice(bytes) {
                        Ok((i, Value::JsonValue(json)))
                    } else {
                        // Try to fixup the json, as serde_json seems to be more strict than whatever parser otherland 
                        // is using. Or they just ignored those errors...
                        let fixed_json = String::from_utf8_lossy(bytes)
                            .replace("},]", "}]")
                            .replace("\":.", "\":0.")
                            .replace("},,{", "},{")
                            .replace("\"QuestID\":,\"QuestTag\":,", "\"QuestID\":null,\"QuestTag\":null,");

                        if let Ok(json) = serde_json::from_str(&fixed_json) {
                            Ok((i, Value::JsonValue(json)))
                        } else {
                            println!("Failed to parse json: \"{}\"", String::from_utf8_lossy(bytes));
                            fail(i)
                        }
                    }
                })(i)
            },
            19 => {
                context("Int64", |i| {
                    let (i, val) = number::complete::le_i64(i)?;
                    Ok((i, Value::Int64(val)))
                })(i)
            },
            20 => {
                // todo: validate the binary layout
                context("Quarternion", |i| {
                    let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                    Ok((i, Value::Quarternion(Quat::from_xyzw(val[0], val[1], val[2], val[3]))))
                })(i)
            },
            21 => {
                // todo: validate the binary layout
                context("Positionable", |i| {
                    let (i, quat_val) = count(number::complete::le_f32, 4usize)(i)?;
                    let (i, vec_val) = count(number::complete::le_f32, 3usize)(i)?;
                    Ok((i, Value::Positionable(
                        Quat::from_xyzw(quat_val[0], quat_val[1], quat_val[2], quat_val[3]), 
                        Vec3::new(vec_val[0], vec_val[1], vec_val[2])
                    )))
                })(i)
            },
            22 => {
                context("ContentRef", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::ContentRef(string)))
                    } else {
                        println!("Failed to parse ContentRef: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            23 => {
                context("ContentRefAndInt", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::ContentRefAndInt(string)))
                    } else {
                        println!("Failed to parse ContentRefAndInt: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            24 => {
                context("ContentRefAndFloat", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::ContentRefAndFloat(string)))
                    } else {
                        println!("Failed to parse ContentRefAndFloat: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            25 => {
                context("ContentRefList", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::ContentRefList(string)))
                    } else {
                        println!("Failed to parse ContentRefList: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            26 => {
                context("ClassRefPowerRangeList", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::ClassRefPowerRangeList(string)))
                    } else {
                        println!("Failed to parse ClassRefPowerRangeList: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            },
            29 => {
                context("VectorInt", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_i32, count as usize)(i)?;
                    Ok((i, Value::VectorInt(data)))
                })(i)
            },
            30 => {
                context("VectorInt64", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_i64, count as usize)(i)?;
                    Ok((i, Value::VectorInt64(data)))
                })(i)
            },
            31 => {
                context("VectorFloat", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_f32, count as usize)(i)?;
                    Ok((i, Value::VectorFloat(data)))
                })(i)
            },
            32 => {
                context("VectorString", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(|i: &'a [u8]| {
                        let (i, len) = number::complete::le_u16(i)?;
                        let (i, bytes) = take(len as usize)(i)?;
        
                        if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                            Ok((i, string))
                        } else {
                            println!("Failed to parse string: {:#?}", bytes);
                            fail(i)
                        }
                    }, count as usize)(i)?;
                    Ok((i, Value::VectorString(data)))
                })(i)
            },
            37 => {
                context("GuidSet", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                        ), count as usize)(i)?;

                    Ok((i, Value::GuidSet(data.into_iter().collect())))
                })(i)
            },
            38 => {
                context("VectorGuid", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                        ), count as usize)(i)?;

                    Ok((i, Value::VectorGuid(data)))
                })(i)
            },
            41 => {
                context("Any", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u32(i)?;
                    let (i, data) = take(len as usize)(i)?;

                    Ok((i, Value::Any(data.to_vec())))
                })(i)
            },
            42 => {
                context("VectorLocalizedString", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| Uuid::from_bytes_le(v.try_into().unwrap())
                        ), count as usize)(i)?;

                    Ok((i, Value::VectorLocalizedString(data)))
                })(i)
            },
            43 => {
                context("InstanceGroup", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Value::InstanceGroup(string)))
                    } else {
                        println!("Failed to parse InstanceGroup: {:#?}", bytes);
                        fail(i)
                    }
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

    pub fn write<T>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite
    {
        match self {
            Self::String(val) => {
                writer.write(1u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::StringPair((val_a, val_b)) => {
                writer.write(2u8)?;
                writer.write(val_a.len() as u16)?;
                writer.write_bytes(val_a.as_bytes())?;
                writer.write(val_b.len() as u16)?;
                writer.write_bytes(val_b.as_bytes())?;
            },
            Self::StringFloatPair((string, val)) => {
                writer.write(3u8)?;
                writer.write(string.len() as u16)?;
                writer.write_bytes(string.as_bytes())?;
                writer.write_bytes(val.to_le_bytes().as_slice())?;
            },
            Self::StringSet(val) => {
                writer.write(4u8)?;
                writer.write(val.len() as u32)?;
                for s in val {
                    let bytes = s.as_bytes();

                    writer.write(bytes.len() as u16)?;
                    writer.write_bytes(bytes)?;
                }
            }
            Self::Guid(val) => {
                writer.write(5u8)?;
                writer.write_bytes(val.to_uuid_1().to_bytes_le().as_slice())?;
            },
            Self::GuidPair((val_a, val_b)) => {
                writer.write(6u8)?;
                writer.write_bytes(val_a.to_uuid_1().to_bytes_le().as_slice())?;
                writer.write_bytes(val_b.to_uuid_1().to_bytes_le().as_slice())?;
            },
            Self::Bool(val) => {
                writer.write(7u8)?;
                writer.write(if *val { 1u8 } else { 0u8 })?;
            },
            Self::Int(val) => {
                writer.write(8u8)?;
                writer.write(*val)?;
            },
            Self::BitField128(val) => {
                writer.write(9u8)?;
                writer.write_bytes(val.to_le_bytes().as_slice())?;
            },
            Self::BitSetFilter(val) => {
                writer.write(10u8)?;
                writer.write_bytes(val.to_le_bytes().as_slice())?;
            },
            Self::Float(val) => {
                writer.write(11u8)?;
                writer.write_bytes(val.to_le_bytes().as_slice())?;
            },
            Self::FloatRange(val) => {
                writer.write(12u8)?;
                writer.write_bytes(val.0.to_le_bytes().as_slice())?;
                writer.write_bytes(val.1.to_le_bytes().as_slice())?;
            },
            Self::Vector3(val) => {
                writer.write(13u8)?;
                writer.write_bytes(val.x.to_le_bytes().as_slice())?;
                writer.write_bytes(val.y.to_le_bytes().as_slice())?;
                writer.write_bytes(val.z.to_le_bytes().as_slice())?;
            },
            Self::Vector3Uts((uts, val)) => {
                writer.write(13u8)?;
                writer.write(*uts)?;
                writer.write_bytes(val.x.to_le_bytes().as_slice())?;
                writer.write_bytes(val.y.to_le_bytes().as_slice())?;
                writer.write_bytes(val.z.to_le_bytes().as_slice())?;
            }
            Self::Vector4(val) => {
                writer.write(14u8)?;
                writer.write_bytes(val.x.to_le_bytes().as_slice())?;
                writer.write_bytes(val.y.to_le_bytes().as_slice())?;
                writer.write_bytes(val.z.to_le_bytes().as_slice())?;
                writer.write_bytes(val.w.to_le_bytes().as_slice())?;
            },
            Self::LocalizedString(val) => {
                writer.write(15u8)?;
                writer.write_bytes(val.to_uuid_1().to_bytes_le().as_slice())?;
            },
            Self::AvatarId(val) => {
                writer.write(16u8)?;
                writer.write(val.as_u64())?;
            },
            Self::UniqueId(val) => {
                writer.write(17u8)?;
                writer.write(*val)?;
            },
            Self::JsonValue(val) => {
                let json = serde_json::to_string(val).unwrap();

                writer.write(18u8)?;
                writer.write(json.len() as u16)?;
                writer.write_bytes(json.as_bytes())?;
            },
            Self::Int64(val) => {
                writer.write(19u8)?;
                writer.write(*val)?;
            },
            Self::Quarternion(val) => {
                writer.write(20u8)?;
                writer.write_bytes(val.x.to_le_bytes().as_slice())?;
                writer.write_bytes(val.y.to_le_bytes().as_slice())?;
                writer.write_bytes(val.z.to_le_bytes().as_slice())?;
                writer.write_bytes(val.w.to_le_bytes().as_slice())?;
            },
            Self::Positionable(rot, pos) => {
                writer.write(21u8)?;
                writer.write_bytes(rot.x.to_le_bytes().as_slice())?;
                writer.write_bytes(rot.y.to_le_bytes().as_slice())?;
                writer.write_bytes(rot.z.to_le_bytes().as_slice())?;
                writer.write_bytes(rot.w.to_le_bytes().as_slice())?;
                writer.write_bytes(pos.x.to_le_bytes().as_slice())?;
                writer.write_bytes(pos.y.to_le_bytes().as_slice())?;
                writer.write_bytes(pos.z.to_le_bytes().as_slice())?;
            },
            Self::ContentRef(val) => {
                writer.write(22u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::ContentRefAndInt(val) => {
                writer.write(23u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::ContentRefAndFloat(val) => {
                writer.write(24u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::ContentRefList(val) => {
                writer.write(25u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::ClassRefPowerRangeList(val) => {
                writer.write(26u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::VectorInt(val) => {
                writer.write(29u8)?;
                writer.write(val.len() as u32)?;
                for &i in val {
                    writer.write(i)?;
                }
            },
            Self::VectorInt64(val) => {
                writer.write(30u8)?;
                writer.write(val.len() as u32)?;
                for &i in val {
                    writer.write(i)?;
                }
            },
            Self::VectorFloat(val) => {
                writer.write(31u8)?;
                writer.write(val.len() as u32)?;
                for &i in val {
                    writer.write_bytes(i.to_le_bytes().as_slice())?;
                }
            },
            Self::VectorString(val) => {
                writer.write(32u8)?;
                writer.write(val.len() as u32)?;
                for s in val {
                    let bytes = s.as_bytes();

                    writer.write(bytes.len() as u16)?;
                    writer.write_bytes(bytes)?;
                }
            }
            Self::VectorGuid(val) => {
                writer.write(38u8)?;
                writer.write(val.len() as u32)?;
                for i in val.iter() {
                    writer.write_bytes(&i.to_uuid_1().to_bytes_le())?;
                }
            },
            Self::Any(val) => {
                writer.write(41u8)?;
                writer.write(val.len() as u32)?;
                writer.write_bytes(val)?;
            },
            Self::InstanceGroup(val) => {
                writer.write(43u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::VectorLocalizedString(val) => {
                writer.write(42u8)?;
                writer.write(val.len() as u32)?;
                for i in val.iter() {
                    writer.write_bytes(&i.to_uuid_1().to_bytes_le())?;
                }
            },
            Self::GuidSet(val) => { // todo: check
                writer.write(37u8)?;
                writer.write(val.len() as u32)?;
                for i in val.iter() {
                    writer.write_bytes(&i.to_uuid_1().to_bytes_le())?;
                }
            },
            Self::AvatarIdSet(val) => { // todo: check
                writer.write(35u8)?;
                writer.write(val.len() as u32)?;
                for v in val.iter() {
                    writer.write(v.as_u64())?;
                }
            },
            Self::VectorAvatarId(val) => { // todo: check
                writer.write(36u8)?;
                writer.write(val.len() as u32)?;
                for v in val.iter() {
                    writer.write(v.as_u64())?;
                }
            }
            _ => todo!("{:?}", self),
        }

        Ok(())
    }

    pub(super) fn should_skip(&self) -> bool {
        match self {
            Self::VectorAvatarId(val) => val.is_empty(),
            Self::VectorString(val) => val.is_empty(),
            Self::VectorFloat(val) => val.is_empty(),
            Self::VectorInt(val) => val.is_empty(),
            Self::VectorInt64(val) => val.is_empty(),
            Self::VectorGuid(val) => val.is_empty(),
            Self::VectorLocalizedString(val) => val.is_empty(),
            _ => false,
        }
    }

    pub fn r#type(&self) -> ParamType {
        match self {
            Value::String(_) => ParamType::String,
            Value::StringPair(_) => ParamType::StringPair,
            Value::StringFloatPair(_) => ParamType::StringFloatPair,
            Value::StringSet(_) => ParamType::StringSet,
            Value::Guid(_) => ParamType::Guid,
            Value::GuidPair(_) => ParamType::GuidPair,
            Value::Bool(_) => ParamType::Bool,
            Value::Int(_) => ParamType::Int,
            Value::BitField128(_) => ParamType::BitField128,
            Value::BitSetFilter(_) => ParamType::BitSetFilter,
            Value::Float(_) => ParamType::Float,
            Value::FloatRange(_) => ParamType::FloatRange,
            Value::Vector3(_) => ParamType::Vector3,
            Value::Vector3Uts(_) => ParamType::Vector3Uts,
            Value::Vector4(_) => ParamType::Vector4,
            Value::LocalizedString(_) => ParamType::LocalizedString,
            Value::AvatarId(_) => ParamType::AvatarId,
            Value::UniqueId(_) => ParamType::UniqueId,
            Value::JsonValue(_) => ParamType::JsonValue,
            Value::Int64(_) => ParamType::Int64,
            Value::Quarternion(_) => ParamType::Quarternion,
            Value::Positionable(_, _) => ParamType::Positionable,
            Value::ContentRef(_) => ParamType::ContentRef,
            Value::ContentRefAndInt(_) => ParamType::ContentRefAndInt,
            Value::ContentRefAndFloat(_) => ParamType::ContentRefAndFloat,
            Value::ContentRefList(_) => ParamType::ContentRefList,
            Value::ClassRefPowerRangeList(_) => ParamType::ClassRefPowerRangeList,
            Value::VectorInt(_) => ParamType::VectorInt,
            Value::VectorInt64(_) => ParamType::VectorInt64,
            Value::VectorFloat(_) => ParamType::VectorFloat,
            Value::VectorString(_) => ParamType::VectorString,
            Value::AvatarIdSet(_) => ParamType::AvatarIdSet,
            Value::VectorAvatarId(_) => ParamType::VectorAvatarId,
            Value::GuidSet(_) => ParamType::GuidSet,
            Value::VectorGuid(_) => ParamType::VectorGuid,
            Value::HashmapStringInt(_) => ParamType::HashmapStringInt,
            Value::HashmapStringString(_) => ParamType::HashmapStringString,
            Value::Any(_) => ParamType::Any,
            Value::VectorLocalizedString(_) => ParamType::VectorLocalizedString,
            Value::InstanceGroup(_) => ParamType::InstanceGroup,
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::StringPair(arg0) => Self::StringPair(arg0.clone()),
            Self::StringFloatPair(arg0) => Self::StringFloatPair(arg0.clone()),
            Self::StringSet(arg0) => Self::StringSet(arg0.clone()),
            Self::Guid(arg0) => Self::Guid(*arg0),
            Self::GuidPair(arg0) => Self::GuidPair(*arg0),
            Self::Bool(arg0) => Self::Bool(*arg0),
            Self::Int(arg0) => Self::Int(*arg0),
            Self::BitField128(arg0) => Self::BitField128(*arg0),
            Self::BitSetFilter(arg0) => Self::BitSetFilter(*arg0),
            Self::Float(arg0) => Self::Float(*arg0),
            Self::FloatRange(arg0) => Self::FloatRange(*arg0),
            Self::Vector3(arg0) => Self::Vector3(*arg0),
            Self::Vector3Uts(arg0) => Self::Vector3Uts(*arg0),
            Self::Vector4(arg0) => Self::Vector4(*arg0),
            Self::LocalizedString(arg0) => Self::LocalizedString(*arg0),
            Self::AvatarId(arg0) => Self::AvatarId(*arg0),
            Self::UniqueId(arg0) => Self::UniqueId(*arg0),
            Self::JsonValue(arg0) => Self::JsonValue(arg0.clone()),
            Self::Int64(arg0) => Self::Int64(*arg0),
            Self::Quarternion(arg0) => Self::Quarternion(*arg0),
            Self::Positionable(arg0, arg1) => Self::Positionable(*arg0, *arg1),
            Self::ContentRef(arg0) => Self::ContentRef(arg0.clone()),
            Self::ContentRefAndInt(arg0) => Self::ContentRefAndInt(arg0.clone()),
            Self::ContentRefAndFloat(arg0) => Self::ContentRefAndFloat(arg0.clone()),
            Self::ContentRefList(arg0) => Self::ContentRefList(arg0.clone()),
            Self::ClassRefPowerRangeList(arg0) => Self::ClassRefPowerRangeList(arg0.clone()),
            Self::VectorInt(arg0) => Self::VectorInt(arg0.clone()),
            Self::VectorInt64(arg0) => Self::VectorInt64(arg0.clone()),
            Self::VectorFloat(arg0) => Self::VectorFloat(arg0.clone()),
            Self::VectorString(arg0) => Self::VectorString(arg0.clone()),
            Self::AvatarIdSet(arg0) => Self::AvatarIdSet(arg0.clone()),
            Self::VectorAvatarId(arg0) => Self::VectorAvatarId(arg0.clone()),
            Self::GuidSet(arg0) => Self::GuidSet(arg0.clone()),
            Self::VectorGuid(arg0) => Self::VectorGuid(arg0.clone()),
            Self::HashmapStringInt(arg0) => Self::HashmapStringInt(arg0.clone()),
            Self::HashmapStringString(arg0) => Self::HashmapStringString(arg0.clone()),
            Self::Any(arg0) => Self::Any(arg0.clone()),
            Self::VectorLocalizedString(arg0) => Self::VectorLocalizedString(arg0.clone()),
            Self::InstanceGroup(arg0) => Self::InstanceGroup(arg0.clone()),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        match self {
            Value::String(val) => val.serialize(serializer),
            Value::StringPair(pair) => pair.serialize(serializer),
            Value::StringFloatPair(pair) => pair.serialize(serializer),
            Value::StringSet(hash_set) => hash_set.serialize(serializer),
            Value::Guid(uuid) => uuid.serialize(serializer),
            Value::GuidPair(pair) => pair.serialize(serializer),
            Value::Bool(val) => val.serialize(serializer),
            Value::Int(val) => val.serialize(serializer),
            Value::BitField128(val) => val.serialize(serializer),
            Value::BitSetFilter(val) => val.serialize(serializer),
            Value::Float(val) => val.serialize(serializer),
            Value::FloatRange(pair) => pair.serialize(serializer),
            Value::Vector3(val) => val.serialize(serializer),
            Value::Vector3Uts(pair) => pair.serialize(serializer),
            Value::Vector4(val) => val.serialize(serializer),
            Value::LocalizedString(uuid) => uuid.serialize(serializer),
            Value::AvatarId(avatar_id) => avatar_id.serialize(serializer),
            Value::UniqueId(val) => val.serialize(serializer),
            Value::JsonValue(val) => val.serialize(serializer),
            Value::Int64(val) => val.serialize(serializer),
            Value::Quarternion(quat) => quat.serialize(serializer),
            Value::Positionable(quat, vec3) => {
                (quat, vec3).serialize(serializer)
            },
            Value::ContentRef(val) => val.serialize(serializer),
            Value::ContentRefAndInt(val) => val.serialize(serializer),
            Value::ContentRefAndFloat(val) => serializer.serialize_str(val),
            Value::ContentRefList(val) => serializer.serialize_str(val),
            Value::ClassRefPowerRangeList(val) => serializer.serialize_str(val),
            Value::VectorInt(vec) => vec.serialize(serializer),
            Value::VectorInt64(vec) => vec.serialize(serializer),
            Value::VectorFloat(vec) => vec.serialize(serializer),
            Value::VectorString(vec) => vec.serialize(serializer),
            Value::AvatarIdSet(hash_set) => hash_set.serialize(serializer),
            Value::VectorAvatarId(vec) => vec.serialize(serializer),
            Value::GuidSet(hash_set) => hash_set.serialize(serializer),
            Value::VectorGuid(vec) => vec.serialize(serializer),
            Value::HashmapStringInt(hash_map) => hash_map.serialize(serializer),
            Value::HashmapStringString(hash_map) => hash_map.serialize(serializer),
            Value::Any(vec) => {
                if serializer.is_human_readable() {
                    BASE64_STANDARD.encode(vec).serialize(serializer)
                } else {
                    vec.serialize(serializer)
                }
            },
            Value::VectorLocalizedString(vec) => vec.serialize(serializer),
            Value::InstanceGroup(val) => val.serialize(serializer),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Uuid> for Value {
    fn from(value: Uuid) -> Self {
        Value::Guid(value)
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Self {
        Value::VectorInt(value)
    }
}

impl From<Vec3> for Value {
    fn from(value: Vec3) -> Self {
        Value::Vector3(value)
    }
}

impl From<(u32, Vec3)> for Value {
    fn from(value: (u32, Vec3)) -> Self {
        Value::Vector3Uts(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Any(value)
    }
}

impl From<AvatarId> for Value {
    fn from(value: AvatarId) -> Self {
        Value::AvatarId(value)
    }
}

impl <'a> TryFrom<&'a Value> for &'a String {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(val) => Ok(val),
            Value::ContentRef(val) => Ok(val),
            Value::ContentRefAndInt(val) => Ok(val),
            Value::ContentRefAndFloat(val) => Ok(val),
            Value::ContentRefList(val) => Ok(val),
            Value::ClassRefPowerRangeList(val) => Ok(val),
            Value::InstanceGroup(val) => Ok(val),
            _ => Err(ParamError::TypeMismatch),
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a (String, String) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::StringPair(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a (String, f32) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::StringFloatPair(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a HashSet<String> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::StringSet(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Uuid {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Guid(val) => Ok(val),
            Value::LocalizedString(val) => Ok(val),
            _ => Err(ParamError::TypeMismatch),
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a (Uuid, Uuid) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::GuidPair(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a bool {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Bool(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a i32 {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(val) => Ok(val),
            Value::UniqueId(val) => Ok(val),
            _ => Err(ParamError::TypeMismatch),
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a f32 {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Float(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a (f32, f32) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::FloatRange(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec3 {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Vector3(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a (u32, Vec3) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Vector3Uts(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec4 {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Vector4(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a AvatarId {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::AvatarId(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a JsonValue {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::JsonValue(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a i64 {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Int64(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Quat {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Quarternion(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for (&'a Quat, &'a Vec3) {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Positionable(rot, pos) = value {
            Ok((rot, pos))
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<i32> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::VectorInt(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<i64> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::VectorInt64(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<f32> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::VectorFloat(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<String> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::VectorString(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a HashSet<AvatarId> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::AvatarIdSet(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<AvatarId> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::VectorAvatarId(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a HashSet<Uuid> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::GuidSet(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a [Uuid] {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::VectorGuid(val) => Ok(val),
            Value::VectorLocalizedString(val) => Ok(val),
            _ => Err(ParamError::TypeMismatch),
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a HashMap<String, i32> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::HashmapStringInt(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a HashMap<String, String> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::HashmapStringString(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

impl <'a> TryFrom<&'a Value> for &'a Vec<u8> {
    type Error = ParamError;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Value::Any(val) = value {
            Ok(val)
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}
