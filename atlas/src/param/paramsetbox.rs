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
use nom::{error::VerboseError, IResult};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, json};

use crate::{ClassId, ParamAttrib, ParamError, ParamSet};

pub struct ParamSetBox {
    class_id: ClassId,
    set: Box<dyn Any + Send + Sync>,
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

    pub fn class_id(&self) -> ClassId { self.class_id }

    pub fn is<T>(&self) -> bool where T: ParamAttrib + 'static {
        self.set.is::<ParamSet<T>>()
    }

    pub fn get<T>(&self) -> Result<&ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        self.set.downcast_ref().ok_or(ParamError(()))
    }

    pub fn get_mut<T>(&mut self) -> Result<&mut ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        self.set.downcast_mut().ok_or(ParamError(()))
    }

    pub fn take<T>(self) -> Result<ParamSet<T>, ParamError> where T: ParamAttrib + 'static {
        self.set.downcast().map_err(|_| ParamError(())).map(|v| *v)
    }

    pub fn as_persistent_json(&self) -> Value {
        let serialized = self.class_id.set_into_json(self.set.as_ref());

        json!({ self.class_id.to_string(): serialized })
    }

    pub fn from_json(value: &Value) -> Result<Self, io::Error> {
        let (class_name, value) = value.as_object()
            .and_then(|v| v.iter().next())
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;

        let class_id = ClassId::from_str(class_name)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "unvalid class name"))?;

        let value = class_id.set_from_json(value)?;

        Ok(Self {
            class_id,
            set: value,
        })
    }

    pub fn read(class_id: u16, i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let class_id: ClassId = class_id.try_into().expect("unknown class id");
        let (i, value) = class_id.read_set(i)?;

        Ok((i, Self {
            class_id,
            set: value
        }))
    }

    pub fn write<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class_id.write_set(self.set.as_ref(), value)
    }

    pub fn write_to_client<T>(&self, value: &mut T) -> Result<(), io::Error> 
    where T: ByteWrite
    {
        self.class_id.write_set_to_client(self.set.as_ref(), value)
    }

    pub fn strip_original_data(&mut self) {

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

