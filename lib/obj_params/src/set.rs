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

use std::{any::type_name, collections::{HashMap, HashSet}, fmt::Debug, io, iter::empty};

use base64::prelude::*;
use bitstream_io::ByteWrite;
use glam::{Quat, Vec3, Vec4};
use log::warn;
use nom::{combinator::fail, error::{context, VerboseError}, multi, number, IResult};
use serde::{de::{self, DeserializeSeed, SeqAccess, Visitor}, ser::{SerializeMap, SerializeStruct}, Deserialize, Serialize};
use toolkit::types::{AvatarId, Uuid};

use crate::{param::{GenericParam, Param}, Attribute, AttributeInfo, Class, ContentRef, ParamError, ParamFlag, ParamType, Value};

pub type AttributeValue = (&'static dyn AttributeInfo, Value);

pub trait GenericParamSet: Debug + Send + Sync {
    fn class(&self) -> Class;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_hash_map(&self) -> HashMap<&'static str, Value>;

    fn set_param(&mut self, name: &str, value: Value) -> Option<Value>;
    fn get_param(&self, name: &str) -> Option<&Value>;
    fn remove_param(&mut self, name: &str) -> Option<Value>;

    fn clear_changes(&mut self);
    fn changes(&self) -> Box<dyn Iterator<Item = AttributeValue>>;
    fn client_params(&self) -> Box<dyn GenericParamSet>;
    fn client_privileged_params(&self) -> Box<dyn GenericParamSet>;

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, &'a Value)> + 'a>;
    fn drain<'a>(&'a mut self) -> Box<dyn Iterator<Item = AttributeValue> + 'a>;

    fn dyn_clone(&self) -> Box<dyn GenericParamSet>;
}

pub trait GenericParamSetBoxExt {
    fn new_for_class(class: Class) -> Box<dyn GenericParamSet>;
}

impl GenericParamSetBoxExt for Box<dyn GenericParamSet> {
    fn new_for_class(class: Class) -> Box<dyn GenericParamSet> {
        class.create_param_set(vec![])
    }
}

pub trait ParamWriter {
    fn write<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error>;
    fn write_to_client<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error>;
    fn write_to_privileged_client<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error>;
}

pub trait ParamReader {
    type Value;

    fn from_slice(class: Class, i: &[u8]) -> IResult<&[u8], Self::Value, VerboseError<&[u8]>>;
    fn from_client_slice(class: Class, i: &[u8]) -> IResult<&[u8], Self::Value, VerboseError<&[u8]>>;
}

#[derive(Clone)]
pub struct ParamSet<T: Attribute> {
    pub(super) values: HashMap<T, Param<T>>,
}

impl <T: Attribute + 'static> Default for ParamSet<T> {
    fn default() -> Self { Self::new() }
}

