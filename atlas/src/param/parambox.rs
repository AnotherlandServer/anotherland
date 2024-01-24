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

use std::{any::Any, io, str::FromStr};

use bitstream_io::ByteWrite;
use log::debug;
use nom::{error::VerboseError, IResult};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, json};
use specs::{Component, EntityBuilder, VecStorage};

use crate::{ClassId, ParamClass, ParamError};

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParamBox {
    class_id: ClassId,
    class: Box<dyn Any>,
}

unsafe impl Send for ParamBox {}
unsafe impl Sync for ParamBox {}

impl ParamBox {
    pub(crate) fn new(class_id: ClassId, class: Box<dyn Any>) -> Self {
        Self { class_id, class }
    }

    pub fn class_id(&self) -> ClassId { self.class_id }

    pub fn is<T>(&self) -> bool where T: ParamClass {
        self.class.is::<T>()
    }

    pub fn get<T>(&self) -> Result<&T, ParamError> where T: ParamClass {
        self.class.downcast_ref().ok_or(ParamError(()))
    }

    pub fn get_mut<T>(&mut self) -> Result<&mut T, ParamError> where T: ParamClass {
        self.class.downcast_mut().ok_or(ParamError(()))
    }

    pub fn take<T>(self) -> Result<T, ParamError> where T: ParamClass {
        self.class.downcast().map_err(|_| ParamError(())).map(|v| *v)
    }

    pub fn into_persistent_json(&self) -> Value {
        let serialized = self.class_id.class_into_json(self.class.as_ref());

        json!({ self.class_id.to_string(): serialized })
    }

    pub fn from_json(value: &Value) -> Result<Self, io::Error> {
        let (class_name, value) = value.as_object()
            .map(|v| v.iter().next())
            .flatten()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;

        let class_id = ClassId::from_str(&class_name)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid class name"))?;

        let value = class_id.class_from_json(value)?;

        Ok(Self {
            class_id,
            class: value,
        })
    }

    pub fn read<'a>(class_id: u16, i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
        let class_id: ClassId = class_id.try_into().expect("unknown class id");
        let (i, class) = class_id.read(i)?;

        Ok((i, Self {
            class_id,
            class
        }))
    }

    pub fn write<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class_id.write(self.class.as_ref(), value)
    }

    pub fn write_to_client<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class_id.write_to_client(self.class.as_ref(), value)
    }

    pub fn strip_original_data(&mut self) {

    }

    pub fn append_to_entity<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        self.class_id.append_to_entity(self.class.as_ref(), builder)
    }
}

impl Serialize for ParamBox {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let json = self.into_persistent_json();
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
            class_id: self.class_id.clone(), 
            class: self.class_id.clone_class(self.class.as_ref()),
        }
    }
}