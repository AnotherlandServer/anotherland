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
pub enum ChessPiece {
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
    ChessType,
    CurrGrid,
    DestGrid,
    Gauge,
    GridSetting,
    HideArrow,
    IsAttacking,
    IsBattleWon,
    IsWhite,
    MoveDest,
    MoveSpeed,
    NpcCountDamage,
    OpponentCount,
    ShowDestroyEffect,
    StartGrid,
}
pub(crate) static CHESS_PIECE_ATTRIBUTES: phf::Map<&'static str, ChessPiece> = phf_map! {
    "action0" => ChessPiece::Action0, "action0Duration" => ChessPiece::Action0Duration,
    "action0Option" => ChessPiece::Action0Option, "alwaysVisibleToPlayers" =>
    ChessPiece::AlwaysVisibleToPlayers, "autoReviveDelay" => ChessPiece::AutoReviveDelay,
    "autoReviveTime" => ChessPiece::AutoReviveTime, "AwareRange" =>
    ChessPiece::AwareRange, "BeaconRadius" => ChessPiece::BeaconRadius, "collisionExtent"
    => ChessPiece::CollisionExtent, "ContentClass" => ChessPiece::ContentClass,
    "CycleQuestBase" => ChessPiece::CycleQuestBase, "defaultWeapon" =>
    ChessPiece::DefaultWeapon, "despawnDelay" => ChessPiece::DespawnDelay, "Dialogs" =>
    ChessPiece::Dialogs, "DisplayName" => ChessPiece::DisplayName, "EnableInGame" =>
    ChessPiece::EnableInGame, "FreedomProperties" => ChessPiece::FreedomProperties,
    "Freq" => ChessPiece::Freq, "generateInterestList" =>
    ChessPiece::GenerateInterestList, "hiddenFromClients" =>
    ChessPiece::HiddenFromClients, "hiddenFromPlayers" => ChessPiece::HiddenFromPlayers,
    "HideAfterInteraction" => ChessPiece::HideAfterInteraction, "Icon" =>
    ChessPiece::Icon, "instanceTags" => ChessPiece::InstanceTags, "instanceZoneKey" =>
    ChessPiece::InstanceZoneKey, "InteractionDuration" =>
    ChessPiece::InteractionDuration, "InteractionRadius" =>
    ChessPiece::InteractionRadius, "InteractionResetTimer" =>
    ChessPiece::InteractionResetTimer, "isNonSpawnedAvatar" =>
    ChessPiece::IsNonSpawnedAvatar, "isSelfRevivable" => ChessPiece::IsSelfRevivable,
    "LastInteractionTime" => ChessPiece::LastInteractionTime, "LuaScript" =>
    ChessPiece::LuaScript, "lvl" => ChessPiece::Lvl, "MaterialOverride" =>
    ChessPiece::MaterialOverride, "nodelink" => ChessPiece::Nodelink, "originalNodeName"
    => ChessPiece::OriginalNodeName, "originalZoneName" => ChessPiece::OriginalZoneName,
    "partyGUID" => ChessPiece::PartyGuid, "pathfindSafeSpawn" =>
    ChessPiece::PathfindSafeSpawn, "pos" => ChessPiece::Pos, "Power" =>
    ChessPiece::Power, "priority" => ChessPiece::Priority, "QuestFlags" =>
    ChessPiece::QuestFlags, "ReadableName" => ChessPiece::ReadableName, "respawnDelay" =>
    ChessPiece::RespawnDelay, "RespawnRegionName" => ChessPiece::RespawnRegionName,
    "RespawnRegionNameOverride" => ChessPiece::RespawnRegionNameOverride, "rot" =>
    ChessPiece::Rot, "selfRadius" => ChessPiece::SelfRadius, "spawnMethod" =>
    ChessPiece::SpawnMethod, "spawnPosition" => ChessPiece::SpawnPosition,
    "spawnRotation" => ChessPiece::SpawnRotation, "tags" => ChessPiece::Tags, "teamID" =>
    ChessPiece::TeamId, "UE3ClassID" => ChessPiece::Ue3ClassId, "UE3EdVisual" =>
    ChessPiece::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    ChessPiece::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    ChessPiece::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    ChessPiece::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    ChessPiece::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    ChessPiece::WorldZoneObjectIndex, "zone" => ChessPiece::Zone, "ZoneGuid" =>
    ChessPiece::ZoneGuid, "awareDist" => ChessPiece::AwareDist, "defb" =>
    ChessPiece::Defb, "instanceGroup" => ChessPiece::InstanceGroup, "isUnAttackable" =>
    ChessPiece::IsUnAttackable, "abilities" => ChessPiece::Abilities, "alive" =>
    ChessPiece::Alive, "attackedBy" => ChessPiece::AttackedBy, "carrierGuid" =>
    ChessPiece::CarrierGuid, "clientLoadingPriority" =>
    ChessPiece::ClientLoadingPriority, "directorTags" => ChessPiece::DirectorTags,
    "forceSpawnOnClient" => ChessPiece::ForceSpawnOnClient, "hpCur" => ChessPiece::HpCur,
    "hpMax" => ChessPiece::HpMax, "isLocked" => ChessPiece::IsLocked, "spawnerAvatarGuid"
    => ChessPiece::SpawnerAvatarGuid, "spawnerAvatarID" => ChessPiece::SpawnerAvatarId,
    "chessType" => ChessPiece::ChessType, "currGrid" => ChessPiece::CurrGrid, "destGrid"
    => ChessPiece::DestGrid, "gauge" => ChessPiece::Gauge, "gridSetting" =>
    ChessPiece::GridSetting, "hideArrow" => ChessPiece::HideArrow, "isAttacking" =>
    ChessPiece::IsAttacking, "isBattleWon" => ChessPiece::IsBattleWon, "isWhite" =>
    ChessPiece::IsWhite, "moveDest" => ChessPiece::MoveDest, "moveSpeed" =>
    ChessPiece::MoveSpeed, "npcCountDamage" => ChessPiece::NpcCountDamage,
    "opponentCount" => ChessPiece::OpponentCount, "showDestroyEffect" =>
    ChessPiece::ShowDestroyEffect, "startGrid" => ChessPiece::StartGrid,
};
pub(crate) static CHESS_PIECE_ATTRIBUTES_ID: phf::Map<u16, ChessPiece> = phf_map! {
    2088u16 => ChessPiece::Action0, 2089u16 => ChessPiece::Action0Duration, 2065u16 =>
    ChessPiece::Action0Option, 3508u16 => ChessPiece::AlwaysVisibleToPlayers, 10540u16 =>
    ChessPiece::AutoReviveDelay, 10480u16 => ChessPiece::AutoReviveTime, 8259u16 =>
    ChessPiece::AwareRange, 10951u16 => ChessPiece::BeaconRadius, 2090u16 =>
    ChessPiece::CollisionExtent, 2068u16 => ChessPiece::ContentClass, 11037u16 =>
    ChessPiece::CycleQuestBase, 7226u16 => ChessPiece::DefaultWeapon, 9650u16 =>
    ChessPiece::DespawnDelay, 8846u16 => ChessPiece::Dialogs, 6612u16 =>
    ChessPiece::DisplayName, 6838u16 => ChessPiece::EnableInGame, 11161u16 =>
    ChessPiece::FreedomProperties, 2103u16 => ChessPiece::Freq, 2091u16 =>
    ChessPiece::GenerateInterestList, 2092u16 => ChessPiece::HiddenFromClients, 2063u16
    => ChessPiece::HiddenFromPlayers, 9096u16 => ChessPiece::HideAfterInteraction,
    4366u16 => ChessPiece::Icon, 2066u16 => ChessPiece::InstanceTags, 5575u16 =>
    ChessPiece::InstanceZoneKey, 11107u16 => ChessPiece::InteractionDuration, 7487u16 =>
    ChessPiece::InteractionRadius, 9098u16 => ChessPiece::InteractionResetTimer, 2051u16
    => ChessPiece::IsNonSpawnedAvatar, 7171u16 => ChessPiece::IsSelfRevivable, 9097u16 =>
    ChessPiece::LastInteractionTime, 7794u16 => ChessPiece::LuaScript, 6195u16 =>
    ChessPiece::Lvl, 4745u16 => ChessPiece::MaterialOverride, 2064u16 =>
    ChessPiece::Nodelink, 2053u16 => ChessPiece::OriginalNodeName, 2054u16 =>
    ChessPiece::OriginalZoneName, 2093u16 => ChessPiece::PartyGuid, 2061u16 =>
    ChessPiece::PathfindSafeSpawn, 2094u16 => ChessPiece::Pos, 2102u16 =>
    ChessPiece::Power, 2095u16 => ChessPiece::Priority, 9948u16 =>
    ChessPiece::QuestFlags, 3692u16 => ChessPiece::ReadableName, 2050u16 =>
    ChessPiece::RespawnDelay, 10798u16 => ChessPiece::RespawnRegionName, 10857u16 =>
    ChessPiece::RespawnRegionNameOverride, 2096u16 => ChessPiece::Rot, 2097u16 =>
    ChessPiece::SelfRadius, 6113u16 => ChessPiece::SpawnMethod, 7849u16 =>
    ChessPiece::SpawnPosition, 8202u16 => ChessPiece::SpawnRotation, 2098u16 =>
    ChessPiece::Tags, 2099u16 => ChessPiece::TeamId, 2082u16 => ChessPiece::Ue3ClassId,
    9824u16 => ChessPiece::Ue3EdVisual, 8650u16 => ChessPiece::VisibleOnQuestAvailable,
    8647u16 => ChessPiece::VisibleOnQuestComplete, 8648u16 =>
    ChessPiece::VisibleOnQuestFinished, 8649u16 => ChessPiece::VisibleOnQuestInProgress,
    2052u16 => ChessPiece::WorldZoneObjectIndex, 2101u16 => ChessPiece::Zone, 2059u16 =>
    ChessPiece::ZoneGuid, 2056u16 => ChessPiece::AwareDist, 2067u16 => ChessPiece::Defb,
    11358u16 => ChessPiece::InstanceGroup, 12428u16 => ChessPiece::IsUnAttackable,
    9319u16 => ChessPiece::Abilities, 2083u16 => ChessPiece::Alive, 2084u16 =>
    ChessPiece::AttackedBy, 2058u16 => ChessPiece::CarrierGuid, 11260u16 =>
    ChessPiece::ClientLoadingPriority, 8076u16 => ChessPiece::DirectorTags, 2057u16 =>
    ChessPiece::ForceSpawnOnClient, 2085u16 => ChessPiece::HpCur, 2086u16 =>
    ChessPiece::HpMax, 5471u16 => ChessPiece::IsLocked, 5956u16 =>
    ChessPiece::SpawnerAvatarGuid, 7681u16 => ChessPiece::SpawnerAvatarId, 2073u16 =>
    ChessPiece::ChessType, 2069u16 => ChessPiece::CurrGrid, 2070u16 =>
    ChessPiece::DestGrid, 2071u16 => ChessPiece::Gauge, 3659u16 =>
    ChessPiece::GridSetting, 2079u16 => ChessPiece::HideArrow, 4052u16 =>
    ChessPiece::IsAttacking, 4051u16 => ChessPiece::IsBattleWon, 2074u16 =>
    ChessPiece::IsWhite, 2076u16 => ChessPiece::MoveDest, 2077u16 =>
    ChessPiece::MoveSpeed, 4959u16 => ChessPiece::NpcCountDamage, 3770u16 =>
    ChessPiece::OpponentCount, 3655u16 => ChessPiece::ShowDestroyEffect, 2081u16 =>
    ChessPiece::StartGrid,
};
impl Attribute for ChessPiece {
    fn class() -> Class {
        Class::ChessPiece
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
            Self::ChessType => &Self::ChessType,
            Self::CurrGrid => &Self::CurrGrid,
            Self::DestGrid => &Self::DestGrid,
            Self::Gauge => &Self::Gauge,
            Self::GridSetting => &Self::GridSetting,
            Self::HideArrow => &Self::HideArrow,
            Self::IsAttacking => &Self::IsAttacking,
            Self::IsBattleWon => &Self::IsBattleWon,
            Self::IsWhite => &Self::IsWhite,
            Self::MoveDest => &Self::MoveDest,
            Self::MoveSpeed => &Self::MoveSpeed,
            Self::NpcCountDamage => &Self::NpcCountDamage,
            Self::OpponentCount => &Self::OpponentCount,
            Self::ShowDestroyEffect => &Self::ShowDestroyEffect,
            Self::StartGrid => &Self::StartGrid,
        }
    }
}
impl AttributeInfo for ChessPiece {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 2088u16,
            Self::Action0Duration => 2089u16,
            Self::Action0Option => 2065u16,
            Self::AlwaysVisibleToPlayers => 3508u16,
            Self::AutoReviveDelay => 10540u16,
            Self::AutoReviveTime => 10480u16,
            Self::AwareRange => 8259u16,
            Self::BeaconRadius => 10951u16,
            Self::CollisionExtent => 2090u16,
            Self::ContentClass => 2068u16,
            Self::CycleQuestBase => 11037u16,
            Self::DefaultWeapon => 7226u16,
            Self::DespawnDelay => 9650u16,
            Self::Dialogs => 8846u16,
            Self::DisplayName => 6612u16,
            Self::EnableInGame => 6838u16,
            Self::FreedomProperties => 11161u16,
            Self::Freq => 2103u16,
            Self::GenerateInterestList => 2091u16,
            Self::HiddenFromClients => 2092u16,
            Self::HiddenFromPlayers => 2063u16,
            Self::HideAfterInteraction => 9096u16,
            Self::Icon => 4366u16,
            Self::InstanceTags => 2066u16,
            Self::InstanceZoneKey => 5575u16,
            Self::InteractionDuration => 11107u16,
            Self::InteractionRadius => 7487u16,
            Self::InteractionResetTimer => 9098u16,
            Self::IsNonSpawnedAvatar => 2051u16,
            Self::IsSelfRevivable => 7171u16,
            Self::LastInteractionTime => 9097u16,
            Self::LuaScript => 7794u16,
            Self::Lvl => 6195u16,
            Self::MaterialOverride => 4745u16,
            Self::Nodelink => 2064u16,
            Self::OriginalNodeName => 2053u16,
            Self::OriginalZoneName => 2054u16,
            Self::PartyGuid => 2093u16,
            Self::PathfindSafeSpawn => 2061u16,
            Self::Pos => 2094u16,
            Self::Power => 2102u16,
            Self::Priority => 2095u16,
            Self::QuestFlags => 9948u16,
            Self::ReadableName => 3692u16,
            Self::RespawnDelay => 2050u16,
            Self::RespawnRegionName => 10798u16,
            Self::RespawnRegionNameOverride => 10857u16,
            Self::Rot => 2096u16,
            Self::SelfRadius => 2097u16,
            Self::SpawnMethod => 6113u16,
            Self::SpawnPosition => 7849u16,
            Self::SpawnRotation => 8202u16,
            Self::Tags => 2098u16,
            Self::TeamId => 2099u16,
            Self::Ue3ClassId => 2082u16,
            Self::Ue3EdVisual => 9824u16,
            Self::VisibleOnQuestAvailable => 8650u16,
            Self::VisibleOnQuestComplete => 8647u16,
            Self::VisibleOnQuestFinished => 8648u16,
            Self::VisibleOnQuestInProgress => 8649u16,
            Self::WorldZoneObjectIndex => 2052u16,
            Self::Zone => 2101u16,
            Self::ZoneGuid => 2059u16,
            Self::AwareDist => 2056u16,
            Self::Defb => 2067u16,
            Self::InstanceGroup => 11358u16,
            Self::IsUnAttackable => 12428u16,
            Self::Abilities => 9319u16,
            Self::Alive => 2083u16,
            Self::AttackedBy => 2084u16,
            Self::CarrierGuid => 2058u16,
            Self::ClientLoadingPriority => 11260u16,
            Self::DirectorTags => 8076u16,
            Self::ForceSpawnOnClient => 2057u16,
            Self::HpCur => 2085u16,
            Self::HpMax => 2086u16,
            Self::IsLocked => 5471u16,
            Self::SpawnerAvatarGuid => 5956u16,
            Self::SpawnerAvatarId => 7681u16,
            Self::ChessType => 2073u16,
            Self::CurrGrid => 2069u16,
            Self::DestGrid => 2070u16,
            Self::Gauge => 2071u16,
            Self::GridSetting => 3659u16,
            Self::HideArrow => 2079u16,
            Self::IsAttacking => 4052u16,
            Self::IsBattleWon => 4051u16,
            Self::IsWhite => 2074u16,
            Self::MoveDest => 2076u16,
            Self::MoveSpeed => 2077u16,
            Self::NpcCountDamage => 4959u16,
            Self::OpponentCount => 3770u16,
            Self::ShowDestroyEffect => 3655u16,
            Self::StartGrid => 2081u16,
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
            Self::ChessType => "chessType",
            Self::CurrGrid => "currGrid",
            Self::DestGrid => "destGrid",
            Self::Gauge => "gauge",
            Self::GridSetting => "gridSetting",
            Self::HideArrow => "hideArrow",
            Self::IsAttacking => "isAttacking",
            Self::IsBattleWon => "isBattleWon",
            Self::IsWhite => "isWhite",
            Self::MoveDest => "moveDest",
            Self::MoveSpeed => "moveSpeed",
            Self::NpcCountDamage => "npcCountDamage",
            Self::OpponentCount => "opponentCount",
            Self::ShowDestroyEffect => "showDestroyEffect",
            Self::StartGrid => "startGrid",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AlwaysVisibleToPlayers => ParamType::Bool,
            Self::ContentClass => ParamType::String,
            Self::Ue3ClassId => ParamType::String,
            Self::AwareDist => ParamType::Float,
            Self::Defb => ParamType::String,
            Self::ChessType => ParamType::Int,
            Self::CurrGrid => ParamType::String,
            Self::DestGrid => ParamType::String,
            Self::Gauge => ParamType::Float,
            Self::GridSetting => ParamType::JsonValue,
            Self::HideArrow => ParamType::Bool,
            Self::IsAttacking => ParamType::Bool,
            Self::IsBattleWon => ParamType::Bool,
            Self::IsWhite => ParamType::Bool,
            Self::MoveDest => ParamType::Vector3,
            Self::MoveSpeed => ParamType::Float,
            Self::NpcCountDamage => ParamType::Int,
            Self::OpponentCount => ParamType::Int,
            Self::ShowDestroyEffect => ParamType::Bool,
            Self::StartGrid => ParamType::String,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::Action0Option => ParamType::Int,
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
            Self::Ue3EdVisual => ParamType::String,
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
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
        static ALWAYS_VISIBLE_TO_PLAYERS: Value = Value::Bool(true);
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            "8SquaredContentInfos.ChessPieces.ChessPiece".to_string(),
        ));
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLAvatarChessPiece".to_string(),
        ));
        static AWARE_DIST: Value = Value::Float(30000f32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String("ChessPiece".to_string()));
        static CHESS_TYPE: Value = Value::Int(0i32);
        static CURR_GRID: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static DEST_GRID: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static GAUGE: Value = Value::Float(0f32);
        static GRID_SETTING: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"originX\":-64700.00000000000,\"originY\":-65867.00000000000,\"size\":16400.00000000000,\"altitude\":15000.00000000000,\"zoneMaxHeight\":39000.00000000000}",
                )
                .unwrap(),
        ));
        static HIDE_ARROW: Value = Value::Bool(true);
        static IS_ATTACKING: Value = Value::Bool(false);
        static IS_BATTLE_WON: Value = Value::Bool(false);
        static IS_WHITE: Value = Value::Bool(false);
        static MOVE_DEST: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static MOVE_SPEED: Value = Value::Float(2000f32);
        static NPC_COUNT_DAMAGE: Value = Value::Int(0i32);
        static OPPONENT_COUNT: Value = Value::Int(0i32);
        static SHOW_DESTROY_EFFECT: Value = Value::Bool(false);
        static START_GRID: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ACTION_0: Lazy<Value> = Lazy::new(|| Value::StringFloatPair((
            String::default(),
            0.0,
        )));
        static ACTION_0_DURATION: Value = Value::Float(0f32);
        static ACTION_0_OPTION: Value = Value::Int(0i32);
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
            Self::AlwaysVisibleToPlayers => &ALWAYS_VISIBLE_TO_PLAYERS,
            Self::ContentClass => &CONTENT_CLASS,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::AwareDist => &AWARE_DIST,
            Self::Defb => &DEFB,
            Self::ChessType => &CHESS_TYPE,
            Self::CurrGrid => &CURR_GRID,
            Self::DestGrid => &DEST_GRID,
            Self::Gauge => &GAUGE,
            Self::GridSetting => &GRID_SETTING,
            Self::HideArrow => &HIDE_ARROW,
            Self::IsAttacking => &IS_ATTACKING,
            Self::IsBattleWon => &IS_BATTLE_WON,
            Self::IsWhite => &IS_WHITE,
            Self::MoveDest => &MOVE_DEST,
            Self::MoveSpeed => &MOVE_SPEED,
            Self::NpcCountDamage => &NPC_COUNT_DAMAGE,
            Self::OpponentCount => &OPPONENT_COUNT,
            Self::ShowDestroyEffect => &SHOW_DESTROY_EFFECT,
            Self::StartGrid => &START_GRID,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::Action0Option => &ACTION_0_OPTION,
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
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
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
            Self::AlwaysVisibleToPlayers => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::ContentClass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwareDist => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Defb => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::ChessType => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CurrGrid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DestGrid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Gauge => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::GridSetting => &[ParamFlag::Persistent],
            Self::HideArrow => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsAttacking => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsBattleWon => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsWhite => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MoveDest => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::MoveSpeed => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::NpcCountDamage => {
                &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting]
            }
            Self::OpponentCount => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ShowDestroyEffect => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StartGrid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Action0 => &[ParamFlag::NodeOwn],
            Self::Action0Duration => &[ParamFlag::NodeOwn],
            Self::Action0Option => &[ParamFlag::NodeOwn],
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
impl FromStr for ChessPiece {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CHESS_PIECE_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ChessPiece {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            2088u16 => Ok(Self::Action0),
            2089u16 => Ok(Self::Action0Duration),
            2065u16 => Ok(Self::Action0Option),
            3508u16 => Ok(Self::AlwaysVisibleToPlayers),
            10540u16 => Ok(Self::AutoReviveDelay),
            10480u16 => Ok(Self::AutoReviveTime),
            8259u16 => Ok(Self::AwareRange),
            10951u16 => Ok(Self::BeaconRadius),
            2090u16 => Ok(Self::CollisionExtent),
            2068u16 => Ok(Self::ContentClass),
            11037u16 => Ok(Self::CycleQuestBase),
            7226u16 => Ok(Self::DefaultWeapon),
            9650u16 => Ok(Self::DespawnDelay),
            8846u16 => Ok(Self::Dialogs),
            6612u16 => Ok(Self::DisplayName),
            6838u16 => Ok(Self::EnableInGame),
            11161u16 => Ok(Self::FreedomProperties),
            2103u16 => Ok(Self::Freq),
            2091u16 => Ok(Self::GenerateInterestList),
            2092u16 => Ok(Self::HiddenFromClients),
            2063u16 => Ok(Self::HiddenFromPlayers),
            9096u16 => Ok(Self::HideAfterInteraction),
            4366u16 => Ok(Self::Icon),
            2066u16 => Ok(Self::InstanceTags),
            5575u16 => Ok(Self::InstanceZoneKey),
            11107u16 => Ok(Self::InteractionDuration),
            7487u16 => Ok(Self::InteractionRadius),
            9098u16 => Ok(Self::InteractionResetTimer),
            2051u16 => Ok(Self::IsNonSpawnedAvatar),
            7171u16 => Ok(Self::IsSelfRevivable),
            9097u16 => Ok(Self::LastInteractionTime),
            7794u16 => Ok(Self::LuaScript),
            6195u16 => Ok(Self::Lvl),
            4745u16 => Ok(Self::MaterialOverride),
            2064u16 => Ok(Self::Nodelink),
            2053u16 => Ok(Self::OriginalNodeName),
            2054u16 => Ok(Self::OriginalZoneName),
            2093u16 => Ok(Self::PartyGuid),
            2061u16 => Ok(Self::PathfindSafeSpawn),
            2094u16 => Ok(Self::Pos),
            2102u16 => Ok(Self::Power),
            2095u16 => Ok(Self::Priority),
            9948u16 => Ok(Self::QuestFlags),
            3692u16 => Ok(Self::ReadableName),
            2050u16 => Ok(Self::RespawnDelay),
            10798u16 => Ok(Self::RespawnRegionName),
            10857u16 => Ok(Self::RespawnRegionNameOverride),
            2096u16 => Ok(Self::Rot),
            2097u16 => Ok(Self::SelfRadius),
            6113u16 => Ok(Self::SpawnMethod),
            7849u16 => Ok(Self::SpawnPosition),
            8202u16 => Ok(Self::SpawnRotation),
            2098u16 => Ok(Self::Tags),
            2099u16 => Ok(Self::TeamId),
            2082u16 => Ok(Self::Ue3ClassId),
            9824u16 => Ok(Self::Ue3EdVisual),
            8650u16 => Ok(Self::VisibleOnQuestAvailable),
            8647u16 => Ok(Self::VisibleOnQuestComplete),
            8648u16 => Ok(Self::VisibleOnQuestFinished),
            8649u16 => Ok(Self::VisibleOnQuestInProgress),
            2052u16 => Ok(Self::WorldZoneObjectIndex),
            2101u16 => Ok(Self::Zone),
            2059u16 => Ok(Self::ZoneGuid),
            2056u16 => Ok(Self::AwareDist),
            2067u16 => Ok(Self::Defb),
            11358u16 => Ok(Self::InstanceGroup),
            12428u16 => Ok(Self::IsUnAttackable),
            9319u16 => Ok(Self::Abilities),
            2083u16 => Ok(Self::Alive),
            2084u16 => Ok(Self::AttackedBy),
            2058u16 => Ok(Self::CarrierGuid),
            11260u16 => Ok(Self::ClientLoadingPriority),
            8076u16 => Ok(Self::DirectorTags),
            2057u16 => Ok(Self::ForceSpawnOnClient),
            2085u16 => Ok(Self::HpCur),
            2086u16 => Ok(Self::HpMax),
            5471u16 => Ok(Self::IsLocked),
            5956u16 => Ok(Self::SpawnerAvatarGuid),
            7681u16 => Ok(Self::SpawnerAvatarId),
            2073u16 => Ok(Self::ChessType),
            2069u16 => Ok(Self::CurrGrid),
            2070u16 => Ok(Self::DestGrid),
            2071u16 => Ok(Self::Gauge),
            3659u16 => Ok(Self::GridSetting),
            2079u16 => Ok(Self::HideArrow),
            4052u16 => Ok(Self::IsAttacking),
            4051u16 => Ok(Self::IsBattleWon),
            2074u16 => Ok(Self::IsWhite),
            2076u16 => Ok(Self::MoveDest),
            2077u16 => Ok(Self::MoveSpeed),
            4959u16 => Ok(Self::NpcCountDamage),
            3770u16 => Ok(Self::OpponentCount),
            3655u16 => Ok(Self::ShowDestroyEffect),
            2081u16 => Ok(Self::StartGrid),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
