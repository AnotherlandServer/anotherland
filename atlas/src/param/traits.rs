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

use std::{hash::Hash, cell::Ref, collections::HashMap, io, str::FromStr, any::Any};

use bitstream_io::ByteWrite;
use log::debug;
use nom::{IResult, error::VerboseError, error::context};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use parking_lot::RwLockReadGuard;

use crate::{ParamType, Param, ParamFlag, ParamSet};

pub trait ParamAttrib: PartialEq + Eq + Hash + Clone + FromStr + TryFrom<u16>
{
    fn class_id() -> u16;

    fn id(&self) -> u16;
    fn name(&self) -> &'static str;
    fn datatype(&self) -> ParamType;
    fn default(&self) -> Option<Param>;
    fn flags(&self) -> &[ParamFlag];

    fn has_flag(&self, flag: &ParamFlag) -> bool {
        self.flags().contains(flag)
    }

    fn deserialize_json_set(value: &Value) -> Result<ParamSet<Self>, io::Error> {
        if !value.is_object() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "expected object"));
        }

        let obj = value.as_object().unwrap();
        let mut set = ParamSet::<Self>::new();

        for (name, value) in obj {
            match Self::from_str(name) {
                Ok(attribute) => {
                    let param = serde_json::from_value(value.clone())?;
                    set.insert(attribute, param);
                },
                Err(_) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown attribute"));
                }
            }
        }

        Ok(set)
    }

    fn serialize_json_set(set: &ParamSet<Self>) -> Value {
        let attribute_map: HashMap<_,_> = set.params.iter()
            .filter(|(k, _)| {
                k.has_flag(&ParamFlag::Persistent)
            })
            .map(|(k, v)| {
                (k.name(), serde_json::to_value(v).unwrap())
            })
            .collect();

        serde_json::to_value(attribute_map).unwrap()
    }
}

pub trait ParamClass: Default + Any {
    type Attributes: ParamAttrib;

    fn new() -> Self;

    fn from_set(set: ParamSet<Self::Attributes>) -> Self;
    fn as_set<'a>(&'a self) -> RwLockReadGuard<'a, ParamSet<Self::Attributes>>;
    fn into_set(self) -> ParamSet<Self::Attributes>;

    fn apply(&mut self, set: ParamSet<Self::Attributes>);

    fn into_persistent_json(&self) -> Value {
        Self::Attributes::serialize_json_set(&self.as_set())
    }

    fn from_json(value: &Value) -> Result<Self, io::Error> {
        let mut instance = Self::default();
        instance.apply(Self::Attributes::deserialize_json_set(value)?);

        Ok(instance)
    }

    fn read<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
        context(std::any::type_name::<Self>(), |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, set) = ParamSet::read(i)?;
            Ok((i, Self::from_set(set)))
        })(i)
    }

    fn write<T>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite
    {
        self.as_set().write(writer)
    }

    fn write_to_client<T>(&self, writer: &mut T) -> Result<(), io::Error> 
        where T: ByteWrite
    {
        self.as_set().write_to_client(writer)
    }

    fn clone_ref(&self) -> Self;
}
