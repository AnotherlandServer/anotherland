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
pub enum Metagame {}
pub(crate) static METAGAME_ATTRIBUTES: phf::Map<&'static str, Metagame> = phf_map! {};
pub(crate) static METAGAME_ATTRIBUTES_ID: phf::Map<u16, Metagame> = phf_map! {};
impl Attribute for Metagame {
    fn class() -> Class {
        Class::Metagame
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        unreachable!()
    }
}
impl AttributeInfo for Metagame {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        unreachable!()
    }
    fn name(&self) -> &'static str {
        unreachable!()
    }
    fn datatype(&self) -> ParamType {
        unreachable!()
    }
    fn default(&self) -> &'static Value {
        unreachable!()
    }
    fn flags(&self) -> &[ParamFlag] {
        unreachable!()
    }
}
impl FromStr for Metagame {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        METAGAME_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Metagame {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
