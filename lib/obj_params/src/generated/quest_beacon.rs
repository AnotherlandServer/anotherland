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
pub enum QuestBeacon {
    AwareRange,
    ContentClass,
    InstanceGroup,
    Pos,
    Rot,
    Ue3EdVisual,
    Uniquename,
    BeaconHeight,
    BeaconRadius,
}
pub(crate) static QUEST_BEACON_ATTRIBUTES: phf::Map<&'static str, QuestBeacon> = phf_map! {
    "AwareRange" => QuestBeacon::AwareRange, "ContentClass" => QuestBeacon::ContentClass,
    "instanceGroup" => QuestBeacon::InstanceGroup, "pos" => QuestBeacon::Pos, "rot" =>
    QuestBeacon::Rot, "UE3EdVisual" => QuestBeacon::Ue3EdVisual, "uniquename" =>
    QuestBeacon::Uniquename, "BeaconHeight" => QuestBeacon::BeaconHeight, "BeaconRadius"
    => QuestBeacon::BeaconRadius,
};
pub(crate) static QUEST_BEACON_ATTRIBUTES_ID: phf::Map<u16, QuestBeacon> = phf_map! {
    10991u16 => QuestBeacon::AwareRange, 10992u16 => QuestBeacon::ContentClass, 11348u16
    => QuestBeacon::InstanceGroup, 10993u16 => QuestBeacon::Pos, 10994u16 =>
    QuestBeacon::Rot, 10989u16 => QuestBeacon::Ue3EdVisual, 10990u16 =>
    QuestBeacon::Uniquename, 12445u16 => QuestBeacon::BeaconHeight, 10995u16 =>
    QuestBeacon::BeaconRadius,
};
impl Attribute for QuestBeacon {
    fn class() -> Class {
        Class::QuestBeacon
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
            Self::BeaconHeight => &Self::BeaconHeight,
            Self::BeaconRadius => &Self::BeaconRadius,
        }
    }
}
impl AttributeInfo for QuestBeacon {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AwareRange => 10991u16,
            Self::ContentClass => 10992u16,
            Self::InstanceGroup => 11348u16,
            Self::Pos => 10993u16,
            Self::Rot => 10994u16,
            Self::Ue3EdVisual => 10989u16,
            Self::Uniquename => 10990u16,
            Self::BeaconHeight => 12445u16,
            Self::BeaconRadius => 10995u16,
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
            Self::BeaconHeight => "BeaconHeight",
            Self::BeaconRadius => "BeaconRadius",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::BeaconHeight => ParamType::Int,
            Self::BeaconRadius => ParamType::Int,
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
        static BEACON_HEIGHT: Value = Value::Int(0i32);
        static BEACON_RADIUS: Value = Value::Int(0i32);
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
            Self::BeaconHeight => &BEACON_HEIGHT,
            Self::BeaconRadius => &BEACON_RADIUS,
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
            Self::BeaconHeight => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BeaconRadius => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
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
impl FromStr for QuestBeacon {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        QUEST_BEACON_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for QuestBeacon {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            10991u16 => Ok(Self::AwareRange),
            10992u16 => Ok(Self::ContentClass),
            11348u16 => Ok(Self::InstanceGroup),
            10993u16 => Ok(Self::Pos),
            10994u16 => Ok(Self::Rot),
            10989u16 => Ok(Self::Ue3EdVisual),
            10990u16 => Ok(Self::Uniquename),
            12445u16 => Ok(Self::BeaconHeight),
            10995u16 => Ok(Self::BeaconRadius),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
