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
pub enum OaZoneConfig {
    AllowSummonPortal,
    ForceGenerateGuidKey,
    InstanceScope,
    InstanceType,
    IsInstance,
    IsPhased,
    IsUseMount,
    JsonConfig,
    NumInitialPooledItems,
    NumPooledItemsBumpUp,
    OnlySpawnToEntryPoint,
    SpawnToTheLastSavePosition,
    UseGuidAsKey,
    ZoneType,
}
pub(crate) static OA_ZONE_CONFIG_ATTRIBUTES: phf::Map<&'static str, OaZoneConfig> = phf_map! {
    "allowSummonPortal" => OaZoneConfig::AllowSummonPortal, "forceGenerateGuidKey" =>
    OaZoneConfig::ForceGenerateGuidKey, "InstanceScope" => OaZoneConfig::InstanceScope,
    "InstanceType" => OaZoneConfig::InstanceType, "isInstance" =>
    OaZoneConfig::IsInstance, "isPhased" => OaZoneConfig::IsPhased, "isUseMount" =>
    OaZoneConfig::IsUseMount, "jsonConfig" => OaZoneConfig::JsonConfig,
    "numInitialPooledItems" => OaZoneConfig::NumInitialPooledItems,
    "numPooledItemsBumpUp" => OaZoneConfig::NumPooledItemsBumpUp, "OnlySpawnToEntryPoint"
    => OaZoneConfig::OnlySpawnToEntryPoint, "SpawnToTheLastSavePosition" =>
    OaZoneConfig::SpawnToTheLastSavePosition, "useGuidAsKey" =>
    OaZoneConfig::UseGuidAsKey, "zoneType" => OaZoneConfig::ZoneType,
};
pub(crate) static OA_ZONE_CONFIG_ATTRIBUTES_ID: phf::Map<u16, OaZoneConfig> = phf_map! {
    9689u16 => OaZoneConfig::AllowSummonPortal, 10391u16 =>
    OaZoneConfig::ForceGenerateGuidKey, 11458u16 => OaZoneConfig::InstanceScope, 11459u16
    => OaZoneConfig::InstanceType, 10062u16 => OaZoneConfig::IsInstance, 5611u16 =>
    OaZoneConfig::IsPhased, 12186u16 => OaZoneConfig::IsUseMount, 10017u16 =>
    OaZoneConfig::JsonConfig, 11010u16 => OaZoneConfig::NumInitialPooledItems, 11009u16
    => OaZoneConfig::NumPooledItemsBumpUp, 10895u16 =>
    OaZoneConfig::OnlySpawnToEntryPoint, 10894u16 =>
    OaZoneConfig::SpawnToTheLastSavePosition, 9697u16 => OaZoneConfig::UseGuidAsKey,
    5612u16 => OaZoneConfig::ZoneType,
};
impl Attribute for OaZoneConfig {
    fn class() -> Class {
        Class::OaZoneConfig
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AllowSummonPortal => &Self::AllowSummonPortal,
            Self::ForceGenerateGuidKey => &Self::ForceGenerateGuidKey,
            Self::InstanceScope => &Self::InstanceScope,
            Self::InstanceType => &Self::InstanceType,
            Self::IsInstance => &Self::IsInstance,
            Self::IsPhased => &Self::IsPhased,
            Self::IsUseMount => &Self::IsUseMount,
            Self::JsonConfig => &Self::JsonConfig,
            Self::NumInitialPooledItems => &Self::NumInitialPooledItems,
            Self::NumPooledItemsBumpUp => &Self::NumPooledItemsBumpUp,
            Self::OnlySpawnToEntryPoint => &Self::OnlySpawnToEntryPoint,
            Self::SpawnToTheLastSavePosition => &Self::SpawnToTheLastSavePosition,
            Self::UseGuidAsKey => &Self::UseGuidAsKey,
            Self::ZoneType => &Self::ZoneType,
        }
    }
}
impl AttributeInfo for OaZoneConfig {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AllowSummonPortal => 9689u16,
            Self::ForceGenerateGuidKey => 10391u16,
            Self::InstanceScope => 11458u16,
            Self::InstanceType => 11459u16,
            Self::IsInstance => 10062u16,
            Self::IsPhased => 5611u16,
            Self::IsUseMount => 12186u16,
            Self::JsonConfig => 10017u16,
            Self::NumInitialPooledItems => 11010u16,
            Self::NumPooledItemsBumpUp => 11009u16,
            Self::OnlySpawnToEntryPoint => 10895u16,
            Self::SpawnToTheLastSavePosition => 10894u16,
            Self::UseGuidAsKey => 9697u16,
            Self::ZoneType => 5612u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AllowSummonPortal => "allowSummonPortal",
            Self::ForceGenerateGuidKey => "forceGenerateGuidKey",
            Self::InstanceScope => "InstanceScope",
            Self::InstanceType => "InstanceType",
            Self::IsInstance => "isInstance",
            Self::IsPhased => "isPhased",
            Self::IsUseMount => "isUseMount",
            Self::JsonConfig => "jsonConfig",
            Self::NumInitialPooledItems => "numInitialPooledItems",
            Self::NumPooledItemsBumpUp => "numPooledItemsBumpUp",
            Self::OnlySpawnToEntryPoint => "OnlySpawnToEntryPoint",
            Self::SpawnToTheLastSavePosition => "SpawnToTheLastSavePosition",
            Self::UseGuidAsKey => "useGuidAsKey",
            Self::ZoneType => "zoneType",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AllowSummonPortal => ParamType::Bool,
            Self::ForceGenerateGuidKey => ParamType::Bool,
            Self::InstanceScope => ParamType::Int,
            Self::InstanceType => ParamType::Int,
            Self::IsInstance => ParamType::Bool,
            Self::IsPhased => ParamType::Bool,
            Self::IsUseMount => ParamType::Bool,
            Self::JsonConfig => ParamType::JsonValue,
            Self::NumInitialPooledItems => ParamType::Int,
            Self::NumPooledItemsBumpUp => ParamType::Int,
            Self::OnlySpawnToEntryPoint => ParamType::Bool,
            Self::SpawnToTheLastSavePosition => ParamType::Bool,
            Self::UseGuidAsKey => ParamType::Bool,
            Self::ZoneType => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static ALLOW_SUMMON_PORTAL: Value = Value::Bool(true);
        static FORCE_GENERATE_GUID_KEY: Value = Value::Bool(false);
        static INSTANCE_SCOPE: Value = Value::Int(0i32);
        static INSTANCE_TYPE: Value = Value::Int(0i32);
        static IS_INSTANCE: Value = Value::Bool(true);
        static IS_PHASED: Value = Value::Bool(false);
        static IS_USE_MOUNT: Value = Value::Bool(true);
        static JSON_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static NUM_INITIAL_POOLED_ITEMS: Value = Value::Int(0i32);
        static NUM_POOLED_ITEMS_BUMP_UP: Value = Value::Int(1i32);
        static ONLY_SPAWN_TO_ENTRY_POINT: Value = Value::Bool(false);
        static SPAWN_TO_THE_LAST_SAVE_POSITION: Value = Value::Bool(false);
        static USE_GUID_AS_KEY: Value = Value::Bool(true);
        static ZONE_TYPE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        match self {
            Self::AllowSummonPortal => &ALLOW_SUMMON_PORTAL,
            Self::ForceGenerateGuidKey => &FORCE_GENERATE_GUID_KEY,
            Self::InstanceScope => &INSTANCE_SCOPE,
            Self::InstanceType => &INSTANCE_TYPE,
            Self::IsInstance => &IS_INSTANCE,
            Self::IsPhased => &IS_PHASED,
            Self::IsUseMount => &IS_USE_MOUNT,
            Self::JsonConfig => &JSON_CONFIG,
            Self::NumInitialPooledItems => &NUM_INITIAL_POOLED_ITEMS,
            Self::NumPooledItemsBumpUp => &NUM_POOLED_ITEMS_BUMP_UP,
            Self::OnlySpawnToEntryPoint => &ONLY_SPAWN_TO_ENTRY_POINT,
            Self::SpawnToTheLastSavePosition => &SPAWN_TO_THE_LAST_SAVE_POSITION,
            Self::UseGuidAsKey => &USE_GUID_AS_KEY,
            Self::ZoneType => &ZONE_TYPE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AllowSummonPortal => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ForceGenerateGuidKey => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InstanceScope => &[ParamFlag::Content],
            Self::InstanceType => &[ParamFlag::Content],
            Self::IsInstance => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsPhased => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsUseMount => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::JsonConfig => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::NumInitialPooledItems => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::NumPooledItemsBumpUp => &[ParamFlag::Persistent],
            Self::OnlySpawnToEntryPoint => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SpawnToTheLastSavePosition => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::UseGuidAsKey => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ZoneType => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for OaZoneConfig {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OA_ZONE_CONFIG_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for OaZoneConfig {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            9689u16 => Ok(Self::AllowSummonPortal),
            10391u16 => Ok(Self::ForceGenerateGuidKey),
            11458u16 => Ok(Self::InstanceScope),
            11459u16 => Ok(Self::InstanceType),
            10062u16 => Ok(Self::IsInstance),
            5611u16 => Ok(Self::IsPhased),
            12186u16 => Ok(Self::IsUseMount),
            10017u16 => Ok(Self::JsonConfig),
            11010u16 => Ok(Self::NumInitialPooledItems),
            11009u16 => Ok(Self::NumPooledItemsBumpUp),
            10895u16 => Ok(Self::OnlySpawnToEntryPoint),
            10894u16 => Ok(Self::SpawnToTheLastSavePosition),
            9697u16 => Ok(Self::UseGuidAsKey),
            5612u16 => Ok(Self::ZoneType),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