impl <T: Attribute + 'static> ParamSet <T> {
    pub fn new() -> Self { Self { values: HashMap::new() }}
    pub(crate) fn new_from_attributes(mut attributes: Vec<AttributeValue>) -> Self {
        let mut set = Self::new();
        set.values.reserve(attributes.len());

        for (attr, val) in attributes.drain(..) {
            set.set_param(attr.name(), val);
        }

        set.clear_changes();

        set
    }

    pub fn insert<P>(&mut self, attr: T, value: P) -> Option<Param<T>>
        where P: Into<Value>
    {
        let value = value.into();
        if value.r#type() != attr.datatype() { panic!("expected value type {:?}", attr.datatype()); }
        self.values.insert(attr, Param::new(attr, value))
    }
    
    pub fn remove(&mut self, key: &T) -> Option<Param<T>> {
        self.values.remove(key)
    }

    pub fn get<'a, V>(&'a self, key: &T) -> Option<&'a V> 
        where &'a V: From<&'a Value>
    {
        self.values.get(key).map(|p| p.value().into())
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Param<T>> {
        self.values.get_mut(key)
    }

    pub fn extend(&mut self, other: ParamSet<T>) {
        for (attr, param) in other.values.iter() {
            self.values.insert(
                *attr, 
                Param::new(*attr, param.value().clone())
            );
        }
    }

    pub(crate) fn read_values<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Param<T>>, VerboseError<&'a [u8]>> {
        context(type_name::<T>(), |i| -> IResult<&'a [u8], Vec<Param<T>>, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let (i, values) = context("Attributes", multi::count(Param::from_slice, count as usize))(i)?;

            Ok((i, values))
        })(i)
    }

    pub fn from_slice(i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (i, params) = Self::read_values(i)?;

        Ok((i, Self{
            values: params
            .into_iter()
            .map(|v| (*v.attribute(), v))
            .collect()
        }))
    }

    pub fn from_client_slice(i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (i, params) = Self::read_values(i)?;

        Ok((i, Self{    	
            values: params
            .into_iter()
            .filter(|v| 
                !v.attribute().flags().contains(&ParamFlag::ExcludeFromClient) &&
                !v.attribute().flags().contains(&ParamFlag::ClientUnknown) &&
                !v.attribute().flags().contains(&ParamFlag::NodeOwn) &&
                !v.attribute().flags().contains(&ParamFlag::ServerOwn)
            )
            .map(|v| (*v.attribute(), v))
            .collect()
        }))
    }

    pub fn as_hash_map(&self) -> HashMap<&'static str, Value> {
        self.values.iter()
            .map(|(a, v)| (a.name(), v.value().clone()))
            .collect()
    }

    pub fn changes(&self) -> Vec<Param<T>> {
        self.values.iter()
            .filter(|&(_, p)| p.is_changed())
            .map(|(_, p)| p.clone())
            .collect()
    }
}

impl <T: Attribute + 'static> IntoIterator for ParamSet<T> {
    type Item = AttributeValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
            .into_values()
            .map(|v| (v.attribute().static_info(), v.take()))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl <T: Attribute> Debug for ParamSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("ParamSet");

        for (attrib, param) in self.values.iter() {
            debug_struct.field(attrib.name(), param.value());
        }

        debug_struct.finish()
    }
}

impl <T: Attribute + 'static> GenericParamSet for ParamSet<T> {
    fn class(&self) -> Class {
        <T as Attribute>::class()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn as_hash_map(&self) -> HashMap<&'static str, Value> {
        self.as_hash_map()
    }

    fn set_param(&mut self, name: &str, value: Value) -> Option<Value> {
        if let Ok(attr) = name.parse() {
            self.values.insert(attr, Param::new(attr, value))
                .map(|p| p.take())
        } else {
            None
        }
    }

    fn get_param(&self, name: &str) -> Option<&Value> {
        if let Ok(attr) = name.parse() {
            self.values.get(&attr)
                .map(|p| p.value())
        } else {
            None
        }
    }

    fn remove_param(&mut self, name: &str) -> Option<Value> {
        if let Ok(attr) = name.parse() {
            self.values.remove(&attr)
                .map(|p| p.take())
        } else {
            None
        }
    }
    
    fn clear_changes(&mut self) {
        for p in self.values.values_mut() {
            p.clear_state();
        }
    }

    fn changes(&self) -> Box<dyn Iterator<Item = AttributeValue>> {
        Box::new(
            self.changes()
                .into_iter()
                .map(|p| (p.attribute().static_info(), p.take()))
        )
    }

    fn client_params(&self) -> Box<dyn GenericParamSet> {
        <T as Attribute>::class().create_param_set(
            self.values.values()
                .filter(|&p| 
                    !p.attribute().has_flag(&ParamFlag::ClientUnknown) &&
                    !p.attribute().has_flag(&ParamFlag::ClientPrivileged)
                )
                .map(|p| (p.attribute().static_info(), p.value().clone()))
                .collect()
        )
    }

    fn client_privileged_params(&self) -> Box<dyn GenericParamSet> {
        <T as Attribute>::class().create_param_set(
            self.values.values()
                .filter(|&p| 
                    !p.attribute().has_flag(&ParamFlag::ClientUnknown) ||
                    p.attribute().has_flag(&ParamFlag::ClientPrivileged)
                )
                .map(|p| (p.attribute().static_info(), p.value().clone()))
                .collect()
        )
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, &'a Value)> + 'a> {
        Box::new(
            self.values.values()
                .map(|p| (p.attribute().static_info(), p.value()))
        )
    }

    fn drain<'a>(&'a mut self) -> Box<dyn Iterator<Item = AttributeValue> + 'a> {
        Box::new(
            self.values.drain()
                .map(|(a, p)| (a.static_info(), p.take()))
        )
    }

    fn dyn_clone(&self) -> Box<dyn GenericParamSet> {
        Box::new(self.clone())
    }
}

