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
pub enum NonSpawnPlacement {
    AwareRange,
    ContentClass,
    InstanceGroup,
    Pos,
    Rot,
    Ue3EdVisual,
    Uniquename,
}
pub(crate) static NON_SPAWN_PLACEMENT_ATTRIBUTES: phf::Map<
    &'static str,
    NonSpawnPlacement,
> = phf_map! {
    "AwareRange" => NonSpawnPlacement::AwareRange, "ContentClass" =>
    NonSpawnPlacement::ContentClass, "instanceGroup" => NonSpawnPlacement::InstanceGroup,
    "pos" => NonSpawnPlacement::Pos, "rot" => NonSpawnPlacement::Rot, "UE3EdVisual" =>
    NonSpawnPlacement::Ue3EdVisual, "uniquename" => NonSpawnPlacement::Uniquename,
};
pub(crate) static NON_SPAWN_PLACEMENT_ATTRIBUTES_ID: phf::Map<u16, NonSpawnPlacement> = phf_map! {
    8329u16 => NonSpawnPlacement::AwareRange, 5431u16 => NonSpawnPlacement::ContentClass,
    11347u16 => NonSpawnPlacement::InstanceGroup, 5416u16 => NonSpawnPlacement::Pos,
    5415u16 => NonSpawnPlacement::Rot, 10035u16 => NonSpawnPlacement::Ue3EdVisual,
    9695u16 => NonSpawnPlacement::Uniquename,
};
impl Attribute for NonSpawnPlacement {
    fn class() -> Class {
        Class::NonSpawnPlacement
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AwareRange => &Self::AwareRange,
            Self::ContentClass => &Self::ContentClass,
            Self::InstanceGroup => &Self::InstanceGroup,
            Self::Pos => &Self::Pos,
            Self::Rot => &Self::Rot,
            Self::Ue3EdVisual => &Self::Ue3EdVisual,
            Self::Uniquename => &Self::Uniquename,
        }
    }
}
impl AttributeInfo for NonSpawnPlacement {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AwareRange => 8329u16,
            Self::ContentClass => 5431u16,
            Self::InstanceGroup => 11347u16,
            Self::Pos => 5416u16,
            Self::Rot => 5415u16,
            Self::Ue3EdVisual => 10035u16,
            Self::Uniquename => 9695u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AwareRange => "AwareRange",
            Self::ContentClass => "ContentClass",
            Self::InstanceGroup => "instanceGroup",
            Self::Pos => "pos",
            Self::Rot => "rot",
            Self::Ue3EdVisual => "UE3EdVisual",
            Self::Uniquename => "uniquename",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AwareRange => ParamType::Float,
            Self::ContentClass => ParamType::String,
            Self::InstanceGroup => ParamType::String,
            Self::Pos => ParamType::Vector3,
            Self::Rot => ParamType::Vector3,
            Self::Ue3EdVisual => ParamType::String,
            Self::Uniquename => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static AWARE_RANGE: Value = Value::Float(0f32);
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static INSTANCE_GROUP: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static POS: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static ROT: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static UE_3_ED_VISUAL: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static UNIQUENAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        match self {
            Self::AwareRange => &AWARE_RANGE,
            Self::ContentClass => &CONTENT_CLASS,
            Self::InstanceGroup => &INSTANCE_GROUP,
            Self::Pos => &POS,
            Self::Rot => &ROT,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::Uniquename => &UNIQUENAME,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AwareRange => &[ParamFlag::Persistent],
            Self::ContentClass => &[ParamFlag::Persistent],
            Self::InstanceGroup => {
                &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting]
            }
            Self::Pos => &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting],
            Self::Rot => &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting],
            Self::Ue3EdVisual => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
            Self::Uniquename => &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting],
        }
    }
}
impl FromStr for NonSpawnPlacement {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NON_SPAWN_PLACEMENT_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for NonSpawnPlacement {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8329u16 => Ok(Self::AwareRange),
            5431u16 => Ok(Self::ContentClass),
            11347u16 => Ok(Self::InstanceGroup),
            5416u16 => Ok(Self::Pos),
            5415u16 => Ok(Self::Rot),
            10035u16 => Ok(Self::Ue3EdVisual),
            9695u16 => Ok(Self::Uniquename),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
