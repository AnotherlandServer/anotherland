use core::fmt;
use std::any;
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::marker::PhantomData;

use bitstream_io::ByteWrite;
use glam::Quat;
use legion::Entity;
use legion::World;
use log::debug;
use log::trace;
use nom::IResult;
use nom::bytes::complete::take;
use nom::error::VerboseError;
use nom::error::VerboseErrorKind;
use nom::error::context;
use nom::multi;
use nom::multi::count;
use nom::number;
use nom::combinator::fail;
use nom::number::complete::le_i32;
use nom::sequence::tuple;
use serde::Deserializer;
use serde::Serializer;
use serde::de;
use serde::de::DeserializeOwned;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde_hex::{SerHex, StrictPfx};

use glam::f32::{Vec3, Vec4};
use nom::number::complete::le_u32;
use serde::Deserialize;
use serde::Serialize;
use serde::ser::SerializeSeq;
use serde_json::Map;
use serde_json::Value;
use crate::avatarid::AvatarId;

use super::generated::*;
use super::serialize::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamFlag {
    NodeOwn,
    ServerOwn,
    ClientOwn,
    ClientUnknown,
    ClientPrivileged,
    ClientInit,
    Persistent,
    ExcludeFromClient,
    Content,
    PerInstanceSetting,
    DupeSetOk,
    Deprecated,
    Metric,
    EquipSlot,
    Uts,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParamError(pub ());

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("param error")
    }
}

impl Error for ParamError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait ParamEntity: BoundParamClass {
    type EntityType;
    type ParamClassType;

    fn to_entity(self) -> Self::EntityType;
    fn from_component(world: &World, entity: Entity) -> Result<Self::ParamClassType, ParamError>;
}

pub trait BoundParamClass: ParamClass + Default {
    const CLASS_ID: ParamClassId;

    fn class_id() -> ParamClassId { Self::CLASS_ID }
    fn attribute_name(id: u16) -> &'static str;
    fn lookup_field(name: &str) -> Option<u16>;

    fn from_anyclass(anyclass: AnyClass) -> Self;

    fn into_persistent_json(&self) -> Value {
        let mut attribute_map = HashMap::<&'static str, Value>::new();

        for (name, param) in &self.as_anyclass().0 {
            if Self::attribute_has_flag_static(name, &ParamFlag::Persistent) {
                //let name = Self::attribute_name(a.0);
                let attrib = serde_json::to_value(&param).unwrap();
                attribute_map.insert(name, attrib);
            }
        }

        serde_json::to_value(attribute_map).unwrap()
    }

    fn from_json(value: &Value) -> Result<Self, io::Error> {
        if !value.is_object() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "expected object"));
        }

        let obj = value.as_object().unwrap();
        let mut anyclass = Self::default().to_anyclass();

        for (name, value) in obj {
            match Self::lookup_field(name.as_str()) {
                Some(id) => {
                    let param = serde_json::from_value(value.clone())?;
                    anyclass.0.insert(Self::attribute_name(id).to_owned(), param);
                },
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown attribute"));
                }
            }
        }

        Ok(Self::from_anyclass(anyclass))
    }

    fn read<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
        context(std::any::type_name::<Self>(), |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, anyclass) = AnyClass::raw_read::<Self>(i)?;
            Ok((i, Self::from_anyclass(anyclass)))
        })(i)
    }

    fn write<T>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite
    {
        let anyclass = self.as_anyclass();
        anyclass.raw_write::<_, Self>(writer)
    }

    fn write_to_client<T>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite
    {
        let anyclass = self.as_anyclass();
        anyclass.raw_write_to_client(self, writer)
    }
}

pub trait ParamClass: Sized {
    fn apply(&mut self, other: Self) {
        let anyclass = self.as_anyclass_mut();
        anyclass.apply(other.to_anyclass());
    }

    fn as_anyclass(&self) -> &AnyClass;
    fn as_anyclass_mut(&mut self) -> &mut AnyClass;
    fn to_anyclass(self) -> AnyClass;

