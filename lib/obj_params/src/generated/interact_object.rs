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
pub enum InteractObject {
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
    InteractRadius,
    IsShardObject,
}
pub(crate) static INTERACT_OBJECT_ATTRIBUTES: phf::Map<&'static str, InteractObject> = phf_map! {
    "action0" => InteractObject::Action0, "action0Duration" =>
    InteractObject::Action0Duration, "action0Option" => InteractObject::Action0Option,
    "alwaysVisibleToPlayers" => InteractObject::AlwaysVisibleToPlayers, "autoReviveDelay"
    => InteractObject::AutoReviveDelay, "autoReviveTime" =>
    InteractObject::AutoReviveTime, "AwareRange" => InteractObject::AwareRange,
    "BeaconRadius" => InteractObject::BeaconRadius, "collisionExtent" =>
    InteractObject::CollisionExtent, "ContentClass" => InteractObject::ContentClass,
    "CycleQuestBase" => InteractObject::CycleQuestBase, "defaultWeapon" =>
    InteractObject::DefaultWeapon, "despawnDelay" => InteractObject::DespawnDelay,
    "Dialogs" => InteractObject::Dialogs, "DisplayName" => InteractObject::DisplayName,
    "EnableInGame" => InteractObject::EnableInGame, "FreedomProperties" =>
    InteractObject::FreedomProperties, "Freq" => InteractObject::Freq,
    "generateInterestList" => InteractObject::GenerateInterestList, "hiddenFromClients"
    => InteractObject::HiddenFromClients, "hiddenFromPlayers" =>
    InteractObject::HiddenFromPlayers, "HideAfterInteraction" =>
    InteractObject::HideAfterInteraction, "Icon" => InteractObject::Icon, "instanceTags"
    => InteractObject::InstanceTags, "instanceZoneKey" =>
    InteractObject::InstanceZoneKey, "InteractionDuration" =>
    InteractObject::InteractionDuration, "InteractionRadius" =>
    InteractObject::InteractionRadius, "InteractionResetTimer" =>
    InteractObject::InteractionResetTimer, "isNonSpawnedAvatar" =>
    InteractObject::IsNonSpawnedAvatar, "isSelfRevivable" =>
    InteractObject::IsSelfRevivable, "LastInteractionTime" =>
    InteractObject::LastInteractionTime, "LuaScript" => InteractObject::LuaScript, "lvl"
    => InteractObject::Lvl, "MaterialOverride" => InteractObject::MaterialOverride,
    "nodelink" => InteractObject::Nodelink, "originalNodeName" =>
    InteractObject::OriginalNodeName, "originalZoneName" =>
    InteractObject::OriginalZoneName, "partyGUID" => InteractObject::PartyGuid,
    "pathfindSafeSpawn" => InteractObject::PathfindSafeSpawn, "pos" =>
    InteractObject::Pos, "Power" => InteractObject::Power, "priority" =>
    InteractObject::Priority, "QuestFlags" => InteractObject::QuestFlags, "ReadableName"
    => InteractObject::ReadableName, "respawnDelay" => InteractObject::RespawnDelay,
    "RespawnRegionName" => InteractObject::RespawnRegionName, "RespawnRegionNameOverride"
    => InteractObject::RespawnRegionNameOverride, "rot" => InteractObject::Rot,
    "selfRadius" => InteractObject::SelfRadius, "spawnMethod" =>
    InteractObject::SpawnMethod, "spawnPosition" => InteractObject::SpawnPosition,
    "spawnRotation" => InteractObject::SpawnRotation, "tags" => InteractObject::Tags,
    "teamID" => InteractObject::TeamId, "UE3ClassID" => InteractObject::Ue3ClassId,
    "UE3EdVisual" => InteractObject::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    InteractObject::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    InteractObject::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    InteractObject::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    InteractObject::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    InteractObject::WorldZoneObjectIndex, "zone" => InteractObject::Zone, "ZoneGuid" =>
    InteractObject::ZoneGuid, "awareDist" => InteractObject::AwareDist, "defb" =>
    InteractObject::Defb, "instanceGroup" => InteractObject::InstanceGroup,
    "isUnAttackable" => InteractObject::IsUnAttackable, "abilities" =>
    InteractObject::Abilities, "alive" => InteractObject::Alive, "attackedBy" =>
    InteractObject::AttackedBy, "carrierGuid" => InteractObject::CarrierGuid,
    "clientLoadingPriority" => InteractObject::ClientLoadingPriority, "directorTags" =>
    InteractObject::DirectorTags, "forceSpawnOnClient" =>
    InteractObject::ForceSpawnOnClient, "hpCur" => InteractObject::HpCur, "hpMax" =>
    InteractObject::HpMax, "isLocked" => InteractObject::IsLocked, "spawnerAvatarGuid" =>
    InteractObject::SpawnerAvatarGuid, "spawnerAvatarID" =>
    InteractObject::SpawnerAvatarId, "interactRadius" => InteractObject::InteractRadius,
    "isShardObject" => InteractObject::IsShardObject,
};
pub(crate) static INTERACT_OBJECT_ATTRIBUTES_ID: phf::Map<u16, InteractObject> = phf_map! {
    2360u16 => InteractObject::Action0, 2359u16 => InteractObject::Action0Duration,
    2370u16 => InteractObject::Action0Option, 3514u16 =>
    InteractObject::AlwaysVisibleToPlayers, 10546u16 => InteractObject::AutoReviveDelay,
    10486u16 => InteractObject::AutoReviveTime, 8265u16 => InteractObject::AwareRange,
    10957u16 => InteractObject::BeaconRadius, 2358u16 => InteractObject::CollisionExtent,
    2362u16 => InteractObject::ContentClass, 11043u16 => InteractObject::CycleQuestBase,
    7232u16 => InteractObject::DefaultWeapon, 9656u16 => InteractObject::DespawnDelay,
    8852u16 => InteractObject::Dialogs, 6618u16 => InteractObject::DisplayName, 6844u16
    => InteractObject::EnableInGame, 11167u16 => InteractObject::FreedomProperties,
    2345u16 => InteractObject::Freq, 2357u16 => InteractObject::GenerateInterestList,
    2356u16 => InteractObject::HiddenFromClients, 2372u16 =>
    InteractObject::HiddenFromPlayers, 9114u16 => InteractObject::HideAfterInteraction,
    4372u16 => InteractObject::Icon, 2369u16 => InteractObject::InstanceTags, 5581u16 =>
    InteractObject::InstanceZoneKey, 11113u16 => InteractObject::InteractionDuration,
    7493u16 => InteractObject::InteractionRadius, 9116u16 =>
    InteractObject::InteractionResetTimer, 2325u16 => InteractObject::IsNonSpawnedAvatar,
    7177u16 => InteractObject::IsSelfRevivable, 9115u16 =>
    InteractObject::LastInteractionTime, 7800u16 => InteractObject::LuaScript, 6201u16 =>
    InteractObject::Lvl, 4751u16 => InteractObject::MaterialOverride, 2371u16 =>
    InteractObject::Nodelink, 2327u16 => InteractObject::OriginalNodeName, 2328u16 =>
    InteractObject::OriginalZoneName, 2355u16 => InteractObject::PartyGuid, 2340u16 =>
    InteractObject::PathfindSafeSpawn, 2354u16 => InteractObject::Pos, 2346u16 =>
    InteractObject::Power, 2353u16 => InteractObject::Priority, 9954u16 =>
    InteractObject::QuestFlags, 3698u16 => InteractObject::ReadableName, 2324u16 =>
    InteractObject::RespawnDelay, 10804u16 => InteractObject::RespawnRegionName, 10863u16
    => InteractObject::RespawnRegionNameOverride, 2352u16 => InteractObject::Rot, 2351u16
    => InteractObject::SelfRadius, 6119u16 => InteractObject::SpawnMethod, 7855u16 =>
    InteractObject::SpawnPosition, 8208u16 => InteractObject::SpawnRotation, 2350u16 =>
    InteractObject::Tags, 2349u16 => InteractObject::TeamId, 2361u16 =>
    InteractObject::Ue3ClassId, 9830u16 => InteractObject::Ue3EdVisual, 8674u16 =>
    InteractObject::VisibleOnQuestAvailable, 8671u16 =>
    InteractObject::VisibleOnQuestComplete, 8672u16 =>
    InteractObject::VisibleOnQuestFinished, 8673u16 =>
    InteractObject::VisibleOnQuestInProgress, 2326u16 =>
    InteractObject::WorldZoneObjectIndex, 2347u16 => InteractObject::Zone, 2333u16 =>
    InteractObject::ZoneGuid, 2330u16 => InteractObject::AwareDist, 2364u16 =>
    InteractObject::Defb, 11364u16 => InteractObject::InstanceGroup, 12424u16 =>
    InteractObject::IsUnAttackable, 9325u16 => InteractObject::Abilities, 2368u16 =>
    InteractObject::Alive, 2367u16 => InteractObject::AttackedBy, 2332u16 =>
    InteractObject::CarrierGuid, 11266u16 => InteractObject::ClientLoadingPriority,
    8082u16 => InteractObject::DirectorTags, 2331u16 =>
    InteractObject::ForceSpawnOnClient, 2366u16 => InteractObject::HpCur, 2365u16 =>
    InteractObject::HpMax, 5477u16 => InteractObject::IsLocked, 5962u16 =>
    InteractObject::SpawnerAvatarGuid, 7687u16 => InteractObject::SpawnerAvatarId,
    2343u16 => InteractObject::InteractRadius, 5941u16 => InteractObject::IsShardObject,
};
impl Attribute for InteractObject {
    fn class() -> Class {
        Class::InteractObject
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
            Self::InteractRadius => &Self::InteractRadius,
            Self::IsShardObject => &Self::IsShardObject,
        }
    }
}
impl AttributeInfo for InteractObject {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 2360u16,
            Self::Action0Duration => 2359u16,
            Self::Action0Option => 2370u16,
            Self::AlwaysVisibleToPlayers => 3514u16,
            Self::AutoReviveDelay => 10546u16,
            Self::AutoReviveTime => 10486u16,
            Self::AwareRange => 8265u16,
            Self::BeaconRadius => 10957u16,
            Self::CollisionExtent => 2358u16,
            Self::ContentClass => 2362u16,
            Self::CycleQuestBase => 11043u16,
            Self::DefaultWeapon => 7232u16,
            Self::DespawnDelay => 9656u16,
            Self::Dialogs => 8852u16,
            Self::DisplayName => 6618u16,
            Self::EnableInGame => 6844u16,
            Self::FreedomProperties => 11167u16,
            Self::Freq => 2345u16,
            Self::GenerateInterestList => 2357u16,
            Self::HiddenFromClients => 2356u16,
            Self::HiddenFromPlayers => 2372u16,
            Self::HideAfterInteraction => 9114u16,
            Self::Icon => 4372u16,
            Self::InstanceTags => 2369u16,
            Self::InstanceZoneKey => 5581u16,
            Self::InteractionDuration => 11113u16,
            Self::InteractionRadius => 7493u16,
            Self::InteractionResetTimer => 9116u16,
            Self::IsNonSpawnedAvatar => 2325u16,
            Self::IsSelfRevivable => 7177u16,
            Self::LastInteractionTime => 9115u16,
            Self::LuaScript => 7800u16,
            Self::Lvl => 6201u16,
            Self::MaterialOverride => 4751u16,
            Self::Nodelink => 2371u16,
            Self::OriginalNodeName => 2327u16,
            Self::OriginalZoneName => 2328u16,
            Self::PartyGuid => 2355u16,
            Self::PathfindSafeSpawn => 2340u16,
            Self::Pos => 2354u16,
            Self::Power => 2346u16,
            Self::Priority => 2353u16,
            Self::QuestFlags => 9954u16,
            Self::ReadableName => 3698u16,
            Self::RespawnDelay => 2324u16,
            Self::RespawnRegionName => 10804u16,
            Self::RespawnRegionNameOverride => 10863u16,
            Self::Rot => 2352u16,
            Self::SelfRadius => 2351u16,
            Self::SpawnMethod => 6119u16,
            Self::SpawnPosition => 7855u16,
            Self::SpawnRotation => 8208u16,
            Self::Tags => 2350u16,
            Self::TeamId => 2349u16,
            Self::Ue3ClassId => 2361u16,
            Self::Ue3EdVisual => 9830u16,
            Self::VisibleOnQuestAvailable => 8674u16,
            Self::VisibleOnQuestComplete => 8671u16,
            Self::VisibleOnQuestFinished => 8672u16,
            Self::VisibleOnQuestInProgress => 8673u16,
            Self::WorldZoneObjectIndex => 2326u16,
            Self::Zone => 2347u16,
            Self::ZoneGuid => 2333u16,
            Self::AwareDist => 2330u16,
            Self::Defb => 2364u16,
            Self::InstanceGroup => 11364u16,
            Self::IsUnAttackable => 12424u16,
            Self::Abilities => 9325u16,
            Self::Alive => 2368u16,
            Self::AttackedBy => 2367u16,
            Self::CarrierGuid => 2332u16,
            Self::ClientLoadingPriority => 11266u16,
            Self::DirectorTags => 8082u16,
            Self::ForceSpawnOnClient => 2331u16,
            Self::HpCur => 2366u16,
            Self::HpMax => 2365u16,
            Self::IsLocked => 5477u16,
            Self::SpawnerAvatarGuid => 5962u16,
            Self::SpawnerAvatarId => 7687u16,
            Self::InteractRadius => 2343u16,
            Self::IsShardObject => 5941u16,
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
            Self::InteractRadius => "interactRadius",
            Self::IsShardObject => "isShardObject",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Defb => ParamType::String,
            Self::InteractRadius => ParamType::Float,
            Self::IsShardObject => ParamType::Bool,
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
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(
            "lootWait perm=1".to_string(),
        ));
        static INTERACT_RADIUS: Value = Value::Float(140f32);
        static IS_SHARD_OBJECT: Value = Value::Bool(false);
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
            Self::Defb => &DEFB,
            Self::InteractRadius => &INTERACT_RADIUS,
            Self::IsShardObject => &IS_SHARD_OBJECT,
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
            Self::Defb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InteractRadius => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsShardObject => {
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
impl FromStr for InteractObject {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        INTERACT_OBJECT_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for InteractObject {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            2360u16 => Ok(Self::Action0),
            2359u16 => Ok(Self::Action0Duration),
            2370u16 => Ok(Self::Action0Option),
            3514u16 => Ok(Self::AlwaysVisibleToPlayers),
            10546u16 => Ok(Self::AutoReviveDelay),
            10486u16 => Ok(Self::AutoReviveTime),
            8265u16 => Ok(Self::AwareRange),
            10957u16 => Ok(Self::BeaconRadius),
            2358u16 => Ok(Self::CollisionExtent),
            2362u16 => Ok(Self::ContentClass),
            11043u16 => Ok(Self::CycleQuestBase),
            7232u16 => Ok(Self::DefaultWeapon),
            9656u16 => Ok(Self::DespawnDelay),
            8852u16 => Ok(Self::Dialogs),
            6618u16 => Ok(Self::DisplayName),
            6844u16 => Ok(Self::EnableInGame),
            11167u16 => Ok(Self::FreedomProperties),
            2345u16 => Ok(Self::Freq),
            2357u16 => Ok(Self::GenerateInterestList),
            2356u16 => Ok(Self::HiddenFromClients),
            2372u16 => Ok(Self::HiddenFromPlayers),
            9114u16 => Ok(Self::HideAfterInteraction),
            4372u16 => Ok(Self::Icon),
            2369u16 => Ok(Self::InstanceTags),
            5581u16 => Ok(Self::InstanceZoneKey),
            11113u16 => Ok(Self::InteractionDuration),
            7493u16 => Ok(Self::InteractionRadius),
            9116u16 => Ok(Self::InteractionResetTimer),
            2325u16 => Ok(Self::IsNonSpawnedAvatar),
            7177u16 => Ok(Self::IsSelfRevivable),
            9115u16 => Ok(Self::LastInteractionTime),
            7800u16 => Ok(Self::LuaScript),
            6201u16 => Ok(Self::Lvl),
            4751u16 => Ok(Self::MaterialOverride),
            2371u16 => Ok(Self::Nodelink),
            2327u16 => Ok(Self::OriginalNodeName),
            2328u16 => Ok(Self::OriginalZoneName),
            2355u16 => Ok(Self::PartyGuid),
            2340u16 => Ok(Self::PathfindSafeSpawn),
            2354u16 => Ok(Self::Pos),
            2346u16 => Ok(Self::Power),
            2353u16 => Ok(Self::Priority),
            9954u16 => Ok(Self::QuestFlags),
            3698u16 => Ok(Self::ReadableName),
            2324u16 => Ok(Self::RespawnDelay),
            10804u16 => Ok(Self::RespawnRegionName),
            10863u16 => Ok(Self::RespawnRegionNameOverride),
            2352u16 => Ok(Self::Rot),
            2351u16 => Ok(Self::SelfRadius),
            6119u16 => Ok(Self::SpawnMethod),
            7855u16 => Ok(Self::SpawnPosition),
            8208u16 => Ok(Self::SpawnRotation),
            2350u16 => Ok(Self::Tags),
            2349u16 => Ok(Self::TeamId),
            2361u16 => Ok(Self::Ue3ClassId),
            9830u16 => Ok(Self::Ue3EdVisual),
            8674u16 => Ok(Self::VisibleOnQuestAvailable),
            8671u16 => Ok(Self::VisibleOnQuestComplete),
            8672u16 => Ok(Self::VisibleOnQuestFinished),
            8673u16 => Ok(Self::VisibleOnQuestInProgress),
            2326u16 => Ok(Self::WorldZoneObjectIndex),
            2347u16 => Ok(Self::Zone),
            2333u16 => Ok(Self::ZoneGuid),
            2330u16 => Ok(Self::AwareDist),
            2364u16 => Ok(Self::Defb),
            11364u16 => Ok(Self::InstanceGroup),
            12424u16 => Ok(Self::IsUnAttackable),
            9325u16 => Ok(Self::Abilities),
            2368u16 => Ok(Self::Alive),
            2367u16 => Ok(Self::AttackedBy),
            2332u16 => Ok(Self::CarrierGuid),
            11266u16 => Ok(Self::ClientLoadingPriority),
            8082u16 => Ok(Self::DirectorTags),
            2331u16 => Ok(Self::ForceSpawnOnClient),
            2366u16 => Ok(Self::HpCur),
            2365u16 => Ok(Self::HpMax),
            5477u16 => Ok(Self::IsLocked),
            5962u16 => Ok(Self::SpawnerAvatarGuid),
            7687u16 => Ok(Self::SpawnerAvatarId),
            2343u16 => Ok(Self::InteractRadius),
            5941u16 => Ok(Self::IsShardObject),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
