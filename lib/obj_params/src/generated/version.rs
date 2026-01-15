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
pub enum Version {
    Description,
    VersionNumber,
}
pub(crate) static VERSION_ATTRIBUTES: phf::Map<&'static str, Version> = phf_map! {
    "description" => Version::Description, "versionNumber" => Version::VersionNumber,
};
pub(crate) static VERSION_ATTRIBUTES_ID: phf::Map<u16, Version> = phf_map! {
    3027u16 => Version::Description, 3026u16 => Version::VersionNumber,
};
impl Attribute for Version {
    fn class() -> Class {
        Class::Version
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Description => &Self::Description,
            Self::VersionNumber => &Self::VersionNumber,
        }
    }
}
impl AttributeInfo for Version {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Description => 3027u16,
            Self::VersionNumber => 3026u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Description => "description",
            Self::VersionNumber => "versionNumber",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Description => ParamType::String,
            Self::VersionNumber => ParamType::Int,
        }
    }
    fn default(&self) -> &'static Value {
        static DESCRIPTION: Lazy<Value> = Lazy::new(|| Value::String(
            "This is the default version description".to_string(),
        ));
        static VERSION_NUMBER: Value = Value::Int(1i32);
        match self {
            Self::Description => &DESCRIPTION,
            Self::VersionNumber => &VERSION_NUMBER,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Description => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::VersionNumber => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for Version {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VERSION_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Version {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            3027u16 => Ok(Self::Description),
            3026u16 => Ok(Self::VersionNumber),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
