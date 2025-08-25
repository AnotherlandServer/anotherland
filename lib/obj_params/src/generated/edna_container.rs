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
pub enum EdnaContainer {
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
    ContainerConfig,
    EdnaItem,
    InitStatus,
    IsBattlegroundContainer,
    JsonConfig,
    LootTable,
    OnlyAllowLootBy,
    TeamFilter,
}
pub(crate) static EDNA_CONTAINER_ATTRIBUTES: phf::Map<&'static str, EdnaContainer> = phf_map! {
    "action0" => EdnaContainer::Action0, "action0Duration" =>
    EdnaContainer::Action0Duration, "action0Option" => EdnaContainer::Action0Option,
    "alwaysVisibleToPlayers" => EdnaContainer::AlwaysVisibleToPlayers, "autoReviveDelay"
    => EdnaContainer::AutoReviveDelay, "autoReviveTime" => EdnaContainer::AutoReviveTime,
    "AwareRange" => EdnaContainer::AwareRange, "BeaconRadius" =>
    EdnaContainer::BeaconRadius, "collisionExtent" => EdnaContainer::CollisionExtent,
    "ContentClass" => EdnaContainer::ContentClass, "CycleQuestBase" =>
    EdnaContainer::CycleQuestBase, "defaultWeapon" => EdnaContainer::DefaultWeapon,
    "despawnDelay" => EdnaContainer::DespawnDelay, "Dialogs" => EdnaContainer::Dialogs,
    "DisplayName" => EdnaContainer::DisplayName, "EnableInGame" =>
    EdnaContainer::EnableInGame, "FreedomProperties" => EdnaContainer::FreedomProperties,
    "Freq" => EdnaContainer::Freq, "generateInterestList" =>
    EdnaContainer::GenerateInterestList, "hiddenFromClients" =>
    EdnaContainer::HiddenFromClients, "hiddenFromPlayers" =>
    EdnaContainer::HiddenFromPlayers, "HideAfterInteraction" =>
    EdnaContainer::HideAfterInteraction, "Icon" => EdnaContainer::Icon, "instanceTags" =>
    EdnaContainer::InstanceTags, "instanceZoneKey" => EdnaContainer::InstanceZoneKey,
    "InteractionDuration" => EdnaContainer::InteractionDuration, "InteractionRadius" =>
    EdnaContainer::InteractionRadius, "InteractionResetTimer" =>
    EdnaContainer::InteractionResetTimer, "isNonSpawnedAvatar" =>
    EdnaContainer::IsNonSpawnedAvatar, "isSelfRevivable" =>
    EdnaContainer::IsSelfRevivable, "LastInteractionTime" =>
    EdnaContainer::LastInteractionTime, "LuaScript" => EdnaContainer::LuaScript, "lvl" =>
    EdnaContainer::Lvl, "MaterialOverride" => EdnaContainer::MaterialOverride, "nodelink"
    => EdnaContainer::Nodelink, "originalNodeName" => EdnaContainer::OriginalNodeName,
    "originalZoneName" => EdnaContainer::OriginalZoneName, "partyGUID" =>
    EdnaContainer::PartyGuid, "pathfindSafeSpawn" => EdnaContainer::PathfindSafeSpawn,
    "pos" => EdnaContainer::Pos, "Power" => EdnaContainer::Power, "priority" =>
    EdnaContainer::Priority, "QuestFlags" => EdnaContainer::QuestFlags, "ReadableName" =>
    EdnaContainer::ReadableName, "respawnDelay" => EdnaContainer::RespawnDelay,
    "RespawnRegionName" => EdnaContainer::RespawnRegionName, "RespawnRegionNameOverride"
    => EdnaContainer::RespawnRegionNameOverride, "rot" => EdnaContainer::Rot,
    "selfRadius" => EdnaContainer::SelfRadius, "spawnMethod" =>
    EdnaContainer::SpawnMethod, "spawnPosition" => EdnaContainer::SpawnPosition,
    "spawnRotation" => EdnaContainer::SpawnRotation, "tags" => EdnaContainer::Tags,
    "teamID" => EdnaContainer::TeamId, "UE3ClassID" => EdnaContainer::Ue3ClassId,
    "UE3EdVisual" => EdnaContainer::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    EdnaContainer::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    EdnaContainer::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    EdnaContainer::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    EdnaContainer::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    EdnaContainer::WorldZoneObjectIndex, "zone" => EdnaContainer::Zone, "ZoneGuid" =>
    EdnaContainer::ZoneGuid, "awareDist" => EdnaContainer::AwareDist, "defb" =>
    EdnaContainer::Defb, "instanceGroup" => EdnaContainer::InstanceGroup,
    "isUnAttackable" => EdnaContainer::IsUnAttackable, "abilities" =>
    EdnaContainer::Abilities, "alive" => EdnaContainer::Alive, "attackedBy" =>
    EdnaContainer::AttackedBy, "carrierGuid" => EdnaContainer::CarrierGuid,
    "clientLoadingPriority" => EdnaContainer::ClientLoadingPriority, "directorTags" =>
    EdnaContainer::DirectorTags, "forceSpawnOnClient" =>
    EdnaContainer::ForceSpawnOnClient, "hpCur" => EdnaContainer::HpCur, "hpMax" =>
    EdnaContainer::HpMax, "isLocked" => EdnaContainer::IsLocked, "spawnerAvatarGuid" =>
    EdnaContainer::SpawnerAvatarGuid, "spawnerAvatarID" =>
    EdnaContainer::SpawnerAvatarId, "interactRadius" => EdnaContainer::InteractRadius,
    "isShardObject" => EdnaContainer::IsShardObject, "ContainerConfig" =>
    EdnaContainer::ContainerConfig, "ednaItem" => EdnaContainer::EdnaItem, "InitStatus"
    => EdnaContainer::InitStatus, "IsBattlegroundContainer" =>
    EdnaContainer::IsBattlegroundContainer, "jsonConfig" => EdnaContainer::JsonConfig,
    "lootTable" => EdnaContainer::LootTable, "onlyAllowLootBy" =>
    EdnaContainer::OnlyAllowLootBy, "TeamFilter" => EdnaContainer::TeamFilter,
};
pub(crate) static EDNA_CONTAINER_ATTRIBUTES_ID: phf::Map<u16, EdnaContainer> = phf_map! {
    3841u16 => EdnaContainer::Action0, 3842u16 => EdnaContainer::Action0Duration, 3831u16
    => EdnaContainer::Action0Option, 3825u16 => EdnaContainer::AlwaysVisibleToPlayers,
    10547u16 => EdnaContainer::AutoReviveDelay, 10487u16 =>
    EdnaContainer::AutoReviveTime, 8266u16 => EdnaContainer::AwareRange, 10958u16 =>
    EdnaContainer::BeaconRadius, 3843u16 => EdnaContainer::CollisionExtent, 3839u16 =>
    EdnaContainer::ContentClass, 11044u16 => EdnaContainer::CycleQuestBase, 7233u16 =>
    EdnaContainer::DefaultWeapon, 9657u16 => EdnaContainer::DespawnDelay, 8853u16 =>
    EdnaContainer::Dialogs, 6619u16 => EdnaContainer::DisplayName, 6845u16 =>
    EdnaContainer::EnableInGame, 11168u16 => EdnaContainer::FreedomProperties, 3856u16 =>
    EdnaContainer::Freq, 3844u16 => EdnaContainer::GenerateInterestList, 3845u16 =>
    EdnaContainer::HiddenFromClients, 3829u16 => EdnaContainer::HiddenFromPlayers,
    9117u16 => EdnaContainer::HideAfterInteraction, 4373u16 => EdnaContainer::Icon,
    3832u16 => EdnaContainer::InstanceTags, 5582u16 => EdnaContainer::InstanceZoneKey,
    11114u16 => EdnaContainer::InteractionDuration, 7494u16 =>
    EdnaContainer::InteractionRadius, 9119u16 => EdnaContainer::InteractionResetTimer,
    3875u16 => EdnaContainer::IsNonSpawnedAvatar, 7178u16 =>
    EdnaContainer::IsSelfRevivable, 9118u16 => EdnaContainer::LastInteractionTime,
    7801u16 => EdnaContainer::LuaScript, 6202u16 => EdnaContainer::Lvl, 4752u16 =>
    EdnaContainer::MaterialOverride, 3830u16 => EdnaContainer::Nodelink, 3873u16 =>
    EdnaContainer::OriginalNodeName, 3872u16 => EdnaContainer::OriginalZoneName, 3846u16
    => EdnaContainer::PartyGuid, 3861u16 => EdnaContainer::PathfindSafeSpawn, 3847u16 =>
    EdnaContainer::Pos, 3855u16 => EdnaContainer::Power, 3848u16 =>
    EdnaContainer::Priority, 9955u16 => EdnaContainer::QuestFlags, 3824u16 =>
    EdnaContainer::ReadableName, 3876u16 => EdnaContainer::RespawnDelay, 10805u16 =>
    EdnaContainer::RespawnRegionName, 10864u16 =>
    EdnaContainer::RespawnRegionNameOverride, 3849u16 => EdnaContainer::Rot, 3850u16 =>
    EdnaContainer::SelfRadius, 6120u16 => EdnaContainer::SpawnMethod, 7856u16 =>
    EdnaContainer::SpawnPosition, 8209u16 => EdnaContainer::SpawnRotation, 3851u16 =>
    EdnaContainer::Tags, 3852u16 => EdnaContainer::TeamId, 3840u16 =>
    EdnaContainer::Ue3ClassId, 9831u16 => EdnaContainer::Ue3EdVisual, 8678u16 =>
    EdnaContainer::VisibleOnQuestAvailable, 8675u16 =>
    EdnaContainer::VisibleOnQuestComplete, 8676u16 =>
    EdnaContainer::VisibleOnQuestFinished, 8677u16 =>
    EdnaContainer::VisibleOnQuestInProgress, 3874u16 =>
    EdnaContainer::WorldZoneObjectIndex, 3854u16 => EdnaContainer::Zone, 3867u16 =>
    EdnaContainer::ZoneGuid, 3870u16 => EdnaContainer::AwareDist, 3837u16 =>
    EdnaContainer::Defb, 11365u16 => EdnaContainer::InstanceGroup, 12425u16 =>
    EdnaContainer::IsUnAttackable, 9326u16 => EdnaContainer::Abilities, 3833u16 =>
    EdnaContainer::Alive, 3834u16 => EdnaContainer::AttackedBy, 3868u16 =>
    EdnaContainer::CarrierGuid, 11267u16 => EdnaContainer::ClientLoadingPriority, 8083u16
    => EdnaContainer::DirectorTags, 3869u16 => EdnaContainer::ForceSpawnOnClient, 3835u16
    => EdnaContainer::HpCur, 3836u16 => EdnaContainer::HpMax, 5478u16 =>
    EdnaContainer::IsLocked, 5963u16 => EdnaContainer::SpawnerAvatarGuid, 7688u16 =>
    EdnaContainer::SpawnerAvatarId, 3858u16 => EdnaContainer::InteractRadius, 5942u16 =>
    EdnaContainer::IsShardObject, 12108u16 => EdnaContainer::ContainerConfig, 3879u16 =>
    EdnaContainer::EdnaItem, 12109u16 => EdnaContainer::InitStatus, 12107u16 =>
    EdnaContainer::IsBattlegroundContainer, 12295u16 => EdnaContainer::JsonConfig,
    8032u16 => EdnaContainer::LootTable, 11003u16 => EdnaContainer::OnlyAllowLootBy,
    12387u16 => EdnaContainer::TeamFilter,
};
impl Attribute for EdnaContainer {
    fn class() -> Class {
        Class::EdnaContainer
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
            Self::ContainerConfig => &Self::ContainerConfig,
            Self::EdnaItem => &Self::EdnaItem,
            Self::InitStatus => &Self::InitStatus,
            Self::IsBattlegroundContainer => &Self::IsBattlegroundContainer,
            Self::JsonConfig => &Self::JsonConfig,
            Self::LootTable => &Self::LootTable,
            Self::OnlyAllowLootBy => &Self::OnlyAllowLootBy,
            Self::TeamFilter => &Self::TeamFilter,
        }
    }
}
impl AttributeInfo for EdnaContainer {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 3841u16,
            Self::Action0Duration => 3842u16,
            Self::Action0Option => 3831u16,
            Self::AlwaysVisibleToPlayers => 3825u16,
            Self::AutoReviveDelay => 10547u16,
            Self::AutoReviveTime => 10487u16,
            Self::AwareRange => 8266u16,
            Self::BeaconRadius => 10958u16,
            Self::CollisionExtent => 3843u16,
            Self::ContentClass => 3839u16,
            Self::CycleQuestBase => 11044u16,
            Self::DefaultWeapon => 7233u16,
            Self::DespawnDelay => 9657u16,
            Self::Dialogs => 8853u16,
            Self::DisplayName => 6619u16,
            Self::EnableInGame => 6845u16,
            Self::FreedomProperties => 11168u16,
            Self::Freq => 3856u16,
            Self::GenerateInterestList => 3844u16,
            Self::HiddenFromClients => 3845u16,
            Self::HiddenFromPlayers => 3829u16,
            Self::HideAfterInteraction => 9117u16,
            Self::Icon => 4373u16,
            Self::InstanceTags => 3832u16,
            Self::InstanceZoneKey => 5582u16,
            Self::InteractionDuration => 11114u16,
            Self::InteractionRadius => 7494u16,
            Self::InteractionResetTimer => 9119u16,
            Self::IsNonSpawnedAvatar => 3875u16,
            Self::IsSelfRevivable => 7178u16,
            Self::LastInteractionTime => 9118u16,
            Self::LuaScript => 7801u16,
            Self::Lvl => 6202u16,
            Self::MaterialOverride => 4752u16,
            Self::Nodelink => 3830u16,
            Self::OriginalNodeName => 3873u16,
            Self::OriginalZoneName => 3872u16,
            Self::PartyGuid => 3846u16,
            Self::PathfindSafeSpawn => 3861u16,
            Self::Pos => 3847u16,
            Self::Power => 3855u16,
            Self::Priority => 3848u16,
            Self::QuestFlags => 9955u16,
            Self::ReadableName => 3824u16,
            Self::RespawnDelay => 3876u16,
            Self::RespawnRegionName => 10805u16,
            Self::RespawnRegionNameOverride => 10864u16,
            Self::Rot => 3849u16,
            Self::SelfRadius => 3850u16,
            Self::SpawnMethod => 6120u16,
            Self::SpawnPosition => 7856u16,
            Self::SpawnRotation => 8209u16,
            Self::Tags => 3851u16,
            Self::TeamId => 3852u16,
            Self::Ue3ClassId => 3840u16,
            Self::Ue3EdVisual => 9831u16,
            Self::VisibleOnQuestAvailable => 8678u16,
            Self::VisibleOnQuestComplete => 8675u16,
            Self::VisibleOnQuestFinished => 8676u16,
            Self::VisibleOnQuestInProgress => 8677u16,
            Self::WorldZoneObjectIndex => 3874u16,
            Self::Zone => 3854u16,
            Self::ZoneGuid => 3867u16,
            Self::AwareDist => 3870u16,
            Self::Defb => 3837u16,
            Self::InstanceGroup => 11365u16,
            Self::IsUnAttackable => 12425u16,
            Self::Abilities => 9326u16,
            Self::Alive => 3833u16,
            Self::AttackedBy => 3834u16,
            Self::CarrierGuid => 3868u16,
            Self::ClientLoadingPriority => 11267u16,
            Self::DirectorTags => 8083u16,
            Self::ForceSpawnOnClient => 3869u16,
            Self::HpCur => 3835u16,
            Self::HpMax => 3836u16,
            Self::IsLocked => 5478u16,
            Self::SpawnerAvatarGuid => 5963u16,
            Self::SpawnerAvatarId => 7688u16,
            Self::InteractRadius => 3858u16,
            Self::IsShardObject => 5942u16,
            Self::ContainerConfig => 12108u16,
            Self::EdnaItem => 3879u16,
            Self::InitStatus => 12109u16,
            Self::IsBattlegroundContainer => 12107u16,
            Self::JsonConfig => 12295u16,
            Self::LootTable => 8032u16,
            Self::OnlyAllowLootBy => 11003u16,
            Self::TeamFilter => 12387u16,
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
            Self::ContainerConfig => "ContainerConfig",
            Self::EdnaItem => "ednaItem",
            Self::InitStatus => "InitStatus",
            Self::IsBattlegroundContainer => "IsBattlegroundContainer",
            Self::JsonConfig => "jsonConfig",
            Self::LootTable => "lootTable",
            Self::OnlyAllowLootBy => "onlyAllowLootBy",
            Self::TeamFilter => "TeamFilter",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::ContentClass => ParamType::String,
            Self::Defb => ParamType::String,
            Self::ContainerConfig => ParamType::JsonValue,
            Self::EdnaItem => ParamType::Bool,
            Self::InitStatus => ParamType::String,
            Self::IsBattlegroundContainer => ParamType::Bool,
            Self::JsonConfig => ParamType::JsonValue,
            Self::LootTable => ParamType::JsonValue,
            Self::OnlyAllowLootBy => ParamType::AvatarId,
            Self::TeamFilter => ParamType::String,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::Action0Option => ParamType::Int,
            Self::AlwaysVisibleToPlayers => ParamType::Bool,
            Self::AutoReviveDelay => ParamType::Float,
            Self::AutoReviveTime => ParamType::Int64,
            Self::AwareRange => ParamType::Float,
            Self::BeaconRadius => ParamType::Int,
            Self::CollisionExtent => ParamType::Vector3,
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
            Self::InteractRadius => ParamType::Float,
            Self::IsShardObject => ParamType::Bool,
        }
    }
    fn default(&self) -> &'static Value {
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(
            "EDNAContainer".to_string(),
        ));
        static CONTAINER_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static EDNA_ITEM: Value = Value::Bool(false);
        static INIT_STATUS: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static IS_BATTLEGROUND_CONTAINER: Value = Value::Bool(false);
        static JSON_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static LOOT_TABLE: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static ONLY_ALLOW_LOOT_BY: Value = Value::AvatarId(AvatarId::from_u64(0));
        static TEAM_FILTER: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
        static INTERACT_RADIUS: Value = Value::Float(140f32);
        static IS_SHARD_OBJECT: Value = Value::Bool(false);
        match self {
            Self::ContentClass => &CONTENT_CLASS,
            Self::Defb => &DEFB,
            Self::ContainerConfig => &CONTAINER_CONFIG,
            Self::EdnaItem => &EDNA_ITEM,
            Self::InitStatus => &INIT_STATUS,
            Self::IsBattlegroundContainer => &IS_BATTLEGROUND_CONTAINER,
            Self::JsonConfig => &JSON_CONFIG,
            Self::LootTable => &LOOT_TABLE,
            Self::OnlyAllowLootBy => &ONLY_ALLOW_LOOT_BY,
            Self::TeamFilter => &TEAM_FILTER,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::Action0Option => &ACTION_0_OPTION,
            Self::AlwaysVisibleToPlayers => &ALWAYS_VISIBLE_TO_PLAYERS,
            Self::AutoReviveDelay => &AUTO_REVIVE_DELAY,
            Self::AutoReviveTime => &AUTO_REVIVE_TIME,
            Self::AwareRange => &AWARE_RANGE,
            Self::BeaconRadius => &BEACON_RADIUS,
            Self::CollisionExtent => &COLLISION_EXTENT,
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
            Self::InteractRadius => &INTERACT_RADIUS,
            Self::IsShardObject => &IS_SHARD_OBJECT,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::ContentClass => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Defb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ContainerConfig => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EdnaItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InitStatus => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsBattlegroundContainer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::JsonConfig => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::LootTable => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::OnlyAllowLootBy => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::TeamFilter => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
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
            Self::InteractRadius => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsShardObject => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
        }
    }
}
impl FromStr for EdnaContainer {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EDNA_CONTAINER_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for EdnaContainer {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            3841u16 => Ok(Self::Action0),
            3842u16 => Ok(Self::Action0Duration),
            3831u16 => Ok(Self::Action0Option),
            3825u16 => Ok(Self::AlwaysVisibleToPlayers),
            10547u16 => Ok(Self::AutoReviveDelay),
            10487u16 => Ok(Self::AutoReviveTime),
            8266u16 => Ok(Self::AwareRange),
            10958u16 => Ok(Self::BeaconRadius),
            3843u16 => Ok(Self::CollisionExtent),
            3839u16 => Ok(Self::ContentClass),
            11044u16 => Ok(Self::CycleQuestBase),
            7233u16 => Ok(Self::DefaultWeapon),
            9657u16 => Ok(Self::DespawnDelay),
            8853u16 => Ok(Self::Dialogs),
            6619u16 => Ok(Self::DisplayName),
            6845u16 => Ok(Self::EnableInGame),
            11168u16 => Ok(Self::FreedomProperties),
            3856u16 => Ok(Self::Freq),
            3844u16 => Ok(Self::GenerateInterestList),
            3845u16 => Ok(Self::HiddenFromClients),
            3829u16 => Ok(Self::HiddenFromPlayers),
            9117u16 => Ok(Self::HideAfterInteraction),
            4373u16 => Ok(Self::Icon),
            3832u16 => Ok(Self::InstanceTags),
            5582u16 => Ok(Self::InstanceZoneKey),
            11114u16 => Ok(Self::InteractionDuration),
            7494u16 => Ok(Self::InteractionRadius),
            9119u16 => Ok(Self::InteractionResetTimer),
            3875u16 => Ok(Self::IsNonSpawnedAvatar),
            7178u16 => Ok(Self::IsSelfRevivable),
            9118u16 => Ok(Self::LastInteractionTime),
            7801u16 => Ok(Self::LuaScript),
            6202u16 => Ok(Self::Lvl),
            4752u16 => Ok(Self::MaterialOverride),
            3830u16 => Ok(Self::Nodelink),
            3873u16 => Ok(Self::OriginalNodeName),
            3872u16 => Ok(Self::OriginalZoneName),
            3846u16 => Ok(Self::PartyGuid),
            3861u16 => Ok(Self::PathfindSafeSpawn),
            3847u16 => Ok(Self::Pos),
            3855u16 => Ok(Self::Power),
            3848u16 => Ok(Self::Priority),
            9955u16 => Ok(Self::QuestFlags),
            3824u16 => Ok(Self::ReadableName),
            3876u16 => Ok(Self::RespawnDelay),
            10805u16 => Ok(Self::RespawnRegionName),
            10864u16 => Ok(Self::RespawnRegionNameOverride),
            3849u16 => Ok(Self::Rot),
            3850u16 => Ok(Self::SelfRadius),
            6120u16 => Ok(Self::SpawnMethod),
            7856u16 => Ok(Self::SpawnPosition),
            8209u16 => Ok(Self::SpawnRotation),
            3851u16 => Ok(Self::Tags),
            3852u16 => Ok(Self::TeamId),
            3840u16 => Ok(Self::Ue3ClassId),
            9831u16 => Ok(Self::Ue3EdVisual),
            8678u16 => Ok(Self::VisibleOnQuestAvailable),
            8675u16 => Ok(Self::VisibleOnQuestComplete),
            8676u16 => Ok(Self::VisibleOnQuestFinished),
            8677u16 => Ok(Self::VisibleOnQuestInProgress),
            3874u16 => Ok(Self::WorldZoneObjectIndex),
            3854u16 => Ok(Self::Zone),
            3867u16 => Ok(Self::ZoneGuid),
            3870u16 => Ok(Self::AwareDist),
            3837u16 => Ok(Self::Defb),
            11365u16 => Ok(Self::InstanceGroup),
            12425u16 => Ok(Self::IsUnAttackable),
            9326u16 => Ok(Self::Abilities),
            3833u16 => Ok(Self::Alive),
            3834u16 => Ok(Self::AttackedBy),
            3868u16 => Ok(Self::CarrierGuid),
            11267u16 => Ok(Self::ClientLoadingPriority),
            8083u16 => Ok(Self::DirectorTags),
            3869u16 => Ok(Self::ForceSpawnOnClient),
            3835u16 => Ok(Self::HpCur),
            3836u16 => Ok(Self::HpMax),
            5478u16 => Ok(Self::IsLocked),
            5963u16 => Ok(Self::SpawnerAvatarGuid),
            7688u16 => Ok(Self::SpawnerAvatarId),
            3858u16 => Ok(Self::InteractRadius),
            5942u16 => Ok(Self::IsShardObject),
            12108u16 => Ok(Self::ContainerConfig),
            3879u16 => Ok(Self::EdnaItem),
            12109u16 => Ok(Self::InitStatus),
            12107u16 => Ok(Self::IsBattlegroundContainer),
            12295u16 => Ok(Self::JsonConfig),
            8032u16 => Ok(Self::LootTable),
            11003u16 => Ok(Self::OnlyAllowLootBy),
            12387u16 => Ok(Self::TeamFilter),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
