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
pub enum CtfGameFlag {
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
    AdditionalThreatPerSecond,
    CaptureDistance,
    DurationToCapture,
    HasCarrier,
}
pub(crate) static CTF_GAME_FLAG_ATTRIBUTES: phf::Map<&'static str, CtfGameFlag> = phf_map! {
    "action0" => CtfGameFlag::Action0, "action0Duration" => CtfGameFlag::Action0Duration,
    "action0Option" => CtfGameFlag::Action0Option, "alwaysVisibleToPlayers" =>
    CtfGameFlag::AlwaysVisibleToPlayers, "autoReviveDelay" =>
    CtfGameFlag::AutoReviveDelay, "autoReviveTime" => CtfGameFlag::AutoReviveTime,
    "AwareRange" => CtfGameFlag::AwareRange, "BeaconRadius" => CtfGameFlag::BeaconRadius,
    "collisionExtent" => CtfGameFlag::CollisionExtent, "ContentClass" =>
    CtfGameFlag::ContentClass, "CycleQuestBase" => CtfGameFlag::CycleQuestBase,
    "defaultWeapon" => CtfGameFlag::DefaultWeapon, "despawnDelay" =>
    CtfGameFlag::DespawnDelay, "Dialogs" => CtfGameFlag::Dialogs, "DisplayName" =>
    CtfGameFlag::DisplayName, "EnableInGame" => CtfGameFlag::EnableInGame,
    "FreedomProperties" => CtfGameFlag::FreedomProperties, "Freq" => CtfGameFlag::Freq,
    "generateInterestList" => CtfGameFlag::GenerateInterestList, "hiddenFromClients" =>
    CtfGameFlag::HiddenFromClients, "hiddenFromPlayers" =>
    CtfGameFlag::HiddenFromPlayers, "HideAfterInteraction" =>
    CtfGameFlag::HideAfterInteraction, "Icon" => CtfGameFlag::Icon, "instanceTags" =>
    CtfGameFlag::InstanceTags, "instanceZoneKey" => CtfGameFlag::InstanceZoneKey,
    "InteractionDuration" => CtfGameFlag::InteractionDuration, "InteractionRadius" =>
    CtfGameFlag::InteractionRadius, "InteractionResetTimer" =>
    CtfGameFlag::InteractionResetTimer, "isNonSpawnedAvatar" =>
    CtfGameFlag::IsNonSpawnedAvatar, "isSelfRevivable" => CtfGameFlag::IsSelfRevivable,
    "LastInteractionTime" => CtfGameFlag::LastInteractionTime, "LuaScript" =>
    CtfGameFlag::LuaScript, "lvl" => CtfGameFlag::Lvl, "MaterialOverride" =>
    CtfGameFlag::MaterialOverride, "nodelink" => CtfGameFlag::Nodelink,
    "originalNodeName" => CtfGameFlag::OriginalNodeName, "originalZoneName" =>
    CtfGameFlag::OriginalZoneName, "partyGUID" => CtfGameFlag::PartyGuid,
    "pathfindSafeSpawn" => CtfGameFlag::PathfindSafeSpawn, "pos" => CtfGameFlag::Pos,
    "Power" => CtfGameFlag::Power, "priority" => CtfGameFlag::Priority, "QuestFlags" =>
    CtfGameFlag::QuestFlags, "ReadableName" => CtfGameFlag::ReadableName, "respawnDelay"
    => CtfGameFlag::RespawnDelay, "RespawnRegionName" => CtfGameFlag::RespawnRegionName,
    "RespawnRegionNameOverride" => CtfGameFlag::RespawnRegionNameOverride, "rot" =>
    CtfGameFlag::Rot, "selfRadius" => CtfGameFlag::SelfRadius, "spawnMethod" =>
    CtfGameFlag::SpawnMethod, "spawnPosition" => CtfGameFlag::SpawnPosition,
    "spawnRotation" => CtfGameFlag::SpawnRotation, "tags" => CtfGameFlag::Tags, "teamID"
    => CtfGameFlag::TeamId, "UE3ClassID" => CtfGameFlag::Ue3ClassId, "UE3EdVisual" =>
    CtfGameFlag::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    CtfGameFlag::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    CtfGameFlag::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    CtfGameFlag::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    CtfGameFlag::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    CtfGameFlag::WorldZoneObjectIndex, "zone" => CtfGameFlag::Zone, "ZoneGuid" =>
    CtfGameFlag::ZoneGuid, "awareDist" => CtfGameFlag::AwareDist, "defb" =>
    CtfGameFlag::Defb, "instanceGroup" => CtfGameFlag::InstanceGroup, "isUnAttackable" =>
    CtfGameFlag::IsUnAttackable, "abilities" => CtfGameFlag::Abilities, "alive" =>
    CtfGameFlag::Alive, "attackedBy" => CtfGameFlag::AttackedBy, "carrierGuid" =>
    CtfGameFlag::CarrierGuid, "clientLoadingPriority" =>
    CtfGameFlag::ClientLoadingPriority, "directorTags" => CtfGameFlag::DirectorTags,
    "forceSpawnOnClient" => CtfGameFlag::ForceSpawnOnClient, "hpCur" =>
    CtfGameFlag::HpCur, "hpMax" => CtfGameFlag::HpMax, "isLocked" =>
    CtfGameFlag::IsLocked, "spawnerAvatarGuid" => CtfGameFlag::SpawnerAvatarGuid,
    "spawnerAvatarID" => CtfGameFlag::SpawnerAvatarId, "additionalThreatPerSecond" =>
    CtfGameFlag::AdditionalThreatPerSecond, "captureDistance" =>
    CtfGameFlag::CaptureDistance, "durationToCapture" => CtfGameFlag::DurationToCapture,
    "hasCarrier" => CtfGameFlag::HasCarrier,
};
pub(crate) static CTF_GAME_FLAG_ATTRIBUTES_ID: phf::Map<u16, CtfGameFlag> = phf_map! {
    5186u16 => CtfGameFlag::Action0, 5185u16 => CtfGameFlag::Action0Duration, 5196u16 =>
    CtfGameFlag::Action0Option, 5167u16 => CtfGameFlag::AlwaysVisibleToPlayers, 10556u16
    => CtfGameFlag::AutoReviveDelay, 10496u16 => CtfGameFlag::AutoReviveTime, 8275u16 =>
    CtfGameFlag::AwareRange, 10967u16 => CtfGameFlag::BeaconRadius, 5184u16 =>
    CtfGameFlag::CollisionExtent, 5188u16 => CtfGameFlag::ContentClass, 11054u16 =>
    CtfGameFlag::CycleQuestBase, 7244u16 => CtfGameFlag::DefaultWeapon, 9666u16 =>
    CtfGameFlag::DespawnDelay, 8862u16 => CtfGameFlag::Dialogs, 6630u16 =>
    CtfGameFlag::DisplayName, 6856u16 => CtfGameFlag::EnableInGame, 11178u16 =>
    CtfGameFlag::FreedomProperties, 5171u16 => CtfGameFlag::Freq, 5183u16 =>
    CtfGameFlag::GenerateInterestList, 5182u16 => CtfGameFlag::HiddenFromClients, 5198u16
    => CtfGameFlag::HiddenFromPlayers, 9144u16 => CtfGameFlag::HideAfterInteraction,
    5164u16 => CtfGameFlag::Icon, 5195u16 => CtfGameFlag::InstanceTags, 5593u16 =>
    CtfGameFlag::InstanceZoneKey, 11124u16 => CtfGameFlag::InteractionDuration, 7505u16
    => CtfGameFlag::InteractionRadius, 9146u16 => CtfGameFlag::InteractionResetTimer,
    5208u16 => CtfGameFlag::IsNonSpawnedAvatar, 7189u16 => CtfGameFlag::IsSelfRevivable,
    9145u16 => CtfGameFlag::LastInteractionTime, 7810u16 => CtfGameFlag::LuaScript,
    6213u16 => CtfGameFlag::Lvl, 5157u16 => CtfGameFlag::MaterialOverride, 5197u16 =>
    CtfGameFlag::Nodelink, 5206u16 => CtfGameFlag::OriginalNodeName, 5205u16 =>
    CtfGameFlag::OriginalZoneName, 5181u16 => CtfGameFlag::PartyGuid, 5199u16 =>
    CtfGameFlag::PathfindSafeSpawn, 5180u16 => CtfGameFlag::Pos, 5172u16 =>
    CtfGameFlag::Power, 5179u16 => CtfGameFlag::Priority, 9964u16 =>
    CtfGameFlag::QuestFlags, 5166u16 => CtfGameFlag::ReadableName, 5209u16 =>
    CtfGameFlag::RespawnDelay, 10814u16 => CtfGameFlag::RespawnRegionName, 10873u16 =>
    CtfGameFlag::RespawnRegionNameOverride, 5178u16 => CtfGameFlag::Rot, 5177u16 =>
    CtfGameFlag::SelfRadius, 6131u16 => CtfGameFlag::SpawnMethod, 7865u16 =>
    CtfGameFlag::SpawnPosition, 8218u16 => CtfGameFlag::SpawnRotation, 5176u16 =>
    CtfGameFlag::Tags, 5175u16 => CtfGameFlag::TeamId, 5187u16 =>
    CtfGameFlag::Ue3ClassId, 9840u16 => CtfGameFlag::Ue3EdVisual, 8714u16 =>
    CtfGameFlag::VisibleOnQuestAvailable, 8711u16 => CtfGameFlag::VisibleOnQuestComplete,
    8712u16 => CtfGameFlag::VisibleOnQuestFinished, 8713u16 =>
    CtfGameFlag::VisibleOnQuestInProgress, 5207u16 => CtfGameFlag::WorldZoneObjectIndex,
    5173u16 => CtfGameFlag::Zone, 5200u16 => CtfGameFlag::ZoneGuid, 5203u16 =>
    CtfGameFlag::AwareDist, 5190u16 => CtfGameFlag::Defb, 11375u16 =>
    CtfGameFlag::InstanceGroup, 12435u16 => CtfGameFlag::IsUnAttackable, 9335u16 =>
    CtfGameFlag::Abilities, 5194u16 => CtfGameFlag::Alive, 5193u16 =>
    CtfGameFlag::AttackedBy, 5201u16 => CtfGameFlag::CarrierGuid, 11277u16 =>
    CtfGameFlag::ClientLoadingPriority, 8092u16 => CtfGameFlag::DirectorTags, 5202u16 =>
    CtfGameFlag::ForceSpawnOnClient, 5192u16 => CtfGameFlag::HpCur, 5191u16 =>
    CtfGameFlag::HpMax, 5489u16 => CtfGameFlag::IsLocked, 5974u16 =>
    CtfGameFlag::SpawnerAvatarGuid, 7697u16 => CtfGameFlag::SpawnerAvatarId, 5615u16 =>
    CtfGameFlag::AdditionalThreatPerSecond, 5156u16 => CtfGameFlag::CaptureDistance,
    5616u16 => CtfGameFlag::DurationToCapture, 5155u16 => CtfGameFlag::HasCarrier,
};
impl Attribute for CtfGameFlag {
    fn class() -> Class {
        Class::CtfGameFlag
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
            Self::AdditionalThreatPerSecond => &Self::AdditionalThreatPerSecond,
            Self::CaptureDistance => &Self::CaptureDistance,
            Self::DurationToCapture => &Self::DurationToCapture,
            Self::HasCarrier => &Self::HasCarrier,
        }
    }
}
impl AttributeInfo for CtfGameFlag {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 5186u16,
            Self::Action0Duration => 5185u16,
            Self::Action0Option => 5196u16,
            Self::AlwaysVisibleToPlayers => 5167u16,
            Self::AutoReviveDelay => 10556u16,
            Self::AutoReviveTime => 10496u16,
            Self::AwareRange => 8275u16,
            Self::BeaconRadius => 10967u16,
            Self::CollisionExtent => 5184u16,
            Self::ContentClass => 5188u16,
            Self::CycleQuestBase => 11054u16,
            Self::DefaultWeapon => 7244u16,
            Self::DespawnDelay => 9666u16,
            Self::Dialogs => 8862u16,
            Self::DisplayName => 6630u16,
            Self::EnableInGame => 6856u16,
            Self::FreedomProperties => 11178u16,
            Self::Freq => 5171u16,
            Self::GenerateInterestList => 5183u16,
            Self::HiddenFromClients => 5182u16,
            Self::HiddenFromPlayers => 5198u16,
            Self::HideAfterInteraction => 9144u16,
            Self::Icon => 5164u16,
            Self::InstanceTags => 5195u16,
            Self::InstanceZoneKey => 5593u16,
            Self::InteractionDuration => 11124u16,
            Self::InteractionRadius => 7505u16,
            Self::InteractionResetTimer => 9146u16,
            Self::IsNonSpawnedAvatar => 5208u16,
            Self::IsSelfRevivable => 7189u16,
            Self::LastInteractionTime => 9145u16,
            Self::LuaScript => 7810u16,
            Self::Lvl => 6213u16,
            Self::MaterialOverride => 5157u16,
            Self::Nodelink => 5197u16,
            Self::OriginalNodeName => 5206u16,
            Self::OriginalZoneName => 5205u16,
            Self::PartyGuid => 5181u16,
            Self::PathfindSafeSpawn => 5199u16,
            Self::Pos => 5180u16,
            Self::Power => 5172u16,
            Self::Priority => 5179u16,
            Self::QuestFlags => 9964u16,
            Self::ReadableName => 5166u16,
            Self::RespawnDelay => 5209u16,
            Self::RespawnRegionName => 10814u16,
            Self::RespawnRegionNameOverride => 10873u16,
            Self::Rot => 5178u16,
            Self::SelfRadius => 5177u16,
            Self::SpawnMethod => 6131u16,
            Self::SpawnPosition => 7865u16,
            Self::SpawnRotation => 8218u16,
            Self::Tags => 5176u16,
            Self::TeamId => 5175u16,
            Self::Ue3ClassId => 5187u16,
            Self::Ue3EdVisual => 9840u16,
            Self::VisibleOnQuestAvailable => 8714u16,
            Self::VisibleOnQuestComplete => 8711u16,
            Self::VisibleOnQuestFinished => 8712u16,
            Self::VisibleOnQuestInProgress => 8713u16,
            Self::WorldZoneObjectIndex => 5207u16,
            Self::Zone => 5173u16,
            Self::ZoneGuid => 5200u16,
            Self::AwareDist => 5203u16,
            Self::Defb => 5190u16,
            Self::InstanceGroup => 11375u16,
            Self::IsUnAttackable => 12435u16,
            Self::Abilities => 9335u16,
            Self::Alive => 5194u16,
            Self::AttackedBy => 5193u16,
            Self::CarrierGuid => 5201u16,
            Self::ClientLoadingPriority => 11277u16,
            Self::DirectorTags => 8092u16,
            Self::ForceSpawnOnClient => 5202u16,
            Self::HpCur => 5192u16,
            Self::HpMax => 5191u16,
            Self::IsLocked => 5489u16,
            Self::SpawnerAvatarGuid => 5974u16,
            Self::SpawnerAvatarId => 7697u16,
            Self::AdditionalThreatPerSecond => 5615u16,
            Self::CaptureDistance => 5156u16,
            Self::DurationToCapture => 5616u16,
            Self::HasCarrier => 5155u16,
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
            Self::AdditionalThreatPerSecond => "additionalThreatPerSecond",
            Self::CaptureDistance => "captureDistance",
            Self::DurationToCapture => "durationToCapture",
            Self::HasCarrier => "hasCarrier",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AwareDist => ParamType::Float,
            Self::AdditionalThreatPerSecond => ParamType::Float,
            Self::CaptureDistance => ParamType::Float,
            Self::DurationToCapture => ParamType::Float,
            Self::HasCarrier => ParamType::Bool,
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
        static AWARE_DIST: Value = Value::Float(500f32);
        static ADDITIONAL_THREAT_PER_SECOND: Value = Value::Float(500f32);
        static CAPTURE_DISTANCE: Value = Value::Float(50f32);
        static DURATION_TO_CAPTURE: Value = Value::Float(15f32);
        static HAS_CARRIER: Value = Value::Bool(false);
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
            Self::AwareDist => &AWARE_DIST,
            Self::AdditionalThreatPerSecond => &ADDITIONAL_THREAT_PER_SECOND,
            Self::CaptureDistance => &CAPTURE_DISTANCE,
            Self::DurationToCapture => &DURATION_TO_CAPTURE,
            Self::HasCarrier => &HAS_CARRIER,
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
            Self::AwareDist => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AdditionalThreatPerSecond => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CaptureDistance => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DurationToCapture => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::HasCarrier => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
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
impl FromStr for CtfGameFlag {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CTF_GAME_FLAG_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CtfGameFlag {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5186u16 => Ok(Self::Action0),
            5185u16 => Ok(Self::Action0Duration),
            5196u16 => Ok(Self::Action0Option),
            5167u16 => Ok(Self::AlwaysVisibleToPlayers),
            10556u16 => Ok(Self::AutoReviveDelay),
            10496u16 => Ok(Self::AutoReviveTime),
            8275u16 => Ok(Self::AwareRange),
            10967u16 => Ok(Self::BeaconRadius),
            5184u16 => Ok(Self::CollisionExtent),
            5188u16 => Ok(Self::ContentClass),
            11054u16 => Ok(Self::CycleQuestBase),
            7244u16 => Ok(Self::DefaultWeapon),
            9666u16 => Ok(Self::DespawnDelay),
            8862u16 => Ok(Self::Dialogs),
            6630u16 => Ok(Self::DisplayName),
            6856u16 => Ok(Self::EnableInGame),
            11178u16 => Ok(Self::FreedomProperties),
            5171u16 => Ok(Self::Freq),
            5183u16 => Ok(Self::GenerateInterestList),
            5182u16 => Ok(Self::HiddenFromClients),
            5198u16 => Ok(Self::HiddenFromPlayers),
            9144u16 => Ok(Self::HideAfterInteraction),
            5164u16 => Ok(Self::Icon),
            5195u16 => Ok(Self::InstanceTags),
            5593u16 => Ok(Self::InstanceZoneKey),
            11124u16 => Ok(Self::InteractionDuration),
            7505u16 => Ok(Self::InteractionRadius),
            9146u16 => Ok(Self::InteractionResetTimer),
            5208u16 => Ok(Self::IsNonSpawnedAvatar),
            7189u16 => Ok(Self::IsSelfRevivable),
            9145u16 => Ok(Self::LastInteractionTime),
            7810u16 => Ok(Self::LuaScript),
            6213u16 => Ok(Self::Lvl),
            5157u16 => Ok(Self::MaterialOverride),
            5197u16 => Ok(Self::Nodelink),
            5206u16 => Ok(Self::OriginalNodeName),
            5205u16 => Ok(Self::OriginalZoneName),
            5181u16 => Ok(Self::PartyGuid),
            5199u16 => Ok(Self::PathfindSafeSpawn),
            5180u16 => Ok(Self::Pos),
            5172u16 => Ok(Self::Power),
            5179u16 => Ok(Self::Priority),
            9964u16 => Ok(Self::QuestFlags),
            5166u16 => Ok(Self::ReadableName),
            5209u16 => Ok(Self::RespawnDelay),
            10814u16 => Ok(Self::RespawnRegionName),
            10873u16 => Ok(Self::RespawnRegionNameOverride),
            5178u16 => Ok(Self::Rot),
            5177u16 => Ok(Self::SelfRadius),
            6131u16 => Ok(Self::SpawnMethod),
            7865u16 => Ok(Self::SpawnPosition),
            8218u16 => Ok(Self::SpawnRotation),
            5176u16 => Ok(Self::Tags),
            5175u16 => Ok(Self::TeamId),
            5187u16 => Ok(Self::Ue3ClassId),
            9840u16 => Ok(Self::Ue3EdVisual),
            8714u16 => Ok(Self::VisibleOnQuestAvailable),
            8711u16 => Ok(Self::VisibleOnQuestComplete),
            8712u16 => Ok(Self::VisibleOnQuestFinished),
            8713u16 => Ok(Self::VisibleOnQuestInProgress),
            5207u16 => Ok(Self::WorldZoneObjectIndex),
            5173u16 => Ok(Self::Zone),
            5200u16 => Ok(Self::ZoneGuid),
            5203u16 => Ok(Self::AwareDist),
            5190u16 => Ok(Self::Defb),
            11375u16 => Ok(Self::InstanceGroup),
            12435u16 => Ok(Self::IsUnAttackable),
            9335u16 => Ok(Self::Abilities),
            5194u16 => Ok(Self::Alive),
            5193u16 => Ok(Self::AttackedBy),
            5201u16 => Ok(Self::CarrierGuid),
            11277u16 => Ok(Self::ClientLoadingPriority),
            8092u16 => Ok(Self::DirectorTags),
            5202u16 => Ok(Self::ForceSpawnOnClient),
            5192u16 => Ok(Self::HpCur),
            5191u16 => Ok(Self::HpMax),
            5489u16 => Ok(Self::IsLocked),
            5974u16 => Ok(Self::SpawnerAvatarGuid),
            7697u16 => Ok(Self::SpawnerAvatarId),
            5615u16 => Ok(Self::AdditionalThreatPerSecond),
            5156u16 => Ok(Self::CaptureDistance),
            5616u16 => Ok(Self::DurationToCapture),
            5155u16 => Ok(Self::HasCarrier),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
