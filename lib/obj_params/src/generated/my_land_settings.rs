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
pub enum MyLandSettings {
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
    AwareDist,
    Defb,
    InstanceGroup,
    IsUnAttackable,
    Abilities,
    Alive,
    AttackedBy,
    CarrierGuid,
    ClientLoadingPriority,
    DirectorTags,
    ForceSpawnOnClient,
    HpCur,
    HpMax,
    IsLocked,
    SpawnerAvatarGuid,
    SpawnerAvatarId,
    GridRevealDelayTime,
    GridSizeMultiplier,
    InitialDomeSize,
    InitialVisibleGrids,
    LootPointConfig,
    MinDistanceBetweenDefenses,
    NumGridIncreasePerLevel,
    NumGuardDefense,
    NumStaticDefense,
    RadiusChangeFactor,
}
pub(crate) static MY_LAND_SETTINGS_ATTRIBUTES: phf::Map<&'static str, MyLandSettings> = phf_map! {
    "action0" => MyLandSettings::Action0, "action0Duration" =>
    MyLandSettings::Action0Duration, "action0Option" => MyLandSettings::Action0Option,
    "alwaysVisibleToPlayers" => MyLandSettings::AlwaysVisibleToPlayers, "autoReviveDelay"
    => MyLandSettings::AutoReviveDelay, "autoReviveTime" =>
    MyLandSettings::AutoReviveTime, "AwareRange" => MyLandSettings::AwareRange,
    "BeaconRadius" => MyLandSettings::BeaconRadius, "collisionExtent" =>
    MyLandSettings::CollisionExtent, "ContentClass" => MyLandSettings::ContentClass,
    "CycleQuestBase" => MyLandSettings::CycleQuestBase, "defaultWeapon" =>
    MyLandSettings::DefaultWeapon, "despawnDelay" => MyLandSettings::DespawnDelay,
    "Dialogs" => MyLandSettings::Dialogs, "DisplayName" => MyLandSettings::DisplayName,
    "EnableInGame" => MyLandSettings::EnableInGame, "FreedomProperties" =>
    MyLandSettings::FreedomProperties, "Freq" => MyLandSettings::Freq,
    "generateInterestList" => MyLandSettings::GenerateInterestList, "hiddenFromClients"
    => MyLandSettings::HiddenFromClients, "hiddenFromPlayers" =>
    MyLandSettings::HiddenFromPlayers, "HideAfterInteraction" =>
    MyLandSettings::HideAfterInteraction, "Icon" => MyLandSettings::Icon, "instanceTags"
    => MyLandSettings::InstanceTags, "instanceZoneKey" =>
    MyLandSettings::InstanceZoneKey, "InteractionDuration" =>
    MyLandSettings::InteractionDuration, "InteractionRadius" =>
    MyLandSettings::InteractionRadius, "InteractionResetTimer" =>
    MyLandSettings::InteractionResetTimer, "isNonSpawnedAvatar" =>
    MyLandSettings::IsNonSpawnedAvatar, "isSelfRevivable" =>
    MyLandSettings::IsSelfRevivable, "LastInteractionTime" =>
    MyLandSettings::LastInteractionTime, "LuaScript" => MyLandSettings::LuaScript, "lvl"
    => MyLandSettings::Lvl, "MaterialOverride" => MyLandSettings::MaterialOverride,
    "nodelink" => MyLandSettings::Nodelink, "originalNodeName" =>
    MyLandSettings::OriginalNodeName, "originalZoneName" =>
    MyLandSettings::OriginalZoneName, "partyGUID" => MyLandSettings::PartyGuid,
    "pathfindSafeSpawn" => MyLandSettings::PathfindSafeSpawn, "pos" =>
    MyLandSettings::Pos, "Power" => MyLandSettings::Power, "priority" =>
    MyLandSettings::Priority, "QuestFlags" => MyLandSettings::QuestFlags, "ReadableName"
    => MyLandSettings::ReadableName, "respawnDelay" => MyLandSettings::RespawnDelay,
    "RespawnRegionName" => MyLandSettings::RespawnRegionName, "RespawnRegionNameOverride"
    => MyLandSettings::RespawnRegionNameOverride, "rot" => MyLandSettings::Rot,
    "selfRadius" => MyLandSettings::SelfRadius, "spawnMethod" =>
    MyLandSettings::SpawnMethod, "spawnPosition" => MyLandSettings::SpawnPosition,
    "spawnRotation" => MyLandSettings::SpawnRotation, "tags" => MyLandSettings::Tags,
    "teamID" => MyLandSettings::TeamId, "UE3ClassID" => MyLandSettings::Ue3ClassId,
    "UE3EdVisual" => MyLandSettings::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    MyLandSettings::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    MyLandSettings::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    MyLandSettings::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    MyLandSettings::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    MyLandSettings::WorldZoneObjectIndex, "zone" => MyLandSettings::Zone, "ZoneGuid" =>
    MyLandSettings::ZoneGuid, "awareDist" => MyLandSettings::AwareDist, "defb" =>
    MyLandSettings::Defb, "instanceGroup" => MyLandSettings::InstanceGroup,
    "isUnAttackable" => MyLandSettings::IsUnAttackable, "abilities" =>
    MyLandSettings::Abilities, "alive" => MyLandSettings::Alive, "attackedBy" =>
    MyLandSettings::AttackedBy, "carrierGuid" => MyLandSettings::CarrierGuid,
    "clientLoadingPriority" => MyLandSettings::ClientLoadingPriority, "directorTags" =>
    MyLandSettings::DirectorTags, "forceSpawnOnClient" =>
    MyLandSettings::ForceSpawnOnClient, "hpCur" => MyLandSettings::HpCur, "hpMax" =>
    MyLandSettings::HpMax, "isLocked" => MyLandSettings::IsLocked, "spawnerAvatarGuid" =>
    MyLandSettings::SpawnerAvatarGuid, "spawnerAvatarID" =>
    MyLandSettings::SpawnerAvatarId, "GridRevealDelayTime" =>
    MyLandSettings::GridRevealDelayTime, "GridSizeMultiplier" =>
    MyLandSettings::GridSizeMultiplier, "InitialDomeSize" =>
    MyLandSettings::InitialDomeSize, "InitialVisibleGrids" =>
    MyLandSettings::InitialVisibleGrids, "LootPointConfig" =>
    MyLandSettings::LootPointConfig, "MinDistanceBetweenDefenses" =>
    MyLandSettings::MinDistanceBetweenDefenses, "NumGridIncreasePerLevel" =>
    MyLandSettings::NumGridIncreasePerLevel, "NumGuardDefense" =>
    MyLandSettings::NumGuardDefense, "NumStaticDefense" =>
    MyLandSettings::NumStaticDefense, "RadiusChangeFactor" =>
    MyLandSettings::RadiusChangeFactor,
};
pub(crate) static MY_LAND_SETTINGS_ATTRIBUTES_ID: phf::Map<u16, MyLandSettings> = phf_map! {
    5656u16 => MyLandSettings::Action0, 5655u16 => MyLandSettings::Action0Duration,
    5666u16 => MyLandSettings::Action0Option, 5639u16 =>
    MyLandSettings::AlwaysVisibleToPlayers, 10563u16 => MyLandSettings::AutoReviveDelay,
    10503u16 => MyLandSettings::AutoReviveTime, 8282u16 => MyLandSettings::AwareRange,
    10974u16 => MyLandSettings::BeaconRadius, 5654u16 => MyLandSettings::CollisionExtent,
    5658u16 => MyLandSettings::ContentClass, 11060u16 => MyLandSettings::CycleQuestBase,
    7248u16 => MyLandSettings::DefaultWeapon, 9673u16 => MyLandSettings::DespawnDelay,
    8869u16 => MyLandSettings::Dialogs, 6634u16 => MyLandSettings::DisplayName, 6860u16
    => MyLandSettings::EnableInGame, 11184u16 => MyLandSettings::FreedomProperties,
    5641u16 => MyLandSettings::Freq, 5653u16 => MyLandSettings::GenerateInterestList,
    5652u16 => MyLandSettings::HiddenFromClients, 5668u16 =>
    MyLandSettings::HiddenFromPlayers, 9165u16 => MyLandSettings::HideAfterInteraction,
    5636u16 => MyLandSettings::Icon, 5665u16 => MyLandSettings::InstanceTags, 5625u16 =>
    MyLandSettings::InstanceZoneKey, 11130u16 => MyLandSettings::InteractionDuration,
    7509u16 => MyLandSettings::InteractionRadius, 9167u16 =>
    MyLandSettings::InteractionResetTimer, 5678u16 => MyLandSettings::IsNonSpawnedAvatar,
    7193u16 => MyLandSettings::IsSelfRevivable, 9166u16 =>
    MyLandSettings::LastInteractionTime, 7817u16 => MyLandSettings::LuaScript, 6217u16 =>
    MyLandSettings::Lvl, 5629u16 => MyLandSettings::MaterialOverride, 5667u16 =>
    MyLandSettings::Nodelink, 5676u16 => MyLandSettings::OriginalNodeName, 5675u16 =>
    MyLandSettings::OriginalZoneName, 5651u16 => MyLandSettings::PartyGuid, 5669u16 =>
    MyLandSettings::PathfindSafeSpawn, 5650u16 => MyLandSettings::Pos, 5642u16 =>
    MyLandSettings::Power, 5649u16 => MyLandSettings::Priority, 9971u16 =>
    MyLandSettings::QuestFlags, 5638u16 => MyLandSettings::ReadableName, 5679u16 =>
    MyLandSettings::RespawnDelay, 10821u16 => MyLandSettings::RespawnRegionName, 10880u16
    => MyLandSettings::RespawnRegionNameOverride, 5648u16 => MyLandSettings::Rot, 5647u16
    => MyLandSettings::SelfRadius, 6135u16 => MyLandSettings::SpawnMethod, 7872u16 =>
    MyLandSettings::SpawnPosition, 8225u16 => MyLandSettings::SpawnRotation, 5646u16 =>
    MyLandSettings::Tags, 5645u16 => MyLandSettings::TeamId, 5657u16 =>
    MyLandSettings::Ue3ClassId, 9847u16 => MyLandSettings::Ue3EdVisual, 8742u16 =>
    MyLandSettings::VisibleOnQuestAvailable, 8739u16 =>
    MyLandSettings::VisibleOnQuestComplete, 8740u16 =>
    MyLandSettings::VisibleOnQuestFinished, 8741u16 =>
    MyLandSettings::VisibleOnQuestInProgress, 5677u16 =>
    MyLandSettings::WorldZoneObjectIndex, 5643u16 => MyLandSettings::Zone, 5670u16 =>
    MyLandSettings::ZoneGuid, 5673u16 => MyLandSettings::AwareDist, 5660u16 =>
    MyLandSettings::Defb, 11381u16 => MyLandSettings::InstanceGroup, 12439u16 =>
    MyLandSettings::IsUnAttackable, 9342u16 => MyLandSettings::Abilities, 5664u16 =>
    MyLandSettings::Alive, 5663u16 => MyLandSettings::AttackedBy, 5671u16 =>
    MyLandSettings::CarrierGuid, 11283u16 => MyLandSettings::ClientLoadingPriority,
    8099u16 => MyLandSettings::DirectorTags, 5672u16 =>
    MyLandSettings::ForceSpawnOnClient, 5662u16 => MyLandSettings::HpCur, 5661u16 =>
    MyLandSettings::HpMax, 5626u16 => MyLandSettings::IsLocked, 5978u16 =>
    MyLandSettings::SpawnerAvatarGuid, 7704u16 => MyLandSettings::SpawnerAvatarId,
    6151u16 => MyLandSettings::GridRevealDelayTime, 5948u16 =>
    MyLandSettings::GridSizeMultiplier, 5981u16 => MyLandSettings::InitialDomeSize,
    5982u16 => MyLandSettings::InitialVisibleGrids, 9216u16 =>
    MyLandSettings::LootPointConfig, 6540u16 =>
    MyLandSettings::MinDistanceBetweenDefenses, 5980u16 =>
    MyLandSettings::NumGridIncreasePerLevel, 6542u16 => MyLandSettings::NumGuardDefense,
    6541u16 => MyLandSettings::NumStaticDefense, 5983u16 =>
    MyLandSettings::RadiusChangeFactor,
};
impl Attribute for MyLandSettings {
    fn class() -> Class {
        Class::MyLandSettings
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
            Self::AwareDist => &Self::AwareDist,
            Self::Defb => &Self::Defb,
            Self::InstanceGroup => &Self::InstanceGroup,
            Self::IsUnAttackable => &Self::IsUnAttackable,
            Self::Abilities => &Self::Abilities,
            Self::Alive => &Self::Alive,
            Self::AttackedBy => &Self::AttackedBy,
            Self::CarrierGuid => &Self::CarrierGuid,
            Self::ClientLoadingPriority => &Self::ClientLoadingPriority,
            Self::DirectorTags => &Self::DirectorTags,
            Self::ForceSpawnOnClient => &Self::ForceSpawnOnClient,
            Self::HpCur => &Self::HpCur,
            Self::HpMax => &Self::HpMax,
            Self::IsLocked => &Self::IsLocked,
            Self::SpawnerAvatarGuid => &Self::SpawnerAvatarGuid,
            Self::SpawnerAvatarId => &Self::SpawnerAvatarId,
            Self::GridRevealDelayTime => &Self::GridRevealDelayTime,
            Self::GridSizeMultiplier => &Self::GridSizeMultiplier,
            Self::InitialDomeSize => &Self::InitialDomeSize,
            Self::InitialVisibleGrids => &Self::InitialVisibleGrids,
            Self::LootPointConfig => &Self::LootPointConfig,
            Self::MinDistanceBetweenDefenses => &Self::MinDistanceBetweenDefenses,
            Self::NumGridIncreasePerLevel => &Self::NumGridIncreasePerLevel,
            Self::NumGuardDefense => &Self::NumGuardDefense,
            Self::NumStaticDefense => &Self::NumStaticDefense,
            Self::RadiusChangeFactor => &Self::RadiusChangeFactor,
        }
    }
}
impl AttributeInfo for MyLandSettings {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 5656u16,
            Self::Action0Duration => 5655u16,
            Self::Action0Option => 5666u16,
            Self::AlwaysVisibleToPlayers => 5639u16,
            Self::AutoReviveDelay => 10563u16,
            Self::AutoReviveTime => 10503u16,
            Self::AwareRange => 8282u16,
            Self::BeaconRadius => 10974u16,
            Self::CollisionExtent => 5654u16,
            Self::ContentClass => 5658u16,
            Self::CycleQuestBase => 11060u16,
            Self::DefaultWeapon => 7248u16,
            Self::DespawnDelay => 9673u16,
            Self::Dialogs => 8869u16,
            Self::DisplayName => 6634u16,
            Self::EnableInGame => 6860u16,
            Self::FreedomProperties => 11184u16,
            Self::Freq => 5641u16,
            Self::GenerateInterestList => 5653u16,
            Self::HiddenFromClients => 5652u16,
            Self::HiddenFromPlayers => 5668u16,
            Self::HideAfterInteraction => 9165u16,
            Self::Icon => 5636u16,
            Self::InstanceTags => 5665u16,
            Self::InstanceZoneKey => 5625u16,
            Self::InteractionDuration => 11130u16,
            Self::InteractionRadius => 7509u16,
            Self::InteractionResetTimer => 9167u16,
            Self::IsNonSpawnedAvatar => 5678u16,
            Self::IsSelfRevivable => 7193u16,
            Self::LastInteractionTime => 9166u16,
            Self::LuaScript => 7817u16,
            Self::Lvl => 6217u16,
            Self::MaterialOverride => 5629u16,
            Self::Nodelink => 5667u16,
            Self::OriginalNodeName => 5676u16,
            Self::OriginalZoneName => 5675u16,
            Self::PartyGuid => 5651u16,
            Self::PathfindSafeSpawn => 5669u16,
            Self::Pos => 5650u16,
            Self::Power => 5642u16,
            Self::Priority => 5649u16,
            Self::QuestFlags => 9971u16,
            Self::ReadableName => 5638u16,
            Self::RespawnDelay => 5679u16,
            Self::RespawnRegionName => 10821u16,
            Self::RespawnRegionNameOverride => 10880u16,
            Self::Rot => 5648u16,
            Self::SelfRadius => 5647u16,
            Self::SpawnMethod => 6135u16,
            Self::SpawnPosition => 7872u16,
            Self::SpawnRotation => 8225u16,
            Self::Tags => 5646u16,
            Self::TeamId => 5645u16,
            Self::Ue3ClassId => 5657u16,
            Self::Ue3EdVisual => 9847u16,
            Self::VisibleOnQuestAvailable => 8742u16,
            Self::VisibleOnQuestComplete => 8739u16,
            Self::VisibleOnQuestFinished => 8740u16,
            Self::VisibleOnQuestInProgress => 8741u16,
            Self::WorldZoneObjectIndex => 5677u16,
            Self::Zone => 5643u16,
            Self::ZoneGuid => 5670u16,
            Self::AwareDist => 5673u16,
            Self::Defb => 5660u16,
            Self::InstanceGroup => 11381u16,
            Self::IsUnAttackable => 12439u16,
            Self::Abilities => 9342u16,
            Self::Alive => 5664u16,
            Self::AttackedBy => 5663u16,
            Self::CarrierGuid => 5671u16,
            Self::ClientLoadingPriority => 11283u16,
            Self::DirectorTags => 8099u16,
            Self::ForceSpawnOnClient => 5672u16,
            Self::HpCur => 5662u16,
            Self::HpMax => 5661u16,
            Self::IsLocked => 5626u16,
            Self::SpawnerAvatarGuid => 5978u16,
            Self::SpawnerAvatarId => 7704u16,
            Self::GridRevealDelayTime => 6151u16,
            Self::GridSizeMultiplier => 5948u16,
            Self::InitialDomeSize => 5981u16,
            Self::InitialVisibleGrids => 5982u16,
            Self::LootPointConfig => 9216u16,
            Self::MinDistanceBetweenDefenses => 6540u16,
            Self::NumGridIncreasePerLevel => 5980u16,
            Self::NumGuardDefense => 6542u16,
            Self::NumStaticDefense => 6541u16,
            Self::RadiusChangeFactor => 5983u16,
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
            Self::AwareDist => "awareDist",
            Self::Defb => "defb",
            Self::InstanceGroup => "instanceGroup",
            Self::IsUnAttackable => "isUnAttackable",
            Self::Abilities => "abilities",
            Self::Alive => "alive",
            Self::AttackedBy => "attackedBy",
            Self::CarrierGuid => "carrierGuid",
            Self::ClientLoadingPriority => "clientLoadingPriority",
            Self::DirectorTags => "directorTags",
            Self::ForceSpawnOnClient => "forceSpawnOnClient",
            Self::HpCur => "hpCur",
            Self::HpMax => "hpMax",
            Self::IsLocked => "isLocked",
            Self::SpawnerAvatarGuid => "spawnerAvatarGuid",
            Self::SpawnerAvatarId => "spawnerAvatarID",
            Self::GridRevealDelayTime => "GridRevealDelayTime",
            Self::GridSizeMultiplier => "GridSizeMultiplier",
            Self::InitialDomeSize => "InitialDomeSize",
            Self::InitialVisibleGrids => "InitialVisibleGrids",
            Self::LootPointConfig => "LootPointConfig",
            Self::MinDistanceBetweenDefenses => "MinDistanceBetweenDefenses",
            Self::NumGridIncreasePerLevel => "NumGridIncreasePerLevel",
            Self::NumGuardDefense => "NumGuardDefense",
            Self::NumStaticDefense => "NumStaticDefense",
            Self::RadiusChangeFactor => "RadiusChangeFactor",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::GridRevealDelayTime => ParamType::Float,
            Self::GridSizeMultiplier => ParamType::Int,
            Self::InitialDomeSize => ParamType::Int,
            Self::InitialVisibleGrids => ParamType::Int,
            Self::LootPointConfig => ParamType::JsonValue,
            Self::MinDistanceBetweenDefenses => ParamType::Float,
            Self::NumGridIncreasePerLevel => ParamType::Int,
            Self::NumGuardDefense => ParamType::Int,
            Self::NumStaticDefense => ParamType::Int,
            Self::RadiusChangeFactor => ParamType::Float,
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
            Self::PathfindSafeSpawn => ParamType::Bool,
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
            Self::TeamId => ParamType::Int,
            Self::Ue3ClassId => ParamType::String,
            Self::Ue3EdVisual => ParamType::String,
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
            Self::AwareDist => ParamType::Float,
            Self::Defb => ParamType::String,
            Self::InstanceGroup => ParamType::InstanceGroup,
            Self::IsUnAttackable => ParamType::Bool,
            Self::Abilities => ParamType::ContentRefList,
            Self::Alive => ParamType::Bool,
            Self::AttackedBy => ParamType::AvatarId,
            Self::CarrierGuid => ParamType::Guid,
            Self::ClientLoadingPriority => ParamType::Int,
            Self::DirectorTags => ParamType::String,
            Self::ForceSpawnOnClient => ParamType::Bool,
            Self::HpCur => ParamType::Int,
            Self::HpMax => ParamType::Int,
            Self::IsLocked => ParamType::Bool,
            Self::SpawnerAvatarGuid => ParamType::Guid,
            Self::SpawnerAvatarId => ParamType::AvatarId,
        }
    }
    fn default(&self) -> &'static Value {
        static GRID_REVEAL_DELAY_TIME: Value = Value::Float(0.5f32);
        static GRID_SIZE_MULTIPLIER: Value = Value::Int(1i32);
        static INITIAL_DOME_SIZE: Value = Value::Int(3i32);
        static INITIAL_VISIBLE_GRIDS: Value = Value::Int(8i32);
        static LOOT_POINT_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"LootLevelBands\":[{\"Level\":3,\"LootConfig\":[{\"lootPointType\":\"Good\",\"weight\":95},{\"lootPointType\":\"Combo\",\"weight\":10},{\"lootPointType\":\"Great\",\"weight\":5}]},{\"Level\":7,\"LootConfig\":[{\"lootPointType\":\"Combo\",\"weight\":60},{\"lootPointType\":\"Good\",\"weight\":20},{\"lootPointType\":\"Great\",\"weight\":10},{\"lootPointType\":\"Bad\",\"weight\":8},{\"lootPointType\":\"Rare\",\"weight\":2}]},{\"Level\":13,\"LootConfig\":[{\"lootPointType\":\"Combo\",\"weight\":70},{\"lootPointType\":\"Bad\",\"weight\":18},{\"lootPointType\":\"Great\",\"weight\":10},{\"lootPointType\":\"Rare\",\"weight\":2}]},{\"Level\":32,\"LootConfig\":[{\"lootPointType\":\"Combo\",\"weight\":70},{\"lootPointType\":\"Great\",\"weight\":10},{\"lootPointType\":\"Bad\",\"weight\":18},{\"lootPointType\":\"Rare\",\"weight\":2},{\"lootPointType\":\"VeryBad\",\"weight\":2}]}],\"LootContents\":[{\"lootPointType\":\"Bad\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxBad1\"},{\"itemContent\":\"MyLandTreasureChestBoxBad2\"}]},{\"lootPointType\":\"Combo\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxCombo1\"},{\"itemContent\":\"MyLandTreasureChestBoxCombo2\"}]},{\"lootPointType\":\"Good\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxGood1\"},{\"itemContent\":\"MyLandTreasureChestBoxGood2\"}]},{\"lootPointType\":\"Great\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxGreat1\"},{\"itemContent\":\"MyLandTreasureChestBoxGreat2\"}]},{\"lootPointType\":\"Rare\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxRare1\"},{\"itemContent\":\"MyLandTreasureChestBoxRare2\"}]},{\"lootPointType\":\"VeryBad\",\"itemContentTemplates\":[{\"itemContent\":\"MyLandTreasureChestBoxVeryBad1\"},{\"itemContent\":\"MyLandTreasureChestBoxVeryBad2\"}]}]}",
                )
                .unwrap(),
        ));
        static MIN_DISTANCE_BETWEEN_DEFENSES: Value = Value::Float(500f32);
        static NUM_GRID_INCREASE_PER_LEVEL: Value = Value::Int(2i32);
        static NUM_GUARD_DEFENSE: Value = Value::Int(10i32);
        static NUM_STATIC_DEFENSE: Value = Value::Int(10i32);
        static RADIUS_CHANGE_FACTOR: Value = Value::Float(100f32);
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
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(true);
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
        static TEAM_ID: Value = Value::Int(0i32);
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Atlas.AtlasStructureAvatar".to_string(),
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
        static AWARE_DIST: Value = Value::Float(2500f32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static INSTANCE_GROUP: Lazy<Value> = Lazy::new(|| Value::InstanceGroup(
            String::default(),
        ));
        static IS_UN_ATTACKABLE: Value = Value::Bool(true);
        static ABILITIES: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static ALIVE: Value = Value::Bool(true);
        static ATTACKED_BY: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static CARRIER_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static CLIENT_LOADING_PRIORITY: Value = Value::Int(0i32);
        static DIRECTOR_TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static FORCE_SPAWN_ON_CLIENT: Value = Value::Bool(false);
        static HP_CUR: Value = Value::Int(2000i32);
        static HP_MAX: Value = Value::Int(2000i32);
        static IS_LOCKED: Value = Value::Bool(false);
        static SPAWNER_AVATAR_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static SPAWNER_AVATAR_ID: Value = Value::AvatarId(AvatarId::from_u64(0));
        match self {
            Self::GridRevealDelayTime => &GRID_REVEAL_DELAY_TIME,
            Self::GridSizeMultiplier => &GRID_SIZE_MULTIPLIER,
            Self::InitialDomeSize => &INITIAL_DOME_SIZE,
            Self::InitialVisibleGrids => &INITIAL_VISIBLE_GRIDS,
            Self::LootPointConfig => &LOOT_POINT_CONFIG,
            Self::MinDistanceBetweenDefenses => &MIN_DISTANCE_BETWEEN_DEFENSES,
            Self::NumGridIncreasePerLevel => &NUM_GRID_INCREASE_PER_LEVEL,
            Self::NumGuardDefense => &NUM_GUARD_DEFENSE,
            Self::NumStaticDefense => &NUM_STATIC_DEFENSE,
            Self::RadiusChangeFactor => &RADIUS_CHANGE_FACTOR,
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
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
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
            Self::TeamId => &TEAM_ID,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
            Self::AwareDist => &AWARE_DIST,
            Self::Defb => &DEFB,
            Self::InstanceGroup => &INSTANCE_GROUP,
            Self::IsUnAttackable => &IS_UN_ATTACKABLE,
            Self::Abilities => &ABILITIES,
            Self::Alive => &ALIVE,
            Self::AttackedBy => &ATTACKED_BY,
            Self::CarrierGuid => &CARRIER_GUID,
            Self::ClientLoadingPriority => &CLIENT_LOADING_PRIORITY,
            Self::DirectorTags => &DIRECTOR_TAGS,
            Self::ForceSpawnOnClient => &FORCE_SPAWN_ON_CLIENT,
            Self::HpCur => &HP_CUR,
            Self::HpMax => &HP_MAX,
            Self::IsLocked => &IS_LOCKED,
            Self::SpawnerAvatarGuid => &SPAWNER_AVATAR_GUID,
            Self::SpawnerAvatarId => &SPAWNER_AVATAR_ID,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::GridRevealDelayTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::GridSizeMultiplier => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InitialDomeSize => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InitialVisibleGrids => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LootPointConfig => &[ParamFlag::PerInstanceSetting],
            Self::MinDistanceBetweenDefenses => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NumGridIncreasePerLevel => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NumGuardDefense => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NumStaticDefense => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RadiusChangeFactor => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
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
                    ParamFlag::PerInstanceSetting,
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
            Self::PathfindSafeSpawn => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
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
            Self::TeamId => {
                &[
                    ParamFlag::NodeOwn,
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
            Self::AwareDist => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Deprecated]
            }
            Self::Defb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InstanceGroup => {
                &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting]
            }
            Self::IsUnAttackable => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Abilities => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Alive => &[ParamFlag::NodeOwn],
            Self::AttackedBy => &[ParamFlag::NodeOwn, ParamFlag::DupeSetOk],
            Self::CarrierGuid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ClientLoadingPriority => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DirectorTags => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ForceSpawnOnClient => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HpCur => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::HpMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsLocked => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnerAvatarGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SpawnerAvatarId => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for MyLandSettings {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MY_LAND_SETTINGS_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MyLandSettings {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5656u16 => Ok(Self::Action0),
            5655u16 => Ok(Self::Action0Duration),
            5666u16 => Ok(Self::Action0Option),
            5639u16 => Ok(Self::AlwaysVisibleToPlayers),
            10563u16 => Ok(Self::AutoReviveDelay),
            10503u16 => Ok(Self::AutoReviveTime),
            8282u16 => Ok(Self::AwareRange),
            10974u16 => Ok(Self::BeaconRadius),
            5654u16 => Ok(Self::CollisionExtent),
            5658u16 => Ok(Self::ContentClass),
            11060u16 => Ok(Self::CycleQuestBase),
            7248u16 => Ok(Self::DefaultWeapon),
            9673u16 => Ok(Self::DespawnDelay),
            8869u16 => Ok(Self::Dialogs),
            6634u16 => Ok(Self::DisplayName),
            6860u16 => Ok(Self::EnableInGame),
            11184u16 => Ok(Self::FreedomProperties),
            5641u16 => Ok(Self::Freq),
            5653u16 => Ok(Self::GenerateInterestList),
            5652u16 => Ok(Self::HiddenFromClients),
            5668u16 => Ok(Self::HiddenFromPlayers),
            9165u16 => Ok(Self::HideAfterInteraction),
            5636u16 => Ok(Self::Icon),
            5665u16 => Ok(Self::InstanceTags),
            5625u16 => Ok(Self::InstanceZoneKey),
            11130u16 => Ok(Self::InteractionDuration),
            7509u16 => Ok(Self::InteractionRadius),
            9167u16 => Ok(Self::InteractionResetTimer),
            5678u16 => Ok(Self::IsNonSpawnedAvatar),
            7193u16 => Ok(Self::IsSelfRevivable),
            9166u16 => Ok(Self::LastInteractionTime),
            7817u16 => Ok(Self::LuaScript),
            6217u16 => Ok(Self::Lvl),
            5629u16 => Ok(Self::MaterialOverride),
            5667u16 => Ok(Self::Nodelink),
            5676u16 => Ok(Self::OriginalNodeName),
            5675u16 => Ok(Self::OriginalZoneName),
            5651u16 => Ok(Self::PartyGuid),
            5669u16 => Ok(Self::PathfindSafeSpawn),
            5650u16 => Ok(Self::Pos),
            5642u16 => Ok(Self::Power),
            5649u16 => Ok(Self::Priority),
            9971u16 => Ok(Self::QuestFlags),
            5638u16 => Ok(Self::ReadableName),
            5679u16 => Ok(Self::RespawnDelay),
            10821u16 => Ok(Self::RespawnRegionName),
            10880u16 => Ok(Self::RespawnRegionNameOverride),
            5648u16 => Ok(Self::Rot),
            5647u16 => Ok(Self::SelfRadius),
            6135u16 => Ok(Self::SpawnMethod),
            7872u16 => Ok(Self::SpawnPosition),
            8225u16 => Ok(Self::SpawnRotation),
            5646u16 => Ok(Self::Tags),
            5645u16 => Ok(Self::TeamId),
            5657u16 => Ok(Self::Ue3ClassId),
            9847u16 => Ok(Self::Ue3EdVisual),
            8742u16 => Ok(Self::VisibleOnQuestAvailable),
            8739u16 => Ok(Self::VisibleOnQuestComplete),
            8740u16 => Ok(Self::VisibleOnQuestFinished),
            8741u16 => Ok(Self::VisibleOnQuestInProgress),
            5677u16 => Ok(Self::WorldZoneObjectIndex),
            5643u16 => Ok(Self::Zone),
            5670u16 => Ok(Self::ZoneGuid),
            5673u16 => Ok(Self::AwareDist),
            5660u16 => Ok(Self::Defb),
            11381u16 => Ok(Self::InstanceGroup),
            12439u16 => Ok(Self::IsUnAttackable),
            9342u16 => Ok(Self::Abilities),
            5664u16 => Ok(Self::Alive),
            5663u16 => Ok(Self::AttackedBy),
            5671u16 => Ok(Self::CarrierGuid),
            11283u16 => Ok(Self::ClientLoadingPriority),
            8099u16 => Ok(Self::DirectorTags),
            5672u16 => Ok(Self::ForceSpawnOnClient),
            5662u16 => Ok(Self::HpCur),
            5661u16 => Ok(Self::HpMax),
            5626u16 => Ok(Self::IsLocked),
            5978u16 => Ok(Self::SpawnerAvatarGuid),
            7704u16 => Ok(Self::SpawnerAvatarId),
            6151u16 => Ok(Self::GridRevealDelayTime),
            5948u16 => Ok(Self::GridSizeMultiplier),
            5981u16 => Ok(Self::InitialDomeSize),
            5982u16 => Ok(Self::InitialVisibleGrids),
            9216u16 => Ok(Self::LootPointConfig),
            6540u16 => Ok(Self::MinDistanceBetweenDefenses),
            5980u16 => Ok(Self::NumGridIncreasePerLevel),
            6542u16 => Ok(Self::NumGuardDefense),
            6541u16 => Ok(Self::NumStaticDefense),
            5983u16 => Ok(Self::RadiusChangeFactor),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
