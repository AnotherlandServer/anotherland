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
pub enum Portal {
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
    AccessConditions,
    ActivationDuration,
    CoolDownTimeLeft,
    CurrentEnergy,
    CurrentState,
    DestinationName,
    DestroyDuration,
    DestTexture,
    DoorKeeper,
    EnergyPerTransfer,
    ExitPoint,
    GatewayPositionRatio,
    HibernationThreshold,
    InteractRadius,
    IsShardObject,
    ItemsNeeded,
    LiveDuration,
    LocalPortalArrive,
    LocalPortalArriveEvent,
    MaxQueuingAvatar,
    OnArrivalApplyBuff,
    OnArrivalUnequipWeapon,
    Owner,
    PortalEnergyReserveCapacity,
    PortalLocationName,
    ProximityCheckDelay,
    ProximityCheckDistance,
    QueuingPlayers,
    QueuingRange,
    Range,
    RechargeRatePerSecond,
    TargetInstanceZoneKey,
    TargetPlacementGuid,
    TimePerTransfer,
    WorldPortalArrive,
    WorldPortalArriveEvent,
}
pub(crate) static PORTAL_ATTRIBUTES: phf::Map<&'static str, Portal> = phf_map! {
    "action0" => Portal::Action0, "action0Duration" => Portal::Action0Duration,
    "action0Option" => Portal::Action0Option, "alwaysVisibleToPlayers" =>
    Portal::AlwaysVisibleToPlayers, "autoReviveDelay" => Portal::AutoReviveDelay,
    "autoReviveTime" => Portal::AutoReviveTime, "AwareRange" => Portal::AwareRange,
    "BeaconRadius" => Portal::BeaconRadius, "collisionExtent" => Portal::CollisionExtent,
    "ContentClass" => Portal::ContentClass, "CycleQuestBase" => Portal::CycleQuestBase,
    "defaultWeapon" => Portal::DefaultWeapon, "despawnDelay" => Portal::DespawnDelay,
    "Dialogs" => Portal::Dialogs, "DisplayName" => Portal::DisplayName, "EnableInGame" =>
    Portal::EnableInGame, "FreedomProperties" => Portal::FreedomProperties, "Freq" =>
    Portal::Freq, "generateInterestList" => Portal::GenerateInterestList,
    "hiddenFromClients" => Portal::HiddenFromClients, "hiddenFromPlayers" =>
    Portal::HiddenFromPlayers, "HideAfterInteraction" => Portal::HideAfterInteraction,
    "Icon" => Portal::Icon, "instanceTags" => Portal::InstanceTags, "instanceZoneKey" =>
    Portal::InstanceZoneKey, "InteractionDuration" => Portal::InteractionDuration,
    "InteractionRadius" => Portal::InteractionRadius, "InteractionResetTimer" =>
    Portal::InteractionResetTimer, "isNonSpawnedAvatar" => Portal::IsNonSpawnedAvatar,
    "isSelfRevivable" => Portal::IsSelfRevivable, "LastInteractionTime" =>
    Portal::LastInteractionTime, "LuaScript" => Portal::LuaScript, "lvl" => Portal::Lvl,
    "MaterialOverride" => Portal::MaterialOverride, "nodelink" => Portal::Nodelink,
    "originalNodeName" => Portal::OriginalNodeName, "originalZoneName" =>
    Portal::OriginalZoneName, "partyGUID" => Portal::PartyGuid, "pathfindSafeSpawn" =>
    Portal::PathfindSafeSpawn, "pos" => Portal::Pos, "Power" => Portal::Power, "priority"
    => Portal::Priority, "QuestFlags" => Portal::QuestFlags, "ReadableName" =>
    Portal::ReadableName, "respawnDelay" => Portal::RespawnDelay, "RespawnRegionName" =>
    Portal::RespawnRegionName, "RespawnRegionNameOverride" =>
    Portal::RespawnRegionNameOverride, "rot" => Portal::Rot, "selfRadius" =>
    Portal::SelfRadius, "spawnMethod" => Portal::SpawnMethod, "spawnPosition" =>
    Portal::SpawnPosition, "spawnRotation" => Portal::SpawnRotation, "tags" =>
    Portal::Tags, "teamID" => Portal::TeamId, "UE3ClassID" => Portal::Ue3ClassId,
    "UE3EdVisual" => Portal::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    Portal::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    Portal::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    Portal::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    Portal::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    Portal::WorldZoneObjectIndex, "zone" => Portal::Zone, "ZoneGuid" => Portal::ZoneGuid,
    "awareDist" => Portal::AwareDist, "defb" => Portal::Defb, "instanceGroup" =>
    Portal::InstanceGroup, "isUnAttackable" => Portal::IsUnAttackable, "abilities" =>
    Portal::Abilities, "alive" => Portal::Alive, "attackedBy" => Portal::AttackedBy,
    "carrierGuid" => Portal::CarrierGuid, "clientLoadingPriority" =>
    Portal::ClientLoadingPriority, "directorTags" => Portal::DirectorTags,
    "forceSpawnOnClient" => Portal::ForceSpawnOnClient, "hpCur" => Portal::HpCur, "hpMax"
    => Portal::HpMax, "isLocked" => Portal::IsLocked, "spawnerAvatarGuid" =>
    Portal::SpawnerAvatarGuid, "spawnerAvatarID" => Portal::SpawnerAvatarId,
    "accessConditions" => Portal::AccessConditions, "ActivationDuration" =>
    Portal::ActivationDuration, "CoolDownTimeLeft" => Portal::CoolDownTimeLeft,
    "CurrentEnergy" => Portal::CurrentEnergy, "CurrentState" => Portal::CurrentState,
    "DestinationName" => Portal::DestinationName, "DestroyDuration" =>
    Portal::DestroyDuration, "destTexture" => Portal::DestTexture, "doorKeeper" =>
    Portal::DoorKeeper, "EnergyPerTransfer" => Portal::EnergyPerTransfer, "exitPoint" =>
    Portal::ExitPoint, "gatewayPositionRatio" => Portal::GatewayPositionRatio,
    "HibernationThreshold" => Portal::HibernationThreshold, "InteractRadius" =>
    Portal::InteractRadius, "isShardObject" => Portal::IsShardObject, "itemsNeeded" =>
    Portal::ItemsNeeded, "LiveDuration" => Portal::LiveDuration, "LocalPortalArrive" =>
    Portal::LocalPortalArrive, "LocalPortalArriveEvent" =>
    Portal::LocalPortalArriveEvent, "MaxQueuingAvatar" => Portal::MaxQueuingAvatar,
    "OnArrivalApplyBuff" => Portal::OnArrivalApplyBuff, "OnArrivalUnequipWeapon" =>
    Portal::OnArrivalUnequipWeapon, "owner" => Portal::Owner,
    "PortalEnergyReserveCapacity" => Portal::PortalEnergyReserveCapacity,
    "PortalLocationName" => Portal::PortalLocationName, "ProximityCheckDelay" =>
    Portal::ProximityCheckDelay, "ProximityCheckDistance" =>
    Portal::ProximityCheckDistance, "QueuingPlayers" => Portal::QueuingPlayers,
    "QueuingRange" => Portal::QueuingRange, "range" => Portal::Range,
    "RechargeRatePerSecond" => Portal::RechargeRatePerSecond, "targetInstanceZoneKey" =>
    Portal::TargetInstanceZoneKey, "targetPlacementGuid" => Portal::TargetPlacementGuid,
    "TimePerTransfer" => Portal::TimePerTransfer, "WorldPortalArrive" =>
    Portal::WorldPortalArrive, "WorldPortalArriveEvent" =>
    Portal::WorldPortalArriveEvent,
};
pub(crate) static PORTAL_ATTRIBUTES_ID: phf::Map<u16, Portal> = phf_map! {
    1806u16 => Portal::Action0, 1807u16 => Portal::Action0Duration, 1791u16 =>
    Portal::Action0Option, 1783u16 => Portal::AlwaysVisibleToPlayers, 10534u16 =>
    Portal::AutoReviveDelay, 10474u16 => Portal::AutoReviveTime, 8253u16 =>
    Portal::AwareRange, 10945u16 => Portal::BeaconRadius, 1808u16 =>
    Portal::CollisionExtent, 1804u16 => Portal::ContentClass, 11031u16 =>
    Portal::CycleQuestBase, 7220u16 => Portal::DefaultWeapon, 9644u16 =>
    Portal::DespawnDelay, 8840u16 => Portal::Dialogs, 6606u16 => Portal::DisplayName,
    6832u16 => Portal::EnableInGame, 11155u16 => Portal::FreedomProperties, 1821u16 =>
    Portal::Freq, 1809u16 => Portal::GenerateInterestList, 1810u16 =>
    Portal::HiddenFromClients, 1789u16 => Portal::HiddenFromPlayers, 9078u16 =>
    Portal::HideAfterInteraction, 4360u16 => Portal::Icon, 1792u16 =>
    Portal::InstanceTags, 5569u16 => Portal::InstanceZoneKey, 11101u16 =>
    Portal::InteractionDuration, 7481u16 => Portal::InteractionRadius, 9080u16 =>
    Portal::InteractionResetTimer, 1775u16 => Portal::IsNonSpawnedAvatar, 7165u16 =>
    Portal::IsSelfRevivable, 9079u16 => Portal::LastInteractionTime, 7788u16 =>
    Portal::LuaScript, 6189u16 => Portal::Lvl, 4739u16 => Portal::MaterialOverride,
    1790u16 => Portal::Nodelink, 1777u16 => Portal::OriginalNodeName, 1778u16 =>
    Portal::OriginalZoneName, 1811u16 => Portal::PartyGuid, 1788u16 =>
    Portal::PathfindSafeSpawn, 1812u16 => Portal::Pos, 1820u16 => Portal::Power, 1813u16
    => Portal::Priority, 9942u16 => Portal::QuestFlags, 3686u16 => Portal::ReadableName,
    1774u16 => Portal::RespawnDelay, 10792u16 => Portal::RespawnRegionName, 10851u16 =>
    Portal::RespawnRegionNameOverride, 1814u16 => Portal::Rot, 1815u16 =>
    Portal::SelfRadius, 6107u16 => Portal::SpawnMethod, 7843u16 => Portal::SpawnPosition,
    8196u16 => Portal::SpawnRotation, 1816u16 => Portal::Tags, 1817u16 => Portal::TeamId,
    1805u16 => Portal::Ue3ClassId, 9818u16 => Portal::Ue3EdVisual, 8626u16 =>
    Portal::VisibleOnQuestAvailable, 8623u16 => Portal::VisibleOnQuestComplete, 8624u16
    => Portal::VisibleOnQuestFinished, 8625u16 => Portal::VisibleOnQuestInProgress,
    1776u16 => Portal::WorldZoneObjectIndex, 1819u16 => Portal::Zone, 1785u16 =>
    Portal::ZoneGuid, 1780u16 => Portal::AwareDist, 1802u16 => Portal::Defb, 11352u16 =>
    Portal::InstanceGroup, 12420u16 => Portal::IsUnAttackable, 9313u16 =>
    Portal::Abilities, 1798u16 => Portal::Alive, 1799u16 => Portal::AttackedBy, 1782u16
    => Portal::CarrierGuid, 11254u16 => Portal::ClientLoadingPriority, 8070u16 =>
    Portal::DirectorTags, 1781u16 => Portal::ForceSpawnOnClient, 1800u16 =>
    Portal::HpCur, 1801u16 => Portal::HpMax, 5465u16 => Portal::IsLocked, 5950u16 =>
    Portal::SpawnerAvatarGuid, 7675u16 => Portal::SpawnerAvatarId, 8036u16 =>
    Portal::AccessConditions, 7731u16 => Portal::ActivationDuration, 6159u16 =>
    Portal::CoolDownTimeLeft, 6157u16 => Portal::CurrentEnergy, 6163u16 =>
    Portal::CurrentState, 1787u16 => Portal::DestinationName, 7771u16 =>
    Portal::DestroyDuration, 9210u16 => Portal::DestTexture, 8035u16 =>
    Portal::DoorKeeper, 6155u16 => Portal::EnergyPerTransfer, 9628u16 =>
    Portal::ExitPoint, 11008u16 => Portal::GatewayPositionRatio, 7732u16 =>
    Portal::HibernationThreshold, 10061u16 => Portal::InteractRadius, 1793u16 =>
    Portal::IsShardObject, 7525u16 => Portal::ItemsNeeded, 7772u16 =>
    Portal::LiveDuration, 5092u16 => Portal::LocalPortalArrive, 11012u16 =>
    Portal::LocalPortalArriveEvent, 6153u16 => Portal::MaxQueuingAvatar, 7885u16 =>
    Portal::OnArrivalApplyBuff, 7886u16 => Portal::OnArrivalUnequipWeapon, 1795u16 =>
    Portal::Owner, 6156u16 => Portal::PortalEnergyReserveCapacity, 10052u16 =>
    Portal::PortalLocationName, 10054u16 => Portal::ProximityCheckDelay, 10053u16 =>
    Portal::ProximityCheckDistance, 6158u16 => Portal::QueuingPlayers, 6162u16 =>
    Portal::QueuingRange, 1796u16 => Portal::Range, 6154u16 =>
    Portal::RechargeRatePerSecond, 10135u16 => Portal::TargetInstanceZoneKey, 10134u16 =>
    Portal::TargetPlacementGuid, 6152u16 => Portal::TimePerTransfer, 5091u16 =>
    Portal::WorldPortalArrive, 11013u16 => Portal::WorldPortalArriveEvent,
};
impl Attribute for Portal {
    fn class() -> Class {
        Class::Portal
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
            Self::AccessConditions => &Self::AccessConditions,
            Self::ActivationDuration => &Self::ActivationDuration,
            Self::CoolDownTimeLeft => &Self::CoolDownTimeLeft,
            Self::CurrentEnergy => &Self::CurrentEnergy,
            Self::CurrentState => &Self::CurrentState,
            Self::DestinationName => &Self::DestinationName,
            Self::DestroyDuration => &Self::DestroyDuration,
            Self::DestTexture => &Self::DestTexture,
            Self::DoorKeeper => &Self::DoorKeeper,
            Self::EnergyPerTransfer => &Self::EnergyPerTransfer,
            Self::ExitPoint => &Self::ExitPoint,
            Self::GatewayPositionRatio => &Self::GatewayPositionRatio,
            Self::HibernationThreshold => &Self::HibernationThreshold,
            Self::InteractRadius => &Self::InteractRadius,
            Self::IsShardObject => &Self::IsShardObject,
            Self::ItemsNeeded => &Self::ItemsNeeded,
            Self::LiveDuration => &Self::LiveDuration,
            Self::LocalPortalArrive => &Self::LocalPortalArrive,
            Self::LocalPortalArriveEvent => &Self::LocalPortalArriveEvent,
            Self::MaxQueuingAvatar => &Self::MaxQueuingAvatar,
            Self::OnArrivalApplyBuff => &Self::OnArrivalApplyBuff,
            Self::OnArrivalUnequipWeapon => &Self::OnArrivalUnequipWeapon,
            Self::Owner => &Self::Owner,
            Self::PortalEnergyReserveCapacity => &Self::PortalEnergyReserveCapacity,
            Self::PortalLocationName => &Self::PortalLocationName,
            Self::ProximityCheckDelay => &Self::ProximityCheckDelay,
            Self::ProximityCheckDistance => &Self::ProximityCheckDistance,
            Self::QueuingPlayers => &Self::QueuingPlayers,
            Self::QueuingRange => &Self::QueuingRange,
            Self::Range => &Self::Range,
            Self::RechargeRatePerSecond => &Self::RechargeRatePerSecond,
            Self::TargetInstanceZoneKey => &Self::TargetInstanceZoneKey,
            Self::TargetPlacementGuid => &Self::TargetPlacementGuid,
            Self::TimePerTransfer => &Self::TimePerTransfer,
            Self::WorldPortalArrive => &Self::WorldPortalArrive,
            Self::WorldPortalArriveEvent => &Self::WorldPortalArriveEvent,
        }
    }
}
impl AttributeInfo for Portal {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 1806u16,
            Self::Action0Duration => 1807u16,
            Self::Action0Option => 1791u16,
            Self::AlwaysVisibleToPlayers => 1783u16,
            Self::AutoReviveDelay => 10534u16,
            Self::AutoReviveTime => 10474u16,
            Self::AwareRange => 8253u16,
            Self::BeaconRadius => 10945u16,
            Self::CollisionExtent => 1808u16,
            Self::ContentClass => 1804u16,
            Self::CycleQuestBase => 11031u16,
            Self::DefaultWeapon => 7220u16,
            Self::DespawnDelay => 9644u16,
            Self::Dialogs => 8840u16,
            Self::DisplayName => 6606u16,
            Self::EnableInGame => 6832u16,
            Self::FreedomProperties => 11155u16,
            Self::Freq => 1821u16,
            Self::GenerateInterestList => 1809u16,
            Self::HiddenFromClients => 1810u16,
            Self::HiddenFromPlayers => 1789u16,
            Self::HideAfterInteraction => 9078u16,
            Self::Icon => 4360u16,
            Self::InstanceTags => 1792u16,
            Self::InstanceZoneKey => 5569u16,
            Self::InteractionDuration => 11101u16,
            Self::InteractionRadius => 7481u16,
            Self::InteractionResetTimer => 9080u16,
            Self::IsNonSpawnedAvatar => 1775u16,
            Self::IsSelfRevivable => 7165u16,
            Self::LastInteractionTime => 9079u16,
            Self::LuaScript => 7788u16,
            Self::Lvl => 6189u16,
            Self::MaterialOverride => 4739u16,
            Self::Nodelink => 1790u16,
            Self::OriginalNodeName => 1777u16,
            Self::OriginalZoneName => 1778u16,
            Self::PartyGuid => 1811u16,
            Self::PathfindSafeSpawn => 1788u16,
            Self::Pos => 1812u16,
            Self::Power => 1820u16,
            Self::Priority => 1813u16,
            Self::QuestFlags => 9942u16,
            Self::ReadableName => 3686u16,
            Self::RespawnDelay => 1774u16,
            Self::RespawnRegionName => 10792u16,
            Self::RespawnRegionNameOverride => 10851u16,
            Self::Rot => 1814u16,
            Self::SelfRadius => 1815u16,
            Self::SpawnMethod => 6107u16,
            Self::SpawnPosition => 7843u16,
            Self::SpawnRotation => 8196u16,
            Self::Tags => 1816u16,
            Self::TeamId => 1817u16,
            Self::Ue3ClassId => 1805u16,
            Self::Ue3EdVisual => 9818u16,
            Self::VisibleOnQuestAvailable => 8626u16,
            Self::VisibleOnQuestComplete => 8623u16,
            Self::VisibleOnQuestFinished => 8624u16,
            Self::VisibleOnQuestInProgress => 8625u16,
            Self::WorldZoneObjectIndex => 1776u16,
            Self::Zone => 1819u16,
            Self::ZoneGuid => 1785u16,
            Self::AwareDist => 1780u16,
            Self::Defb => 1802u16,
            Self::InstanceGroup => 11352u16,
            Self::IsUnAttackable => 12420u16,
            Self::Abilities => 9313u16,
            Self::Alive => 1798u16,
            Self::AttackedBy => 1799u16,
            Self::CarrierGuid => 1782u16,
            Self::ClientLoadingPriority => 11254u16,
            Self::DirectorTags => 8070u16,
            Self::ForceSpawnOnClient => 1781u16,
            Self::HpCur => 1800u16,
            Self::HpMax => 1801u16,
            Self::IsLocked => 5465u16,
            Self::SpawnerAvatarGuid => 5950u16,
            Self::SpawnerAvatarId => 7675u16,
            Self::AccessConditions => 8036u16,
            Self::ActivationDuration => 7731u16,
            Self::CoolDownTimeLeft => 6159u16,
            Self::CurrentEnergy => 6157u16,
            Self::CurrentState => 6163u16,
            Self::DestinationName => 1787u16,
            Self::DestroyDuration => 7771u16,
            Self::DestTexture => 9210u16,
            Self::DoorKeeper => 8035u16,
            Self::EnergyPerTransfer => 6155u16,
            Self::ExitPoint => 9628u16,
            Self::GatewayPositionRatio => 11008u16,
            Self::HibernationThreshold => 7732u16,
            Self::InteractRadius => 10061u16,
            Self::IsShardObject => 1793u16,
            Self::ItemsNeeded => 7525u16,
            Self::LiveDuration => 7772u16,
            Self::LocalPortalArrive => 5092u16,
            Self::LocalPortalArriveEvent => 11012u16,
            Self::MaxQueuingAvatar => 6153u16,
            Self::OnArrivalApplyBuff => 7885u16,
            Self::OnArrivalUnequipWeapon => 7886u16,
            Self::Owner => 1795u16,
            Self::PortalEnergyReserveCapacity => 6156u16,
            Self::PortalLocationName => 10052u16,
            Self::ProximityCheckDelay => 10054u16,
            Self::ProximityCheckDistance => 10053u16,
            Self::QueuingPlayers => 6158u16,
            Self::QueuingRange => 6162u16,
            Self::Range => 1796u16,
            Self::RechargeRatePerSecond => 6154u16,
            Self::TargetInstanceZoneKey => 10135u16,
            Self::TargetPlacementGuid => 10134u16,
            Self::TimePerTransfer => 6152u16,
            Self::WorldPortalArrive => 5091u16,
            Self::WorldPortalArriveEvent => 11013u16,
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
            Self::AccessConditions => "accessConditions",
            Self::ActivationDuration => "ActivationDuration",
            Self::CoolDownTimeLeft => "CoolDownTimeLeft",
            Self::CurrentEnergy => "CurrentEnergy",
            Self::CurrentState => "CurrentState",
            Self::DestinationName => "DestinationName",
            Self::DestroyDuration => "DestroyDuration",
            Self::DestTexture => "destTexture",
            Self::DoorKeeper => "doorKeeper",
            Self::EnergyPerTransfer => "EnergyPerTransfer",
            Self::ExitPoint => "exitPoint",
            Self::GatewayPositionRatio => "gatewayPositionRatio",
            Self::HibernationThreshold => "HibernationThreshold",
            Self::InteractRadius => "InteractRadius",
            Self::IsShardObject => "isShardObject",
            Self::ItemsNeeded => "itemsNeeded",
            Self::LiveDuration => "LiveDuration",
            Self::LocalPortalArrive => "LocalPortalArrive",
            Self::LocalPortalArriveEvent => "LocalPortalArriveEvent",
            Self::MaxQueuingAvatar => "MaxQueuingAvatar",
            Self::OnArrivalApplyBuff => "OnArrivalApplyBuff",
            Self::OnArrivalUnequipWeapon => "OnArrivalUnequipWeapon",
            Self::Owner => "owner",
            Self::PortalEnergyReserveCapacity => "PortalEnergyReserveCapacity",
            Self::PortalLocationName => "PortalLocationName",
            Self::ProximityCheckDelay => "ProximityCheckDelay",
            Self::ProximityCheckDistance => "ProximityCheckDistance",
            Self::QueuingPlayers => "QueuingPlayers",
            Self::QueuingRange => "QueuingRange",
            Self::Range => "range",
            Self::RechargeRatePerSecond => "RechargeRatePerSecond",
            Self::TargetInstanceZoneKey => "targetInstanceZoneKey",
            Self::TargetPlacementGuid => "targetPlacementGuid",
            Self::TimePerTransfer => "TimePerTransfer",
            Self::WorldPortalArrive => "WorldPortalArrive",
            Self::WorldPortalArriveEvent => "WorldPortalArriveEvent",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AlwaysVisibleToPlayers => ParamType::Bool,
            Self::CollisionExtent => ParamType::Vector3,
            Self::GenerateInterestList => ParamType::Bool,
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::Ue3EdVisual => ParamType::String,
            Self::AccessConditions => ParamType::JsonValue,
            Self::ActivationDuration => ParamType::Float,
            Self::CoolDownTimeLeft => ParamType::Float,
            Self::CurrentEnergy => ParamType::Float,
            Self::CurrentState => ParamType::Int,
            Self::DestinationName => ParamType::String,
            Self::DestroyDuration => ParamType::Float,
            Self::DestTexture => ParamType::StringFloatPair,
            Self::DoorKeeper => ParamType::String,
            Self::EnergyPerTransfer => ParamType::Float,
            Self::ExitPoint => ParamType::String,
            Self::GatewayPositionRatio => ParamType::Vector3,
            Self::HibernationThreshold => ParamType::Float,
            Self::InteractRadius => ParamType::Float,
            Self::IsShardObject => ParamType::Bool,
            Self::ItemsNeeded => ParamType::ContentRefList,
            Self::LiveDuration => ParamType::Float,
            Self::LocalPortalArrive => ParamType::String,
            Self::LocalPortalArriveEvent => ParamType::String,
            Self::MaxQueuingAvatar => ParamType::Int,
            Self::OnArrivalApplyBuff => ParamType::ContentRefList,
            Self::OnArrivalUnequipWeapon => ParamType::Bool,
            Self::Owner => ParamType::String,
            Self::PortalEnergyReserveCapacity => ParamType::Float,
            Self::PortalLocationName => ParamType::String,
            Self::ProximityCheckDelay => ParamType::Int,
            Self::ProximityCheckDistance => ParamType::Int,
            Self::QueuingPlayers => ParamType::JsonValue,
            Self::QueuingRange => ParamType::Float,
            Self::Range => ParamType::Float,
            Self::RechargeRatePerSecond => ParamType::Float,
            Self::TargetInstanceZoneKey => ParamType::String,
            Self::TargetPlacementGuid => ParamType::Guid,
            Self::TimePerTransfer => ParamType::Float,
            Self::WorldPortalArrive => ParamType::String,
            Self::WorldPortalArriveEvent => ParamType::String,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::Action0Option => ParamType::Int,
            Self::AutoReviveDelay => ParamType::Float,
            Self::AutoReviveTime => ParamType::Int64,
            Self::AwareRange => ParamType::Float,
            Self::BeaconRadius => ParamType::Int,
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
        static ALWAYS_VISIBLE_TO_PLAYERS: Value = Value::Bool(false);
        static COLLISION_EXTENT: Value = Value::Vector3(Vec3::new(21f32, 21f32, 44f32));
        static GENERATE_INTEREST_LIST: Value = Value::Bool(true);
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(true);
        static UE_3_ED_VISUAL: Lazy<Value> = Lazy::new(|| Value::String(
            "AmunEditorIcons.Portal".to_string(),
        ));
        static ACCESS_CONDITIONS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[]").unwrap(),
        ));
        static ACTIVATION_DURATION: Value = Value::Float(10f32);
        static COOL_DOWN_TIME_LEFT: Value = Value::Float(0f32);
        static CURRENT_ENERGY: Value = Value::Float(0f32);
        static CURRENT_STATE: Value = Value::Int(0i32);
        static DESTINATION_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static DESTROY_DURATION: Value = Value::Float(10f32);
        static DEST_TEXTURE: Lazy<Value> = Lazy::new(|| Value::StringFloatPair((
            String::default(),
            0.0,
        )));
        static DOOR_KEEPER: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ENERGY_PER_TRANSFER: Value = Value::Float(10f32);
        static EXIT_POINT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static GATEWAY_POSITION_RATIO: Value = Value::Vector3(
            Vec3::new(0.75f32, 0f32, 0f32),
        );
        static HIBERNATION_THRESHOLD: Value = Value::Float(
            100000000000000000000000000000f32,
        );
        static INTERACT_RADIUS: Value = Value::Float(10f32);
        static IS_SHARD_OBJECT: Value = Value::Bool(true);
        static ITEMS_NEEDED: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static LIVE_DURATION: Value = Value::Float(30f32);
        static LOCAL_PORTAL_ARRIVE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static LOCAL_PORTAL_ARRIVE_EVENT: Lazy<Value> = Lazy::new(|| Value::String(
            "PortalArriveDefault".to_string(),
        ));
        static MAX_QUEUING_AVATAR: Value = Value::Int(8i32);
        static ON_ARRIVAL_APPLY_BUFF: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            "[9:2769e15a-56cc-45ee-835a-17660fa44e50]".parse().unwrap_or_default(),
        ));
        static ON_ARRIVAL_UNEQUIP_WEAPON: Value = Value::Bool(false);
        static OWNER: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static PORTAL_ENERGY_RESERVE_CAPACITY: Value = Value::Float(400f32);
        static PORTAL_LOCATION_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PROXIMITY_CHECK_DELAY: Value = Value::Int(8i32);
        static PROXIMITY_CHECK_DISTANCE: Value = Value::Int(8i32);
        static QUEUING_PLAYERS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static QUEUING_RANGE: Value = Value::Float(500f32);
        static RANGE: Value = Value::Float(2f32);
        static RECHARGE_RATE_PER_SECOND: Value = Value::Float(20f32);
        static TARGET_INSTANCE_ZONE_KEY: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static TARGET_PLACEMENT_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static TIME_PER_TRANSFER: Value = Value::Float(10f32);
        static WORLD_PORTAL_ARRIVE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static WORLD_PORTAL_ARRIVE_EVENT: Lazy<Value> = Lazy::new(|| Value::String(
            "PortalArriveDefault".to_string(),
        ));
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
            Self::AlwaysVisibleToPlayers => &ALWAYS_VISIBLE_TO_PLAYERS,
            Self::CollisionExtent => &COLLISION_EXTENT,
            Self::GenerateInterestList => &GENERATE_INTEREST_LIST,
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::AccessConditions => &ACCESS_CONDITIONS,
            Self::ActivationDuration => &ACTIVATION_DURATION,
            Self::CoolDownTimeLeft => &COOL_DOWN_TIME_LEFT,
            Self::CurrentEnergy => &CURRENT_ENERGY,
            Self::CurrentState => &CURRENT_STATE,
            Self::DestinationName => &DESTINATION_NAME,
            Self::DestroyDuration => &DESTROY_DURATION,
            Self::DestTexture => &DEST_TEXTURE,
            Self::DoorKeeper => &DOOR_KEEPER,
            Self::EnergyPerTransfer => &ENERGY_PER_TRANSFER,
            Self::ExitPoint => &EXIT_POINT,
            Self::GatewayPositionRatio => &GATEWAY_POSITION_RATIO,
            Self::HibernationThreshold => &HIBERNATION_THRESHOLD,
            Self::InteractRadius => &INTERACT_RADIUS,
            Self::IsShardObject => &IS_SHARD_OBJECT,
            Self::ItemsNeeded => &ITEMS_NEEDED,
            Self::LiveDuration => &LIVE_DURATION,
            Self::LocalPortalArrive => &LOCAL_PORTAL_ARRIVE,
            Self::LocalPortalArriveEvent => &LOCAL_PORTAL_ARRIVE_EVENT,
            Self::MaxQueuingAvatar => &MAX_QUEUING_AVATAR,
            Self::OnArrivalApplyBuff => &ON_ARRIVAL_APPLY_BUFF,
            Self::OnArrivalUnequipWeapon => &ON_ARRIVAL_UNEQUIP_WEAPON,
            Self::Owner => &OWNER,
            Self::PortalEnergyReserveCapacity => &PORTAL_ENERGY_RESERVE_CAPACITY,
            Self::PortalLocationName => &PORTAL_LOCATION_NAME,
            Self::ProximityCheckDelay => &PROXIMITY_CHECK_DELAY,
            Self::ProximityCheckDistance => &PROXIMITY_CHECK_DISTANCE,
            Self::QueuingPlayers => &QUEUING_PLAYERS,
            Self::QueuingRange => &QUEUING_RANGE,
            Self::Range => &RANGE,
            Self::RechargeRatePerSecond => &RECHARGE_RATE_PER_SECOND,
            Self::TargetInstanceZoneKey => &TARGET_INSTANCE_ZONE_KEY,
            Self::TargetPlacementGuid => &TARGET_PLACEMENT_GUID,
            Self::TimePerTransfer => &TIME_PER_TRANSFER,
            Self::WorldPortalArrive => &WORLD_PORTAL_ARRIVE,
            Self::WorldPortalArriveEvent => &WORLD_PORTAL_ARRIVE_EVENT,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::Action0Option => &ACTION_0_OPTION,
            Self::AutoReviveDelay => &AUTO_REVIVE_DELAY,
            Self::AutoReviveTime => &AUTO_REVIVE_TIME,
            Self::AwareRange => &AWARE_RANGE,
            Self::BeaconRadius => &BEACON_RADIUS,
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
            Self::AlwaysVisibleToPlayers => {
                &[
                    ParamFlag::NodeOwn,
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
            Self::GenerateInterestList => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::PathfindSafeSpawn => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Ue3EdVisual => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
            Self::AccessConditions => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ActivationDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CoolDownTimeLeft => &[ParamFlag::NodeOwn],
            Self::CurrentEnergy => &[ParamFlag::NodeOwn],
            Self::CurrentState => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::DestinationName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DestroyDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DestTexture => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DoorKeeper => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EnergyPerTransfer => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ExitPoint => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::GatewayPositionRatio => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HibernationThreshold => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InteractRadius => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsShardObject => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ItemsNeeded => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LiveDuration => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::LocalPortalArrive => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::LocalPortalArriveEvent => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MaxQueuingAvatar => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::OnArrivalApplyBuff => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::OnArrivalUnequipWeapon => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Owner => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PortalEnergyReserveCapacity => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::PortalLocationName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ProximityCheckDelay => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ProximityCheckDistance => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::QueuingPlayers => &[ParamFlag::NodeOwn],
            Self::QueuingRange => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Range => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RechargeRatePerSecond => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TargetInstanceZoneKey => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::TargetPlacementGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::TimePerTransfer => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::WorldPortalArrive => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::WorldPortalArriveEvent => {
                &[
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
impl FromStr for Portal {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PORTAL_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Portal {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            1806u16 => Ok(Self::Action0),
            1807u16 => Ok(Self::Action0Duration),
            1791u16 => Ok(Self::Action0Option),
            1783u16 => Ok(Self::AlwaysVisibleToPlayers),
            10534u16 => Ok(Self::AutoReviveDelay),
            10474u16 => Ok(Self::AutoReviveTime),
            8253u16 => Ok(Self::AwareRange),
            10945u16 => Ok(Self::BeaconRadius),
            1808u16 => Ok(Self::CollisionExtent),
            1804u16 => Ok(Self::ContentClass),
            11031u16 => Ok(Self::CycleQuestBase),
            7220u16 => Ok(Self::DefaultWeapon),
            9644u16 => Ok(Self::DespawnDelay),
            8840u16 => Ok(Self::Dialogs),
            6606u16 => Ok(Self::DisplayName),
            6832u16 => Ok(Self::EnableInGame),
            11155u16 => Ok(Self::FreedomProperties),
            1821u16 => Ok(Self::Freq),
            1809u16 => Ok(Self::GenerateInterestList),
            1810u16 => Ok(Self::HiddenFromClients),
            1789u16 => Ok(Self::HiddenFromPlayers),
            9078u16 => Ok(Self::HideAfterInteraction),
            4360u16 => Ok(Self::Icon),
            1792u16 => Ok(Self::InstanceTags),
            5569u16 => Ok(Self::InstanceZoneKey),
            11101u16 => Ok(Self::InteractionDuration),
            7481u16 => Ok(Self::InteractionRadius),
            9080u16 => Ok(Self::InteractionResetTimer),
            1775u16 => Ok(Self::IsNonSpawnedAvatar),
            7165u16 => Ok(Self::IsSelfRevivable),
            9079u16 => Ok(Self::LastInteractionTime),
            7788u16 => Ok(Self::LuaScript),
            6189u16 => Ok(Self::Lvl),
            4739u16 => Ok(Self::MaterialOverride),
            1790u16 => Ok(Self::Nodelink),
            1777u16 => Ok(Self::OriginalNodeName),
            1778u16 => Ok(Self::OriginalZoneName),
            1811u16 => Ok(Self::PartyGuid),
            1788u16 => Ok(Self::PathfindSafeSpawn),
            1812u16 => Ok(Self::Pos),
            1820u16 => Ok(Self::Power),
            1813u16 => Ok(Self::Priority),
            9942u16 => Ok(Self::QuestFlags),
            3686u16 => Ok(Self::ReadableName),
            1774u16 => Ok(Self::RespawnDelay),
            10792u16 => Ok(Self::RespawnRegionName),
            10851u16 => Ok(Self::RespawnRegionNameOverride),
            1814u16 => Ok(Self::Rot),
            1815u16 => Ok(Self::SelfRadius),
            6107u16 => Ok(Self::SpawnMethod),
            7843u16 => Ok(Self::SpawnPosition),
            8196u16 => Ok(Self::SpawnRotation),
            1816u16 => Ok(Self::Tags),
            1817u16 => Ok(Self::TeamId),
            1805u16 => Ok(Self::Ue3ClassId),
            9818u16 => Ok(Self::Ue3EdVisual),
            8626u16 => Ok(Self::VisibleOnQuestAvailable),
            8623u16 => Ok(Self::VisibleOnQuestComplete),
            8624u16 => Ok(Self::VisibleOnQuestFinished),
            8625u16 => Ok(Self::VisibleOnQuestInProgress),
            1776u16 => Ok(Self::WorldZoneObjectIndex),
            1819u16 => Ok(Self::Zone),
            1785u16 => Ok(Self::ZoneGuid),
            1780u16 => Ok(Self::AwareDist),
            1802u16 => Ok(Self::Defb),
            11352u16 => Ok(Self::InstanceGroup),
            12420u16 => Ok(Self::IsUnAttackable),
            9313u16 => Ok(Self::Abilities),
            1798u16 => Ok(Self::Alive),
            1799u16 => Ok(Self::AttackedBy),
            1782u16 => Ok(Self::CarrierGuid),
            11254u16 => Ok(Self::ClientLoadingPriority),
            8070u16 => Ok(Self::DirectorTags),
            1781u16 => Ok(Self::ForceSpawnOnClient),
            1800u16 => Ok(Self::HpCur),
            1801u16 => Ok(Self::HpMax),
            5465u16 => Ok(Self::IsLocked),
            5950u16 => Ok(Self::SpawnerAvatarGuid),
            7675u16 => Ok(Self::SpawnerAvatarId),
            8036u16 => Ok(Self::AccessConditions),
            7731u16 => Ok(Self::ActivationDuration),
            6159u16 => Ok(Self::CoolDownTimeLeft),
            6157u16 => Ok(Self::CurrentEnergy),
            6163u16 => Ok(Self::CurrentState),
            1787u16 => Ok(Self::DestinationName),
            7771u16 => Ok(Self::DestroyDuration),
            9210u16 => Ok(Self::DestTexture),
            8035u16 => Ok(Self::DoorKeeper),
            6155u16 => Ok(Self::EnergyPerTransfer),
            9628u16 => Ok(Self::ExitPoint),
            11008u16 => Ok(Self::GatewayPositionRatio),
            7732u16 => Ok(Self::HibernationThreshold),
            10061u16 => Ok(Self::InteractRadius),
            1793u16 => Ok(Self::IsShardObject),
            7525u16 => Ok(Self::ItemsNeeded),
            7772u16 => Ok(Self::LiveDuration),
            5092u16 => Ok(Self::LocalPortalArrive),
            11012u16 => Ok(Self::LocalPortalArriveEvent),
            6153u16 => Ok(Self::MaxQueuingAvatar),
            7885u16 => Ok(Self::OnArrivalApplyBuff),
            7886u16 => Ok(Self::OnArrivalUnequipWeapon),
            1795u16 => Ok(Self::Owner),
            6156u16 => Ok(Self::PortalEnergyReserveCapacity),
            10052u16 => Ok(Self::PortalLocationName),
            10054u16 => Ok(Self::ProximityCheckDelay),
            10053u16 => Ok(Self::ProximityCheckDistance),
            6158u16 => Ok(Self::QueuingPlayers),
            6162u16 => Ok(Self::QueuingRange),
            1796u16 => Ok(Self::Range),
            6154u16 => Ok(Self::RechargeRatePerSecond),
            10135u16 => Ok(Self::TargetInstanceZoneKey),
            10134u16 => Ok(Self::TargetPlacementGuid),
            6152u16 => Ok(Self::TimePerTransfer),
            5091u16 => Ok(Self::WorldPortalArrive),
            11013u16 => Ok(Self::WorldPortalArriveEvent),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
