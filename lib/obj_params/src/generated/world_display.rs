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
pub enum WorldDisplay {
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
    DisplayContent,
    DisplayTemplate,
}
pub(crate) static WORLD_DISPLAY_ATTRIBUTES: phf::Map<&'static str, WorldDisplay> = phf_map! {
    "action0" => WorldDisplay::Action0, "action0Duration" =>
    WorldDisplay::Action0Duration, "action0Option" => WorldDisplay::Action0Option,
    "alwaysVisibleToPlayers" => WorldDisplay::AlwaysVisibleToPlayers, "autoReviveDelay"
    => WorldDisplay::AutoReviveDelay, "autoReviveTime" => WorldDisplay::AutoReviveTime,
    "AwareRange" => WorldDisplay::AwareRange, "BeaconRadius" =>
    WorldDisplay::BeaconRadius, "collisionExtent" => WorldDisplay::CollisionExtent,
    "ContentClass" => WorldDisplay::ContentClass, "CycleQuestBase" =>
    WorldDisplay::CycleQuestBase, "defaultWeapon" => WorldDisplay::DefaultWeapon,
    "despawnDelay" => WorldDisplay::DespawnDelay, "Dialogs" => WorldDisplay::Dialogs,
    "DisplayName" => WorldDisplay::DisplayName, "EnableInGame" =>
    WorldDisplay::EnableInGame, "FreedomProperties" => WorldDisplay::FreedomProperties,
    "Freq" => WorldDisplay::Freq, "generateInterestList" =>
    WorldDisplay::GenerateInterestList, "hiddenFromClients" =>
    WorldDisplay::HiddenFromClients, "hiddenFromPlayers" =>
    WorldDisplay::HiddenFromPlayers, "HideAfterInteraction" =>
    WorldDisplay::HideAfterInteraction, "Icon" => WorldDisplay::Icon, "instanceTags" =>
    WorldDisplay::InstanceTags, "instanceZoneKey" => WorldDisplay::InstanceZoneKey,
    "InteractionDuration" => WorldDisplay::InteractionDuration, "InteractionRadius" =>
    WorldDisplay::InteractionRadius, "InteractionResetTimer" =>
    WorldDisplay::InteractionResetTimer, "isNonSpawnedAvatar" =>
    WorldDisplay::IsNonSpawnedAvatar, "isSelfRevivable" => WorldDisplay::IsSelfRevivable,
    "LastInteractionTime" => WorldDisplay::LastInteractionTime, "LuaScript" =>
    WorldDisplay::LuaScript, "lvl" => WorldDisplay::Lvl, "MaterialOverride" =>
    WorldDisplay::MaterialOverride, "nodelink" => WorldDisplay::Nodelink,
    "originalNodeName" => WorldDisplay::OriginalNodeName, "originalZoneName" =>
    WorldDisplay::OriginalZoneName, "partyGUID" => WorldDisplay::PartyGuid,
    "pathfindSafeSpawn" => WorldDisplay::PathfindSafeSpawn, "pos" => WorldDisplay::Pos,
    "Power" => WorldDisplay::Power, "priority" => WorldDisplay::Priority, "QuestFlags" =>
    WorldDisplay::QuestFlags, "ReadableName" => WorldDisplay::ReadableName,
    "respawnDelay" => WorldDisplay::RespawnDelay, "RespawnRegionName" =>
    WorldDisplay::RespawnRegionName, "RespawnRegionNameOverride" =>
    WorldDisplay::RespawnRegionNameOverride, "rot" => WorldDisplay::Rot, "selfRadius" =>
    WorldDisplay::SelfRadius, "spawnMethod" => WorldDisplay::SpawnMethod, "spawnPosition"
    => WorldDisplay::SpawnPosition, "spawnRotation" => WorldDisplay::SpawnRotation,
    "tags" => WorldDisplay::Tags, "teamID" => WorldDisplay::TeamId, "UE3ClassID" =>
    WorldDisplay::Ue3ClassId, "UE3EdVisual" => WorldDisplay::Ue3EdVisual,
    "VisibleOnQuestAvailable" => WorldDisplay::VisibleOnQuestAvailable,
    "VisibleOnQuestComplete" => WorldDisplay::VisibleOnQuestComplete,
    "VisibleOnQuestFinished" => WorldDisplay::VisibleOnQuestFinished,
    "VisibleOnQuestInProgress" => WorldDisplay::VisibleOnQuestInProgress,
    "WorldZoneObjectIndex" => WorldDisplay::WorldZoneObjectIndex, "zone" =>
    WorldDisplay::Zone, "ZoneGuid" => WorldDisplay::ZoneGuid, "awareDist" =>
    WorldDisplay::AwareDist, "defb" => WorldDisplay::Defb, "instanceGroup" =>
    WorldDisplay::InstanceGroup, "isUnAttackable" => WorldDisplay::IsUnAttackable,
    "abilities" => WorldDisplay::Abilities, "alive" => WorldDisplay::Alive, "attackedBy"
    => WorldDisplay::AttackedBy, "carrierGuid" => WorldDisplay::CarrierGuid,
    "clientLoadingPriority" => WorldDisplay::ClientLoadingPriority, "directorTags" =>
    WorldDisplay::DirectorTags, "forceSpawnOnClient" => WorldDisplay::ForceSpawnOnClient,
    "hpCur" => WorldDisplay::HpCur, "hpMax" => WorldDisplay::HpMax, "isLocked" =>
    WorldDisplay::IsLocked, "spawnerAvatarGuid" => WorldDisplay::SpawnerAvatarGuid,
    "spawnerAvatarID" => WorldDisplay::SpawnerAvatarId, "DisplayContent" =>
    WorldDisplay::DisplayContent, "DisplayTemplate" => WorldDisplay::DisplayTemplate,
};
pub(crate) static WORLD_DISPLAY_ATTRIBUTES_ID: phf::Map<u16, WorldDisplay> = phf_map! {
    8152u16 => WorldDisplay::Action0, 8151u16 => WorldDisplay::Action0Duration, 8162u16
    => WorldDisplay::Action0Option, 8135u16 => WorldDisplay::AlwaysVisibleToPlayers,
    10566u16 => WorldDisplay::AutoReviveDelay, 10506u16 => WorldDisplay::AutoReviveTime,
    8285u16 => WorldDisplay::AwareRange, 10978u16 => WorldDisplay::BeaconRadius, 8150u16
    => WorldDisplay::CollisionExtent, 8154u16 => WorldDisplay::ContentClass, 11064u16 =>
    WorldDisplay::CycleQuestBase, 8112u16 => WorldDisplay::DefaultWeapon, 9676u16 =>
    WorldDisplay::DespawnDelay, 8872u16 => WorldDisplay::Dialogs, 8115u16 =>
    WorldDisplay::DisplayName, 8114u16 => WorldDisplay::EnableInGame, 11188u16 =>
    WorldDisplay::FreedomProperties, 8137u16 => WorldDisplay::Freq, 8149u16 =>
    WorldDisplay::GenerateInterestList, 8148u16 => WorldDisplay::HiddenFromClients,
    8164u16 => WorldDisplay::HiddenFromPlayers, 9174u16 =>
    WorldDisplay::HideAfterInteraction, 8132u16 => WorldDisplay::Icon, 8161u16 =>
    WorldDisplay::InstanceTags, 8121u16 => WorldDisplay::InstanceZoneKey, 11134u16 =>
    WorldDisplay::InteractionDuration, 8111u16 => WorldDisplay::InteractionRadius,
    9176u16 => WorldDisplay::InteractionResetTimer, 8174u16 =>
    WorldDisplay::IsNonSpawnedAvatar, 8113u16 => WorldDisplay::IsSelfRevivable, 9175u16
    => WorldDisplay::LastInteractionTime, 8109u16 => WorldDisplay::LuaScript, 8116u16 =>
    WorldDisplay::Lvl, 8125u16 => WorldDisplay::MaterialOverride, 8163u16 =>
    WorldDisplay::Nodelink, 8172u16 => WorldDisplay::OriginalNodeName, 8171u16 =>
    WorldDisplay::OriginalZoneName, 8147u16 => WorldDisplay::PartyGuid, 8165u16 =>
    WorldDisplay::PathfindSafeSpawn, 8146u16 => WorldDisplay::Pos, 8138u16 =>
    WorldDisplay::Power, 8145u16 => WorldDisplay::Priority, 9974u16 =>
    WorldDisplay::QuestFlags, 8134u16 => WorldDisplay::ReadableName, 8175u16 =>
    WorldDisplay::RespawnDelay, 10825u16 => WorldDisplay::RespawnRegionName, 10884u16 =>
    WorldDisplay::RespawnRegionNameOverride, 8144u16 => WorldDisplay::Rot, 8143u16 =>
    WorldDisplay::SelfRadius, 8117u16 => WorldDisplay::SpawnMethod, 8108u16 =>
    WorldDisplay::SpawnPosition, 8228u16 => WorldDisplay::SpawnRotation, 8142u16 =>
    WorldDisplay::Tags, 8141u16 => WorldDisplay::TeamId, 8153u16 =>
    WorldDisplay::Ue3ClassId, 9850u16 => WorldDisplay::Ue3EdVisual, 8754u16 =>
    WorldDisplay::VisibleOnQuestAvailable, 8751u16 =>
    WorldDisplay::VisibleOnQuestComplete, 8752u16 =>
    WorldDisplay::VisibleOnQuestFinished, 8753u16 =>
    WorldDisplay::VisibleOnQuestInProgress, 8173u16 =>
    WorldDisplay::WorldZoneObjectIndex, 8139u16 => WorldDisplay::Zone, 8166u16 =>
    WorldDisplay::ZoneGuid, 8169u16 => WorldDisplay::AwareDist, 8156u16 =>
    WorldDisplay::Defb, 11384u16 => WorldDisplay::InstanceGroup, 12442u16 =>
    WorldDisplay::IsUnAttackable, 9345u16 => WorldDisplay::Abilities, 8160u16 =>
    WorldDisplay::Alive, 8159u16 => WorldDisplay::AttackedBy, 8167u16 =>
    WorldDisplay::CarrierGuid, 11287u16 => WorldDisplay::ClientLoadingPriority, 8107u16
    => WorldDisplay::DirectorTags, 8168u16 => WorldDisplay::ForceSpawnOnClient, 8158u16
    => WorldDisplay::HpCur, 8157u16 => WorldDisplay::HpMax, 8122u16 =>
    WorldDisplay::IsLocked, 8118u16 => WorldDisplay::SpawnerAvatarGuid, 8110u16 =>
    WorldDisplay::SpawnerAvatarId, 8106u16 => WorldDisplay::DisplayContent, 8180u16 =>
    WorldDisplay::DisplayTemplate,
};
impl Attribute for WorldDisplay {
    fn class() -> Class {
        Class::WorldDisplay
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
            Self::DisplayContent => &Self::DisplayContent,
            Self::DisplayTemplate => &Self::DisplayTemplate,
        }
    }
}
impl AttributeInfo for WorldDisplay {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 8152u16,
            Self::Action0Duration => 8151u16,
            Self::Action0Option => 8162u16,
            Self::AlwaysVisibleToPlayers => 8135u16,
            Self::AutoReviveDelay => 10566u16,
            Self::AutoReviveTime => 10506u16,
            Self::AwareRange => 8285u16,
            Self::BeaconRadius => 10978u16,
            Self::CollisionExtent => 8150u16,
            Self::ContentClass => 8154u16,
            Self::CycleQuestBase => 11064u16,
            Self::DefaultWeapon => 8112u16,
            Self::DespawnDelay => 9676u16,
            Self::Dialogs => 8872u16,
            Self::DisplayName => 8115u16,
            Self::EnableInGame => 8114u16,
            Self::FreedomProperties => 11188u16,
            Self::Freq => 8137u16,
            Self::GenerateInterestList => 8149u16,
            Self::HiddenFromClients => 8148u16,
            Self::HiddenFromPlayers => 8164u16,
            Self::HideAfterInteraction => 9174u16,
            Self::Icon => 8132u16,
            Self::InstanceTags => 8161u16,
            Self::InstanceZoneKey => 8121u16,
            Self::InteractionDuration => 11134u16,
            Self::InteractionRadius => 8111u16,
            Self::InteractionResetTimer => 9176u16,
            Self::IsNonSpawnedAvatar => 8174u16,
            Self::IsSelfRevivable => 8113u16,
            Self::LastInteractionTime => 9175u16,
            Self::LuaScript => 8109u16,
            Self::Lvl => 8116u16,
            Self::MaterialOverride => 8125u16,
            Self::Nodelink => 8163u16,
            Self::OriginalNodeName => 8172u16,
            Self::OriginalZoneName => 8171u16,
            Self::PartyGuid => 8147u16,
            Self::PathfindSafeSpawn => 8165u16,
            Self::Pos => 8146u16,
            Self::Power => 8138u16,
            Self::Priority => 8145u16,
            Self::QuestFlags => 9974u16,
            Self::ReadableName => 8134u16,
            Self::RespawnDelay => 8175u16,
            Self::RespawnRegionName => 10825u16,
            Self::RespawnRegionNameOverride => 10884u16,
            Self::Rot => 8144u16,
            Self::SelfRadius => 8143u16,
            Self::SpawnMethod => 8117u16,
            Self::SpawnPosition => 8108u16,
            Self::SpawnRotation => 8228u16,
            Self::Tags => 8142u16,
            Self::TeamId => 8141u16,
            Self::Ue3ClassId => 8153u16,
            Self::Ue3EdVisual => 9850u16,
            Self::VisibleOnQuestAvailable => 8754u16,
            Self::VisibleOnQuestComplete => 8751u16,
            Self::VisibleOnQuestFinished => 8752u16,
            Self::VisibleOnQuestInProgress => 8753u16,
            Self::WorldZoneObjectIndex => 8173u16,
            Self::Zone => 8139u16,
            Self::ZoneGuid => 8166u16,
            Self::AwareDist => 8169u16,
            Self::Defb => 8156u16,
            Self::InstanceGroup => 11384u16,
            Self::IsUnAttackable => 12442u16,
            Self::Abilities => 9345u16,
            Self::Alive => 8160u16,
            Self::AttackedBy => 8159u16,
            Self::CarrierGuid => 8167u16,
            Self::ClientLoadingPriority => 11287u16,
            Self::DirectorTags => 8107u16,
            Self::ForceSpawnOnClient => 8168u16,
            Self::HpCur => 8158u16,
            Self::HpMax => 8157u16,
            Self::IsLocked => 8122u16,
            Self::SpawnerAvatarGuid => 8118u16,
            Self::SpawnerAvatarId => 8110u16,
            Self::DisplayContent => 8106u16,
            Self::DisplayTemplate => 8180u16,
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
            Self::DisplayContent => "DisplayContent",
            Self::DisplayTemplate => "DisplayTemplate",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Ue3ClassId => ParamType::String,
            Self::DisplayContent => ParamType::JsonValue,
            Self::DisplayTemplate => ParamType::String,
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
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLGFxStructureDisplay".to_string(),
        ));
        static DISPLAY_CONTENT: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"dialog\":\"lalala\\nlalala\\nwuwuwlalala\\n\",\"garbage\":\"WOW\"}",
                )
                .unwrap(),
        ));
        static DISPLAY_TEMPLATE: Lazy<Value> = Lazy::new(|| Value::String(
            "UI_GFx.WorldDisplay".to_string(),
        ));
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
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::DisplayContent => &DISPLAY_CONTENT,
            Self::DisplayTemplate => &DISPLAY_TEMPLATE,
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
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DisplayContent => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DisplayTemplate => &[ParamFlag::Persistent, ParamFlag::Content],
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
impl FromStr for WorldDisplay {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WORLD_DISPLAY_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for WorldDisplay {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8152u16 => Ok(Self::Action0),
            8151u16 => Ok(Self::Action0Duration),
            8162u16 => Ok(Self::Action0Option),
            8135u16 => Ok(Self::AlwaysVisibleToPlayers),
            10566u16 => Ok(Self::AutoReviveDelay),
            10506u16 => Ok(Self::AutoReviveTime),
            8285u16 => Ok(Self::AwareRange),
            10978u16 => Ok(Self::BeaconRadius),
            8150u16 => Ok(Self::CollisionExtent),
            8154u16 => Ok(Self::ContentClass),
            11064u16 => Ok(Self::CycleQuestBase),
            8112u16 => Ok(Self::DefaultWeapon),
            9676u16 => Ok(Self::DespawnDelay),
            8872u16 => Ok(Self::Dialogs),
            8115u16 => Ok(Self::DisplayName),
            8114u16 => Ok(Self::EnableInGame),
            11188u16 => Ok(Self::FreedomProperties),
            8137u16 => Ok(Self::Freq),
            8149u16 => Ok(Self::GenerateInterestList),
            8148u16 => Ok(Self::HiddenFromClients),
            8164u16 => Ok(Self::HiddenFromPlayers),
            9174u16 => Ok(Self::HideAfterInteraction),
            8132u16 => Ok(Self::Icon),
            8161u16 => Ok(Self::InstanceTags),
            8121u16 => Ok(Self::InstanceZoneKey),
            11134u16 => Ok(Self::InteractionDuration),
            8111u16 => Ok(Self::InteractionRadius),
            9176u16 => Ok(Self::InteractionResetTimer),
            8174u16 => Ok(Self::IsNonSpawnedAvatar),
            8113u16 => Ok(Self::IsSelfRevivable),
            9175u16 => Ok(Self::LastInteractionTime),
            8109u16 => Ok(Self::LuaScript),
            8116u16 => Ok(Self::Lvl),
            8125u16 => Ok(Self::MaterialOverride),
            8163u16 => Ok(Self::Nodelink),
            8172u16 => Ok(Self::OriginalNodeName),
            8171u16 => Ok(Self::OriginalZoneName),
            8147u16 => Ok(Self::PartyGuid),
            8165u16 => Ok(Self::PathfindSafeSpawn),
            8146u16 => Ok(Self::Pos),
            8138u16 => Ok(Self::Power),
            8145u16 => Ok(Self::Priority),
            9974u16 => Ok(Self::QuestFlags),
            8134u16 => Ok(Self::ReadableName),
            8175u16 => Ok(Self::RespawnDelay),
            10825u16 => Ok(Self::RespawnRegionName),
            10884u16 => Ok(Self::RespawnRegionNameOverride),
            8144u16 => Ok(Self::Rot),
            8143u16 => Ok(Self::SelfRadius),
            8117u16 => Ok(Self::SpawnMethod),
            8108u16 => Ok(Self::SpawnPosition),
            8228u16 => Ok(Self::SpawnRotation),
            8142u16 => Ok(Self::Tags),
            8141u16 => Ok(Self::TeamId),
            8153u16 => Ok(Self::Ue3ClassId),
            9850u16 => Ok(Self::Ue3EdVisual),
            8754u16 => Ok(Self::VisibleOnQuestAvailable),
            8751u16 => Ok(Self::VisibleOnQuestComplete),
            8752u16 => Ok(Self::VisibleOnQuestFinished),
            8753u16 => Ok(Self::VisibleOnQuestInProgress),
            8173u16 => Ok(Self::WorldZoneObjectIndex),
            8139u16 => Ok(Self::Zone),
            8166u16 => Ok(Self::ZoneGuid),
            8169u16 => Ok(Self::AwareDist),
            8156u16 => Ok(Self::Defb),
            11384u16 => Ok(Self::InstanceGroup),
            12442u16 => Ok(Self::IsUnAttackable),
            9345u16 => Ok(Self::Abilities),
            8160u16 => Ok(Self::Alive),
            8159u16 => Ok(Self::AttackedBy),
            8167u16 => Ok(Self::CarrierGuid),
            11287u16 => Ok(Self::ClientLoadingPriority),
            8107u16 => Ok(Self::DirectorTags),
            8168u16 => Ok(Self::ForceSpawnOnClient),
            8158u16 => Ok(Self::HpCur),
            8157u16 => Ok(Self::HpMax),
            8122u16 => Ok(Self::IsLocked),
            8118u16 => Ok(Self::SpawnerAvatarGuid),
            8110u16 => Ok(Self::SpawnerAvatarId),
            8106u16 => Ok(Self::DisplayContent),
            8180u16 => Ok(Self::DisplayTemplate),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
