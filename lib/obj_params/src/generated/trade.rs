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
pub enum Trade {
    A1Item1,
    A1Item2,
    A1Item3,
    A1Item4,
    A1Item5,
    A1Item6,
    A2Item1,
    A2Item2,
    A2Item3,
    A2Item4,
    A2Item5,
    A2Item6,
    Avatar1,
    Avatar1Ok,
    Avatar2,
    Avatar2Ok,
    Freq,
    Power,
}
pub(crate) static TRADE_ATTRIBUTES: phf::Map<&'static str, Trade> = phf_map! {
    "a1_item1" => Trade::A1Item1, "a1_item2" => Trade::A1Item2, "a1_item3" =>
    Trade::A1Item3, "a1_item4" => Trade::A1Item4, "a1_item5" => Trade::A1Item5,
    "a1_item6" => Trade::A1Item6, "a2_item1" => Trade::A2Item1, "a2_item2" =>
    Trade::A2Item2, "a2_item3" => Trade::A2Item3, "a2_item4" => Trade::A2Item4,
    "a2_item5" => Trade::A2Item5, "a2_item6" => Trade::A2Item6, "avatar1" =>
    Trade::Avatar1, "avatar1_ok" => Trade::Avatar1Ok, "avatar2" => Trade::Avatar2,
    "avatar2_ok" => Trade::Avatar2Ok, "Freq" => Trade::Freq, "Power" => Trade::Power,
};
pub(crate) static TRADE_ATTRIBUTES_ID: phf::Map<u16, Trade> = phf_map! {
    2983u16 => Trade::A1Item1, 2984u16 => Trade::A1Item2, 2985u16 => Trade::A1Item3,
    2986u16 => Trade::A1Item4, 2987u16 => Trade::A1Item5, 2988u16 => Trade::A1Item6,
    2990u16 => Trade::A2Item1, 2991u16 => Trade::A2Item2, 2992u16 => Trade::A2Item3,
    2993u16 => Trade::A2Item4, 2994u16 => Trade::A2Item5, 2995u16 => Trade::A2Item6,
    2997u16 => Trade::Avatar1, 2998u16 => Trade::Avatar1Ok, 2999u16 => Trade::Avatar2,
    3000u16 => Trade::Avatar2Ok, 3002u16 => Trade::Freq, 3001u16 => Trade::Power,
};
impl Attribute for Trade {
    fn class() -> Class {
        Class::Trade
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::A1Item1 => &Self::A1Item1,
            Self::A1Item2 => &Self::A1Item2,
            Self::A1Item3 => &Self::A1Item3,
            Self::A1Item4 => &Self::A1Item4,
            Self::A1Item5 => &Self::A1Item5,
            Self::A1Item6 => &Self::A1Item6,
            Self::A2Item1 => &Self::A2Item1,
            Self::A2Item2 => &Self::A2Item2,
            Self::A2Item3 => &Self::A2Item3,
            Self::A2Item4 => &Self::A2Item4,
            Self::A2Item5 => &Self::A2Item5,
            Self::A2Item6 => &Self::A2Item6,
            Self::Avatar1 => &Self::Avatar1,
            Self::Avatar1Ok => &Self::Avatar1Ok,
            Self::Avatar2 => &Self::Avatar2,
            Self::Avatar2Ok => &Self::Avatar2Ok,
            Self::Freq => &Self::Freq,
            Self::Power => &Self::Power,
        }
    }
}
impl AttributeInfo for Trade {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::A1Item1 => 2983u16,
            Self::A1Item2 => 2984u16,
            Self::A1Item3 => 2985u16,
            Self::A1Item4 => 2986u16,
            Self::A1Item5 => 2987u16,
            Self::A1Item6 => 2988u16,
            Self::A2Item1 => 2990u16,
            Self::A2Item2 => 2991u16,
            Self::A2Item3 => 2992u16,
            Self::A2Item4 => 2993u16,
            Self::A2Item5 => 2994u16,
            Self::A2Item6 => 2995u16,
            Self::Avatar1 => 2997u16,
            Self::Avatar1Ok => 2998u16,
            Self::Avatar2 => 2999u16,
            Self::Avatar2Ok => 3000u16,
            Self::Freq => 3002u16,
            Self::Power => 3001u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::A1Item1 => "a1_item1",
            Self::A1Item2 => "a1_item2",
            Self::A1Item3 => "a1_item3",
            Self::A1Item4 => "a1_item4",
            Self::A1Item5 => "a1_item5",
            Self::A1Item6 => "a1_item6",
            Self::A2Item1 => "a2_item1",
            Self::A2Item2 => "a2_item2",
            Self::A2Item3 => "a2_item3",
            Self::A2Item4 => "a2_item4",
            Self::A2Item5 => "a2_item5",
            Self::A2Item6 => "a2_item6",
            Self::Avatar1 => "avatar1",
            Self::Avatar1Ok => "avatar1_ok",
            Self::Avatar2 => "avatar2",
            Self::Avatar2Ok => "avatar2_ok",
            Self::Freq => "Freq",
            Self::Power => "Power",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::A1Item1 => ParamType::Guid,
            Self::A1Item2 => ParamType::Guid,
            Self::A1Item3 => ParamType::Guid,
            Self::A1Item4 => ParamType::Guid,
            Self::A1Item5 => ParamType::Guid,
            Self::A1Item6 => ParamType::Guid,
            Self::A2Item1 => ParamType::Guid,
            Self::A2Item2 => ParamType::Guid,
            Self::A2Item3 => ParamType::Guid,
            Self::A2Item4 => ParamType::Guid,
            Self::A2Item5 => ParamType::Guid,
            Self::A2Item6 => ParamType::Guid,
            Self::Avatar1 => ParamType::AvatarId,
            Self::Avatar1Ok => ParamType::Bool,
            Self::Avatar2 => ParamType::AvatarId,
            Self::Avatar2Ok => ParamType::Bool,
            Self::Freq => ParamType::Int,
            Self::Power => ParamType::Int,
        }
    }
    fn default(&self) -> &'static Value {
        static A_1_ITEM_1: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_1_ITEM_2: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_1_ITEM_3: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_1_ITEM_4: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_1_ITEM_5: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_1_ITEM_6: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_1: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_2: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_3: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_4: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_5: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static A_2_ITEM_6: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static AVATAR_1: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static AVATAR_1_OK: Value = Value::Bool(false);
        static AVATAR_2: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static AVATAR_2_OK: Value = Value::Bool(false);
        static FREQ: Value = Value::Int(0i32);
        static POWER: Value = Value::Int(0i32);
        match self {
            Self::A1Item1 => &A_1_ITEM_1,
            Self::A1Item2 => &A_1_ITEM_2,
            Self::A1Item3 => &A_1_ITEM_3,
            Self::A1Item4 => &A_1_ITEM_4,
            Self::A1Item5 => &A_1_ITEM_5,
            Self::A1Item6 => &A_1_ITEM_6,
            Self::A2Item1 => &A_2_ITEM_1,
            Self::A2Item2 => &A_2_ITEM_2,
            Self::A2Item3 => &A_2_ITEM_3,
            Self::A2Item4 => &A_2_ITEM_4,
            Self::A2Item5 => &A_2_ITEM_5,
            Self::A2Item6 => &A_2_ITEM_6,
            Self::Avatar1 => &AVATAR_1,
            Self::Avatar1Ok => &AVATAR_1_OK,
            Self::Avatar2 => &AVATAR_2,
            Self::Avatar2Ok => &AVATAR_2_OK,
            Self::Freq => &FREQ,
            Self::Power => &POWER,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::A1Item1 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A1Item2 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A1Item3 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A1Item4 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A1Item5 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A1Item6 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item1 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item2 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item3 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item4 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item5 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::A2Item6 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Avatar1 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Avatar1Ok => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Avatar2 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Avatar2Ok => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for Trade {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TRADE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Trade {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            2983u16 => Ok(Self::A1Item1),
            2984u16 => Ok(Self::A1Item2),
            2985u16 => Ok(Self::A1Item3),
            2986u16 => Ok(Self::A1Item4),
            2987u16 => Ok(Self::A1Item5),
            2988u16 => Ok(Self::A1Item6),
            2990u16 => Ok(Self::A2Item1),
            2991u16 => Ok(Self::A2Item2),
            2992u16 => Ok(Self::A2Item3),
            2993u16 => Ok(Self::A2Item4),
            2994u16 => Ok(Self::A2Item5),
            2995u16 => Ok(Self::A2Item6),
            2997u16 => Ok(Self::Avatar1),
            2998u16 => Ok(Self::Avatar1Ok),
            2999u16 => Ok(Self::Avatar2),
            3000u16 => Ok(Self::Avatar2Ok),
            3002u16 => Ok(Self::Freq),
            3001u16 => Ok(Self::Power),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
