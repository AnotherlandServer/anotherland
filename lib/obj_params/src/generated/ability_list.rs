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
pub enum AbilityList {
    AbilityInfo,
    AbilityName,
    TargetAbilityInfo,
}
pub(crate) static ABILITY_LIST_ATTRIBUTES: phf::Map<&'static str, AbilityList> = phf_map! {
    "abilityInfo" => AbilityList::AbilityInfo, "abilityName" => AbilityList::AbilityName,
    "targetAbilityInfo" => AbilityList::TargetAbilityInfo,
};
pub(crate) static ABILITY_LIST_ATTRIBUTES_ID: phf::Map<u16, AbilityList> = phf_map! {
    9688u16 => AbilityList::AbilityInfo, 9687u16 => AbilityList::AbilityName, 9686u16 =>
    AbilityList::TargetAbilityInfo,
};
impl Attribute for AbilityList {
    fn class() -> Class {
        Class::AbilityList
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AbilityInfo => &Self::AbilityInfo,
            Self::AbilityName => &Self::AbilityName,
            Self::TargetAbilityInfo => &Self::TargetAbilityInfo,
        }
    }
}
impl AttributeInfo for AbilityList {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AbilityInfo => 9688u16,
            Self::AbilityName => 9687u16,
            Self::TargetAbilityInfo => 9686u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AbilityInfo => "abilityInfo",
            Self::AbilityName => "abilityName",
            Self::TargetAbilityInfo => "targetAbilityInfo",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AbilityInfo => ParamType::String,
            Self::AbilityName => ParamType::String,
            Self::TargetAbilityInfo => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static ABILITY_INFO: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ABILITY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static TARGET_ABILITY_INFO: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        match self {
            Self::AbilityInfo => &ABILITY_INFO,
            Self::AbilityName => &ABILITY_NAME,
            Self::TargetAbilityInfo => &TARGET_ABILITY_INFO,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AbilityInfo => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AbilityName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TargetAbilityInfo => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for AbilityList {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ABILITY_LIST_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for AbilityList {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            9688u16 => Ok(Self::AbilityInfo),
            9687u16 => Ok(Self::AbilityName),
            9686u16 => Ok(Self::TargetAbilityInfo),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
