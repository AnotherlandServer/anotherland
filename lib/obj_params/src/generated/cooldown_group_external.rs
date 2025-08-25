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
pub enum CooldownGroupExternal {
    DisplayName,
    SubGroups,
    Cooldown,
    CooldownWhileOnlineOnly,
}
pub(crate) static COOLDOWN_GROUP_EXTERNAL_ATTRIBUTES: phf::Map<
    &'static str,
    CooldownGroupExternal,
> = phf_map! {
    "DisplayName" => CooldownGroupExternal::DisplayName, "SubGroups" =>
    CooldownGroupExternal::SubGroups, "Cooldown" => CooldownGroupExternal::Cooldown,
    "CooldownWhileOnlineOnly" => CooldownGroupExternal::CooldownWhileOnlineOnly,
};
pub(crate) static COOLDOWN_GROUP_EXTERNAL_ATTRIBUTES_ID: phf::Map<
    u16,
    CooldownGroupExternal,
> = phf_map! {
    11335u16 => CooldownGroupExternal::DisplayName, 11334u16 =>
    CooldownGroupExternal::SubGroups, 11336u16 => CooldownGroupExternal::Cooldown,
    11345u16 => CooldownGroupExternal::CooldownWhileOnlineOnly,
};
impl Attribute for CooldownGroupExternal {
    fn class() -> Class {
        Class::CooldownGroupExternal
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::DisplayName => &Self::DisplayName,
            Self::SubGroups => &Self::SubGroups,
            Self::Cooldown => &Self::Cooldown,
            Self::CooldownWhileOnlineOnly => &Self::CooldownWhileOnlineOnly,
        }
    }
}
impl AttributeInfo for CooldownGroupExternal {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::DisplayName => 11335u16,
            Self::SubGroups => 11334u16,
            Self::Cooldown => 11336u16,
            Self::CooldownWhileOnlineOnly => 11345u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::DisplayName => "DisplayName",
            Self::SubGroups => "SubGroups",
            Self::Cooldown => "Cooldown",
            Self::CooldownWhileOnlineOnly => "CooldownWhileOnlineOnly",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Cooldown => ParamType::Float,
            Self::CooldownWhileOnlineOnly => ParamType::Bool,
            Self::DisplayName => ParamType::String,
            Self::SubGroups => ParamType::ContentRefList,
        }
    }
    fn default(&self) -> &'static Value {
        static COOLDOWN: Value = Value::Float(0f32);
        static COOLDOWN_WHILE_ONLINE_ONLY: Value = Value::Bool(false);
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SUB_GROUPS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        match self {
            Self::Cooldown => &COOLDOWN,
            Self::CooldownWhileOnlineOnly => &COOLDOWN_WHILE_ONLINE_ONLY,
            Self::DisplayName => &DISPLAY_NAME,
            Self::SubGroups => &SUB_GROUPS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Cooldown => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CooldownWhileOnlineOnly => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SubGroups => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for CooldownGroupExternal {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COOLDOWN_GROUP_EXTERNAL_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CooldownGroupExternal {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11335u16 => Ok(Self::DisplayName),
            11334u16 => Ok(Self::SubGroups),
            11336u16 => Ok(Self::Cooldown),
            11345u16 => Ok(Self::CooldownWhileOnlineOnly),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