    fn attribute_flags(&self, attribute: &str) -> &'static [ParamFlag] {
        Self::attribute_flags_static(attribute)
    }

    fn attribute_flags_static(attribute: &str) -> &'static [ParamFlag];

    fn attribute_has_flag_static(attribute: &str, flag: &ParamFlag) -> bool {
        Self::attribute_flags_static(attribute).contains(flag)
    }

    fn attribute_has_flag(&self, attribute: &str, flag: &ParamFlag) -> bool {
        self.attribute_flags(attribute).contains(flag)
    }
}


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
                    let (i, uuid) = Uuid::from_bytes(i)?;

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
                    let (i, uuid) = Uuid::from_bytes(i)?;

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
                    } else {
                        if let Ok(json) = serde_json::from_slice(bytes) {
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
                    Ok((i, Param::StringArray(data)))
                })(i)
            },
            38 | 42 => {
                context("GuidArray", |i| {
                    let (i, count) = number::complete::le_u32(i)?;
                    let (i, data) = multi::count(Uuid::from_bytes, count as usize)(i)?;

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
                writer.write_bytes(val.to_bytes().as_slice())?;
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
                writer.write_bytes(val.to_bytes().as_slice())?;
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
                    writer.write_bytes(&i.to_bytes())?;
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

    fn should_skip(&self) -> bool {
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
        match self {
            Param::None => false,
            _ => true,
        }
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
    fn from(value: HashMap<String, i32>) -> Self {
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

impl <'a>TryInto<&'a bool> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a bool, Self::Error> {
        match self {
            Param::Bool(val) => Ok(val),
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

impl <'a>TryInto<&'a Value> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Value, Self::Error> {
        match self {
            Param::JsonValue(val, _) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Vec<AvatarId>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec<AvatarId>, Self::Error> {
        match self {
            Param::AvatarIdArray(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Vec<i32>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec<i32>, Self::Error> {
        match self {
            Param::IntArray(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

impl <'a>TryInto<&'a Vec<String>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec<String>, Self::Error> {
        match self {
            Param::StringArray(val) => Ok(val),
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

impl <'a>TryInto<&'a Vec<Uuid>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a  Vec<Uuid>, Self::Error> {
        match self {
            Param::GuidArray(val, _) => Ok(val),
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

impl <'a>TryInto<&'a Vec<i64>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec<i64>, Self::Error> {
        match self {
            Param::Int64Array(val) => Ok(val),
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

impl <'a>TryInto<&'a Vec<f32>> for &'a Param {
    type Error = ParamError;

    fn try_into(self) -> Result<&'a Vec<f32>, Self::Error> {
        match self {
            Param::FloatArray(val) => Ok(val),
            _ => Err(ParamError(()))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ClassAttrib(u16, Param);

impl ClassAttrib {
    pub fn read<'a, T>(i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> 
        where T: BoundParamClass
    {
        let (i, attribute_id) = context("Attribute Id", number::complete::le_u16)(i)?;
        let attribute_name = T::attribute_name(attribute_id);
        let (i, param) = Param::read(i, T::attribute_flags_static(attribute_name))?;
        Ok((i, Self(attribute_id, param)))
    }

    pub fn write<T, C>(&self, writer: &mut T) -> Result<(), io::Error> 
        where 
            T: ByteWrite,
            C: BoundParamClass
    {
        writer.write(self.0)?;
        self.1.write(writer)?;

        Ok(())
    }

    pub fn id(&self) -> u16 { self.0 }
    pub fn get(&self) -> &Param { &self.1 }
    pub fn set(&mut self, param: Param) { self.1 = param; }
    pub fn take(self) -> Param { self.1 }

    pub fn is_set(&self) -> bool { self.1.is_set() }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct AnyClass(HashMap<String, Param>);

impl AnyClass {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    fn raw_read<'a, T>(i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> 
        where T: BoundParamClass
    {
        context("AnyClass", |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let (i, attribs) = context("Attributes", multi::count(ClassAttrib::read::<T>, count as usize))(i)?;

            Ok((i, Self(attribs
                .into_iter()
                .map(|a| (T::attribute_name(a.0).to_owned(), a.1))
                .collect())
            ))
        })(i)
    }

    fn raw_write<T, C>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite,
        C: BoundParamClass
    {
        writer.write(1u8)?;

        let mut filtered_params: Vec<_> = self.0.iter().filter(|(_, a)| !a.should_skip())
            .map(|(name, param)| {
                (C::lookup_field(name).unwrap(), name, param)
            })
            .collect();
        writer.write(filtered_params.len() as u16)?;

        filtered_params.sort_by(|(_, a, _), (_, b, _)| {
            let cmp_a = a.to_lowercase();
            let cmp_b = b.to_lowercase();

            if cmp_a == cmp_b { 
                Ordering::Equal 
            } else if cmp_a < cmp_b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        
        for (id, name, a) in filtered_params {
            writer.write(id)?;
            a.write(writer)?;
        }

        Ok(())
    }

    fn raw_write_to_client<T, C>(&self, outer: &C, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite,
        C: BoundParamClass
    {
        let mut filtered_params: Vec<_> = self.0.iter()
        .filter(|(name, a)| !a.should_skip() && !C::attribute_has_flag_static(name, &ParamFlag::ExcludeFromClient))
        .map(|(name, param)| {
            (C::lookup_field(name).unwrap(), name, param)
        })
        .collect();
    
        filtered_params.sort_by(|(_, a, _), (_, b, _)| {
            let cmp_a = a.to_lowercase();
            let cmp_b = b.to_lowercase();

            if cmp_a == cmp_b { 
                Ordering::Equal 
            } else if cmp_a < cmp_b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        writer.write(1u8)?;
        writer.write(filtered_params.len() as u16)?;

        for (id, name, a) in filtered_params {
            writer.write(id)?;
            a.write(writer)?;
        }

        Ok(())
    }

    pub(crate) fn get_param(&self, name: &str) -> Option<&Param> {
        /*for a in &self.0 {
            if a.0 == id { return Some(&a.1); }
        }

        None*/
        self.0.get(name).map(|a| a)
    }

    pub(crate) fn set_param(&mut self, name: &str, param: Param) {
        self.0.insert(name.to_owned(), param);
    }

    pub fn strip_original_data(&mut self) {
        self.0 = self.0.drain().into_iter().map(|mut a| {
            a.1 = a.1.strip_original_data();
            a
        }).collect();
    }

    pub fn as_hashmap(&self) -> &HashMap<String, Param> { &self.0 }
}

impl ParamClass for AnyClass {
    fn apply(&mut self, other: Self) {
        for (name, param) in other {
            /*match self.0.iter_mut().find(|a| a.0 == o.0) {
                Some(attrib) => attrib.set(o.take()),
                None => self.0.push(o),
            }*/

            self.0.insert(name, param);
        }
    }

    fn as_anyclass(&self) -> &AnyClass {
        self
    }

    fn as_anyclass_mut(&mut self) -> &mut AnyClass {
        self
    }

    fn to_anyclass(self) -> AnyClass { self }

    fn attribute_flags_static(name: &str) -> &'static [ParamFlag] {
        &[]
    }
}

impl IntoIterator for AnyClass {
    type Item = (String, Param);
    type IntoIter = std::collections::hash_map::IntoIter<String, Param>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/*#[cfg(test)]
mod tests {
    use std::{io, path::Path, env, collections::HashSet};
    use bitstream_io::{ByteWriter, LittleEndian};
    use nom::{number, multi, IResult, error::VerboseError};
    use test_case::test_case;

    use crate::{param::{AnyClass, ParamClass}, ParamClassContainer, Param, ClassAttrib};

    fn read_ordered_params<'a>(i: &'a [u8]) ->  IResult<&'a [u8], Vec<ClassAttrib>, VerboseError<&'a [u8]>> {
        let (i, _) = number::complete::le_u8(i)?;
        let (i, count) = number::complete::le_u16(i)?;
        let (i, attribs) = multi::count(ClassAttrib::read, count as usize)(i)?;

        Ok((i, attribs))
    }

    fn test_content(client_path: &Path, table: &str) -> io::Result<()> {
        let db = sqlite::open(
            client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/content.db")
        ).unwrap();
    
        let result = db
            .prepare(format!("SELECT * FROM {}", table))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());
    
        // dump data
        for row in result {
            let original_data = row.read::<&[u8], _>("data");
            let guid = row.read::<&str,_>("guid");
            let name: String = row.read::<&str,_>("name").chars().into_iter().filter(|c| c.is_ascii_graphic()).collect();
            let class_id = row.read::<i64,_>("ixClass") as u16;

            println!("Testing {} - {}", guid.to_string(), name);

            let (_, class) = ParamClassContainer::read(class_id, original_data).expect("Parse failed");
            let mut serialized_data = Vec::new();
            let mut writer = ByteWriter::endian(&mut serialized_data, LittleEndian);
            class.write(&mut writer)?;

            assert_eq!(serialized_data, original_data);
        }
    
        Ok(())
    }
    
    #[test_case("NoBinding")]
    #[test_case("buffs")]
    #[test_case("drops")]
    #[test_case("enemies")]
    #[test_case("factions")]
    #[test_case("items")]
    #[test_case("metagame")]
    #[test_case("misc")]
    #[test_case("npcs")]
    #[test_case("projectiles")]
    #[test_case("quests")]
    #[test_case("recipes")]
    #[test_case("skills")]
    #[test_case("spawners")]
    #[test_case("structures")]
    fn item_param_test(table: &str) -> io::Result<()>{ 
        let client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
        let client_path = Path::new(&client_env);

        test_content(client_path, table)?;

        Ok(())
    }
}*/
