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

// #################################################
// # This file is generated. Do not edit manually. #
// #################################################

#![allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use phf::phf_map;
use toolkit::types::AvatarId;
use toolkit::types::Uuid;
use toolkit::types::UUID_NIL;
use glam::Vec3;
use serde_json::Value as JsonValue;
use crate::Attribute;
use crate::AttributeInfo;
use crate::Class;
use crate::ContentRefList;
use crate::ParamType;
use crate::ParamFlag;
use crate::ParamError;
use crate::Value;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Instance {
    EntryPoint,
    ExitPoint,
    GroupType,
    InstanceWorld,
    InstanceZones,
    ScopeType,
}
pub(crate) static INSTANCE_ATTRIBUTES: phf::Map<&'static str, Instance> = phf_map! {
    "EntryPoint" => Instance::EntryPoint, "ExitPoint" => Instance::ExitPoint, "GroupType"
    => Instance::GroupType, "InstanceWorld" => Instance::InstanceWorld, "InstanceZones"
    => Instance::InstanceZones, "ScopeType" => Instance::ScopeType,
};
pub(crate) static INSTANCE_ATTRIBUTES_ID: phf::Map<u16, Instance> = phf_map! {
    11403u16 => Instance::EntryPoint, 11402u16 => Instance::ExitPoint, 11401u16 =>
    Instance::GroupType, 11405u16 => Instance::InstanceWorld, 11404u16 =>
    Instance::InstanceZones, 11400u16 => Instance::ScopeType,
};
impl Attribute for Instance {
    fn class() -> Class {
        Class::Instance
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::EntryPoint => &Self::EntryPoint,
            Self::ExitPoint => &Self::ExitPoint,
            Self::GroupType => &Self::GroupType,
            Self::InstanceWorld => &Self::InstanceWorld,
            Self::InstanceZones => &Self::InstanceZones,
            Self::ScopeType => &Self::ScopeType,
        }
    }
}
impl AttributeInfo for Instance {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::EntryPoint => 11403u16,
            Self::ExitPoint => 11402u16,
            Self::GroupType => 11401u16,
            Self::InstanceWorld => 11405u16,
            Self::InstanceZones => 11404u16,
            Self::ScopeType => 11400u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::EntryPoint => "EntryPoint",
            Self::ExitPoint => "ExitPoint",
            Self::GroupType => "GroupType",
            Self::InstanceWorld => "InstanceWorld",
            Self::InstanceZones => "InstanceZones",
            Self::ScopeType => "ScopeType",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::EntryPoint => ParamType::String,
            Self::ExitPoint => ParamType::String,
            Self::GroupType => ParamType::Int,
            Self::InstanceWorld => ParamType::Guid,
            Self::InstanceZones => ParamType::GuidSet,
            Self::ScopeType => ParamType::Int,
        }
    }
    fn default(&self) -> &'static Value {
        static ENTRY_POINT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static EXIT_POINT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static GROUP_TYPE: Value = Value::Int(0i32);
        static INSTANCE_WORLD: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static INSTANCE_ZONES: Lazy<Value> = Lazy::new(|| Value::GuidSet(
            HashSet::new(),
        ));
        static SCOPE_TYPE: Value = Value::Int(0i32);
        match self {
            Self::EntryPoint => &ENTRY_POINT,
            Self::ExitPoint => &EXIT_POINT,
            Self::GroupType => &GROUP_TYPE,
            Self::InstanceWorld => &INSTANCE_WORLD,
            Self::InstanceZones => &INSTANCE_ZONES,
            Self::ScopeType => &SCOPE_TYPE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::EntryPoint => &[ParamFlag::Content],
            Self::ExitPoint => &[ParamFlag::Content],
            Self::GroupType => &[ParamFlag::Content],
            Self::InstanceWorld => &[ParamFlag::Content],
            Self::InstanceZones => &[ParamFlag::Content],
            Self::ScopeType => &[ParamFlag::Content],
        }
    }
}
impl FromStr for Instance {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        INSTANCE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Instance {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11403u16 => Ok(Self::EntryPoint),
            11402u16 => Ok(Self::ExitPoint),
            11401u16 => Ok(Self::GroupType),
            11405u16 => Ok(Self::InstanceWorld),
            11404u16 => Ok(Self::InstanceZones),
            11400u16 => Ok(Self::ScopeType),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
