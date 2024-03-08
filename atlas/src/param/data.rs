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

use std::{collections::{HashSet, HashMap}, io};

use bitstream_io::{ByteWrite, Primitive};
use glam::{Vec4, Vec3, Quat};
use nom::{bytes::complete::take, combinator::{fail, map}, error::{context, VerboseError}, multi::{self, count}, number::{self, complete::{le_f32, le_i32, le_i64}}, IResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{serialize::{serialize_string, deserialize_string, serialize_json, deserialize_json, serialize_vec_uuid, deserialize_vec_uuid, serialize_i32, deserialize_i32}, AvatarId, Uuid, ParamFlag, ParamError};


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "v")]
pub enum Param {
    None,
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
    JsonValue(Value), // 18
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
    Any(u32, Vec<u8>), // 41
    VectorLocalizedString(Vec<Uuid>), // 42
    InstanceGroup(String), // 43
}

impl Param {
    pub fn read<'a>(i: &'a [u8], flags: &[ParamFlag]) -> IResult<&'a [u8], Param, VerboseError<&'a [u8]>> {
        let (i, type_id) = number::complete::le_u8(i)?;
        match type_id & 0x7F {
            1 => {
                context("String", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Param::String(string)))
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
                        Ok((i, Param::StringPair((string_a, string_b))))
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
                        Ok((i, Param::StringFloatPair((string, val))))
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
                    Ok((i, Param::StringSet(data.into_iter().collect())))
                })(i)
            },
            5 => {
                context("Guid", |i| {
                    let (i, uuid) = map(
                        take(16usize), 
                        |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                    )(i)?;

                    Ok((i, Param::Guid(uuid)))
                })(i)
            },
            6 => {
                context("GuidPair", |i| {
                    let (i, uuid_a) = map(
                        take(16usize), 
                        |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                    )(i)?;

                    let (i, uuid_b) = map(
                        take(16usize), 
                        |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                    )(i)?;

                    Ok((i, Param::GuidPair((uuid_a, uuid_b))))
                })(i)
            },
            7 => {
                context("Bool", |i| {
                    let (i, val) = number::complete::le_u8(i)?;
                    Ok((i, Param::Bool(val != 0)))
                })(i)
            },
            8 => {
                context("Int", |i| {
                    let (i, val) = number::complete::le_i32(i)?;
                    Ok((i, Param::Int(val)))
                })(i)
            },
            9 => {
                context("BitField128", |i| {
                    let (i, val) = number::complete::le_u128(i)?;
                    Ok((i, Param::BitField128(val)))
                })(i)
            },
            10 => {
                context("BitSetFilter", |i| {
                    let (i, val) = number::complete::le_u32(i)?;
                    Ok((i, Param::BitSetFilter(val)))
                })(i)
            },
            11 => {
                context("Float", |i| {
                    let (i, val) = number::complete::le_f32(i)?;
                    Ok((i, Param::Float(val)))
                })(i)
            },
            12 => {
                context("FloatRange", |i| {
                    let (i, val) = count(number::complete::le_f32, 2usize)(i)?;
                    Ok((i, Param::FloatRange((val[0], val[1]))))
                })(i)
            },
            13 => {
                if flags.contains(&ParamFlag::Uts) {
                    context("Vector3Uts", |i| {
                        let (i, uts) = number::complete::le_u32(i)?;
                        let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                        Ok((i, Param::Vector3Uts((uts, Vec3::new(val[0], val[1], val[2])))))
                    })(i)
                } else {
                    context("Vector3", |i| {
                        let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                        Ok((i, Param::Vector3(Vec3::new(val[0], val[1], val[2]))))
                    })(i)
                }
            },
            14 => {
                context("Vector4", |i| {
                    let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                    Ok((i, Param::Vector4(Vec4::new(val[0], val[1], val[2], val[3]))))
                })(i)
            },
            15 => {
                context("LocalizedString", |i| {
                    let (i, uuid) = map(
                        take(16usize), 
                        |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                    )(i)?;

                    Ok((i, Param::LocalizedString(uuid)))
                })(i)
            },
            16 => {
                context("AvatarID", |i| {
                    let (i, id) = number::complete::le_u64(i)?;

                    Ok((i, Param::AvatarId(id.into())))
                })(i)
            },
            17 => {
                context("UniqueId", |i| {
                    let (i, id) = number::complete::le_i32(i)?;

                    Ok((i, Param::UniqueId(id)))
                })(i)
            },
            18 => {
                context("JsonValue", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;

                    if bytes.is_empty() {
                        Ok((i, Param::JsonValue(Value::Null)))
                    } else if let Ok(json) = serde_json::from_slice(bytes) {
                        Ok((i, Param::JsonValue(json)))
                    } else {
                        // Try to fixup the json, as serde_json seems to be more strict than whatever parser otherland 
                        // is using. Or they just ignored those errors...
                        let fixed_json = String::from_utf8_lossy(bytes)
                            .replace("},]", "}]")
                            .replace("\":.", "\":0.")
                            .replace("},,{", "},{")
                            .replace("\"QuestID\":,\"QuestTag\":,", "\"QuestID\":null,\"QuestTag\":null,");

                        if let Ok(json) = serde_json::from_str(&fixed_json) {
                            Ok((i, Param::JsonValue(json)))
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
                    Ok((i, Param::Int64(val)))
                })(i)
            },
            20 => {
                // todo: validate the binary layout
                context("Quarternion", |i| {
                    let (i, val) = count(number::complete::le_f32, 4usize)(i)?;
                    Ok((i, Param::Quarternion(Quat::from_xyzw(val[0], val[1], val[2], val[3]))))
                })(i)
            },
            21 => {
                // todo: validate the binary layout
                context("Positionable", |i| {
                    let (i, quat_val) = count(number::complete::le_f32, 4usize)(i)?;
                    let (i, vec_val) = count(number::complete::le_f32, 3usize)(i)?;
                    Ok((i, Param::Positionable(
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
                        Ok((i, Param::ContentRef(string)))
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
                        Ok((i, Param::ContentRefAndInt(string)))
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
                        Ok((i, Param::ContentRefAndFloat(string)))
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
                        Ok((i, Param::ContentRefList(string)))
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
                        Ok((i, Param::ClassRefPowerRangeList(string)))
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
                    Ok((i, Param::VectorInt(data)))
                })(i)
            },
            30 => {
                context("VectorInt64", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_i64, count as usize)(i)?;
                    Ok((i, Param::VectorInt64(data)))
                })(i)
            },
            31 => {
                context("VectorFloat", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_f32, count as usize)(i)?;
                    Ok((i, Param::VectorFloat(data)))
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
                    Ok((i, Param::VectorString(data)))
                })(i)
            },
            37 => {
                context("GuidSet", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                        ), count as usize)(i)?;

                    Ok((i, Param::GuidSet(data.into_iter().collect())))
                })(i)
            },
            38 => {
                context("VectorGuid", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                        ), count as usize)(i)?;

                    Ok((i, Param::VectorGuid(data)))
                })(i)
            },
            41 => {
                context("Any", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u32(i)?;
                    let (i, class) = number::complete::le_u32(i)?;
                    let (i, data) = take(len as usize)(i)?;

                    Ok((i, Param::Any(class, data.to_vec())))
                })(i)
            },
            42 => {
                context("VectorLocalizedString", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                        ), count as usize)(i)?;

                    Ok((i, Param::VectorLocalizedString(data)))
                })(i)
            },
            43 => {
                context("InstanceGroup", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Param::InstanceGroup(string)))
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
            Self::Any(class, val) => {
                writer.write(41u8)?;
                writer.write(*class)?;
                writer.write(val.len() as u32)?;
                writer.write_bytes(val)?;
            },
            Self::InstanceGroup(val) => {
                writer.write(43u8)?;
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            _ => todo!(),
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

    pub fn is_set(&self) -> bool {
        !matches!(self, Param::None)
    }
}

