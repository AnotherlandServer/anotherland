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
pub enum SteamDlc {
    CommandValue,
    DlcId,
    IsActiveDlc,
    Name,
}
pub(crate) static STEAM_DLC_ATTRIBUTES: phf::Map<&'static str, SteamDlc> = phf_map! {
    "CommandValue" => SteamDlc::CommandValue, "DlcID" => SteamDlc::DlcId, "IsActiveDlc"
    => SteamDlc::IsActiveDlc, "Name" => SteamDlc::Name,
};
pub(crate) static STEAM_DLC_ATTRIBUTES_ID: phf::Map<u16, SteamDlc> = phf_map! {
    12110u16 => SteamDlc::CommandValue, 12112u16 => SteamDlc::DlcId, 12113u16 =>
    SteamDlc::IsActiveDlc, 12111u16 => SteamDlc::Name,
};
impl Attribute for SteamDlc {
    fn class() -> Class {
        Class::SteamDlc
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::CommandValue => &Self::CommandValue,
            Self::DlcId => &Self::DlcId,
            Self::IsActiveDlc => &Self::IsActiveDlc,
            Self::Name => &Self::Name,
        }
    }
}
impl AttributeInfo for SteamDlc {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::CommandValue => 12110u16,
            Self::DlcId => 12112u16,
            Self::IsActiveDlc => 12113u16,
            Self::Name => 12111u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::CommandValue => "CommandValue",
            Self::DlcId => "DlcID",
            Self::IsActiveDlc => "IsActiveDlc",
            Self::Name => "Name",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::CommandValue => ParamType::JsonValue,
            Self::DlcId => ParamType::Int,
            Self::IsActiveDlc => ParamType::Bool,
            Self::Name => ParamType::LocalizedString,
        }
    }
    fn default(&self) -> &'static Value {
        static COMMAND_VALUE: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static DLC_ID: Value = Value::Int(0i32);
        static IS_ACTIVE_DLC: Value = Value::Bool(false);
        static NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        match self {
            Self::CommandValue => &COMMAND_VALUE,
            Self::DlcId => &DLC_ID,
            Self::IsActiveDlc => &IS_ACTIVE_DLC,
            Self::Name => &NAME,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::CommandValue => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DlcId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsActiveDlc => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for SteamDlc {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        STEAM_DLC_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for SteamDlc {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12110u16 => Ok(Self::CommandValue),
            12112u16 => Ok(Self::DlcId),
            12113u16 => Ok(Self::IsActiveDlc),
            12111u16 => Ok(Self::Name),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
