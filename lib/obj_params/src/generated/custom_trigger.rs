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
pub enum CustomTrigger {
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
    JsonConfig,
    TriggerRadius,
}
pub(crate) static CUSTOM_TRIGGER_ATTRIBUTES: phf::Map<&'static str, CustomTrigger> = phf_map! {
    "action0" => CustomTrigger::Action0, "action0Duration" =>
    CustomTrigger::Action0Duration, "action0Option" => CustomTrigger::Action0Option,
    "alwaysVisibleToPlayers" => CustomTrigger::AlwaysVisibleToPlayers, "autoReviveDelay"
    => CustomTrigger::AutoReviveDelay, "autoReviveTime" => CustomTrigger::AutoReviveTime,
    "AwareRange" => CustomTrigger::AwareRange, "BeaconRadius" =>
    CustomTrigger::BeaconRadius, "collisionExtent" => CustomTrigger::CollisionExtent,
    "ContentClass" => CustomTrigger::ContentClass, "CycleQuestBase" =>
    CustomTrigger::CycleQuestBase, "defaultWeapon" => CustomTrigger::DefaultWeapon,
    "despawnDelay" => CustomTrigger::DespawnDelay, "Dialogs" => CustomTrigger::Dialogs,
    "DisplayName" => CustomTrigger::DisplayName, "EnableInGame" =>
    CustomTrigger::EnableInGame, "FreedomProperties" => CustomTrigger::FreedomProperties,
    "Freq" => CustomTrigger::Freq, "generateInterestList" =>
    CustomTrigger::GenerateInterestList, "hiddenFromClients" =>
    CustomTrigger::HiddenFromClients, "hiddenFromPlayers" =>
    CustomTrigger::HiddenFromPlayers, "HideAfterInteraction" =>
    CustomTrigger::HideAfterInteraction, "Icon" => CustomTrigger::Icon, "instanceTags" =>
    CustomTrigger::InstanceTags, "instanceZoneKey" => CustomTrigger::InstanceZoneKey,
    "InteractionDuration" => CustomTrigger::InteractionDuration, "InteractionRadius" =>
    CustomTrigger::InteractionRadius, "InteractionResetTimer" =>
    CustomTrigger::InteractionResetTimer, "isNonSpawnedAvatar" =>
    CustomTrigger::IsNonSpawnedAvatar, "isSelfRevivable" =>
    CustomTrigger::IsSelfRevivable, "LastInteractionTime" =>
    CustomTrigger::LastInteractionTime, "LuaScript" => CustomTrigger::LuaScript, "lvl" =>
    CustomTrigger::Lvl, "MaterialOverride" => CustomTrigger::MaterialOverride, "nodelink"
    => CustomTrigger::Nodelink, "originalNodeName" => CustomTrigger::OriginalNodeName,
    "originalZoneName" => CustomTrigger::OriginalZoneName, "partyGUID" =>
    CustomTrigger::PartyGuid, "pathfindSafeSpawn" => CustomTrigger::PathfindSafeSpawn,
    "pos" => CustomTrigger::Pos, "Power" => CustomTrigger::Power, "priority" =>
    CustomTrigger::Priority, "QuestFlags" => CustomTrigger::QuestFlags, "ReadableName" =>
    CustomTrigger::ReadableName, "respawnDelay" => CustomTrigger::RespawnDelay,
    "RespawnRegionName" => CustomTrigger::RespawnRegionName, "RespawnRegionNameOverride"
    => CustomTrigger::RespawnRegionNameOverride, "rot" => CustomTrigger::Rot,
    "selfRadius" => CustomTrigger::SelfRadius, "spawnMethod" =>
    CustomTrigger::SpawnMethod, "spawnPosition" => CustomTrigger::SpawnPosition,
    "spawnRotation" => CustomTrigger::SpawnRotation, "tags" => CustomTrigger::Tags,
    "teamID" => CustomTrigger::TeamId, "UE3ClassID" => CustomTrigger::Ue3ClassId,
    "UE3EdVisual" => CustomTrigger::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    CustomTrigger::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    CustomTrigger::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    CustomTrigger::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    CustomTrigger::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    CustomTrigger::WorldZoneObjectIndex, "zone" => CustomTrigger::Zone, "ZoneGuid" =>
    CustomTrigger::ZoneGuid, "awareDist" => CustomTrigger::AwareDist, "defb" =>
    CustomTrigger::Defb, "instanceGroup" => CustomTrigger::InstanceGroup,
    "isUnAttackable" => CustomTrigger::IsUnAttackable, "abilities" =>
    CustomTrigger::Abilities, "alive" => CustomTrigger::Alive, "attackedBy" =>
    CustomTrigger::AttackedBy, "carrierGuid" => CustomTrigger::CarrierGuid,
    "clientLoadingPriority" => CustomTrigger::ClientLoadingPriority, "directorTags" =>
    CustomTrigger::DirectorTags, "forceSpawnOnClient" =>
    CustomTrigger::ForceSpawnOnClient, "hpCur" => CustomTrigger::HpCur, "hpMax" =>
    CustomTrigger::HpMax, "isLocked" => CustomTrigger::IsLocked, "spawnerAvatarGuid" =>
    CustomTrigger::SpawnerAvatarGuid, "spawnerAvatarID" =>
    CustomTrigger::SpawnerAvatarId, "jsonConfig" => CustomTrigger::JsonConfig,
    "triggerRadius" => CustomTrigger::TriggerRadius,
};
pub(crate) static CUSTOM_TRIGGER_ATTRIBUTES_ID: phf::Map<u16, CustomTrigger> = phf_map! {
    12526u16 => CustomTrigger::Action0, 12525u16 => CustomTrigger::Action0Duration,
    12524u16 => CustomTrigger::Action0Option, 12523u16 =>
    CustomTrigger::AlwaysVisibleToPlayers, 12522u16 => CustomTrigger::AutoReviveDelay,
    12521u16 => CustomTrigger::AutoReviveTime, 12520u16 => CustomTrigger::AwareRange,
    12519u16 => CustomTrigger::BeaconRadius, 12518u16 => CustomTrigger::CollisionExtent,
    12517u16 => CustomTrigger::ContentClass, 12516u16 => CustomTrigger::CycleQuestBase,
    12515u16 => CustomTrigger::DefaultWeapon, 12514u16 => CustomTrigger::DespawnDelay,
    12513u16 => CustomTrigger::Dialogs, 12512u16 => CustomTrigger::DisplayName, 12511u16
    => CustomTrigger::EnableInGame, 12510u16 => CustomTrigger::FreedomProperties,
    12509u16 => CustomTrigger::Freq, 12508u16 => CustomTrigger::GenerateInterestList,
    12507u16 => CustomTrigger::HiddenFromClients, 12506u16 =>
    CustomTrigger::HiddenFromPlayers, 12505u16 => CustomTrigger::HideAfterInteraction,
    12504u16 => CustomTrigger::Icon, 12503u16 => CustomTrigger::InstanceTags, 12502u16 =>
    CustomTrigger::InstanceZoneKey, 12501u16 => CustomTrigger::InteractionDuration,
    12500u16 => CustomTrigger::InteractionRadius, 12499u16 =>
    CustomTrigger::InteractionResetTimer, 12498u16 => CustomTrigger::IsNonSpawnedAvatar,
    12497u16 => CustomTrigger::IsSelfRevivable, 12496u16 =>
    CustomTrigger::LastInteractionTime, 12495u16 => CustomTrigger::LuaScript, 12494u16 =>
    CustomTrigger::Lvl, 12493u16 => CustomTrigger::MaterialOverride, 12492u16 =>
    CustomTrigger::Nodelink, 12491u16 => CustomTrigger::OriginalNodeName, 12490u16 =>
    CustomTrigger::OriginalZoneName, 12489u16 => CustomTrigger::PartyGuid, 12488u16 =>
    CustomTrigger::PathfindSafeSpawn, 12487u16 => CustomTrigger::Pos, 12486u16 =>
    CustomTrigger::Power, 12485u16 => CustomTrigger::Priority, 12484u16 =>
    CustomTrigger::QuestFlags, 12483u16 => CustomTrigger::ReadableName, 12482u16 =>
    CustomTrigger::RespawnDelay, 12481u16 => CustomTrigger::RespawnRegionName, 12480u16
    => CustomTrigger::RespawnRegionNameOverride, 12479u16 => CustomTrigger::Rot, 12478u16
    => CustomTrigger::SelfRadius, 12477u16 => CustomTrigger::SpawnMethod, 12476u16 =>
    CustomTrigger::SpawnPosition, 12475u16 => CustomTrigger::SpawnRotation, 12474u16 =>
    CustomTrigger::Tags, 12473u16 => CustomTrigger::TeamId, 12472u16 =>
    CustomTrigger::Ue3ClassId, 12471u16 => CustomTrigger::Ue3EdVisual, 12470u16 =>
    CustomTrigger::VisibleOnQuestAvailable, 12469u16 =>
    CustomTrigger::VisibleOnQuestComplete, 12468u16 =>
    CustomTrigger::VisibleOnQuestFinished, 12467u16 =>
    CustomTrigger::VisibleOnQuestInProgress, 12466u16 =>
    CustomTrigger::WorldZoneObjectIndex, 12465u16 => CustomTrigger::Zone, 12464u16 =>
    CustomTrigger::ZoneGuid, 12463u16 => CustomTrigger::AwareDist, 12462u16 =>
    CustomTrigger::Defb, 12461u16 => CustomTrigger::InstanceGroup, 12460u16 =>
    CustomTrigger::IsUnAttackable, 12459u16 => CustomTrigger::Abilities, 12458u16 =>
    CustomTrigger::Alive, 12457u16 => CustomTrigger::AttackedBy, 12456u16 =>
    CustomTrigger::CarrierGuid, 12455u16 => CustomTrigger::ClientLoadingPriority,
    12454u16 => CustomTrigger::DirectorTags, 12453u16 =>
    CustomTrigger::ForceSpawnOnClient, 12452u16 => CustomTrigger::HpCur, 12451u16 =>
    CustomTrigger::HpMax, 12450u16 => CustomTrigger::IsLocked, 12449u16 =>
    CustomTrigger::SpawnerAvatarGuid, 12448u16 => CustomTrigger::SpawnerAvatarId,
    12447u16 => CustomTrigger::JsonConfig, 12446u16 => CustomTrigger::TriggerRadius,
};
impl Attribute for CustomTrigger {
    fn class() -> Class {
        Class::CustomTrigger
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
            Self::JsonConfig => &Self::JsonConfig,
            Self::TriggerRadius => &Self::TriggerRadius,
        }
    }
}
impl AttributeInfo for CustomTrigger {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 12526u16,
            Self::Action0Duration => 12525u16,
            Self::Action0Option => 12524u16,
            Self::AlwaysVisibleToPlayers => 12523u16,
            Self::AutoReviveDelay => 12522u16,
            Self::AutoReviveTime => 12521u16,
            Self::AwareRange => 12520u16,
            Self::BeaconRadius => 12519u16,
            Self::CollisionExtent => 12518u16,
            Self::ContentClass => 12517u16,
            Self::CycleQuestBase => 12516u16,
            Self::DefaultWeapon => 12515u16,
            Self::DespawnDelay => 12514u16,
            Self::Dialogs => 12513u16,
            Self::DisplayName => 12512u16,
            Self::EnableInGame => 12511u16,
            Self::FreedomProperties => 12510u16,
            Self::Freq => 12509u16,
            Self::GenerateInterestList => 12508u16,
            Self::HiddenFromClients => 12507u16,
            Self::HiddenFromPlayers => 12506u16,
            Self::HideAfterInteraction => 12505u16,
            Self::Icon => 12504u16,
            Self::InstanceTags => 12503u16,
            Self::InstanceZoneKey => 12502u16,
            Self::InteractionDuration => 12501u16,
            Self::InteractionRadius => 12500u16,
            Self::InteractionResetTimer => 12499u16,
            Self::IsNonSpawnedAvatar => 12498u16,
            Self::IsSelfRevivable => 12497u16,
            Self::LastInteractionTime => 12496u16,
            Self::LuaScript => 12495u16,
            Self::Lvl => 12494u16,
            Self::MaterialOverride => 12493u16,
            Self::Nodelink => 12492u16,
            Self::OriginalNodeName => 12491u16,
            Self::OriginalZoneName => 12490u16,
            Self::PartyGuid => 12489u16,
            Self::PathfindSafeSpawn => 12488u16,
            Self::Pos => 12487u16,
            Self::Power => 12486u16,
            Self::Priority => 12485u16,
            Self::QuestFlags => 12484u16,
            Self::ReadableName => 12483u16,
            Self::RespawnDelay => 12482u16,
            Self::RespawnRegionName => 12481u16,
            Self::RespawnRegionNameOverride => 12480u16,
            Self::Rot => 12479u16,
            Self::SelfRadius => 12478u16,
            Self::SpawnMethod => 12477u16,
            Self::SpawnPosition => 12476u16,
            Self::SpawnRotation => 12475u16,
            Self::Tags => 12474u16,
            Self::TeamId => 12473u16,
            Self::Ue3ClassId => 12472u16,
            Self::Ue3EdVisual => 12471u16,
            Self::VisibleOnQuestAvailable => 12470u16,
            Self::VisibleOnQuestComplete => 12469u16,
            Self::VisibleOnQuestFinished => 12468u16,
            Self::VisibleOnQuestInProgress => 12467u16,
            Self::WorldZoneObjectIndex => 12466u16,
            Self::Zone => 12465u16,
            Self::ZoneGuid => 12464u16,
            Self::AwareDist => 12463u16,
            Self::Defb => 12462u16,
            Self::InstanceGroup => 12461u16,
            Self::IsUnAttackable => 12460u16,
            Self::Abilities => 12459u16,
            Self::Alive => 12458u16,
            Self::AttackedBy => 12457u16,
            Self::CarrierGuid => 12456u16,
            Self::ClientLoadingPriority => 12455u16,
            Self::DirectorTags => 12454u16,
            Self::ForceSpawnOnClient => 12453u16,
            Self::HpCur => 12452u16,
            Self::HpMax => 12451u16,
            Self::IsLocked => 12450u16,
            Self::SpawnerAvatarGuid => 12449u16,
            Self::SpawnerAvatarId => 12448u16,
            Self::JsonConfig => 12447u16,
            Self::TriggerRadius => 12446u16,
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
            Self::JsonConfig => "jsonConfig",
            Self::TriggerRadius => "triggerRadius",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::JsonConfig => ParamType::JsonValue,
            Self::TriggerRadius => ParamType::Float,
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
        static JSON_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static TRIGGER_RADIUS: Value = Value::Float(4f32);
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
            Self::JsonConfig => &JSON_CONFIG,
            Self::TriggerRadius => &TRIGGER_RADIUS,
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
            Self::JsonConfig => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TriggerRadius => {
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
impl FromStr for CustomTrigger {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CUSTOM_TRIGGER_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CustomTrigger {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12526u16 => Ok(Self::Action0),
            12525u16 => Ok(Self::Action0Duration),
            12524u16 => Ok(Self::Action0Option),
            12523u16 => Ok(Self::AlwaysVisibleToPlayers),
            12522u16 => Ok(Self::AutoReviveDelay),
            12521u16 => Ok(Self::AutoReviveTime),
            12520u16 => Ok(Self::AwareRange),
            12519u16 => Ok(Self::BeaconRadius),
            12518u16 => Ok(Self::CollisionExtent),
            12517u16 => Ok(Self::ContentClass),
            12516u16 => Ok(Self::CycleQuestBase),
            12515u16 => Ok(Self::DefaultWeapon),
            12514u16 => Ok(Self::DespawnDelay),
            12513u16 => Ok(Self::Dialogs),
            12512u16 => Ok(Self::DisplayName),
            12511u16 => Ok(Self::EnableInGame),
            12510u16 => Ok(Self::FreedomProperties),
            12509u16 => Ok(Self::Freq),
            12508u16 => Ok(Self::GenerateInterestList),
            12507u16 => Ok(Self::HiddenFromClients),
            12506u16 => Ok(Self::HiddenFromPlayers),
            12505u16 => Ok(Self::HideAfterInteraction),
            12504u16 => Ok(Self::Icon),
            12503u16 => Ok(Self::InstanceTags),
            12502u16 => Ok(Self::InstanceZoneKey),
            12501u16 => Ok(Self::InteractionDuration),
            12500u16 => Ok(Self::InteractionRadius),
            12499u16 => Ok(Self::InteractionResetTimer),
            12498u16 => Ok(Self::IsNonSpawnedAvatar),
            12497u16 => Ok(Self::IsSelfRevivable),
            12496u16 => Ok(Self::LastInteractionTime),
            12495u16 => Ok(Self::LuaScript),
            12494u16 => Ok(Self::Lvl),
            12493u16 => Ok(Self::MaterialOverride),
            12492u16 => Ok(Self::Nodelink),
            12491u16 => Ok(Self::OriginalNodeName),
            12490u16 => Ok(Self::OriginalZoneName),
            12489u16 => Ok(Self::PartyGuid),
            12488u16 => Ok(Self::PathfindSafeSpawn),
            12487u16 => Ok(Self::Pos),
            12486u16 => Ok(Self::Power),
            12485u16 => Ok(Self::Priority),
            12484u16 => Ok(Self::QuestFlags),
            12483u16 => Ok(Self::ReadableName),
            12482u16 => Ok(Self::RespawnDelay),
            12481u16 => Ok(Self::RespawnRegionName),
            12480u16 => Ok(Self::RespawnRegionNameOverride),
            12479u16 => Ok(Self::Rot),
            12478u16 => Ok(Self::SelfRadius),
            12477u16 => Ok(Self::SpawnMethod),
            12476u16 => Ok(Self::SpawnPosition),
            12475u16 => Ok(Self::SpawnRotation),
            12474u16 => Ok(Self::Tags),
            12473u16 => Ok(Self::TeamId),
            12472u16 => Ok(Self::Ue3ClassId),
            12471u16 => Ok(Self::Ue3EdVisual),
            12470u16 => Ok(Self::VisibleOnQuestAvailable),
            12469u16 => Ok(Self::VisibleOnQuestComplete),
            12468u16 => Ok(Self::VisibleOnQuestFinished),
            12467u16 => Ok(Self::VisibleOnQuestInProgress),
            12466u16 => Ok(Self::WorldZoneObjectIndex),
            12465u16 => Ok(Self::Zone),
            12464u16 => Ok(Self::ZoneGuid),
            12463u16 => Ok(Self::AwareDist),
            12462u16 => Ok(Self::Defb),
            12461u16 => Ok(Self::InstanceGroup),
            12460u16 => Ok(Self::IsUnAttackable),
            12459u16 => Ok(Self::Abilities),
            12458u16 => Ok(Self::Alive),
            12457u16 => Ok(Self::AttackedBy),
            12456u16 => Ok(Self::CarrierGuid),
            12455u16 => Ok(Self::ClientLoadingPriority),
            12454u16 => Ok(Self::DirectorTags),
            12453u16 => Ok(Self::ForceSpawnOnClient),
            12452u16 => Ok(Self::HpCur),
            12451u16 => Ok(Self::HpMax),
            12450u16 => Ok(Self::IsLocked),
            12449u16 => Ok(Self::SpawnerAvatarGuid),
            12448u16 => Ok(Self::SpawnerAvatarId),
            12447u16 => Ok(Self::JsonConfig),
            12446u16 => Ok(Self::TriggerRadius),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