impl From<bool> for Param {
    fn from(value: bool) -> Self {
        Param::Bool(value)
    }
}

impl From<Vec3> for Param {
    fn from(value: Vec3) -> Self {
        Param::Vector3(value)
    }
}

impl From<(u32, Vec3)> for Param {
    fn from(val: (u32, Vec3)) -> Self {
        Param::Vector3Uts(val)
    }
}

impl From<Vec4> for Param {
    fn from(value: Vec4) -> Self {
        Param::Vector4(value)
    }
}

impl From<i32> for Param {
    fn from(value: i32) -> Self {
        Param::Int(value)
    }
}

impl From<&i32> for Param {
    fn from(value: &i32) -> Self {
        Param::Int(value.to_owned())
    }
}

impl From<String> for Param {
    fn from(value: String) -> Self {
        Param::String(value)
    }
}

impl From<&str> for Param {
    fn from(value: &str) -> Self {
        Param::String(value.to_owned())
    }
}

impl From<Uuid> for Param {
    fn from(value: Uuid) -> Self {
        Param::Guid(value)
    }
}

impl From<f32> for Param {
    fn from(value: f32) -> Self {
        Param::Float(value)
    }
}

impl From<i64> for Param {
    fn from(value: i64) -> Self {
        Param::Int64(value)
    }
}

impl From<Vec<Uuid>> for Param {
    fn from(value: Vec<Uuid>) -> Self {
        Param::VectorGuid(value)
    }
}

impl From<Vec<i32>> for Param {
    fn from(value: Vec<i32>) -> Self {
        Param::VectorInt(value)
    }
}

