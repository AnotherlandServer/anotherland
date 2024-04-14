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

use std::{any::Any, fmt::Debug, io};

use bitstream_io::ByteWrite;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, json};
use bevy_ecs::prelude::*;

use crate::{ClassId, DynParamClass, MightIncludeParams, MightIncludeParamsMut, ParamClass, ParamError, ParamSetBox};

#[derive(Component)]
pub struct ParamBox {
    pub(crate) class_id: ClassId,
    pub(crate) class: Box<dyn DynParamClass>,
}

impl ParamBox {
    pub(crate) fn new(class_id: ClassId, class: Box<dyn DynParamClass>) -> Self {
        Self { class_id, class }
    }

    pub fn class_id(&self) -> ClassId { self.class_id }

    pub fn is<T>(&self) -> bool where T: ParamClass {
        self.class.is::<T>()
    }

    pub fn get<T>(&self) -> Result<&T, ParamError> where T: ParamClass {
        self.class.get()
    }

    pub fn get_mut<T>(&mut self) -> Result<&mut T, ParamError> where T: ParamClass {
        self.class.get_mut()
    }

    pub fn take<T>(self) -> Result<T, ParamError> where T: ParamClass {
        let class: Box<dyn Any> = self.class;
        class.downcast().map_err(|_| ParamError(())).map(|v| *v)
    }

    pub fn as_persistent_json(&self) -> Value {
        let serialized = self.class.as_json();

        json!({ self.class_id.to_string(): serialized })
    }

    pub fn write<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class.write(value)
    }

    pub fn write_to_client<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class.write_to_client(value)
    }

    pub fn get_impl<'a, T: ?Sized + 'static>(&'a self) -> Option<&'a T> 
        where dyn DynParamClass: MightIncludeParams<'a, T> 
    { 
        self.class.as_ref().as_params() 
    }

    pub fn get_impl_mut<'a, T: ?Sized + 'static>(&'a mut self) -> Option<&'a mut T> 
        where dyn DynParamClass: MightIncludeParamsMut<'a, T> 
    {
        self.class.as_mut().as_params_mut() 
    }

    pub fn diff(&self, other: &ParamBox) -> ParamSetBox {
        self.class.diff(other.class.as_ref())
    }
}

impl Serialize for ParamBox {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let json = self.as_persistent_json();
        json.serialize(s)
    }
}

impl <'de>Deserialize<'de> for ParamBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;
        Ok(ParamBox::from_json(&json).unwrap())
    }
}

impl Clone for ParamBox {
    fn clone(&self) -> Self {
        Self { 
            class_id: self.class_id, 
            class: self.class.cloned(),
        }
    }
}

impl Debug for ParamBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.class.as_hash_map().fmt(f)
    }
}