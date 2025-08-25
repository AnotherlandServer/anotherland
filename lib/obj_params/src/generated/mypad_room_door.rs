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
pub enum MypadRoomDoor {
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
    DoorIndex,
}
pub(crate) static MYPAD_ROOM_DOOR_ATTRIBUTES: phf::Map<&'static str, MypadRoomDoor> = phf_map! {
    "action0" => MypadRoomDoor::Action0, "action0Duration" =>
    MypadRoomDoor::Action0Duration, "action0Option" => MypadRoomDoor::Action0Option,
    "alwaysVisibleToPlayers" => MypadRoomDoor::AlwaysVisibleToPlayers, "autoReviveDelay"
    => MypadRoomDoor::AutoReviveDelay, "autoReviveTime" => MypadRoomDoor::AutoReviveTime,
    "AwareRange" => MypadRoomDoor::AwareRange, "BeaconRadius" =>
    MypadRoomDoor::BeaconRadius, "collisionExtent" => MypadRoomDoor::CollisionExtent,
    "ContentClass" => MypadRoomDoor::ContentClass, "CycleQuestBase" =>
    MypadRoomDoor::CycleQuestBase, "defaultWeapon" => MypadRoomDoor::DefaultWeapon,
    "despawnDelay" => MypadRoomDoor::DespawnDelay, "Dialogs" => MypadRoomDoor::Dialogs,
    "DisplayName" => MypadRoomDoor::DisplayName, "EnableInGame" =>
    MypadRoomDoor::EnableInGame, "FreedomProperties" => MypadRoomDoor::FreedomProperties,
    "Freq" => MypadRoomDoor::Freq, "generateInterestList" =>
    MypadRoomDoor::GenerateInterestList, "hiddenFromClients" =>
    MypadRoomDoor::HiddenFromClients, "hiddenFromPlayers" =>
    MypadRoomDoor::HiddenFromPlayers, "HideAfterInteraction" =>
    MypadRoomDoor::HideAfterInteraction, "Icon" => MypadRoomDoor::Icon, "instanceTags" =>
    MypadRoomDoor::InstanceTags, "instanceZoneKey" => MypadRoomDoor::InstanceZoneKey,
    "InteractionDuration" => MypadRoomDoor::InteractionDuration, "InteractionRadius" =>
    MypadRoomDoor::InteractionRadius, "InteractionResetTimer" =>
    MypadRoomDoor::InteractionResetTimer, "isNonSpawnedAvatar" =>
    MypadRoomDoor::IsNonSpawnedAvatar, "isSelfRevivable" =>
    MypadRoomDoor::IsSelfRevivable, "LastInteractionTime" =>
    MypadRoomDoor::LastInteractionTime, "LuaScript" => MypadRoomDoor::LuaScript, "lvl" =>
    MypadRoomDoor::Lvl, "MaterialOverride" => MypadRoomDoor::MaterialOverride, "nodelink"
    => MypadRoomDoor::Nodelink, "originalNodeName" => MypadRoomDoor::OriginalNodeName,
    "originalZoneName" => MypadRoomDoor::OriginalZoneName, "partyGUID" =>
    MypadRoomDoor::PartyGuid, "pathfindSafeSpawn" => MypadRoomDoor::PathfindSafeSpawn,
    "pos" => MypadRoomDoor::Pos, "Power" => MypadRoomDoor::Power, "priority" =>
    MypadRoomDoor::Priority, "QuestFlags" => MypadRoomDoor::QuestFlags, "ReadableName" =>
    MypadRoomDoor::ReadableName, "respawnDelay" => MypadRoomDoor::RespawnDelay,
    "RespawnRegionName" => MypadRoomDoor::RespawnRegionName, "RespawnRegionNameOverride"
    => MypadRoomDoor::RespawnRegionNameOverride, "rot" => MypadRoomDoor::Rot,
    "selfRadius" => MypadRoomDoor::SelfRadius, "spawnMethod" =>
    MypadRoomDoor::SpawnMethod, "spawnPosition" => MypadRoomDoor::SpawnPosition,
    "spawnRotation" => MypadRoomDoor::SpawnRotation, "tags" => MypadRoomDoor::Tags,
    "teamID" => MypadRoomDoor::TeamId, "UE3ClassID" => MypadRoomDoor::Ue3ClassId,
    "UE3EdVisual" => MypadRoomDoor::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    MypadRoomDoor::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    MypadRoomDoor::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    MypadRoomDoor::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    MypadRoomDoor::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    MypadRoomDoor::WorldZoneObjectIndex, "zone" => MypadRoomDoor::Zone, "ZoneGuid" =>
    MypadRoomDoor::ZoneGuid, "awareDist" => MypadRoomDoor::AwareDist, "defb" =>
    MypadRoomDoor::Defb, "instanceGroup" => MypadRoomDoor::InstanceGroup,
    "isUnAttackable" => MypadRoomDoor::IsUnAttackable, "abilities" =>
    MypadRoomDoor::Abilities, "alive" => MypadRoomDoor::Alive, "attackedBy" =>
    MypadRoomDoor::AttackedBy, "carrierGuid" => MypadRoomDoor::CarrierGuid,
    "clientLoadingPriority" => MypadRoomDoor::ClientLoadingPriority, "directorTags" =>
    MypadRoomDoor::DirectorTags, "forceSpawnOnClient" =>
    MypadRoomDoor::ForceSpawnOnClient, "hpCur" => MypadRoomDoor::HpCur, "hpMax" =>
    MypadRoomDoor::HpMax, "isLocked" => MypadRoomDoor::IsLocked, "spawnerAvatarGuid" =>
    MypadRoomDoor::SpawnerAvatarGuid, "spawnerAvatarID" =>
    MypadRoomDoor::SpawnerAvatarId, "doorIndex" => MypadRoomDoor::DoorIndex,
};
pub(crate) static MYPAD_ROOM_DOOR_ATTRIBUTES_ID: phf::Map<u16, MypadRoomDoor> = phf_map! {
    9777u16 => MypadRoomDoor::Action0, 9776u16 => MypadRoomDoor::Action0Duration, 9787u16
    => MypadRoomDoor::Action0Option, 9760u16 => MypadRoomDoor::AlwaysVisibleToPlayers,
    10567u16 => MypadRoomDoor::AutoReviveDelay, 10507u16 =>
    MypadRoomDoor::AutoReviveTime, 9730u16 => MypadRoomDoor::AwareRange, 10976u16 =>
    MypadRoomDoor::BeaconRadius, 9775u16 => MypadRoomDoor::CollisionExtent, 9779u16 =>
    MypadRoomDoor::ContentClass, 11062u16 => MypadRoomDoor::CycleQuestBase, 9737u16 =>
    MypadRoomDoor::DefaultWeapon, 9720u16 => MypadRoomDoor::DespawnDelay, 9725u16 =>
    MypadRoomDoor::Dialogs, 9740u16 => MypadRoomDoor::DisplayName, 9739u16 =>
    MypadRoomDoor::EnableInGame, 11186u16 => MypadRoomDoor::FreedomProperties, 9762u16 =>
    MypadRoomDoor::Freq, 9774u16 => MypadRoomDoor::GenerateInterestList, 9773u16 =>
    MypadRoomDoor::HiddenFromClients, 9789u16 => MypadRoomDoor::HiddenFromPlayers,
    9724u16 => MypadRoomDoor::HideAfterInteraction, 9757u16 => MypadRoomDoor::Icon,
    9786u16 => MypadRoomDoor::InstanceTags, 9746u16 => MypadRoomDoor::InstanceZoneKey,
    11132u16 => MypadRoomDoor::InteractionDuration, 9736u16 =>
    MypadRoomDoor::InteractionRadius, 9722u16 => MypadRoomDoor::InteractionResetTimer,
    9799u16 => MypadRoomDoor::IsNonSpawnedAvatar, 9738u16 =>
    MypadRoomDoor::IsSelfRevivable, 9723u16 => MypadRoomDoor::LastInteractionTime,
    9734u16 => MypadRoomDoor::LuaScript, 9741u16 => MypadRoomDoor::Lvl, 9750u16 =>
    MypadRoomDoor::MaterialOverride, 9788u16 => MypadRoomDoor::Nodelink, 9797u16 =>
    MypadRoomDoor::OriginalNodeName, 9796u16 => MypadRoomDoor::OriginalZoneName, 9772u16
    => MypadRoomDoor::PartyGuid, 9790u16 => MypadRoomDoor::PathfindSafeSpawn, 9771u16 =>
    MypadRoomDoor::Pos, 9763u16 => MypadRoomDoor::Power, 9770u16 =>
    MypadRoomDoor::Priority, 9975u16 => MypadRoomDoor::QuestFlags, 9759u16 =>
    MypadRoomDoor::ReadableName, 9800u16 => MypadRoomDoor::RespawnDelay, 10823u16 =>
    MypadRoomDoor::RespawnRegionName, 10882u16 =>
    MypadRoomDoor::RespawnRegionNameOverride, 9769u16 => MypadRoomDoor::Rot, 9768u16 =>
    MypadRoomDoor::SelfRadius, 9742u16 => MypadRoomDoor::SpawnMethod, 9733u16 =>
    MypadRoomDoor::SpawnPosition, 9731u16 => MypadRoomDoor::SpawnRotation, 9767u16 =>
    MypadRoomDoor::Tags, 9766u16 => MypadRoomDoor::TeamId, 9778u16 =>
    MypadRoomDoor::Ue3ClassId, 9851u16 => MypadRoomDoor::Ue3EdVisual, 9726u16 =>
    MypadRoomDoor::VisibleOnQuestAvailable, 9729u16 =>
    MypadRoomDoor::VisibleOnQuestComplete, 9728u16 =>
    MypadRoomDoor::VisibleOnQuestFinished, 9727u16 =>
    MypadRoomDoor::VisibleOnQuestInProgress, 9798u16 =>
    MypadRoomDoor::WorldZoneObjectIndex, 9764u16 => MypadRoomDoor::Zone, 9791u16 =>
    MypadRoomDoor::ZoneGuid, 9794u16 => MypadRoomDoor::AwareDist, 9781u16 =>
    MypadRoomDoor::Defb, 11385u16 => MypadRoomDoor::InstanceGroup, 12443u16 =>
    MypadRoomDoor::IsUnAttackable, 9721u16 => MypadRoomDoor::Abilities, 9785u16 =>
    MypadRoomDoor::Alive, 9784u16 => MypadRoomDoor::AttackedBy, 9792u16 =>
    MypadRoomDoor::CarrierGuid, 11285u16 => MypadRoomDoor::ClientLoadingPriority, 9732u16
    => MypadRoomDoor::DirectorTags, 9793u16 => MypadRoomDoor::ForceSpawnOnClient, 9783u16
    => MypadRoomDoor::HpCur, 9782u16 => MypadRoomDoor::HpMax, 9747u16 =>
    MypadRoomDoor::IsLocked, 9743u16 => MypadRoomDoor::SpawnerAvatarGuid, 9735u16 =>
    MypadRoomDoor::SpawnerAvatarId, 9719u16 => MypadRoomDoor::DoorIndex,
};
impl Attribute for MypadRoomDoor {
    fn class() -> Class {
        Class::MypadRoomDoor
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
            Self::DoorIndex => &Self::DoorIndex,
        }
    }
}
impl AttributeInfo for MypadRoomDoor {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 9777u16,
            Self::Action0Duration => 9776u16,
            Self::Action0Option => 9787u16,
            Self::AlwaysVisibleToPlayers => 9760u16,
            Self::AutoReviveDelay => 10567u16,
            Self::AutoReviveTime => 10507u16,
            Self::AwareRange => 9730u16,
            Self::BeaconRadius => 10976u16,
            Self::CollisionExtent => 9775u16,
            Self::ContentClass => 9779u16,
            Self::CycleQuestBase => 11062u16,
            Self::DefaultWeapon => 9737u16,
            Self::DespawnDelay => 9720u16,
            Self::Dialogs => 9725u16,
            Self::DisplayName => 9740u16,
            Self::EnableInGame => 9739u16,
            Self::FreedomProperties => 11186u16,
            Self::Freq => 9762u16,
            Self::GenerateInterestList => 9774u16,
            Self::HiddenFromClients => 9773u16,
            Self::HiddenFromPlayers => 9789u16,
            Self::HideAfterInteraction => 9724u16,
            Self::Icon => 9757u16,
            Self::InstanceTags => 9786u16,
            Self::InstanceZoneKey => 9746u16,
            Self::InteractionDuration => 11132u16,
            Self::InteractionRadius => 9736u16,
            Self::InteractionResetTimer => 9722u16,
            Self::IsNonSpawnedAvatar => 9799u16,
            Self::IsSelfRevivable => 9738u16,
            Self::LastInteractionTime => 9723u16,
            Self::LuaScript => 9734u16,
            Self::Lvl => 9741u16,
            Self::MaterialOverride => 9750u16,
            Self::Nodelink => 9788u16,
            Self::OriginalNodeName => 9797u16,
            Self::OriginalZoneName => 9796u16,
            Self::PartyGuid => 9772u16,
            Self::PathfindSafeSpawn => 9790u16,
            Self::Pos => 9771u16,
            Self::Power => 9763u16,
            Self::Priority => 9770u16,
            Self::QuestFlags => 9975u16,
            Self::ReadableName => 9759u16,
            Self::RespawnDelay => 9800u16,
            Self::RespawnRegionName => 10823u16,
            Self::RespawnRegionNameOverride => 10882u16,
            Self::Rot => 9769u16,
            Self::SelfRadius => 9768u16,
            Self::SpawnMethod => 9742u16,
            Self::SpawnPosition => 9733u16,
            Self::SpawnRotation => 9731u16,
            Self::Tags => 9767u16,
            Self::TeamId => 9766u16,
            Self::Ue3ClassId => 9778u16,
            Self::Ue3EdVisual => 9851u16,
            Self::VisibleOnQuestAvailable => 9726u16,
            Self::VisibleOnQuestComplete => 9729u16,
            Self::VisibleOnQuestFinished => 9728u16,
            Self::VisibleOnQuestInProgress => 9727u16,
            Self::WorldZoneObjectIndex => 9798u16,
            Self::Zone => 9764u16,
            Self::ZoneGuid => 9791u16,
            Self::AwareDist => 9794u16,
            Self::Defb => 9781u16,
            Self::InstanceGroup => 11385u16,
            Self::IsUnAttackable => 12443u16,
            Self::Abilities => 9721u16,
            Self::Alive => 9785u16,
            Self::AttackedBy => 9784u16,
            Self::CarrierGuid => 9792u16,
            Self::ClientLoadingPriority => 11285u16,
            Self::DirectorTags => 9732u16,
            Self::ForceSpawnOnClient => 9793u16,
            Self::HpCur => 9783u16,
            Self::HpMax => 9782u16,
            Self::IsLocked => 9747u16,
            Self::SpawnerAvatarGuid => 9743u16,
            Self::SpawnerAvatarId => 9735u16,
            Self::DoorIndex => 9719u16,
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
            Self::DoorIndex => "doorIndex",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::DoorIndex => ParamType::Int,
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
        static DOOR_INDEX: Value = Value::Int(0i32);
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
            Self::DoorIndex => &DOOR_INDEX,
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
            Self::DoorIndex => &[ParamFlag::Persistent, ParamFlag::PerInstanceSetting],
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
impl FromStr for MypadRoomDoor {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MYPAD_ROOM_DOOR_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MypadRoomDoor {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            9777u16 => Ok(Self::Action0),
            9776u16 => Ok(Self::Action0Duration),
            9787u16 => Ok(Self::Action0Option),
            9760u16 => Ok(Self::AlwaysVisibleToPlayers),
            10567u16 => Ok(Self::AutoReviveDelay),
            10507u16 => Ok(Self::AutoReviveTime),
            9730u16 => Ok(Self::AwareRange),
            10976u16 => Ok(Self::BeaconRadius),
            9775u16 => Ok(Self::CollisionExtent),
            9779u16 => Ok(Self::ContentClass),
            11062u16 => Ok(Self::CycleQuestBase),
            9737u16 => Ok(Self::DefaultWeapon),
            9720u16 => Ok(Self::DespawnDelay),
            9725u16 => Ok(Self::Dialogs),
            9740u16 => Ok(Self::DisplayName),
            9739u16 => Ok(Self::EnableInGame),
            11186u16 => Ok(Self::FreedomProperties),
            9762u16 => Ok(Self::Freq),
            9774u16 => Ok(Self::GenerateInterestList),
            9773u16 => Ok(Self::HiddenFromClients),
            9789u16 => Ok(Self::HiddenFromPlayers),
            9724u16 => Ok(Self::HideAfterInteraction),
            9757u16 => Ok(Self::Icon),
            9786u16 => Ok(Self::InstanceTags),
            9746u16 => Ok(Self::InstanceZoneKey),
            11132u16 => Ok(Self::InteractionDuration),
            9736u16 => Ok(Self::InteractionRadius),
            9722u16 => Ok(Self::InteractionResetTimer),
            9799u16 => Ok(Self::IsNonSpawnedAvatar),
            9738u16 => Ok(Self::IsSelfRevivable),
            9723u16 => Ok(Self::LastInteractionTime),
            9734u16 => Ok(Self::LuaScript),
            9741u16 => Ok(Self::Lvl),
            9750u16 => Ok(Self::MaterialOverride),
            9788u16 => Ok(Self::Nodelink),
            9797u16 => Ok(Self::OriginalNodeName),
            9796u16 => Ok(Self::OriginalZoneName),
            9772u16 => Ok(Self::PartyGuid),
            9790u16 => Ok(Self::PathfindSafeSpawn),
            9771u16 => Ok(Self::Pos),
            9763u16 => Ok(Self::Power),
            9770u16 => Ok(Self::Priority),
            9975u16 => Ok(Self::QuestFlags),
            9759u16 => Ok(Self::ReadableName),
            9800u16 => Ok(Self::RespawnDelay),
            10823u16 => Ok(Self::RespawnRegionName),
            10882u16 => Ok(Self::RespawnRegionNameOverride),
            9769u16 => Ok(Self::Rot),
            9768u16 => Ok(Self::SelfRadius),
            9742u16 => Ok(Self::SpawnMethod),
            9733u16 => Ok(Self::SpawnPosition),
            9731u16 => Ok(Self::SpawnRotation),
            9767u16 => Ok(Self::Tags),
            9766u16 => Ok(Self::TeamId),
            9778u16 => Ok(Self::Ue3ClassId),
            9851u16 => Ok(Self::Ue3EdVisual),
            9726u16 => Ok(Self::VisibleOnQuestAvailable),
            9729u16 => Ok(Self::VisibleOnQuestComplete),
            9728u16 => Ok(Self::VisibleOnQuestFinished),
            9727u16 => Ok(Self::VisibleOnQuestInProgress),
            9798u16 => Ok(Self::WorldZoneObjectIndex),
            9764u16 => Ok(Self::Zone),
            9791u16 => Ok(Self::ZoneGuid),
            9794u16 => Ok(Self::AwareDist),
            9781u16 => Ok(Self::Defb),
            11385u16 => Ok(Self::InstanceGroup),
            12443u16 => Ok(Self::IsUnAttackable),
            9721u16 => Ok(Self::Abilities),
            9785u16 => Ok(Self::Alive),
            9784u16 => Ok(Self::AttackedBy),
            9792u16 => Ok(Self::CarrierGuid),
            11285u16 => Ok(Self::ClientLoadingPriority),
            9732u16 => Ok(Self::DirectorTags),
            9793u16 => Ok(Self::ForceSpawnOnClient),
            9783u16 => Ok(Self::HpCur),
            9782u16 => Ok(Self::HpMax),
            9747u16 => Ok(Self::IsLocked),
            9743u16 => Ok(Self::SpawnerAvatarGuid),
            9735u16 => Ok(Self::SpawnerAvatarId),
            9719u16 => Ok(Self::DoorIndex),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
