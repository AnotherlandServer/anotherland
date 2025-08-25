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
pub enum VehicleFlying {
    Action0,
    Action0Duration,
    Action0Option,
    AlwaysVisibleToPlayers,
    AutoReviveDelay,
    AutoReviveTime,
    AwareRange,
    BeaconRadius,
    CollisionExtent,
    ContentClass,
    CycleQuestBase,
    DefaultWeapon,
    DespawnDelay,
    Dialogs,
    DisplayName,
    EnableInGame,
    FreedomProperties,
    Freq,
    GenerateInterestList,
    HiddenFromClients,
    HiddenFromPlayers,
    HideAfterInteraction,
    Icon,
    InstanceTags,
    InstanceZoneKey,
    InteractionDuration,
    InteractionRadius,
    InteractionResetTimer,
    IsNonSpawnedAvatar,
    IsSelfRevivable,
    LastInteractionTime,
    LuaScript,
    Lvl,
    MaterialOverride,
    Nodelink,
    OriginalNodeName,
    OriginalZoneName,
    PartyGuid,
    PathfindSafeSpawn,
    Pos,
    Power,
    Priority,
    QuestFlags,
    ReadableName,
    RespawnDelay,
    RespawnRegionName,
    RespawnRegionNameOverride,
    Rot,
    SelfRadius,
    SpawnMethod,
    SpawnPosition,
    SpawnRotation,
    Tags,
    TeamId,
    Ue3ClassId,
    Ue3EdVisual,
    VisibleOnQuestAvailable,
    VisibleOnQuestComplete,
    VisibleOnQuestFinished,
    VisibleOnQuestInProgress,
    WorldZoneObjectIndex,
    Zone,
    ZoneGuid,
    Alive,
    AttackedBy,
    AttributeEnergyCurrent,
    AttributeEnergyMax,
    Defb,
    DriverId,
    HasAttributes,
    HasPhysicsController,
    HpCur,
    HpMax,
    LastAttackPosition,
    MoveSpeed,
    PhysicsProperties,
    RunSpeed,
    SeatArrangement,
    StatAttackPower,
    StatCritChance,
    StatCriticalDamageMod,
    StatHealth,
    StatHitChance,
    Target,
    VehicleState,
    WalkSpeed,
    Weapon,
    BeingHealed,
    DamageToEnergyRatio,
    EnergyRegenerateRate,
    FixedHeight,
    StatMovement,
}
pub(crate) static VEHICLE_FLYING_ATTRIBUTES: phf::Map<&'static str, VehicleFlying> = phf_map! {
    "action0" => VehicleFlying::Action0, "action0Duration" =>
    VehicleFlying::Action0Duration, "action0Option" => VehicleFlying::Action0Option,
    "alwaysVisibleToPlayers" => VehicleFlying::AlwaysVisibleToPlayers, "autoReviveDelay"
    => VehicleFlying::AutoReviveDelay, "autoReviveTime" => VehicleFlying::AutoReviveTime,
    "AwareRange" => VehicleFlying::AwareRange, "BeaconRadius" =>
    VehicleFlying::BeaconRadius, "collisionExtent" => VehicleFlying::CollisionExtent,
    "ContentClass" => VehicleFlying::ContentClass, "CycleQuestBase" =>
    VehicleFlying::CycleQuestBase, "defaultWeapon" => VehicleFlying::DefaultWeapon,
    "despawnDelay" => VehicleFlying::DespawnDelay, "Dialogs" => VehicleFlying::Dialogs,
    "DisplayName" => VehicleFlying::DisplayName, "EnableInGame" =>
    VehicleFlying::EnableInGame, "FreedomProperties" => VehicleFlying::FreedomProperties,
    "Freq" => VehicleFlying::Freq, "generateInterestList" =>
    VehicleFlying::GenerateInterestList, "hiddenFromClients" =>
    VehicleFlying::HiddenFromClients, "hiddenFromPlayers" =>
    VehicleFlying::HiddenFromPlayers, "HideAfterInteraction" =>
    VehicleFlying::HideAfterInteraction, "Icon" => VehicleFlying::Icon, "instanceTags" =>
    VehicleFlying::InstanceTags, "instanceZoneKey" => VehicleFlying::InstanceZoneKey,
    "InteractionDuration" => VehicleFlying::InteractionDuration, "InteractionRadius" =>
    VehicleFlying::InteractionRadius, "InteractionResetTimer" =>
    VehicleFlying::InteractionResetTimer, "isNonSpawnedAvatar" =>
    VehicleFlying::IsNonSpawnedAvatar, "isSelfRevivable" =>
    VehicleFlying::IsSelfRevivable, "LastInteractionTime" =>
    VehicleFlying::LastInteractionTime, "LuaScript" => VehicleFlying::LuaScript, "lvl" =>
    VehicleFlying::Lvl, "MaterialOverride" => VehicleFlying::MaterialOverride, "nodelink"
    => VehicleFlying::Nodelink, "originalNodeName" => VehicleFlying::OriginalNodeName,
    "originalZoneName" => VehicleFlying::OriginalZoneName, "partyGUID" =>
    VehicleFlying::PartyGuid, "pathfindSafeSpawn" => VehicleFlying::PathfindSafeSpawn,
    "pos" => VehicleFlying::Pos, "Power" => VehicleFlying::Power, "priority" =>
    VehicleFlying::Priority, "QuestFlags" => VehicleFlying::QuestFlags, "ReadableName" =>
    VehicleFlying::ReadableName, "respawnDelay" => VehicleFlying::RespawnDelay,
    "RespawnRegionName" => VehicleFlying::RespawnRegionName, "RespawnRegionNameOverride"
    => VehicleFlying::RespawnRegionNameOverride, "rot" => VehicleFlying::Rot,
    "selfRadius" => VehicleFlying::SelfRadius, "spawnMethod" =>
    VehicleFlying::SpawnMethod, "spawnPosition" => VehicleFlying::SpawnPosition,
    "spawnRotation" => VehicleFlying::SpawnRotation, "tags" => VehicleFlying::Tags,
    "teamID" => VehicleFlying::TeamId, "UE3ClassID" => VehicleFlying::Ue3ClassId,
    "UE3EdVisual" => VehicleFlying::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    VehicleFlying::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    VehicleFlying::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    VehicleFlying::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    VehicleFlying::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    VehicleFlying::WorldZoneObjectIndex, "zone" => VehicleFlying::Zone, "ZoneGuid" =>
    VehicleFlying::ZoneGuid, "alive" => VehicleFlying::Alive, "attackedBy" =>
    VehicleFlying::AttackedBy, "attributeEnergyCurrent" =>
    VehicleFlying::AttributeEnergyCurrent, "attributeEnergyMax" =>
    VehicleFlying::AttributeEnergyMax, "defb" => VehicleFlying::Defb, "driverID" =>
    VehicleFlying::DriverId, "hasAttributes" => VehicleFlying::HasAttributes,
    "hasPhysicsController" => VehicleFlying::HasPhysicsController, "hpCur" =>
    VehicleFlying::HpCur, "hpMax" => VehicleFlying::HpMax, "lastAttackPosition" =>
    VehicleFlying::LastAttackPosition, "moveSpeed" => VehicleFlying::MoveSpeed,
    "physicsProperties" => VehicleFlying::PhysicsProperties, "runSpeed" =>
    VehicleFlying::RunSpeed, "seatArrangement" => VehicleFlying::SeatArrangement,
    "statAttackPower" => VehicleFlying::StatAttackPower, "statCritChance" =>
    VehicleFlying::StatCritChance, "statCriticalDamageMod" =>
    VehicleFlying::StatCriticalDamageMod, "statHealth" => VehicleFlying::StatHealth,
    "statHitChance" => VehicleFlying::StatHitChance, "target" => VehicleFlying::Target,
    "vehicleState" => VehicleFlying::VehicleState, "walkSpeed" =>
    VehicleFlying::WalkSpeed, "weapon" => VehicleFlying::Weapon, "beingHealed" =>
    VehicleFlying::BeingHealed, "damageToEnergyRatio" =>
    VehicleFlying::DamageToEnergyRatio, "energyRegenerateRate" =>
    VehicleFlying::EnergyRegenerateRate, "fixedHeight" => VehicleFlying::FixedHeight,
    "statMovement" => VehicleFlying::StatMovement,
};
pub(crate) static VEHICLE_FLYING_ATTRIBUTES_ID: phf::Map<u16, VehicleFlying> = phf_map! {
    3107u16 => VehicleFlying::Action0, 3106u16 => VehicleFlying::Action0Duration, 3111u16
    => VehicleFlying::Action0Option, 3525u16 => VehicleFlying::AlwaysVisibleToPlayers,
    10576u16 => VehicleFlying::AutoReviveDelay, 10516u16 =>
    VehicleFlying::AutoReviveTime, 8294u16 => VehicleFlying::AwareRange, 10987u16 =>
    VehicleFlying::BeaconRadius, 3105u16 => VehicleFlying::CollisionExtent, 3109u16 =>
    VehicleFlying::ContentClass, 11069u16 => VehicleFlying::CycleQuestBase, 7263u16 =>
    VehicleFlying::DefaultWeapon, 9685u16 => VehicleFlying::DespawnDelay, 8881u16 =>
    VehicleFlying::Dialogs, 6649u16 => VehicleFlying::DisplayName, 6875u16 =>
    VehicleFlying::EnableInGame, 11193u16 => VehicleFlying::FreedomProperties, 3092u16 =>
    VehicleFlying::Freq, 3104u16 => VehicleFlying::GenerateInterestList, 3103u16 =>
    VehicleFlying::HiddenFromClients, 3113u16 => VehicleFlying::HiddenFromPlayers,
    9201u16 => VehicleFlying::HideAfterInteraction, 4390u16 => VehicleFlying::Icon,
    3110u16 => VehicleFlying::InstanceTags, 5609u16 => VehicleFlying::InstanceZoneKey,
    11139u16 => VehicleFlying::InteractionDuration, 7524u16 =>
    VehicleFlying::InteractionRadius, 9203u16 => VehicleFlying::InteractionResetTimer,
    3119u16 => VehicleFlying::IsNonSpawnedAvatar, 7208u16 =>
    VehicleFlying::IsSelfRevivable, 9202u16 => VehicleFlying::LastInteractionTime,
    7827u16 => VehicleFlying::LuaScript, 6232u16 => VehicleFlying::Lvl, 4771u16 =>
    VehicleFlying::MaterialOverride, 3112u16 => VehicleFlying::Nodelink, 3117u16 =>
    VehicleFlying::OriginalNodeName, 3116u16 => VehicleFlying::OriginalZoneName, 3102u16
    => VehicleFlying::PartyGuid, 3114u16 => VehicleFlying::PathfindSafeSpawn, 3101u16 =>
    VehicleFlying::Pos, 3093u16 => VehicleFlying::Power, 3100u16 =>
    VehicleFlying::Priority, 9984u16 => VehicleFlying::QuestFlags, 3710u16 =>
    VehicleFlying::ReadableName, 3120u16 => VehicleFlying::RespawnDelay, 10834u16 =>
    VehicleFlying::RespawnRegionName, 10893u16 =>
    VehicleFlying::RespawnRegionNameOverride, 3099u16 => VehicleFlying::Rot, 3098u16 =>
    VehicleFlying::SelfRadius, 6150u16 => VehicleFlying::SpawnMethod, 7882u16 =>
    VehicleFlying::SpawnPosition, 8237u16 => VehicleFlying::SpawnRotation, 3097u16 =>
    VehicleFlying::Tags, 3096u16 => VehicleFlying::TeamId, 3108u16 =>
    VehicleFlying::Ue3ClassId, 9860u16 => VehicleFlying::Ue3EdVisual, 8790u16 =>
    VehicleFlying::VisibleOnQuestAvailable, 8787u16 =>
    VehicleFlying::VisibleOnQuestComplete, 8788u16 =>
    VehicleFlying::VisibleOnQuestFinished, 8789u16 =>
    VehicleFlying::VisibleOnQuestInProgress, 3118u16 =>
    VehicleFlying::WorldZoneObjectIndex, 3094u16 => VehicleFlying::Zone, 3115u16 =>
    VehicleFlying::ZoneGuid, 3129u16 => VehicleFlying::Alive, 3490u16 =>
    VehicleFlying::AttackedBy, 5502u16 => VehicleFlying::AttributeEnergyCurrent, 5503u16
    => VehicleFlying::AttributeEnergyMax, 3141u16 => VehicleFlying::Defb, 3091u16 =>
    VehicleFlying::DriverId, 6439u16 => VehicleFlying::HasAttributes, 4193u16 =>
    VehicleFlying::HasPhysicsController, 3088u16 => VehicleFlying::HpCur, 3087u16 =>
    VehicleFlying::HpMax, 3489u16 => VehicleFlying::LastAttackPosition, 4050u16 =>
    VehicleFlying::MoveSpeed, 4210u16 => VehicleFlying::PhysicsProperties, 4047u16 =>
    VehicleFlying::RunSpeed, 3086u16 => VehicleFlying::SeatArrangement, 9618u16 =>
    VehicleFlying::StatAttackPower, 9616u16 => VehicleFlying::StatCritChance, 9615u16 =>
    VehicleFlying::StatCriticalDamageMod, 9619u16 => VehicleFlying::StatHealth, 9617u16
    => VehicleFlying::StatHitChance, 3085u16 => VehicleFlying::Target, 3464u16 =>
    VehicleFlying::VehicleState, 4048u16 => VehicleFlying::WalkSpeed, 3127u16 =>
    VehicleFlying::Weapon, 5215u16 => VehicleFlying::BeingHealed, 5498u16 =>
    VehicleFlying::DamageToEnergyRatio, 5499u16 => VehicleFlying::EnergyRegenerateRate,
    3132u16 => VehicleFlying::FixedHeight, 6989u16 => VehicleFlying::StatMovement,
};
impl Attribute for VehicleFlying {
    fn class() -> Class {
        Class::VehicleFlying
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Action0 => &Self::Action0,
            Self::Action0Duration => &Self::Action0Duration,
            Self::Action0Option => &Self::Action0Option,
            Self::AlwaysVisibleToPlayers => &Self::AlwaysVisibleToPlayers,
            Self::AutoReviveDelay => &Self::AutoReviveDelay,
            Self::AutoReviveTime => &Self::AutoReviveTime,
            Self::AwareRange => &Self::AwareRange,
            Self::BeaconRadius => &Self::BeaconRadius,
            Self::CollisionExtent => &Self::CollisionExtent,
            Self::ContentClass => &Self::ContentClass,
            Self::CycleQuestBase => &Self::CycleQuestBase,
            Self::DefaultWeapon => &Self::DefaultWeapon,
            Self::DespawnDelay => &Self::DespawnDelay,
            Self::Dialogs => &Self::Dialogs,
            Self::DisplayName => &Self::DisplayName,
            Self::EnableInGame => &Self::EnableInGame,
            Self::FreedomProperties => &Self::FreedomProperties,
            Self::Freq => &Self::Freq,
            Self::GenerateInterestList => &Self::GenerateInterestList,
            Self::HiddenFromClients => &Self::HiddenFromClients,
            Self::HiddenFromPlayers => &Self::HiddenFromPlayers,
            Self::HideAfterInteraction => &Self::HideAfterInteraction,
            Self::Icon => &Self::Icon,
            Self::InstanceTags => &Self::InstanceTags,
            Self::InstanceZoneKey => &Self::InstanceZoneKey,
            Self::InteractionDuration => &Self::InteractionDuration,
            Self::InteractionRadius => &Self::InteractionRadius,
            Self::InteractionResetTimer => &Self::InteractionResetTimer,
            Self::IsNonSpawnedAvatar => &Self::IsNonSpawnedAvatar,
            Self::IsSelfRevivable => &Self::IsSelfRevivable,
            Self::LastInteractionTime => &Self::LastInteractionTime,
            Self::LuaScript => &Self::LuaScript,
            Self::Lvl => &Self::Lvl,
            Self::MaterialOverride => &Self::MaterialOverride,
            Self::Nodelink => &Self::Nodelink,
            Self::OriginalNodeName => &Self::OriginalNodeName,
            Self::OriginalZoneName => &Self::OriginalZoneName,
            Self::PartyGuid => &Self::PartyGuid,
            Self::PathfindSafeSpawn => &Self::PathfindSafeSpawn,
            Self::Pos => &Self::Pos,
            Self::Power => &Self::Power,
            Self::Priority => &Self::Priority,
            Self::QuestFlags => &Self::QuestFlags,
            Self::ReadableName => &Self::ReadableName,
            Self::RespawnDelay => &Self::RespawnDelay,
            Self::RespawnRegionName => &Self::RespawnRegionName,
            Self::RespawnRegionNameOverride => &Self::RespawnRegionNameOverride,
            Self::Rot => &Self::Rot,
            Self::SelfRadius => &Self::SelfRadius,
            Self::SpawnMethod => &Self::SpawnMethod,
            Self::SpawnPosition => &Self::SpawnPosition,
            Self::SpawnRotation => &Self::SpawnRotation,
            Self::Tags => &Self::Tags,
            Self::TeamId => &Self::TeamId,
            Self::Ue3ClassId => &Self::Ue3ClassId,
            Self::Ue3EdVisual => &Self::Ue3EdVisual,
            Self::VisibleOnQuestAvailable => &Self::VisibleOnQuestAvailable,
            Self::VisibleOnQuestComplete => &Self::VisibleOnQuestComplete,
            Self::VisibleOnQuestFinished => &Self::VisibleOnQuestFinished,
            Self::VisibleOnQuestInProgress => &Self::VisibleOnQuestInProgress,
            Self::WorldZoneObjectIndex => &Self::WorldZoneObjectIndex,
            Self::Zone => &Self::Zone,
            Self::ZoneGuid => &Self::ZoneGuid,
            Self::Alive => &Self::Alive,
            Self::AttackedBy => &Self::AttackedBy,
            Self::AttributeEnergyCurrent => &Self::AttributeEnergyCurrent,
            Self::AttributeEnergyMax => &Self::AttributeEnergyMax,
            Self::Defb => &Self::Defb,
            Self::DriverId => &Self::DriverId,
            Self::HasAttributes => &Self::HasAttributes,
            Self::HasPhysicsController => &Self::HasPhysicsController,
            Self::HpCur => &Self::HpCur,
            Self::HpMax => &Self::HpMax,
            Self::LastAttackPosition => &Self::LastAttackPosition,
            Self::MoveSpeed => &Self::MoveSpeed,
            Self::PhysicsProperties => &Self::PhysicsProperties,
            Self::RunSpeed => &Self::RunSpeed,
            Self::SeatArrangement => &Self::SeatArrangement,
            Self::StatAttackPower => &Self::StatAttackPower,
            Self::StatCritChance => &Self::StatCritChance,
            Self::StatCriticalDamageMod => &Self::StatCriticalDamageMod,
            Self::StatHealth => &Self::StatHealth,
            Self::StatHitChance => &Self::StatHitChance,
            Self::Target => &Self::Target,
            Self::VehicleState => &Self::VehicleState,
            Self::WalkSpeed => &Self::WalkSpeed,
            Self::Weapon => &Self::Weapon,
            Self::BeingHealed => &Self::BeingHealed,
            Self::DamageToEnergyRatio => &Self::DamageToEnergyRatio,
            Self::EnergyRegenerateRate => &Self::EnergyRegenerateRate,
            Self::FixedHeight => &Self::FixedHeight,
            Self::StatMovement => &Self::StatMovement,
        }
    }
}
impl AttributeInfo for VehicleFlying {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 3107u16,
            Self::Action0Duration => 3106u16,
            Self::Action0Option => 3111u16,
            Self::AlwaysVisibleToPlayers => 3525u16,
            Self::AutoReviveDelay => 10576u16,
            Self::AutoReviveTime => 10516u16,
            Self::AwareRange => 8294u16,
            Self::BeaconRadius => 10987u16,
            Self::CollisionExtent => 3105u16,
            Self::ContentClass => 3109u16,
            Self::CycleQuestBase => 11069u16,
            Self::DefaultWeapon => 7263u16,
            Self::DespawnDelay => 9685u16,
            Self::Dialogs => 8881u16,
            Self::DisplayName => 6649u16,
            Self::EnableInGame => 6875u16,
            Self::FreedomProperties => 11193u16,
            Self::Freq => 3092u16,
            Self::GenerateInterestList => 3104u16,
            Self::HiddenFromClients => 3103u16,
            Self::HiddenFromPlayers => 3113u16,
            Self::HideAfterInteraction => 9201u16,
            Self::Icon => 4390u16,
            Self::InstanceTags => 3110u16,
            Self::InstanceZoneKey => 5609u16,
            Self::InteractionDuration => 11139u16,
            Self::InteractionRadius => 7524u16,
            Self::InteractionResetTimer => 9203u16,
            Self::IsNonSpawnedAvatar => 3119u16,
            Self::IsSelfRevivable => 7208u16,
            Self::LastInteractionTime => 9202u16,
            Self::LuaScript => 7827u16,
            Self::Lvl => 6232u16,
            Self::MaterialOverride => 4771u16,
            Self::Nodelink => 3112u16,
            Self::OriginalNodeName => 3117u16,
            Self::OriginalZoneName => 3116u16,
            Self::PartyGuid => 3102u16,
            Self::PathfindSafeSpawn => 3114u16,
            Self::Pos => 3101u16,
            Self::Power => 3093u16,
            Self::Priority => 3100u16,
            Self::QuestFlags => 9984u16,
            Self::ReadableName => 3710u16,
            Self::RespawnDelay => 3120u16,
            Self::RespawnRegionName => 10834u16,
            Self::RespawnRegionNameOverride => 10893u16,
            Self::Rot => 3099u16,
            Self::SelfRadius => 3098u16,
            Self::SpawnMethod => 6150u16,
            Self::SpawnPosition => 7882u16,
            Self::SpawnRotation => 8237u16,
            Self::Tags => 3097u16,
            Self::TeamId => 3096u16,
            Self::Ue3ClassId => 3108u16,
            Self::Ue3EdVisual => 9860u16,
            Self::VisibleOnQuestAvailable => 8790u16,
            Self::VisibleOnQuestComplete => 8787u16,
            Self::VisibleOnQuestFinished => 8788u16,
            Self::VisibleOnQuestInProgress => 8789u16,
            Self::WorldZoneObjectIndex => 3118u16,
            Self::Zone => 3094u16,
            Self::ZoneGuid => 3115u16,
            Self::Alive => 3129u16,
            Self::AttackedBy => 3490u16,
            Self::AttributeEnergyCurrent => 5502u16,
            Self::AttributeEnergyMax => 5503u16,
            Self::Defb => 3141u16,
            Self::DriverId => 3091u16,
            Self::HasAttributes => 6439u16,
            Self::HasPhysicsController => 4193u16,
            Self::HpCur => 3088u16,
            Self::HpMax => 3087u16,
            Self::LastAttackPosition => 3489u16,
            Self::MoveSpeed => 4050u16,
            Self::PhysicsProperties => 4210u16,
            Self::RunSpeed => 4047u16,
            Self::SeatArrangement => 3086u16,
            Self::StatAttackPower => 9618u16,
            Self::StatCritChance => 9616u16,
            Self::StatCriticalDamageMod => 9615u16,
            Self::StatHealth => 9619u16,
            Self::StatHitChance => 9617u16,
            Self::Target => 3085u16,
            Self::VehicleState => 3464u16,
            Self::WalkSpeed => 4048u16,
            Self::Weapon => 3127u16,
            Self::BeingHealed => 5215u16,
            Self::DamageToEnergyRatio => 5498u16,
            Self::EnergyRegenerateRate => 5499u16,
            Self::FixedHeight => 3132u16,
            Self::StatMovement => 6989u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Action0 => "action0",
            Self::Action0Duration => "action0Duration",
            Self::Action0Option => "action0Option",
            Self::AlwaysVisibleToPlayers => "alwaysVisibleToPlayers",
            Self::AutoReviveDelay => "autoReviveDelay",
            Self::AutoReviveTime => "autoReviveTime",
            Self::AwareRange => "AwareRange",
            Self::BeaconRadius => "BeaconRadius",
            Self::CollisionExtent => "collisionExtent",
            Self::ContentClass => "ContentClass",
            Self::CycleQuestBase => "CycleQuestBase",
            Self::DefaultWeapon => "defaultWeapon",
            Self::DespawnDelay => "despawnDelay",
            Self::Dialogs => "Dialogs",
            Self::DisplayName => "DisplayName",
            Self::EnableInGame => "EnableInGame",
            Self::FreedomProperties => "FreedomProperties",
            Self::Freq => "Freq",
            Self::GenerateInterestList => "generateInterestList",
            Self::HiddenFromClients => "hiddenFromClients",
            Self::HiddenFromPlayers => "hiddenFromPlayers",
            Self::HideAfterInteraction => "HideAfterInteraction",
            Self::Icon => "Icon",
            Self::InstanceTags => "instanceTags",
            Self::InstanceZoneKey => "instanceZoneKey",
            Self::InteractionDuration => "InteractionDuration",
            Self::InteractionRadius => "InteractionRadius",
            Self::InteractionResetTimer => "InteractionResetTimer",
            Self::IsNonSpawnedAvatar => "isNonSpawnedAvatar",
            Self::IsSelfRevivable => "isSelfRevivable",
            Self::LastInteractionTime => "LastInteractionTime",
            Self::LuaScript => "LuaScript",
            Self::Lvl => "lvl",
            Self::MaterialOverride => "MaterialOverride",
            Self::Nodelink => "nodelink",
            Self::OriginalNodeName => "originalNodeName",
            Self::OriginalZoneName => "originalZoneName",
            Self::PartyGuid => "partyGUID",
            Self::PathfindSafeSpawn => "pathfindSafeSpawn",
            Self::Pos => "pos",
            Self::Power => "Power",
            Self::Priority => "priority",
            Self::QuestFlags => "QuestFlags",
            Self::ReadableName => "ReadableName",
            Self::RespawnDelay => "respawnDelay",
            Self::RespawnRegionName => "RespawnRegionName",
            Self::RespawnRegionNameOverride => "RespawnRegionNameOverride",
            Self::Rot => "rot",
            Self::SelfRadius => "selfRadius",
            Self::SpawnMethod => "spawnMethod",
            Self::SpawnPosition => "spawnPosition",
            Self::SpawnRotation => "spawnRotation",
            Self::Tags => "tags",
            Self::TeamId => "teamID",
            Self::Ue3ClassId => "UE3ClassID",
            Self::Ue3EdVisual => "UE3EdVisual",
            Self::VisibleOnQuestAvailable => "VisibleOnQuestAvailable",
            Self::VisibleOnQuestComplete => "VisibleOnQuestComplete",
            Self::VisibleOnQuestFinished => "VisibleOnQuestFinished",
            Self::VisibleOnQuestInProgress => "VisibleOnQuestInProgress",
            Self::WorldZoneObjectIndex => "WorldZoneObjectIndex",
            Self::Zone => "zone",
            Self::ZoneGuid => "ZoneGuid",
            Self::Alive => "alive",
            Self::AttackedBy => "attackedBy",
            Self::AttributeEnergyCurrent => "attributeEnergyCurrent",
            Self::AttributeEnergyMax => "attributeEnergyMax",
            Self::Defb => "defb",
            Self::DriverId => "driverID",
            Self::HasAttributes => "hasAttributes",
            Self::HasPhysicsController => "hasPhysicsController",
            Self::HpCur => "hpCur",
            Self::HpMax => "hpMax",
            Self::LastAttackPosition => "lastAttackPosition",
            Self::MoveSpeed => "moveSpeed",
            Self::PhysicsProperties => "physicsProperties",
            Self::RunSpeed => "runSpeed",
            Self::SeatArrangement => "seatArrangement",
            Self::StatAttackPower => "statAttackPower",
            Self::StatCritChance => "statCritChance",
            Self::StatCriticalDamageMod => "statCriticalDamageMod",
            Self::StatHealth => "statHealth",
            Self::StatHitChance => "statHitChance",
            Self::Target => "target",
            Self::VehicleState => "vehicleState",
            Self::WalkSpeed => "walkSpeed",
            Self::Weapon => "weapon",
            Self::BeingHealed => "beingHealed",
            Self::DamageToEnergyRatio => "damageToEnergyRatio",
            Self::EnergyRegenerateRate => "energyRegenerateRate",
            Self::FixedHeight => "fixedHeight",
            Self::StatMovement => "statMovement",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::TeamId => ParamType::Int,
            Self::BeingHealed => ParamType::Bool,
            Self::DamageToEnergyRatio => ParamType::Float,
            Self::EnergyRegenerateRate => ParamType::Float,
            Self::FixedHeight => ParamType::Float,
            Self::StatMovement => ParamType::Float,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::Action0Option => ParamType::Int,
            Self::AlwaysVisibleToPlayers => ParamType::Bool,
            Self::AutoReviveDelay => ParamType::Float,
            Self::AutoReviveTime => ParamType::Int64,
            Self::AwareRange => ParamType::Float,
            Self::BeaconRadius => ParamType::Int,
            Self::CollisionExtent => ParamType::Vector3,
            Self::ContentClass => ParamType::String,
            Self::CycleQuestBase => ParamType::Int,
            Self::DefaultWeapon => ParamType::ContentRefList,
            Self::DespawnDelay => ParamType::Float,
            Self::Dialogs => ParamType::VectorInt,
            Self::DisplayName => ParamType::LocalizedString,
            Self::EnableInGame => ParamType::Bool,
            Self::FreedomProperties => ParamType::VectorInt,
            Self::Freq => ParamType::Int,
            Self::GenerateInterestList => ParamType::Bool,
            Self::HiddenFromClients => ParamType::Bool,
            Self::HiddenFromPlayers => ParamType::Bool,
            Self::HideAfterInteraction => ParamType::Bool,
            Self::Icon => ParamType::String,
            Self::InstanceTags => ParamType::String,
            Self::InstanceZoneKey => ParamType::String,
            Self::InteractionDuration => ParamType::Float,
            Self::InteractionRadius => ParamType::Float,
            Self::InteractionResetTimer => ParamType::Int,
            Self::IsNonSpawnedAvatar => ParamType::Bool,
            Self::IsSelfRevivable => ParamType::Bool,
            Self::LastInteractionTime => ParamType::Int64,
            Self::LuaScript => ParamType::String,
            Self::Lvl => ParamType::Int,
            Self::MaterialOverride => ParamType::Int,
            Self::Nodelink => ParamType::String,
            Self::OriginalNodeName => ParamType::String,
            Self::OriginalZoneName => ParamType::String,
            Self::PartyGuid => ParamType::Guid,
            Self::Pos => ParamType::Vector3,
            Self::Power => ParamType::Int,
            Self::Priority => ParamType::Float,
            Self::QuestFlags => ParamType::VectorInt,
            Self::ReadableName => ParamType::String,
            Self::RespawnDelay => ParamType::Float,
            Self::RespawnRegionName => ParamType::String,
            Self::RespawnRegionNameOverride => ParamType::String,
            Self::Rot => ParamType::Vector3,
            Self::SelfRadius => ParamType::Float,
            Self::SpawnMethod => ParamType::String,
            Self::SpawnPosition => ParamType::Vector3,
            Self::SpawnRotation => ParamType::Vector3,
            Self::Tags => ParamType::String,
            Self::Ue3ClassId => ParamType::String,
            Self::Ue3EdVisual => ParamType::String,
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
            Self::Alive => ParamType::Bool,
            Self::AttackedBy => ParamType::AvatarId,
            Self::AttributeEnergyCurrent => ParamType::Float,
            Self::AttributeEnergyMax => ParamType::Float,
            Self::Defb => ParamType::String,
            Self::DriverId => ParamType::AvatarId,
            Self::HasAttributes => ParamType::Bool,
            Self::HasPhysicsController => ParamType::Bool,
            Self::HpCur => ParamType::Int,
            Self::HpMax => ParamType::Int,
            Self::LastAttackPosition => ParamType::Vector3,
            Self::MoveSpeed => ParamType::Float,
            Self::PhysicsProperties => ParamType::JsonValue,
            Self::RunSpeed => ParamType::Float,
            Self::SeatArrangement => ParamType::JsonValue,
            Self::StatAttackPower => ParamType::Float,
            Self::StatCritChance => ParamType::Float,
            Self::StatCriticalDamageMod => ParamType::Float,
            Self::StatHealth => ParamType::Float,
            Self::StatHitChance => ParamType::Float,
            Self::Target => ParamType::AvatarId,
            Self::VehicleState => ParamType::Int,
            Self::WalkSpeed => ParamType::Float,
            Self::Weapon => ParamType::GuidPair,
        }
    }
    fn default(&self) -> &'static Value {
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
        static TEAM_ID: Value = Value::Int(0i32);
        static BEING_HEALED: Value = Value::Bool(false);
        static DAMAGE_TO_ENERGY_RATIO: Value = Value::Float(0f32);
        static ENERGY_REGENERATE_RATE: Value = Value::Float(0f32);
        static FIXED_HEIGHT: Value = Value::Float(0f32);
        static STAT_MOVEMENT: Value = Value::Float(32f32);
        static ACTION_0: Lazy<Value> = Lazy::new(|| Value::StringFloatPair((
            String::default(),
            0.0,
        )));
        static ACTION_0_DURATION: Value = Value::Float(0f32);
        static ACTION_0_OPTION: Value = Value::Int(0i32);
        static ALWAYS_VISIBLE_TO_PLAYERS: Value = Value::Bool(false);
        static AUTO_REVIVE_DELAY: Value = Value::Float(0f32);
        static AUTO_REVIVE_TIME: Value = Value::Int64(-1i64);
        static AWARE_RANGE: Value = Value::Float(2500f32);
        static BEACON_RADIUS: Value = Value::Int(0i32);
        static COLLISION_EXTENT: Value = Value::Vector3(Vec3::new(21f32, 21f32, 44f32));
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CYCLE_QUEST_BASE: Value = Value::Int(1i32);
        static DEFAULT_WEAPON: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static DESPAWN_DELAY: Value = Value::Float(30f32);
        static DIALOGS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static FREEDOM_PROPERTIES: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static FREQ: Value = Value::Int(0i32);
        static GENERATE_INTEREST_LIST: Value = Value::Bool(true);
        static HIDDEN_FROM_CLIENTS: Value = Value::Bool(false);
        static HIDDEN_FROM_PLAYERS: Value = Value::Bool(false);
        static HIDE_AFTER_INTERACTION: Value = Value::Bool(false);
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(
            "UI_Common.Textures.PlaceHolderIcon".to_string(),
        ));
        static INSTANCE_TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static INSTANCE_ZONE_KEY: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static INTERACTION_DURATION: Value = Value::Float(5f32);
        static INTERACTION_RADIUS: Value = Value::Float(150f32);
        static INTERACTION_RESET_TIMER: Value = Value::Int(1i32);
        static IS_NON_SPAWNED_AVATAR: Value = Value::Bool(true);
        static IS_SELF_REVIVABLE: Value = Value::Bool(false);
        static LAST_INTERACTION_TIME: Value = Value::Int64(0i64);
        static LUA_SCRIPT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static LVL: Value = Value::Int(1i32);
        static MATERIAL_OVERRIDE: Value = Value::Int(0i32);
        static NODELINK: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ORIGINAL_NODE_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ORIGINAL_ZONE_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PARTY_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static POS: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static POWER: Value = Value::Int(0i32);
        static PRIORITY: Value = Value::Float(1f32);
        static QUEST_FLAGS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static READABLE_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static RESPAWN_DELAY: Value = Value::Float(30f32);
        static RESPAWN_REGION_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static RESPAWN_REGION_NAME_OVERRIDE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ROT: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static SELF_RADIUS: Value = Value::Float(20f32);
        static SPAWN_METHOD: Lazy<Value> = Lazy::new(|| Value::String(
            "normal".to_string(),
        ));
        static SPAWN_POSITION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static SPAWN_ROTATION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static TAGS: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLVehicle".to_string(),
        ));
        static UE_3_ED_VISUAL: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static VISIBLE_ON_QUEST_AVAILABLE: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static VISIBLE_ON_QUEST_COMPLETE: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static VISIBLE_ON_QUEST_FINISHED: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static VISIBLE_ON_QUEST_IN_PROGRESS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static WORLD_ZONE_OBJECT_INDEX: Value = Value::Int(0i32);
        static ZONE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ALIVE: Value = Value::Bool(true);
        static ATTACKED_BY: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static ATTRIBUTE_ENERGY_CURRENT: Value = Value::Float(0f32);
        static ATTRIBUTE_ENERGY_MAX: Value = Value::Float(100f32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static DRIVER_ID: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static HAS_ATTRIBUTES: Value = Value::Bool(true);
        static HAS_PHYSICS_CONTROLLER: Value = Value::Bool(true);
        static HP_CUR: Value = Value::Int(100i32);
        static HP_MAX: Value = Value::Int(100i32);
        static LAST_ATTACK_POSITION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static MOVE_SPEED: Value = Value::Float(337f32);
        static PHYSICS_PROPERTIES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{\"OLVehicleSimBase\":{}}").unwrap(),
        ));
        static RUN_SPEED: Value = Value::Float(192f32);
        static SEAT_ARRANGEMENT: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"Seats\":[{\"LocationOffsetX\":0.0000000000000000,\"LocationOffsetY\":0.0000000000000000,\"LocationOffsetZ\":0.0000000000000000,\"RotationOffsetPitch\":0,\"RotationOffsetYaw\":0,\"RotationOffsetRoll\":0,\"Scale\":1}]}",
                )
                .unwrap(),
        ));
        static STAT_ATTACK_POWER: Value = Value::Float(0f32);
        static STAT_CRIT_CHANCE: Value = Value::Float(0f32);
        static STAT_CRITICAL_DAMAGE_MOD: Value = Value::Float(0f32);
        static STAT_HEALTH: Value = Value::Float(0f32);
        static STAT_HIT_CHANCE: Value = Value::Float(0f32);
        static TARGET: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static VEHICLE_STATE: Value = Value::Int(0i32);
        static WALK_SPEED: Value = Value::Float(80f32);
        static WEAPON: Value = Value::GuidPair((UUID_NIL, UUID_NIL));
        match self {
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::TeamId => &TEAM_ID,
            Self::BeingHealed => &BEING_HEALED,
            Self::DamageToEnergyRatio => &DAMAGE_TO_ENERGY_RATIO,
            Self::EnergyRegenerateRate => &ENERGY_REGENERATE_RATE,
            Self::FixedHeight => &FIXED_HEIGHT,
            Self::StatMovement => &STAT_MOVEMENT,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::Action0Option => &ACTION_0_OPTION,
            Self::AlwaysVisibleToPlayers => &ALWAYS_VISIBLE_TO_PLAYERS,
            Self::AutoReviveDelay => &AUTO_REVIVE_DELAY,
            Self::AutoReviveTime => &AUTO_REVIVE_TIME,
            Self::AwareRange => &AWARE_RANGE,
            Self::BeaconRadius => &BEACON_RADIUS,
            Self::CollisionExtent => &COLLISION_EXTENT,
            Self::ContentClass => &CONTENT_CLASS,
            Self::CycleQuestBase => &CYCLE_QUEST_BASE,
            Self::DefaultWeapon => &DEFAULT_WEAPON,
            Self::DespawnDelay => &DESPAWN_DELAY,
            Self::Dialogs => &DIALOGS,
            Self::DisplayName => &DISPLAY_NAME,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::FreedomProperties => &FREEDOM_PROPERTIES,
            Self::Freq => &FREQ,
            Self::GenerateInterestList => &GENERATE_INTEREST_LIST,
            Self::HiddenFromClients => &HIDDEN_FROM_CLIENTS,
            Self::HiddenFromPlayers => &HIDDEN_FROM_PLAYERS,
            Self::HideAfterInteraction => &HIDE_AFTER_INTERACTION,
            Self::Icon => &ICON,
            Self::InstanceTags => &INSTANCE_TAGS,
            Self::InstanceZoneKey => &INSTANCE_ZONE_KEY,
            Self::InteractionDuration => &INTERACTION_DURATION,
            Self::InteractionRadius => &INTERACTION_RADIUS,
            Self::InteractionResetTimer => &INTERACTION_RESET_TIMER,
            Self::IsNonSpawnedAvatar => &IS_NON_SPAWNED_AVATAR,
            Self::IsSelfRevivable => &IS_SELF_REVIVABLE,
            Self::LastInteractionTime => &LAST_INTERACTION_TIME,
            Self::LuaScript => &LUA_SCRIPT,
            Self::Lvl => &LVL,
            Self::MaterialOverride => &MATERIAL_OVERRIDE,
            Self::Nodelink => &NODELINK,
            Self::OriginalNodeName => &ORIGINAL_NODE_NAME,
            Self::OriginalZoneName => &ORIGINAL_ZONE_NAME,
            Self::PartyGuid => &PARTY_GUID,
            Self::Pos => &POS,
            Self::Power => &POWER,
            Self::Priority => &PRIORITY,
            Self::QuestFlags => &QUEST_FLAGS,
            Self::ReadableName => &READABLE_NAME,
            Self::RespawnDelay => &RESPAWN_DELAY,
            Self::RespawnRegionName => &RESPAWN_REGION_NAME,
            Self::RespawnRegionNameOverride => &RESPAWN_REGION_NAME_OVERRIDE,
            Self::Rot => &ROT,
            Self::SelfRadius => &SELF_RADIUS,
            Self::SpawnMethod => &SPAWN_METHOD,
            Self::SpawnPosition => &SPAWN_POSITION,
            Self::SpawnRotation => &SPAWN_ROTATION,
            Self::Tags => &TAGS,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
            Self::Alive => &ALIVE,
            Self::AttackedBy => &ATTACKED_BY,
            Self::AttributeEnergyCurrent => &ATTRIBUTE_ENERGY_CURRENT,
            Self::AttributeEnergyMax => &ATTRIBUTE_ENERGY_MAX,
            Self::Defb => &DEFB,
            Self::DriverId => &DRIVER_ID,
            Self::HasAttributes => &HAS_ATTRIBUTES,
            Self::HasPhysicsController => &HAS_PHYSICS_CONTROLLER,
            Self::HpCur => &HP_CUR,
            Self::HpMax => &HP_MAX,
            Self::LastAttackPosition => &LAST_ATTACK_POSITION,
            Self::MoveSpeed => &MOVE_SPEED,
            Self::PhysicsProperties => &PHYSICS_PROPERTIES,
            Self::RunSpeed => &RUN_SPEED,
            Self::SeatArrangement => &SEAT_ARRANGEMENT,
            Self::StatAttackPower => &STAT_ATTACK_POWER,
            Self::StatCritChance => &STAT_CRIT_CHANCE,
            Self::StatCriticalDamageMod => &STAT_CRITICAL_DAMAGE_MOD,
            Self::StatHealth => &STAT_HEALTH,
            Self::StatHitChance => &STAT_HIT_CHANCE,
            Self::Target => &TARGET,
            Self::VehicleState => &VEHICLE_STATE,
            Self::WalkSpeed => &WALK_SPEED,
            Self::Weapon => &WEAPON,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TeamId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BeingHealed => &[ParamFlag::NodeOwn, ParamFlag::PerInstanceSetting],
            Self::DamageToEnergyRatio => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EnergyRegenerateRate => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::FixedHeight => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::StatMovement => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Action0 => &[ParamFlag::NodeOwn],
            Self::Action0Duration => &[ParamFlag::NodeOwn],
            Self::Action0Option => &[ParamFlag::NodeOwn],
            Self::AlwaysVisibleToPlayers => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::AutoReviveDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoReviveTime => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AwareRange => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BeaconRadius => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CollisionExtent => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::ContentClass => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CycleQuestBase => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DefaultWeapon => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DespawnDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Dialogs => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayName => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EnableInGame => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::FreedomProperties => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GenerateInterestList => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::HiddenFromClients => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::HiddenFromPlayers => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HideAfterInteraction => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InstanceTags => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InstanceZoneKey => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::InteractionDuration => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InteractionRadius => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InteractionResetTimer => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsNonSpawnedAvatar => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::IsSelfRevivable => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::LastInteractionTime => &[ParamFlag::NodeOwn],
            Self::LuaScript => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Lvl => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MaterialOverride => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Nodelink => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::OriginalNodeName => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::OriginalZoneName => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::PartyGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Pos => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Power => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::Priority => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::QuestFlags => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ReadableName => &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting],
            Self::RespawnDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RespawnRegionName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::RespawnRegionNameOverride => {
                &[
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::Rot => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SelfRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::SpawnMethod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SpawnPosition => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnRotation => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Tags => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Ue3EdVisual => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
            Self::VisibleOnQuestAvailable => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::VisibleOnQuestComplete => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::VisibleOnQuestFinished => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::VisibleOnQuestInProgress => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WorldZoneObjectIndex => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Zone => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ZoneGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Alive => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttackedBy => &[ParamFlag::NodeOwn],
            Self::AttributeEnergyCurrent => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Defb => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::DriverId => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::HasAttributes => &[ParamFlag::Persistent],
            Self::HasPhysicsController => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HpCur => &[ParamFlag::NodeOwn],
            Self::HpMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::LastAttackPosition => &[ParamFlag::NodeOwn],
            Self::MoveSpeed => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PhysicsProperties => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RunSpeed => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SeatArrangement => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::StatAttackPower => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCritChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCriticalDamageMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHealth => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHitChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Target => &[ParamFlag::NodeOwn],
            Self::VehicleState => &[ParamFlag::NodeOwn],
            Self::WalkSpeed => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Weapon => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for VehicleFlying {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VEHICLE_FLYING_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for VehicleFlying {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            3107u16 => Ok(Self::Action0),
            3106u16 => Ok(Self::Action0Duration),
            3111u16 => Ok(Self::Action0Option),
            3525u16 => Ok(Self::AlwaysVisibleToPlayers),
            10576u16 => Ok(Self::AutoReviveDelay),
            10516u16 => Ok(Self::AutoReviveTime),
            8294u16 => Ok(Self::AwareRange),
            10987u16 => Ok(Self::BeaconRadius),
            3105u16 => Ok(Self::CollisionExtent),
            3109u16 => Ok(Self::ContentClass),
            11069u16 => Ok(Self::CycleQuestBase),
            7263u16 => Ok(Self::DefaultWeapon),
            9685u16 => Ok(Self::DespawnDelay),
            8881u16 => Ok(Self::Dialogs),
            6649u16 => Ok(Self::DisplayName),
            6875u16 => Ok(Self::EnableInGame),
            11193u16 => Ok(Self::FreedomProperties),
            3092u16 => Ok(Self::Freq),
            3104u16 => Ok(Self::GenerateInterestList),
            3103u16 => Ok(Self::HiddenFromClients),
            3113u16 => Ok(Self::HiddenFromPlayers),
            9201u16 => Ok(Self::HideAfterInteraction),
            4390u16 => Ok(Self::Icon),
            3110u16 => Ok(Self::InstanceTags),
            5609u16 => Ok(Self::InstanceZoneKey),
            11139u16 => Ok(Self::InteractionDuration),
            7524u16 => Ok(Self::InteractionRadius),
            9203u16 => Ok(Self::InteractionResetTimer),
            3119u16 => Ok(Self::IsNonSpawnedAvatar),
            7208u16 => Ok(Self::IsSelfRevivable),
            9202u16 => Ok(Self::LastInteractionTime),
            7827u16 => Ok(Self::LuaScript),
            6232u16 => Ok(Self::Lvl),
            4771u16 => Ok(Self::MaterialOverride),
            3112u16 => Ok(Self::Nodelink),
            3117u16 => Ok(Self::OriginalNodeName),
            3116u16 => Ok(Self::OriginalZoneName),
            3102u16 => Ok(Self::PartyGuid),
            3114u16 => Ok(Self::PathfindSafeSpawn),
            3101u16 => Ok(Self::Pos),
            3093u16 => Ok(Self::Power),
            3100u16 => Ok(Self::Priority),
            9984u16 => Ok(Self::QuestFlags),
            3710u16 => Ok(Self::ReadableName),
            3120u16 => Ok(Self::RespawnDelay),
            10834u16 => Ok(Self::RespawnRegionName),
            10893u16 => Ok(Self::RespawnRegionNameOverride),
            3099u16 => Ok(Self::Rot),
            3098u16 => Ok(Self::SelfRadius),
            6150u16 => Ok(Self::SpawnMethod),
            7882u16 => Ok(Self::SpawnPosition),
            8237u16 => Ok(Self::SpawnRotation),
            3097u16 => Ok(Self::Tags),
            3096u16 => Ok(Self::TeamId),
            3108u16 => Ok(Self::Ue3ClassId),
            9860u16 => Ok(Self::Ue3EdVisual),
            8790u16 => Ok(Self::VisibleOnQuestAvailable),
            8787u16 => Ok(Self::VisibleOnQuestComplete),
            8788u16 => Ok(Self::VisibleOnQuestFinished),
            8789u16 => Ok(Self::VisibleOnQuestInProgress),
            3118u16 => Ok(Self::WorldZoneObjectIndex),
            3094u16 => Ok(Self::Zone),
            3115u16 => Ok(Self::ZoneGuid),
            3129u16 => Ok(Self::Alive),
            3490u16 => Ok(Self::AttackedBy),
            5502u16 => Ok(Self::AttributeEnergyCurrent),
            5503u16 => Ok(Self::AttributeEnergyMax),
            3141u16 => Ok(Self::Defb),
            3091u16 => Ok(Self::DriverId),
            6439u16 => Ok(Self::HasAttributes),
            4193u16 => Ok(Self::HasPhysicsController),
            3088u16 => Ok(Self::HpCur),
            3087u16 => Ok(Self::HpMax),
            3489u16 => Ok(Self::LastAttackPosition),
            4050u16 => Ok(Self::MoveSpeed),
            4210u16 => Ok(Self::PhysicsProperties),
            4047u16 => Ok(Self::RunSpeed),
            3086u16 => Ok(Self::SeatArrangement),
            9618u16 => Ok(Self::StatAttackPower),
            9616u16 => Ok(Self::StatCritChance),
            9615u16 => Ok(Self::StatCriticalDamageMod),
            9619u16 => Ok(Self::StatHealth),
            9617u16 => Ok(Self::StatHitChance),
            3085u16 => Ok(Self::Target),
            3464u16 => Ok(Self::VehicleState),
            4048u16 => Ok(Self::WalkSpeed),
            3127u16 => Ok(Self::Weapon),
            5215u16 => Ok(Self::BeingHealed),
            5498u16 => Ok(Self::DamageToEnergyRatio),
            5499u16 => Ok(Self::EnergyRegenerateRate),
            3132u16 => Ok(Self::FixedHeight),
            6989u16 => Ok(Self::StatMovement),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