impl ParamWriter for dyn GenericParamSet {
    fn write<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write(1u8)?;

        let filtered_params: Vec<_> = self.values().filter(|&(_, a)| !a.should_skip())
            .collect();
        writer.write(filtered_params.len() as u16)?;

        
        for (a, v) in filtered_params {
            writer.write(a.id())?;
            v.write(writer)?;
        }

        Ok(())
    }

    fn write_to_client<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error> {
        let filtered_params: Vec<_> = self.values()
        .filter(|&(a, val)| 
            !a.has_flag(&ParamFlag::ClientUnknown) &&
            !a.has_flag(&ParamFlag::ClientPrivileged) &&
            !val.should_skip()
        )
        .collect();

        writer.write(1u8)?;
        writer.write(filtered_params.len() as u16)?;

        for (a, v) in filtered_params {
            writer.write(a.id())?;
            v.write(writer)?;
        }

        Ok(())
    }

    fn write_to_privileged_client<W: ByteWrite>(&self, writer: &mut W) -> Result<(), io::Error> {
        let filtered_params: Vec<_> = self.values()
        .filter(|&(a, val)| 
            (!a.has_flag(&ParamFlag::ClientUnknown) ||
            a.has_flag(&ParamFlag::ClientPrivileged)) &&
            !val.should_skip()
        )
        .collect();

        writer.write(1u8)?;
        writer.write(filtered_params.len() as u16)?;

        for (a, v) in filtered_params {
            writer.write(a.id())?;
            v.write(writer)?;
        }

        Ok(())
    }
}

fn read_attribute(class: Class, i: &[u8]) -> IResult<&[u8], AttributeValue, VerboseError<&[u8]>> {
    let (i, attribute_id) = context("Attribute Id", number::complete::le_u16)(i)?;
    let attribute = match class.get_attribute_from_id(attribute_id) {
        Some(attribute) => attribute,
        None => {
            warn!("failed to parse attribute id {}", attribute_id);
            return fail(i);
        },
    };

    let (i, param) = Value::from_slice(i, attribute.flags())?;
    Ok((i, (attribute, param)))
}

impl ParamReader for Box<dyn GenericParamSet> {
    type Value = Box<dyn GenericParamSet>;

    fn from_slice<'a>(class: Class, i: &'a [u8]) -> IResult<&'a [u8], Self::Value, VerboseError<&'a [u8]>> {
        context("ParamSet", |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let read_attribute_closure = |class| {
                move |i: &'a [u8]| {
                    read_attribute(class, i)
                }
            };
            let (i, attribs) = context("Attributes", multi::count(read_attribute_closure(class), count as usize))(i)?;

            Ok((i, class.create_param_set(attribs)))
        })(i)
    }

    fn from_client_slice<'a>(class: Class, i: &'a [u8]) -> IResult<&'a [u8], Self::Value, VerboseError<&'a [u8]>> {
        context("ParamSet", |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let read_attribute_closure = |class| {
                move |i: &'a [u8]| {
                    read_attribute(class, i)
                }
            };
            let (i, attribs) = context("Attributes", multi::count(read_attribute_closure(class), count as usize))(i)?;

            Ok((i, class.create_param_set(
                attribs.into_iter()
                    .filter(|&(a,_)| 
                        !a.has_flag(&ParamFlag::ExcludeFromClient) &&
                        !a.has_flag(&ParamFlag::ClientUnknown) &&
                        !a.has_flag(&ParamFlag::ServerOwn) &&
                        !a.has_flag(&ParamFlag::NodeOwn)
                    )
                    .collect()
            )))
        })(i)
    }
}

