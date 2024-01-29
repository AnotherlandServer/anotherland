// Copyright (C) 2023 AnotherlandServer
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

use bitstream_io::ByteWrite;
use glam::{Vec4, Vec3, Quat};
use nom::{IResult, error::{VerboseError, context}, number::{self, complete::le_i32}, bytes::complete::take, combinator::{fail, map}, multi::{count, self}};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{serialize::{serialize_string, deserialize_string, serialize_json, deserialize_json, serialize_vec_uuid, deserialize_vec_uuid, serialize_i32, deserialize_i32}, AvatarId, Uuid, ParamFlag, ParamError};


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "v")]
pub enum Param {
    None,
    #[serde(
        serialize_with = "serialize_string", 
        deserialize_with = "deserialize_string"
    )]
    String(String, Option<u8>), // 1, 22, 23, 24, 25, 26, 43
    Int64(i64), // 19
    Bool(bool), // 7
    AvatarId(AvatarId), // 16
    Uuid(Uuid), // 5
    LocalizedString(Uuid), // 15
    Class(u32, Vec<u8>), // 41
    Positionable(Vec4, Vec3), // 21
    Vector3(Vec3), // 13
    Vector3Uts(u32, Vec3), // 13
    Vector4(Vec4), // 14
    FloatPair((f32, f32)), // 12
    IntArray4((i32, i32, i32, i32)), // 9

    #[serde(
        serialize_with = "serialize_json", 
        deserialize_with = "deserialize_json"
    )]
    JsonValue(
        Value, 
        Option<String>
    ), // 18
    Quarternion(Quat), // 20
    Bitset(u32), // 10
    AvatarIdSet(HashSet<AvatarId>), // 35
    GuidSet(HashSet<Uuid>), // 37
    StringSet(HashSet<String>), // 4
    Int64Array(Vec<i64>), // 30
    AvatarIdArray(Vec<AvatarId>), // 36

    #[serde(
        serialize_with = "serialize_vec_uuid", 
        deserialize_with = "deserialize_vec_uuid"
    )]
    GuidArray(
        Vec<Uuid>, 
        Option<u8>
    ), // 38, 42
    StringArray(Vec<String>), // 32
    FloatArray(Vec<f32>), // 31
    IntArray(Vec<i32>), // 29
    StringMap(HashMap<String, String>), // 40
    IntMap(HashMap<String, u32>), // 39
    Float(f32), // 11

    #[serde(
        serialize_with = "serialize_i32", 
        deserialize_with = "deserialize_i32"
    )]
    Int32(i32, Option<u8>), // 8, 17
    GuidPair((Uuid, Uuid)), // 6
    StringPair((String, String)), // 2
    StringFloatPair((String, f32)), // 3
}

