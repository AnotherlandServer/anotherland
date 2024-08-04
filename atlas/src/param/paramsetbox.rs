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

use std::{any::Any, io, ops::Deref};
use std::fmt::Debug;

use bitstream_io::ByteWrite;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, json};

use crate::{ClassId, DynParamSet, Param, ParamAttrib, ParamError, ParamSet};

pub struct ParamSetBox {
    pub(crate) class_id: ClassId,
    pub(crate) set: Box<dyn DynParamSet>,
}

impl ParamSetBox {
    pub(crate) fn new<T>(set: ParamSet<T>) -> Self 
        where T: ParamAttrib + Any + Send + Sync
    {
        Self {
            class_id: T::class_id(),
            set: Box::new(set),
        }
    }

    pub fn is<T>(&self) -> bool where T: ParamAttrib + 'static {
        self.set.is::<T>()
    }

    pub fn get<T>(&self) -> Result<&ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        self.set.get()
    }

    pub fn get_mut<T>(&mut self) -> Result<&mut ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        self.set.get_mut()
    }

    pub fn take<T>(self) -> Result<ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        let set: Box<dyn Any> = self.set;
        set.downcast().map_err(|_| ParamError(())).map(|v| *v)
    }

    pub fn as_persistent_json(&self) -> Value {
        json!({ self.class_id.to_string(): self.set.as_json() })
    }

    pub fn write<T>(&self, writer: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.set.write(writer)
    }

    pub fn write_to_client<T>(&self, writer: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.set.write_to_client(writer)
    }

    pub fn diff(&self, other: &ParamSetBox) -> ParamSetBox {
        self.set.diff(other)
    }

    pub fn set_param<P>(&mut self, name: &str, param: P) -> Option<Param> 
        where P: Into<Param>
    {
        self.set.set_param(name, param.into())
    }

    pub fn get_param(&self, name: &str) -> Option<&Param> {
        self.set.get_param(name)
    }
}

impl Deref for ParamSetBox {
    type Target = dyn DynParamSet;

    fn deref(&self) -> &Self::Target {
        self.set.as_ref()
    }
}

impl Clone for ParamSetBox {
    fn clone(&self) -> Self {
        Self { 
            class_id: self.class_id, 
            set: self.set.cloned(),
        }
    }
}

impl Serialize for ParamSetBox {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let json = self.as_persistent_json();
        json.serialize(s)
    }
}

impl <'de>Deserialize<'de> for ParamSetBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;
        Ok(ParamSetBox::from_json(&json).unwrap())
    }
}

impl Debug for ParamSetBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.set.fmt(f)
    }
}
