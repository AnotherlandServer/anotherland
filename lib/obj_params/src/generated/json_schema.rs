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
pub enum JsonSchema {
    Value,
}
pub(crate) static JSON_SCHEMA_ATTRIBUTES: phf::Map<&'static str, JsonSchema> = phf_map! {
    "value" => JsonSchema::Value,
};
pub(crate) static JSON_SCHEMA_ATTRIBUTES_ID: phf::Map<u16, JsonSchema> = phf_map! {
    4289u16 => JsonSchema::Value,
};
impl Attribute for JsonSchema {
    fn class() -> Class {
        Class::JsonSchema
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Value => &Self::Value,
        }
    }
}
impl AttributeInfo for JsonSchema {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Value => 4289u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Value => "value",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Value => ParamType::JsonValue,
        }
    }
    fn default(&self) -> &'static Value {
        static VALUE: Lazy<Value> = Lazy::new(|| Value::JsonValue(JsonValue::default()));
        match self {
            Self::Value => &VALUE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Value => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for JsonSchema {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JSON_SCHEMA_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for JsonSchema {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            4289u16 => Ok(Self::Value),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
