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
pub enum AiTemplate {
    Abilities,
    DefaultWeapon,
    DisplayName,
    Icon,
    ModDamage,
    ModSize,
    ModStatHealth,
    ModStatMovement,
    SeekHelpBelowHp,
    SeekHelpExecutionRadius,
    SeekHelpWithinRadius,
}
pub(crate) static AI_TEMPLATE_ATTRIBUTES: phf::Map<&'static str, AiTemplate> = phf_map! {
    "abilities" => AiTemplate::Abilities, "DefaultWeapon" => AiTemplate::DefaultWeapon,
    "DisplayName" => AiTemplate::DisplayName, "Icon" => AiTemplate::Icon, "modDamage" =>
    AiTemplate::ModDamage, "modSize" => AiTemplate::ModSize, "modStatHealth" =>
    AiTemplate::ModStatHealth, "modStatMovement" => AiTemplate::ModStatMovement,
    "SeekHelpBelowHP" => AiTemplate::SeekHelpBelowHp, "SeekHelpExecutionRadius" =>
    AiTemplate::SeekHelpExecutionRadius, "SeekHelpWithinRadius" =>
    AiTemplate::SeekHelpWithinRadius,
};
pub(crate) static AI_TEMPLATE_ATTRIBUTES_ID: phf::Map<u16, AiTemplate> = phf_map! {
    8823u16 => AiTemplate::Abilities, 8888u16 => AiTemplate::DefaultWeapon, 8816u16 =>
    AiTemplate::DisplayName, 8935u16 => AiTemplate::Icon, 8986u16 =>
    AiTemplate::ModDamage, 8817u16 => AiTemplate::ModSize, 8815u16 =>
    AiTemplate::ModStatHealth, 8822u16 => AiTemplate::ModStatMovement, 8887u16 =>
    AiTemplate::SeekHelpBelowHp, 8886u16 => AiTemplate::SeekHelpExecutionRadius, 8885u16
    => AiTemplate::SeekHelpWithinRadius,
};
impl Attribute for AiTemplate {
    fn class() -> Class {
        Class::AiTemplate
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Abilities => &Self::Abilities,
            Self::DefaultWeapon => &Self::DefaultWeapon,
            Self::DisplayName => &Self::DisplayName,
            Self::Icon => &Self::Icon,
            Self::ModDamage => &Self::ModDamage,
            Self::ModSize => &Self::ModSize,
            Self::ModStatHealth => &Self::ModStatHealth,
            Self::ModStatMovement => &Self::ModStatMovement,
            Self::SeekHelpBelowHp => &Self::SeekHelpBelowHp,
            Self::SeekHelpExecutionRadius => &Self::SeekHelpExecutionRadius,
            Self::SeekHelpWithinRadius => &Self::SeekHelpWithinRadius,
        }
    }
}
impl AttributeInfo for AiTemplate {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Abilities => 8823u16,
            Self::DefaultWeapon => 8888u16,
            Self::DisplayName => 8816u16,
            Self::Icon => 8935u16,
            Self::ModDamage => 8986u16,
            Self::ModSize => 8817u16,
            Self::ModStatHealth => 8815u16,
            Self::ModStatMovement => 8822u16,
            Self::SeekHelpBelowHp => 8887u16,
            Self::SeekHelpExecutionRadius => 8886u16,
            Self::SeekHelpWithinRadius => 8885u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Abilities => "abilities",
            Self::DefaultWeapon => "DefaultWeapon",
            Self::DisplayName => "DisplayName",
            Self::Icon => "Icon",
            Self::ModDamage => "modDamage",
            Self::ModSize => "modSize",
            Self::ModStatHealth => "modStatHealth",
            Self::ModStatMovement => "modStatMovement",
            Self::SeekHelpBelowHp => "SeekHelpBelowHP",
            Self::SeekHelpExecutionRadius => "SeekHelpExecutionRadius",
            Self::SeekHelpWithinRadius => "SeekHelpWithinRadius",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Abilities => ParamType::ContentRefList,
            Self::DefaultWeapon => ParamType::ContentRefList,
            Self::DisplayName => ParamType::LocalizedString,
            Self::Icon => ParamType::String,
            Self::ModDamage => ParamType::Float,
            Self::ModSize => ParamType::Float,
            Self::ModStatHealth => ParamType::Float,
            Self::ModStatMovement => ParamType::Float,
            Self::SeekHelpBelowHp => ParamType::Float,
            Self::SeekHelpExecutionRadius => ParamType::Float,
            Self::SeekHelpWithinRadius => ParamType::Float,
        }
    }
    fn default(&self) -> &'static Value {
        static ABILITIES: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static DEFAULT_WEAPON: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(
            "UI_Otherland.Misc.ITEM_PlaceHolderIcon".to_string(),
        ));
        static MOD_DAMAGE: Value = Value::Float(1f32);
        static MOD_SIZE: Value = Value::Float(1f32);
        static MOD_STAT_HEALTH: Value = Value::Float(1f32);
        static MOD_STAT_MOVEMENT: Value = Value::Float(1f32);
        static SEEK_HELP_BELOW_HP: Value = Value::Float(0f32);
        static SEEK_HELP_EXECUTION_RADIUS: Value = Value::Float(0f32);
        static SEEK_HELP_WITHIN_RADIUS: Value = Value::Float(0f32);
        match self {
            Self::Abilities => &ABILITIES,
            Self::DefaultWeapon => &DEFAULT_WEAPON,
            Self::DisplayName => &DISPLAY_NAME,
            Self::Icon => &ICON,
            Self::ModDamage => &MOD_DAMAGE,
            Self::ModSize => &MOD_SIZE,
            Self::ModStatHealth => &MOD_STAT_HEALTH,
            Self::ModStatMovement => &MOD_STAT_MOVEMENT,
            Self::SeekHelpBelowHp => &SEEK_HELP_BELOW_HP,
            Self::SeekHelpExecutionRadius => &SEEK_HELP_EXECUTION_RADIUS,
            Self::SeekHelpWithinRadius => &SEEK_HELP_WITHIN_RADIUS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Abilities => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DefaultWeapon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ModDamage => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ModSize => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ModStatHealth => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ModStatMovement => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SeekHelpBelowHp => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SeekHelpExecutionRadius => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SeekHelpWithinRadius => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for AiTemplate {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AI_TEMPLATE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for AiTemplate {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8823u16 => Ok(Self::Abilities),
            8888u16 => Ok(Self::DefaultWeapon),
            8816u16 => Ok(Self::DisplayName),
            8935u16 => Ok(Self::Icon),
            8986u16 => Ok(Self::ModDamage),
            8817u16 => Ok(Self::ModSize),
            8815u16 => Ok(Self::ModStatHealth),
            8822u16 => Ok(Self::ModStatMovement),
            8887u16 => Ok(Self::SeekHelpBelowHp),
            8886u16 => Ok(Self::SeekHelpExecutionRadius),
            8885u16 => Ok(Self::SeekHelpWithinRadius),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
