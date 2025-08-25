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
pub enum ItemPreset {
    Presets,
    Race,
    SlotMapping,
}
pub(crate) static ITEM_PRESET_ATTRIBUTES: phf::Map<&'static str, ItemPreset> = phf_map! {
    "presets" => ItemPreset::Presets, "race" => ItemPreset::Race, "slotMapping" =>
    ItemPreset::SlotMapping,
};
pub(crate) static ITEM_PRESET_ATTRIBUTES_ID: phf::Map<u16, ItemPreset> = phf_map! {
    6234u16 => ItemPreset::Presets, 6236u16 => ItemPreset::Race, 6237u16 =>
    ItemPreset::SlotMapping,
};
impl Attribute for ItemPreset {
    fn class() -> Class {
        Class::ItemPreset
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Presets => &Self::Presets,
            Self::Race => &Self::Race,
            Self::SlotMapping => &Self::SlotMapping,
        }
    }
}
impl AttributeInfo for ItemPreset {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Presets => 6234u16,
            Self::Race => 6236u16,
            Self::SlotMapping => 6237u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Presets => "presets",
            Self::Race => "race",
            Self::SlotMapping => "slotMapping",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Presets => ParamType::JsonValue,
            Self::Race => ParamType::String,
            Self::SlotMapping => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static PRESETS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static RACE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SLOT_MAPPING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        match self {
            Self::Presets => &PRESETS,
            Self::Race => &RACE,
            Self::SlotMapping => &SLOT_MAPPING,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Presets => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Race => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SlotMapping => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for ItemPreset {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ITEM_PRESET_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ItemPreset {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            6234u16 => Ok(Self::Presets),
            6236u16 => Ok(Self::Race),
            6237u16 => Ok(Self::SlotMapping),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