impl Serialize for Box<dyn GenericParamSet + '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {

        let mut state = serializer.serialize_struct("ParamSet", 2)?;

        state.serialize_field("class", &self.class())?;
        state.serialize_field("attributes", &DynSetSerializer(self.as_ref()))?;
        state.end()
    }
}

impl <'de> Deserialize<'de> for Box<dyn GenericParamSet> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        enum Field { Class, Attributes }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de> {
                
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("'class' or 'attributes'")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: de::Error, {
                        match v {
                            "class" => Ok(Field::Class),
                            "attributes" => Ok(Field::Attributes),
                            _ => Err(de::Error::unknown_field(v, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ParamSetVisitor;
        impl <'de> Visitor<'de> for ParamSetVisitor {
            type Value = Box<dyn GenericParamSet>;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ParamSet")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error> 
                where V: SeqAccess<'de>
            {
                let seed = seq.next_element::<DynSetDeserializer>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let set = seq.next_element_seed(seed)?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                
                Ok(set)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>, {

                let mut class = None;
                let mut attributes = None;
                
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Class => {
                            if class.is_some() {
                                return Err(de::Error::duplicate_field("class"));
                            }
                            class = Some(map.next_value::<DynSetDeserializer>()?);
                        },
                        Field::Attributes => {
                            if class.is_none() {
                                return Err(de::Error::missing_field("class"));
                            }

                            if attributes.is_some() {
                                return Err(de::Error::duplicate_field("attributes"));
                            }

                            attributes = Some(map.next_value_seed(class.unwrap())?);
                        },
                    }
                }

                let attributes = attributes.ok_or_else(|| de::Error::missing_field("attributes"))?;
                Ok(attributes)
            }
        }

        const FIELDS: &[&str] = &["class", "attributes"];
        deserializer.deserialize_struct("ParamSet", FIELDS, ParamSetVisitor)
    }
}

struct DynSetSerializer<'a>(&'a dyn GenericParamSet);

impl Serialize for DynSetSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        let mut state = serializer.serialize_map(Some(self.0.len()))?;
        
        for (attr, value) in self.0.values() {
            state.serialize_entry(attr.name(), value)?;
        }

        state.end()
    }
}

#[derive(Clone, Copy)]
struct DynSetDeserializer(Class);

impl <'de> Deserialize<'de> for DynSetDeserializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de> {

        Class::deserialize(deserializer)
            .map(DynSetDeserializer)
    }
}

impl <'de> DeserializeSeed<'de> for DynSetDeserializer {
    type Value = Box<dyn GenericParamSet>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de> {

        struct AttributeMapVisitor(Class);

        impl <'de> Visitor<'de> for AttributeMapVisitor {
            type Value = Vec<AttributeValue>;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("attribute map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>, {

                let mut attributes = Vec::with_capacity(map.size_hint().unwrap_or_default());
                
                while let Some(attribute) = map.next_key_seed(AttributeDeserializer(self.0))? {
                    let value = map.next_value_seed(ValueDeserializer(attribute))?;
                    attributes.push((attribute, value));
                }

                Ok(attributes)
            }
        }

        struct AttributeDeserializer(Class);

        impl <'de> DeserializeSeed<'de> for AttributeDeserializer {
            type Value = &'static dyn AttributeInfo;
        
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de> {

                struct AttributeVisitor(Class);
                impl Visitor<'_> for AttributeVisitor {
                    type Value = &'static dyn AttributeInfo;
                
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("attribute")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: de::Error, {
                        
                        self.0.get_attribute(v)
                            .ok_or_else(|| E::invalid_value(de::Unexpected::Str(v), &"attribute name"))
                    }
                }
                
                deserializer.deserialize_str(AttributeVisitor(self.0))
            }
        }