impl From<Vec<AvatarId>> for Param {
    fn from(value: Vec<AvatarId>) -> Self {
        Param::VectorAvatarId(value)
    }
}

impl From<Vec<String>> for Param {
    fn from(value: Vec<String>) -> Self {
        Param::VectorString(value)
    }
}

impl From<HashMap<String, i32>> for Param {
    fn from(_value: HashMap<String, i32>) -> Self {
        todo!()
    }
}

impl From<HashMap<String, String>> for Param {
    fn from(value: HashMap<String, String>) -> Self {
        Param::HashmapStringString(value)
    }
}

impl From<Value> for Param {
    fn from(value: Value) -> Self {
        Param::JsonValue(value)
    }
}

impl From<(String, f32)> for Param {
    fn from(value: (String, f32)) -> Self {
        Param::StringFloatPair(value)
    }
}

impl From<AvatarId> for Param {
    fn from(value: AvatarId) -> Self {
        Param::AvatarId(value)
    }
}

impl From<HashSet<AvatarId>> for Param {
    fn from(value: HashSet<AvatarId>) -> Self {
        Param::AvatarIdSet(value)
    }
}

impl From<Vec<f32>> for Param {
    fn from(value: Vec<f32>) -> Self {
        Param::VectorFloat(value)
    }
}

impl From<Vec<i64>> for Param {
    fn from(value: Vec<i64>) -> Self {
        Param::VectorInt64(value)
    }
}

impl From<(Uuid, Uuid)> for Param {
    fn from(value: (Uuid, Uuid)) -> Self {
        Param::GuidPair(value)
    }
}

impl From<u32> for Param {
    fn from(value: u32) -> Self {
        Param::BitSetFilter(value)
    }
}

impl From<(f32, f32)> for Param {
    fn from(value: (f32, f32)) -> Self {
        Param::FloatRange(value)
    }
}

impl <'a>TryInto<&'a Vec3> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec3, Self::Error> {
        match self {
            Param::Vector3(val) => Ok(val),
            Param::Vector3Uts((_, val)) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a (u32, Vec3)> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a (u32, Vec3), Self::Error> {
        match self {
            Param::Vector3Uts(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Vec4> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec4, Self::Error> {
        match self {
            Param::Vector4(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Uuid> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Uuid, Self::Error> {
        match self {
            Param::Guid(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}


impl <'a>TryInto<&'a f32> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a f32, Self::Error> {
        match self {
            Param::Float(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<f32> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Param::Float(val) => Ok(*val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a bool> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a bool, Self::Error> {
        match self {
            Param::Bool(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<bool> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Param::Bool(val) => Ok(*val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a AvatarId> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a AvatarId, Self::Error> {
        match self {
            Param::AvatarId(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<AvatarId> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<AvatarId, Self::Error> {
        match self {
            Param::AvatarId(val) => Ok(val.to_owned()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Value> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Value, Self::Error> {
        match self {
            Param::JsonValue(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [AvatarId]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [AvatarId], Self::Error> {
        match self {
            Param::VectorAvatarId(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [i32]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [i32], Self::Error> {
        match self {
            Param::VectorInt(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a str> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self {
            Param::String(val) => Ok(val.as_str()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [String]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [String], Self::Error> {
        match self {
            Param::VectorString(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a i32> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a i32, Self::Error> {
        match self {
            Param::Int(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<i32> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Param::Int(val) => Ok(val.to_owned()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a i64> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a i64, Self::Error> {
        match self {
            Param::Int64(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<i64> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Param::Int64(val) => Ok(val.to_owned()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a HashMap<String, i32>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a HashMap<String, i32>, Self::Error> {
        todo!()
    }
}

impl <'a>TryInto<&'a HashMap<String, String>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a HashMap<String, String>, Self::Error> {
        todo!()
    }
}

impl <'a>TryInto<&'a (Uuid, Uuid)> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a  (Uuid, Uuid), Self::Error> {
        todo!()
    }
}

impl <'a>TryInto<&'a [Uuid]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a  [Uuid], Self::Error> {
        match self {
            Param::VectorGuid(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a HashSet<AvatarId>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a  HashSet<AvatarId>, Self::Error> {
        match self {
            Param::AvatarIdSet(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a (String, f32)> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a (String, f32), Self::Error> {
        match self {
            Param::StringFloatPair(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a u32> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a u32, Self::Error> {
        match self {
            Param::BitSetFilter(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<u32> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Param::BitSetFilter(val) => Ok(*val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [i64]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [i64], Self::Error> {
        match self {
            Param::VectorInt64(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a (f32, f32)> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a (f32, f32), Self::Error> {
        match self {
            Param::FloatRange(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [f32]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [f32], Self::Error> {
        match self {
            Param::VectorFloat(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}
