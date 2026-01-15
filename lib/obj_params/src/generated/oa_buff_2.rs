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
pub enum OaBuff2 {
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
    AbilityInfo,
    AddCcEffectToOwner,
    Attributes,
    AvatarEffectorSetting,
    BuffGroup,
    BuffGroups,
    ConsumeOnAbilityUse,
    ConsumeOnBend,
    ConsumeOnCriticalHit,
    ConsumeOnHeavyAbilityUse,
    ConsumeOnReflect,
    ConsumeOnSpecialAbilityUse,
    CreationTime,
    DamageIncreasePerTick,
    DestroyOnAbilityStart,
    DestroyOnAbilityUse,
    DestroyOnCriticalHit,
    DestroyOnEnergyLevelNorm,
    DestroyOnGetHit,
    DestroyOnInstigatorDie,
    DestroyOnInvalidInstigator,
    DestroyOnMove,
    DestroyOnOwnerDied,
    DurationLeft,
    EffectorSettings,
    EndLifetime,
    EventTriggeredEffectors,
    FreedomDisablers,
    GrantedImmunityMainEffectGroups,
    GrantedImmunitySubEffectGroups,
    GroupIds,
    Instigator,
    InstigatorIsSource,
    InstigatorSnapshot,
    IsPersistent,
    IsStackable,
    LastTickTime,
    MainEffectGroup,
    MaxNumberOfSameBuff,
    MaxStackCount,
    MutuallyExclusiveToBuff,
    MutuallyExclusiveToBuffGroups,
    OverrideFaction,
    PositiveEffect,
    Power,
    ProtectsHeavyEnergy,
    ProtectsSpecialEnergy,
    Rank,
    RefreshDuration,
    SomaChanceMultiplier,
    SomaMultiplier,
    StackCount,
    StartStopEffectorSettings,
    SubEffectGroup,
    TickPeriod,
    TickUpdateToClient,
    TriggeredEffectors,
}
pub(crate) static OA_BUFF_2_ATTRIBUTES: phf::Map<&'static str, OaBuff2> = phf_map! {
    "age" => OaBuff2::Age, "Agility" => OaBuff2::Agility, "Armor" => OaBuff2::Armor,
    "ArmorMod" => OaBuff2::ArmorMod, "AttackPowerRating" => OaBuff2::AttackPowerRating,
    "attributeType1" => OaBuff2::AttributeType1, "attributeType2" =>
    OaBuff2::AttributeType2, "attributeType3" => OaBuff2::AttributeType3,
    "attributeType4" => OaBuff2::AttributeType4, "attributeValue1" =>
    OaBuff2::AttributeValue1, "attributeValue2" => OaBuff2::AttributeValue2,
    "attributeValue3" => OaBuff2::AttributeValue3, "attributeValue4" =>
    OaBuff2::AttributeValue4, "BlockRating" => OaBuff2::BlockRating, "ContentClass" =>
    OaBuff2::ContentClass, "CritDamageRating" => OaBuff2::CritDamageRating,
    "CritHitRating" => OaBuff2::CritHitRating, "DamageDealtMod" =>
    OaBuff2::DamageDealtMod, "DisplayName" => OaBuff2::DisplayName, "DodgeMod" =>
    OaBuff2::DodgeMod, "DodgeRating" => OaBuff2::DodgeRating, "EnableInGame" =>
    OaBuff2::EnableInGame, "Focus" => OaBuff2::Focus, "HealingDoneMod" =>
    OaBuff2::HealingDoneMod, "HealingTakenMod" => OaBuff2::HealingTakenMod, "HeavyRating"
    => OaBuff2::HeavyRating, "HitRating" => OaBuff2::HitRating, "Icon" => OaBuff2::Icon,
    "lifespan" => OaBuff2::Lifespan, "MovementSpeedMod" => OaBuff2::MovementSpeedMod,
    "OnAttach" => OaBuff2::OnAttach, "OnCalculation" => OaBuff2::OnCalculation,
    "OnDetach" => OaBuff2::OnDetach, "ParryMod" => OaBuff2::ParryMod, "ParryRating" =>
    OaBuff2::ParryRating, "PeneRating" => OaBuff2::PeneRating, "SpecialRating" =>
    OaBuff2::SpecialRating, "Stamina" => OaBuff2::Stamina, "StaminaMod" =>
    OaBuff2::StaminaMod, "Strength" => OaBuff2::Strength, "ThreatMod" =>
    OaBuff2::ThreatMod, "abilityInfo" => OaBuff2::AbilityInfo, "AddCCEffectToOwner" =>
    OaBuff2::AddCcEffectToOwner, "attributes" => OaBuff2::Attributes,
    "avatarEffectorSetting" => OaBuff2::AvatarEffectorSetting, "buffGroup" =>
    OaBuff2::BuffGroup, "buffGroups" => OaBuff2::BuffGroups, "consumeOn_AbilityUse" =>
    OaBuff2::ConsumeOnAbilityUse, "consumeOn_Bend" => OaBuff2::ConsumeOnBend,
    "consumeOn_CriticalHit" => OaBuff2::ConsumeOnCriticalHit, "consumeOn_HeavyAbilityUse"
    => OaBuff2::ConsumeOnHeavyAbilityUse, "consumeOn_Reflect" =>
    OaBuff2::ConsumeOnReflect, "consumeOn_SpecialAbilityUse" =>
    OaBuff2::ConsumeOnSpecialAbilityUse, "creationTime" => OaBuff2::CreationTime,
    "damageIncreasePerTick" => OaBuff2::DamageIncreasePerTick, "destroyOn_AbilityStart"
    => OaBuff2::DestroyOnAbilityStart, "destroyOnAbilityUse" =>
    OaBuff2::DestroyOnAbilityUse, "destroyOnCriticalHit" =>
    OaBuff2::DestroyOnCriticalHit, "destroyOnEnergyLevelNorm" =>
    OaBuff2::DestroyOnEnergyLevelNorm, "destroyOnGetHit" => OaBuff2::DestroyOnGetHit,
    "destroyOnInstigatorDie" => OaBuff2::DestroyOnInstigatorDie,
    "destroyOnInvalidInstigator" => OaBuff2::DestroyOnInvalidInstigator, "destroyOnMove"
    => OaBuff2::DestroyOnMove, "destroyOnOwnerDied" => OaBuff2::DestroyOnOwnerDied,
    "durationLeft" => OaBuff2::DurationLeft, "effectorSettings" =>
    OaBuff2::EffectorSettings, "endLifetime" => OaBuff2::EndLifetime,
    "eventTriggeredEffectors" => OaBuff2::EventTriggeredEffectors, "freedomDisablers" =>
    OaBuff2::FreedomDisablers, "GrantedImmunityMainEffectGroups" =>
    OaBuff2::GrantedImmunityMainEffectGroups, "GrantedImmunitySubEffectGroups" =>
    OaBuff2::GrantedImmunitySubEffectGroups, "groupIds" => OaBuff2::GroupIds,
    "instigator" => OaBuff2::Instigator, "instigatorIsSource" =>
    OaBuff2::InstigatorIsSource, "instigatorSnapshot" => OaBuff2::InstigatorSnapshot,
    "isPersistent" => OaBuff2::IsPersistent, "isStackable" => OaBuff2::IsStackable,
    "lastTickTime" => OaBuff2::LastTickTime, "MainEffectGroup" =>
    OaBuff2::MainEffectGroup, "maxNumberOfSameBuff" => OaBuff2::MaxNumberOfSameBuff,
    "maxStackCount" => OaBuff2::MaxStackCount, "mutuallyExclusiveToBuff" =>
    OaBuff2::MutuallyExclusiveToBuff, "mutuallyExclusiveToBuffGroups" =>
    OaBuff2::MutuallyExclusiveToBuffGroups, "OverrideFaction" =>
    OaBuff2::OverrideFaction, "PositiveEffect" => OaBuff2::PositiveEffect, "Power" =>
    OaBuff2::Power, "protectsHeavyEnergy" => OaBuff2::ProtectsHeavyEnergy,
    "protectsSpecialEnergy" => OaBuff2::ProtectsSpecialEnergy, "Rank" => OaBuff2::Rank,
    "refreshDuration" => OaBuff2::RefreshDuration, "somaChanceMultiplier" =>
    OaBuff2::SomaChanceMultiplier, "somaMultiplier" => OaBuff2::SomaMultiplier,
    "stackCount" => OaBuff2::StackCount, "startStopEffectorSettings" =>
    OaBuff2::StartStopEffectorSettings, "SubEffectGroup" => OaBuff2::SubEffectGroup,
    "tickPeriod" => OaBuff2::TickPeriod, "tickUpdateToClient" =>
    OaBuff2::TickUpdateToClient, "triggeredEffectors" => OaBuff2::TriggeredEffectors,
};
pub(crate) static OA_BUFF_2_ATTRIBUTES_ID: phf::Map<u16, OaBuff2> = phf_map! {
    44u16 => OaBuff2::Age, 12089u16 => OaBuff2::Agility, 12088u16 => OaBuff2::Armor,
    12071u16 => OaBuff2::ArmorMod, 12084u16 => OaBuff2::AttackPowerRating, 6938u16 =>
    OaBuff2::AttributeType1, 6937u16 => OaBuff2::AttributeType2, 6936u16 =>
    OaBuff2::AttributeType3, 6935u16 => OaBuff2::AttributeType4, 6971u16 =>
    OaBuff2::AttributeValue1, 6970u16 => OaBuff2::AttributeValue2, 6969u16 =>
    OaBuff2::AttributeValue3, 6968u16 => OaBuff2::AttributeValue4, 12083u16 =>
    OaBuff2::BlockRating, 46u16 => OaBuff2::ContentClass, 12082u16 =>
    OaBuff2::CritDamageRating, 12081u16 => OaBuff2::CritHitRating, 12074u16 =>
    OaBuff2::DamageDealtMod, 45u16 => OaBuff2::DisplayName, 12070u16 =>
    OaBuff2::DodgeMod, 12080u16 => OaBuff2::DodgeRating, 6803u16 =>
    OaBuff2::EnableInGame, 12087u16 => OaBuff2::Focus, 12066u16 =>
    OaBuff2::HealingDoneMod, 12073u16 => OaBuff2::HealingTakenMod, 12079u16 =>
    OaBuff2::HeavyRating, 12078u16 => OaBuff2::HitRating, 4334u16 => OaBuff2::Icon, 43u16
    => OaBuff2::Lifespan, 12072u16 => OaBuff2::MovementSpeedMod, 7533u16 =>
    OaBuff2::OnAttach, 7536u16 => OaBuff2::OnCalculation, 7532u16 => OaBuff2::OnDetach,
    12069u16 => OaBuff2::ParryMod, 12077u16 => OaBuff2::ParryRating, 12076u16 =>
    OaBuff2::PeneRating, 12075u16 => OaBuff2::SpecialRating, 12086u16 =>
    OaBuff2::Stamina, 12068u16 => OaBuff2::StaminaMod, 12085u16 => OaBuff2::Strength,
    12067u16 => OaBuff2::ThreatMod, 3579u16 => OaBuff2::AbilityInfo, 11151u16 =>
    OaBuff2::AddCcEffectToOwner, 47u16 => OaBuff2::Attributes, 9386u16 =>
    OaBuff2::AvatarEffectorSetting, 11326u16 => OaBuff2::BuffGroup, 11323u16 =>
    OaBuff2::BuffGroups, 10059u16 => OaBuff2::ConsumeOnAbilityUse, 11395u16 =>
    OaBuff2::ConsumeOnBend, 10056u16 => OaBuff2::ConsumeOnCriticalHit, 10058u16 =>
    OaBuff2::ConsumeOnHeavyAbilityUse, 11394u16 => OaBuff2::ConsumeOnReflect, 10057u16 =>
    OaBuff2::ConsumeOnSpecialAbilityUse, 9308u16 => OaBuff2::CreationTime, 7070u16 =>
    OaBuff2::DamageIncreasePerTick, 10060u16 => OaBuff2::DestroyOnAbilityStart, 4309u16
    => OaBuff2::DestroyOnAbilityUse, 10055u16 => OaBuff2::DestroyOnCriticalHit, 4310u16
    => OaBuff2::DestroyOnEnergyLevelNorm, 9310u16 => OaBuff2::DestroyOnGetHit, 8931u16 =>
    OaBuff2::DestroyOnInstigatorDie, 5417u16 => OaBuff2::DestroyOnInvalidInstigator,
    7883u16 => OaBuff2::DestroyOnMove, 9384u16 => OaBuff2::DestroyOnOwnerDied, 9309u16 =>
    OaBuff2::DurationLeft, 48u16 => OaBuff2::EffectorSettings, 49u16 =>
    OaBuff2::EndLifetime, 11219u16 => OaBuff2::EventTriggeredEffectors, 50u16 =>
    OaBuff2::FreedomDisablers, 11093u16 => OaBuff2::GrantedImmunityMainEffectGroups,
    11092u16 => OaBuff2::GrantedImmunitySubEffectGroups, 11316u16 => OaBuff2::GroupIds,
    51u16 => OaBuff2::Instigator, 11218u16 => OaBuff2::InstigatorIsSource, 11398u16 =>
    OaBuff2::InstigatorSnapshot, 8932u16 => OaBuff2::IsPersistent, 5418u16 =>
    OaBuff2::IsStackable, 52u16 => OaBuff2::LastTickTime, 11084u16 =>
    OaBuff2::MainEffectGroup, 11248u16 => OaBuff2::MaxNumberOfSameBuff, 9351u16 =>
    OaBuff2::MaxStackCount, 9311u16 => OaBuff2::MutuallyExclusiveToBuff, 11318u16 =>
    OaBuff2::MutuallyExclusiveToBuffGroups, 9369u16 => OaBuff2::OverrideFaction, 11091u16
    => OaBuff2::PositiveEffect, 11325u16 => OaBuff2::Power, 9353u16 =>
    OaBuff2::ProtectsHeavyEnergy, 9354u16 => OaBuff2::ProtectsSpecialEnergy, 11346u16 =>
    OaBuff2::Rank, 8941u16 => OaBuff2::RefreshDuration, 12417u16 =>
    OaBuff2::SomaChanceMultiplier, 12416u16 => OaBuff2::SomaMultiplier, 9352u16 =>
    OaBuff2::StackCount, 3578u16 => OaBuff2::StartStopEffectorSettings, 11083u16 =>
    OaBuff2::SubEffectGroup, 53u16 => OaBuff2::TickPeriod, 5509u16 =>
    OaBuff2::TickUpdateToClient, 10623u16 => OaBuff2::TriggeredEffectors,
};
impl Attribute for OaBuff2 {
    fn class() -> Class {
        Class::OaBuff2
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
            Self::AbilityInfo => &Self::AbilityInfo,
            Self::AddCcEffectToOwner => &Self::AddCcEffectToOwner,
            Self::Attributes => &Self::Attributes,
            Self::AvatarEffectorSetting => &Self::AvatarEffectorSetting,
            Self::BuffGroup => &Self::BuffGroup,
            Self::BuffGroups => &Self::BuffGroups,
            Self::ConsumeOnAbilityUse => &Self::ConsumeOnAbilityUse,
            Self::ConsumeOnBend => &Self::ConsumeOnBend,
            Self::ConsumeOnCriticalHit => &Self::ConsumeOnCriticalHit,
            Self::ConsumeOnHeavyAbilityUse => &Self::ConsumeOnHeavyAbilityUse,
            Self::ConsumeOnReflect => &Self::ConsumeOnReflect,
            Self::ConsumeOnSpecialAbilityUse => &Self::ConsumeOnSpecialAbilityUse,
            Self::CreationTime => &Self::CreationTime,
            Self::DamageIncreasePerTick => &Self::DamageIncreasePerTick,
            Self::DestroyOnAbilityStart => &Self::DestroyOnAbilityStart,
            Self::DestroyOnAbilityUse => &Self::DestroyOnAbilityUse,
            Self::DestroyOnCriticalHit => &Self::DestroyOnCriticalHit,
            Self::DestroyOnEnergyLevelNorm => &Self::DestroyOnEnergyLevelNorm,
            Self::DestroyOnGetHit => &Self::DestroyOnGetHit,
            Self::DestroyOnInstigatorDie => &Self::DestroyOnInstigatorDie,
            Self::DestroyOnInvalidInstigator => &Self::DestroyOnInvalidInstigator,
            Self::DestroyOnMove => &Self::DestroyOnMove,
            Self::DestroyOnOwnerDied => &Self::DestroyOnOwnerDied,
            Self::DurationLeft => &Self::DurationLeft,
            Self::EffectorSettings => &Self::EffectorSettings,
            Self::EndLifetime => &Self::EndLifetime,
            Self::EventTriggeredEffectors => &Self::EventTriggeredEffectors,
            Self::FreedomDisablers => &Self::FreedomDisablers,
            Self::GrantedImmunityMainEffectGroups => {
                &Self::GrantedImmunityMainEffectGroups
            }
            Self::GrantedImmunitySubEffectGroups => &Self::GrantedImmunitySubEffectGroups,
            Self::GroupIds => &Self::GroupIds,
            Self::Instigator => &Self::Instigator,
            Self::InstigatorIsSource => &Self::InstigatorIsSource,
            Self::InstigatorSnapshot => &Self::InstigatorSnapshot,
            Self::IsPersistent => &Self::IsPersistent,
            Self::IsStackable => &Self::IsStackable,
            Self::LastTickTime => &Self::LastTickTime,
            Self::MainEffectGroup => &Self::MainEffectGroup,
            Self::MaxNumberOfSameBuff => &Self::MaxNumberOfSameBuff,
            Self::MaxStackCount => &Self::MaxStackCount,
            Self::MutuallyExclusiveToBuff => &Self::MutuallyExclusiveToBuff,
            Self::MutuallyExclusiveToBuffGroups => &Self::MutuallyExclusiveToBuffGroups,
            Self::OverrideFaction => &Self::OverrideFaction,
            Self::PositiveEffect => &Self::PositiveEffect,
            Self::Power => &Self::Power,
            Self::ProtectsHeavyEnergy => &Self::ProtectsHeavyEnergy,
            Self::ProtectsSpecialEnergy => &Self::ProtectsSpecialEnergy,
            Self::Rank => &Self::Rank,
            Self::RefreshDuration => &Self::RefreshDuration,
            Self::SomaChanceMultiplier => &Self::SomaChanceMultiplier,
            Self::SomaMultiplier => &Self::SomaMultiplier,
            Self::StackCount => &Self::StackCount,
            Self::StartStopEffectorSettings => &Self::StartStopEffectorSettings,
            Self::SubEffectGroup => &Self::SubEffectGroup,
            Self::TickPeriod => &Self::TickPeriod,
            Self::TickUpdateToClient => &Self::TickUpdateToClient,
            Self::TriggeredEffectors => &Self::TriggeredEffectors,
        }
    }
}
impl AttributeInfo for OaBuff2 {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Age => 44u16,
            Self::Agility => 12089u16,
            Self::Armor => 12088u16,
            Self::ArmorMod => 12071u16,
            Self::AttackPowerRating => 12084u16,
            Self::AttributeType1 => 6938u16,
            Self::AttributeType2 => 6937u16,
            Self::AttributeType3 => 6936u16,
            Self::AttributeType4 => 6935u16,
            Self::AttributeValue1 => 6971u16,
            Self::AttributeValue2 => 6970u16,
            Self::AttributeValue3 => 6969u16,
            Self::AttributeValue4 => 6968u16,
            Self::BlockRating => 12083u16,
            Self::ContentClass => 46u16,
            Self::CritDamageRating => 12082u16,
            Self::CritHitRating => 12081u16,
            Self::DamageDealtMod => 12074u16,
            Self::DisplayName => 45u16,
            Self::DodgeMod => 12070u16,
            Self::DodgeRating => 12080u16,
            Self::EnableInGame => 6803u16,
            Self::Focus => 12087u16,
            Self::HealingDoneMod => 12066u16,
            Self::HealingTakenMod => 12073u16,
            Self::HeavyRating => 12079u16,
            Self::HitRating => 12078u16,
            Self::Icon => 4334u16,
            Self::Lifespan => 43u16,
            Self::MovementSpeedMod => 12072u16,
            Self::OnAttach => 7533u16,
            Self::OnCalculation => 7536u16,
            Self::OnDetach => 7532u16,
            Self::ParryMod => 12069u16,
            Self::ParryRating => 12077u16,
            Self::PeneRating => 12076u16,
            Self::SpecialRating => 12075u16,
            Self::Stamina => 12086u16,
            Self::StaminaMod => 12068u16,
            Self::Strength => 12085u16,
            Self::ThreatMod => 12067u16,
            Self::AbilityInfo => 3579u16,
            Self::AddCcEffectToOwner => 11151u16,
            Self::Attributes => 47u16,
            Self::AvatarEffectorSetting => 9386u16,
            Self::BuffGroup => 11326u16,
            Self::BuffGroups => 11323u16,
            Self::ConsumeOnAbilityUse => 10059u16,
            Self::ConsumeOnBend => 11395u16,
            Self::ConsumeOnCriticalHit => 10056u16,
            Self::ConsumeOnHeavyAbilityUse => 10058u16,
            Self::ConsumeOnReflect => 11394u16,
            Self::ConsumeOnSpecialAbilityUse => 10057u16,
            Self::CreationTime => 9308u16,
            Self::DamageIncreasePerTick => 7070u16,
            Self::DestroyOnAbilityStart => 10060u16,
            Self::DestroyOnAbilityUse => 4309u16,
            Self::DestroyOnCriticalHit => 10055u16,
            Self::DestroyOnEnergyLevelNorm => 4310u16,
            Self::DestroyOnGetHit => 9310u16,
            Self::DestroyOnInstigatorDie => 8931u16,
            Self::DestroyOnInvalidInstigator => 5417u16,
            Self::DestroyOnMove => 7883u16,
            Self::DestroyOnOwnerDied => 9384u16,
            Self::DurationLeft => 9309u16,
            Self::EffectorSettings => 48u16,
            Self::EndLifetime => 49u16,
            Self::EventTriggeredEffectors => 11219u16,
            Self::FreedomDisablers => 50u16,
            Self::GrantedImmunityMainEffectGroups => 11093u16,
            Self::GrantedImmunitySubEffectGroups => 11092u16,
            Self::GroupIds => 11316u16,
            Self::Instigator => 51u16,
            Self::InstigatorIsSource => 11218u16,
            Self::InstigatorSnapshot => 11398u16,
            Self::IsPersistent => 8932u16,
            Self::IsStackable => 5418u16,
            Self::LastTickTime => 52u16,
            Self::MainEffectGroup => 11084u16,
            Self::MaxNumberOfSameBuff => 11248u16,
            Self::MaxStackCount => 9351u16,
            Self::MutuallyExclusiveToBuff => 9311u16,
            Self::MutuallyExclusiveToBuffGroups => 11318u16,
            Self::OverrideFaction => 9369u16,
            Self::PositiveEffect => 11091u16,
            Self::Power => 11325u16,
            Self::ProtectsHeavyEnergy => 9353u16,
            Self::ProtectsSpecialEnergy => 9354u16,
            Self::Rank => 11346u16,
            Self::RefreshDuration => 8941u16,
            Self::SomaChanceMultiplier => 12417u16,
            Self::SomaMultiplier => 12416u16,
            Self::StackCount => 9352u16,
            Self::StartStopEffectorSettings => 3578u16,
            Self::SubEffectGroup => 11083u16,
            Self::TickPeriod => 53u16,
            Self::TickUpdateToClient => 5509u16,
            Self::TriggeredEffectors => 10623u16,
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
            Self::AbilityInfo => "abilityInfo",
            Self::AddCcEffectToOwner => "AddCCEffectToOwner",
            Self::Attributes => "attributes",
            Self::AvatarEffectorSetting => "avatarEffectorSetting",
            Self::BuffGroup => "buffGroup",
            Self::BuffGroups => "buffGroups",
            Self::ConsumeOnAbilityUse => "consumeOn_AbilityUse",
            Self::ConsumeOnBend => "consumeOn_Bend",
            Self::ConsumeOnCriticalHit => "consumeOn_CriticalHit",
            Self::ConsumeOnHeavyAbilityUse => "consumeOn_HeavyAbilityUse",
            Self::ConsumeOnReflect => "consumeOn_Reflect",
            Self::ConsumeOnSpecialAbilityUse => "consumeOn_SpecialAbilityUse",
            Self::CreationTime => "creationTime",
            Self::DamageIncreasePerTick => "damageIncreasePerTick",
            Self::DestroyOnAbilityStart => "destroyOn_AbilityStart",
            Self::DestroyOnAbilityUse => "destroyOnAbilityUse",
            Self::DestroyOnCriticalHit => "destroyOnCriticalHit",
            Self::DestroyOnEnergyLevelNorm => "destroyOnEnergyLevelNorm",
            Self::DestroyOnGetHit => "destroyOnGetHit",
            Self::DestroyOnInstigatorDie => "destroyOnInstigatorDie",
            Self::DestroyOnInvalidInstigator => "destroyOnInvalidInstigator",
            Self::DestroyOnMove => "destroyOnMove",
            Self::DestroyOnOwnerDied => "destroyOnOwnerDied",
            Self::DurationLeft => "durationLeft",
            Self::EffectorSettings => "effectorSettings",
            Self::EndLifetime => "endLifetime",
            Self::EventTriggeredEffectors => "eventTriggeredEffectors",
            Self::FreedomDisablers => "freedomDisablers",
            Self::GrantedImmunityMainEffectGroups => "GrantedImmunityMainEffectGroups",
            Self::GrantedImmunitySubEffectGroups => "GrantedImmunitySubEffectGroups",
            Self::GroupIds => "groupIds",
            Self::Instigator => "instigator",
            Self::InstigatorIsSource => "instigatorIsSource",
            Self::InstigatorSnapshot => "instigatorSnapshot",
            Self::IsPersistent => "isPersistent",
            Self::IsStackable => "isStackable",
            Self::LastTickTime => "lastTickTime",
            Self::MainEffectGroup => "MainEffectGroup",
            Self::MaxNumberOfSameBuff => "maxNumberOfSameBuff",
            Self::MaxStackCount => "maxStackCount",
            Self::MutuallyExclusiveToBuff => "mutuallyExclusiveToBuff",
            Self::MutuallyExclusiveToBuffGroups => "mutuallyExclusiveToBuffGroups",
            Self::OverrideFaction => "OverrideFaction",
            Self::PositiveEffect => "PositiveEffect",
            Self::Power => "Power",
            Self::ProtectsHeavyEnergy => "protectsHeavyEnergy",
            Self::ProtectsSpecialEnergy => "protectsSpecialEnergy",
            Self::Rank => "Rank",
            Self::RefreshDuration => "refreshDuration",
            Self::SomaChanceMultiplier => "somaChanceMultiplier",
            Self::SomaMultiplier => "somaMultiplier",
            Self::StackCount => "stackCount",
            Self::StartStopEffectorSettings => "startStopEffectorSettings",
            Self::SubEffectGroup => "SubEffectGroup",
            Self::TickPeriod => "tickPeriod",
            Self::TickUpdateToClient => "tickUpdateToClient",
            Self::TriggeredEffectors => "triggeredEffectors",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AbilityInfo => ParamType::JsonValue,
            Self::AddCcEffectToOwner => ParamType::VectorInt,
            Self::Attributes => ParamType::JsonValue,
            Self::AvatarEffectorSetting => ParamType::JsonValue,
            Self::BuffGroup => ParamType::ContentRef,
            Self::BuffGroups => ParamType::ContentRefList,
            Self::ConsumeOnAbilityUse => ParamType::Bool,
            Self::ConsumeOnBend => ParamType::Bool,
            Self::ConsumeOnCriticalHit => ParamType::Bool,
            Self::ConsumeOnHeavyAbilityUse => ParamType::Bool,
            Self::ConsumeOnReflect => ParamType::Bool,
            Self::ConsumeOnSpecialAbilityUse => ParamType::Bool,
            Self::CreationTime => ParamType::Float,
            Self::DamageIncreasePerTick => ParamType::Float,
            Self::DestroyOnAbilityStart => ParamType::Bool,
            Self::DestroyOnAbilityUse => ParamType::Bool,
            Self::DestroyOnCriticalHit => ParamType::Bool,
            Self::DestroyOnEnergyLevelNorm => ParamType::Float,
            Self::DestroyOnGetHit => ParamType::Bool,
            Self::DestroyOnInstigatorDie => ParamType::Bool,
            Self::DestroyOnInvalidInstigator => ParamType::Bool,
            Self::DestroyOnMove => ParamType::Bool,
            Self::DestroyOnOwnerDied => ParamType::Bool,
            Self::DurationLeft => ParamType::Float,
            Self::EffectorSettings => ParamType::JsonValue,
            Self::EndLifetime => ParamType::Float,
            Self::EventTriggeredEffectors => ParamType::JsonValue,
            Self::FreedomDisablers => ParamType::JsonValue,
            Self::GrantedImmunityMainEffectGroups => ParamType::VectorInt,
            Self::GrantedImmunitySubEffectGroups => ParamType::VectorInt,
            Self::GroupIds => ParamType::VectorString,
            Self::Instigator => ParamType::AvatarId,
            Self::InstigatorIsSource => ParamType::Bool,
            Self::InstigatorSnapshot => ParamType::Any,
            Self::IsPersistent => ParamType::Bool,
            Self::IsStackable => ParamType::Bool,
            Self::LastTickTime => ParamType::Float,
            Self::MainEffectGroup => ParamType::VectorInt,
            Self::MaxNumberOfSameBuff => ParamType::Int,
            Self::MaxStackCount => ParamType::Int,
            Self::MutuallyExclusiveToBuff => ParamType::ContentRef,
            Self::MutuallyExclusiveToBuffGroups => ParamType::ContentRefList,
            Self::OverrideFaction => ParamType::ContentRefList,
            Self::PositiveEffect => ParamType::Bool,
            Self::Power => ParamType::Int,
            Self::ProtectsHeavyEnergy => ParamType::Bool,
            Self::ProtectsSpecialEnergy => ParamType::Bool,
            Self::Rank => ParamType::Int,
            Self::RefreshDuration => ParamType::Bool,
            Self::SomaChanceMultiplier => ParamType::Float,
            Self::SomaMultiplier => ParamType::Float,
            Self::StackCount => ParamType::Int,
            Self::StartStopEffectorSettings => ParamType::JsonValue,
            Self::SubEffectGroup => ParamType::VectorInt,
            Self::TickPeriod => ParamType::Float,
            Self::TickUpdateToClient => ParamType::Bool,
            Self::TriggeredEffectors => ParamType::JsonValue,
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
        static ABILITY_INFO: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static ADD_CC_EFFECT_TO_OWNER: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static ATTRIBUTES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static AVATAR_EFFECTOR_SETTING: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static BUFF_GROUP: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static BUFF_GROUPS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static CONSUME_ON_ABILITY_USE: Value = Value::Bool(false);
        static CONSUME_ON_BEND: Value = Value::Bool(false);
        static CONSUME_ON_CRITICAL_HIT: Value = Value::Bool(false);
        static CONSUME_ON_HEAVY_ABILITY_USE: Value = Value::Bool(false);
        static CONSUME_ON_REFLECT: Value = Value::Bool(false);
        static CONSUME_ON_SPECIAL_ABILITY_USE: Value = Value::Bool(false);
        static CREATION_TIME: Value = Value::Float(0f32);
        static DAMAGE_INCREASE_PER_TICK: Value = Value::Float(0f32);
        static DESTROY_ON_ABILITY_START: Value = Value::Bool(false);
        static DESTROY_ON_ABILITY_USE: Value = Value::Bool(false);
        static DESTROY_ON_CRITICAL_HIT: Value = Value::Bool(false);
        static DESTROY_ON_ENERGY_LEVEL_NORM: Value = Value::Float(-1f32);
        static DESTROY_ON_GET_HIT: Value = Value::Bool(false);
        static DESTROY_ON_INSTIGATOR_DIE: Value = Value::Bool(false);
        static DESTROY_ON_INVALID_INSTIGATOR: Value = Value::Bool(false);
        static DESTROY_ON_MOVE: Value = Value::Bool(false);
        static DESTROY_ON_OWNER_DIED: Value = Value::Bool(true);
        static DURATION_LEFT: Value = Value::Float(0f32);
        static EFFECTOR_SETTINGS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static END_LIFETIME: Value = Value::Float(0f32);
        static EVENT_TRIGGERED_EFFECTORS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static FREEDOM_DISABLERS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static GRANTED_IMMUNITY_MAIN_EFFECT_GROUPS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static GRANTED_IMMUNITY_SUB_EFFECT_GROUPS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static GROUP_IDS: Lazy<Value> = Lazy::new(|| Value::VectorString(vec![]));
        static INSTIGATOR: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static INSTIGATOR_IS_SOURCE: Value = Value::Bool(false);
        static INSTIGATOR_SNAPSHOT: Value = Value::Any(vec![]);
        static IS_PERSISTENT: Value = Value::Bool(true);
        static IS_STACKABLE: Value = Value::Bool(true);
        static LAST_TICK_TIME: Value = Value::Float(0f32);
        static MAIN_EFFECT_GROUP: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static MAX_NUMBER_OF_SAME_BUFF: Value = Value::Int(-1i32);
        static MAX_STACK_COUNT: Value = Value::Int(1i32);
        static MUTUALLY_EXCLUSIVE_TO_BUFF: Lazy<Value> = Lazy::new(|| Value::ContentRef(
            None,
        ));
        static MUTUALLY_EXCLUSIVE_TO_BUFF_GROUPS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static OVERRIDE_FACTION: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static POSITIVE_EFFECT: Value = Value::Bool(true);
        static POWER: Value = Value::Int(0i32);
        static PROTECTS_HEAVY_ENERGY: Value = Value::Bool(false);
        static PROTECTS_SPECIAL_ENERGY: Value = Value::Bool(false);
        static RANK: Value = Value::Int(0i32);
        static REFRESH_DURATION: Value = Value::Bool(true);
        static SOMA_CHANCE_MULTIPLIER: Value = Value::Float(0f32);
        static SOMA_MULTIPLIER: Value = Value::Float(0f32);
        static STACK_COUNT: Value = Value::Int(1i32);
        static START_STOP_EFFECTOR_SETTINGS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static SUB_EFFECT_GROUP: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static TICK_PERIOD: Value = Value::Float(0f32);
        static TICK_UPDATE_TO_CLIENT: Value = Value::Bool(false);
        static TRIGGERED_EFFECTORS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
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
            Self::AbilityInfo => &ABILITY_INFO,
            Self::AddCcEffectToOwner => &ADD_CC_EFFECT_TO_OWNER,
            Self::Attributes => &ATTRIBUTES,
            Self::AvatarEffectorSetting => &AVATAR_EFFECTOR_SETTING,
            Self::BuffGroup => &BUFF_GROUP,
            Self::BuffGroups => &BUFF_GROUPS,
            Self::ConsumeOnAbilityUse => &CONSUME_ON_ABILITY_USE,
            Self::ConsumeOnBend => &CONSUME_ON_BEND,
            Self::ConsumeOnCriticalHit => &CONSUME_ON_CRITICAL_HIT,
            Self::ConsumeOnHeavyAbilityUse => &CONSUME_ON_HEAVY_ABILITY_USE,
            Self::ConsumeOnReflect => &CONSUME_ON_REFLECT,
            Self::ConsumeOnSpecialAbilityUse => &CONSUME_ON_SPECIAL_ABILITY_USE,
            Self::CreationTime => &CREATION_TIME,
            Self::DamageIncreasePerTick => &DAMAGE_INCREASE_PER_TICK,
            Self::DestroyOnAbilityStart => &DESTROY_ON_ABILITY_START,
            Self::DestroyOnAbilityUse => &DESTROY_ON_ABILITY_USE,
            Self::DestroyOnCriticalHit => &DESTROY_ON_CRITICAL_HIT,
            Self::DestroyOnEnergyLevelNorm => &DESTROY_ON_ENERGY_LEVEL_NORM,
            Self::DestroyOnGetHit => &DESTROY_ON_GET_HIT,
            Self::DestroyOnInstigatorDie => &DESTROY_ON_INSTIGATOR_DIE,
            Self::DestroyOnInvalidInstigator => &DESTROY_ON_INVALID_INSTIGATOR,
            Self::DestroyOnMove => &DESTROY_ON_MOVE,
            Self::DestroyOnOwnerDied => &DESTROY_ON_OWNER_DIED,
            Self::DurationLeft => &DURATION_LEFT,
            Self::EffectorSettings => &EFFECTOR_SETTINGS,
            Self::EndLifetime => &END_LIFETIME,
            Self::EventTriggeredEffectors => &EVENT_TRIGGERED_EFFECTORS,
            Self::FreedomDisablers => &FREEDOM_DISABLERS,
            Self::GrantedImmunityMainEffectGroups => &GRANTED_IMMUNITY_MAIN_EFFECT_GROUPS,
            Self::GrantedImmunitySubEffectGroups => &GRANTED_IMMUNITY_SUB_EFFECT_GROUPS,
            Self::GroupIds => &GROUP_IDS,
            Self::Instigator => &INSTIGATOR,
            Self::InstigatorIsSource => &INSTIGATOR_IS_SOURCE,
            Self::InstigatorSnapshot => &INSTIGATOR_SNAPSHOT,
            Self::IsPersistent => &IS_PERSISTENT,
            Self::IsStackable => &IS_STACKABLE,
            Self::LastTickTime => &LAST_TICK_TIME,
            Self::MainEffectGroup => &MAIN_EFFECT_GROUP,
            Self::MaxNumberOfSameBuff => &MAX_NUMBER_OF_SAME_BUFF,
            Self::MaxStackCount => &MAX_STACK_COUNT,
            Self::MutuallyExclusiveToBuff => &MUTUALLY_EXCLUSIVE_TO_BUFF,
            Self::MutuallyExclusiveToBuffGroups => &MUTUALLY_EXCLUSIVE_TO_BUFF_GROUPS,
            Self::OverrideFaction => &OVERRIDE_FACTION,
            Self::PositiveEffect => &POSITIVE_EFFECT,
            Self::Power => &POWER,
            Self::ProtectsHeavyEnergy => &PROTECTS_HEAVY_ENERGY,
            Self::ProtectsSpecialEnergy => &PROTECTS_SPECIAL_ENERGY,
            Self::Rank => &RANK,
            Self::RefreshDuration => &REFRESH_DURATION,
            Self::SomaChanceMultiplier => &SOMA_CHANCE_MULTIPLIER,
            Self::SomaMultiplier => &SOMA_MULTIPLIER,
            Self::StackCount => &STACK_COUNT,
            Self::StartStopEffectorSettings => &START_STOP_EFFECTOR_SETTINGS,
            Self::SubEffectGroup => &SUB_EFFECT_GROUP,
            Self::TickPeriod => &TICK_PERIOD,
            Self::TickUpdateToClient => &TICK_UPDATE_TO_CLIENT,
            Self::TriggeredEffectors => &TRIGGERED_EFFECTORS,
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
            Self::AbilityInfo => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AddCcEffectToOwner => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Attributes => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AvatarEffectorSetting => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::BuffGroup => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::BuffGroups => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::ConsumeOnAbilityUse => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ConsumeOnBend => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ConsumeOnCriticalHit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ConsumeOnHeavyAbilityUse => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::ConsumeOnReflect => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ConsumeOnSpecialAbilityUse => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CreationTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DamageIncreasePerTick => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnAbilityStart => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnAbilityUse => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnCriticalHit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnEnergyLevelNorm => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::DestroyOnGetHit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnInstigatorDie => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnInvalidInstigator => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::DestroyOnMove => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestroyOnOwnerDied => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DurationLeft => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EffectorSettings => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EndLifetime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EventTriggeredEffectors => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::FreedomDisablers => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GrantedImmunityMainEffectGroups => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::GrantedImmunitySubEffectGroups => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::GroupIds => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::Instigator => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InstigatorIsSource => &[ParamFlag::Persistent, ParamFlag::Deprecated],
            Self::InstigatorSnapshot => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsPersistent => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsStackable => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::LastTickTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MainEffectGroup => &[ParamFlag::Persistent],
            Self::MaxNumberOfSameBuff => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MaxStackCount => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MutuallyExclusiveToBuff => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::MutuallyExclusiveToBuffGroups => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::OverrideFaction => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PositiveEffect => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ProtectsHeavyEnergy => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ProtectsSpecialEnergy => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Rank => &[ParamFlag::Content],
            Self::RefreshDuration => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::SomaChanceMultiplier => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SomaMultiplier => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::StackCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::StartStopEffectorSettings => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SubEffectGroup => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TickPeriod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TickUpdateToClient => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TriggeredEffectors => &[ParamFlag::Persistent, ParamFlag::Content],
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
impl FromStr for OaBuff2 {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OA_BUFF_2_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for OaBuff2 {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            44u16 => Ok(Self::Age),
            12089u16 => Ok(Self::Agility),
            12088u16 => Ok(Self::Armor),
            12071u16 => Ok(Self::ArmorMod),
            12084u16 => Ok(Self::AttackPowerRating),
            6938u16 => Ok(Self::AttributeType1),
            6937u16 => Ok(Self::AttributeType2),
            6936u16 => Ok(Self::AttributeType3),
            6935u16 => Ok(Self::AttributeType4),
            6971u16 => Ok(Self::AttributeValue1),
            6970u16 => Ok(Self::AttributeValue2),
            6969u16 => Ok(Self::AttributeValue3),
            6968u16 => Ok(Self::AttributeValue4),
            12083u16 => Ok(Self::BlockRating),
            46u16 => Ok(Self::ContentClass),
            12082u16 => Ok(Self::CritDamageRating),
            12081u16 => Ok(Self::CritHitRating),
            12074u16 => Ok(Self::DamageDealtMod),
            45u16 => Ok(Self::DisplayName),
            12070u16 => Ok(Self::DodgeMod),
            12080u16 => Ok(Self::DodgeRating),
            6803u16 => Ok(Self::EnableInGame),
            12087u16 => Ok(Self::Focus),
            12066u16 => Ok(Self::HealingDoneMod),
            12073u16 => Ok(Self::HealingTakenMod),
            12079u16 => Ok(Self::HeavyRating),
            12078u16 => Ok(Self::HitRating),
            4334u16 => Ok(Self::Icon),
            43u16 => Ok(Self::Lifespan),
            12072u16 => Ok(Self::MovementSpeedMod),
            7533u16 => Ok(Self::OnAttach),
            7536u16 => Ok(Self::OnCalculation),
            7532u16 => Ok(Self::OnDetach),
            12069u16 => Ok(Self::ParryMod),
            12077u16 => Ok(Self::ParryRating),
            12076u16 => Ok(Self::PeneRating),
            12075u16 => Ok(Self::SpecialRating),
            12086u16 => Ok(Self::Stamina),
            12068u16 => Ok(Self::StaminaMod),
            12085u16 => Ok(Self::Strength),
            12067u16 => Ok(Self::ThreatMod),
            3579u16 => Ok(Self::AbilityInfo),
            11151u16 => Ok(Self::AddCcEffectToOwner),
            47u16 => Ok(Self::Attributes),
            9386u16 => Ok(Self::AvatarEffectorSetting),
            11326u16 => Ok(Self::BuffGroup),
            11323u16 => Ok(Self::BuffGroups),
            10059u16 => Ok(Self::ConsumeOnAbilityUse),
            11395u16 => Ok(Self::ConsumeOnBend),
            10056u16 => Ok(Self::ConsumeOnCriticalHit),
            10058u16 => Ok(Self::ConsumeOnHeavyAbilityUse),
            11394u16 => Ok(Self::ConsumeOnReflect),
            10057u16 => Ok(Self::ConsumeOnSpecialAbilityUse),
            9308u16 => Ok(Self::CreationTime),
            7070u16 => Ok(Self::DamageIncreasePerTick),
            10060u16 => Ok(Self::DestroyOnAbilityStart),
            4309u16 => Ok(Self::DestroyOnAbilityUse),
            10055u16 => Ok(Self::DestroyOnCriticalHit),
            4310u16 => Ok(Self::DestroyOnEnergyLevelNorm),
            9310u16 => Ok(Self::DestroyOnGetHit),
            8931u16 => Ok(Self::DestroyOnInstigatorDie),
            5417u16 => Ok(Self::DestroyOnInvalidInstigator),
            7883u16 => Ok(Self::DestroyOnMove),
            9384u16 => Ok(Self::DestroyOnOwnerDied),
            9309u16 => Ok(Self::DurationLeft),
            48u16 => Ok(Self::EffectorSettings),
            49u16 => Ok(Self::EndLifetime),
            11219u16 => Ok(Self::EventTriggeredEffectors),
            50u16 => Ok(Self::FreedomDisablers),
            11093u16 => Ok(Self::GrantedImmunityMainEffectGroups),
            11092u16 => Ok(Self::GrantedImmunitySubEffectGroups),
            11316u16 => Ok(Self::GroupIds),
            51u16 => Ok(Self::Instigator),
            11218u16 => Ok(Self::InstigatorIsSource),
            11398u16 => Ok(Self::InstigatorSnapshot),
            8932u16 => Ok(Self::IsPersistent),
            5418u16 => Ok(Self::IsStackable),
            52u16 => Ok(Self::LastTickTime),
            11084u16 => Ok(Self::MainEffectGroup),
            11248u16 => Ok(Self::MaxNumberOfSameBuff),
            9351u16 => Ok(Self::MaxStackCount),
            9311u16 => Ok(Self::MutuallyExclusiveToBuff),
            11318u16 => Ok(Self::MutuallyExclusiveToBuffGroups),
            9369u16 => Ok(Self::OverrideFaction),
            11091u16 => Ok(Self::PositiveEffect),
            11325u16 => Ok(Self::Power),
            9353u16 => Ok(Self::ProtectsHeavyEnergy),
            9354u16 => Ok(Self::ProtectsSpecialEnergy),
            11346u16 => Ok(Self::Rank),
            8941u16 => Ok(Self::RefreshDuration),
            12417u16 => Ok(Self::SomaChanceMultiplier),
            12416u16 => Ok(Self::SomaMultiplier),
            9352u16 => Ok(Self::StackCount),
            3578u16 => Ok(Self::StartStopEffectorSettings),
            11083u16 => Ok(Self::SubEffectGroup),
            53u16 => Ok(Self::TickPeriod),
            5509u16 => Ok(Self::TickUpdateToClient),
            10623u16 => Ok(Self::TriggeredEffectors),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
