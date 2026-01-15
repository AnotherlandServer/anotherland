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
pub enum NpcShopConfig {
    SellCategories,
    SellExacts,
}
pub(crate) static NPC_SHOP_CONFIG_ATTRIBUTES: phf::Map<&'static str, NpcShopConfig> = phf_map! {
    "sellCategories" => NpcShopConfig::SellCategories, "sellExacts" =>
    NpcShopConfig::SellExacts,
};
pub(crate) static NPC_SHOP_CONFIG_ATTRIBUTES_ID: phf::Map<u16, NpcShopConfig> = phf_map! {
    11397u16 => NpcShopConfig::SellCategories, 11396u16 => NpcShopConfig::SellExacts,
};
impl Attribute for NpcShopConfig {
    fn class() -> Class {
        Class::NpcShopConfig
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::SellCategories => &Self::SellCategories,
            Self::SellExacts => &Self::SellExacts,
        }
    }
}
impl AttributeInfo for NpcShopConfig {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::SellCategories => 11397u16,
            Self::SellExacts => 11396u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::SellCategories => "sellCategories",
            Self::SellExacts => "sellExacts",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::SellCategories => ParamType::JsonValue,
            Self::SellExacts => ParamType::ContentRefList,
        }
    }
    fn default(&self) -> &'static Value {
        static SELL_CATEGORIES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static SELL_EXACTS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        match self {
            Self::SellCategories => &SELL_CATEGORIES,
            Self::SellExacts => &SELL_EXACTS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::SellCategories => &[ParamFlag::Content],
            Self::SellExacts => &[ParamFlag::Content],
        }
    }
}
impl FromStr for NpcShopConfig {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NPC_SHOP_CONFIG_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for NpcShopConfig {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11397u16 => Ok(Self::SellCategories),
            11396u16 => Ok(Self::SellExacts),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
