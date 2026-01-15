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
pub enum MinigameInfo {
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
    AdaptiveSpawnerBasePlayers,
    AdaptiveSpawnerMax,
    AdaptiveSpawnerMin,
    AwardBaseCredit,
    AwardBaseLosingTeamModifier,
    AwardBaseNumOfPlayers,
    AwardBasePlayerScore,
    AwardBaseStaleMateModifier,
    AwardBaseTeamScore,
    AwardBaseWinningTeamModifier,
    AwardEnableAwarding,
    CurrentPlayer,
    ExtraData,
    GameBehavior,
    GameBehaviorInit,
    GameDuration,
    GameHostType,
    GameMapName,
    GameStartTime,
    GameState,
    GameZoneGuid,
    HidePlayerWeapon,
    MaxPlayersPerTeam,
    MaxTeams,
    MinigameQueueMap,
    MinPlayersPerTeam,
    MinTeams,
    Name,
    NextPlayer,
    PoolBallPositions,
    PoolBallRadius,
    PreEndGameDuration,
    PreGameBehavior,
    PreGameBehaviorInit,
    PreGameDuration,
    StartDelaySeconds,
    TableConfiguration,
    TargetBall,
    TravelPointGuid,
    TurnState,
    TurnTime,
    TurnTimeLeft,
    UseRespawnDelay,
}
pub(crate) static MINIGAME_INFO_ATTRIBUTES: phf::Map<&'static str, MinigameInfo> = phf_map! {
    "action0" => MinigameInfo::Action0, "action0Duration" =>
    MinigameInfo::Action0Duration, "action0Option" => MinigameInfo::Action0Option,
    "alwaysVisibleToPlayers" => MinigameInfo::AlwaysVisibleToPlayers, "autoReviveDelay"
    => MinigameInfo::AutoReviveDelay, "autoReviveTime" => MinigameInfo::AutoReviveTime,
    "AwareRange" => MinigameInfo::AwareRange, "BeaconRadius" =>
    MinigameInfo::BeaconRadius, "collisionExtent" => MinigameInfo::CollisionExtent,
    "ContentClass" => MinigameInfo::ContentClass, "CycleQuestBase" =>
    MinigameInfo::CycleQuestBase, "defaultWeapon" => MinigameInfo::DefaultWeapon,
    "despawnDelay" => MinigameInfo::DespawnDelay, "Dialogs" => MinigameInfo::Dialogs,
    "DisplayName" => MinigameInfo::DisplayName, "EnableInGame" =>
    MinigameInfo::EnableInGame, "FreedomProperties" => MinigameInfo::FreedomProperties,
    "Freq" => MinigameInfo::Freq, "generateInterestList" =>
    MinigameInfo::GenerateInterestList, "hiddenFromClients" =>
    MinigameInfo::HiddenFromClients, "hiddenFromPlayers" =>
    MinigameInfo::HiddenFromPlayers, "HideAfterInteraction" =>
    MinigameInfo::HideAfterInteraction, "Icon" => MinigameInfo::Icon, "instanceTags" =>
    MinigameInfo::InstanceTags, "instanceZoneKey" => MinigameInfo::InstanceZoneKey,
    "InteractionDuration" => MinigameInfo::InteractionDuration, "InteractionRadius" =>
    MinigameInfo::InteractionRadius, "InteractionResetTimer" =>
    MinigameInfo::InteractionResetTimer, "isNonSpawnedAvatar" =>
    MinigameInfo::IsNonSpawnedAvatar, "isSelfRevivable" => MinigameInfo::IsSelfRevivable,
    "LastInteractionTime" => MinigameInfo::LastInteractionTime, "LuaScript" =>
    MinigameInfo::LuaScript, "lvl" => MinigameInfo::Lvl, "MaterialOverride" =>
    MinigameInfo::MaterialOverride, "nodelink" => MinigameInfo::Nodelink,
    "originalNodeName" => MinigameInfo::OriginalNodeName, "originalZoneName" =>
    MinigameInfo::OriginalZoneName, "partyGUID" => MinigameInfo::PartyGuid,
    "pathfindSafeSpawn" => MinigameInfo::PathfindSafeSpawn, "pos" => MinigameInfo::Pos,
    "Power" => MinigameInfo::Power, "priority" => MinigameInfo::Priority, "QuestFlags" =>
    MinigameInfo::QuestFlags, "ReadableName" => MinigameInfo::ReadableName,
    "respawnDelay" => MinigameInfo::RespawnDelay, "RespawnRegionName" =>
    MinigameInfo::RespawnRegionName, "RespawnRegionNameOverride" =>
    MinigameInfo::RespawnRegionNameOverride, "rot" => MinigameInfo::Rot, "selfRadius" =>
    MinigameInfo::SelfRadius, "spawnMethod" => MinigameInfo::SpawnMethod, "spawnPosition"
    => MinigameInfo::SpawnPosition, "spawnRotation" => MinigameInfo::SpawnRotation,
    "tags" => MinigameInfo::Tags, "teamID" => MinigameInfo::TeamId, "UE3ClassID" =>
    MinigameInfo::Ue3ClassId, "UE3EdVisual" => MinigameInfo::Ue3EdVisual,
    "VisibleOnQuestAvailable" => MinigameInfo::VisibleOnQuestAvailable,
    "VisibleOnQuestComplete" => MinigameInfo::VisibleOnQuestComplete,
    "VisibleOnQuestFinished" => MinigameInfo::VisibleOnQuestFinished,
    "VisibleOnQuestInProgress" => MinigameInfo::VisibleOnQuestInProgress,
    "WorldZoneObjectIndex" => MinigameInfo::WorldZoneObjectIndex, "zone" =>
    MinigameInfo::Zone, "ZoneGuid" => MinigameInfo::ZoneGuid,
    "AdaptiveSpawnerBasePlayers" => MinigameInfo::AdaptiveSpawnerBasePlayers,
    "AdaptiveSpawnerMax" => MinigameInfo::AdaptiveSpawnerMax, "AdaptiveSpawnerMin" =>
    MinigameInfo::AdaptiveSpawnerMin, "AwardBaseCredit" => MinigameInfo::AwardBaseCredit,
    "AwardBaseLosingTeamModifier" => MinigameInfo::AwardBaseLosingTeamModifier,
    "AwardBaseNumOfPlayers" => MinigameInfo::AwardBaseNumOfPlayers,
    "AwardBasePlayerScore" => MinigameInfo::AwardBasePlayerScore,
    "AwardBaseStaleMateModifier" => MinigameInfo::AwardBaseStaleMateModifier,
    "AwardBaseTeamScore" => MinigameInfo::AwardBaseTeamScore,
    "AwardBaseWinningTeamModifier" => MinigameInfo::AwardBaseWinningTeamModifier,
    "AwardEnableAwarding" => MinigameInfo::AwardEnableAwarding, "CurrentPlayer" =>
    MinigameInfo::CurrentPlayer, "ExtraData" => MinigameInfo::ExtraData, "GameBehavior"
    => MinigameInfo::GameBehavior, "GameBehaviorInit" => MinigameInfo::GameBehaviorInit,
    "GameDuration" => MinigameInfo::GameDuration, "GameHostType" =>
    MinigameInfo::GameHostType, "GameMapName" => MinigameInfo::GameMapName,
    "GameStartTime" => MinigameInfo::GameStartTime, "GameState" =>
    MinigameInfo::GameState, "GameZoneGuid" => MinigameInfo::GameZoneGuid,
    "HidePlayerWeapon" => MinigameInfo::HidePlayerWeapon, "MaxPlayersPerTeam" =>
    MinigameInfo::MaxPlayersPerTeam, "MaxTeams" => MinigameInfo::MaxTeams,
    "MinigameQueueMap" => MinigameInfo::MinigameQueueMap, "MinPlayersPerTeam" =>
    MinigameInfo::MinPlayersPerTeam, "MinTeams" => MinigameInfo::MinTeams, "Name" =>
    MinigameInfo::Name, "NextPlayer" => MinigameInfo::NextPlayer, "PoolBallPositions" =>
    MinigameInfo::PoolBallPositions, "PoolBallRadius" => MinigameInfo::PoolBallRadius,
    "PreEndGameDuration" => MinigameInfo::PreEndGameDuration, "PreGameBehavior" =>
    MinigameInfo::PreGameBehavior, "PreGameBehaviorInit" =>
    MinigameInfo::PreGameBehaviorInit, "PreGameDuration" =>
    MinigameInfo::PreGameDuration, "StartDelaySeconds" =>
    MinigameInfo::StartDelaySeconds, "TableConfiguration" =>
    MinigameInfo::TableConfiguration, "TargetBall" => MinigameInfo::TargetBall,
    "TravelPointGuid" => MinigameInfo::TravelPointGuid, "TurnState" =>
    MinigameInfo::TurnState, "TurnTime" => MinigameInfo::TurnTime, "TurnTimeLeft" =>
    MinigameInfo::TurnTimeLeft, "useRespawnDelay" => MinigameInfo::UseRespawnDelay,
};
pub(crate) static MINIGAME_INFO_ATTRIBUTES_ID: phf::Map<u16, MinigameInfo> = phf_map! {
    3444u16 => MinigameInfo::Action0, 3443u16 => MinigameInfo::Action0Duration, 3448u16
    => MinigameInfo::Action0Option, 3526u16 => MinigameInfo::AlwaysVisibleToPlayers,
    10569u16 => MinigameInfo::AutoReviveDelay, 10509u16 => MinigameInfo::AutoReviveTime,
    8287u16 => MinigameInfo::AwareRange, 10980u16 => MinigameInfo::BeaconRadius, 3442u16
    => MinigameInfo::CollisionExtent, 3446u16 => MinigameInfo::ContentClass, 11073u16 =>
    MinigameInfo::CycleQuestBase, 7257u16 => MinigameInfo::DefaultWeapon, 9678u16 =>
    MinigameInfo::DespawnDelay, 8874u16 => MinigameInfo::Dialogs, 6643u16 =>
    MinigameInfo::DisplayName, 6869u16 => MinigameInfo::EnableInGame, 11197u16 =>
    MinigameInfo::FreedomProperties, 3429u16 => MinigameInfo::Freq, 3441u16 =>
    MinigameInfo::GenerateInterestList, 3440u16 => MinigameInfo::HiddenFromClients,
    3450u16 => MinigameInfo::HiddenFromPlayers, 9180u16 =>
    MinigameInfo::HideAfterInteraction, 4384u16 => MinigameInfo::Icon, 3447u16 =>
    MinigameInfo::InstanceTags, 5603u16 => MinigameInfo::InstanceZoneKey, 11143u16 =>
    MinigameInfo::InteractionDuration, 7518u16 => MinigameInfo::InteractionRadius,
    9182u16 => MinigameInfo::InteractionResetTimer, 3456u16 =>
    MinigameInfo::IsNonSpawnedAvatar, 7202u16 => MinigameInfo::IsSelfRevivable, 9181u16
    => MinigameInfo::LastInteractionTime, 7820u16 => MinigameInfo::LuaScript, 6226u16 =>
    MinigameInfo::Lvl, 4765u16 => MinigameInfo::MaterialOverride, 3449u16 =>
    MinigameInfo::Nodelink, 3454u16 => MinigameInfo::OriginalNodeName, 3453u16 =>
    MinigameInfo::OriginalZoneName, 3439u16 => MinigameInfo::PartyGuid, 3451u16 =>
    MinigameInfo::PathfindSafeSpawn, 3438u16 => MinigameInfo::Pos, 3430u16 =>
    MinigameInfo::Power, 3437u16 => MinigameInfo::Priority, 9977u16 =>
    MinigameInfo::QuestFlags, 3711u16 => MinigameInfo::ReadableName, 3457u16 =>
    MinigameInfo::RespawnDelay, 10827u16 => MinigameInfo::RespawnRegionName, 10886u16 =>
    MinigameInfo::RespawnRegionNameOverride, 3436u16 => MinigameInfo::Rot, 3435u16 =>
    MinigameInfo::SelfRadius, 6144u16 => MinigameInfo::SpawnMethod, 7875u16 =>
    MinigameInfo::SpawnPosition, 8230u16 => MinigameInfo::SpawnRotation, 3434u16 =>
    MinigameInfo::Tags, 3433u16 => MinigameInfo::TeamId, 3445u16 =>
    MinigameInfo::Ue3ClassId, 9853u16 => MinigameInfo::Ue3EdVisual, 8762u16 =>
    MinigameInfo::VisibleOnQuestAvailable, 8759u16 =>
    MinigameInfo::VisibleOnQuestComplete, 8760u16 =>
    MinigameInfo::VisibleOnQuestFinished, 8761u16 =>
    MinigameInfo::VisibleOnQuestInProgress, 3455u16 =>
    MinigameInfo::WorldZoneObjectIndex, 3431u16 => MinigameInfo::Zone, 3452u16 =>
    MinigameInfo::ZoneGuid, 4833u16 => MinigameInfo::AdaptiveSpawnerBasePlayers, 4832u16
    => MinigameInfo::AdaptiveSpawnerMax, 4831u16 => MinigameInfo::AdaptiveSpawnerMin,
    3718u16 => MinigameInfo::AwardBaseCredit, 3721u16 =>
    MinigameInfo::AwardBaseLosingTeamModifier, 3717u16 =>
    MinigameInfo::AwardBaseNumOfPlayers, 3720u16 => MinigameInfo::AwardBasePlayerScore,
    3729u16 => MinigameInfo::AwardBaseStaleMateModifier, 3719u16 =>
    MinigameInfo::AwardBaseTeamScore, 3722u16 =>
    MinigameInfo::AwardBaseWinningTeamModifier, 3723u16 =>
    MinigameInfo::AwardEnableAwarding, 4178u16 => MinigameInfo::CurrentPlayer, 3584u16 =>
    MinigameInfo::ExtraData, 3422u16 => MinigameInfo::GameBehavior, 3421u16 =>
    MinigameInfo::GameBehaviorInit, 3420u16 => MinigameInfo::GameDuration, 3419u16 =>
    MinigameInfo::GameHostType, 7031u16 => MinigameInfo::GameMapName, 3427u16 =>
    MinigameInfo::GameStartTime, 3426u16 => MinigameInfo::GameState, 6990u16 =>
    MinigameInfo::GameZoneGuid, 6943u16 => MinigameInfo::HidePlayerWeapon, 3819u16 =>
    MinigameInfo::MaxPlayersPerTeam, 3821u16 => MinigameInfo::MaxTeams, 7058u16 =>
    MinigameInfo::MinigameQueueMap, 3818u16 => MinigameInfo::MinPlayersPerTeam, 3820u16
    => MinigameInfo::MinTeams, 3822u16 => MinigameInfo::Name, 7889u16 =>
    MinigameInfo::NextPlayer, 7707u16 => MinigameInfo::PoolBallPositions, 7706u16 =>
    MinigameInfo::PoolBallRadius, 3423u16 => MinigameInfo::PreEndGameDuration, 3417u16 =>
    MinigameInfo::PreGameBehavior, 3416u16 => MinigameInfo::PreGameBehaviorInit, 3418u16
    => MinigameInfo::PreGameDuration, 3817u16 => MinigameInfo::StartDelaySeconds, 7537u16
    => MinigameInfo::TableConfiguration, 4194u16 => MinigameInfo::TargetBall, 6972u16 =>
    MinigameInfo::TravelPointGuid, 4177u16 => MinigameInfo::TurnState, 4176u16 =>
    MinigameInfo::TurnTime, 4175u16 => MinigameInfo::TurnTimeLeft, 5621u16 =>
    MinigameInfo::UseRespawnDelay,
};
impl Attribute for MinigameInfo {
    fn class() -> Class {
        Class::MinigameInfo
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
            Self::AdaptiveSpawnerBasePlayers => &Self::AdaptiveSpawnerBasePlayers,
            Self::AdaptiveSpawnerMax => &Self::AdaptiveSpawnerMax,
            Self::AdaptiveSpawnerMin => &Self::AdaptiveSpawnerMin,
            Self::AwardBaseCredit => &Self::AwardBaseCredit,
            Self::AwardBaseLosingTeamModifier => &Self::AwardBaseLosingTeamModifier,
            Self::AwardBaseNumOfPlayers => &Self::AwardBaseNumOfPlayers,
            Self::AwardBasePlayerScore => &Self::AwardBasePlayerScore,
            Self::AwardBaseStaleMateModifier => &Self::AwardBaseStaleMateModifier,
            Self::AwardBaseTeamScore => &Self::AwardBaseTeamScore,
            Self::AwardBaseWinningTeamModifier => &Self::AwardBaseWinningTeamModifier,
            Self::AwardEnableAwarding => &Self::AwardEnableAwarding,
            Self::CurrentPlayer => &Self::CurrentPlayer,
            Self::ExtraData => &Self::ExtraData,
            Self::GameBehavior => &Self::GameBehavior,
            Self::GameBehaviorInit => &Self::GameBehaviorInit,
            Self::GameDuration => &Self::GameDuration,
            Self::GameHostType => &Self::GameHostType,
            Self::GameMapName => &Self::GameMapName,
            Self::GameStartTime => &Self::GameStartTime,
            Self::GameState => &Self::GameState,
            Self::GameZoneGuid => &Self::GameZoneGuid,
            Self::HidePlayerWeapon => &Self::HidePlayerWeapon,
            Self::MaxPlayersPerTeam => &Self::MaxPlayersPerTeam,
            Self::MaxTeams => &Self::MaxTeams,
            Self::MinigameQueueMap => &Self::MinigameQueueMap,
            Self::MinPlayersPerTeam => &Self::MinPlayersPerTeam,
            Self::MinTeams => &Self::MinTeams,
            Self::Name => &Self::Name,
            Self::NextPlayer => &Self::NextPlayer,
            Self::PoolBallPositions => &Self::PoolBallPositions,
            Self::PoolBallRadius => &Self::PoolBallRadius,
            Self::PreEndGameDuration => &Self::PreEndGameDuration,
            Self::PreGameBehavior => &Self::PreGameBehavior,
            Self::PreGameBehaviorInit => &Self::PreGameBehaviorInit,
            Self::PreGameDuration => &Self::PreGameDuration,
            Self::StartDelaySeconds => &Self::StartDelaySeconds,
            Self::TableConfiguration => &Self::TableConfiguration,
            Self::TargetBall => &Self::TargetBall,
            Self::TravelPointGuid => &Self::TravelPointGuid,
            Self::TurnState => &Self::TurnState,
            Self::TurnTime => &Self::TurnTime,
            Self::TurnTimeLeft => &Self::TurnTimeLeft,
            Self::UseRespawnDelay => &Self::UseRespawnDelay,
        }
    }
}
impl AttributeInfo for MinigameInfo {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 3444u16,
            Self::Action0Duration => 3443u16,
            Self::Action0Option => 3448u16,
            Self::AlwaysVisibleToPlayers => 3526u16,
            Self::AutoReviveDelay => 10569u16,
            Self::AutoReviveTime => 10509u16,
            Self::AwareRange => 8287u16,
            Self::BeaconRadius => 10980u16,
            Self::CollisionExtent => 3442u16,
            Self::ContentClass => 3446u16,
            Self::CycleQuestBase => 11073u16,
            Self::DefaultWeapon => 7257u16,
            Self::DespawnDelay => 9678u16,
            Self::Dialogs => 8874u16,
            Self::DisplayName => 6643u16,
            Self::EnableInGame => 6869u16,
            Self::FreedomProperties => 11197u16,
            Self::Freq => 3429u16,
            Self::GenerateInterestList => 3441u16,
            Self::HiddenFromClients => 3440u16,
            Self::HiddenFromPlayers => 3450u16,
            Self::HideAfterInteraction => 9180u16,
            Self::Icon => 4384u16,
            Self::InstanceTags => 3447u16,
            Self::InstanceZoneKey => 5603u16,
            Self::InteractionDuration => 11143u16,
            Self::InteractionRadius => 7518u16,
            Self::InteractionResetTimer => 9182u16,
            Self::IsNonSpawnedAvatar => 3456u16,
            Self::IsSelfRevivable => 7202u16,
            Self::LastInteractionTime => 9181u16,
            Self::LuaScript => 7820u16,
            Self::Lvl => 6226u16,
            Self::MaterialOverride => 4765u16,
            Self::Nodelink => 3449u16,
            Self::OriginalNodeName => 3454u16,
            Self::OriginalZoneName => 3453u16,
            Self::PartyGuid => 3439u16,
            Self::PathfindSafeSpawn => 3451u16,
            Self::Pos => 3438u16,
            Self::Power => 3430u16,
            Self::Priority => 3437u16,
            Self::QuestFlags => 9977u16,
            Self::ReadableName => 3711u16,
            Self::RespawnDelay => 3457u16,
            Self::RespawnRegionName => 10827u16,
            Self::RespawnRegionNameOverride => 10886u16,
            Self::Rot => 3436u16,
            Self::SelfRadius => 3435u16,
            Self::SpawnMethod => 6144u16,
            Self::SpawnPosition => 7875u16,
            Self::SpawnRotation => 8230u16,
            Self::Tags => 3434u16,
            Self::TeamId => 3433u16,
            Self::Ue3ClassId => 3445u16,
            Self::Ue3EdVisual => 9853u16,
            Self::VisibleOnQuestAvailable => 8762u16,
            Self::VisibleOnQuestComplete => 8759u16,
            Self::VisibleOnQuestFinished => 8760u16,
            Self::VisibleOnQuestInProgress => 8761u16,
            Self::WorldZoneObjectIndex => 3455u16,
            Self::Zone => 3431u16,
            Self::ZoneGuid => 3452u16,
            Self::AdaptiveSpawnerBasePlayers => 4833u16,
            Self::AdaptiveSpawnerMax => 4832u16,
            Self::AdaptiveSpawnerMin => 4831u16,
            Self::AwardBaseCredit => 3718u16,
            Self::AwardBaseLosingTeamModifier => 3721u16,
            Self::AwardBaseNumOfPlayers => 3717u16,
            Self::AwardBasePlayerScore => 3720u16,
            Self::AwardBaseStaleMateModifier => 3729u16,
            Self::AwardBaseTeamScore => 3719u16,
            Self::AwardBaseWinningTeamModifier => 3722u16,
            Self::AwardEnableAwarding => 3723u16,
            Self::CurrentPlayer => 4178u16,
            Self::ExtraData => 3584u16,
            Self::GameBehavior => 3422u16,
            Self::GameBehaviorInit => 3421u16,
            Self::GameDuration => 3420u16,
            Self::GameHostType => 3419u16,
            Self::GameMapName => 7031u16,
            Self::GameStartTime => 3427u16,
            Self::GameState => 3426u16,
            Self::GameZoneGuid => 6990u16,
            Self::HidePlayerWeapon => 6943u16,
            Self::MaxPlayersPerTeam => 3819u16,
            Self::MaxTeams => 3821u16,
            Self::MinigameQueueMap => 7058u16,
            Self::MinPlayersPerTeam => 3818u16,
            Self::MinTeams => 3820u16,
            Self::Name => 3822u16,
            Self::NextPlayer => 7889u16,
            Self::PoolBallPositions => 7707u16,
            Self::PoolBallRadius => 7706u16,
            Self::PreEndGameDuration => 3423u16,
            Self::PreGameBehavior => 3417u16,
            Self::PreGameBehaviorInit => 3416u16,
            Self::PreGameDuration => 3418u16,
            Self::StartDelaySeconds => 3817u16,
            Self::TableConfiguration => 7537u16,
            Self::TargetBall => 4194u16,
            Self::TravelPointGuid => 6972u16,
            Self::TurnState => 4177u16,
            Self::TurnTime => 4176u16,
            Self::TurnTimeLeft => 4175u16,
            Self::UseRespawnDelay => 5621u16,
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
            Self::AdaptiveSpawnerBasePlayers => "AdaptiveSpawnerBasePlayers",
            Self::AdaptiveSpawnerMax => "AdaptiveSpawnerMax",
            Self::AdaptiveSpawnerMin => "AdaptiveSpawnerMin",
            Self::AwardBaseCredit => "AwardBaseCredit",
            Self::AwardBaseLosingTeamModifier => "AwardBaseLosingTeamModifier",
            Self::AwardBaseNumOfPlayers => "AwardBaseNumOfPlayers",
            Self::AwardBasePlayerScore => "AwardBasePlayerScore",
            Self::AwardBaseStaleMateModifier => "AwardBaseStaleMateModifier",
            Self::AwardBaseTeamScore => "AwardBaseTeamScore",
            Self::AwardBaseWinningTeamModifier => "AwardBaseWinningTeamModifier",
            Self::AwardEnableAwarding => "AwardEnableAwarding",
            Self::CurrentPlayer => "CurrentPlayer",
            Self::ExtraData => "ExtraData",
            Self::GameBehavior => "GameBehavior",
            Self::GameBehaviorInit => "GameBehaviorInit",
            Self::GameDuration => "GameDuration",
            Self::GameHostType => "GameHostType",
            Self::GameMapName => "GameMapName",
            Self::GameStartTime => "GameStartTime",
            Self::GameState => "GameState",
            Self::GameZoneGuid => "GameZoneGuid",
            Self::HidePlayerWeapon => "HidePlayerWeapon",
            Self::MaxPlayersPerTeam => "MaxPlayersPerTeam",
            Self::MaxTeams => "MaxTeams",
            Self::MinigameQueueMap => "MinigameQueueMap",
            Self::MinPlayersPerTeam => "MinPlayersPerTeam",
            Self::MinTeams => "MinTeams",
            Self::Name => "Name",
            Self::NextPlayer => "NextPlayer",
            Self::PoolBallPositions => "PoolBallPositions",
            Self::PoolBallRadius => "PoolBallRadius",
            Self::PreEndGameDuration => "PreEndGameDuration",
            Self::PreGameBehavior => "PreGameBehavior",
            Self::PreGameBehaviorInit => "PreGameBehaviorInit",
            Self::PreGameDuration => "PreGameDuration",
            Self::StartDelaySeconds => "StartDelaySeconds",
            Self::TableConfiguration => "TableConfiguration",
            Self::TargetBall => "TargetBall",
            Self::TravelPointGuid => "TravelPointGuid",
            Self::TurnState => "TurnState",
            Self::TurnTime => "TurnTime",
            Self::TurnTimeLeft => "TurnTimeLeft",
            Self::UseRespawnDelay => "useRespawnDelay",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AlwaysVisibleToPlayers => ParamType::Bool,
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::SelfRadius => ParamType::Float,
            Self::Tags => ParamType::String,
            Self::Ue3ClassId => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
            Self::AdaptiveSpawnerBasePlayers => ParamType::Int,
            Self::AdaptiveSpawnerMax => ParamType::Float,
            Self::AdaptiveSpawnerMin => ParamType::Float,
            Self::AwardBaseCredit => ParamType::Float,
            Self::AwardBaseLosingTeamModifier => ParamType::Float,
            Self::AwardBaseNumOfPlayers => ParamType::Int,
            Self::AwardBasePlayerScore => ParamType::Int,
            Self::AwardBaseStaleMateModifier => ParamType::Float,
            Self::AwardBaseTeamScore => ParamType::Int,
            Self::AwardBaseWinningTeamModifier => ParamType::Float,
            Self::AwardEnableAwarding => ParamType::Bool,
            Self::CurrentPlayer => ParamType::String,
            Self::ExtraData => ParamType::JsonValue,
            Self::GameBehavior => ParamType::String,
            Self::GameBehaviorInit => ParamType::String,
            Self::GameDuration => ParamType::Float,
            Self::GameHostType => ParamType::Int,
            Self::GameMapName => ParamType::String,
            Self::GameStartTime => ParamType::Float,
            Self::GameState => ParamType::Int,
            Self::GameZoneGuid => ParamType::Guid,
            Self::HidePlayerWeapon => ParamType::Bool,
            Self::MaxPlayersPerTeam => ParamType::Int,
            Self::MaxTeams => ParamType::Int,
            Self::MinigameQueueMap => ParamType::String,
            Self::MinPlayersPerTeam => ParamType::Int,
            Self::MinTeams => ParamType::Int,
            Self::Name => ParamType::String,
            Self::NextPlayer => ParamType::String,
            Self::PoolBallPositions => ParamType::JsonValue,
            Self::PoolBallRadius => ParamType::Float,
            Self::PreEndGameDuration => ParamType::Float,
            Self::PreGameBehavior => ParamType::String,
            Self::PreGameBehaviorInit => ParamType::String,
            Self::PreGameDuration => ParamType::Float,
            Self::StartDelaySeconds => ParamType::Int,
            Self::TableConfiguration => ParamType::JsonValue,
            Self::TargetBall => ParamType::Int,
            Self::TravelPointGuid => ParamType::Guid,
            Self::TurnState => ParamType::JsonValue,
            Self::TurnTime => ParamType::Float,
            Self::TurnTimeLeft => ParamType::Float,
            Self::UseRespawnDelay => ParamType::Bool,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::Action0Option => ParamType::Int,
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
            Self::SpawnMethod => ParamType::String,
            Self::SpawnPosition => ParamType::Vector3,
            Self::SpawnRotation => ParamType::Vector3,
            Self::TeamId => ParamType::Int,
            Self::Ue3EdVisual => ParamType::String,
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static ALWAYS_VISIBLE_TO_PLAYERS: Value = Value::Bool(false);
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
        static SELF_RADIUS: Value = Value::Float(200000f32);
        static TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            "minigameInfo".to_string(),
        ));
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLMiniGame".to_string(),
        ));
        static ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ADAPTIVE_SPAWNER_BASE_PLAYERS: Value = Value::Int(5i32);
        static ADAPTIVE_SPAWNER_MAX: Value = Value::Float(2f32);
        static ADAPTIVE_SPAWNER_MIN: Value = Value::Float(0.5f32);
        static AWARD_BASE_CREDIT: Value = Value::Float(0f32);
        static AWARD_BASE_LOSING_TEAM_MODIFIER: Value = Value::Float(0f32);
        static AWARD_BASE_NUM_OF_PLAYERS: Value = Value::Int(0i32);
        static AWARD_BASE_PLAYER_SCORE: Value = Value::Int(0i32);
        static AWARD_BASE_STALE_MATE_MODIFIER: Value = Value::Float(0f32);
        static AWARD_BASE_TEAM_SCORE: Value = Value::Int(0i32);
        static AWARD_BASE_WINNING_TEAM_MODIFIER: Value = Value::Float(0f32);
        static AWARD_ENABLE_AWARDING: Value = Value::Bool(false);
        static CURRENT_PLAYER: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static EXTRA_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[]").unwrap(),
        ));
        static GAME_BEHAVIOR: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static GAME_BEHAVIOR_INIT: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static GAME_DURATION: Value = Value::Float(240f32);
        static GAME_HOST_TYPE: Value = Value::Int(0i32);
        static GAME_MAP_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            "00000000-0000-0000-0000-000000000000".to_string(),
        ));
        static GAME_START_TIME: Value = Value::Float(0f32);
        static GAME_STATE: Value = Value::Int(0i32);
        static GAME_ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static HIDE_PLAYER_WEAPON: Value = Value::Bool(true);
        static MAX_PLAYERS_PER_TEAM: Value = Value::Int(0i32);
        static MAX_TEAMS: Value = Value::Int(0i32);
        static MINIGAME_QUEUE_MAP: Lazy<Value> = Lazy::new(|| Value::String(
            "LM_P".to_string(),
        ));
        static MIN_PLAYERS_PER_TEAM: Value = Value::Int(0i32);
        static MIN_TEAMS: Value = Value::Int(0i32);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static NEXT_PLAYER: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static POOL_BALL_POSITIONS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static POOL_BALL_RADIUS: Value = Value::Float(2.12068f32);
        static PRE_END_GAME_DURATION: Value = Value::Float(8f32);
        static PRE_GAME_BEHAVIOR: Lazy<Value> = Lazy::new(|| Value::String(
            "MinigamePlayerStart".to_string(),
        ));
        static PRE_GAME_BEHAVIOR_INIT: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PRE_GAME_DURATION: Value = Value::Float(25f32);
        static START_DELAY_SECONDS: Value = Value::Int(0i32);
        static TABLE_CONFIGURATION: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static TARGET_BALL: Value = Value::Int(0i32);
        static TRAVEL_POINT_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static TURN_STATE: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static TURN_TIME: Value = Value::Float(0f32);
        static TURN_TIME_LEFT: Value = Value::Float(0f32);
        static USE_RESPAWN_DELAY: Value = Value::Bool(false);
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
        static SPAWN_METHOD: Lazy<Value> = Lazy::new(|| Value::String(
            "normal".to_string(),
        ));
        static SPAWN_POSITION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static SPAWN_ROTATION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
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
        match self {
            Self::AlwaysVisibleToPlayers => &ALWAYS_VISIBLE_TO_PLAYERS,
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::SelfRadius => &SELF_RADIUS,
            Self::Tags => &TAGS,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::ZoneGuid => &ZONE_GUID,
            Self::AdaptiveSpawnerBasePlayers => &ADAPTIVE_SPAWNER_BASE_PLAYERS,
            Self::AdaptiveSpawnerMax => &ADAPTIVE_SPAWNER_MAX,
            Self::AdaptiveSpawnerMin => &ADAPTIVE_SPAWNER_MIN,
            Self::AwardBaseCredit => &AWARD_BASE_CREDIT,
            Self::AwardBaseLosingTeamModifier => &AWARD_BASE_LOSING_TEAM_MODIFIER,
            Self::AwardBaseNumOfPlayers => &AWARD_BASE_NUM_OF_PLAYERS,
            Self::AwardBasePlayerScore => &AWARD_BASE_PLAYER_SCORE,
            Self::AwardBaseStaleMateModifier => &AWARD_BASE_STALE_MATE_MODIFIER,
            Self::AwardBaseTeamScore => &AWARD_BASE_TEAM_SCORE,
            Self::AwardBaseWinningTeamModifier => &AWARD_BASE_WINNING_TEAM_MODIFIER,
            Self::AwardEnableAwarding => &AWARD_ENABLE_AWARDING,
            Self::CurrentPlayer => &CURRENT_PLAYER,
            Self::ExtraData => &EXTRA_DATA,
            Self::GameBehavior => &GAME_BEHAVIOR,
            Self::GameBehaviorInit => &GAME_BEHAVIOR_INIT,
            Self::GameDuration => &GAME_DURATION,
            Self::GameHostType => &GAME_HOST_TYPE,
            Self::GameMapName => &GAME_MAP_NAME,
            Self::GameStartTime => &GAME_START_TIME,
            Self::GameState => &GAME_STATE,
            Self::GameZoneGuid => &GAME_ZONE_GUID,
            Self::HidePlayerWeapon => &HIDE_PLAYER_WEAPON,
            Self::MaxPlayersPerTeam => &MAX_PLAYERS_PER_TEAM,
            Self::MaxTeams => &MAX_TEAMS,
            Self::MinigameQueueMap => &MINIGAME_QUEUE_MAP,
            Self::MinPlayersPerTeam => &MIN_PLAYERS_PER_TEAM,
            Self::MinTeams => &MIN_TEAMS,
            Self::Name => &NAME,
            Self::NextPlayer => &NEXT_PLAYER,
            Self::PoolBallPositions => &POOL_BALL_POSITIONS,
            Self::PoolBallRadius => &POOL_BALL_RADIUS,
            Self::PreEndGameDuration => &PRE_END_GAME_DURATION,
            Self::PreGameBehavior => &PRE_GAME_BEHAVIOR,
            Self::PreGameBehaviorInit => &PRE_GAME_BEHAVIOR_INIT,
            Self::PreGameDuration => &PRE_GAME_DURATION,
            Self::StartDelaySeconds => &START_DELAY_SECONDS,
            Self::TableConfiguration => &TABLE_CONFIGURATION,
            Self::TargetBall => &TARGET_BALL,
            Self::TravelPointGuid => &TRAVEL_POINT_GUID,
            Self::TurnState => &TURN_STATE,
            Self::TurnTime => &TURN_TIME,
            Self::TurnTimeLeft => &TURN_TIME_LEFT,
            Self::UseRespawnDelay => &USE_RESPAWN_DELAY,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::Action0Option => &ACTION_0_OPTION,
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
            Self::SpawnMethod => &SPAWN_METHOD,
            Self::SpawnPosition => &SPAWN_POSITION,
            Self::SpawnRotation => &SPAWN_ROTATION,
            Self::TeamId => &TEAM_ID,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AlwaysVisibleToPlayers => {
                &[ParamFlag::ClientUnknown, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SelfRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
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
            Self::ZoneGuid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::AdaptiveSpawnerBasePlayers => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AdaptiveSpawnerMax => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdaptiveSpawnerMin => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwardBaseCredit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwardBaseLosingTeamModifier => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AwardBaseNumOfPlayers => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwardBasePlayerScore => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwardBaseStaleMateModifier => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AwardBaseTeamScore => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AwardBaseWinningTeamModifier => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AwardEnableAwarding => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CurrentPlayer => &[ParamFlag::NodeOwn],
            Self::ExtraData => &[ParamFlag::NodeOwn],
            Self::GameBehavior => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameBehaviorInit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameDuration => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::GameHostType => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameMapName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameStartTime => &[ParamFlag::NodeOwn],
            Self::GameState => &[ParamFlag::NodeOwn],
            Self::GameZoneGuid => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HidePlayerWeapon => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::MaxPlayersPerTeam => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MaxTeams => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MinigameQueueMap => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MinPlayersPerTeam => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::MinTeams => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::NextPlayer => &[ParamFlag::NodeOwn],
            Self::PoolBallPositions => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::PoolBallRadius => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PreEndGameDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PreGameBehavior => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PreGameBehaviorInit => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PreGameDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::StartDelaySeconds => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TableConfiguration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TargetBall => &[ParamFlag::NodeOwn],
            Self::TravelPointGuid => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TurnState => &[ParamFlag::NodeOwn],
            Self::TurnTime => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TurnTimeLeft => &[ParamFlag::NodeOwn],
            Self::UseRespawnDelay => &[ParamFlag::Persistent, ParamFlag::Content],
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
        }
    }
}
impl FromStr for MinigameInfo {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MINIGAME_INFO_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MinigameInfo {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            3444u16 => Ok(Self::Action0),
            3443u16 => Ok(Self::Action0Duration),
            3448u16 => Ok(Self::Action0Option),
            3526u16 => Ok(Self::AlwaysVisibleToPlayers),
            10569u16 => Ok(Self::AutoReviveDelay),
            10509u16 => Ok(Self::AutoReviveTime),
            8287u16 => Ok(Self::AwareRange),
            10980u16 => Ok(Self::BeaconRadius),
            3442u16 => Ok(Self::CollisionExtent),
            3446u16 => Ok(Self::ContentClass),
            11073u16 => Ok(Self::CycleQuestBase),
            7257u16 => Ok(Self::DefaultWeapon),
            9678u16 => Ok(Self::DespawnDelay),
            8874u16 => Ok(Self::Dialogs),
            6643u16 => Ok(Self::DisplayName),
            6869u16 => Ok(Self::EnableInGame),
            11197u16 => Ok(Self::FreedomProperties),
            3429u16 => Ok(Self::Freq),
            3441u16 => Ok(Self::GenerateInterestList),
            3440u16 => Ok(Self::HiddenFromClients),
            3450u16 => Ok(Self::HiddenFromPlayers),
            9180u16 => Ok(Self::HideAfterInteraction),
            4384u16 => Ok(Self::Icon),
            3447u16 => Ok(Self::InstanceTags),
            5603u16 => Ok(Self::InstanceZoneKey),
            11143u16 => Ok(Self::InteractionDuration),
            7518u16 => Ok(Self::InteractionRadius),
            9182u16 => Ok(Self::InteractionResetTimer),
            3456u16 => Ok(Self::IsNonSpawnedAvatar),
            7202u16 => Ok(Self::IsSelfRevivable),
            9181u16 => Ok(Self::LastInteractionTime),
            7820u16 => Ok(Self::LuaScript),
            6226u16 => Ok(Self::Lvl),
            4765u16 => Ok(Self::MaterialOverride),
            3449u16 => Ok(Self::Nodelink),
            3454u16 => Ok(Self::OriginalNodeName),
            3453u16 => Ok(Self::OriginalZoneName),
            3439u16 => Ok(Self::PartyGuid),
            3451u16 => Ok(Self::PathfindSafeSpawn),
            3438u16 => Ok(Self::Pos),
            3430u16 => Ok(Self::Power),
            3437u16 => Ok(Self::Priority),
            9977u16 => Ok(Self::QuestFlags),
            3711u16 => Ok(Self::ReadableName),
            3457u16 => Ok(Self::RespawnDelay),
            10827u16 => Ok(Self::RespawnRegionName),
            10886u16 => Ok(Self::RespawnRegionNameOverride),
            3436u16 => Ok(Self::Rot),
            3435u16 => Ok(Self::SelfRadius),
            6144u16 => Ok(Self::SpawnMethod),
            7875u16 => Ok(Self::SpawnPosition),
            8230u16 => Ok(Self::SpawnRotation),
            3434u16 => Ok(Self::Tags),
            3433u16 => Ok(Self::TeamId),
            3445u16 => Ok(Self::Ue3ClassId),
            9853u16 => Ok(Self::Ue3EdVisual),
            8762u16 => Ok(Self::VisibleOnQuestAvailable),
            8759u16 => Ok(Self::VisibleOnQuestComplete),
            8760u16 => Ok(Self::VisibleOnQuestFinished),
            8761u16 => Ok(Self::VisibleOnQuestInProgress),
            3455u16 => Ok(Self::WorldZoneObjectIndex),
            3431u16 => Ok(Self::Zone),
            3452u16 => Ok(Self::ZoneGuid),
            4833u16 => Ok(Self::AdaptiveSpawnerBasePlayers),
            4832u16 => Ok(Self::AdaptiveSpawnerMax),
            4831u16 => Ok(Self::AdaptiveSpawnerMin),
            3718u16 => Ok(Self::AwardBaseCredit),
            3721u16 => Ok(Self::AwardBaseLosingTeamModifier),
            3717u16 => Ok(Self::AwardBaseNumOfPlayers),
            3720u16 => Ok(Self::AwardBasePlayerScore),
            3729u16 => Ok(Self::AwardBaseStaleMateModifier),
            3719u16 => Ok(Self::AwardBaseTeamScore),
            3722u16 => Ok(Self::AwardBaseWinningTeamModifier),
            3723u16 => Ok(Self::AwardEnableAwarding),
            4178u16 => Ok(Self::CurrentPlayer),
            3584u16 => Ok(Self::ExtraData),
            3422u16 => Ok(Self::GameBehavior),
            3421u16 => Ok(Self::GameBehaviorInit),
            3420u16 => Ok(Self::GameDuration),
            3419u16 => Ok(Self::GameHostType),
            7031u16 => Ok(Self::GameMapName),
            3427u16 => Ok(Self::GameStartTime),
            3426u16 => Ok(Self::GameState),
            6990u16 => Ok(Self::GameZoneGuid),
            6943u16 => Ok(Self::HidePlayerWeapon),
            3819u16 => Ok(Self::MaxPlayersPerTeam),
            3821u16 => Ok(Self::MaxTeams),
            7058u16 => Ok(Self::MinigameQueueMap),
            3818u16 => Ok(Self::MinPlayersPerTeam),
            3820u16 => Ok(Self::MinTeams),
            3822u16 => Ok(Self::Name),
            7889u16 => Ok(Self::NextPlayer),
            7707u16 => Ok(Self::PoolBallPositions),
            7706u16 => Ok(Self::PoolBallRadius),
            3423u16 => Ok(Self::PreEndGameDuration),
            3417u16 => Ok(Self::PreGameBehavior),
            3416u16 => Ok(Self::PreGameBehaviorInit),
            3418u16 => Ok(Self::PreGameDuration),
            3817u16 => Ok(Self::StartDelaySeconds),
            7537u16 => Ok(Self::TableConfiguration),
            4194u16 => Ok(Self::TargetBall),
            6972u16 => Ok(Self::TravelPointGuid),
            4177u16 => Ok(Self::TurnState),
            4176u16 => Ok(Self::TurnTime),
            4175u16 => Ok(Self::TurnTimeLeft),
            5621u16 => Ok(Self::UseRespawnDelay),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
