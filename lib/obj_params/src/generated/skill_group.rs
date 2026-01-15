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
pub enum SkillGroup {
    DisplayName,
    SubGroups,
    CooldownWhileOnlineOnly,
    InternalCooldownAsOtherGroup,
}
pub(crate) static SKILL_GROUP_ATTRIBUTES: phf::Map<&'static str, SkillGroup> = phf_map! {
    "DisplayName" => SkillGroup::DisplayName, "SubGroups" => SkillGroup::SubGroups,
    "CooldownWhileOnlineOnly" => SkillGroup::CooldownWhileOnlineOnly,
    "InternalCooldownAsOtherGroup" => SkillGroup::InternalCooldownAsOtherGroup,
};
pub(crate) static SKILL_GROUP_ATTRIBUTES_ID: phf::Map<u16, SkillGroup> = phf_map! {
    11329u16 => SkillGroup::DisplayName, 11328u16 => SkillGroup::SubGroups, 11343u16 =>
    SkillGroup::CooldownWhileOnlineOnly, 11333u16 =>
    SkillGroup::InternalCooldownAsOtherGroup,
};
impl Attribute for SkillGroup {
    fn class() -> Class {
        Class::SkillGroup
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::DisplayName => &Self::DisplayName,
            Self::SubGroups => &Self::SubGroups,
            Self::CooldownWhileOnlineOnly => &Self::CooldownWhileOnlineOnly,
            Self::InternalCooldownAsOtherGroup => &Self::InternalCooldownAsOtherGroup,
        }
    }
}
impl AttributeInfo for SkillGroup {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::DisplayName => 11329u16,
            Self::SubGroups => 11328u16,
            Self::CooldownWhileOnlineOnly => 11343u16,
            Self::InternalCooldownAsOtherGroup => 11333u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::DisplayName => "DisplayName",
            Self::SubGroups => "SubGroups",
            Self::CooldownWhileOnlineOnly => "CooldownWhileOnlineOnly",
            Self::InternalCooldownAsOtherGroup => "InternalCooldownAsOtherGroup",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::CooldownWhileOnlineOnly => ParamType::Bool,
            Self::InternalCooldownAsOtherGroup => ParamType::ContentRef,
            Self::DisplayName => ParamType::String,
            Self::SubGroups => ParamType::ContentRefList,
        }
    }
    fn default(&self) -> &'static Value {
        static COOLDOWN_WHILE_ONLINE_ONLY: Value = Value::Bool(false);
        static INTERNAL_COOLDOWN_AS_OTHER_GROUP: Lazy<Value> = Lazy::new(|| Value::ContentRef(
            None,
        ));
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SUB_GROUPS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        match self {
            Self::CooldownWhileOnlineOnly => &COOLDOWN_WHILE_ONLINE_ONLY,
            Self::InternalCooldownAsOtherGroup => &INTERNAL_COOLDOWN_AS_OTHER_GROUP,
            Self::DisplayName => &DISPLAY_NAME,
            Self::SubGroups => &SUB_GROUPS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::CooldownWhileOnlineOnly => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InternalCooldownAsOtherGroup => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SubGroups => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for SkillGroup {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SKILL_GROUP_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for SkillGroup {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11329u16 => Ok(Self::DisplayName),
            11328u16 => Ok(Self::SubGroups),
            11343u16 => Ok(Self::CooldownWhileOnlineOnly),
            11333u16 => Ok(Self::InternalCooldownAsOtherGroup),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