impl Param {
    pub fn read<'a>(i: &'a [u8], flags: &[ParamFlag]) -> IResult<&'a [u8], Param, VerboseError<&'a [u8]>> {
        let (i, type_id) = number::complete::le_u8(i)?;
        match type_id & 0x7F {
            1 | 22 | 23 | 24 | 25 | 26 | 43 => {
                context("String", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;
    
                    if let Ok(string) = String::from_utf8(bytes.to_vec()) {
                        Ok((i, Param::String(string, Some(type_id))))
                    } else {
                        println!("Failed to parse string: {:#?}", bytes);
                        fail(i)
                    }
                })(i)
            }
            5 => {
                context("Uuid", |i| {
                    let (i, uuid) = map(
                        take(16usize), 
                        |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                    )(i)?;

                    Ok((i, Param::Uuid(uuid)))
                })(i)
            },
            7 => {
                context("Bool", |i| {
                    let (i, val) = number::complete::le_u8(i)?;
                    Ok((i, Param::Bool(val != 0)))
                })(i)
            },
            8 | 17 => {
                context("Int32", |i| {
                    let (i, val) = number::complete::le_i32(i)?;
                    Ok((i, Param::Int32(val, Some(type_id))))
                })(i)
            },
            11 => {
                context("Float", |i| {
                    let (i, val) = number::complete::le_f32(i)?;
                    Ok((i, Param::Float(val)))
                })(i)
            },
            12 => {
                context("FloatPair", |i| {
                    let (i, val) = count(number::complete::le_f32, 2usize)(i)?;
                    Ok((i, Param::FloatPair((val[0], val[1]))))
                })(i)
            },
            13 => {
                if flags.contains(&ParamFlag::Uts) {
                    context("Vector3Uts", |i| {
                        let (i, uts) = number::complete::le_u32(i)?;
                        let (i, val) = count(number::complete::le_f32, 3usize)(i)?;
                        Ok((i, Param::Vector3Uts(uts, Vec3::new(val[0], val[1], val[2]))))
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
            18 => {
                context("JsonValue", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u16(i)?;
                    let (i, bytes) = take(len as usize)(i)?;

                    if bytes.is_empty() {
                        Ok((i, Param::JsonValue(Value::Null, Some("".to_owned()))))
                    } else if let Ok(json) = serde_json::from_slice(bytes) {
                        Ok((i, Param::JsonValue(json, Some(String::from_utf8_lossy(bytes).to_string()))))
                    } else {
                        // Try to fixup the json, as serde_json seems to be more strict than whatever parser otherland 
                        // is using. Or they just ignored those errors...
                        let fixed_json = String::from_utf8_lossy(bytes)
                            .replace("},]", "}]")
                            .replace("\":.", "\":0.")
                            .replace("},,{", "},{")
                            .replace("\"QuestID\":,\"QuestTag\":,", "\"QuestID\":null,\"QuestTag\":null,");

                        if let Ok(json) = serde_json::from_str(&fixed_json) {
                            Ok((i, Param::JsonValue(json, Some(String::from_utf8_lossy(bytes).to_string()))))
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
            29 => {
                context("IntArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(le_i32, count as usize)(i)?;
                    Ok((i, Param::IntArray(data)))
                })(i)
            }
            32 => {
                context("StringArray", |i| {
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
                    Ok((i, Param::StringArray(data)))
                })(i)
            },
            38 | 42 => {
                context("GuidArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(
                        map(
                            take(16usize), 
                            |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into()
                        ), count as usize)(i)?;

                    Ok((i, Param::GuidArray(data, Some(type_id))))
                })(i)
            },
            41 => {
                context("Class", |i: &'a [u8]| {
                    let (i, len) = number::complete::le_u32(i)?;
                    let (i, class) = number::complete::le_u32(i)?;
                    let (i, data) = take(len as usize)(i)?;

                    Ok((i, Param::Class(class, data.to_vec())))
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
            Self::String(val, orig_type) => {
                match orig_type {
                    Some(type_id) => writer.write(*type_id)?,
                    None => writer.write(1u8)?,
                }
                writer.write(val.len() as u16)?;
                writer.write_bytes(val.as_bytes())?;
            },
            Self::Uuid(val) => {
                writer.write(5u8)?;
                writer.write_bytes(val.to_uuid_1().to_bytes_le().as_slice())?;
            },
            Self::Bool(val) => {
                writer.write(7u8)?;
                writer.write(if *val { 1u8 } else { 0u8 })?;
            },
            Self::Int32(val, orig_type) => {
                match orig_type {
                    Some(type_id) => writer.write(*type_id)?,
                    None => writer.write(8u8)?,
                }
                writer.write(*val)?;
            },
            Self::Float(val) => {
                writer.write(11u8)?;
                writer.write_bytes(val.to_le_bytes().as_slice())?;
            },
            Self::FloatPair(val) => {
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
            Self::Vector3Uts(uts, val) => {
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
            Self::JsonValue(val, orig) => {
                let json = if orig.is_some() {
                    orig.as_ref().unwrap().to_owned()
                 } else {
                    serde_json::to_string(val).unwrap()
                 };

                writer.write(18u8)?;
                writer.write(json.len() as u16)?;
                writer.write_bytes(json.as_bytes())?;
            },
            Self::Int64(val) => {
                writer.write(19u8)?;
                writer.write(*val)?;
            },
            Self::IntArray(val) => {
                writer.write(29u8)?;
                writer.write(val.len() as u32)?;
                for &i in val {
                    writer.write(i)?;
                }
            },
            Self::StringArray(val) => {
                writer.write(32u8)?;
                writer.write(val.len() as u32)?;
                for s in val {
                    let bytes = s.as_bytes();

                    writer.write(bytes.len() as u16)?;
                    writer.write_bytes(bytes)?;
                }
            }
            Self::GuidArray(val, orig_type) => {
                match orig_type {
                    Some(type_id) => writer.write(*type_id)?,
                    None => writer.write(38u8)?,
                }
                writer.write(val.len() as u32)?;
                for i in val.iter() {
                    writer.write_bytes(&i.to_uuid_1().to_bytes_le())?;
                }
            },
            Self::Class(class, val) => {
                writer.write(41u8)?;
                writer.write(*class)?;
                writer.write(val.len() as u32)?;
                writer.write_bytes(val)?;
            },
            _ => todo!(),
        }

        Ok(())
    }

    pub(super) fn should_skip(&self) -> bool {
        match self {
            Self::Int64Array(val) => val.is_empty(),
            Self::AvatarIdArray(val) => val.is_empty(),
            Self::StringArray(val) => val.is_empty(),
            Self::FloatArray(val) => val.is_empty(),
            Self::IntArray(val) => val.is_empty(),
            _ => false,
        }
    }

    /// Original data is usefol to test serializing/deserializing of params. 
    /// Normally we'd strip those infos for general use.
    pub fn strip_original_data(self) -> Self {
        match self {
            Self::String(val, _) => Self::String(val, None),
            Self::JsonValue(val, _) => Self::JsonValue(val, None),
            Self::GuidArray(val, _) => Self::GuidArray(val, None),
            Self::Int32(val, _) => Self::Int32(val, None),
            _ => self,
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

impl From<Vec4> for Param {
    fn from(value: Vec4) -> Self {
        Param::Vector4(value)
    }
}

impl From<i32> for Param {
    fn from(value: i32) -> Self {
        Param::Int32(value, None)
    }
}

impl From<&i32> for Param {
    fn from(value: &i32) -> Self {
        Param::Int32(value.to_owned(), None)
    }
}

impl From<String> for Param {
    fn from(value: String) -> Self {
        Param::String(value, None)
    }
}

impl From<&str> for Param {
    fn from(value: &str) -> Self {
        Param::String(value.to_owned(), None)
    }
}

impl From<Uuid> for Param {
    fn from(value: Uuid) -> Self {
        Param::Uuid(value)
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
        Param::GuidArray(value, None)
    }
}

impl From<Vec<i32>> for Param {
    fn from(value: Vec<i32>) -> Self {
        Param::IntArray(value)
    }
}

impl From<Vec<AvatarId>> for Param {
    fn from(value: Vec<AvatarId>) -> Self {
        Param::AvatarIdArray(value)
    }
}

impl From<Vec<String>> for Param {
    fn from(value: Vec<String>) -> Self {
        Param::StringArray(value)
    }
}

impl From<HashMap<String, i32>> for Param {
    fn from(_value: HashMap<String, i32>) -> Self {
        todo!()
    }
}

impl From<HashMap<String, String>> for Param {
    fn from(value: HashMap<String, String>) -> Self {
        Param::StringMap(value)
    }
}

impl From<Value> for Param {
    fn from(value: Value) -> Self {
        Param::JsonValue(value, None)
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
        Param::FloatArray(value)
    }
}

impl From<Vec<i64>> for Param {
    fn from(value: Vec<i64>) -> Self {
        Param::Int64Array(value)
    }
}

impl From<(Uuid, Uuid)> for Param {
    fn from(value: (Uuid, Uuid)) -> Self {
        Param::GuidPair(value)
    }
}

impl From<u32> for Param {
    fn from(value: u32) -> Self {
        Param::Bitset(value)
    }
}

impl From<(f32, f32)> for Param {
    fn from(value: (f32, f32)) -> Self {
        Param::FloatPair(value)
    }
}

impl <'a>TryInto<&'a Vec3> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec3, Self::Error> {
        match self {
            Param::Vector3(val) => Ok(val),
            Param::Vector3Uts(_, val) => Ok(val),
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
            Param::Uuid(val) => Ok(val),
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
            Param::JsonValue(val, _) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [AvatarId]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [AvatarId], Self::Error> {
        match self {
            Param::AvatarIdArray(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [i32]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [i32], Self::Error> {
        match self {
            Param::IntArray(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a str> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self {
            Param::String(val, _) => Ok(val.as_str()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [String]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [String], Self::Error> {
        match self {
            Param::StringArray(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a i32> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a i32, Self::Error> {
        match self {
            Param::Int32(val, _) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<i32> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Param::Int32(val, _) => Ok(val.to_owned()),
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
            Param::GuidArray(val, _) => Ok(val.as_slice()),
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
            Param::Bitset(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl TryInto<u32> for &Param {
    type Error = ParamError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Param::Bitset(val) => Ok(*val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [i64]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [i64], Self::Error> {
        match self {
            Param::Int64Array(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a (f32, f32)> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a (f32, f32), Self::Error> {
        match self {
            Param::FloatPair(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a [f32]> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a [f32], Self::Error> {
        match self {
            Param::FloatArray(val) => Ok(val.as_slice()),
            _ => Err(ParamError(()))
        }
    }
}
