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
pub enum NonSpawnPlacementRadius {
    AwareRange,
    ContentClass,
    InstanceGroup,
    Pos,
    Rot,
    Ue3EdVisual,
    Uniquename,
}
pub(crate) static NON_SPAWN_PLACEMENT_RADIUS_ATTRIBUTES: phf::Map<
    &'static str,
    NonSpawnPlacementRadius,
> = phf_map! {
    "AwareRange" => NonSpawnPlacementRadius::AwareRange, "ContentClass" =>
    NonSpawnPlacementRadius::ContentClass, "instanceGroup" =>
    NonSpawnPlacementRadius::InstanceGroup, "pos" => NonSpawnPlacementRadius::Pos, "rot"
    => NonSpawnPlacementRadius::Rot, "UE3EdVisual" =>
    NonSpawnPlacementRadius::Ue3EdVisual, "uniquename" =>
    NonSpawnPlacementRadius::Uniquename,
};
pub(crate) static NON_SPAWN_PLACEMENT_RADIUS_ATTRIBUTES_ID: phf::Map<
    u16,
    NonSpawnPlacementRadius,
> = phf_map! {
    9691u16 => NonSpawnPlacementRadius::AwareRange, 9692u16 =>
    NonSpawnPlacementRadius::ContentClass, 11349u16 =>
    NonSpawnPlacementRadius::InstanceGroup, 9693u16 => NonSpawnPlacementRadius::Pos,
    9694u16 => NonSpawnPlacementRadius::Rot, 10036u16 =>
    NonSpawnPlacementRadius::Ue3EdVisual, 9696u16 => NonSpawnPlacementRadius::Uniquename,
};
impl Attribute for NonSpawnPlacementRadius {
    fn class() -> Class {
        Class::NonSpawnPlacementRadius
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
impl AttributeInfo for NonSpawnPlacementRadius {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AwareRange => 9691u16,
            Self::ContentClass => 9692u16,
            Self::InstanceGroup => 11349u16,
            Self::Pos => 9693u16,
            Self::Rot => 9694u16,
            Self::Ue3EdVisual => 10036u16,
            Self::Uniquename => 9696u16,
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
impl FromStr for NonSpawnPlacementRadius {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NON_SPAWN_PLACEMENT_RADIUS_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for NonSpawnPlacementRadius {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            9691u16 => Ok(Self::AwareRange),
            9692u16 => Ok(Self::ContentClass),
            11349u16 => Ok(Self::InstanceGroup),
            9693u16 => Ok(Self::Pos),
            9694u16 => Ok(Self::Rot),
            10036u16 => Ok(Self::Ue3EdVisual),
            9696u16 => Ok(Self::Uniquename),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
