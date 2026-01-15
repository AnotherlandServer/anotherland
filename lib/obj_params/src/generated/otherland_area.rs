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
pub enum OtherlandArea {
    AwareRange,
    BlockedAbilities,
    Defb,
    DisplayName,
    Freq,
    PartyGuid,
    PathfindSafeSpawn,
    Pos,
    Power,
    Rot,
    Tags,
    Zone,
    ZoneGuid,
}
pub(crate) static OTHERLAND_AREA_ATTRIBUTES: phf::Map<&'static str, OtherlandArea> = phf_map! {
    "AwareRange" => OtherlandArea::AwareRange, "blockedAbilities" =>
    OtherlandArea::BlockedAbilities, "defb" => OtherlandArea::Defb, "DisplayName" =>
    OtherlandArea::DisplayName, "Freq" => OtherlandArea::Freq, "partyGUID" =>
    OtherlandArea::PartyGuid, "pathfindSafeSpawn" => OtherlandArea::PathfindSafeSpawn,
    "pos" => OtherlandArea::Pos, "Power" => OtherlandArea::Power, "rot" =>
    OtherlandArea::Rot, "tags" => OtherlandArea::Tags, "zone" => OtherlandArea::Zone,
    "ZoneGuid" => OtherlandArea::ZoneGuid,
};
pub(crate) static OTHERLAND_AREA_ATTRIBUTES_ID: phf::Map<u16, OtherlandArea> = phf_map! {
    8330u16 => OtherlandArea::AwareRange, 4034u16 => OtherlandArea::BlockedAbilities,
    6u16 => OtherlandArea::Defb, 5u16 => OtherlandArea::DisplayName, 12u16 =>
    OtherlandArea::Freq, 9u16 => OtherlandArea::PartyGuid, 4u16 =>
    OtherlandArea::PathfindSafeSpawn, 7u16 => OtherlandArea::Pos, 11u16 =>
    OtherlandArea::Power, 1u16 => OtherlandArea::Rot, 5093u16 => OtherlandArea::Tags,
    8u16 => OtherlandArea::Zone, 3u16 => OtherlandArea::ZoneGuid,
};
impl Attribute for OtherlandArea {
    fn class() -> Class {
        Class::OtherlandArea
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AwareRange => &Self::AwareRange,
            Self::BlockedAbilities => &Self::BlockedAbilities,
            Self::Defb => &Self::Defb,
            Self::DisplayName => &Self::DisplayName,
            Self::Freq => &Self::Freq,
            Self::PartyGuid => &Self::PartyGuid,
            Self::PathfindSafeSpawn => &Self::PathfindSafeSpawn,
            Self::Pos => &Self::Pos,
            Self::Power => &Self::Power,
            Self::Rot => &Self::Rot,
            Self::Tags => &Self::Tags,
            Self::Zone => &Self::Zone,
            Self::ZoneGuid => &Self::ZoneGuid,
        }
    }
}
impl AttributeInfo for OtherlandArea {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AwareRange => 8330u16,
            Self::BlockedAbilities => 4034u16,
            Self::Defb => 6u16,
            Self::DisplayName => 5u16,
            Self::Freq => 12u16,
            Self::PartyGuid => 9u16,
            Self::PathfindSafeSpawn => 4u16,
            Self::Pos => 7u16,
            Self::Power => 11u16,
            Self::Rot => 1u16,
            Self::Tags => 5093u16,
            Self::Zone => 8u16,
            Self::ZoneGuid => 3u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AwareRange => "AwareRange",
            Self::BlockedAbilities => "blockedAbilities",
            Self::Defb => "defb",
            Self::DisplayName => "DisplayName",
            Self::Freq => "Freq",
            Self::PartyGuid => "partyGUID",
            Self::PathfindSafeSpawn => "pathfindSafeSpawn",
            Self::Pos => "pos",
            Self::Power => "Power",
            Self::Rot => "rot",
            Self::Tags => "tags",
            Self::Zone => "zone",
            Self::ZoneGuid => "ZoneGuid",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AwareRange => ParamType::Float,
            Self::BlockedAbilities => ParamType::Int,
            Self::Defb => ParamType::String,
            Self::DisplayName => ParamType::String,
            Self::Freq => ParamType::Int,
            Self::PartyGuid => ParamType::Guid,
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::Pos => ParamType::Vector3,
            Self::Power => ParamType::Int,
            Self::Rot => ParamType::Vector3,
            Self::Tags => ParamType::String,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
        }
    }
    fn default(&self) -> &'static Value {
        static AWARE_RANGE: Value = Value::Float(0f32);
        static BLOCKED_ABILITIES: Value = Value::Int(0i32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            "DefaultDisplayName".to_string(),
        ));
        static FREQ: Value = Value::Int(0i32);
        static PARTY_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
        static POS: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static POWER: Value = Value::Int(0i32);
        static ROT: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            "otherlandArea".to_string(),
        ));
        static ZONE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        match self {
            Self::AwareRange => &AWARE_RANGE,
            Self::BlockedAbilities => &BLOCKED_ABILITIES,
            Self::Defb => &DEFB,
            Self::DisplayName => &DISPLAY_NAME,
            Self::Freq => &FREQ,
            Self::PartyGuid => &PARTY_GUID,
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::Pos => &POS,
            Self::Power => &POWER,
            Self::Rot => &ROT,
            Self::Tags => &TAGS,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AwareRange => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::BlockedAbilities => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Defb => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DisplayName => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PartyGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent],
            Self::Pos => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Rot => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Tags => &[ParamFlag::Persistent],
            Self::Zone => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ZoneGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
        }
    }
}
impl FromStr for OtherlandArea {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OTHERLAND_AREA_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for OtherlandArea {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8330u16 => Ok(Self::AwareRange),
            4034u16 => Ok(Self::BlockedAbilities),
            6u16 => Ok(Self::Defb),
            5u16 => Ok(Self::DisplayName),
            12u16 => Ok(Self::Freq),
            9u16 => Ok(Self::PartyGuid),
            4u16 => Ok(Self::PathfindSafeSpawn),
            7u16 => Ok(Self::Pos),
            11u16 => Ok(Self::Power),
            1u16 => Ok(Self::Rot),
            5093u16 => Ok(Self::Tags),
            8u16 => Ok(Self::Zone),
            3u16 => Ok(Self::ZoneGuid),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