        struct ValueDeserializer<'a>(&'a dyn AttributeInfo);

        impl <'de> DeserializeSeed<'de> for ValueDeserializer<'_> {
            type Value = Value;
        
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de> {

                match self.0.datatype() {
                    ParamType::String => Ok(
                        Value::String(String::deserialize(deserializer)?)
                    ),
                    ParamType::StringPair => Ok(
                        Value::StringPair(<(String,String)>::deserialize(deserializer)?)
                    ),
                    ParamType::StringFloatPair => Ok(
                        Value::StringFloatPair(<(String,f32)>::deserialize(deserializer)?)
                    ),
                    ParamType::StringSet => Ok(
                        Value::StringSet(HashSet::<String>::deserialize(deserializer)?)
                    ),
                    ParamType::Guid => Ok(
                        Value::Guid(Uuid::deserialize(deserializer)?)
                    ),
                    ParamType::GuidPair => Ok(
                        Value::GuidPair(<(Uuid,Uuid)>::deserialize(deserializer)?)
                    ),
                    ParamType::Bool => Ok(
                        Value::Bool(bool::deserialize(deserializer)?)
                    ),
                    ParamType::Int => Ok(
                        Value::Int(i32::deserialize(deserializer)?)
                    ),
                    ParamType::BitField128 => Ok(
                        Value::BitField128(u128::deserialize(deserializer)?)
                    ),
                    ParamType::BitSetFilter => Ok(
                        Value::BitSetFilter(u32::deserialize(deserializer)?)
                    ),
                    ParamType::Float => Ok(
                        Value::Float(f32::deserialize(deserializer)?)
                    ),
                    ParamType::FloatRange => Ok(
                        Value::FloatRange(<(f32,f32)>::deserialize(deserializer)?)
                    ),
                    ParamType::Vector3 => {
                        if self.0.has_flag(&ParamFlag::Uts) {
                            Ok(Value::Vector3Uts(<(u32, Vec3)>::deserialize(deserializer)?))
                        } else {
                            Ok(Value::Vector3(Vec3::deserialize(deserializer)?))
                        }
                    },
                    ParamType::Vector3Uts => unreachable!(),
                    ParamType::Vector4 => Ok(
                        Value::Vector4(Vec4::deserialize(deserializer)?)
                    ),
                    ParamType::LocalizedString => Ok(
                        Value::LocalizedString(Uuid::deserialize(deserializer)?)
                    ),
                    ParamType::AvatarId => Ok(
                        Value::AvatarId(AvatarId::deserialize(deserializer)?)
                    ),
                    ParamType::UniqueId => Ok(
                        Value::UniqueId(i32::deserialize(deserializer)?)
                    ),
                    ParamType::JsonValue => Ok(
                        Value::JsonValue(serde_json::Value::deserialize(deserializer)?)
                    ),
                    ParamType::Int64 => Ok(
                        Value::Int64(i64::deserialize(deserializer)?)
                    ),
                    ParamType::Quarternion => Ok(
                        Value::Quarternion(Quat::deserialize(deserializer)?)
                    ),
                    ParamType::Positionable =>{
                        let positionable = <(Quat,Vec3)>::deserialize(deserializer)?;

                        Ok(
                            Value::Positionable(
                                positionable.0,
                                positionable.1,
                            )
                        )
                    }
                    ParamType::ContentRef => Ok(
                        Value::ContentRef(
                            String::deserialize(deserializer)?
                                .parse::<ContentRef>()
                                .ok()
                        )
                    ),
                    ParamType::ContentRefAndInt => Ok(
                        Value::ContentRefAndInt(String::deserialize(deserializer)?)
                    ),
                    ParamType::ContentRefAndFloat => Ok(
                        Value::ContentRefAndFloat(String::deserialize(deserializer)?)
                    ),
                    ParamType::ContentRefList => Ok(
                        Value::ContentRefList(
                            String::deserialize(deserializer)?
                                .parse()
                                .map_err(|e: ParamError| serde::de::Error::custom(e))?
                        )
                    ),
                    ParamType::ClassRefPowerRangeList => Ok(
                        Value::ClassRefPowerRangeList(String::deserialize(deserializer)?)
                    ),
                    ParamType::VectorInt => Ok(
                        Value::VectorInt(Vec::<i32>::deserialize(deserializer)?)
                    ),
                    ParamType::VectorInt64 => Ok(
                        Value::VectorInt64(Vec::<i64>::deserialize(deserializer)?)
                    ),
                    ParamType::VectorFloat => Ok(
                        Value::VectorFloat(Vec::<f32>::deserialize(deserializer)?)
                    ),
                    ParamType::VectorString => Ok(
                        Value::VectorString(Vec::<String>::deserialize(deserializer)?)
                    ),
                    ParamType::AvatarIdSet => Ok(
                        Value::AvatarIdSet(HashSet::<AvatarId>::deserialize(deserializer)?)
                    ),
                    ParamType::VectorAvatarId => Ok(
                        Value::VectorAvatarId(Vec::<AvatarId>::deserialize(deserializer)?)
                    ),
                    ParamType::GuidSet => Ok(
                        Value::GuidSet(HashSet::<Uuid>::deserialize(deserializer)?)
                    ),
                    ParamType::VectorGuid => Ok(
                        Value::VectorGuid(Vec::<Uuid>::deserialize(deserializer)?)
                    ),
                    ParamType::HashmapStringInt => Ok(
                        Value::HashmapStringInt(HashMap::<String,i32>::deserialize(deserializer)?)
                    ),
                    ParamType::HashmapStringString => Ok(
                        Value::HashmapStringString(HashMap::<String,String>::deserialize(deserializer)?)
                    ),
                    ParamType::Any => Ok(
                        if deserializer.is_human_readable() {
                            Value::Any(
                                BASE64_STANDARD.decode(String::deserialize(deserializer)?)
                                    .map_err(|e| serde::de::Error::custom(e.to_string()))?
                            )
                        } else {
                            Value::Any(Vec::<u8>::deserialize(deserializer)?)
                        }
                    ),
                    ParamType::VectorLocalizedString => Ok(
                        Value::VectorLocalizedString(Vec::<Uuid>::deserialize(deserializer)?)
                    ),
                    ParamType::InstanceGroup => Ok(
                        Value::InstanceGroup(String::deserialize(deserializer)?)
                    ),
                }
            }
        }

        let attributes = deserializer.deserialize_map(AttributeMapVisitor(self.0))?;
        Ok(self.0.create_param_set(attributes))
    }
}

