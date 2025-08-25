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
pub enum MylandScoreboard {
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
    ExtraData,
}
pub(crate) static MYLAND_SCOREBOARD_ATTRIBUTES: phf::Map<
    &'static str,
    MylandScoreboard,
> = phf_map! {
    "action0" => MylandScoreboard::Action0, "action0Duration" =>
    MylandScoreboard::Action0Duration, "action0Option" =>
    MylandScoreboard::Action0Option, "alwaysVisibleToPlayers" =>
    MylandScoreboard::AlwaysVisibleToPlayers, "autoReviveDelay" =>
    MylandScoreboard::AutoReviveDelay, "autoReviveTime" =>
    MylandScoreboard::AutoReviveTime, "AwareRange" => MylandScoreboard::AwareRange,
    "BeaconRadius" => MylandScoreboard::BeaconRadius, "collisionExtent" =>
    MylandScoreboard::CollisionExtent, "ContentClass" => MylandScoreboard::ContentClass,
    "CycleQuestBase" => MylandScoreboard::CycleQuestBase, "defaultWeapon" =>
    MylandScoreboard::DefaultWeapon, "despawnDelay" => MylandScoreboard::DespawnDelay,
    "Dialogs" => MylandScoreboard::Dialogs, "DisplayName" =>
    MylandScoreboard::DisplayName, "EnableInGame" => MylandScoreboard::EnableInGame,
    "FreedomProperties" => MylandScoreboard::FreedomProperties, "Freq" =>
    MylandScoreboard::Freq, "generateInterestList" =>
    MylandScoreboard::GenerateInterestList, "hiddenFromClients" =>
    MylandScoreboard::HiddenFromClients, "hiddenFromPlayers" =>
    MylandScoreboard::HiddenFromPlayers, "HideAfterInteraction" =>
    MylandScoreboard::HideAfterInteraction, "Icon" => MylandScoreboard::Icon,
    "instanceTags" => MylandScoreboard::InstanceTags, "instanceZoneKey" =>
    MylandScoreboard::InstanceZoneKey, "InteractionDuration" =>
    MylandScoreboard::InteractionDuration, "InteractionRadius" =>
    MylandScoreboard::InteractionRadius, "InteractionResetTimer" =>
    MylandScoreboard::InteractionResetTimer, "isNonSpawnedAvatar" =>
    MylandScoreboard::IsNonSpawnedAvatar, "isSelfRevivable" =>
    MylandScoreboard::IsSelfRevivable, "LastInteractionTime" =>
    MylandScoreboard::LastInteractionTime, "LuaScript" => MylandScoreboard::LuaScript,
    "lvl" => MylandScoreboard::Lvl, "MaterialOverride" =>
    MylandScoreboard::MaterialOverride, "nodelink" => MylandScoreboard::Nodelink,
    "originalNodeName" => MylandScoreboard::OriginalNodeName, "originalZoneName" =>
    MylandScoreboard::OriginalZoneName, "partyGUID" => MylandScoreboard::PartyGuid,
    "pathfindSafeSpawn" => MylandScoreboard::PathfindSafeSpawn, "pos" =>
    MylandScoreboard::Pos, "Power" => MylandScoreboard::Power, "priority" =>
    MylandScoreboard::Priority, "QuestFlags" => MylandScoreboard::QuestFlags,
    "ReadableName" => MylandScoreboard::ReadableName, "respawnDelay" =>
    MylandScoreboard::RespawnDelay, "RespawnRegionName" =>
    MylandScoreboard::RespawnRegionName, "RespawnRegionNameOverride" =>
    MylandScoreboard::RespawnRegionNameOverride, "rot" => MylandScoreboard::Rot,
    "selfRadius" => MylandScoreboard::SelfRadius, "spawnMethod" =>
    MylandScoreboard::SpawnMethod, "spawnPosition" => MylandScoreboard::SpawnPosition,
    "spawnRotation" => MylandScoreboard::SpawnRotation, "tags" => MylandScoreboard::Tags,
    "teamID" => MylandScoreboard::TeamId, "UE3ClassID" => MylandScoreboard::Ue3ClassId,
    "UE3EdVisual" => MylandScoreboard::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    MylandScoreboard::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    MylandScoreboard::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    MylandScoreboard::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    MylandScoreboard::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    MylandScoreboard::WorldZoneObjectIndex, "zone" => MylandScoreboard::Zone, "ZoneGuid"
    => MylandScoreboard::ZoneGuid, "ExtraData" => MylandScoreboard::ExtraData,
};
pub(crate) static MYLAND_SCOREBOARD_ATTRIBUTES_ID: phf::Map<u16, MylandScoreboard> = phf_map! {
    5389u16 => MylandScoreboard::Action0, 5388u16 => MylandScoreboard::Action0Duration,
    5393u16 => MylandScoreboard::Action0Option, 5371u16 =>
    MylandScoreboard::AlwaysVisibleToPlayers, 10568u16 =>
    MylandScoreboard::AutoReviveDelay, 10508u16 => MylandScoreboard::AutoReviveTime,
    8286u16 => MylandScoreboard::AwareRange, 10979u16 => MylandScoreboard::BeaconRadius,
    5387u16 => MylandScoreboard::CollisionExtent, 5391u16 =>
    MylandScoreboard::ContentClass, 11079u16 => MylandScoreboard::CycleQuestBase, 7255u16
    => MylandScoreboard::DefaultWeapon, 9677u16 => MylandScoreboard::DespawnDelay,
    8873u16 => MylandScoreboard::Dialogs, 6641u16 => MylandScoreboard::DisplayName,
    6867u16 => MylandScoreboard::EnableInGame, 11203u16 =>
    MylandScoreboard::FreedomProperties, 5374u16 => MylandScoreboard::Freq, 5386u16 =>
    MylandScoreboard::GenerateInterestList, 5385u16 =>
    MylandScoreboard::HiddenFromClients, 5395u16 => MylandScoreboard::HiddenFromPlayers,
    9177u16 => MylandScoreboard::HideAfterInteraction, 5369u16 => MylandScoreboard::Icon,
    5392u16 => MylandScoreboard::InstanceTags, 5601u16 =>
    MylandScoreboard::InstanceZoneKey, 11149u16 => MylandScoreboard::InteractionDuration,
    7516u16 => MylandScoreboard::InteractionRadius, 9179u16 =>
    MylandScoreboard::InteractionResetTimer, 5401u16 =>
    MylandScoreboard::IsNonSpawnedAvatar, 7200u16 => MylandScoreboard::IsSelfRevivable,
    9178u16 => MylandScoreboard::LastInteractionTime, 7819u16 =>
    MylandScoreboard::LuaScript, 6224u16 => MylandScoreboard::Lvl, 5368u16 =>
    MylandScoreboard::MaterialOverride, 5394u16 => MylandScoreboard::Nodelink, 5399u16 =>
    MylandScoreboard::OriginalNodeName, 5398u16 => MylandScoreboard::OriginalZoneName,
    5384u16 => MylandScoreboard::PartyGuid, 5396u16 =>
    MylandScoreboard::PathfindSafeSpawn, 5383u16 => MylandScoreboard::Pos, 5375u16 =>
    MylandScoreboard::Power, 5382u16 => MylandScoreboard::Priority, 9976u16 =>
    MylandScoreboard::QuestFlags, 5370u16 => MylandScoreboard::ReadableName, 5402u16 =>
    MylandScoreboard::RespawnDelay, 10826u16 => MylandScoreboard::RespawnRegionName,
    10885u16 => MylandScoreboard::RespawnRegionNameOverride, 5381u16 =>
    MylandScoreboard::Rot, 5380u16 => MylandScoreboard::SelfRadius, 6142u16 =>
    MylandScoreboard::SpawnMethod, 7874u16 => MylandScoreboard::SpawnPosition, 8229u16 =>
    MylandScoreboard::SpawnRotation, 5379u16 => MylandScoreboard::Tags, 5378u16 =>
    MylandScoreboard::TeamId, 5390u16 => MylandScoreboard::Ue3ClassId, 9852u16 =>
    MylandScoreboard::Ue3EdVisual, 8758u16 => MylandScoreboard::VisibleOnQuestAvailable,
    8755u16 => MylandScoreboard::VisibleOnQuestComplete, 8756u16 =>
    MylandScoreboard::VisibleOnQuestFinished, 8757u16 =>
    MylandScoreboard::VisibleOnQuestInProgress, 5400u16 =>
    MylandScoreboard::WorldZoneObjectIndex, 5376u16 => MylandScoreboard::Zone, 5397u16 =>
    MylandScoreboard::ZoneGuid, 5367u16 => MylandScoreboard::ExtraData,
};
impl Attribute for MylandScoreboard {
    fn class() -> Class {
        Class::MylandScoreboard
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
            Self::ExtraData => &Self::ExtraData,
        }
    }
}
impl AttributeInfo for MylandScoreboard {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 5389u16,
            Self::Action0Duration => 5388u16,
            Self::Action0Option => 5393u16,
            Self::AlwaysVisibleToPlayers => 5371u16,
            Self::AutoReviveDelay => 10568u16,
            Self::AutoReviveTime => 10508u16,
            Self::AwareRange => 8286u16,
            Self::BeaconRadius => 10979u16,
            Self::CollisionExtent => 5387u16,
            Self::ContentClass => 5391u16,
            Self::CycleQuestBase => 11079u16,
            Self::DefaultWeapon => 7255u16,
            Self::DespawnDelay => 9677u16,
            Self::Dialogs => 8873u16,
            Self::DisplayName => 6641u16,
            Self::EnableInGame => 6867u16,
            Self::FreedomProperties => 11203u16,
            Self::Freq => 5374u16,
            Self::GenerateInterestList => 5386u16,
            Self::HiddenFromClients => 5385u16,
            Self::HiddenFromPlayers => 5395u16,
            Self::HideAfterInteraction => 9177u16,
            Self::Icon => 5369u16,
            Self::InstanceTags => 5392u16,
            Self::InstanceZoneKey => 5601u16,
            Self::InteractionDuration => 11149u16,
            Self::InteractionRadius => 7516u16,
            Self::InteractionResetTimer => 9179u16,
            Self::IsNonSpawnedAvatar => 5401u16,
            Self::IsSelfRevivable => 7200u16,
            Self::LastInteractionTime => 9178u16,
            Self::LuaScript => 7819u16,
            Self::Lvl => 6224u16,
            Self::MaterialOverride => 5368u16,
            Self::Nodelink => 5394u16,
            Self::OriginalNodeName => 5399u16,
            Self::OriginalZoneName => 5398u16,
            Self::PartyGuid => 5384u16,
            Self::PathfindSafeSpawn => 5396u16,
            Self::Pos => 5383u16,
            Self::Power => 5375u16,
            Self::Priority => 5382u16,
            Self::QuestFlags => 9976u16,
            Self::ReadableName => 5370u16,
            Self::RespawnDelay => 5402u16,
            Self::RespawnRegionName => 10826u16,
            Self::RespawnRegionNameOverride => 10885u16,
            Self::Rot => 5381u16,
            Self::SelfRadius => 5380u16,
            Self::SpawnMethod => 6142u16,
            Self::SpawnPosition => 7874u16,
            Self::SpawnRotation => 8229u16,
            Self::Tags => 5379u16,
            Self::TeamId => 5378u16,
            Self::Ue3ClassId => 5390u16,
            Self::Ue3EdVisual => 9852u16,
            Self::VisibleOnQuestAvailable => 8758u16,
            Self::VisibleOnQuestComplete => 8755u16,
            Self::VisibleOnQuestFinished => 8756u16,
            Self::VisibleOnQuestInProgress => 8757u16,
            Self::WorldZoneObjectIndex => 5400u16,
            Self::Zone => 5376u16,
            Self::ZoneGuid => 5397u16,
            Self::ExtraData => 5367u16,
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
            Self::ExtraData => "ExtraData",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::ContentClass => ParamType::String,
            Self::PathfindSafeSpawn => ParamType::Bool,
            Self::Tags => ParamType::String,
            Self::Ue3ClassId => ParamType::String,
            Self::ExtraData => ParamType::String,
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
            Self::TeamId => ParamType::Int,
            Self::Ue3EdVisual => ParamType::String,
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
        }
    }
    fn default(&self) -> &'static Value {
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            "GeneralContentInfos.Others.MyLandScoreboard".to_string(),
        ));
        static PATHFIND_SAFE_SPAWN: Value = Value::Bool(false);
        static TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            "mylandScore".to_string(),
        ));
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLMyLandIgloScoreboard".to_string(),
        ));
        static EXTRA_DATA: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
        match self {
            Self::ContentClass => &CONTENT_CLASS,
            Self::PathfindSafeSpawn => &PATHFIND_SAFE_SPAWN,
            Self::Tags => &TAGS,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::ExtraData => &EXTRA_DATA,
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
            Self::TeamId => &TEAM_ID,
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::ContentClass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PathfindSafeSpawn => &[ParamFlag::Persistent, ParamFlag::Content],
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
            Self::ExtraData => &[ParamFlag::NodeOwn],
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
        }
    }
}
impl FromStr for MylandScoreboard {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MYLAND_SCOREBOARD_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MylandScoreboard {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5389u16 => Ok(Self::Action0),
            5388u16 => Ok(Self::Action0Duration),
            5393u16 => Ok(Self::Action0Option),
            5371u16 => Ok(Self::AlwaysVisibleToPlayers),
            10568u16 => Ok(Self::AutoReviveDelay),
            10508u16 => Ok(Self::AutoReviveTime),
            8286u16 => Ok(Self::AwareRange),
            10979u16 => Ok(Self::BeaconRadius),
            5387u16 => Ok(Self::CollisionExtent),
            5391u16 => Ok(Self::ContentClass),
            11079u16 => Ok(Self::CycleQuestBase),
            7255u16 => Ok(Self::DefaultWeapon),
            9677u16 => Ok(Self::DespawnDelay),
            8873u16 => Ok(Self::Dialogs),
            6641u16 => Ok(Self::DisplayName),
            6867u16 => Ok(Self::EnableInGame),
            11203u16 => Ok(Self::FreedomProperties),
            5374u16 => Ok(Self::Freq),
            5386u16 => Ok(Self::GenerateInterestList),
            5385u16 => Ok(Self::HiddenFromClients),
            5395u16 => Ok(Self::HiddenFromPlayers),
            9177u16 => Ok(Self::HideAfterInteraction),
            5369u16 => Ok(Self::Icon),
            5392u16 => Ok(Self::InstanceTags),
            5601u16 => Ok(Self::InstanceZoneKey),
            11149u16 => Ok(Self::InteractionDuration),
            7516u16 => Ok(Self::InteractionRadius),
            9179u16 => Ok(Self::InteractionResetTimer),
            5401u16 => Ok(Self::IsNonSpawnedAvatar),
            7200u16 => Ok(Self::IsSelfRevivable),
            9178u16 => Ok(Self::LastInteractionTime),
            7819u16 => Ok(Self::LuaScript),
            6224u16 => Ok(Self::Lvl),
            5368u16 => Ok(Self::MaterialOverride),
            5394u16 => Ok(Self::Nodelink),
            5399u16 => Ok(Self::OriginalNodeName),
            5398u16 => Ok(Self::OriginalZoneName),
            5384u16 => Ok(Self::PartyGuid),
            5396u16 => Ok(Self::PathfindSafeSpawn),
            5383u16 => Ok(Self::Pos),
            5375u16 => Ok(Self::Power),
            5382u16 => Ok(Self::Priority),
            9976u16 => Ok(Self::QuestFlags),
            5370u16 => Ok(Self::ReadableName),
            5402u16 => Ok(Self::RespawnDelay),
            10826u16 => Ok(Self::RespawnRegionName),
            10885u16 => Ok(Self::RespawnRegionNameOverride),
            5381u16 => Ok(Self::Rot),
            5380u16 => Ok(Self::SelfRadius),
            6142u16 => Ok(Self::SpawnMethod),
            7874u16 => Ok(Self::SpawnPosition),
            8229u16 => Ok(Self::SpawnRotation),
            5379u16 => Ok(Self::Tags),
            5378u16 => Ok(Self::TeamId),
            5390u16 => Ok(Self::Ue3ClassId),
            9852u16 => Ok(Self::Ue3EdVisual),
            8758u16 => Ok(Self::VisibleOnQuestAvailable),
            8755u16 => Ok(Self::VisibleOnQuestComplete),
            8756u16 => Ok(Self::VisibleOnQuestFinished),
            8757u16 => Ok(Self::VisibleOnQuestInProgress),
            5400u16 => Ok(Self::WorldZoneObjectIndex),
            5376u16 => Ok(Self::Zone),
            5397u16 => Ok(Self::ZoneGuid),
            5367u16 => Ok(Self::ExtraData),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
