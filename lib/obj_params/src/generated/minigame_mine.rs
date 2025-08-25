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
pub enum MinigameMine {
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
    Damage,
    DetectionRange,
    DetonateRange,
    EffectiveRange,
    HpMin,
    IsDestroyed,
    OwnerId,
    Speed,
}
pub(crate) static MINIGAME_MINE_ATTRIBUTES: phf::Map<&'static str, MinigameMine> = phf_map! {
    "action0" => MinigameMine::Action0, "action0Duration" =>
    MinigameMine::Action0Duration, "action0Option" => MinigameMine::Action0Option,
    "alwaysVisibleToPlayers" => MinigameMine::AlwaysVisibleToPlayers, "autoReviveDelay"
    => MinigameMine::AutoReviveDelay, "autoReviveTime" => MinigameMine::AutoReviveTime,
    "AwareRange" => MinigameMine::AwareRange, "BeaconRadius" =>
    MinigameMine::BeaconRadius, "collisionExtent" => MinigameMine::CollisionExtent,
    "ContentClass" => MinigameMine::ContentClass, "CycleQuestBase" =>
    MinigameMine::CycleQuestBase, "defaultWeapon" => MinigameMine::DefaultWeapon,
    "despawnDelay" => MinigameMine::DespawnDelay, "Dialogs" => MinigameMine::Dialogs,
    "DisplayName" => MinigameMine::DisplayName, "EnableInGame" =>
    MinigameMine::EnableInGame, "FreedomProperties" => MinigameMine::FreedomProperties,
    "Freq" => MinigameMine::Freq, "generateInterestList" =>
    MinigameMine::GenerateInterestList, "hiddenFromClients" =>
    MinigameMine::HiddenFromClients, "hiddenFromPlayers" =>
    MinigameMine::HiddenFromPlayers, "HideAfterInteraction" =>
    MinigameMine::HideAfterInteraction, "Icon" => MinigameMine::Icon, "instanceTags" =>
    MinigameMine::InstanceTags, "instanceZoneKey" => MinigameMine::InstanceZoneKey,
    "InteractionDuration" => MinigameMine::InteractionDuration, "InteractionRadius" =>
    MinigameMine::InteractionRadius, "InteractionResetTimer" =>
    MinigameMine::InteractionResetTimer, "isNonSpawnedAvatar" =>
    MinigameMine::IsNonSpawnedAvatar, "isSelfRevivable" => MinigameMine::IsSelfRevivable,
    "LastInteractionTime" => MinigameMine::LastInteractionTime, "LuaScript" =>
    MinigameMine::LuaScript, "lvl" => MinigameMine::Lvl, "MaterialOverride" =>
    MinigameMine::MaterialOverride, "nodelink" => MinigameMine::Nodelink,
    "originalNodeName" => MinigameMine::OriginalNodeName, "originalZoneName" =>
    MinigameMine::OriginalZoneName, "partyGUID" => MinigameMine::PartyGuid,
    "pathfindSafeSpawn" => MinigameMine::PathfindSafeSpawn, "pos" => MinigameMine::Pos,
    "Power" => MinigameMine::Power, "priority" => MinigameMine::Priority, "QuestFlags" =>
    MinigameMine::QuestFlags, "ReadableName" => MinigameMine::ReadableName,
    "respawnDelay" => MinigameMine::RespawnDelay, "RespawnRegionName" =>
    MinigameMine::RespawnRegionName, "RespawnRegionNameOverride" =>
    MinigameMine::RespawnRegionNameOverride, "rot" => MinigameMine::Rot, "selfRadius" =>
    MinigameMine::SelfRadius, "spawnMethod" => MinigameMine::SpawnMethod, "spawnPosition"
    => MinigameMine::SpawnPosition, "spawnRotation" => MinigameMine::SpawnRotation,
    "tags" => MinigameMine::Tags, "teamID" => MinigameMine::TeamId, "UE3ClassID" =>
    MinigameMine::Ue3ClassId, "UE3EdVisual" => MinigameMine::Ue3EdVisual,
    "VisibleOnQuestAvailable" => MinigameMine::VisibleOnQuestAvailable,
    "VisibleOnQuestComplete" => MinigameMine::VisibleOnQuestComplete,
    "VisibleOnQuestFinished" => MinigameMine::VisibleOnQuestFinished,
    "VisibleOnQuestInProgress" => MinigameMine::VisibleOnQuestInProgress,
    "WorldZoneObjectIndex" => MinigameMine::WorldZoneObjectIndex, "zone" =>
    MinigameMine::Zone, "ZoneGuid" => MinigameMine::ZoneGuid, "awareDist" =>
    MinigameMine::AwareDist, "defb" => MinigameMine::Defb, "instanceGroup" =>
    MinigameMine::InstanceGroup, "isUnAttackable" => MinigameMine::IsUnAttackable,
    "abilities" => MinigameMine::Abilities, "alive" => MinigameMine::Alive, "attackedBy"
    => MinigameMine::AttackedBy, "carrierGuid" => MinigameMine::CarrierGuid,
    "clientLoadingPriority" => MinigameMine::ClientLoadingPriority, "directorTags" =>
    MinigameMine::DirectorTags, "forceSpawnOnClient" => MinigameMine::ForceSpawnOnClient,
    "hpCur" => MinigameMine::HpCur, "hpMax" => MinigameMine::HpMax, "isLocked" =>
    MinigameMine::IsLocked, "spawnerAvatarGuid" => MinigameMine::SpawnerAvatarGuid,
    "spawnerAvatarID" => MinigameMine::SpawnerAvatarId, "damage" => MinigameMine::Damage,
    "detectionRange" => MinigameMine::DetectionRange, "detonateRange" =>
    MinigameMine::DetonateRange, "effectiveRange" => MinigameMine::EffectiveRange,
    "hpMin" => MinigameMine::HpMin, "isDestroyed" => MinigameMine::IsDestroyed, "ownerID"
    => MinigameMine::OwnerId, "speed" => MinigameMine::Speed,
};
pub(crate) static MINIGAME_MINE_ATTRIBUTES_ID: phf::Map<u16, MinigameMine> = phf_map! {
    5058u16 => MinigameMine::Action0, 5057u16 => MinigameMine::Action0Duration, 5068u16
    => MinigameMine::Action0Option, 5039u16 => MinigameMine::AlwaysVisibleToPlayers,
    10554u16 => MinigameMine::AutoReviveDelay, 10494u16 => MinigameMine::AutoReviveTime,
    8273u16 => MinigameMine::AwareRange, 10965u16 => MinigameMine::BeaconRadius, 5056u16
    => MinigameMine::CollisionExtent, 5060u16 => MinigameMine::ContentClass, 11052u16 =>
    MinigameMine::CycleQuestBase, 7242u16 => MinigameMine::DefaultWeapon, 9664u16 =>
    MinigameMine::DespawnDelay, 8860u16 => MinigameMine::Dialogs, 6628u16 =>
    MinigameMine::DisplayName, 6854u16 => MinigameMine::EnableInGame, 11176u16 =>
    MinigameMine::FreedomProperties, 5043u16 => MinigameMine::Freq, 5055u16 =>
    MinigameMine::GenerateInterestList, 5054u16 => MinigameMine::HiddenFromClients,
    5070u16 => MinigameMine::HiddenFromPlayers, 9138u16 =>
    MinigameMine::HideAfterInteraction, 5036u16 => MinigameMine::Icon, 5067u16 =>
    MinigameMine::InstanceTags, 5591u16 => MinigameMine::InstanceZoneKey, 11122u16 =>
    MinigameMine::InteractionDuration, 7503u16 => MinigameMine::InteractionRadius,
    9140u16 => MinigameMine::InteractionResetTimer, 5080u16 =>
    MinigameMine::IsNonSpawnedAvatar, 7187u16 => MinigameMine::IsSelfRevivable, 9139u16
    => MinigameMine::LastInteractionTime, 7808u16 => MinigameMine::LuaScript, 6211u16 =>
    MinigameMine::Lvl, 5029u16 => MinigameMine::MaterialOverride, 5069u16 =>
    MinigameMine::Nodelink, 5078u16 => MinigameMine::OriginalNodeName, 5077u16 =>
    MinigameMine::OriginalZoneName, 5053u16 => MinigameMine::PartyGuid, 5071u16 =>
    MinigameMine::PathfindSafeSpawn, 5052u16 => MinigameMine::Pos, 5044u16 =>
    MinigameMine::Power, 5051u16 => MinigameMine::Priority, 9962u16 =>
    MinigameMine::QuestFlags, 5038u16 => MinigameMine::ReadableName, 5081u16 =>
    MinigameMine::RespawnDelay, 10812u16 => MinigameMine::RespawnRegionName, 10871u16 =>
    MinigameMine::RespawnRegionNameOverride, 5050u16 => MinigameMine::Rot, 5049u16 =>
    MinigameMine::SelfRadius, 6129u16 => MinigameMine::SpawnMethod, 7863u16 =>
    MinigameMine::SpawnPosition, 8216u16 => MinigameMine::SpawnRotation, 5048u16 =>
    MinigameMine::Tags, 5047u16 => MinigameMine::TeamId, 5059u16 =>
    MinigameMine::Ue3ClassId, 9838u16 => MinigameMine::Ue3EdVisual, 8706u16 =>
    MinigameMine::VisibleOnQuestAvailable, 8703u16 =>
    MinigameMine::VisibleOnQuestComplete, 8704u16 =>
    MinigameMine::VisibleOnQuestFinished, 8705u16 =>
    MinigameMine::VisibleOnQuestInProgress, 5079u16 =>
    MinigameMine::WorldZoneObjectIndex, 5045u16 => MinigameMine::Zone, 5072u16 =>
    MinigameMine::ZoneGuid, 5075u16 => MinigameMine::AwareDist, 5062u16 =>
    MinigameMine::Defb, 11373u16 => MinigameMine::InstanceGroup, 12433u16 =>
    MinigameMine::IsUnAttackable, 9333u16 => MinigameMine::Abilities, 5066u16 =>
    MinigameMine::Alive, 5065u16 => MinigameMine::AttackedBy, 5073u16 =>
    MinigameMine::CarrierGuid, 11275u16 => MinigameMine::ClientLoadingPriority, 8090u16
    => MinigameMine::DirectorTags, 5074u16 => MinigameMine::ForceSpawnOnClient, 5064u16
    => MinigameMine::HpCur, 5063u16 => MinigameMine::HpMax, 5487u16 =>
    MinigameMine::IsLocked, 5972u16 => MinigameMine::SpawnerAvatarGuid, 7695u16 =>
    MinigameMine::SpawnerAvatarId, 5406u16 => MinigameMine::Damage, 5409u16 =>
    MinigameMine::DetectionRange, 5408u16 => MinigameMine::DetonateRange, 5407u16 =>
    MinigameMine::EffectiveRange, 5087u16 => MinigameMine::HpMin, 5086u16 =>
    MinigameMine::IsDestroyed, 5084u16 => MinigameMine::OwnerId, 5410u16 =>
    MinigameMine::Speed,
};
impl Attribute for MinigameMine {
    fn class() -> Class {
        Class::MinigameMine
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
            Self::Damage => &Self::Damage,
            Self::DetectionRange => &Self::DetectionRange,
            Self::DetonateRange => &Self::DetonateRange,
            Self::EffectiveRange => &Self::EffectiveRange,
            Self::HpMin => &Self::HpMin,
            Self::IsDestroyed => &Self::IsDestroyed,
            Self::OwnerId => &Self::OwnerId,
            Self::Speed => &Self::Speed,
        }
    }
}
impl AttributeInfo for MinigameMine {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 5058u16,
            Self::Action0Duration => 5057u16,
            Self::Action0Option => 5068u16,
            Self::AlwaysVisibleToPlayers => 5039u16,
            Self::AutoReviveDelay => 10554u16,
            Self::AutoReviveTime => 10494u16,
            Self::AwareRange => 8273u16,
            Self::BeaconRadius => 10965u16,
            Self::CollisionExtent => 5056u16,
            Self::ContentClass => 5060u16,
            Self::CycleQuestBase => 11052u16,
            Self::DefaultWeapon => 7242u16,
            Self::DespawnDelay => 9664u16,
            Self::Dialogs => 8860u16,
            Self::DisplayName => 6628u16,
            Self::EnableInGame => 6854u16,
            Self::FreedomProperties => 11176u16,
            Self::Freq => 5043u16,
            Self::GenerateInterestList => 5055u16,
            Self::HiddenFromClients => 5054u16,
            Self::HiddenFromPlayers => 5070u16,
            Self::HideAfterInteraction => 9138u16,
            Self::Icon => 5036u16,
            Self::InstanceTags => 5067u16,
            Self::InstanceZoneKey => 5591u16,
            Self::InteractionDuration => 11122u16,
            Self::InteractionRadius => 7503u16,
            Self::InteractionResetTimer => 9140u16,
            Self::IsNonSpawnedAvatar => 5080u16,
            Self::IsSelfRevivable => 7187u16,
            Self::LastInteractionTime => 9139u16,
            Self::LuaScript => 7808u16,
            Self::Lvl => 6211u16,
            Self::MaterialOverride => 5029u16,
            Self::Nodelink => 5069u16,
            Self::OriginalNodeName => 5078u16,
            Self::OriginalZoneName => 5077u16,
            Self::PartyGuid => 5053u16,
            Self::PathfindSafeSpawn => 5071u16,
            Self::Pos => 5052u16,
            Self::Power => 5044u16,
            Self::Priority => 5051u16,
            Self::QuestFlags => 9962u16,
            Self::ReadableName => 5038u16,
            Self::RespawnDelay => 5081u16,
            Self::RespawnRegionName => 10812u16,
            Self::RespawnRegionNameOverride => 10871u16,
            Self::Rot => 5050u16,
            Self::SelfRadius => 5049u16,
            Self::SpawnMethod => 6129u16,
            Self::SpawnPosition => 7863u16,
            Self::SpawnRotation => 8216u16,
            Self::Tags => 5048u16,
            Self::TeamId => 5047u16,
            Self::Ue3ClassId => 5059u16,
            Self::Ue3EdVisual => 9838u16,
            Self::VisibleOnQuestAvailable => 8706u16,
            Self::VisibleOnQuestComplete => 8703u16,
            Self::VisibleOnQuestFinished => 8704u16,
            Self::VisibleOnQuestInProgress => 8705u16,
            Self::WorldZoneObjectIndex => 5079u16,
            Self::Zone => 5045u16,
            Self::ZoneGuid => 5072u16,
            Self::AwareDist => 5075u16,
            Self::Defb => 5062u16,
            Self::InstanceGroup => 11373u16,
            Self::IsUnAttackable => 12433u16,
            Self::Abilities => 9333u16,
            Self::Alive => 5066u16,
            Self::AttackedBy => 5065u16,
            Self::CarrierGuid => 5073u16,
            Self::ClientLoadingPriority => 11275u16,
            Self::DirectorTags => 8090u16,
            Self::ForceSpawnOnClient => 5074u16,
            Self::HpCur => 5064u16,
            Self::HpMax => 5063u16,
            Self::IsLocked => 5487u16,
            Self::SpawnerAvatarGuid => 5972u16,
            Self::SpawnerAvatarId => 7695u16,
            Self::Damage => 5406u16,
            Self::DetectionRange => 5409u16,
            Self::DetonateRange => 5408u16,
            Self::EffectiveRange => 5407u16,
            Self::HpMin => 5087u16,
            Self::IsDestroyed => 5086u16,
            Self::OwnerId => 5084u16,
            Self::Speed => 5410u16,
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
            Self::Damage => "damage",
            Self::DetectionRange => "detectionRange",
            Self::DetonateRange => "detonateRange",
            Self::EffectiveRange => "effectiveRange",
            Self::HpMin => "hpMin",
            Self::IsDestroyed => "isDestroyed",
            Self::OwnerId => "ownerID",
            Self::Speed => "speed",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::HpCur => ParamType::Float,
            Self::Damage => ParamType::Float,
            Self::DetectionRange => ParamType::Float,
            Self::DetonateRange => ParamType::Float,
            Self::EffectiveRange => ParamType::Float,
            Self::HpMin => ParamType::Int,
            Self::IsDestroyed => ParamType::Bool,
            Self::OwnerId => ParamType::AvatarId,
            Self::Speed => ParamType::Float,
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
            Self::HpMax => ParamType::Int,
            Self::IsLocked => ParamType::Bool,
            Self::SpawnerAvatarGuid => ParamType::Guid,
            Self::SpawnerAvatarId => ParamType::AvatarId,
        }
    }
    fn default(&self) -> &'static Value {
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
        static HP_CUR: Value = Value::Float(2000f32);
        static DAMAGE: Value = Value::Float(0f32);
        static DETECTION_RANGE: Value = Value::Float(0f32);
        static DETONATE_RANGE: Value = Value::Float(0f32);
        static EFFECTIVE_RANGE: Value = Value::Float(0f32);
        static HP_MIN: Value = Value::Int(10i32);
        static IS_DESTROYED: Value = Value::Bool(false);
        static OWNER_ID: Value = Value::AvatarId(AvatarId::from_u64(0));
        static SPEED: Value = Value::Float(0f32);
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
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::HpCur => &HP_CUR,
            Self::Damage => &DAMAGE,
            Self::DetectionRange => &DETECTION_RANGE,
            Self::DetonateRange => &DETONATE_RANGE,
            Self::EffectiveRange => &EFFECTIVE_RANGE,
            Self::HpMin => &HP_MIN,
            Self::IsDestroyed => &IS_DESTROYED,
            Self::OwnerId => &OWNER_ID,
            Self::Speed => &SPEED,
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
            Self::HpMax => &HP_MAX,
            Self::IsLocked => &IS_LOCKED,
            Self::SpawnerAvatarGuid => &SPAWNER_AVATAR_GUID,
            Self::SpawnerAvatarId => &SPAWNER_AVATAR_ID,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HpCur => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Damage => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DetectionRange => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DetonateRange => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::EffectiveRange => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HpMin => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsDestroyed => &[ParamFlag::NodeOwn],
            Self::OwnerId => &[ParamFlag::NodeOwn],
            Self::Speed => &[ParamFlag::Persistent, ParamFlag::Content],
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
impl FromStr for MinigameMine {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MINIGAME_MINE_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MinigameMine {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5058u16 => Ok(Self::Action0),
            5057u16 => Ok(Self::Action0Duration),
            5068u16 => Ok(Self::Action0Option),
            5039u16 => Ok(Self::AlwaysVisibleToPlayers),
            10554u16 => Ok(Self::AutoReviveDelay),
            10494u16 => Ok(Self::AutoReviveTime),
            8273u16 => Ok(Self::AwareRange),
            10965u16 => Ok(Self::BeaconRadius),
            5056u16 => Ok(Self::CollisionExtent),
            5060u16 => Ok(Self::ContentClass),
            11052u16 => Ok(Self::CycleQuestBase),
            7242u16 => Ok(Self::DefaultWeapon),
            9664u16 => Ok(Self::DespawnDelay),
            8860u16 => Ok(Self::Dialogs),
            6628u16 => Ok(Self::DisplayName),
            6854u16 => Ok(Self::EnableInGame),
            11176u16 => Ok(Self::FreedomProperties),
            5043u16 => Ok(Self::Freq),
            5055u16 => Ok(Self::GenerateInterestList),
            5054u16 => Ok(Self::HiddenFromClients),
            5070u16 => Ok(Self::HiddenFromPlayers),
            9138u16 => Ok(Self::HideAfterInteraction),
            5036u16 => Ok(Self::Icon),
            5067u16 => Ok(Self::InstanceTags),
            5591u16 => Ok(Self::InstanceZoneKey),
            11122u16 => Ok(Self::InteractionDuration),
            7503u16 => Ok(Self::InteractionRadius),
            9140u16 => Ok(Self::InteractionResetTimer),
            5080u16 => Ok(Self::IsNonSpawnedAvatar),
            7187u16 => Ok(Self::IsSelfRevivable),
            9139u16 => Ok(Self::LastInteractionTime),
            7808u16 => Ok(Self::LuaScript),
            6211u16 => Ok(Self::Lvl),
            5029u16 => Ok(Self::MaterialOverride),
            5069u16 => Ok(Self::Nodelink),
            5078u16 => Ok(Self::OriginalNodeName),
            5077u16 => Ok(Self::OriginalZoneName),
            5053u16 => Ok(Self::PartyGuid),
            5071u16 => Ok(Self::PathfindSafeSpawn),
            5052u16 => Ok(Self::Pos),
            5044u16 => Ok(Self::Power),
            5051u16 => Ok(Self::Priority),
            9962u16 => Ok(Self::QuestFlags),
            5038u16 => Ok(Self::ReadableName),
            5081u16 => Ok(Self::RespawnDelay),
            10812u16 => Ok(Self::RespawnRegionName),
            10871u16 => Ok(Self::RespawnRegionNameOverride),
            5050u16 => Ok(Self::Rot),
            5049u16 => Ok(Self::SelfRadius),
            6129u16 => Ok(Self::SpawnMethod),
            7863u16 => Ok(Self::SpawnPosition),
            8216u16 => Ok(Self::SpawnRotation),
            5048u16 => Ok(Self::Tags),
            5047u16 => Ok(Self::TeamId),
            5059u16 => Ok(Self::Ue3ClassId),
            9838u16 => Ok(Self::Ue3EdVisual),
            8706u16 => Ok(Self::VisibleOnQuestAvailable),
            8703u16 => Ok(Self::VisibleOnQuestComplete),
            8704u16 => Ok(Self::VisibleOnQuestFinished),
            8705u16 => Ok(Self::VisibleOnQuestInProgress),
            5079u16 => Ok(Self::WorldZoneObjectIndex),
            5045u16 => Ok(Self::Zone),
            5072u16 => Ok(Self::ZoneGuid),
            5075u16 => Ok(Self::AwareDist),
            5062u16 => Ok(Self::Defb),
            11373u16 => Ok(Self::InstanceGroup),
            12433u16 => Ok(Self::IsUnAttackable),
            9333u16 => Ok(Self::Abilities),
            5066u16 => Ok(Self::Alive),
            5065u16 => Ok(Self::AttackedBy),
            5073u16 => Ok(Self::CarrierGuid),
            11275u16 => Ok(Self::ClientLoadingPriority),
            8090u16 => Ok(Self::DirectorTags),
            5074u16 => Ok(Self::ForceSpawnOnClient),
            5064u16 => Ok(Self::HpCur),
            5063u16 => Ok(Self::HpMax),
            5487u16 => Ok(Self::IsLocked),
            5972u16 => Ok(Self::SpawnerAvatarGuid),
            7695u16 => Ok(Self::SpawnerAvatarId),
            5406u16 => Ok(Self::Damage),
            5409u16 => Ok(Self::DetectionRange),
            5408u16 => Ok(Self::DetonateRange),
            5407u16 => Ok(Self::EffectiveRange),
            5087u16 => Ok(Self::HpMin),
            5086u16 => Ok(Self::IsDestroyed),
            5084u16 => Ok(Self::OwnerId),
            5410u16 => Ok(Self::Speed),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