#[derive(Debug)]
pub struct NullParamSet;

impl GenericParamSet for NullParamSet {
    fn class(&self) -> Class { panic!("null param set doesn't have a class") }
    fn len(&self) -> usize {0 }
    fn is_empty(&self) -> bool { true }
    fn as_hash_map(&self) -> HashMap<&'static str, Value> { HashMap::new() }
    fn set_param(&mut self, _: &str, _: Value) -> Option<Value> { panic!("can't set param on null param set") }
    fn get_param(&self, _: &str) -> Option<&Value> { None }
    fn remove_param(&mut self, _: &str) -> Option<Value> { None }
    fn clear_changes(&mut self) {}
    fn changes(&self) -> Box<dyn Iterator<Item = AttributeValue>> { Box::new(empty()) }
    fn client_params(&self) -> Box<dyn GenericParamSet> { Box::new(NullParamSet) }
    fn client_privileged_params(&self) -> Box<dyn GenericParamSet> { Box::new(NullParamSet) }
    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, &'a Value)> + 'a> { Box::new(empty()) }
    fn drain<'a>(&'a mut self) -> Box<dyn Iterator<Item = AttributeValue> + 'a> { Box::new(empty()) }
    fn dyn_clone(&self) -> Box<dyn GenericParamSet> { Box::new(NullParamSet) }
}

impl FromIterator<AttributeValue> for Box<dyn GenericParamSet> {
    fn from_iter<T: IntoIterator<Item = AttributeValue>>(iter: T) -> Self {
        let attributes = iter.into_iter()
            .collect::<Vec<_>>();

        if let Some(class) = attributes.first()
            .map(|(info, _)| info.class()) {
        
            class.create_param_set(attributes)
        } else {
            Box::new(NullParamSet)
        }
    }
}
