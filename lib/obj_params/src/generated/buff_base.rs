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
pub enum BuffBase {
    Age,
    Agility,
    Armor,
    ArmorMod,
    AttackPowerRating,
    AttributeType1,
    AttributeType2,
    AttributeType3,
    AttributeType4,
    AttributeValue1,
    AttributeValue2,
    AttributeValue3,
    AttributeValue4,
    BlockRating,
    ContentClass,
    CritDamageRating,
    CritHitRating,
    DamageDealtMod,
    DisplayName,
    DodgeMod,
    DodgeRating,
    EnableInGame,
    Focus,
    HealingDoneMod,
    HealingTakenMod,
    HeavyRating,
    HitRating,
    Icon,
    Lifespan,
    MovementSpeedMod,
    OnAttach,
    OnCalculation,
    OnDetach,
    ParryMod,
    ParryRating,
    PeneRating,
    SpecialRating,
    Stamina,
    StaminaMod,
    Strength,
    ThreatMod,
}
pub(crate) static BUFF_BASE_ATTRIBUTES: phf::Map<&'static str, BuffBase> = phf_map! {
    "age" => BuffBase::Age, "Agility" => BuffBase::Agility, "Armor" => BuffBase::Armor,
    "ArmorMod" => BuffBase::ArmorMod, "AttackPowerRating" => BuffBase::AttackPowerRating,
    "attributeType1" => BuffBase::AttributeType1, "attributeType2" =>
    BuffBase::AttributeType2, "attributeType3" => BuffBase::AttributeType3,
    "attributeType4" => BuffBase::AttributeType4, "attributeValue1" =>
    BuffBase::AttributeValue1, "attributeValue2" => BuffBase::AttributeValue2,
    "attributeValue3" => BuffBase::AttributeValue3, "attributeValue4" =>
    BuffBase::AttributeValue4, "BlockRating" => BuffBase::BlockRating, "ContentClass" =>
    BuffBase::ContentClass, "CritDamageRating" => BuffBase::CritDamageRating,
    "CritHitRating" => BuffBase::CritHitRating, "DamageDealtMod" =>
    BuffBase::DamageDealtMod, "DisplayName" => BuffBase::DisplayName, "DodgeMod" =>
    BuffBase::DodgeMod, "DodgeRating" => BuffBase::DodgeRating, "EnableInGame" =>
    BuffBase::EnableInGame, "Focus" => BuffBase::Focus, "HealingDoneMod" =>
    BuffBase::HealingDoneMod, "HealingTakenMod" => BuffBase::HealingTakenMod,
    "HeavyRating" => BuffBase::HeavyRating, "HitRating" => BuffBase::HitRating, "Icon" =>
    BuffBase::Icon, "lifespan" => BuffBase::Lifespan, "MovementSpeedMod" =>
    BuffBase::MovementSpeedMod, "OnAttach" => BuffBase::OnAttach, "OnCalculation" =>
    BuffBase::OnCalculation, "OnDetach" => BuffBase::OnDetach, "ParryMod" =>
    BuffBase::ParryMod, "ParryRating" => BuffBase::ParryRating, "PeneRating" =>
    BuffBase::PeneRating, "SpecialRating" => BuffBase::SpecialRating, "Stamina" =>
    BuffBase::Stamina, "StaminaMod" => BuffBase::StaminaMod, "Strength" =>
    BuffBase::Strength, "ThreatMod" => BuffBase::ThreatMod,
};
pub(crate) static BUFF_BASE_ATTRIBUTES_ID: phf::Map<u16, BuffBase> = phf_map! {
    22u16 => BuffBase::Age, 12065u16 => BuffBase::Agility, 12064u16 => BuffBase::Armor,
    12047u16 => BuffBase::ArmorMod, 12060u16 => BuffBase::AttackPowerRating, 6914u16 =>
    BuffBase::AttributeType1, 6913u16 => BuffBase::AttributeType2, 6912u16 =>
    BuffBase::AttributeType3, 6911u16 => BuffBase::AttributeType4, 6963u16 =>
    BuffBase::AttributeValue1, 6962u16 => BuffBase::AttributeValue2, 6961u16 =>
    BuffBase::AttributeValue3, 6960u16 => BuffBase::AttributeValue4, 12059u16 =>
    BuffBase::BlockRating, 20u16 => BuffBase::ContentClass, 12058u16 =>
    BuffBase::CritDamageRating, 12057u16 => BuffBase::CritHitRating, 12050u16 =>
    BuffBase::DamageDealtMod, 21u16 => BuffBase::DisplayName, 12046u16 =>
    BuffBase::DodgeMod, 12056u16 => BuffBase::DodgeRating, 6801u16 =>
    BuffBase::EnableInGame, 12063u16 => BuffBase::Focus, 12042u16 =>
    BuffBase::HealingDoneMod, 12049u16 => BuffBase::HealingTakenMod, 12055u16 =>
    BuffBase::HeavyRating, 12054u16 => BuffBase::HitRating, 4332u16 => BuffBase::Icon,
    23u16 => BuffBase::Lifespan, 12048u16 => BuffBase::MovementSpeedMod, 7529u16 =>
    BuffBase::OnAttach, 7534u16 => BuffBase::OnCalculation, 7528u16 =>
    BuffBase::OnDetach, 12045u16 => BuffBase::ParryMod, 12053u16 =>
    BuffBase::ParryRating, 12052u16 => BuffBase::PeneRating, 12051u16 =>
    BuffBase::SpecialRating, 12062u16 => BuffBase::Stamina, 12044u16 =>
    BuffBase::StaminaMod, 12061u16 => BuffBase::Strength, 12043u16 =>
    BuffBase::ThreatMod,
};
impl Attribute for BuffBase {
    fn class() -> Class {
        Class::BuffBase
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Age => &Self::Age,
            Self::Agility => &Self::Agility,
            Self::Armor => &Self::Armor,
            Self::ArmorMod => &Self::ArmorMod,
            Self::AttackPowerRating => &Self::AttackPowerRating,
            Self::AttributeType1 => &Self::AttributeType1,
            Self::AttributeType2 => &Self::AttributeType2,
            Self::AttributeType3 => &Self::AttributeType3,
            Self::AttributeType4 => &Self::AttributeType4,
            Self::AttributeValue1 => &Self::AttributeValue1,
            Self::AttributeValue2 => &Self::AttributeValue2,
            Self::AttributeValue3 => &Self::AttributeValue3,
            Self::AttributeValue4 => &Self::AttributeValue4,
            Self::BlockRating => &Self::BlockRating,
            Self::ContentClass => &Self::ContentClass,
            Self::CritDamageRating => &Self::CritDamageRating,
            Self::CritHitRating => &Self::CritHitRating,
            Self::DamageDealtMod => &Self::DamageDealtMod,
            Self::DisplayName => &Self::DisplayName,
            Self::DodgeMod => &Self::DodgeMod,
            Self::DodgeRating => &Self::DodgeRating,
            Self::EnableInGame => &Self::EnableInGame,
            Self::Focus => &Self::Focus,
            Self::HealingDoneMod => &Self::HealingDoneMod,
            Self::HealingTakenMod => &Self::HealingTakenMod,
            Self::HeavyRating => &Self::HeavyRating,
            Self::HitRating => &Self::HitRating,
            Self::Icon => &Self::Icon,
            Self::Lifespan => &Self::Lifespan,
            Self::MovementSpeedMod => &Self::MovementSpeedMod,
            Self::OnAttach => &Self::OnAttach,
            Self::OnCalculation => &Self::OnCalculation,
            Self::OnDetach => &Self::OnDetach,
            Self::ParryMod => &Self::ParryMod,
            Self::ParryRating => &Self::ParryRating,
            Self::PeneRating => &Self::PeneRating,
            Self::SpecialRating => &Self::SpecialRating,
            Self::Stamina => &Self::Stamina,
            Self::StaminaMod => &Self::StaminaMod,
            Self::Strength => &Self::Strength,
            Self::ThreatMod => &Self::ThreatMod,
        }
    }
}
impl AttributeInfo for BuffBase {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Age => 22u16,
            Self::Agility => 12065u16,
            Self::Armor => 12064u16,
            Self::ArmorMod => 12047u16,
            Self::AttackPowerRating => 12060u16,
            Self::AttributeType1 => 6914u16,
            Self::AttributeType2 => 6913u16,
            Self::AttributeType3 => 6912u16,
            Self::AttributeType4 => 6911u16,
            Self::AttributeValue1 => 6963u16,
            Self::AttributeValue2 => 6962u16,
            Self::AttributeValue3 => 6961u16,
            Self::AttributeValue4 => 6960u16,
            Self::BlockRating => 12059u16,
            Self::ContentClass => 20u16,
            Self::CritDamageRating => 12058u16,
            Self::CritHitRating => 12057u16,
            Self::DamageDealtMod => 12050u16,
            Self::DisplayName => 21u16,
            Self::DodgeMod => 12046u16,
            Self::DodgeRating => 12056u16,
            Self::EnableInGame => 6801u16,
            Self::Focus => 12063u16,
            Self::HealingDoneMod => 12042u16,
            Self::HealingTakenMod => 12049u16,
            Self::HeavyRating => 12055u16,
            Self::HitRating => 12054u16,
            Self::Icon => 4332u16,
            Self::Lifespan => 23u16,
            Self::MovementSpeedMod => 12048u16,
            Self::OnAttach => 7529u16,
            Self::OnCalculation => 7534u16,
            Self::OnDetach => 7528u16,
            Self::ParryMod => 12045u16,
            Self::ParryRating => 12053u16,
            Self::PeneRating => 12052u16,
            Self::SpecialRating => 12051u16,
            Self::Stamina => 12062u16,
            Self::StaminaMod => 12044u16,
            Self::Strength => 12061u16,
            Self::ThreatMod => 12043u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Age => "age",
            Self::Agility => "Agility",
            Self::Armor => "Armor",
            Self::ArmorMod => "ArmorMod",
            Self::AttackPowerRating => "AttackPowerRating",
            Self::AttributeType1 => "attributeType1",
            Self::AttributeType2 => "attributeType2",
            Self::AttributeType3 => "attributeType3",
            Self::AttributeType4 => "attributeType4",
            Self::AttributeValue1 => "attributeValue1",
            Self::AttributeValue2 => "attributeValue2",
            Self::AttributeValue3 => "attributeValue3",
            Self::AttributeValue4 => "attributeValue4",
            Self::BlockRating => "BlockRating",
            Self::ContentClass => "ContentClass",
            Self::CritDamageRating => "CritDamageRating",
            Self::CritHitRating => "CritHitRating",
            Self::DamageDealtMod => "DamageDealtMod",
            Self::DisplayName => "DisplayName",
            Self::DodgeMod => "DodgeMod",
            Self::DodgeRating => "DodgeRating",
            Self::EnableInGame => "EnableInGame",
            Self::Focus => "Focus",
            Self::HealingDoneMod => "HealingDoneMod",
            Self::HealingTakenMod => "HealingTakenMod",
            Self::HeavyRating => "HeavyRating",
            Self::HitRating => "HitRating",
            Self::Icon => "Icon",
            Self::Lifespan => "lifespan",
            Self::MovementSpeedMod => "MovementSpeedMod",
            Self::OnAttach => "OnAttach",
            Self::OnCalculation => "OnCalculation",
            Self::OnDetach => "OnDetach",
            Self::ParryMod => "ParryMod",
            Self::ParryRating => "ParryRating",
            Self::PeneRating => "PeneRating",
            Self::SpecialRating => "SpecialRating",
            Self::Stamina => "Stamina",
            Self::StaminaMod => "StaminaMod",
            Self::Strength => "Strength",
            Self::ThreatMod => "ThreatMod",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Age => ParamType::Float,
            Self::Agility => ParamType::Float,
            Self::Armor => ParamType::Float,
            Self::ArmorMod => ParamType::Float,
            Self::AttackPowerRating => ParamType::Float,
            Self::AttributeType1 => ParamType::String,
            Self::AttributeType2 => ParamType::String,
            Self::AttributeType3 => ParamType::String,
            Self::AttributeType4 => ParamType::String,
            Self::AttributeValue1 => ParamType::Float,
            Self::AttributeValue2 => ParamType::Float,
            Self::AttributeValue3 => ParamType::Float,
            Self::AttributeValue4 => ParamType::Float,
            Self::BlockRating => ParamType::Float,
            Self::ContentClass => ParamType::String,
            Self::CritDamageRating => ParamType::Float,
            Self::CritHitRating => ParamType::Float,
            Self::DamageDealtMod => ParamType::Float,
            Self::DisplayName => ParamType::LocalizedString,
            Self::DodgeMod => ParamType::Float,
            Self::DodgeRating => ParamType::Float,
            Self::EnableInGame => ParamType::Bool,
            Self::Focus => ParamType::Float,
            Self::HealingDoneMod => ParamType::Float,
            Self::HealingTakenMod => ParamType::Float,
            Self::HeavyRating => ParamType::Float,
            Self::HitRating => ParamType::Float,
            Self::Icon => ParamType::String,
            Self::Lifespan => ParamType::Float,
            Self::MovementSpeedMod => ParamType::Float,
            Self::OnAttach => ParamType::String,
            Self::OnCalculation => ParamType::String,
            Self::OnDetach => ParamType::String,
            Self::ParryMod => ParamType::Float,
            Self::ParryRating => ParamType::Float,
            Self::PeneRating => ParamType::Float,
            Self::SpecialRating => ParamType::Float,
            Self::Stamina => ParamType::Float,
            Self::StaminaMod => ParamType::Float,
            Self::Strength => ParamType::Float,
            Self::ThreatMod => ParamType::Float,
        }
    }
    fn default(&self) -> &'static Value {
        static AGE: Value = Value::Float(0f32);
        static AGILITY: Value = Value::Float(0f32);
        static ARMOR: Value = Value::Float(0f32);
        static ARMOR_MOD: Value = Value::Float(0f32);
        static ATTACK_POWER_RATING: Value = Value::Float(0f32);
        static ATTRIBUTE_TYPE_1: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_2: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_3: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_4: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_VALUE_1: Value = Value::Float(0f32);
        static ATTRIBUTE_VALUE_2: Value = Value::Float(0f32);
        static ATTRIBUTE_VALUE_3: Value = Value::Float(0f32);
        static ATTRIBUTE_VALUE_4: Value = Value::Float(0f32);
        static BLOCK_RATING: Value = Value::Float(0f32);
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CRIT_DAMAGE_RATING: Value = Value::Float(0f32);
        static CRIT_HIT_RATING: Value = Value::Float(0f32);
        static DAMAGE_DEALT_MOD: Value = Value::Float(0f32);
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DODGE_MOD: Value = Value::Float(0f32);
        static DODGE_RATING: Value = Value::Float(0f32);
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static FOCUS: Value = Value::Float(0f32);
        static HEALING_DONE_MOD: Value = Value::Float(0f32);
        static HEALING_TAKEN_MOD: Value = Value::Float(0f32);
        static HEAVY_RATING: Value = Value::Float(0f32);
        static HIT_RATING: Value = Value::Float(0f32);
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static LIFESPAN: Value = Value::Float(0f32);
        static MOVEMENT_SPEED_MOD: Value = Value::Float(0f32);
        static ON_ATTACH: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ON_CALCULATION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ON_DETACH: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static PARRY_MOD: Value = Value::Float(0f32);
        static PARRY_RATING: Value = Value::Float(0f32);
        static PENE_RATING: Value = Value::Float(0f32);
        static SPECIAL_RATING: Value = Value::Float(0f32);
        static STAMINA: Value = Value::Float(0f32);
        static STAMINA_MOD: Value = Value::Float(0f32);
        static STRENGTH: Value = Value::Float(0f32);
        static THREAT_MOD: Value = Value::Float(0f32);
        match self {
            Self::Age => &AGE,
            Self::Agility => &AGILITY,
            Self::Armor => &ARMOR,
            Self::ArmorMod => &ARMOR_MOD,
            Self::AttackPowerRating => &ATTACK_POWER_RATING,
            Self::AttributeType1 => &ATTRIBUTE_TYPE_1,
            Self::AttributeType2 => &ATTRIBUTE_TYPE_2,
            Self::AttributeType3 => &ATTRIBUTE_TYPE_3,
            Self::AttributeType4 => &ATTRIBUTE_TYPE_4,
            Self::AttributeValue1 => &ATTRIBUTE_VALUE_1,
            Self::AttributeValue2 => &ATTRIBUTE_VALUE_2,
            Self::AttributeValue3 => &ATTRIBUTE_VALUE_3,
            Self::AttributeValue4 => &ATTRIBUTE_VALUE_4,
            Self::BlockRating => &BLOCK_RATING,
            Self::ContentClass => &CONTENT_CLASS,
            Self::CritDamageRating => &CRIT_DAMAGE_RATING,
            Self::CritHitRating => &CRIT_HIT_RATING,
            Self::DamageDealtMod => &DAMAGE_DEALT_MOD,
            Self::DisplayName => &DISPLAY_NAME,
            Self::DodgeMod => &DODGE_MOD,
            Self::DodgeRating => &DODGE_RATING,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::Focus => &FOCUS,
            Self::HealingDoneMod => &HEALING_DONE_MOD,
            Self::HealingTakenMod => &HEALING_TAKEN_MOD,
            Self::HeavyRating => &HEAVY_RATING,
            Self::HitRating => &HIT_RATING,
            Self::Icon => &ICON,
            Self::Lifespan => &LIFESPAN,
            Self::MovementSpeedMod => &MOVEMENT_SPEED_MOD,
            Self::OnAttach => &ON_ATTACH,
            Self::OnCalculation => &ON_CALCULATION,
            Self::OnDetach => &ON_DETACH,
            Self::ParryMod => &PARRY_MOD,
            Self::ParryRating => &PARRY_RATING,
            Self::PeneRating => &PENE_RATING,
            Self::SpecialRating => &SPECIAL_RATING,
            Self::Stamina => &STAMINA,
            Self::StaminaMod => &STAMINA_MOD,
            Self::Strength => &STRENGTH,
            Self::ThreatMod => &THREAT_MOD,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Age => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Agility => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Armor => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ArmorMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttackPowerRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttributeType1 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeType2 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeType3 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeType4 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeValue1 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeValue2 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeValue3 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeValue4 => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::BlockRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ContentClass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CritDamageRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CritHitRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DamageDealtMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DodgeMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DodgeRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EnableInGame => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Focus => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HealingDoneMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HealingTakenMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HeavyRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HitRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Lifespan => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MovementSpeedMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::OnAttach => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::OnCalculation => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::OnDetach => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::ParryMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ParryRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PeneRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpecialRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Stamina => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::StaminaMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Strength => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ThreatMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
        }
    }
}
impl FromStr for BuffBase {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BUFF_BASE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for BuffBase {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            22u16 => Ok(Self::Age),
            12065u16 => Ok(Self::Agility),
            12064u16 => Ok(Self::Armor),
            12047u16 => Ok(Self::ArmorMod),
            12060u16 => Ok(Self::AttackPowerRating),
            6914u16 => Ok(Self::AttributeType1),
            6913u16 => Ok(Self::AttributeType2),
            6912u16 => Ok(Self::AttributeType3),
            6911u16 => Ok(Self::AttributeType4),
            6963u16 => Ok(Self::AttributeValue1),
            6962u16 => Ok(Self::AttributeValue2),
            6961u16 => Ok(Self::AttributeValue3),
            6960u16 => Ok(Self::AttributeValue4),
            12059u16 => Ok(Self::BlockRating),
            20u16 => Ok(Self::ContentClass),
            12058u16 => Ok(Self::CritDamageRating),
            12057u16 => Ok(Self::CritHitRating),
            12050u16 => Ok(Self::DamageDealtMod),
            21u16 => Ok(Self::DisplayName),
            12046u16 => Ok(Self::DodgeMod),
            12056u16 => Ok(Self::DodgeRating),
            6801u16 => Ok(Self::EnableInGame),
            12063u16 => Ok(Self::Focus),
            12042u16 => Ok(Self::HealingDoneMod),
            12049u16 => Ok(Self::HealingTakenMod),
            12055u16 => Ok(Self::HeavyRating),
            12054u16 => Ok(Self::HitRating),
            4332u16 => Ok(Self::Icon),
            23u16 => Ok(Self::Lifespan),
            12048u16 => Ok(Self::MovementSpeedMod),
            7529u16 => Ok(Self::OnAttach),
            7534u16 => Ok(Self::OnCalculation),
            7528u16 => Ok(Self::OnDetach),
            12045u16 => Ok(Self::ParryMod),
            12053u16 => Ok(Self::ParryRating),
            12052u16 => Ok(Self::PeneRating),
            12051u16 => Ok(Self::SpecialRating),
            12062u16 => Ok(Self::Stamina),
            12044u16 => Ok(Self::StaminaMod),
            12061u16 => Ok(Self::Strength),
            12043u16 => Ok(Self::ThreatMod),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
