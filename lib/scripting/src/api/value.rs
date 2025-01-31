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

use std::{collections::{HashMap, HashSet}, str::FromStr};

use mlua::{IntoLua, FromLua, Lua, LuaSerdeExt, Value};
use obj_params::{AttributeInfo, ParamType};
use toolkit::{types::{AvatarId, Uuid}, QuatWrapper, Vec3Wrapper, Vec4Wrapper};

pub struct ParamValue(obj_params::Value);

impl ParamValue {
    pub fn new(val: obj_params::Value) -> Self {
        Self(val)
    }

    pub fn from_lua(attr: &'static dyn AttributeInfo, val: Value, lua: &Lua) -> Result<Self, mlua::Error> {
        let param_val = match attr.datatype() {
            ParamType::String => obj_params::Value::String(String::from_lua(val, lua)?),
            ParamType::StringPair => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("string pair expected"))?;
                obj_params::Value::StringPair((
                    tbl.get::<String>(1)?,
                    tbl.get::<String>(2)?,
                ))
            },
            ParamType::StringFloatPair => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("string pair expected"))?;
                obj_params::Value::StringFloatPair((
                    tbl.get::<String>(1)?,
                    tbl.get::<f32>(2)?,
                ))
            },
            ParamType::StringSet => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("string set"))?;
                let mut set = HashSet::new();
                for val in tbl.sequence_values::<String>() {
                    set.insert(val?);
                }

                obj_params::Value::StringSet(set)
            },
            ParamType::Guid => obj_params::Value::Guid(String::from_lua(val, lua)?.parse().map_err(mlua::Error::external)?),
            ParamType::GuidPair => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("guid pair expected"))?;
                obj_params::Value::GuidPair((
                    tbl.get::<String>(1)?.parse().map_err(mlua::Error::external)?,
                    tbl.get::<String>(2)?.parse().map_err(mlua::Error::external)?,
                ))
            },
            ParamType::Bool => obj_params::Value::Bool(bool::from_lua(val, lua)?),
            ParamType::Int => obj_params::Value::Int(i32::from_lua(val, lua)?),
            ParamType::BitField128 => obj_params::Value::BitField128(u128::from_lua(val, lua)?),
            ParamType::BitSetFilter => obj_params::Value::BitSetFilter(u32::from_lua(val, lua)?),
            ParamType::Float => obj_params::Value::Float(f32::from_lua(val, lua)?),
            ParamType::FloatRange => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("float pair expected"))?;
                obj_params::Value::FloatRange((
                    tbl.get::<f32>(1)?,
                    tbl.get::<f32>(2)?,
                ))
            },
            ParamType::Vector3 => obj_params::Value::Vector3(Vec3Wrapper::from_lua(val, lua)?.0),
            ParamType::Vector3Uts => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("float pair expected"))?;
                obj_params::Value::Vector3Uts((
                    tbl.get::<u32>(1)?,
                    tbl.get::<Vec3Wrapper>(2)?.0,
                ))
            },
            ParamType::Vector4 => obj_params::Value::Vector4(Vec4Wrapper::from_lua(val, lua)?.0),
            ParamType::LocalizedString => obj_params::Value::LocalizedString(val.to_string()?.parse().map_err(mlua::Error::external)?),
            ParamType::AvatarId => obj_params::Value::AvatarId(AvatarId::from_u64(u64::from_lua(val, lua)?)),
            ParamType::UniqueId => obj_params::Value::UniqueId(i32::from_lua(val, lua)?),
            ParamType::JsonValue => obj_params::Value::JsonValue(serde_json::to_value(val).map_err(mlua::Error::external)?),
            ParamType::Int64 => obj_params::Value::Int64(i64::from_lua(val, lua)?),
            ParamType::Quarternion => obj_params::Value::Quarternion(QuatWrapper::from_lua(val, lua)?.0),
            ParamType::Positionable => {
                let tbl = val.as_table().ok_or(mlua::Error::runtime("positionable expected"))?;
                obj_params::Value::Positionable(
                    tbl.get::<QuatWrapper>(1)?.0,
                    tbl.get::<Vec3Wrapper>(2)?.0,
                )
            },
            ParamType::ContentRef => obj_params::Value::ContentRef(String::from_lua(val, lua)?),
            ParamType::ContentRefAndInt => obj_params::Value::ContentRefAndInt(String::from_lua(val, lua)?),
            ParamType::ContentRefAndFloat => obj_params::Value::ContentRefAndFloat(String::from_lua(val, lua)?),
            ParamType::ContentRefList => obj_params::Value::ContentRefList(String::from_lua(val, lua)?),
            ParamType::ClassRefPowerRangeList => obj_params::Value::ClassRefPowerRangeList(String::from_lua(val, lua)?),
            ParamType::VectorInt => obj_params::Value::VectorInt(Vec::<i32>::from_lua(val, lua)?),
            ParamType::VectorInt64 => obj_params::Value::VectorInt64(Vec::<i64>::from_lua(val, lua)?),
            ParamType::VectorFloat => obj_params::Value::VectorFloat(Vec::<f32>::from_lua(val, lua)?),
            ParamType::VectorString => obj_params::Value::VectorString(Vec::<String>::from_lua(val, lua)?),
            ParamType::AvatarIdSet => obj_params::Value::AvatarIdSet(
                Vec::<u64>::from_lua(val, lua)?
                    .into_iter()
                    .map(AvatarId::from_u64)
                    .collect()
            ),
            ParamType::VectorAvatarId => obj_params::Value::VectorAvatarId(
                Vec::<u64>::from_lua(val, lua)?
                    .into_iter()
                    .map(AvatarId::from_u64)
                    .collect()
            ),
            ParamType::GuidSet => obj_params::Value::GuidSet(
                Vec::<String>::from_lua(val, lua)?
                    .into_iter()
                    .map(|s| Uuid::from_str(&s).map_err(mlua::Error::external))
                    .collect::<mlua::Result<_>>()?
            ),
            ParamType::VectorGuid => obj_params::Value::VectorGuid(
                Vec::<String>::from_lua(val, lua)?
                    .into_iter()
                    .map(|s| Uuid::from_str(&s).map_err(mlua::Error::external))
                    .collect::<mlua::Result<_>>()?
            ),
            ParamType::HashmapStringInt => obj_params::Value::HashmapStringInt(HashMap::<String, i32>::from_lua(val, lua)?),
            ParamType::HashmapStringString => obj_params::Value::HashmapStringString(HashMap::<String, String>::from_lua(val, lua)?),
            ParamType::Any => obj_params::Value::Any(Vec::<u8>::from_lua(val, lua)?),
            ParamType::VectorLocalizedString => obj_params::Value::VectorLocalizedString(
                Vec::<String>::from_lua(val, lua)?
                    .into_iter()
                    .map(|s| Uuid::from_str(&s).map_err(mlua::Error::external))
                    .collect::<mlua::Result<_>>()?
            ),
            ParamType::InstanceGroup => obj_params::Value::InstanceGroup(String::from_lua(val, lua)?),
        };

        Ok(ParamValue::new(param_val))
    }
}

