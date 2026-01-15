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
pub enum PhysicsActor {
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
    Alive,
    Defb,
    AngularDamping,
    CollisionShape,
    DynamicFriction,
    Kinematic,
    Mass,
    Restitution,
    SkinWidth,
    Static,
    StaticFriction,
}
pub(crate) static PHYSICS_ACTOR_ATTRIBUTES: phf::Map<&'static str, PhysicsActor> = phf_map! {
    "action0" => PhysicsActor::Action0, "action0Duration" =>
    PhysicsActor::Action0Duration, "action0Option" => PhysicsActor::Action0Option,
    "alwaysVisibleToPlayers" => PhysicsActor::AlwaysVisibleToPlayers, "autoReviveDelay"
    => PhysicsActor::AutoReviveDelay, "autoReviveTime" => PhysicsActor::AutoReviveTime,
    "AwareRange" => PhysicsActor::AwareRange, "BeaconRadius" =>
    PhysicsActor::BeaconRadius, "collisionExtent" => PhysicsActor::CollisionExtent,
    "ContentClass" => PhysicsActor::ContentClass, "CycleQuestBase" =>
    PhysicsActor::CycleQuestBase, "defaultWeapon" => PhysicsActor::DefaultWeapon,
    "despawnDelay" => PhysicsActor::DespawnDelay, "Dialogs" => PhysicsActor::Dialogs,
    "DisplayName" => PhysicsActor::DisplayName, "EnableInGame" =>
    PhysicsActor::EnableInGame, "FreedomProperties" => PhysicsActor::FreedomProperties,
    "Freq" => PhysicsActor::Freq, "generateInterestList" =>
    PhysicsActor::GenerateInterestList, "hiddenFromClients" =>
    PhysicsActor::HiddenFromClients, "hiddenFromPlayers" =>
    PhysicsActor::HiddenFromPlayers, "HideAfterInteraction" =>
    PhysicsActor::HideAfterInteraction, "Icon" => PhysicsActor::Icon, "instanceTags" =>
    PhysicsActor::InstanceTags, "instanceZoneKey" => PhysicsActor::InstanceZoneKey,
    "InteractionDuration" => PhysicsActor::InteractionDuration, "InteractionRadius" =>
    PhysicsActor::InteractionRadius, "InteractionResetTimer" =>
    PhysicsActor::InteractionResetTimer, "isNonSpawnedAvatar" =>
    PhysicsActor::IsNonSpawnedAvatar, "isSelfRevivable" => PhysicsActor::IsSelfRevivable,
    "LastInteractionTime" => PhysicsActor::LastInteractionTime, "LuaScript" =>
    PhysicsActor::LuaScript, "lvl" => PhysicsActor::Lvl, "MaterialOverride" =>
    PhysicsActor::MaterialOverride, "nodelink" => PhysicsActor::Nodelink,
    "originalNodeName" => PhysicsActor::OriginalNodeName, "originalZoneName" =>
    PhysicsActor::OriginalZoneName, "partyGUID" => PhysicsActor::PartyGuid,
    "pathfindSafeSpawn" => PhysicsActor::PathfindSafeSpawn, "pos" => PhysicsActor::Pos,
    "Power" => PhysicsActor::Power, "priority" => PhysicsActor::Priority, "QuestFlags" =>
    PhysicsActor::QuestFlags, "ReadableName" => PhysicsActor::ReadableName,
    "respawnDelay" => PhysicsActor::RespawnDelay, "RespawnRegionName" =>
    PhysicsActor::RespawnRegionName, "RespawnRegionNameOverride" =>
    PhysicsActor::RespawnRegionNameOverride, "rot" => PhysicsActor::Rot, "selfRadius" =>
    PhysicsActor::SelfRadius, "spawnMethod" => PhysicsActor::SpawnMethod, "spawnPosition"
    => PhysicsActor::SpawnPosition, "spawnRotation" => PhysicsActor::SpawnRotation,
    "tags" => PhysicsActor::Tags, "teamID" => PhysicsActor::TeamId, "UE3ClassID" =>
    PhysicsActor::Ue3ClassId, "UE3EdVisual" => PhysicsActor::Ue3EdVisual,
    "VisibleOnQuestAvailable" => PhysicsActor::VisibleOnQuestAvailable,
    "VisibleOnQuestComplete" => PhysicsActor::VisibleOnQuestComplete,
    "VisibleOnQuestFinished" => PhysicsActor::VisibleOnQuestFinished,
    "VisibleOnQuestInProgress" => PhysicsActor::VisibleOnQuestInProgress,
    "WorldZoneObjectIndex" => PhysicsActor::WorldZoneObjectIndex, "zone" =>
    PhysicsActor::Zone, "ZoneGuid" => PhysicsActor::ZoneGuid, "alive" =>
    PhysicsActor::Alive, "defb" => PhysicsActor::Defb, "angularDamping" =>
    PhysicsActor::AngularDamping, "collisionShape" => PhysicsActor::CollisionShape,
    "dynamicFriction" => PhysicsActor::DynamicFriction, "kinematic" =>
    PhysicsActor::Kinematic, "mass" => PhysicsActor::Mass, "restitution" =>
    PhysicsActor::Restitution, "skinWidth" => PhysicsActor::SkinWidth, "static" =>
    PhysicsActor::Static, "staticFriction" => PhysicsActor::StaticFriction,
};
pub(crate) static PHYSICS_ACTOR_ATTRIBUTES_ID: phf::Map<u16, PhysicsActor> = phf_map! {
    3749u16 => PhysicsActor::Action0, 3750u16 => PhysicsActor::Action0Duration, 3745u16
    => PhysicsActor::Action0Option, 3767u16 => PhysicsActor::AlwaysVisibleToPlayers,
    10572u16 => PhysicsActor::AutoReviveDelay, 10512u16 => PhysicsActor::AutoReviveTime,
    8290u16 => PhysicsActor::AwareRange, 10983u16 => PhysicsActor::BeaconRadius, 3751u16
    => PhysicsActor::CollisionExtent, 3747u16 => PhysicsActor::ContentClass, 11076u16 =>
    PhysicsActor::CycleQuestBase, 7260u16 => PhysicsActor::DefaultWeapon, 9681u16 =>
    PhysicsActor::DespawnDelay, 8877u16 => PhysicsActor::Dialogs, 6646u16 =>
    PhysicsActor::DisplayName, 6872u16 => PhysicsActor::EnableInGame, 11200u16 =>
    PhysicsActor::FreedomProperties, 3764u16 => PhysicsActor::Freq, 3752u16 =>
    PhysicsActor::GenerateInterestList, 3753u16 => PhysicsActor::HiddenFromClients,
    3743u16 => PhysicsActor::HiddenFromPlayers, 9189u16 =>
    PhysicsActor::HideAfterInteraction, 4387u16 => PhysicsActor::Icon, 3746u16 =>
    PhysicsActor::InstanceTags, 5606u16 => PhysicsActor::InstanceZoneKey, 11146u16 =>
    PhysicsActor::InteractionDuration, 7521u16 => PhysicsActor::InteractionRadius,
    9191u16 => PhysicsActor::InteractionResetTimer, 3737u16 =>
    PhysicsActor::IsNonSpawnedAvatar, 7205u16 => PhysicsActor::IsSelfRevivable, 9190u16
    => PhysicsActor::LastInteractionTime, 7823u16 => PhysicsActor::LuaScript, 6229u16 =>
    PhysicsActor::Lvl, 4768u16 => PhysicsActor::MaterialOverride, 3744u16 =>
    PhysicsActor::Nodelink, 3739u16 => PhysicsActor::OriginalNodeName, 3740u16 =>
    PhysicsActor::OriginalZoneName, 3754u16 => PhysicsActor::PartyGuid, 3742u16 =>
    PhysicsActor::PathfindSafeSpawn, 3755u16 => PhysicsActor::Pos, 3763u16 =>
    PhysicsActor::Power, 3756u16 => PhysicsActor::Priority, 9980u16 =>
    PhysicsActor::QuestFlags, 3735u16 => PhysicsActor::ReadableName, 3736u16 =>
    PhysicsActor::RespawnDelay, 10830u16 => PhysicsActor::RespawnRegionName, 10889u16 =>
    PhysicsActor::RespawnRegionNameOverride, 3757u16 => PhysicsActor::Rot, 3758u16 =>
    PhysicsActor::SelfRadius, 6147u16 => PhysicsActor::SpawnMethod, 7878u16 =>
    PhysicsActor::SpawnPosition, 8233u16 => PhysicsActor::SpawnRotation, 3759u16 =>
    PhysicsActor::Tags, 3760u16 => PhysicsActor::TeamId, 3748u16 =>
    PhysicsActor::Ue3ClassId, 9856u16 => PhysicsActor::Ue3EdVisual, 8774u16 =>
    PhysicsActor::VisibleOnQuestAvailable, 8771u16 =>
    PhysicsActor::VisibleOnQuestComplete, 8772u16 =>
    PhysicsActor::VisibleOnQuestFinished, 8773u16 =>
    PhysicsActor::VisibleOnQuestInProgress, 3738u16 =>
    PhysicsActor::WorldZoneObjectIndex, 3762u16 => PhysicsActor::Zone, 3741u16 =>
    PhysicsActor::ZoneGuid, 3769u16 => PhysicsActor::Alive, 3768u16 =>
    PhysicsActor::Defb, 4054u16 => PhysicsActor::AngularDamping, 3734u16 =>
    PhysicsActor::CollisionShape, 3777u16 => PhysicsActor::DynamicFriction, 3732u16 =>
    PhysicsActor::Kinematic, 3733u16 => PhysicsActor::Mass, 3731u16 =>
    PhysicsActor::Restitution, 4053u16 => PhysicsActor::SkinWidth, 3780u16 =>
    PhysicsActor::Static, 3776u16 => PhysicsActor::StaticFriction,
};
impl Attribute for PhysicsActor {
    fn class() -> Class {
        Class::PhysicsActor
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
            Self::Alive => &Self::Alive,
            Self::Defb => &Self::Defb,
            Self::AngularDamping => &Self::AngularDamping,
            Self::CollisionShape => &Self::CollisionShape,
            Self::DynamicFriction => &Self::DynamicFriction,
            Self::Kinematic => &Self::Kinematic,
            Self::Mass => &Self::Mass,
            Self::Restitution => &Self::Restitution,
            Self::SkinWidth => &Self::SkinWidth,
            Self::Static => &Self::Static,
            Self::StaticFriction => &Self::StaticFriction,
        }
    }
}
impl AttributeInfo for PhysicsActor {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 3749u16,
            Self::Action0Duration => 3750u16,
            Self::Action0Option => 3745u16,
            Self::AlwaysVisibleToPlayers => 3767u16,
            Self::AutoReviveDelay => 10572u16,
            Self::AutoReviveTime => 10512u16,
            Self::AwareRange => 8290u16,
            Self::BeaconRadius => 10983u16,
            Self::CollisionExtent => 3751u16,
            Self::ContentClass => 3747u16,
            Self::CycleQuestBase => 11076u16,
            Self::DefaultWeapon => 7260u16,
            Self::DespawnDelay => 9681u16,
            Self::Dialogs => 8877u16,
            Self::DisplayName => 6646u16,
            Self::EnableInGame => 6872u16,
            Self::FreedomProperties => 11200u16,
            Self::Freq => 3764u16,
            Self::GenerateInterestList => 3752u16,
            Self::HiddenFromClients => 3753u16,
            Self::HiddenFromPlayers => 3743u16,
            Self::HideAfterInteraction => 9189u16,
            Self::Icon => 4387u16,
            Self::InstanceTags => 3746u16,
            Self::InstanceZoneKey => 5606u16,
            Self::InteractionDuration => 11146u16,
            Self::InteractionRadius => 7521u16,
            Self::InteractionResetTimer => 9191u16,
            Self::IsNonSpawnedAvatar => 3737u16,
            Self::IsSelfRevivable => 7205u16,
            Self::LastInteractionTime => 9190u16,
            Self::LuaScript => 7823u16,
            Self::Lvl => 6229u16,
            Self::MaterialOverride => 4768u16,
            Self::Nodelink => 3744u16,
            Self::OriginalNodeName => 3739u16,
            Self::OriginalZoneName => 3740u16,
            Self::PartyGuid => 3754u16,
            Self::PathfindSafeSpawn => 3742u16,
            Self::Pos => 3755u16,
            Self::Power => 3763u16,
            Self::Priority => 3756u16,
            Self::QuestFlags => 9980u16,
            Self::ReadableName => 3735u16,
            Self::RespawnDelay => 3736u16,
            Self::RespawnRegionName => 10830u16,
            Self::RespawnRegionNameOverride => 10889u16,
            Self::Rot => 3757u16,
            Self::SelfRadius => 3758u16,
            Self::SpawnMethod => 6147u16,
            Self::SpawnPosition => 7878u16,
            Self::SpawnRotation => 8233u16,
            Self::Tags => 3759u16,
            Self::TeamId => 3760u16,
            Self::Ue3ClassId => 3748u16,
            Self::Ue3EdVisual => 9856u16,
            Self::VisibleOnQuestAvailable => 8774u16,
            Self::VisibleOnQuestComplete => 8771u16,
            Self::VisibleOnQuestFinished => 8772u16,
            Self::VisibleOnQuestInProgress => 8773u16,
            Self::WorldZoneObjectIndex => 3738u16,
            Self::Zone => 3762u16,
            Self::ZoneGuid => 3741u16,
            Self::Alive => 3769u16,
            Self::Defb => 3768u16,
            Self::AngularDamping => 4054u16,
            Self::CollisionShape => 3734u16,
            Self::DynamicFriction => 3777u16,
            Self::Kinematic => 3732u16,
            Self::Mass => 3733u16,
            Self::Restitution => 3731u16,
            Self::SkinWidth => 4053u16,
            Self::Static => 3780u16,
            Self::StaticFriction => 3776u16,
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
            Self::Alive => "alive",
            Self::Defb => "defb",
            Self::AngularDamping => "angularDamping",
            Self::CollisionShape => "collisionShape",
            Self::DynamicFriction => "dynamicFriction",
            Self::Kinematic => "kinematic",
            Self::Mass => "mass",
            Self::Restitution => "restitution",
            Self::SkinWidth => "skinWidth",
            Self::Static => "static",
            Self::StaticFriction => "staticFriction",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Ue3ClassId => ParamType::String,
            Self::AngularDamping => ParamType::Float,
            Self::CollisionShape => ParamType::Int,
            Self::DynamicFriction => ParamType::Float,
            Self::Kinematic => ParamType::Bool,
            Self::Mass => ParamType::Float,
            Self::Restitution => ParamType::Float,
            Self::SkinWidth => ParamType::Float,
            Self::Static => ParamType::Bool,
            Self::StaticFriction => ParamType::Float,
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
            Self::Alive => ParamType::Bool,
            Self::Defb => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLAvatarPhysXActor".to_string(),
        ));
        static ANGULAR_DAMPING: Value = Value::Float(0.05f32);
        static COLLISION_SHAPE: Value = Value::Int(2i32);
        static DYNAMIC_FRICTION: Value = Value::Float(0f32);
        static KINEMATIC: Value = Value::Bool(false);
        static MASS: Value = Value::Float(10f32);
        static RESTITUTION: Value = Value::Float(0f32);
        static SKIN_WIDTH: Value = Value::Float(1.25f32);
        static STATIC: Value = Value::Bool(false);
        static STATIC_FRICTION: Value = Value::Float(0f32);
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
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
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
        static ALIVE: Value = Value::Bool(true);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        match self {
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::AngularDamping => &ANGULAR_DAMPING,
            Self::CollisionShape => &COLLISION_SHAPE,
            Self::DynamicFriction => &DYNAMIC_FRICTION,
            Self::Kinematic => &KINEMATIC,
            Self::Mass => &MASS,
            Self::Restitution => &RESTITUTION,
            Self::SkinWidth => &SKIN_WIDTH,
            Self::Static => &STATIC,
            Self::StaticFriction => &STATIC_FRICTION,
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
            Self::Alive => &ALIVE,
            Self::Defb => &DEFB,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AngularDamping => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CollisionShape => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::DynamicFriction => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Kinematic => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Mass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Restitution => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SkinWidth => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Static => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::StaticFriction => &[ParamFlag::Persistent, ParamFlag::Content],
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
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent, ParamFlag::Content],
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
            Self::Alive => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Defb => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
        }
    }
}
impl FromStr for PhysicsActor {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PHYSICS_ACTOR_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for PhysicsActor {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            3749u16 => Ok(Self::Action0),
            3750u16 => Ok(Self::Action0Duration),
            3745u16 => Ok(Self::Action0Option),
            3767u16 => Ok(Self::AlwaysVisibleToPlayers),
            10572u16 => Ok(Self::AutoReviveDelay),
            10512u16 => Ok(Self::AutoReviveTime),
            8290u16 => Ok(Self::AwareRange),
            10983u16 => Ok(Self::BeaconRadius),
            3751u16 => Ok(Self::CollisionExtent),
            3747u16 => Ok(Self::ContentClass),
            11076u16 => Ok(Self::CycleQuestBase),
            7260u16 => Ok(Self::DefaultWeapon),
            9681u16 => Ok(Self::DespawnDelay),
            8877u16 => Ok(Self::Dialogs),
            6646u16 => Ok(Self::DisplayName),
            6872u16 => Ok(Self::EnableInGame),
            11200u16 => Ok(Self::FreedomProperties),
            3764u16 => Ok(Self::Freq),
            3752u16 => Ok(Self::GenerateInterestList),
            3753u16 => Ok(Self::HiddenFromClients),
            3743u16 => Ok(Self::HiddenFromPlayers),
            9189u16 => Ok(Self::HideAfterInteraction),
            4387u16 => Ok(Self::Icon),
            3746u16 => Ok(Self::InstanceTags),
            5606u16 => Ok(Self::InstanceZoneKey),
            11146u16 => Ok(Self::InteractionDuration),
            7521u16 => Ok(Self::InteractionRadius),
            9191u16 => Ok(Self::InteractionResetTimer),
            3737u16 => Ok(Self::IsNonSpawnedAvatar),
            7205u16 => Ok(Self::IsSelfRevivable),
            9190u16 => Ok(Self::LastInteractionTime),
            7823u16 => Ok(Self::LuaScript),
            6229u16 => Ok(Self::Lvl),
            4768u16 => Ok(Self::MaterialOverride),
            3744u16 => Ok(Self::Nodelink),
            3739u16 => Ok(Self::OriginalNodeName),
            3740u16 => Ok(Self::OriginalZoneName),
            3754u16 => Ok(Self::PartyGuid),
            3742u16 => Ok(Self::PathfindSafeSpawn),
            3755u16 => Ok(Self::Pos),
            3763u16 => Ok(Self::Power),
            3756u16 => Ok(Self::Priority),
            9980u16 => Ok(Self::QuestFlags),
            3735u16 => Ok(Self::ReadableName),
            3736u16 => Ok(Self::RespawnDelay),
            10830u16 => Ok(Self::RespawnRegionName),
            10889u16 => Ok(Self::RespawnRegionNameOverride),
            3757u16 => Ok(Self::Rot),
            3758u16 => Ok(Self::SelfRadius),
            6147u16 => Ok(Self::SpawnMethod),
            7878u16 => Ok(Self::SpawnPosition),
            8233u16 => Ok(Self::SpawnRotation),
            3759u16 => Ok(Self::Tags),
            3760u16 => Ok(Self::TeamId),
            3748u16 => Ok(Self::Ue3ClassId),
            9856u16 => Ok(Self::Ue3EdVisual),
            8774u16 => Ok(Self::VisibleOnQuestAvailable),
            8771u16 => Ok(Self::VisibleOnQuestComplete),
            8772u16 => Ok(Self::VisibleOnQuestFinished),
            8773u16 => Ok(Self::VisibleOnQuestInProgress),
            3738u16 => Ok(Self::WorldZoneObjectIndex),
            3762u16 => Ok(Self::Zone),
            3741u16 => Ok(Self::ZoneGuid),
            3769u16 => Ok(Self::Alive),
            3768u16 => Ok(Self::Defb),
            4054u16 => Ok(Self::AngularDamping),
            3734u16 => Ok(Self::CollisionShape),
            3777u16 => Ok(Self::DynamicFriction),
            3732u16 => Ok(Self::Kinematic),
            3733u16 => Ok(Self::Mass),
            3731u16 => Ok(Self::Restitution),
            4053u16 => Ok(Self::SkinWidth),
            3780u16 => Ok(Self::Static),
            3776u16 => Ok(Self::StaticFriction),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
