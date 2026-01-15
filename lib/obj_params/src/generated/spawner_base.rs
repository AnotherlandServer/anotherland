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
pub enum SpawnerBase {
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
    ExactCount,
    InstanceGroup,
    IsUnAttackable,
    SameRespawnType,
    SpawnInWave,
}
pub(crate) static SPAWNER_BASE_ATTRIBUTES: phf::Map<&'static str, SpawnerBase> = phf_map! {
    "action0" => SpawnerBase::Action0, "action0Duration" => SpawnerBase::Action0Duration,
    "action0Option" => SpawnerBase::Action0Option, "alwaysVisibleToPlayers" =>
    SpawnerBase::AlwaysVisibleToPlayers, "autoReviveDelay" =>
    SpawnerBase::AutoReviveDelay, "autoReviveTime" => SpawnerBase::AutoReviveTime,
    "AwareRange" => SpawnerBase::AwareRange, "BeaconRadius" => SpawnerBase::BeaconRadius,
    "collisionExtent" => SpawnerBase::CollisionExtent, "ContentClass" =>
    SpawnerBase::ContentClass, "CycleQuestBase" => SpawnerBase::CycleQuestBase,
    "defaultWeapon" => SpawnerBase::DefaultWeapon, "despawnDelay" =>
    SpawnerBase::DespawnDelay, "Dialogs" => SpawnerBase::Dialogs, "DisplayName" =>
    SpawnerBase::DisplayName, "EnableInGame" => SpawnerBase::EnableInGame,
    "FreedomProperties" => SpawnerBase::FreedomProperties, "Freq" => SpawnerBase::Freq,
    "generateInterestList" => SpawnerBase::GenerateInterestList, "hiddenFromClients" =>
    SpawnerBase::HiddenFromClients, "hiddenFromPlayers" =>
    SpawnerBase::HiddenFromPlayers, "HideAfterInteraction" =>
    SpawnerBase::HideAfterInteraction, "Icon" => SpawnerBase::Icon, "instanceTags" =>
    SpawnerBase::InstanceTags, "instanceZoneKey" => SpawnerBase::InstanceZoneKey,
    "InteractionDuration" => SpawnerBase::InteractionDuration, "InteractionRadius" =>
    SpawnerBase::InteractionRadius, "InteractionResetTimer" =>
    SpawnerBase::InteractionResetTimer, "isNonSpawnedAvatar" =>
    SpawnerBase::IsNonSpawnedAvatar, "isSelfRevivable" => SpawnerBase::IsSelfRevivable,
    "LastInteractionTime" => SpawnerBase::LastInteractionTime, "LuaScript" =>
    SpawnerBase::LuaScript, "lvl" => SpawnerBase::Lvl, "MaterialOverride" =>
    SpawnerBase::MaterialOverride, "nodelink" => SpawnerBase::Nodelink,
    "originalNodeName" => SpawnerBase::OriginalNodeName, "originalZoneName" =>
    SpawnerBase::OriginalZoneName, "partyGUID" => SpawnerBase::PartyGuid,
    "pathfindSafeSpawn" => SpawnerBase::PathfindSafeSpawn, "pos" => SpawnerBase::Pos,
    "Power" => SpawnerBase::Power, "priority" => SpawnerBase::Priority, "QuestFlags" =>
    SpawnerBase::QuestFlags, "ReadableName" => SpawnerBase::ReadableName, "respawnDelay"
    => SpawnerBase::RespawnDelay, "RespawnRegionName" => SpawnerBase::RespawnRegionName,
    "RespawnRegionNameOverride" => SpawnerBase::RespawnRegionNameOverride, "rot" =>
    SpawnerBase::Rot, "selfRadius" => SpawnerBase::SelfRadius, "spawnMethod" =>
    SpawnerBase::SpawnMethod, "spawnPosition" => SpawnerBase::SpawnPosition,
    "spawnRotation" => SpawnerBase::SpawnRotation, "tags" => SpawnerBase::Tags, "teamID"
    => SpawnerBase::TeamId, "UE3ClassID" => SpawnerBase::Ue3ClassId, "UE3EdVisual" =>
    SpawnerBase::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    SpawnerBase::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    SpawnerBase::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    SpawnerBase::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    SpawnerBase::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    SpawnerBase::WorldZoneObjectIndex, "zone" => SpawnerBase::Zone, "ZoneGuid" =>
    SpawnerBase::ZoneGuid, "exactCount" => SpawnerBase::ExactCount, "instanceGroup" =>
    SpawnerBase::InstanceGroup, "isUnAttackable" => SpawnerBase::IsUnAttackable,
    "sameRespawnType" => SpawnerBase::SameRespawnType, "spawnInWave" =>
    SpawnerBase::SpawnInWave,
};
pub(crate) static SPAWNER_BASE_ATTRIBUTES_ID: phf::Map<u16, SpawnerBase> = phf_map! {
    865u16 => SpawnerBase::Action0, 866u16 => SpawnerBase::Action0Duration, 860u16 =>
    SpawnerBase::Action0Option, 3520u16 => SpawnerBase::AlwaysVisibleToPlayers, 10520u16
    => SpawnerBase::AutoReviveDelay, 10460u16 => SpawnerBase::AutoReviveTime, 8239u16 =>
    SpawnerBase::AwareRange, 10931u16 => SpawnerBase::BeaconRadius, 867u16 =>
    SpawnerBase::CollisionExtent, 863u16 => SpawnerBase::ContentClass, 11065u16 =>
    SpawnerBase::CycleQuestBase, 7251u16 => SpawnerBase::DefaultWeapon, 9630u16 =>
    SpawnerBase::DespawnDelay, 8826u16 => SpawnerBase::Dialogs, 6637u16 =>
    SpawnerBase::DisplayName, 6863u16 => SpawnerBase::EnableInGame, 11189u16 =>
    SpawnerBase::FreedomProperties, 879u16 => SpawnerBase::Freq, 868u16 =>
    SpawnerBase::GenerateInterestList, 869u16 => SpawnerBase::HiddenFromClients, 858u16
    => SpawnerBase::HiddenFromPlayers, 9036u16 => SpawnerBase::HideAfterInteraction,
    4380u16 => SpawnerBase::Icon, 861u16 => SpawnerBase::InstanceTags, 5597u16 =>
    SpawnerBase::InstanceZoneKey, 11135u16 => SpawnerBase::InteractionDuration, 7512u16
    => SpawnerBase::InteractionRadius, 9038u16 => SpawnerBase::InteractionResetTimer,
    852u16 => SpawnerBase::IsNonSpawnedAvatar, 7196u16 => SpawnerBase::IsSelfRevivable,
    9037u16 => SpawnerBase::LastInteractionTime, 7774u16 => SpawnerBase::LuaScript,
    6220u16 => SpawnerBase::Lvl, 4761u16 => SpawnerBase::MaterialOverride, 859u16 =>
    SpawnerBase::Nodelink, 854u16 => SpawnerBase::OriginalNodeName, 855u16 =>
    SpawnerBase::OriginalZoneName, 870u16 => SpawnerBase::PartyGuid, 857u16 =>
    SpawnerBase::PathfindSafeSpawn, 862u16 => SpawnerBase::Pos, 878u16 =>
    SpawnerBase::Power, 871u16 => SpawnerBase::Priority, 9928u16 =>
    SpawnerBase::QuestFlags, 3705u16 => SpawnerBase::ReadableName, 851u16 =>
    SpawnerBase::RespawnDelay, 10778u16 => SpawnerBase::RespawnRegionName, 10837u16 =>
    SpawnerBase::RespawnRegionNameOverride, 872u16 => SpawnerBase::Rot, 873u16 =>
    SpawnerBase::SelfRadius, 6138u16 => SpawnerBase::SpawnMethod, 7829u16 =>
    SpawnerBase::SpawnPosition, 8182u16 => SpawnerBase::SpawnRotation, 874u16 =>
    SpawnerBase::Tags, 875u16 => SpawnerBase::TeamId, 864u16 => SpawnerBase::Ue3ClassId,
    9804u16 => SpawnerBase::Ue3EdVisual, 8570u16 => SpawnerBase::VisibleOnQuestAvailable,
    8567u16 => SpawnerBase::VisibleOnQuestComplete, 8568u16 =>
    SpawnerBase::VisibleOnQuestFinished, 8569u16 =>
    SpawnerBase::VisibleOnQuestInProgress, 853u16 => SpawnerBase::WorldZoneObjectIndex,
    877u16 => SpawnerBase::Zone, 856u16 => SpawnerBase::ZoneGuid, 5683u16 =>
    SpawnerBase::ExactCount, 11386u16 => SpawnerBase::InstanceGroup, 10382u16 =>
    SpawnerBase::IsUnAttackable, 5682u16 => SpawnerBase::SameRespawnType, 5617u16 =>
    SpawnerBase::SpawnInWave,
};
impl Attribute for SpawnerBase {
    fn class() -> Class {
        Class::SpawnerBase
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
            Self::ExactCount => &Self::ExactCount,
            Self::InstanceGroup => &Self::InstanceGroup,
            Self::IsUnAttackable => &Self::IsUnAttackable,
            Self::SameRespawnType => &Self::SameRespawnType,
            Self::SpawnInWave => &Self::SpawnInWave,
        }
    }
}
impl AttributeInfo for SpawnerBase {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 865u16,
            Self::Action0Duration => 866u16,
            Self::Action0Option => 860u16,
            Self::AlwaysVisibleToPlayers => 3520u16,
            Self::AutoReviveDelay => 10520u16,
            Self::AutoReviveTime => 10460u16,
            Self::AwareRange => 8239u16,
            Self::BeaconRadius => 10931u16,
            Self::CollisionExtent => 867u16,
            Self::ContentClass => 863u16,
            Self::CycleQuestBase => 11065u16,
            Self::DefaultWeapon => 7251u16,
            Self::DespawnDelay => 9630u16,
            Self::Dialogs => 8826u16,
            Self::DisplayName => 6637u16,
            Self::EnableInGame => 6863u16,
            Self::FreedomProperties => 11189u16,
            Self::Freq => 879u16,
            Self::GenerateInterestList => 868u16,
            Self::HiddenFromClients => 869u16,
            Self::HiddenFromPlayers => 858u16,
            Self::HideAfterInteraction => 9036u16,
            Self::Icon => 4380u16,
            Self::InstanceTags => 861u16,
            Self::InstanceZoneKey => 5597u16,
            Self::InteractionDuration => 11135u16,
            Self::InteractionRadius => 7512u16,
            Self::InteractionResetTimer => 9038u16,
            Self::IsNonSpawnedAvatar => 852u16,
            Self::IsSelfRevivable => 7196u16,
            Self::LastInteractionTime => 9037u16,
            Self::LuaScript => 7774u16,
            Self::Lvl => 6220u16,
            Self::MaterialOverride => 4761u16,
            Self::Nodelink => 859u16,
            Self::OriginalNodeName => 854u16,
            Self::OriginalZoneName => 855u16,
            Self::PartyGuid => 870u16,
            Self::PathfindSafeSpawn => 857u16,
            Self::Pos => 862u16,
            Self::Power => 878u16,
            Self::Priority => 871u16,
            Self::QuestFlags => 9928u16,
            Self::ReadableName => 3705u16,
            Self::RespawnDelay => 851u16,
            Self::RespawnRegionName => 10778u16,
            Self::RespawnRegionNameOverride => 10837u16,
            Self::Rot => 872u16,
            Self::SelfRadius => 873u16,
            Self::SpawnMethod => 6138u16,
            Self::SpawnPosition => 7829u16,
            Self::SpawnRotation => 8182u16,
            Self::Tags => 874u16,
            Self::TeamId => 875u16,
            Self::Ue3ClassId => 864u16,
            Self::Ue3EdVisual => 9804u16,
            Self::VisibleOnQuestAvailable => 8570u16,
            Self::VisibleOnQuestComplete => 8567u16,
            Self::VisibleOnQuestFinished => 8568u16,
            Self::VisibleOnQuestInProgress => 8569u16,
            Self::WorldZoneObjectIndex => 853u16,
            Self::Zone => 877u16,
            Self::ZoneGuid => 856u16,
            Self::ExactCount => 5683u16,
            Self::InstanceGroup => 11386u16,
            Self::IsUnAttackable => 10382u16,
            Self::SameRespawnType => 5682u16,
            Self::SpawnInWave => 5617u16,
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
            Self::ExactCount => "exactCount",
            Self::InstanceGroup => "instanceGroup",
            Self::IsUnAttackable => "isUnAttackable",
            Self::SameRespawnType => "sameRespawnType",
            Self::SpawnInWave => "spawnInWave",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::GenerateInterestList => ParamType::Bool,
            Self::HiddenFromPlayers => ParamType::Bool,
            Self::Lvl => ParamType::Int,
            Self::ExactCount => ParamType::Bool,
            Self::InstanceGroup => ParamType::InstanceGroup,
            Self::IsUnAttackable => ParamType::Bool,
            Self::SameRespawnType => ParamType::Bool,
            Self::SpawnInWave => ParamType::Bool,
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
            Self::HiddenFromClients => ParamType::Bool,
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
        }
    }
    fn default(&self) -> &'static Value {
        static GENERATE_INTEREST_LIST: Value = Value::Bool(false);
        static HIDDEN_FROM_PLAYERS: Value = Value::Bool(true);
        static LVL: Value = Value::Int(-1i32);
        static EXACT_COUNT: Value = Value::Bool(false);
        static INSTANCE_GROUP: Lazy<Value> = Lazy::new(|| Value::InstanceGroup(
            String::default(),
        ));
        static IS_UN_ATTACKABLE: Value = Value::Bool(true);
        static SAME_RESPAWN_TYPE: Value = Value::Bool(false);
        static SPAWN_IN_WAVE: Value = Value::Bool(false);
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
        static HIDDEN_FROM_CLIENTS: Value = Value::Bool(false);
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
            "Engine.AtlasAvatar".to_string(),
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
        match self {
            Self::GenerateInterestList => &GENERATE_INTEREST_LIST,
            Self::HiddenFromPlayers => &HIDDEN_FROM_PLAYERS,
            Self::Lvl => &LVL,
            Self::ExactCount => &EXACT_COUNT,
            Self::InstanceGroup => &INSTANCE_GROUP,
            Self::IsUnAttackable => &IS_UN_ATTACKABLE,
            Self::SameRespawnType => &SAME_RESPAWN_TYPE,
            Self::SpawnInWave => &SPAWN_IN_WAVE,
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
            Self::HiddenFromClients => &HIDDEN_FROM_CLIENTS,
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
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::GenerateInterestList => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::HiddenFromPlayers => {
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
            Self::ExactCount => {
                &[
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
            Self::SameRespawnType => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnInWave => &[ParamFlag::Persistent, ParamFlag::Content],
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
            Self::HiddenFromClients => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
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
        }
    }
}
impl FromStr for SpawnerBase {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SPAWNER_BASE_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for SpawnerBase {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            865u16 => Ok(Self::Action0),
            866u16 => Ok(Self::Action0Duration),
            860u16 => Ok(Self::Action0Option),
            3520u16 => Ok(Self::AlwaysVisibleToPlayers),
            10520u16 => Ok(Self::AutoReviveDelay),
            10460u16 => Ok(Self::AutoReviveTime),
            8239u16 => Ok(Self::AwareRange),
            10931u16 => Ok(Self::BeaconRadius),
            867u16 => Ok(Self::CollisionExtent),
            863u16 => Ok(Self::ContentClass),
            11065u16 => Ok(Self::CycleQuestBase),
            7251u16 => Ok(Self::DefaultWeapon),
            9630u16 => Ok(Self::DespawnDelay),
            8826u16 => Ok(Self::Dialogs),
            6637u16 => Ok(Self::DisplayName),
            6863u16 => Ok(Self::EnableInGame),
            11189u16 => Ok(Self::FreedomProperties),
            879u16 => Ok(Self::Freq),
            868u16 => Ok(Self::GenerateInterestList),
            869u16 => Ok(Self::HiddenFromClients),
            858u16 => Ok(Self::HiddenFromPlayers),
            9036u16 => Ok(Self::HideAfterInteraction),
            4380u16 => Ok(Self::Icon),
            861u16 => Ok(Self::InstanceTags),
            5597u16 => Ok(Self::InstanceZoneKey),
            11135u16 => Ok(Self::InteractionDuration),
            7512u16 => Ok(Self::InteractionRadius),
            9038u16 => Ok(Self::InteractionResetTimer),
            852u16 => Ok(Self::IsNonSpawnedAvatar),
            7196u16 => Ok(Self::IsSelfRevivable),
            9037u16 => Ok(Self::LastInteractionTime),
            7774u16 => Ok(Self::LuaScript),
            6220u16 => Ok(Self::Lvl),
            4761u16 => Ok(Self::MaterialOverride),
            859u16 => Ok(Self::Nodelink),
            854u16 => Ok(Self::OriginalNodeName),
            855u16 => Ok(Self::OriginalZoneName),
            870u16 => Ok(Self::PartyGuid),
            857u16 => Ok(Self::PathfindSafeSpawn),
            862u16 => Ok(Self::Pos),
            878u16 => Ok(Self::Power),
            871u16 => Ok(Self::Priority),
            9928u16 => Ok(Self::QuestFlags),
            3705u16 => Ok(Self::ReadableName),
            851u16 => Ok(Self::RespawnDelay),
            10778u16 => Ok(Self::RespawnRegionName),
            10837u16 => Ok(Self::RespawnRegionNameOverride),
            872u16 => Ok(Self::Rot),
            873u16 => Ok(Self::SelfRadius),
            6138u16 => Ok(Self::SpawnMethod),
            7829u16 => Ok(Self::SpawnPosition),
            8182u16 => Ok(Self::SpawnRotation),
            874u16 => Ok(Self::Tags),
            875u16 => Ok(Self::TeamId),
            864u16 => Ok(Self::Ue3ClassId),
            9804u16 => Ok(Self::Ue3EdVisual),
            8570u16 => Ok(Self::VisibleOnQuestAvailable),
            8567u16 => Ok(Self::VisibleOnQuestComplete),
            8568u16 => Ok(Self::VisibleOnQuestFinished),
            8569u16 => Ok(Self::VisibleOnQuestInProgress),
            853u16 => Ok(Self::WorldZoneObjectIndex),
            877u16 => Ok(Self::Zone),
            856u16 => Ok(Self::ZoneGuid),
            5683u16 => Ok(Self::ExactCount),
            11386u16 => Ok(Self::InstanceGroup),
            10382u16 => Ok(Self::IsUnAttackable),
            5682u16 => Ok(Self::SameRespawnType),
            5617u16 => Ok(Self::SpawnInWave),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