fn pair_to_lua<T1: IntoLua, T2: IntoLua>(lua: &Lua, pair: (T1, T2)) -> mlua::Result<mlua::Value> {
    let tbl = lua.create_table_with_capacity(2, 0)?;
    tbl.set(1, pair.0.into_lua(lua)?)?;
    tbl.set(2, pair.1.into_lua(lua)?)?;
    tbl.into_lua(lua)
}

impl IntoLua for ParamValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self.0 {
            obj_params::Value::String(str) => str.into_lua(lua),
            obj_params::Value::StringPair(pair) => pair_to_lua(lua, pair),
            obj_params::Value::StringFloatPair(pair) => pair_to_lua(lua, pair),
            obj_params::Value::StringSet(hash_set) => hash_set.into_lua(lua),
            obj_params::Value::Guid(uuid) => uuid.to_string().into_lua(lua),
            obj_params::Value::GuidPair(pair) => pair_to_lua(lua, (
                pair.0.to_string(),
                pair.1.to_string(),
            )),
            obj_params::Value::Bool(val) => val.into_lua(lua),
            obj_params::Value::Int(val) => val.into_lua(lua),
            obj_params::Value::BitField128(val) => val.into_lua(lua),
            obj_params::Value::BitSetFilter(val) => val.into_lua(lua),
            obj_params::Value::Float(val) => val.into_lua(lua),
            obj_params::Value::FloatRange(pair) => pair_to_lua(lua, pair),
            obj_params::Value::Vector3(vec3) => Vec3Wrapper(vec3).into_lua(lua),
            obj_params::Value::Vector3Uts((uts, vec3)) => pair_to_lua(lua, (
                uts,
                Vec3Wrapper(vec3)
            )),
            obj_params::Value::Vector4(vec4) => Vec4Wrapper(vec4).into_lua(lua),
            obj_params::Value::LocalizedString(uuid) => uuid.to_string().into_lua(lua),
            obj_params::Value::AvatarId(avatar_id) => avatar_id.as_u64().into_lua(lua),
            obj_params::Value::UniqueId(id) => id.into_lua(lua),
            obj_params::Value::JsonValue(value) => lua.to_value(&value),
            obj_params::Value::Int64(val) => val.into_lua(lua),
            obj_params::Value::Quarternion(quat) => QuatWrapper(quat).into_lua(lua),
            obj_params::Value::Positionable(quat, vec3) => pair_to_lua(lua, (
                QuatWrapper(quat),
                Vec3Wrapper(vec3)
            )),
            obj_params::Value::ContentRef(val) => val.into_lua(lua),
            obj_params::Value::ContentRefAndInt(val) => val.into_lua(lua),
            obj_params::Value::ContentRefAndFloat(val) => val.into_lua(lua),
            obj_params::Value::ContentRefList(val) => val.into_lua(lua),
            obj_params::Value::ClassRefPowerRangeList(val) => val.into_lua(lua),
            obj_params::Value::VectorInt(vec) => vec.into_lua(lua),
            obj_params::Value::VectorInt64(vec) => vec.into_lua(lua),
            obj_params::Value::VectorFloat(vec) => vec.into_lua(lua),
            obj_params::Value::VectorString(vec) => vec.into_lua(lua),
            obj_params::Value::AvatarIdSet(hash_set) => {
                hash_set.into_iter()
                    .map(|id| id.as_u64())
                    .collect::<Vec<_>>()
                    .into_lua(lua)
            },
            obj_params::Value::VectorAvatarId(vec) => {
                vec.into_iter()
                    .map(|id| id.as_u64())
                    .collect::<Vec<_>>()
                    .into_lua(lua)
            },
            obj_params::Value::GuidSet(hash_set) => {
                hash_set.into_iter()
                    .map(|guid| guid.to_string())
                    .collect::<Vec<_>>()
                    .into_lua(lua)
            },
            obj_params::Value::VectorGuid(vec) => {
                vec.into_iter()
                    .map(|guid| guid.to_string())
                    .collect::<Vec<_>>()
                    .into_lua(lua)
            },
            obj_params::Value::HashmapStringInt(hash_map) => hash_map.into_lua(lua),
            obj_params::Value::HashmapStringString(hash_map) => hash_map.into_lua(lua),
            obj_params::Value::Any(vec) => vec.into_lua(lua),
            obj_params::Value::VectorLocalizedString(vec) => {
                vec.into_iter()
                    .map(|guid| guid.to_string())
                    .collect::<Vec<_>>()
                    .into_lua(lua)
            },
            obj_params::Value::InstanceGroup(group) => group.into_lua(lua),
        }
    }
}

impl From<ParamValue> for obj_params::Value {
    fn from(value: ParamValue) -> Self {
        value.0
    }
}