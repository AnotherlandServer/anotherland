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
pub enum LocalTacticNode {
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
    AwareOfResourceLocation,
    Bias,
    IsWayPoint,
    MovementBehavior,
    NotAwareOfResourceLocation,
    PopulationMax,
    PopulationMin,
    ProtectionRadius,
    RemainingTimeLessThan,
    RemainingTimeMoreThan,
    ResourceGuid,
    SelfTeamNotOwnNode,
    SelfTeamNotOwnResource,
    SelfTeamOwnNode,
    SelfTeamOwnResource,
    SyncToResourceAvatar,
    TacticPriority,
    TacticRadius,
    TacticType,
}
pub(crate) static LOCAL_TACTIC_NODE_ATTRIBUTES: phf::Map<
    &'static str,
    LocalTacticNode,
> = phf_map! {
    "action0" => LocalTacticNode::Action0, "action0Duration" =>
    LocalTacticNode::Action0Duration, "action0Option" => LocalTacticNode::Action0Option,
    "alwaysVisibleToPlayers" => LocalTacticNode::AlwaysVisibleToPlayers,
    "autoReviveDelay" => LocalTacticNode::AutoReviveDelay, "autoReviveTime" =>
    LocalTacticNode::AutoReviveTime, "AwareRange" => LocalTacticNode::AwareRange,
    "BeaconRadius" => LocalTacticNode::BeaconRadius, "collisionExtent" =>
    LocalTacticNode::CollisionExtent, "ContentClass" => LocalTacticNode::ContentClass,
    "CycleQuestBase" => LocalTacticNode::CycleQuestBase, "defaultWeapon" =>
    LocalTacticNode::DefaultWeapon, "despawnDelay" => LocalTacticNode::DespawnDelay,
    "Dialogs" => LocalTacticNode::Dialogs, "DisplayName" => LocalTacticNode::DisplayName,
    "EnableInGame" => LocalTacticNode::EnableInGame, "FreedomProperties" =>
    LocalTacticNode::FreedomProperties, "Freq" => LocalTacticNode::Freq,
    "generateInterestList" => LocalTacticNode::GenerateInterestList, "hiddenFromClients"
    => LocalTacticNode::HiddenFromClients, "hiddenFromPlayers" =>
    LocalTacticNode::HiddenFromPlayers, "HideAfterInteraction" =>
    LocalTacticNode::HideAfterInteraction, "Icon" => LocalTacticNode::Icon,
    "instanceTags" => LocalTacticNode::InstanceTags, "instanceZoneKey" =>
    LocalTacticNode::InstanceZoneKey, "InteractionDuration" =>
    LocalTacticNode::InteractionDuration, "InteractionRadius" =>
    LocalTacticNode::InteractionRadius, "InteractionResetTimer" =>
    LocalTacticNode::InteractionResetTimer, "isNonSpawnedAvatar" =>
    LocalTacticNode::IsNonSpawnedAvatar, "isSelfRevivable" =>
    LocalTacticNode::IsSelfRevivable, "LastInteractionTime" =>
    LocalTacticNode::LastInteractionTime, "LuaScript" => LocalTacticNode::LuaScript,
    "lvl" => LocalTacticNode::Lvl, "MaterialOverride" =>
    LocalTacticNode::MaterialOverride, "nodelink" => LocalTacticNode::Nodelink,
    "originalNodeName" => LocalTacticNode::OriginalNodeName, "originalZoneName" =>
    LocalTacticNode::OriginalZoneName, "partyGUID" => LocalTacticNode::PartyGuid,
    "pathfindSafeSpawn" => LocalTacticNode::PathfindSafeSpawn, "pos" =>
    LocalTacticNode::Pos, "Power" => LocalTacticNode::Power, "priority" =>
    LocalTacticNode::Priority, "QuestFlags" => LocalTacticNode::QuestFlags,
    "ReadableName" => LocalTacticNode::ReadableName, "respawnDelay" =>
    LocalTacticNode::RespawnDelay, "RespawnRegionName" =>
    LocalTacticNode::RespawnRegionName, "RespawnRegionNameOverride" =>
    LocalTacticNode::RespawnRegionNameOverride, "rot" => LocalTacticNode::Rot,
    "selfRadius" => LocalTacticNode::SelfRadius, "spawnMethod" =>
    LocalTacticNode::SpawnMethod, "spawnPosition" => LocalTacticNode::SpawnPosition,
    "spawnRotation" => LocalTacticNode::SpawnRotation, "tags" => LocalTacticNode::Tags,
    "teamID" => LocalTacticNode::TeamId, "UE3ClassID" => LocalTacticNode::Ue3ClassId,
    "UE3EdVisual" => LocalTacticNode::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    LocalTacticNode::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    LocalTacticNode::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    LocalTacticNode::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    LocalTacticNode::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    LocalTacticNode::WorldZoneObjectIndex, "zone" => LocalTacticNode::Zone, "ZoneGuid" =>
    LocalTacticNode::ZoneGuid, "awareDist" => LocalTacticNode::AwareDist, "defb" =>
    LocalTacticNode::Defb, "instanceGroup" => LocalTacticNode::InstanceGroup,
    "isUnAttackable" => LocalTacticNode::IsUnAttackable, "abilities" =>
    LocalTacticNode::Abilities, "alive" => LocalTacticNode::Alive, "attackedBy" =>
    LocalTacticNode::AttackedBy, "carrierGuid" => LocalTacticNode::CarrierGuid,
    "clientLoadingPriority" => LocalTacticNode::ClientLoadingPriority, "directorTags" =>
    LocalTacticNode::DirectorTags, "forceSpawnOnClient" =>
    LocalTacticNode::ForceSpawnOnClient, "hpCur" => LocalTacticNode::HpCur, "hpMax" =>
    LocalTacticNode::HpMax, "isLocked" => LocalTacticNode::IsLocked, "spawnerAvatarGuid"
    => LocalTacticNode::SpawnerAvatarGuid, "spawnerAvatarID" =>
    LocalTacticNode::SpawnerAvatarId, "AwareOfResourceLocation" =>
    LocalTacticNode::AwareOfResourceLocation, "Bias" => LocalTacticNode::Bias,
    "IsWayPoint" => LocalTacticNode::IsWayPoint, "MovementBehavior" =>
    LocalTacticNode::MovementBehavior, "NotAwareOfResourceLocation" =>
    LocalTacticNode::NotAwareOfResourceLocation, "PopulationMax" =>
    LocalTacticNode::PopulationMax, "PopulationMin" => LocalTacticNode::PopulationMin,
    "ProtectionRadius" => LocalTacticNode::ProtectionRadius, "RemainingTimeLessThan" =>
    LocalTacticNode::RemainingTimeLessThan, "RemainingTimeMoreThan" =>
    LocalTacticNode::RemainingTimeMoreThan, "ResourceGuid" =>
    LocalTacticNode::ResourceGuid, "SelfTeamNotOwnNode" =>
    LocalTacticNode::SelfTeamNotOwnNode, "SelfTeamNotOwnResource" =>
    LocalTacticNode::SelfTeamNotOwnResource, "SelfTeamOwnNode" =>
    LocalTacticNode::SelfTeamOwnNode, "SelfTeamOwnResource" =>
    LocalTacticNode::SelfTeamOwnResource, "SyncToResourceAvatar" =>
    LocalTacticNode::SyncToResourceAvatar, "TacticPriority" =>
    LocalTacticNode::TacticPriority, "TacticRadius" => LocalTacticNode::TacticRadius,
    "TacticType" => LocalTacticNode::TacticType,
};
pub(crate) static LOCAL_TACTIC_NODE_ATTRIBUTES_ID: phf::Map<u16, LocalTacticNode> = phf_map! {
    5741u16 => LocalTacticNode::Action0, 5740u16 => LocalTacticNode::Action0Duration,
    5751u16 => LocalTacticNode::Action0Option, 5724u16 =>
    LocalTacticNode::AlwaysVisibleToPlayers, 10564u16 =>
    LocalTacticNode::AutoReviveDelay, 10504u16 => LocalTacticNode::AutoReviveTime,
    8283u16 => LocalTacticNode::AwareRange, 10975u16 => LocalTacticNode::BeaconRadius,
    5739u16 => LocalTacticNode::CollisionExtent, 5743u16 =>
    LocalTacticNode::ContentClass, 11061u16 => LocalTacticNode::CycleQuestBase, 7249u16
    => LocalTacticNode::DefaultWeapon, 9674u16 => LocalTacticNode::DespawnDelay, 8870u16
    => LocalTacticNode::Dialogs, 6635u16 => LocalTacticNode::DisplayName, 6861u16 =>
    LocalTacticNode::EnableInGame, 11185u16 => LocalTacticNode::FreedomProperties,
    5726u16 => LocalTacticNode::Freq, 5738u16 => LocalTacticNode::GenerateInterestList,
    5737u16 => LocalTacticNode::HiddenFromClients, 5753u16 =>
    LocalTacticNode::HiddenFromPlayers, 9168u16 => LocalTacticNode::HideAfterInteraction,
    5721u16 => LocalTacticNode::Icon, 5750u16 => LocalTacticNode::InstanceTags, 5710u16
    => LocalTacticNode::InstanceZoneKey, 11131u16 =>
    LocalTacticNode::InteractionDuration, 7510u16 => LocalTacticNode::InteractionRadius,
    9170u16 => LocalTacticNode::InteractionResetTimer, 5763u16 =>
    LocalTacticNode::IsNonSpawnedAvatar, 7194u16 => LocalTacticNode::IsSelfRevivable,
    9169u16 => LocalTacticNode::LastInteractionTime, 7818u16 =>
    LocalTacticNode::LuaScript, 6218u16 => LocalTacticNode::Lvl, 5714u16 =>
    LocalTacticNode::MaterialOverride, 5752u16 => LocalTacticNode::Nodelink, 5761u16 =>
    LocalTacticNode::OriginalNodeName, 5760u16 => LocalTacticNode::OriginalZoneName,
    5736u16 => LocalTacticNode::PartyGuid, 5754u16 => LocalTacticNode::PathfindSafeSpawn,
    5735u16 => LocalTacticNode::Pos, 5727u16 => LocalTacticNode::Power, 5734u16 =>
    LocalTacticNode::Priority, 9972u16 => LocalTacticNode::QuestFlags, 5723u16 =>
    LocalTacticNode::ReadableName, 5764u16 => LocalTacticNode::RespawnDelay, 10822u16 =>
    LocalTacticNode::RespawnRegionName, 10881u16 =>
    LocalTacticNode::RespawnRegionNameOverride, 5733u16 => LocalTacticNode::Rot, 5732u16
    => LocalTacticNode::SelfRadius, 6136u16 => LocalTacticNode::SpawnMethod, 7873u16 =>
    LocalTacticNode::SpawnPosition, 8226u16 => LocalTacticNode::SpawnRotation, 5731u16 =>
    LocalTacticNode::Tags, 5730u16 => LocalTacticNode::TeamId, 5742u16 =>
    LocalTacticNode::Ue3ClassId, 9848u16 => LocalTacticNode::Ue3EdVisual, 8746u16 =>
    LocalTacticNode::VisibleOnQuestAvailable, 8743u16 =>
    LocalTacticNode::VisibleOnQuestComplete, 8744u16 =>
    LocalTacticNode::VisibleOnQuestFinished, 8745u16 =>
    LocalTacticNode::VisibleOnQuestInProgress, 5762u16 =>
    LocalTacticNode::WorldZoneObjectIndex, 5728u16 => LocalTacticNode::Zone, 5755u16 =>
    LocalTacticNode::ZoneGuid, 5758u16 => LocalTacticNode::AwareDist, 5745u16 =>
    LocalTacticNode::Defb, 11382u16 => LocalTacticNode::InstanceGroup, 12440u16 =>
    LocalTacticNode::IsUnAttackable, 9343u16 => LocalTacticNode::Abilities, 5749u16 =>
    LocalTacticNode::Alive, 5748u16 => LocalTacticNode::AttackedBy, 5756u16 =>
    LocalTacticNode::CarrierGuid, 11284u16 => LocalTacticNode::ClientLoadingPriority,
    8100u16 => LocalTacticNode::DirectorTags, 5757u16 =>
    LocalTacticNode::ForceSpawnOnClient, 5747u16 => LocalTacticNode::HpCur, 5746u16 =>
    LocalTacticNode::HpMax, 5711u16 => LocalTacticNode::IsLocked, 5979u16 =>
    LocalTacticNode::SpawnerAvatarGuid, 7705u16 => LocalTacticNode::SpawnerAvatarId,
    5704u16 => LocalTacticNode::AwareOfResourceLocation, 5708u16 =>
    LocalTacticNode::Bias, 5693u16 => LocalTacticNode::IsWayPoint, 5825u16 =>
    LocalTacticNode::MovementBehavior, 5703u16 =>
    LocalTacticNode::NotAwareOfResourceLocation, 5702u16 =>
    LocalTacticNode::PopulationMax, 5701u16 => LocalTacticNode::PopulationMin, 5895u16 =>
    LocalTacticNode::ProtectionRadius, 5699u16 => LocalTacticNode::RemainingTimeLessThan,
    5698u16 => LocalTacticNode::RemainingTimeMoreThan, 5707u16 =>
    LocalTacticNode::ResourceGuid, 5697u16 => LocalTacticNode::SelfTeamNotOwnNode,
    5694u16 => LocalTacticNode::SelfTeamNotOwnResource, 5696u16 =>
    LocalTacticNode::SelfTeamOwnNode, 5695u16 => LocalTacticNode::SelfTeamOwnResource,
    5894u16 => LocalTacticNode::SyncToResourceAvatar, 5706u16 =>
    LocalTacticNode::TacticPriority, 5705u16 => LocalTacticNode::TacticRadius, 5860u16 =>
    LocalTacticNode::TacticType,
};
impl Attribute for LocalTacticNode {
    fn class() -> Class {
        Class::LocalTacticNode
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
            Self::AwareOfResourceLocation => &Self::AwareOfResourceLocation,
            Self::Bias => &Self::Bias,
            Self::IsWayPoint => &Self::IsWayPoint,
            Self::MovementBehavior => &Self::MovementBehavior,
            Self::NotAwareOfResourceLocation => &Self::NotAwareOfResourceLocation,
            Self::PopulationMax => &Self::PopulationMax,
            Self::PopulationMin => &Self::PopulationMin,
            Self::ProtectionRadius => &Self::ProtectionRadius,
            Self::RemainingTimeLessThan => &Self::RemainingTimeLessThan,
            Self::RemainingTimeMoreThan => &Self::RemainingTimeMoreThan,
            Self::ResourceGuid => &Self::ResourceGuid,
            Self::SelfTeamNotOwnNode => &Self::SelfTeamNotOwnNode,
            Self::SelfTeamNotOwnResource => &Self::SelfTeamNotOwnResource,
            Self::SelfTeamOwnNode => &Self::SelfTeamOwnNode,
            Self::SelfTeamOwnResource => &Self::SelfTeamOwnResource,
            Self::SyncToResourceAvatar => &Self::SyncToResourceAvatar,
            Self::TacticPriority => &Self::TacticPriority,
            Self::TacticRadius => &Self::TacticRadius,
            Self::TacticType => &Self::TacticType,
        }
    }
}
impl AttributeInfo for LocalTacticNode {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 5741u16,
            Self::Action0Duration => 5740u16,
            Self::Action0Option => 5751u16,
            Self::AlwaysVisibleToPlayers => 5724u16,
            Self::AutoReviveDelay => 10564u16,
            Self::AutoReviveTime => 10504u16,
            Self::AwareRange => 8283u16,
            Self::BeaconRadius => 10975u16,
            Self::CollisionExtent => 5739u16,
            Self::ContentClass => 5743u16,
            Self::CycleQuestBase => 11061u16,
            Self::DefaultWeapon => 7249u16,
            Self::DespawnDelay => 9674u16,
            Self::Dialogs => 8870u16,
            Self::DisplayName => 6635u16,
            Self::EnableInGame => 6861u16,
            Self::FreedomProperties => 11185u16,
            Self::Freq => 5726u16,
            Self::GenerateInterestList => 5738u16,
            Self::HiddenFromClients => 5737u16,
            Self::HiddenFromPlayers => 5753u16,
            Self::HideAfterInteraction => 9168u16,
            Self::Icon => 5721u16,
            Self::InstanceTags => 5750u16,
            Self::InstanceZoneKey => 5710u16,
            Self::InteractionDuration => 11131u16,
            Self::InteractionRadius => 7510u16,
            Self::InteractionResetTimer => 9170u16,
            Self::IsNonSpawnedAvatar => 5763u16,
            Self::IsSelfRevivable => 7194u16,
            Self::LastInteractionTime => 9169u16,
            Self::LuaScript => 7818u16,
            Self::Lvl => 6218u16,
            Self::MaterialOverride => 5714u16,
            Self::Nodelink => 5752u16,
            Self::OriginalNodeName => 5761u16,
            Self::OriginalZoneName => 5760u16,
            Self::PartyGuid => 5736u16,
            Self::PathfindSafeSpawn => 5754u16,
            Self::Pos => 5735u16,
            Self::Power => 5727u16,
            Self::Priority => 5734u16,
            Self::QuestFlags => 9972u16,
            Self::ReadableName => 5723u16,
            Self::RespawnDelay => 5764u16,
            Self::RespawnRegionName => 10822u16,
            Self::RespawnRegionNameOverride => 10881u16,
            Self::Rot => 5733u16,
            Self::SelfRadius => 5732u16,
            Self::SpawnMethod => 6136u16,
            Self::SpawnPosition => 7873u16,
            Self::SpawnRotation => 8226u16,
            Self::Tags => 5731u16,
            Self::TeamId => 5730u16,
            Self::Ue3ClassId => 5742u16,
            Self::Ue3EdVisual => 9848u16,
            Self::VisibleOnQuestAvailable => 8746u16,
            Self::VisibleOnQuestComplete => 8743u16,
            Self::VisibleOnQuestFinished => 8744u16,
            Self::VisibleOnQuestInProgress => 8745u16,
            Self::WorldZoneObjectIndex => 5762u16,
            Self::Zone => 5728u16,
            Self::ZoneGuid => 5755u16,
            Self::AwareDist => 5758u16,
            Self::Defb => 5745u16,
            Self::InstanceGroup => 11382u16,
            Self::IsUnAttackable => 12440u16,
            Self::Abilities => 9343u16,
            Self::Alive => 5749u16,
            Self::AttackedBy => 5748u16,
            Self::CarrierGuid => 5756u16,
            Self::ClientLoadingPriority => 11284u16,
            Self::DirectorTags => 8100u16,
            Self::ForceSpawnOnClient => 5757u16,
            Self::HpCur => 5747u16,
            Self::HpMax => 5746u16,
            Self::IsLocked => 5711u16,
            Self::SpawnerAvatarGuid => 5979u16,
            Self::SpawnerAvatarId => 7705u16,
            Self::AwareOfResourceLocation => 5704u16,
            Self::Bias => 5708u16,
            Self::IsWayPoint => 5693u16,
            Self::MovementBehavior => 5825u16,
            Self::NotAwareOfResourceLocation => 5703u16,
            Self::PopulationMax => 5702u16,
            Self::PopulationMin => 5701u16,
            Self::ProtectionRadius => 5895u16,
            Self::RemainingTimeLessThan => 5699u16,
            Self::RemainingTimeMoreThan => 5698u16,
            Self::ResourceGuid => 5707u16,
            Self::SelfTeamNotOwnNode => 5697u16,
            Self::SelfTeamNotOwnResource => 5694u16,
            Self::SelfTeamOwnNode => 5696u16,
            Self::SelfTeamOwnResource => 5695u16,
            Self::SyncToResourceAvatar => 5894u16,
            Self::TacticPriority => 5706u16,
            Self::TacticRadius => 5705u16,
            Self::TacticType => 5860u16,
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
            Self::AwareOfResourceLocation => "AwareOfResourceLocation",
            Self::Bias => "Bias",
            Self::IsWayPoint => "IsWayPoint",
            Self::MovementBehavior => "MovementBehavior",
            Self::NotAwareOfResourceLocation => "NotAwareOfResourceLocation",
            Self::PopulationMax => "PopulationMax",
            Self::PopulationMin => "PopulationMin",
            Self::ProtectionRadius => "ProtectionRadius",
            Self::RemainingTimeLessThan => "RemainingTimeLessThan",
            Self::RemainingTimeMoreThan => "RemainingTimeMoreThan",
            Self::ResourceGuid => "ResourceGuid",
            Self::SelfTeamNotOwnNode => "SelfTeamNotOwnNode",
            Self::SelfTeamNotOwnResource => "SelfTeamNotOwnResource",
            Self::SelfTeamOwnNode => "SelfTeamOwnNode",
            Self::SelfTeamOwnResource => "SelfTeamOwnResource",
            Self::SyncToResourceAvatar => "SyncToResourceAvatar",
            Self::TacticPriority => "TacticPriority",
            Self::TacticRadius => "TacticRadius",
            Self::TacticType => "TacticType",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Defb => ParamType::String,
            Self::AwareOfResourceLocation => ParamType::Bool,
            Self::Bias => ParamType::Float,
            Self::IsWayPoint => ParamType::Bool,
            Self::MovementBehavior => ParamType::String,
            Self::NotAwareOfResourceLocation => ParamType::Bool,
            Self::PopulationMax => ParamType::Int,
            Self::PopulationMin => ParamType::Int,
            Self::ProtectionRadius => ParamType::Float,
            Self::RemainingTimeLessThan => ParamType::Float,
            Self::RemainingTimeMoreThan => ParamType::Float,
            Self::ResourceGuid => ParamType::Guid,
            Self::SelfTeamNotOwnNode => ParamType::Bool,
            Self::SelfTeamNotOwnResource => ParamType::Bool,
            Self::SelfTeamOwnNode => ParamType::Bool,
            Self::SelfTeamOwnResource => ParamType::Bool,
            Self::SyncToResourceAvatar => ParamType::Bool,
            Self::TacticPriority => ParamType::Float,
            Self::TacticRadius => ParamType::Float,
            Self::TacticType => ParamType::String,
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
            "LocalTacticNode".to_string(),
        ));
        static AWARE_OF_RESOURCE_LOCATION: Value = Value::Bool(false);
        static BIAS: Value = Value::Float(1f32);
        static IS_WAY_POINT: Value = Value::Bool(false);
        static MOVEMENT_BEHAVIOR: Lazy<Value> = Lazy::new(|| Value::String(
            "MoveToTarget".to_string(),
        ));
        static NOT_AWARE_OF_RESOURCE_LOCATION: Value = Value::Bool(false);
        static POPULATION_MAX: Value = Value::Int(50i32);
        static POPULATION_MIN: Value = Value::Int(-1i32);
        static PROTECTION_RADIUS: Value = Value::Float(2500f32);
        static REMAINING_TIME_LESS_THAN: Value = Value::Float(-1f32);
        static REMAINING_TIME_MORE_THAN: Value = Value::Float(-1f32);
        static RESOURCE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static SELF_TEAM_NOT_OWN_NODE: Value = Value::Bool(false);
        static SELF_TEAM_NOT_OWN_RESOURCE: Value = Value::Bool(false);
        static SELF_TEAM_OWN_NODE: Value = Value::Bool(false);
        static SELF_TEAM_OWN_RESOURCE: Value = Value::Bool(false);
        static SYNC_TO_RESOURCE_AVATAR: Value = Value::Bool(false);
        static TACTIC_PRIORITY: Value = Value::Float(1f32);
        static TACTIC_RADIUS: Value = Value::Float(250f32);
        static TACTIC_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            "MeleeDefense".to_string(),
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
            Self::AwareOfResourceLocation => &AWARE_OF_RESOURCE_LOCATION,
            Self::Bias => &BIAS,
            Self::IsWayPoint => &IS_WAY_POINT,
            Self::MovementBehavior => &MOVEMENT_BEHAVIOR,
            Self::NotAwareOfResourceLocation => &NOT_AWARE_OF_RESOURCE_LOCATION,
            Self::PopulationMax => &POPULATION_MAX,
            Self::PopulationMin => &POPULATION_MIN,
            Self::ProtectionRadius => &PROTECTION_RADIUS,
            Self::RemainingTimeLessThan => &REMAINING_TIME_LESS_THAN,
            Self::RemainingTimeMoreThan => &REMAINING_TIME_MORE_THAN,
            Self::ResourceGuid => &RESOURCE_GUID,
            Self::SelfTeamNotOwnNode => &SELF_TEAM_NOT_OWN_NODE,
            Self::SelfTeamNotOwnResource => &SELF_TEAM_NOT_OWN_RESOURCE,
            Self::SelfTeamOwnNode => &SELF_TEAM_OWN_NODE,
            Self::SelfTeamOwnResource => &SELF_TEAM_OWN_RESOURCE,
            Self::SyncToResourceAvatar => &SYNC_TO_RESOURCE_AVATAR,
            Self::TacticPriority => &TACTIC_PRIORITY,
            Self::TacticRadius => &TACTIC_RADIUS,
            Self::TacticType => &TACTIC_TYPE,
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
            Self::AwareOfResourceLocation => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Bias => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsWayPoint => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MovementBehavior => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NotAwareOfResourceLocation => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PopulationMax => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PopulationMin => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ProtectionRadius => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RemainingTimeLessThan => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RemainingTimeMoreThan => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ResourceGuid => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SelfTeamNotOwnNode => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SelfTeamNotOwnResource => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SelfTeamOwnNode => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SelfTeamOwnResource => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SyncToResourceAvatar => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TacticPriority => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TacticRadius => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TacticType => {
                &[
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
impl FromStr for LocalTacticNode {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LOCAL_TACTIC_NODE_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for LocalTacticNode {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5741u16 => Ok(Self::Action0),
            5740u16 => Ok(Self::Action0Duration),
            5751u16 => Ok(Self::Action0Option),
            5724u16 => Ok(Self::AlwaysVisibleToPlayers),
            10564u16 => Ok(Self::AutoReviveDelay),
            10504u16 => Ok(Self::AutoReviveTime),
            8283u16 => Ok(Self::AwareRange),
            10975u16 => Ok(Self::BeaconRadius),
            5739u16 => Ok(Self::CollisionExtent),
            5743u16 => Ok(Self::ContentClass),
            11061u16 => Ok(Self::CycleQuestBase),
            7249u16 => Ok(Self::DefaultWeapon),
            9674u16 => Ok(Self::DespawnDelay),
            8870u16 => Ok(Self::Dialogs),
            6635u16 => Ok(Self::DisplayName),
            6861u16 => Ok(Self::EnableInGame),
            11185u16 => Ok(Self::FreedomProperties),
            5726u16 => Ok(Self::Freq),
            5738u16 => Ok(Self::GenerateInterestList),
            5737u16 => Ok(Self::HiddenFromClients),
            5753u16 => Ok(Self::HiddenFromPlayers),
            9168u16 => Ok(Self::HideAfterInteraction),
            5721u16 => Ok(Self::Icon),
            5750u16 => Ok(Self::InstanceTags),
            5710u16 => Ok(Self::InstanceZoneKey),
            11131u16 => Ok(Self::InteractionDuration),
            7510u16 => Ok(Self::InteractionRadius),
            9170u16 => Ok(Self::InteractionResetTimer),
            5763u16 => Ok(Self::IsNonSpawnedAvatar),
            7194u16 => Ok(Self::IsSelfRevivable),
            9169u16 => Ok(Self::LastInteractionTime),
            7818u16 => Ok(Self::LuaScript),
            6218u16 => Ok(Self::Lvl),
            5714u16 => Ok(Self::MaterialOverride),
            5752u16 => Ok(Self::Nodelink),
            5761u16 => Ok(Self::OriginalNodeName),
            5760u16 => Ok(Self::OriginalZoneName),
            5736u16 => Ok(Self::PartyGuid),
            5754u16 => Ok(Self::PathfindSafeSpawn),
            5735u16 => Ok(Self::Pos),
            5727u16 => Ok(Self::Power),
            5734u16 => Ok(Self::Priority),
            9972u16 => Ok(Self::QuestFlags),
            5723u16 => Ok(Self::ReadableName),
            5764u16 => Ok(Self::RespawnDelay),
            10822u16 => Ok(Self::RespawnRegionName),
            10881u16 => Ok(Self::RespawnRegionNameOverride),
            5733u16 => Ok(Self::Rot),
            5732u16 => Ok(Self::SelfRadius),
            6136u16 => Ok(Self::SpawnMethod),
            7873u16 => Ok(Self::SpawnPosition),
            8226u16 => Ok(Self::SpawnRotation),
            5731u16 => Ok(Self::Tags),
            5730u16 => Ok(Self::TeamId),
            5742u16 => Ok(Self::Ue3ClassId),
            9848u16 => Ok(Self::Ue3EdVisual),
            8746u16 => Ok(Self::VisibleOnQuestAvailable),
            8743u16 => Ok(Self::VisibleOnQuestComplete),
            8744u16 => Ok(Self::VisibleOnQuestFinished),
            8745u16 => Ok(Self::VisibleOnQuestInProgress),
            5762u16 => Ok(Self::WorldZoneObjectIndex),
            5728u16 => Ok(Self::Zone),
            5755u16 => Ok(Self::ZoneGuid),
            5758u16 => Ok(Self::AwareDist),
            5745u16 => Ok(Self::Defb),
            11382u16 => Ok(Self::InstanceGroup),
            12440u16 => Ok(Self::IsUnAttackable),
            9343u16 => Ok(Self::Abilities),
            5749u16 => Ok(Self::Alive),
            5748u16 => Ok(Self::AttackedBy),
            5756u16 => Ok(Self::CarrierGuid),
            11284u16 => Ok(Self::ClientLoadingPriority),
            8100u16 => Ok(Self::DirectorTags),
            5757u16 => Ok(Self::ForceSpawnOnClient),
            5747u16 => Ok(Self::HpCur),
            5746u16 => Ok(Self::HpMax),
            5711u16 => Ok(Self::IsLocked),
            5979u16 => Ok(Self::SpawnerAvatarGuid),
            7705u16 => Ok(Self::SpawnerAvatarId),
            5704u16 => Ok(Self::AwareOfResourceLocation),
            5708u16 => Ok(Self::Bias),
            5693u16 => Ok(Self::IsWayPoint),
            5825u16 => Ok(Self::MovementBehavior),
            5703u16 => Ok(Self::NotAwareOfResourceLocation),
            5702u16 => Ok(Self::PopulationMax),
            5701u16 => Ok(Self::PopulationMin),
            5895u16 => Ok(Self::ProtectionRadius),
            5699u16 => Ok(Self::RemainingTimeLessThan),
            5698u16 => Ok(Self::RemainingTimeMoreThan),
            5707u16 => Ok(Self::ResourceGuid),
            5697u16 => Ok(Self::SelfTeamNotOwnNode),
            5694u16 => Ok(Self::SelfTeamNotOwnResource),
            5696u16 => Ok(Self::SelfTeamOwnNode),
            5695u16 => Ok(Self::SelfTeamOwnResource),
            5894u16 => Ok(Self::SyncToResourceAvatar),
            5706u16 => Ok(Self::TacticPriority),
            5705u16 => Ok(Self::TacticRadius),
            5860u16 => Ok(Self::TacticType),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
