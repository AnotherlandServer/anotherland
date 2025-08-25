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
pub enum EdnaAbility {
    AbilityInfo,
    AbilityType,
    AbilityUsedCounter,
    ActivationAngle,
    ActivationType,
    AffectEnemies,
    AffectFriends,
    AffectNeutral,
    AffectSelf,
    AllowMoveWhileChanneling,
    AllowMoveWhileCharging,
    AllowMoveWhileEndState,
    AllowPlayerMoveWhileChanneling,
    AllowRangedAttackAdjustMove,
    AllowUseAboveHp,
    AllowUseAfterKillNumberOfNpCs,
    AllowUseAfterKillNumberOfPlayers,
    AllowUseBelowHp,
    AllowUseWhenAvailable,
    AlwaysExecute,
    Attributes,
    AutoFaceTargetOrLocation,
    BlockFaceTargetWhenPreparing,
    CanBeBend,
    CanBeBlocked,
    CanBeInterrupted,
    CanBeReflected,
    CanPredict,
    CastTime,
    ChannelIndefinitely,
    ChannelTime,
    Charge,
    CombatStyle,
    ConsumeCooldowns,
    ConsumeEnergyInChannelTick,
    Description,
    DisplayName,
    DoNotCheckObstruction,
    EffectorCastSettings,
    EffectorChannelingSettings,
    EffectorSettings,
    EffectType,
    EmitCooldownOnCast,
    EmitCooldownOnUse,
    EnableInGame,
    EndStateDuration,
    EnergyConsumed,
    ExecutionTime,
    ExternalCooldownsConsumed,
    ExternalCooldownsEmitted,
    GeneralPreRequisites,
    Icon,
    InternalCooldown,
    IsActive,
    IsAutoAttack,
    IsPhysical,
    IsQueued,
    IssueAttackPause,
    KeepTargetWithinConeAngle,
    LuaScript,
    MaxPendingRequestAge,
    MutuallyExclusiveToBuff,
    NeedBuffGroupsToExecute,
    NeedBuffToExecute,
    OnlyCheckMiddleObstruction,
    PreventUseEffectWithoutTarget,
    RangeMax,
    RangeMin,
    Rank,
    RemoveActiveBuff,
    RemoveActiveBuffGroupsAndQuit,
    RequireRunningWhenActivated,
    RezoutWeapon,
    ScrambleCooldownOnCombat,
    SkillGroup,
    SourceMustBeAlive,
    TargetAbilityInfo,
    TargetFactory,
    TargetMustBeAlive,
    TargetMustBeDead,
    TargetMustBeOnGround,
    TargetType,
    TickPeriod,
    TriggerCooldown,
    UnequipWeapon,
    UsableInCombat,
    UsableOutOfCombat,
    UsableWithClassWeapon,
    UsableWithMainEffectGroup,
    UsableWithMeleeWeapon,
    UsableWithoutWeapon,
    UsableWithRangedWeapon,
    UsableWithSubEffectGroup,
    UseAfterKillTarget,
    UseBeforeSeekHelp,
    WarmUpDuration,
}
pub(crate) static EDNA_ABILITY_ATTRIBUTES: phf::Map<&'static str, EdnaAbility> = phf_map! {
    "abilityInfo" => EdnaAbility::AbilityInfo, "abilityType" => EdnaAbility::AbilityType,
    "abilityUsedCounter" => EdnaAbility::AbilityUsedCounter, "activationAngle" =>
    EdnaAbility::ActivationAngle, "activationType" => EdnaAbility::ActivationType,
    "affectEnemies" => EdnaAbility::AffectEnemies, "affectFriends" =>
    EdnaAbility::AffectFriends, "affectNeutral" => EdnaAbility::AffectNeutral,
    "affectSelf" => EdnaAbility::AffectSelf, "allowMoveWhileChanneling" =>
    EdnaAbility::AllowMoveWhileChanneling, "allowMoveWhileCharging" =>
    EdnaAbility::AllowMoveWhileCharging, "allowMoveWhileEndState" =>
    EdnaAbility::AllowMoveWhileEndState, "allowPlayerMoveWhileChanneling" =>
    EdnaAbility::AllowPlayerMoveWhileChanneling, "allowRangedAttackAdjustMove" =>
    EdnaAbility::AllowRangedAttackAdjustMove, "allowUseAboveHP" =>
    EdnaAbility::AllowUseAboveHp, "allowUseAfterKillNumberOfNPCs" =>
    EdnaAbility::AllowUseAfterKillNumberOfNpCs, "allowUseAfterKillNumberOfPlayers" =>
    EdnaAbility::AllowUseAfterKillNumberOfPlayers, "allowUseBelowHP" =>
    EdnaAbility::AllowUseBelowHp, "allowUseWhenAvailable" =>
    EdnaAbility::AllowUseWhenAvailable, "alwaysExecute" => EdnaAbility::AlwaysExecute,
    "attributes" => EdnaAbility::Attributes, "autoFaceTargetOrLocation" =>
    EdnaAbility::AutoFaceTargetOrLocation, "blockFaceTargetWhenPreparing" =>
    EdnaAbility::BlockFaceTargetWhenPreparing, "canBeBend" => EdnaAbility::CanBeBend,
    "canBeBlocked" => EdnaAbility::CanBeBlocked, "canBeInterrupted" =>
    EdnaAbility::CanBeInterrupted, "canBeReflected" => EdnaAbility::CanBeReflected,
    "canPredict" => EdnaAbility::CanPredict, "CastTime" => EdnaAbility::CastTime,
    "ChannelIndefinitely" => EdnaAbility::ChannelIndefinitely, "ChannelTime" =>
    EdnaAbility::ChannelTime, "charge" => EdnaAbility::Charge, "combatStyle" =>
    EdnaAbility::CombatStyle, "consumeCooldowns" => EdnaAbility::ConsumeCooldowns,
    "consumeEnergyInChannelTick" => EdnaAbility::ConsumeEnergyInChannelTick,
    "Description" => EdnaAbility::Description, "DisplayName" => EdnaAbility::DisplayName,
    "doNotCheckObstruction" => EdnaAbility::DoNotCheckObstruction, "effectorCastSettings"
    => EdnaAbility::EffectorCastSettings, "effectorChannelingSettings" =>
    EdnaAbility::EffectorChannelingSettings, "effectorSettings" =>
    EdnaAbility::EffectorSettings, "effectType" => EdnaAbility::EffectType,
    "emitCooldownOnCast" => EdnaAbility::EmitCooldownOnCast, "emitCooldownOnUse" =>
    EdnaAbility::EmitCooldownOnUse, "EnableInGame" => EdnaAbility::EnableInGame,
    "endStateDuration" => EdnaAbility::EndStateDuration, "energyConsumed" =>
    EdnaAbility::EnergyConsumed, "executionTime" => EdnaAbility::ExecutionTime,
    "externalCooldownsConsumed" => EdnaAbility::ExternalCooldownsConsumed,
    "externalCooldownsEmitted" => EdnaAbility::ExternalCooldownsEmitted,
    "GeneralPreRequisites" => EdnaAbility::GeneralPreRequisites, "Icon" =>
    EdnaAbility::Icon, "internalCooldown" => EdnaAbility::InternalCooldown, "isActive" =>
    EdnaAbility::IsActive, "isAutoAttack" => EdnaAbility::IsAutoAttack, "isPhysical" =>
    EdnaAbility::IsPhysical, "isQueued" => EdnaAbility::IsQueued, "issueAttackPause" =>
    EdnaAbility::IssueAttackPause, "keepTargetWithinConeAngle" =>
    EdnaAbility::KeepTargetWithinConeAngle, "luaScript" => EdnaAbility::LuaScript,
    "maxPendingRequestAge" => EdnaAbility::MaxPendingRequestAge,
    "mutuallyExclusiveToBuff" => EdnaAbility::MutuallyExclusiveToBuff,
    "needBuffGroupsToExecute" => EdnaAbility::NeedBuffGroupsToExecute,
    "needBuffToExecute" => EdnaAbility::NeedBuffToExecute, "onlyCheckMiddleObstruction"
    => EdnaAbility::OnlyCheckMiddleObstruction, "preventUseEffectWithoutTarget" =>
    EdnaAbility::PreventUseEffectWithoutTarget, "RangeMax" => EdnaAbility::RangeMax,
    "RangeMin" => EdnaAbility::RangeMin, "Rank" => EdnaAbility::Rank, "removeActiveBuff"
    => EdnaAbility::RemoveActiveBuff, "removeActiveBuffGroupsAndQuit" =>
    EdnaAbility::RemoveActiveBuffGroupsAndQuit, "requireRunningWhenActivated" =>
    EdnaAbility::RequireRunningWhenActivated, "rezoutWeapon" =>
    EdnaAbility::RezoutWeapon, "scrambleCooldownOnCombat" =>
    EdnaAbility::ScrambleCooldownOnCombat, "SkillGroup" => EdnaAbility::SkillGroup,
    "sourceMustBeAlive" => EdnaAbility::SourceMustBeAlive, "targetAbilityInfo" =>
    EdnaAbility::TargetAbilityInfo, "TargetFactory" => EdnaAbility::TargetFactory,
    "targetMustBeAlive" => EdnaAbility::TargetMustBeAlive, "targetMustBeDead" =>
    EdnaAbility::TargetMustBeDead, "targetMustBeOnGround" =>
    EdnaAbility::TargetMustBeOnGround, "targetType" => EdnaAbility::TargetType,
    "TickPeriod" => EdnaAbility::TickPeriod, "triggerCooldown" =>
    EdnaAbility::TriggerCooldown, "unequipWeapon" => EdnaAbility::UnequipWeapon,
    "usableInCombat" => EdnaAbility::UsableInCombat, "usableOutOfCombat" =>
    EdnaAbility::UsableOutOfCombat, "usableWithClassWeapon" =>
    EdnaAbility::UsableWithClassWeapon, "UsableWithMainEffectGroup" =>
    EdnaAbility::UsableWithMainEffectGroup, "usableWithMeleeWeapon" =>
    EdnaAbility::UsableWithMeleeWeapon, "usableWithoutWeapon" =>
    EdnaAbility::UsableWithoutWeapon, "usableWithRangedWeapon" =>
    EdnaAbility::UsableWithRangedWeapon, "UsableWithSubEffectGroup" =>
    EdnaAbility::UsableWithSubEffectGroup, "useAfterKillTarget" =>
    EdnaAbility::UseAfterKillTarget, "useBeforeSeekHelp" =>
    EdnaAbility::UseBeforeSeekHelp, "warmUpDuration" => EdnaAbility::WarmUpDuration,
};
pub(crate) static EDNA_ABILITY_ATTRIBUTES_ID: phf::Map<u16, EdnaAbility> = phf_map! {
    8883u16 => EdnaAbility::AbilityInfo, 6500u16 => EdnaAbility::AbilityType, 4773u16 =>
    EdnaAbility::AbilityUsedCounter, 241u16 => EdnaAbility::ActivationAngle, 264u16 =>
    EdnaAbility::ActivationType, 5213u16 => EdnaAbility::AffectEnemies, 255u16 =>
    EdnaAbility::AffectFriends, 9204u16 => EdnaAbility::AffectNeutral, 254u16 =>
    EdnaAbility::AffectSelf, 3035u16 => EdnaAbility::AllowMoveWhileChanneling, 3036u16 =>
    EdnaAbility::AllowMoveWhileCharging, 10022u16 => EdnaAbility::AllowMoveWhileEndState,
    8917u16 => EdnaAbility::AllowPlayerMoveWhileChanneling, 10034u16 =>
    EdnaAbility::AllowRangedAttackAdjustMove, 6730u16 => EdnaAbility::AllowUseAboveHp,
    9032u16 => EdnaAbility::AllowUseAfterKillNumberOfNpCs, 8890u16 =>
    EdnaAbility::AllowUseAfterKillNumberOfPlayers, 6729u16 =>
    EdnaAbility::AllowUseBelowHp, 8911u16 => EdnaAbility::AllowUseWhenAvailable, 7888u16
    => EdnaAbility::AlwaysExecute, 249u16 => EdnaAbility::Attributes, 10917u16 =>
    EdnaAbility::AutoFaceTargetOrLocation, 11471u16 =>
    EdnaAbility::BlockFaceTargetWhenPreparing, 11210u16 => EdnaAbility::CanBeBend,
    11208u16 => EdnaAbility::CanBeBlocked, 11206u16 => EdnaAbility::CanBeInterrupted,
    11209u16 => EdnaAbility::CanBeReflected, 8022u16 => EdnaAbility::CanPredict, 9625u16
    => EdnaAbility::CastTime, 10102u16 => EdnaAbility::ChannelIndefinitely, 9624u16 =>
    EdnaAbility::ChannelTime, 9385u16 => EdnaAbility::Charge, 4216u16 =>
    EdnaAbility::CombatStyle, 4030u16 => EdnaAbility::ConsumeCooldowns, 10626u16 =>
    EdnaAbility::ConsumeEnergyInChannelTick, 10622u16 => EdnaAbility::Description,
    4290u16 => EdnaAbility::DisplayName, 11291u16 => EdnaAbility::DoNotCheckObstruction,
    5556u16 => EdnaAbility::EffectorCastSettings, 240u16 =>
    EdnaAbility::EffectorChannelingSettings, 248u16 => EdnaAbility::EffectorSettings,
    260u16 => EdnaAbility::EffectType, 4062u16 => EdnaAbility::EmitCooldownOnCast,
    4061u16 => EdnaAbility::EmitCooldownOnUse, 6804u16 => EdnaAbility::EnableInGame,
    8936u16 => EdnaAbility::EndStateDuration, 4215u16 => EdnaAbility::EnergyConsumed,
    6776u16 => EdnaAbility::ExecutionTime, 11332u16 =>
    EdnaAbility::ExternalCooldownsConsumed, 11331u16 =>
    EdnaAbility::ExternalCooldownsEmitted, 11233u16 => EdnaAbility::GeneralPreRequisites,
    4335u16 => EdnaAbility::Icon, 4031u16 => EdnaAbility::InternalCooldown, 258u16 =>
    EdnaAbility::IsActive, 4221u16 => EdnaAbility::IsAutoAttack, 8037u16 =>
    EdnaAbility::IsPhysical, 4774u16 => EdnaAbility::IsQueued, 10918u16 =>
    EdnaAbility::IssueAttackPause, 10919u16 => EdnaAbility::KeepTargetWithinConeAngle,
    8906u16 => EdnaAbility::LuaScript, 8031u16 => EdnaAbility::MaxPendingRequestAge,
    9303u16 => EdnaAbility::MutuallyExclusiveToBuff, 11317u16 =>
    EdnaAbility::NeedBuffGroupsToExecute, 9293u16 => EdnaAbility::NeedBuffToExecute,
    12414u16 => EdnaAbility::OnlyCheckMiddleObstruction, 10835u16 =>
    EdnaAbility::PreventUseEffectWithoutTarget, 9621u16 => EdnaAbility::RangeMax, 9622u16
    => EdnaAbility::RangeMin, 11344u16 => EdnaAbility::Rank, 10396u16 =>
    EdnaAbility::RemoveActiveBuff, 11324u16 =>
    EdnaAbility::RemoveActiveBuffGroupsAndQuit, 6785u16 =>
    EdnaAbility::RequireRunningWhenActivated, 11290u16 => EdnaAbility::RezoutWeapon,
    11399u16 => EdnaAbility::ScrambleCooldownOnCombat, 11330u16 =>
    EdnaAbility::SkillGroup, 265u16 => EdnaAbility::SourceMustBeAlive, 8884u16 =>
    EdnaAbility::TargetAbilityInfo, 9626u16 => EdnaAbility::TargetFactory, 266u16 =>
    EdnaAbility::TargetMustBeAlive, 267u16 => EdnaAbility::TargetMustBeDead, 8810u16 =>
    EdnaAbility::TargetMustBeOnGround, 262u16 => EdnaAbility::TargetType, 9623u16 =>
    EdnaAbility::TickPeriod, 10138u16 => EdnaAbility::TriggerCooldown, 252u16 =>
    EdnaAbility::UnequipWeapon, 251u16 => EdnaAbility::UsableInCombat, 8889u16 =>
    EdnaAbility::UsableOutOfCombat, 10140u16 => EdnaAbility::UsableWithClassWeapon,
    11085u16 => EdnaAbility::UsableWithMainEffectGroup, 9295u16 =>
    EdnaAbility::UsableWithMeleeWeapon, 9296u16 => EdnaAbility::UsableWithoutWeapon,
    9294u16 => EdnaAbility::UsableWithRangedWeapon, 11086u16 =>
    EdnaAbility::UsableWithSubEffectGroup, 7153u16 => EdnaAbility::UseAfterKillTarget,
    7087u16 => EdnaAbility::UseBeforeSeekHelp, 5405u16 => EdnaAbility::WarmUpDuration,
};
impl Attribute for EdnaAbility {
    fn class() -> Class {
        Class::EdnaAbility
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AbilityInfo => &Self::AbilityInfo,
            Self::AbilityType => &Self::AbilityType,
            Self::AbilityUsedCounter => &Self::AbilityUsedCounter,
            Self::ActivationAngle => &Self::ActivationAngle,
            Self::ActivationType => &Self::ActivationType,
            Self::AffectEnemies => &Self::AffectEnemies,
            Self::AffectFriends => &Self::AffectFriends,
            Self::AffectNeutral => &Self::AffectNeutral,
            Self::AffectSelf => &Self::AffectSelf,
            Self::AllowMoveWhileChanneling => &Self::AllowMoveWhileChanneling,
            Self::AllowMoveWhileCharging => &Self::AllowMoveWhileCharging,
            Self::AllowMoveWhileEndState => &Self::AllowMoveWhileEndState,
            Self::AllowPlayerMoveWhileChanneling => &Self::AllowPlayerMoveWhileChanneling,
            Self::AllowRangedAttackAdjustMove => &Self::AllowRangedAttackAdjustMove,
            Self::AllowUseAboveHp => &Self::AllowUseAboveHp,
            Self::AllowUseAfterKillNumberOfNpCs => &Self::AllowUseAfterKillNumberOfNpCs,
            Self::AllowUseAfterKillNumberOfPlayers => {
                &Self::AllowUseAfterKillNumberOfPlayers
            }
            Self::AllowUseBelowHp => &Self::AllowUseBelowHp,
            Self::AllowUseWhenAvailable => &Self::AllowUseWhenAvailable,
            Self::AlwaysExecute => &Self::AlwaysExecute,
            Self::Attributes => &Self::Attributes,
            Self::AutoFaceTargetOrLocation => &Self::AutoFaceTargetOrLocation,
            Self::BlockFaceTargetWhenPreparing => &Self::BlockFaceTargetWhenPreparing,
            Self::CanBeBend => &Self::CanBeBend,
            Self::CanBeBlocked => &Self::CanBeBlocked,
            Self::CanBeInterrupted => &Self::CanBeInterrupted,
            Self::CanBeReflected => &Self::CanBeReflected,
            Self::CanPredict => &Self::CanPredict,
            Self::CastTime => &Self::CastTime,
            Self::ChannelIndefinitely => &Self::ChannelIndefinitely,
            Self::ChannelTime => &Self::ChannelTime,
            Self::Charge => &Self::Charge,
            Self::CombatStyle => &Self::CombatStyle,
            Self::ConsumeCooldowns => &Self::ConsumeCooldowns,
            Self::ConsumeEnergyInChannelTick => &Self::ConsumeEnergyInChannelTick,
            Self::Description => &Self::Description,
            Self::DisplayName => &Self::DisplayName,
            Self::DoNotCheckObstruction => &Self::DoNotCheckObstruction,
            Self::EffectorCastSettings => &Self::EffectorCastSettings,
            Self::EffectorChannelingSettings => &Self::EffectorChannelingSettings,
            Self::EffectorSettings => &Self::EffectorSettings,
            Self::EffectType => &Self::EffectType,
            Self::EmitCooldownOnCast => &Self::EmitCooldownOnCast,
            Self::EmitCooldownOnUse => &Self::EmitCooldownOnUse,
            Self::EnableInGame => &Self::EnableInGame,
            Self::EndStateDuration => &Self::EndStateDuration,
            Self::EnergyConsumed => &Self::EnergyConsumed,
            Self::ExecutionTime => &Self::ExecutionTime,
            Self::ExternalCooldownsConsumed => &Self::ExternalCooldownsConsumed,
            Self::ExternalCooldownsEmitted => &Self::ExternalCooldownsEmitted,
            Self::GeneralPreRequisites => &Self::GeneralPreRequisites,
            Self::Icon => &Self::Icon,
            Self::InternalCooldown => &Self::InternalCooldown,
            Self::IsActive => &Self::IsActive,
            Self::IsAutoAttack => &Self::IsAutoAttack,
            Self::IsPhysical => &Self::IsPhysical,
            Self::IsQueued => &Self::IsQueued,
            Self::IssueAttackPause => &Self::IssueAttackPause,
            Self::KeepTargetWithinConeAngle => &Self::KeepTargetWithinConeAngle,
            Self::LuaScript => &Self::LuaScript,
            Self::MaxPendingRequestAge => &Self::MaxPendingRequestAge,
            Self::MutuallyExclusiveToBuff => &Self::MutuallyExclusiveToBuff,
            Self::NeedBuffGroupsToExecute => &Self::NeedBuffGroupsToExecute,
            Self::NeedBuffToExecute => &Self::NeedBuffToExecute,
            Self::OnlyCheckMiddleObstruction => &Self::OnlyCheckMiddleObstruction,
            Self::PreventUseEffectWithoutTarget => &Self::PreventUseEffectWithoutTarget,
            Self::RangeMax => &Self::RangeMax,
            Self::RangeMin => &Self::RangeMin,
            Self::Rank => &Self::Rank,
            Self::RemoveActiveBuff => &Self::RemoveActiveBuff,
            Self::RemoveActiveBuffGroupsAndQuit => &Self::RemoveActiveBuffGroupsAndQuit,
            Self::RequireRunningWhenActivated => &Self::RequireRunningWhenActivated,
            Self::RezoutWeapon => &Self::RezoutWeapon,
            Self::ScrambleCooldownOnCombat => &Self::ScrambleCooldownOnCombat,
            Self::SkillGroup => &Self::SkillGroup,
            Self::SourceMustBeAlive => &Self::SourceMustBeAlive,
            Self::TargetAbilityInfo => &Self::TargetAbilityInfo,
            Self::TargetFactory => &Self::TargetFactory,
            Self::TargetMustBeAlive => &Self::TargetMustBeAlive,
            Self::TargetMustBeDead => &Self::TargetMustBeDead,
            Self::TargetMustBeOnGround => &Self::TargetMustBeOnGround,
            Self::TargetType => &Self::TargetType,
            Self::TickPeriod => &Self::TickPeriod,
            Self::TriggerCooldown => &Self::TriggerCooldown,
            Self::UnequipWeapon => &Self::UnequipWeapon,
            Self::UsableInCombat => &Self::UsableInCombat,
            Self::UsableOutOfCombat => &Self::UsableOutOfCombat,
            Self::UsableWithClassWeapon => &Self::UsableWithClassWeapon,
            Self::UsableWithMainEffectGroup => &Self::UsableWithMainEffectGroup,
            Self::UsableWithMeleeWeapon => &Self::UsableWithMeleeWeapon,
            Self::UsableWithoutWeapon => &Self::UsableWithoutWeapon,
            Self::UsableWithRangedWeapon => &Self::UsableWithRangedWeapon,
            Self::UsableWithSubEffectGroup => &Self::UsableWithSubEffectGroup,
            Self::UseAfterKillTarget => &Self::UseAfterKillTarget,
            Self::UseBeforeSeekHelp => &Self::UseBeforeSeekHelp,
            Self::WarmUpDuration => &Self::WarmUpDuration,
        }
    }
}
impl AttributeInfo for EdnaAbility {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AbilityInfo => 8883u16,
            Self::AbilityType => 6500u16,
            Self::AbilityUsedCounter => 4773u16,
            Self::ActivationAngle => 241u16,
            Self::ActivationType => 264u16,
            Self::AffectEnemies => 5213u16,
            Self::AffectFriends => 255u16,
            Self::AffectNeutral => 9204u16,
            Self::AffectSelf => 254u16,
            Self::AllowMoveWhileChanneling => 3035u16,
            Self::AllowMoveWhileCharging => 3036u16,
            Self::AllowMoveWhileEndState => 10022u16,
            Self::AllowPlayerMoveWhileChanneling => 8917u16,
            Self::AllowRangedAttackAdjustMove => 10034u16,
            Self::AllowUseAboveHp => 6730u16,
            Self::AllowUseAfterKillNumberOfNpCs => 9032u16,
            Self::AllowUseAfterKillNumberOfPlayers => 8890u16,
            Self::AllowUseBelowHp => 6729u16,
            Self::AllowUseWhenAvailable => 8911u16,
            Self::AlwaysExecute => 7888u16,
            Self::Attributes => 249u16,
            Self::AutoFaceTargetOrLocation => 10917u16,
            Self::BlockFaceTargetWhenPreparing => 11471u16,
            Self::CanBeBend => 11210u16,
            Self::CanBeBlocked => 11208u16,
            Self::CanBeInterrupted => 11206u16,
            Self::CanBeReflected => 11209u16,
            Self::CanPredict => 8022u16,
            Self::CastTime => 9625u16,
            Self::ChannelIndefinitely => 10102u16,
            Self::ChannelTime => 9624u16,
            Self::Charge => 9385u16,
            Self::CombatStyle => 4216u16,
            Self::ConsumeCooldowns => 4030u16,
            Self::ConsumeEnergyInChannelTick => 10626u16,
            Self::Description => 10622u16,
            Self::DisplayName => 4290u16,
            Self::DoNotCheckObstruction => 11291u16,
            Self::EffectorCastSettings => 5556u16,
            Self::EffectorChannelingSettings => 240u16,
            Self::EffectorSettings => 248u16,
            Self::EffectType => 260u16,
            Self::EmitCooldownOnCast => 4062u16,
            Self::EmitCooldownOnUse => 4061u16,
            Self::EnableInGame => 6804u16,
            Self::EndStateDuration => 8936u16,
            Self::EnergyConsumed => 4215u16,
            Self::ExecutionTime => 6776u16,
            Self::ExternalCooldownsConsumed => 11332u16,
            Self::ExternalCooldownsEmitted => 11331u16,
            Self::GeneralPreRequisites => 11233u16,
            Self::Icon => 4335u16,
            Self::InternalCooldown => 4031u16,
            Self::IsActive => 258u16,
            Self::IsAutoAttack => 4221u16,
            Self::IsPhysical => 8037u16,
            Self::IsQueued => 4774u16,
            Self::IssueAttackPause => 10918u16,
            Self::KeepTargetWithinConeAngle => 10919u16,
            Self::LuaScript => 8906u16,
            Self::MaxPendingRequestAge => 8031u16,
            Self::MutuallyExclusiveToBuff => 9303u16,
            Self::NeedBuffGroupsToExecute => 11317u16,
            Self::NeedBuffToExecute => 9293u16,
            Self::OnlyCheckMiddleObstruction => 12414u16,
            Self::PreventUseEffectWithoutTarget => 10835u16,
            Self::RangeMax => 9621u16,
            Self::RangeMin => 9622u16,
            Self::Rank => 11344u16,
            Self::RemoveActiveBuff => 10396u16,
            Self::RemoveActiveBuffGroupsAndQuit => 11324u16,
            Self::RequireRunningWhenActivated => 6785u16,
            Self::RezoutWeapon => 11290u16,
            Self::ScrambleCooldownOnCombat => 11399u16,
            Self::SkillGroup => 11330u16,
            Self::SourceMustBeAlive => 265u16,
            Self::TargetAbilityInfo => 8884u16,
            Self::TargetFactory => 9626u16,
            Self::TargetMustBeAlive => 266u16,
            Self::TargetMustBeDead => 267u16,
            Self::TargetMustBeOnGround => 8810u16,
            Self::TargetType => 262u16,
            Self::TickPeriod => 9623u16,
            Self::TriggerCooldown => 10138u16,
            Self::UnequipWeapon => 252u16,
            Self::UsableInCombat => 251u16,
            Self::UsableOutOfCombat => 8889u16,
            Self::UsableWithClassWeapon => 10140u16,
            Self::UsableWithMainEffectGroup => 11085u16,
            Self::UsableWithMeleeWeapon => 9295u16,
            Self::UsableWithoutWeapon => 9296u16,
            Self::UsableWithRangedWeapon => 9294u16,
            Self::UsableWithSubEffectGroup => 11086u16,
            Self::UseAfterKillTarget => 7153u16,
            Self::UseBeforeSeekHelp => 7087u16,
            Self::WarmUpDuration => 5405u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AbilityInfo => "abilityInfo",
            Self::AbilityType => "abilityType",
            Self::AbilityUsedCounter => "abilityUsedCounter",
            Self::ActivationAngle => "activationAngle",
            Self::ActivationType => "activationType",
            Self::AffectEnemies => "affectEnemies",
            Self::AffectFriends => "affectFriends",
            Self::AffectNeutral => "affectNeutral",
            Self::AffectSelf => "affectSelf",
            Self::AllowMoveWhileChanneling => "allowMoveWhileChanneling",
            Self::AllowMoveWhileCharging => "allowMoveWhileCharging",
            Self::AllowMoveWhileEndState => "allowMoveWhileEndState",
            Self::AllowPlayerMoveWhileChanneling => "allowPlayerMoveWhileChanneling",
            Self::AllowRangedAttackAdjustMove => "allowRangedAttackAdjustMove",
            Self::AllowUseAboveHp => "allowUseAboveHP",
            Self::AllowUseAfterKillNumberOfNpCs => "allowUseAfterKillNumberOfNPCs",
            Self::AllowUseAfterKillNumberOfPlayers => "allowUseAfterKillNumberOfPlayers",
            Self::AllowUseBelowHp => "allowUseBelowHP",
            Self::AllowUseWhenAvailable => "allowUseWhenAvailable",
            Self::AlwaysExecute => "alwaysExecute",
            Self::Attributes => "attributes",
            Self::AutoFaceTargetOrLocation => "autoFaceTargetOrLocation",
            Self::BlockFaceTargetWhenPreparing => "blockFaceTargetWhenPreparing",
            Self::CanBeBend => "canBeBend",
            Self::CanBeBlocked => "canBeBlocked",
            Self::CanBeInterrupted => "canBeInterrupted",
            Self::CanBeReflected => "canBeReflected",
            Self::CanPredict => "canPredict",
            Self::CastTime => "CastTime",
            Self::ChannelIndefinitely => "ChannelIndefinitely",
            Self::ChannelTime => "ChannelTime",
            Self::Charge => "charge",
            Self::CombatStyle => "combatStyle",
            Self::ConsumeCooldowns => "consumeCooldowns",
            Self::ConsumeEnergyInChannelTick => "consumeEnergyInChannelTick",
            Self::Description => "Description",
            Self::DisplayName => "DisplayName",
            Self::DoNotCheckObstruction => "doNotCheckObstruction",
            Self::EffectorCastSettings => "effectorCastSettings",
            Self::EffectorChannelingSettings => "effectorChannelingSettings",
            Self::EffectorSettings => "effectorSettings",
            Self::EffectType => "effectType",
            Self::EmitCooldownOnCast => "emitCooldownOnCast",
            Self::EmitCooldownOnUse => "emitCooldownOnUse",
            Self::EnableInGame => "EnableInGame",
            Self::EndStateDuration => "endStateDuration",
            Self::EnergyConsumed => "energyConsumed",
            Self::ExecutionTime => "executionTime",
            Self::ExternalCooldownsConsumed => "externalCooldownsConsumed",
            Self::ExternalCooldownsEmitted => "externalCooldownsEmitted",
            Self::GeneralPreRequisites => "GeneralPreRequisites",
            Self::Icon => "Icon",
            Self::InternalCooldown => "internalCooldown",
            Self::IsActive => "isActive",
            Self::IsAutoAttack => "isAutoAttack",
            Self::IsPhysical => "isPhysical",
            Self::IsQueued => "isQueued",
            Self::IssueAttackPause => "issueAttackPause",
            Self::KeepTargetWithinConeAngle => "keepTargetWithinConeAngle",
            Self::LuaScript => "luaScript",
            Self::MaxPendingRequestAge => "maxPendingRequestAge",
            Self::MutuallyExclusiveToBuff => "mutuallyExclusiveToBuff",
            Self::NeedBuffGroupsToExecute => "needBuffGroupsToExecute",
            Self::NeedBuffToExecute => "needBuffToExecute",
            Self::OnlyCheckMiddleObstruction => "onlyCheckMiddleObstruction",
            Self::PreventUseEffectWithoutTarget => "preventUseEffectWithoutTarget",
            Self::RangeMax => "RangeMax",
            Self::RangeMin => "RangeMin",
            Self::Rank => "Rank",
            Self::RemoveActiveBuff => "removeActiveBuff",
            Self::RemoveActiveBuffGroupsAndQuit => "removeActiveBuffGroupsAndQuit",
            Self::RequireRunningWhenActivated => "requireRunningWhenActivated",
            Self::RezoutWeapon => "rezoutWeapon",
            Self::ScrambleCooldownOnCombat => "scrambleCooldownOnCombat",
            Self::SkillGroup => "SkillGroup",
            Self::SourceMustBeAlive => "sourceMustBeAlive",
            Self::TargetAbilityInfo => "targetAbilityInfo",
            Self::TargetFactory => "TargetFactory",
            Self::TargetMustBeAlive => "targetMustBeAlive",
            Self::TargetMustBeDead => "targetMustBeDead",
            Self::TargetMustBeOnGround => "targetMustBeOnGround",
            Self::TargetType => "targetType",
            Self::TickPeriod => "TickPeriod",
            Self::TriggerCooldown => "triggerCooldown",
            Self::UnequipWeapon => "unequipWeapon",
            Self::UsableInCombat => "usableInCombat",
            Self::UsableOutOfCombat => "usableOutOfCombat",
            Self::UsableWithClassWeapon => "usableWithClassWeapon",
            Self::UsableWithMainEffectGroup => "UsableWithMainEffectGroup",
            Self::UsableWithMeleeWeapon => "usableWithMeleeWeapon",
            Self::UsableWithoutWeapon => "usableWithoutWeapon",
            Self::UsableWithRangedWeapon => "usableWithRangedWeapon",
            Self::UsableWithSubEffectGroup => "UsableWithSubEffectGroup",
            Self::UseAfterKillTarget => "useAfterKillTarget",
            Self::UseBeforeSeekHelp => "useBeforeSeekHelp",
            Self::WarmUpDuration => "warmUpDuration",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AbilityInfo => ParamType::String,
            Self::AbilityType => ParamType::String,
            Self::AbilityUsedCounter => ParamType::Int,
            Self::ActivationAngle => ParamType::Float,
            Self::ActivationType => ParamType::String,
            Self::AffectEnemies => ParamType::Bool,
            Self::AffectFriends => ParamType::Bool,
            Self::AffectNeutral => ParamType::Bool,
            Self::AffectSelf => ParamType::Bool,
            Self::AllowMoveWhileChanneling => ParamType::Bool,
            Self::AllowMoveWhileCharging => ParamType::Bool,
            Self::AllowMoveWhileEndState => ParamType::Bool,
            Self::AllowPlayerMoveWhileChanneling => ParamType::Bool,
            Self::AllowRangedAttackAdjustMove => ParamType::Bool,
            Self::AllowUseAboveHp => ParamType::Float,
            Self::AllowUseAfterKillNumberOfNpCs => ParamType::Int,
            Self::AllowUseAfterKillNumberOfPlayers => ParamType::Int,
            Self::AllowUseBelowHp => ParamType::Float,
            Self::AllowUseWhenAvailable => ParamType::Bool,
            Self::AlwaysExecute => ParamType::Bool,
            Self::Attributes => ParamType::JsonValue,
            Self::AutoFaceTargetOrLocation => ParamType::Bool,
            Self::BlockFaceTargetWhenPreparing => ParamType::Bool,
            Self::CanBeBend => ParamType::Bool,
            Self::CanBeBlocked => ParamType::Bool,
            Self::CanBeInterrupted => ParamType::Bool,
            Self::CanBeReflected => ParamType::Bool,
            Self::CanPredict => ParamType::Bool,
            Self::CastTime => ParamType::Float,
            Self::ChannelIndefinitely => ParamType::Bool,
            Self::ChannelTime => ParamType::Float,
            Self::Charge => ParamType::Int,
            Self::CombatStyle => ParamType::Int,
            Self::ConsumeCooldowns => ParamType::JsonValue,
            Self::ConsumeEnergyInChannelTick => ParamType::Bool,
            Self::Description => ParamType::LocalizedString,
            Self::DisplayName => ParamType::LocalizedString,
            Self::DoNotCheckObstruction => ParamType::Bool,
            Self::EffectorCastSettings => ParamType::JsonValue,
            Self::EffectorChannelingSettings => ParamType::JsonValue,
            Self::EffectorSettings => ParamType::JsonValue,
            Self::EffectType => ParamType::String,
            Self::EmitCooldownOnCast => ParamType::JsonValue,
            Self::EmitCooldownOnUse => ParamType::JsonValue,
            Self::EnableInGame => ParamType::Bool,
            Self::EndStateDuration => ParamType::Float,
            Self::EnergyConsumed => ParamType::Float,
            Self::ExecutionTime => ParamType::Float,
            Self::ExternalCooldownsConsumed => ParamType::ContentRefList,
            Self::ExternalCooldownsEmitted => ParamType::ContentRefList,
            Self::GeneralPreRequisites => ParamType::String,
            Self::Icon => ParamType::String,
            Self::InternalCooldown => ParamType::Float,
            Self::IsActive => ParamType::Bool,
            Self::IsAutoAttack => ParamType::Bool,
            Self::IsPhysical => ParamType::Bool,
            Self::IsQueued => ParamType::Bool,
            Self::IssueAttackPause => ParamType::Bool,
            Self::KeepTargetWithinConeAngle => ParamType::Float,
            Self::LuaScript => ParamType::String,
            Self::MaxPendingRequestAge => ParamType::Float,
            Self::MutuallyExclusiveToBuff => ParamType::ContentRef,
            Self::NeedBuffGroupsToExecute => ParamType::ContentRefList,
            Self::NeedBuffToExecute => ParamType::ContentRefList,
            Self::OnlyCheckMiddleObstruction => ParamType::Bool,
            Self::PreventUseEffectWithoutTarget => ParamType::Bool,
            Self::RangeMax => ParamType::Float,
            Self::RangeMin => ParamType::Float,
            Self::Rank => ParamType::Int,
            Self::RemoveActiveBuff => ParamType::ContentRef,
            Self::RemoveActiveBuffGroupsAndQuit => ParamType::ContentRefList,
            Self::RequireRunningWhenActivated => ParamType::Bool,
            Self::RezoutWeapon => ParamType::Bool,
            Self::ScrambleCooldownOnCombat => ParamType::Bool,
            Self::SkillGroup => ParamType::ContentRef,
            Self::SourceMustBeAlive => ParamType::Bool,
            Self::TargetAbilityInfo => ParamType::String,
            Self::TargetFactory => ParamType::JsonValue,
            Self::TargetMustBeAlive => ParamType::Bool,
            Self::TargetMustBeDead => ParamType::Bool,
            Self::TargetMustBeOnGround => ParamType::Bool,
            Self::TargetType => ParamType::String,
            Self::TickPeriod => ParamType::Float,
            Self::TriggerCooldown => ParamType::Bool,
            Self::UnequipWeapon => ParamType::Bool,
            Self::UsableInCombat => ParamType::Bool,
            Self::UsableOutOfCombat => ParamType::Bool,
            Self::UsableWithClassWeapon => ParamType::Int,
            Self::UsableWithMainEffectGroup => ParamType::VectorInt,
            Self::UsableWithMeleeWeapon => ParamType::Bool,
            Self::UsableWithoutWeapon => ParamType::Bool,
            Self::UsableWithRangedWeapon => ParamType::Bool,
            Self::UsableWithSubEffectGroup => ParamType::VectorInt,
            Self::UseAfterKillTarget => ParamType::Bool,
            Self::UseBeforeSeekHelp => ParamType::Bool,
            Self::WarmUpDuration => ParamType::Float,
        }
    }
    fn default(&self) -> &'static Value {
        static ABILITY_INFO: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ABILITY_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            "Normal".to_string(),
        ));
        static ABILITY_USED_COUNTER: Value = Value::Int(0i32);
        static ACTIVATION_ANGLE: Value = Value::Float(120f32);
        static ACTIVATION_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AFFECT_ENEMIES: Value = Value::Bool(true);
        static AFFECT_FRIENDS: Value = Value::Bool(false);
        static AFFECT_NEUTRAL: Value = Value::Bool(false);
        static AFFECT_SELF: Value = Value::Bool(false);
        static ALLOW_MOVE_WHILE_CHANNELING: Value = Value::Bool(true);
        static ALLOW_MOVE_WHILE_CHARGING: Value = Value::Bool(true);
        static ALLOW_MOVE_WHILE_END_STATE: Value = Value::Bool(true);
        static ALLOW_PLAYER_MOVE_WHILE_CHANNELING: Value = Value::Bool(true);
        static ALLOW_RANGED_ATTACK_ADJUST_MOVE: Value = Value::Bool(true);
        static ALLOW_USE_ABOVE_HP: Value = Value::Float(0f32);
        static ALLOW_USE_AFTER_KILL_NUMBER_OF_NP_CS: Value = Value::Int(0i32);
        static ALLOW_USE_AFTER_KILL_NUMBER_OF_PLAYERS: Value = Value::Int(0i32);
        static ALLOW_USE_BELOW_HP: Value = Value::Float(1f32);
        static ALLOW_USE_WHEN_AVAILABLE: Value = Value::Bool(false);
        static ALWAYS_EXECUTE: Value = Value::Bool(false);
        static ATTRIBUTES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static AUTO_FACE_TARGET_OR_LOCATION: Value = Value::Bool(true);
        static BLOCK_FACE_TARGET_WHEN_PREPARING: Value = Value::Bool(true);
        static CAN_BE_BEND: Value = Value::Bool(true);
        static CAN_BE_BLOCKED: Value = Value::Bool(true);
        static CAN_BE_INTERRUPTED: Value = Value::Bool(true);
        static CAN_BE_REFLECTED: Value = Value::Bool(true);
        static CAN_PREDICT: Value = Value::Bool(false);
        static CAST_TIME: Value = Value::Float(0f32);
        static CHANNEL_INDEFINITELY: Value = Value::Bool(false);
        static CHANNEL_TIME: Value = Value::Float(0f32);
        static CHARGE: Value = Value::Int(-1i32);
        static COMBAT_STYLE: Value = Value::Int(0i32);
        static CONSUME_COOLDOWNS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "[\"globalCooldownGroup\",\"globalCastCooldownGroup\",\"globalChannelCooldownGroup\"]",
                )
                .unwrap(),
        ));
        static CONSUME_ENERGY_IN_CHANNEL_TICK: Value = Value::Bool(false);
        static DESCRIPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DO_NOT_CHECK_OBSTRUCTION: Value = Value::Bool(false);
        static EFFECTOR_CAST_SETTINGS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static EFFECTOR_CHANNELING_SETTINGS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static EFFECTOR_SETTINGS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static EFFECT_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            "Others".to_string(),
        ));
        static EMIT_COOLDOWN_ON_CAST: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[]").unwrap(),
        ));
        static EMIT_COOLDOWN_ON_USE: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[\"globalCooldownGroup\"]").unwrap(),
        ));
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static END_STATE_DURATION: Value = Value::Float(0f32);
        static ENERGY_CONSUMED: Value = Value::Float(0f32);
        static EXECUTION_TIME: Value = Value::Float(1.5f32);
        static EXTERNAL_COOLDOWNS_CONSUMED: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            "[182:7daff75b-6078-419b-aa75-c06799b21bf8]".parse().unwrap_or_default(),
        ));
        static EXTERNAL_COOLDOWNS_EMITTED: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            "[182:7daff75b-6078-419b-aa75-c06799b21bf8]".parse().unwrap_or_default(),
        ));
        static GENERAL_PRE_REQUISITES: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static INTERNAL_COOLDOWN: Value = Value::Float(3f32);
        static IS_ACTIVE: Value = Value::Bool(false);
        static IS_AUTO_ATTACK: Value = Value::Bool(false);
        static IS_PHYSICAL: Value = Value::Bool(true);
        static IS_QUEUED: Value = Value::Bool(false);
        static ISSUE_ATTACK_PAUSE: Value = Value::Bool(true);
        static KEEP_TARGET_WITHIN_CONE_ANGLE: Value = Value::Float(90f32);
        static LUA_SCRIPT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static MAX_PENDING_REQUEST_AGE: Value = Value::Float(1f32);
        static MUTUALLY_EXCLUSIVE_TO_BUFF: Lazy<Value> = Lazy::new(|| Value::ContentRef(
            None,
        ));
        static NEED_BUFF_GROUPS_TO_EXECUTE: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static NEED_BUFF_TO_EXECUTE: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static ONLY_CHECK_MIDDLE_OBSTRUCTION: Value = Value::Bool(true);
        static PREVENT_USE_EFFECT_WITHOUT_TARGET: Value = Value::Bool(false);
        static RANGE_MAX: Value = Value::Float(50f32);
        static RANGE_MIN: Value = Value::Float(0f32);
        static RANK: Value = Value::Int(0i32);
        static REMOVE_ACTIVE_BUFF: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static REMOVE_ACTIVE_BUFF_GROUPS_AND_QUIT: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static REQUIRE_RUNNING_WHEN_ACTIVATED: Value = Value::Bool(false);
        static REZOUT_WEAPON: Value = Value::Bool(false);
        static SCRAMBLE_COOLDOWN_ON_COMBAT: Value = Value::Bool(true);
        static SKILL_GROUP: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static SOURCE_MUST_BE_ALIVE: Value = Value::Bool(false);
        static TARGET_ABILITY_INFO: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static TARGET_FACTORY: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static TARGET_MUST_BE_ALIVE: Value = Value::Bool(true);
        static TARGET_MUST_BE_DEAD: Value = Value::Bool(false);
        static TARGET_MUST_BE_ON_GROUND: Value = Value::Bool(true);
        static TARGET_TYPE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static TICK_PERIOD: Value = Value::Float(0f32);
        static TRIGGER_COOLDOWN: Value = Value::Bool(true);
        static UNEQUIP_WEAPON: Value = Value::Bool(false);
        static USABLE_IN_COMBAT: Value = Value::Bool(true);
        static USABLE_OUT_OF_COMBAT: Value = Value::Bool(false);
        static USABLE_WITH_CLASS_WEAPON: Value = Value::Int(-1i32);
        static USABLE_WITH_MAIN_EFFECT_GROUP: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static USABLE_WITH_MELEE_WEAPON: Value = Value::Bool(true);
        static USABLE_WITHOUT_WEAPON: Value = Value::Bool(true);
        static USABLE_WITH_RANGED_WEAPON: Value = Value::Bool(true);
        static USABLE_WITH_SUB_EFFECT_GROUP: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static USE_AFTER_KILL_TARGET: Value = Value::Bool(false);
        static USE_BEFORE_SEEK_HELP: Value = Value::Bool(false);
        static WARM_UP_DURATION: Value = Value::Float(0f32);
        match self {
            Self::AbilityInfo => &ABILITY_INFO,
            Self::AbilityType => &ABILITY_TYPE,
            Self::AbilityUsedCounter => &ABILITY_USED_COUNTER,
            Self::ActivationAngle => &ACTIVATION_ANGLE,
            Self::ActivationType => &ACTIVATION_TYPE,
            Self::AffectEnemies => &AFFECT_ENEMIES,
            Self::AffectFriends => &AFFECT_FRIENDS,
            Self::AffectNeutral => &AFFECT_NEUTRAL,
            Self::AffectSelf => &AFFECT_SELF,
            Self::AllowMoveWhileChanneling => &ALLOW_MOVE_WHILE_CHANNELING,
            Self::AllowMoveWhileCharging => &ALLOW_MOVE_WHILE_CHARGING,
            Self::AllowMoveWhileEndState => &ALLOW_MOVE_WHILE_END_STATE,
            Self::AllowPlayerMoveWhileChanneling => &ALLOW_PLAYER_MOVE_WHILE_CHANNELING,
            Self::AllowRangedAttackAdjustMove => &ALLOW_RANGED_ATTACK_ADJUST_MOVE,
            Self::AllowUseAboveHp => &ALLOW_USE_ABOVE_HP,
            Self::AllowUseAfterKillNumberOfNpCs => &ALLOW_USE_AFTER_KILL_NUMBER_OF_NP_CS,
            Self::AllowUseAfterKillNumberOfPlayers => {
                &ALLOW_USE_AFTER_KILL_NUMBER_OF_PLAYERS
            }
            Self::AllowUseBelowHp => &ALLOW_USE_BELOW_HP,
            Self::AllowUseWhenAvailable => &ALLOW_USE_WHEN_AVAILABLE,
            Self::AlwaysExecute => &ALWAYS_EXECUTE,
            Self::Attributes => &ATTRIBUTES,
            Self::AutoFaceTargetOrLocation => &AUTO_FACE_TARGET_OR_LOCATION,
            Self::BlockFaceTargetWhenPreparing => &BLOCK_FACE_TARGET_WHEN_PREPARING,
            Self::CanBeBend => &CAN_BE_BEND,
            Self::CanBeBlocked => &CAN_BE_BLOCKED,
            Self::CanBeInterrupted => &CAN_BE_INTERRUPTED,
            Self::CanBeReflected => &CAN_BE_REFLECTED,
            Self::CanPredict => &CAN_PREDICT,
            Self::CastTime => &CAST_TIME,
            Self::ChannelIndefinitely => &CHANNEL_INDEFINITELY,
            Self::ChannelTime => &CHANNEL_TIME,
            Self::Charge => &CHARGE,
            Self::CombatStyle => &COMBAT_STYLE,
            Self::ConsumeCooldowns => &CONSUME_COOLDOWNS,
            Self::ConsumeEnergyInChannelTick => &CONSUME_ENERGY_IN_CHANNEL_TICK,
            Self::Description => &DESCRIPTION,
            Self::DisplayName => &DISPLAY_NAME,
            Self::DoNotCheckObstruction => &DO_NOT_CHECK_OBSTRUCTION,
            Self::EffectorCastSettings => &EFFECTOR_CAST_SETTINGS,
            Self::EffectorChannelingSettings => &EFFECTOR_CHANNELING_SETTINGS,
            Self::EffectorSettings => &EFFECTOR_SETTINGS,
            Self::EffectType => &EFFECT_TYPE,
            Self::EmitCooldownOnCast => &EMIT_COOLDOWN_ON_CAST,
            Self::EmitCooldownOnUse => &EMIT_COOLDOWN_ON_USE,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::EndStateDuration => &END_STATE_DURATION,
            Self::EnergyConsumed => &ENERGY_CONSUMED,
            Self::ExecutionTime => &EXECUTION_TIME,
            Self::ExternalCooldownsConsumed => &EXTERNAL_COOLDOWNS_CONSUMED,
            Self::ExternalCooldownsEmitted => &EXTERNAL_COOLDOWNS_EMITTED,
            Self::GeneralPreRequisites => &GENERAL_PRE_REQUISITES,
            Self::Icon => &ICON,
            Self::InternalCooldown => &INTERNAL_COOLDOWN,
            Self::IsActive => &IS_ACTIVE,
            Self::IsAutoAttack => &IS_AUTO_ATTACK,
            Self::IsPhysical => &IS_PHYSICAL,
            Self::IsQueued => &IS_QUEUED,
            Self::IssueAttackPause => &ISSUE_ATTACK_PAUSE,
            Self::KeepTargetWithinConeAngle => &KEEP_TARGET_WITHIN_CONE_ANGLE,
            Self::LuaScript => &LUA_SCRIPT,
            Self::MaxPendingRequestAge => &MAX_PENDING_REQUEST_AGE,
            Self::MutuallyExclusiveToBuff => &MUTUALLY_EXCLUSIVE_TO_BUFF,
            Self::NeedBuffGroupsToExecute => &NEED_BUFF_GROUPS_TO_EXECUTE,
            Self::NeedBuffToExecute => &NEED_BUFF_TO_EXECUTE,
            Self::OnlyCheckMiddleObstruction => &ONLY_CHECK_MIDDLE_OBSTRUCTION,
            Self::PreventUseEffectWithoutTarget => &PREVENT_USE_EFFECT_WITHOUT_TARGET,
            Self::RangeMax => &RANGE_MAX,
            Self::RangeMin => &RANGE_MIN,
            Self::Rank => &RANK,
            Self::RemoveActiveBuff => &REMOVE_ACTIVE_BUFF,
            Self::RemoveActiveBuffGroupsAndQuit => &REMOVE_ACTIVE_BUFF_GROUPS_AND_QUIT,
            Self::RequireRunningWhenActivated => &REQUIRE_RUNNING_WHEN_ACTIVATED,
            Self::RezoutWeapon => &REZOUT_WEAPON,
            Self::ScrambleCooldownOnCombat => &SCRAMBLE_COOLDOWN_ON_COMBAT,
            Self::SkillGroup => &SKILL_GROUP,
            Self::SourceMustBeAlive => &SOURCE_MUST_BE_ALIVE,
            Self::TargetAbilityInfo => &TARGET_ABILITY_INFO,
            Self::TargetFactory => &TARGET_FACTORY,
            Self::TargetMustBeAlive => &TARGET_MUST_BE_ALIVE,
            Self::TargetMustBeDead => &TARGET_MUST_BE_DEAD,
            Self::TargetMustBeOnGround => &TARGET_MUST_BE_ON_GROUND,
            Self::TargetType => &TARGET_TYPE,
            Self::TickPeriod => &TICK_PERIOD,
            Self::TriggerCooldown => &TRIGGER_COOLDOWN,
            Self::UnequipWeapon => &UNEQUIP_WEAPON,
            Self::UsableInCombat => &USABLE_IN_COMBAT,
            Self::UsableOutOfCombat => &USABLE_OUT_OF_COMBAT,
            Self::UsableWithClassWeapon => &USABLE_WITH_CLASS_WEAPON,
            Self::UsableWithMainEffectGroup => &USABLE_WITH_MAIN_EFFECT_GROUP,
            Self::UsableWithMeleeWeapon => &USABLE_WITH_MELEE_WEAPON,
            Self::UsableWithoutWeapon => &USABLE_WITHOUT_WEAPON,
            Self::UsableWithRangedWeapon => &USABLE_WITH_RANGED_WEAPON,
            Self::UsableWithSubEffectGroup => &USABLE_WITH_SUB_EFFECT_GROUP,
            Self::UseAfterKillTarget => &USE_AFTER_KILL_TARGET,
            Self::UseBeforeSeekHelp => &USE_BEFORE_SEEK_HELP,
            Self::WarmUpDuration => &WARM_UP_DURATION,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AbilityInfo => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AbilityType => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AbilityUsedCounter => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ActivationAngle => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::ActivationType => &[ParamFlag::Content],
            Self::AffectEnemies => &[ParamFlag::Content],
            Self::AffectFriends => &[ParamFlag::Content],
            Self::AffectNeutral => &[ParamFlag::Content, ParamFlag::Deprecated],
            Self::AffectSelf => &[ParamFlag::Content],
            Self::AllowMoveWhileChanneling => &[ParamFlag::Content],
            Self::AllowMoveWhileCharging => &[ParamFlag::Content],
            Self::AllowMoveWhileEndState => &[ParamFlag::Content],
            Self::AllowPlayerMoveWhileChanneling => &[ParamFlag::Content],
            Self::AllowRangedAttackAdjustMove => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AllowUseAboveHp => &[ParamFlag::Content],
            Self::AllowUseAfterKillNumberOfNpCs => &[ParamFlag::Content],
            Self::AllowUseAfterKillNumberOfPlayers => &[ParamFlag::Content],
            Self::AllowUseBelowHp => &[ParamFlag::Content],
            Self::AllowUseWhenAvailable => &[ParamFlag::Content],
            Self::AlwaysExecute => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Attributes => &[ParamFlag::Content, ParamFlag::Deprecated],
            Self::AutoFaceTargetOrLocation => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::BlockFaceTargetWhenPreparing => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CanBeBend => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CanBeBlocked => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CanBeInterrupted => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CanBeReflected => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CanPredict => &[ParamFlag::Content],
            Self::CastTime => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ChannelIndefinitely => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ChannelTime => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Charge => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CombatStyle => &[ParamFlag::Deprecated],
            Self::ConsumeCooldowns => &[ParamFlag::Content],
            Self::ConsumeEnergyInChannelTick => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Description => &[ParamFlag::Content],
            Self::DisplayName => &[ParamFlag::Content],
            Self::DoNotCheckObstruction => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EffectorCastSettings => &[ParamFlag::Content],
            Self::EffectorChannelingSettings => &[ParamFlag::Content],
            Self::EffectorSettings => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::EffectType => &[ParamFlag::Content],
            Self::EmitCooldownOnCast => &[ParamFlag::Content],
            Self::EmitCooldownOnUse => &[ParamFlag::Content],
            Self::EnableInGame => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EndStateDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EnergyConsumed => &[ParamFlag::Content],
            Self::ExecutionTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::ExternalCooldownsConsumed => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::ExternalCooldownsEmitted => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::GeneralPreRequisites => &[ParamFlag::Content],
            Self::Icon => &[ParamFlag::Content],
            Self::InternalCooldown => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::IsActive => &[ParamFlag::NodeOwn],
            Self::IsAutoAttack => &[ParamFlag::Content],
            Self::IsPhysical => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsQueued => &[ParamFlag::NodeOwn],
            Self::IssueAttackPause => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::KeepTargetWithinConeAngle => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::LuaScript => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::MaxPendingRequestAge => &[ParamFlag::Content],
            Self::MutuallyExclusiveToBuff => {
                &[ParamFlag::Persistent, ParamFlag::Deprecated]
            }
            Self::NeedBuffGroupsToExecute => &[ParamFlag::Content],
            Self::NeedBuffToExecute => &[ParamFlag::Content],
            Self::OnlyCheckMiddleObstruction => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::PreventUseEffectWithoutTarget => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RangeMax => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::RangeMin => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Rank => &[ParamFlag::Content],
            Self::RemoveActiveBuff => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::RemoveActiveBuffGroupsAndQuit => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RequireRunningWhenActivated => {
                &[ParamFlag::NodeOwn, ParamFlag::Content]
            }
            Self::RezoutWeapon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ScrambleCooldownOnCombat => &[ParamFlag::Content],
            Self::SkillGroup => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SourceMustBeAlive => &[ParamFlag::Content],
            Self::TargetAbilityInfo => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::TargetFactory => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TargetMustBeAlive => &[ParamFlag::Content],
            Self::TargetMustBeDead => &[ParamFlag::Content],
            Self::TargetMustBeOnGround => &[ParamFlag::Content],
            Self::TargetType => &[ParamFlag::Content],
            Self::TickPeriod => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TriggerCooldown => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UnequipWeapon => &[ParamFlag::Content],
            Self::UsableInCombat => &[ParamFlag::Content],
            Self::UsableOutOfCombat => &[ParamFlag::Content],
            Self::UsableWithClassWeapon => &[ParamFlag::Content],
            Self::UsableWithMainEffectGroup => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::UsableWithMeleeWeapon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UsableWithoutWeapon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UsableWithRangedWeapon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UsableWithSubEffectGroup => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::UseAfterKillTarget => &[ParamFlag::Content],
            Self::UseBeforeSeekHelp => &[ParamFlag::Content],
            Self::WarmUpDuration => &[ParamFlag::Persistent, ParamFlag::Content],
        }
    }
}
impl FromStr for EdnaAbility {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EDNA_ABILITY_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for EdnaAbility {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8883u16 => Ok(Self::AbilityInfo),
            6500u16 => Ok(Self::AbilityType),
            4773u16 => Ok(Self::AbilityUsedCounter),
            241u16 => Ok(Self::ActivationAngle),
            264u16 => Ok(Self::ActivationType),
            5213u16 => Ok(Self::AffectEnemies),
            255u16 => Ok(Self::AffectFriends),
            9204u16 => Ok(Self::AffectNeutral),
            254u16 => Ok(Self::AffectSelf),
            3035u16 => Ok(Self::AllowMoveWhileChanneling),
            3036u16 => Ok(Self::AllowMoveWhileCharging),
            10022u16 => Ok(Self::AllowMoveWhileEndState),
            8917u16 => Ok(Self::AllowPlayerMoveWhileChanneling),
            10034u16 => Ok(Self::AllowRangedAttackAdjustMove),
            6730u16 => Ok(Self::AllowUseAboveHp),
            9032u16 => Ok(Self::AllowUseAfterKillNumberOfNpCs),
            8890u16 => Ok(Self::AllowUseAfterKillNumberOfPlayers),
            6729u16 => Ok(Self::AllowUseBelowHp),
            8911u16 => Ok(Self::AllowUseWhenAvailable),
            7888u16 => Ok(Self::AlwaysExecute),
            249u16 => Ok(Self::Attributes),
            10917u16 => Ok(Self::AutoFaceTargetOrLocation),
            11471u16 => Ok(Self::BlockFaceTargetWhenPreparing),
            11210u16 => Ok(Self::CanBeBend),
            11208u16 => Ok(Self::CanBeBlocked),
            11206u16 => Ok(Self::CanBeInterrupted),
            11209u16 => Ok(Self::CanBeReflected),
            8022u16 => Ok(Self::CanPredict),
            9625u16 => Ok(Self::CastTime),
            10102u16 => Ok(Self::ChannelIndefinitely),
            9624u16 => Ok(Self::ChannelTime),
            9385u16 => Ok(Self::Charge),
            4216u16 => Ok(Self::CombatStyle),
            4030u16 => Ok(Self::ConsumeCooldowns),
            10626u16 => Ok(Self::ConsumeEnergyInChannelTick),
            10622u16 => Ok(Self::Description),
            4290u16 => Ok(Self::DisplayName),
            11291u16 => Ok(Self::DoNotCheckObstruction),
            5556u16 => Ok(Self::EffectorCastSettings),
            240u16 => Ok(Self::EffectorChannelingSettings),
            248u16 => Ok(Self::EffectorSettings),
            260u16 => Ok(Self::EffectType),
            4062u16 => Ok(Self::EmitCooldownOnCast),
            4061u16 => Ok(Self::EmitCooldownOnUse),
            6804u16 => Ok(Self::EnableInGame),
            8936u16 => Ok(Self::EndStateDuration),
            4215u16 => Ok(Self::EnergyConsumed),
            6776u16 => Ok(Self::ExecutionTime),
            11332u16 => Ok(Self::ExternalCooldownsConsumed),
            11331u16 => Ok(Self::ExternalCooldownsEmitted),
            11233u16 => Ok(Self::GeneralPreRequisites),
            4335u16 => Ok(Self::Icon),
            4031u16 => Ok(Self::InternalCooldown),
            258u16 => Ok(Self::IsActive),
            4221u16 => Ok(Self::IsAutoAttack),
            8037u16 => Ok(Self::IsPhysical),
            4774u16 => Ok(Self::IsQueued),
            10918u16 => Ok(Self::IssueAttackPause),
            10919u16 => Ok(Self::KeepTargetWithinConeAngle),
            8906u16 => Ok(Self::LuaScript),
            8031u16 => Ok(Self::MaxPendingRequestAge),
            9303u16 => Ok(Self::MutuallyExclusiveToBuff),
            11317u16 => Ok(Self::NeedBuffGroupsToExecute),
            9293u16 => Ok(Self::NeedBuffToExecute),
            12414u16 => Ok(Self::OnlyCheckMiddleObstruction),
            10835u16 => Ok(Self::PreventUseEffectWithoutTarget),
            9621u16 => Ok(Self::RangeMax),
            9622u16 => Ok(Self::RangeMin),
            11344u16 => Ok(Self::Rank),
            10396u16 => Ok(Self::RemoveActiveBuff),
            11324u16 => Ok(Self::RemoveActiveBuffGroupsAndQuit),
            6785u16 => Ok(Self::RequireRunningWhenActivated),
            11290u16 => Ok(Self::RezoutWeapon),
            11399u16 => Ok(Self::ScrambleCooldownOnCombat),
            11330u16 => Ok(Self::SkillGroup),
            265u16 => Ok(Self::SourceMustBeAlive),
            8884u16 => Ok(Self::TargetAbilityInfo),
            9626u16 => Ok(Self::TargetFactory),
            266u16 => Ok(Self::TargetMustBeAlive),
            267u16 => Ok(Self::TargetMustBeDead),
            8810u16 => Ok(Self::TargetMustBeOnGround),
            262u16 => Ok(Self::TargetType),
            9623u16 => Ok(Self::TickPeriod),
            10138u16 => Ok(Self::TriggerCooldown),
            252u16 => Ok(Self::UnequipWeapon),
            251u16 => Ok(Self::UsableInCombat),
            8889u16 => Ok(Self::UsableOutOfCombat),
            10140u16 => Ok(Self::UsableWithClassWeapon),
            11085u16 => Ok(Self::UsableWithMainEffectGroup),
            9295u16 => Ok(Self::UsableWithMeleeWeapon),
            9296u16 => Ok(Self::UsableWithoutWeapon),
            9294u16 => Ok(Self::UsableWithRangedWeapon),
            11086u16 => Ok(Self::UsableWithSubEffectGroup),
            7153u16 => Ok(Self::UseAfterKillTarget),
            7087u16 => Ok(Self::UseBeforeSeekHelp),
            5405u16 => Ok(Self::WarmUpDuration),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
