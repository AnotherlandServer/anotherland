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

use std::{hash::Hash, collections::HashMap, io, str::FromStr, any::Any};
use std::fmt::Debug;

use bevy_ecs::system::EntityCommands;
use bitstream_io::ByteWrite;
use nom::{IResult, error::VerboseError, error::context};
use serde_json::Value;
use bevy_ecs::prelude::*;

use crate::{ClassId, MightIncludeBase, Param, ParamBox, ParamFlag, ParamSet, ParamSetBox, ParamType};

pub trait ParamAttrib: PartialEq + Eq + Hash + Copy + Clone + FromStr + TryFrom<u16> + Any + Send + Sync
{
    fn class_id() -> ClassId;

    fn id(&self) -> u16;
    fn name(&self) -> &'static str;
    fn datatype(&self) -> ParamType;
    fn default(&self) -> &Param;
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
                    let param = serde_json::from_value::<Param>(value.clone())?;
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
    type EntityBundle: Bundle;

    fn from_set(set: ParamSet<Self::Attributes>) -> Self;
    fn as_set(&self) -> &ParamSet<Self::Attributes>;
    fn as_mut(&mut self) -> &mut ParamSet<Self::Attributes>;
    fn into_set(self) -> ParamSet<Self::Attributes>;

    fn apply(&mut self, set: ParamSet<Self::Attributes>);
    fn diff(&self, other: &Self) -> ParamSet<Self::Attributes> {
        self.as_set().diff(other.as_set())
    }

    fn as_persistent_json(&self) -> Value {
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

    fn into_bundle(self) -> Self::EntityBundle;
    fn into_box(self) -> ParamBox;
}

pub trait DynParamClass: Any + Send + Sync + for<'a> MightIncludeBase<'a> {
    fn class_id(&self) -> ClassId;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn diff(&self, other: &dyn DynParamClass) -> ParamSetBox;
    fn cloned(&self) -> Box<dyn DynParamClass>;
    fn as_json(&self) -> Value;

    fn apply(&mut self, set: ParamSetBox);

    fn as_hash_map(&self) -> HashMap<String, Param>;

    fn set_param(&mut self, name: &str, param: Param) -> Option<Param>;
    fn get_param(&self, name: &str) -> Option<&Param>;

    fn build_entity<'a>(&self, cmds: &'a mut Commands) -> EntityCommands<'a>;
}

// blanked implementations for param classes
impl <T: ParamClass + Clone + Any + Send + Sync + for<'a> MightIncludeBase<'a>> DynParamClass for T {
    fn class_id(&self) -> ClassId {
        T::Attributes::class_id()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn diff(&self, other: &dyn DynParamClass) -> ParamSetBox {
        if self.class_id() != other.class_id() {
            panic!("tried to diff mismatching param boxes")
        }

        self
            .as_set()
            .diff(
                other.get::<T>()
                .unwrap()
                .as_set()
            )
            .into_box()
    }

    fn cloned(&self) -> Box<dyn DynParamClass> {
        Box::new(self.clone())
    }

    fn as_json(&self) -> Value {
        self.as_persistent_json()
    }

    fn apply(&mut self, set: ParamSetBox) {
        if self.class_id() != set.class_id {
            panic!("tried to diff mismatching param boxes")
        }

        self
            .apply(set.take::<T::Attributes>().unwrap());
    }

    fn as_hash_map(&self) -> HashMap<String, Param> {
        self.as_set().as_hash_map()
    }

    fn set_param(&mut self, name: &str, param: Param) -> Option<Param> {
        self.as_mut().set_param(name, param)
    }

    fn get_param(&self, name: &str) -> Option<&Param> {
        self.as_set().get_param(name)
    }

    fn build_entity<'a>(&self, cmds: &'a mut Commands) -> EntityCommands<'a> {
        cmds.spawn(self.clone().into_bundle())
    }
}

pub trait DynParamSet: Debug + Any + Send + Sync {
    fn class_id(&self) -> ClassId;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_hash_map(&self) -> HashMap<String, Param>;

    fn set_param(&mut self, name: &str, param: Param) -> Option<Param>;
    fn get_param(&self, name: &str) -> Option<&Param>;
}

pub trait MightIncludeParams<'a, T: ?Sized + 'static> {
    fn as_params(&'a self) -> Option<&'a T>;
}

pub trait MightIncludeParamsMut<'a, T: ?Sized + 'static> {
    fn as_params_mut(&'a mut self) -> Option<&'a mut T>;
}