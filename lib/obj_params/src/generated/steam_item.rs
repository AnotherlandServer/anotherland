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
pub enum SteamItem {
    Amount,
    Category,
    Credits,
    Description,
    Icon,
    IsActive,
    ItemId,
    Name,
}
pub(crate) static STEAM_ITEM_ATTRIBUTES: phf::Map<&'static str, SteamItem> = phf_map! {
    "Amount" => SteamItem::Amount, "Category" => SteamItem::Category, "Credits" =>
    SteamItem::Credits, "Description" => SteamItem::Description, "Icon" =>
    SteamItem::Icon, "IsActive" => SteamItem::IsActive, "itemId" => SteamItem::ItemId,
    "Name" => SteamItem::Name,
};
pub(crate) static STEAM_ITEM_ATTRIBUTES_ID: phf::Map<u16, SteamItem> = phf_map! {
    12131u16 => SteamItem::Amount, 12130u16 => SteamItem::Category, 12136u16 =>
    SteamItem::Credits, 12129u16 => SteamItem::Description, 12138u16 => SteamItem::Icon,
    12127u16 => SteamItem::IsActive, 12139u16 => SteamItem::ItemId, 12137u16 =>
    SteamItem::Name,
};
impl Attribute for SteamItem {
    fn class() -> Class {
        Class::SteamItem
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Amount => &Self::Amount,
            Self::Category => &Self::Category,
            Self::Credits => &Self::Credits,
            Self::Description => &Self::Description,
            Self::Icon => &Self::Icon,
            Self::IsActive => &Self::IsActive,
            Self::ItemId => &Self::ItemId,
            Self::Name => &Self::Name,
        }
    }
}
impl AttributeInfo for SteamItem {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Amount => 12131u16,
            Self::Category => 12130u16,
            Self::Credits => 12136u16,
            Self::Description => 12129u16,
            Self::Icon => 12138u16,
            Self::IsActive => 12127u16,
            Self::ItemId => 12139u16,
            Self::Name => 12137u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Amount => "Amount",
            Self::Category => "Category",
            Self::Credits => "Credits",
            Self::Description => "Description",
            Self::Icon => "Icon",
            Self::IsActive => "IsActive",
            Self::ItemId => "itemId",
            Self::Name => "Name",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Amount => ParamType::Int,
            Self::Category => ParamType::LocalizedString,
            Self::Credits => ParamType::Int,
            Self::Description => ParamType::LocalizedString,
            Self::Icon => ParamType::String,
            Self::IsActive => ParamType::Bool,
            Self::ItemId => ParamType::Int,
            Self::Name => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static AMOUNT: Value = Value::Int(0i32);
        static CATEGORY: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static CREDITS: Value = Value::Int(0i32);
        static DESCRIPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static IS_ACTIVE: Value = Value::Bool(false);
        static ITEM_ID: Value = Value::Int(0i32);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        match self {
            Self::Amount => &AMOUNT,
            Self::Category => &CATEGORY,
            Self::Credits => &CREDITS,
            Self::Description => &DESCRIPTION,
            Self::Icon => &ICON,
            Self::IsActive => &IS_ACTIVE,
            Self::ItemId => &ITEM_ID,
            Self::Name => &NAME,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Amount => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Category => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Credits => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Description => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsActive => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ItemId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for SteamItem {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        STEAM_ITEM_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for SteamItem {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12131u16 => Ok(Self::Amount),
            12130u16 => Ok(Self::Category),
            12136u16 => Ok(Self::Credits),
            12129u16 => Ok(Self::Description),
            12138u16 => Ok(Self::Icon),
            12127u16 => Ok(Self::IsActive),
            12139u16 => Ok(Self::ItemId),
            12137u16 => Ok(Self::Name),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
