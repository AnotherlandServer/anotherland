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
pub enum Faction {
    Description,
    DisplayName,
    Icon,
    IsReputation,
    JsonConfig,
    Relations,
}
pub(crate) static FACTION_ATTRIBUTES: phf::Map<&'static str, Faction> = phf_map! {
    "Description" => Faction::Description, "DisplayName" => Faction::DisplayName, "Icon"
    => Faction::Icon, "IsReputation" => Faction::IsReputation, "JsonConfig" =>
    Faction::JsonConfig, "Relations" => Faction::Relations,
};
pub(crate) static FACTION_ATTRIBUTES_ID: phf::Map<u16, Faction> = phf_map! {
    12184u16 => Faction::Description, 9360u16 => Faction::DisplayName, 12183u16 =>
    Faction::Icon, 12182u16 => Faction::IsReputation, 12189u16 => Faction::JsonConfig,
    9361u16 => Faction::Relations,
};
impl Attribute for Faction {
    fn class() -> Class {
        Class::Faction
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Description => &Self::Description,
            Self::DisplayName => &Self::DisplayName,
            Self::Icon => &Self::Icon,
            Self::IsReputation => &Self::IsReputation,
            Self::JsonConfig => &Self::JsonConfig,
            Self::Relations => &Self::Relations,
        }
    }
}
impl AttributeInfo for Faction {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Description => 12184u16,
            Self::DisplayName => 9360u16,
            Self::Icon => 12183u16,
            Self::IsReputation => 12182u16,
            Self::JsonConfig => 12189u16,
            Self::Relations => 9361u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Description => "Description",
            Self::DisplayName => "DisplayName",
            Self::Icon => "Icon",
            Self::IsReputation => "IsReputation",
            Self::JsonConfig => "JsonConfig",
            Self::Relations => "Relations",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Description => ParamType::LocalizedString,
            Self::DisplayName => ParamType::LocalizedString,
            Self::Icon => ParamType::String,
            Self::IsReputation => ParamType::Bool,
            Self::JsonConfig => ParamType::JsonValue,
            Self::Relations => ParamType::JsonValue,
        }
    }
    fn default(&self) -> &'static Value {
        static DESCRIPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static IS_REPUTATION: Value = Value::Bool(false);
        static JSON_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static RELATIONS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        match self {
            Self::Description => &DESCRIPTION,
            Self::DisplayName => &DISPLAY_NAME,
            Self::Icon => &ICON,
            Self::IsReputation => &IS_REPUTATION,
            Self::JsonConfig => &JSON_CONFIG,
            Self::Relations => &RELATIONS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Description => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsReputation => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::JsonConfig => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Relations => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for Faction {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FACTION_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Faction {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12184u16 => Ok(Self::Description),
            9360u16 => Ok(Self::DisplayName),
            12183u16 => Ok(Self::Icon),
            12182u16 => Ok(Self::IsReputation),
            12189u16 => Ok(Self::JsonConfig),
            9361u16 => Ok(Self::Relations),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
