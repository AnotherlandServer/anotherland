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
pub enum CombatSystemGroup {
    DisplayName,
    SubGroups,
}
pub(crate) static COMBAT_SYSTEM_GROUP_ATTRIBUTES: phf::Map<
    &'static str,
    CombatSystemGroup,
> = phf_map! {
    "DisplayName" => CombatSystemGroup::DisplayName, "SubGroups" =>
    CombatSystemGroup::SubGroups,
};
pub(crate) static COMBAT_SYSTEM_GROUP_ATTRIBUTES_ID: phf::Map<u16, CombatSystemGroup> = phf_map! {
    11319u16 => CombatSystemGroup::DisplayName, 11321u16 => CombatSystemGroup::SubGroups,
};
impl Attribute for CombatSystemGroup {
    fn class() -> Class {
        Class::CombatSystemGroup
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::DisplayName => &Self::DisplayName,
            Self::SubGroups => &Self::SubGroups,
        }
    }
}
impl AttributeInfo for CombatSystemGroup {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::DisplayName => 11319u16,
            Self::SubGroups => 11321u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::DisplayName => "DisplayName",
            Self::SubGroups => "SubGroups",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::DisplayName => ParamType::String,
            Self::SubGroups => ParamType::ContentRefList,
        }
    }
    fn default(&self) -> &'static Value {
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SUB_GROUPS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        match self {
            Self::DisplayName => &DISPLAY_NAME,
            Self::SubGroups => &SUB_GROUPS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SubGroups => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for CombatSystemGroup {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMBAT_SYSTEM_GROUP_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CombatSystemGroup {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11319u16 => Ok(Self::DisplayName),
            11321u16 => Ok(Self::SubGroups),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
