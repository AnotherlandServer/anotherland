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
pub enum LootSystem {
    LootedNpcBlingBase,
    LootedNpcBlingPerLevelIncrement,
    LootedNpcBlingPerLevelQuadraticCoeff,
    LootedNpcBlingVariance,
}
pub(crate) static LOOT_SYSTEM_ATTRIBUTES: phf::Map<&'static str, LootSystem> = phf_map! {
    "lootedNPCBlingBase" => LootSystem::LootedNpcBlingBase,
    "lootedNPCBlingPerLevelIncrement" => LootSystem::LootedNpcBlingPerLevelIncrement,
    "lootedNPCBlingPerLevelQuadraticCoeff" =>
    LootSystem::LootedNpcBlingPerLevelQuadraticCoeff, "lootedNPCBlingVariance" =>
    LootSystem::LootedNpcBlingVariance,
};
pub(crate) static LOOT_SYSTEM_ATTRIBUTES_ID: phf::Map<u16, LootSystem> = phf_map! {
    7067u16 => LootSystem::LootedNpcBlingBase, 7066u16 =>
    LootSystem::LootedNpcBlingPerLevelIncrement, 7068u16 =>
    LootSystem::LootedNpcBlingPerLevelQuadraticCoeff, 7065u16 =>
    LootSystem::LootedNpcBlingVariance,
};
impl Attribute for LootSystem {
    fn class() -> Class {
        Class::LootSystem
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::LootedNpcBlingBase => &Self::LootedNpcBlingBase,
            Self::LootedNpcBlingPerLevelIncrement => {
                &Self::LootedNpcBlingPerLevelIncrement
            }
            Self::LootedNpcBlingPerLevelQuadraticCoeff => {
                &Self::LootedNpcBlingPerLevelQuadraticCoeff
            }
            Self::LootedNpcBlingVariance => &Self::LootedNpcBlingVariance,
        }
    }
}
impl AttributeInfo for LootSystem {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::LootedNpcBlingBase => 7067u16,
            Self::LootedNpcBlingPerLevelIncrement => 7066u16,
            Self::LootedNpcBlingPerLevelQuadraticCoeff => 7068u16,
            Self::LootedNpcBlingVariance => 7065u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::LootedNpcBlingBase => "lootedNPCBlingBase",
            Self::LootedNpcBlingPerLevelIncrement => "lootedNPCBlingPerLevelIncrement",
            Self::LootedNpcBlingPerLevelQuadraticCoeff => {
                "lootedNPCBlingPerLevelQuadraticCoeff"
            }
            Self::LootedNpcBlingVariance => "lootedNPCBlingVariance",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::LootedNpcBlingBase => ParamType::Float,
            Self::LootedNpcBlingPerLevelIncrement => ParamType::Float,
            Self::LootedNpcBlingPerLevelQuadraticCoeff => ParamType::Float,
            Self::LootedNpcBlingVariance => ParamType::Float,
        }
    }
    fn default(&self) -> &'static Value {
        static LOOTED_NPC_BLING_BASE: Value = Value::Float(2f32);
        static LOOTED_NPC_BLING_PER_LEVEL_INCREMENT: Value = Value::Float(3f32);
        static LOOTED_NPC_BLING_PER_LEVEL_QUADRATIC_COEFF: Value = Value::Float(0.2f32);
        static LOOTED_NPC_BLING_VARIANCE: Value = Value::Float(0.12f32);
        match self {
            Self::LootedNpcBlingBase => &LOOTED_NPC_BLING_BASE,
            Self::LootedNpcBlingPerLevelIncrement => {
                &LOOTED_NPC_BLING_PER_LEVEL_INCREMENT
            }
            Self::LootedNpcBlingPerLevelQuadraticCoeff => {
                &LOOTED_NPC_BLING_PER_LEVEL_QUADRATIC_COEFF
            }
            Self::LootedNpcBlingVariance => &LOOTED_NPC_BLING_VARIANCE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::LootedNpcBlingBase => {
                &[ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::LootedNpcBlingPerLevelIncrement => {
                &[ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::LootedNpcBlingPerLevelQuadraticCoeff => {
                &[ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::LootedNpcBlingVariance => {
                &[ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
        }
    }
}
impl FromStr for LootSystem {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LOOT_SYSTEM_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for LootSystem {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            7067u16 => Ok(Self::LootedNpcBlingBase),
            7066u16 => Ok(Self::LootedNpcBlingPerLevelIncrement),
            7068u16 => Ok(Self::LootedNpcBlingPerLevelQuadraticCoeff),
            7065u16 => Ok(Self::LootedNpcBlingVariance),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
