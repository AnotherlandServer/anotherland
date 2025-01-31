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

use std::{any::TypeId, fmt::Debug, marker::PhantomData, mem::swap, ops::{Deref, DerefMut}, sync::{MappedMutexGuard, Mutex, MutexGuard}};

use bitstream_io::{ByteWriter, LittleEndian};
use serde::{de::DeserializeOwned, ser::Error, Deserialize, Serialize};
use serde_json::Value;
use toolkit::anyhow::anyhow;

use crate::{ParamError, ParamResult};

pub trait AnyParam: Debug + Serialize + DeserializeOwned + 'static {
    fn from_slice(data: &[u8]) -> ParamResult<impl AnyParam>;
    fn write_bytes(&self, w: impl bitstream_io::ByteWrite) -> ParamResult<()>;
}

trait AnyReader {
    fn from_json(val: serde_json::Value) -> ParamResult<impl AnyParam>;
}

impl <T> AnyReader for T 
    where T: AnyParam
{
    fn from_json(value: serde_json::Value) -> ParamResult<impl AnyParam> {
        serde_json::from_value::<T>(value)
            .map_err(|e| e.into())
    }
}

trait AnyWriter {
    fn as_json(&self) -> ParamResult<serde_json::Value>;
}

impl <T> AnyWriter for T
    where T: AnyParam
{
    fn as_json(&self) -> ParamResult<serde_json::Value> {
        serde_json::to_value(self)
            .map_err(|e| e.into())
    }
}

pub trait DynAnyParam: Debug {
    fn type_id(&self) -> TypeId;
    fn as_bytes(&self) -> ParamResult<Vec<u8>>;
    fn as_json(&self) -> ParamResult<serde_json::Value>;
}

impl <T> DynAnyParam for T 
    where T: AnyParam
{
    fn type_id(&self) -> TypeId { TypeId::of::<T>() }
    fn as_bytes(&self) -> ParamResult<Vec<u8>> {
        let mut serialized = Vec::new();
        let writer = ByteWriter::endian(&mut serialized, LittleEndian);

        self.write_bytes(writer)?;

        Ok(serialized)
    }

    fn as_json(&self) -> ParamResult<serde_json::Value> {
        <T as AnyWriter>::as_json(self)
    }
}

impl dyn DynAnyParam {
    pub fn is<T: AnyParam>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast_ref<T: AnyParam>(&self) -> ParamResult<&T> {
        if self.is::<T>() {
            unsafe {
                Ok(&*(self as *const dyn DynAnyParam as * const T))
            }
        } else {
            Err(ParamError::TypeMismatch)
        }
    }

    pub fn downcast_mut<T: AnyParam>(&mut self) -> ParamResult<&mut T> {
        if self.is::<T>() {
            unsafe {
                Ok(&mut *(self as * mut dyn DynAnyParam as * mut T))
            }
        } else {
            Err(ParamError::TypeMismatch)
        }
    }
}

pub struct AnyRef<'a, T: AnyParam>(MappedMutexGuard<'a, T>);

impl <'a, T: AnyParam> Deref for AnyRef<'a, T> {
    type Target = <MappedMutexGuard<'a, T> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct AnyRefMut<'a, T: AnyParam>(MappedMutexGuard<'a, T>);

impl <'a, T: AnyParam> Deref for AnyRefMut<'a, T> {
    type Target = <MappedMutexGuard<'a, T> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl <T: AnyParam> DerefMut for AnyRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

#[derive(Debug)]
enum AnyData {
    Undefined,
    Buffer(Vec<u8>),
    Json(Value),
    Struct(Box<dyn DynAnyParam>),
}

#[derive(Debug)]
pub struct AnyHolder {
    value: Mutex<AnyData>,
}

unsafe impl Send for AnyHolder {}
unsafe impl Sync for AnyHolder {}

impl AnyHolder {
    fn prepare_value<T: AnyParam>(&self) -> ParamResult<()> {
        let mut v = self.value.lock().unwrap();

        match v.deref() {
            AnyData::Undefined => Err(ParamError::TypeMismatch),
            AnyData::Buffer(vec) => {
                let data = T::from_slice(vec)?;
                *v = AnyData::Struct(Box::new(data));
                Ok(())
            },
            AnyData::Json(_) => {
                let mut value = AnyData::Undefined;
                swap(&mut value, v.deref_mut());

                if let AnyData::Json(value) = value {
                    let data = T::from_json(value)?;
                    *v = AnyData::Struct(Box::new(data));
                    Ok(())
                } else {
                    unreachable!()
                }
            },
            AnyData::Struct(data) => {
                if data.is::<T>() {
                    Ok(())
                } else {
                    Err(ParamError::TypeMismatch)
                }
            },
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { 
            value: Mutex::new(AnyData::Buffer(data))
        }
    }

    pub fn as_ref<T: AnyParam>(&self) -> ParamResult<AnyRef<T>> {
        self.prepare_value::<T>()?;

        let v = self.value.lock().unwrap();
        Ok(AnyRef(MutexGuard::map(v, |v| {
            if let AnyData::Struct(data) = v {
                data.downcast_mut::<T>().unwrap()
            } else {
                unreachable!()
            }
        })))
    }

    pub fn as_mut<T: AnyParam>(&self) -> ParamResult<AnyRefMut<T>> {
        self.prepare_value::<T>()?;

        let v = self.value.lock().unwrap();
        Ok(AnyRefMut(MutexGuard::map(v, |v| {
            if let AnyData::Struct(data) = v {
                data.downcast_mut::<T>().unwrap()
            } else {
                unreachable!()
            }
        })))
    }

    pub fn as_bytes(&self) -> ParamResult<Vec<u8>> {
        let v = self.value.lock().unwrap();
        
        match v.deref() {
            AnyData::Undefined => todo!(),
            AnyData::Buffer(vec) => Ok(vec.clone()),
            AnyData::Json(value) => Err(anyhow!("struct not parsed (yet)!").into()),
            AnyData::Struct(data) => data.as_bytes(),
        }
    }
}

impl Serialize for AnyHolder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        let s = self.value.lock().unwrap();
        
        match s.deref() {
            AnyData::Undefined => serializer.serialize_unit(),
            AnyData::Buffer(_) => Err(S::Error::custom("struct unknown")),
            AnyData::Json(value) => {
                value.serialize(serializer)
            },
            AnyData::Struct(data) => {
                let value = data.as_json()
                    .map_err(S::Error::custom)?;

                value.serialize(serializer)
            },
        }
    }
}

impl <'de> Deserialize<'de> for AnyHolder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        
        Ok(AnyHolder {
            value: Mutex::new(AnyData::Json(Value::deserialize(deserializer)?))
        })
    }
}