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
pub enum NpcBase {
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
    BitsMultiplier,
    Defb,
    DropLevelVariance,
    EvadeRadius,
    ExperienceMultiplier,
    FragmentsMultiplier,
    InstanceGroup,
    InstanceGuid,
    InteractRadius,
    IsUnAttackable,
    MoveDest,
    MoveSpeed,
    NickName,
    NpcStateInfo,
    NpcType,
    Orientation,
    RunSpeed,
    ShuffleSpeed,
    SomaMultiplier,
    SpawnLeashRadius,
    SpeakResponse,
    WalkSpeed,
}
pub(crate) static NPC_BASE_ATTRIBUTES: phf::Map<&'static str, NpcBase> = phf_map! {
    "action0" => NpcBase::Action0, "action0Duration" => NpcBase::Action0Duration,
    "action0Option" => NpcBase::Action0Option, "alwaysVisibleToPlayers" =>
    NpcBase::AlwaysVisibleToPlayers, "autoReviveDelay" => NpcBase::AutoReviveDelay,
    "autoReviveTime" => NpcBase::AutoReviveTime, "AwareRange" => NpcBase::AwareRange,
    "BeaconRadius" => NpcBase::BeaconRadius, "collisionExtent" =>
    NpcBase::CollisionExtent, "ContentClass" => NpcBase::ContentClass, "CycleQuestBase"
    => NpcBase::CycleQuestBase, "defaultWeapon" => NpcBase::DefaultWeapon, "despawnDelay"
    => NpcBase::DespawnDelay, "Dialogs" => NpcBase::Dialogs, "DisplayName" =>
    NpcBase::DisplayName, "EnableInGame" => NpcBase::EnableInGame, "FreedomProperties" =>
    NpcBase::FreedomProperties, "Freq" => NpcBase::Freq, "generateInterestList" =>
    NpcBase::GenerateInterestList, "hiddenFromClients" => NpcBase::HiddenFromClients,
    "hiddenFromPlayers" => NpcBase::HiddenFromPlayers, "HideAfterInteraction" =>
    NpcBase::HideAfterInteraction, "Icon" => NpcBase::Icon, "instanceTags" =>
    NpcBase::InstanceTags, "instanceZoneKey" => NpcBase::InstanceZoneKey,
    "InteractionDuration" => NpcBase::InteractionDuration, "InteractionRadius" =>
    NpcBase::InteractionRadius, "InteractionResetTimer" =>
    NpcBase::InteractionResetTimer, "isNonSpawnedAvatar" => NpcBase::IsNonSpawnedAvatar,
    "isSelfRevivable" => NpcBase::IsSelfRevivable, "LastInteractionTime" =>
    NpcBase::LastInteractionTime, "LuaScript" => NpcBase::LuaScript, "lvl" =>
    NpcBase::Lvl, "MaterialOverride" => NpcBase::MaterialOverride, "nodelink" =>
    NpcBase::Nodelink, "originalNodeName" => NpcBase::OriginalNodeName,
    "originalZoneName" => NpcBase::OriginalZoneName, "partyGUID" => NpcBase::PartyGuid,
    "pathfindSafeSpawn" => NpcBase::PathfindSafeSpawn, "pos" => NpcBase::Pos, "Power" =>
    NpcBase::Power, "priority" => NpcBase::Priority, "QuestFlags" => NpcBase::QuestFlags,
    "ReadableName" => NpcBase::ReadableName, "respawnDelay" => NpcBase::RespawnDelay,
    "RespawnRegionName" => NpcBase::RespawnRegionName, "RespawnRegionNameOverride" =>
    NpcBase::RespawnRegionNameOverride, "rot" => NpcBase::Rot, "selfRadius" =>
    NpcBase::SelfRadius, "spawnMethod" => NpcBase::SpawnMethod, "spawnPosition" =>
    NpcBase::SpawnPosition, "spawnRotation" => NpcBase::SpawnRotation, "tags" =>
    NpcBase::Tags, "teamID" => NpcBase::TeamId, "UE3ClassID" => NpcBase::Ue3ClassId,
    "UE3EdVisual" => NpcBase::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    NpcBase::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    NpcBase::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    NpcBase::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    NpcBase::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    NpcBase::WorldZoneObjectIndex, "zone" => NpcBase::Zone, "ZoneGuid" =>
    NpcBase::ZoneGuid, "BitsMultiplier" => NpcBase::BitsMultiplier, "defb" =>
    NpcBase::Defb, "dropLevelVariance" => NpcBase::DropLevelVariance, "evadeRadius" =>
    NpcBase::EvadeRadius, "ExperienceMultiplier" => NpcBase::ExperienceMultiplier,
    "FragmentsMultiplier" => NpcBase::FragmentsMultiplier, "instanceGroup" =>
    NpcBase::InstanceGroup, "instanceGUID" => NpcBase::InstanceGuid, "interactRadius" =>
    NpcBase::InteractRadius, "isUnAttackable" => NpcBase::IsUnAttackable, "moveDest" =>
    NpcBase::MoveDest, "moveSpeed" => NpcBase::MoveSpeed, "nickName" =>
    NpcBase::NickName, "npcStateInfo" => NpcBase::NpcStateInfo, "npcType" =>
    NpcBase::NpcType, "orientation" => NpcBase::Orientation, "runSpeed" =>
    NpcBase::RunSpeed, "shuffleSpeed" => NpcBase::ShuffleSpeed, "SOMAMultiplier" =>
    NpcBase::SomaMultiplier, "spawnLeashRadius" => NpcBase::SpawnLeashRadius,
    "speakResponse" => NpcBase::SpeakResponse, "walkSpeed" => NpcBase::WalkSpeed,
};
pub(crate) static NPC_BASE_ATTRIBUTES_ID: phf::Map<u16, NpcBase> = phf_map! {
    1060u16 => NpcBase::Action0, 1061u16 => NpcBase::Action0Duration, 1046u16 =>
    NpcBase::Action0Option, 3492u16 => NpcBase::AlwaysVisibleToPlayers, 10524u16 =>
    NpcBase::AutoReviveDelay, 10464u16 => NpcBase::AutoReviveTime, 8243u16 =>
    NpcBase::AwareRange, 10935u16 => NpcBase::BeaconRadius, 1062u16 =>
    NpcBase::CollisionExtent, 1058u16 => NpcBase::ContentClass, 11070u16 =>
    NpcBase::CycleQuestBase, 7210u16 => NpcBase::DefaultWeapon, 9634u16 =>
    NpcBase::DespawnDelay, 8830u16 => NpcBase::Dialogs, 6596u16 => NpcBase::DisplayName,
    6822u16 => NpcBase::EnableInGame, 11194u16 => NpcBase::FreedomProperties, 1074u16 =>
    NpcBase::Freq, 1063u16 => NpcBase::GenerateInterestList, 1064u16 =>
    NpcBase::HiddenFromClients, 1040u16 => NpcBase::HiddenFromPlayers, 9048u16 =>
    NpcBase::HideAfterInteraction, 4350u16 => NpcBase::Icon, 1047u16 =>
    NpcBase::InstanceTags, 5559u16 => NpcBase::InstanceZoneKey, 11140u16 =>
    NpcBase::InteractionDuration, 7471u16 => NpcBase::InteractionRadius, 9050u16 =>
    NpcBase::InteractionResetTimer, 1032u16 => NpcBase::IsNonSpawnedAvatar, 7155u16 =>
    NpcBase::IsSelfRevivable, 9049u16 => NpcBase::LastInteractionTime, 7778u16 =>
    NpcBase::LuaScript, 6179u16 => NpcBase::Lvl, 4729u16 => NpcBase::MaterialOverride,
    1044u16 => NpcBase::Nodelink, 1034u16 => NpcBase::OriginalNodeName, 1035u16 =>
    NpcBase::OriginalZoneName, 1065u16 => NpcBase::PartyGuid, 1038u16 =>
    NpcBase::PathfindSafeSpawn, 1056u16 => NpcBase::Pos, 1073u16 => NpcBase::Power,
    1066u16 => NpcBase::Priority, 9932u16 => NpcBase::QuestFlags, 3676u16 =>
    NpcBase::ReadableName, 1031u16 => NpcBase::RespawnDelay, 10782u16 =>
    NpcBase::RespawnRegionName, 10841u16 => NpcBase::RespawnRegionNameOverride, 1067u16
    => NpcBase::Rot, 1068u16 => NpcBase::SelfRadius, 6097u16 => NpcBase::SpawnMethod,
    7833u16 => NpcBase::SpawnPosition, 8186u16 => NpcBase::SpawnRotation, 1069u16 =>
    NpcBase::Tags, 1070u16 => NpcBase::TeamId, 1059u16 => NpcBase::Ue3ClassId, 9808u16 =>
    NpcBase::Ue3EdVisual, 8586u16 => NpcBase::VisibleOnQuestAvailable, 8583u16 =>
    NpcBase::VisibleOnQuestComplete, 8584u16 => NpcBase::VisibleOnQuestFinished, 8585u16
    => NpcBase::VisibleOnQuestInProgress, 1033u16 => NpcBase::WorldZoneObjectIndex,
    1072u16 => NpcBase::Zone, 1037u16 => NpcBase::ZoneGuid, 11298u16 =>
    NpcBase::BitsMultiplier, 1048u16 => NpcBase::Defb, 9711u16 =>
    NpcBase::DropLevelVariance, 6876u16 => NpcBase::EvadeRadius, 11300u16 =>
    NpcBase::ExperienceMultiplier, 11297u16 => NpcBase::FragmentsMultiplier, 8041u16 =>
    NpcBase::InstanceGroup, 1050u16 => NpcBase::InstanceGuid, 4179u16 =>
    NpcBase::InteractRadius, 4933u16 => NpcBase::IsUnAttackable, 1051u16 =>
    NpcBase::MoveDest, 1052u16 => NpcBase::MoveSpeed, 1053u16 => NpcBase::NickName,
    7922u16 => NpcBase::NpcStateInfo, 1054u16 => NpcBase::NpcType, 1055u16 =>
    NpcBase::Orientation, 1045u16 => NpcBase::RunSpeed, 1041u16 => NpcBase::ShuffleSpeed,
    11299u16 => NpcBase::SomaMultiplier, 5301u16 => NpcBase::SpawnLeashRadius, 1057u16 =>
    NpcBase::SpeakResponse, 1042u16 => NpcBase::WalkSpeed,
};
impl Attribute for NpcBase {
    fn class() -> Class {
        Class::NpcBase
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
            Self::BitsMultiplier => &Self::BitsMultiplier,
            Self::Defb => &Self::Defb,
            Self::DropLevelVariance => &Self::DropLevelVariance,
            Self::EvadeRadius => &Self::EvadeRadius,
            Self::ExperienceMultiplier => &Self::ExperienceMultiplier,
            Self::FragmentsMultiplier => &Self::FragmentsMultiplier,
            Self::InstanceGroup => &Self::InstanceGroup,
            Self::InstanceGuid => &Self::InstanceGuid,
            Self::InteractRadius => &Self::InteractRadius,
            Self::IsUnAttackable => &Self::IsUnAttackable,
            Self::MoveDest => &Self::MoveDest,
            Self::MoveSpeed => &Self::MoveSpeed,
            Self::NickName => &Self::NickName,
            Self::NpcStateInfo => &Self::NpcStateInfo,
            Self::NpcType => &Self::NpcType,
            Self::Orientation => &Self::Orientation,
            Self::RunSpeed => &Self::RunSpeed,
            Self::ShuffleSpeed => &Self::ShuffleSpeed,
            Self::SomaMultiplier => &Self::SomaMultiplier,
            Self::SpawnLeashRadius => &Self::SpawnLeashRadius,
            Self::SpeakResponse => &Self::SpeakResponse,
            Self::WalkSpeed => &Self::WalkSpeed,
        }
    }
}
impl AttributeInfo for NpcBase {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 1060u16,
            Self::Action0Duration => 1061u16,
            Self::Action0Option => 1046u16,
            Self::AlwaysVisibleToPlayers => 3492u16,
            Self::AutoReviveDelay => 10524u16,
            Self::AutoReviveTime => 10464u16,
            Self::AwareRange => 8243u16,
            Self::BeaconRadius => 10935u16,
            Self::CollisionExtent => 1062u16,
            Self::ContentClass => 1058u16,
            Self::CycleQuestBase => 11070u16,
            Self::DefaultWeapon => 7210u16,
            Self::DespawnDelay => 9634u16,
            Self::Dialogs => 8830u16,
            Self::DisplayName => 6596u16,
            Self::EnableInGame => 6822u16,
            Self::FreedomProperties => 11194u16,
            Self::Freq => 1074u16,
            Self::GenerateInterestList => 1063u16,
            Self::HiddenFromClients => 1064u16,
            Self::HiddenFromPlayers => 1040u16,
            Self::HideAfterInteraction => 9048u16,
            Self::Icon => 4350u16,
            Self::InstanceTags => 1047u16,
            Self::InstanceZoneKey => 5559u16,
            Self::InteractionDuration => 11140u16,
            Self::InteractionRadius => 7471u16,
            Self::InteractionResetTimer => 9050u16,
            Self::IsNonSpawnedAvatar => 1032u16,
            Self::IsSelfRevivable => 7155u16,
            Self::LastInteractionTime => 9049u16,
            Self::LuaScript => 7778u16,
            Self::Lvl => 6179u16,
            Self::MaterialOverride => 4729u16,
            Self::Nodelink => 1044u16,
            Self::OriginalNodeName => 1034u16,
            Self::OriginalZoneName => 1035u16,
            Self::PartyGuid => 1065u16,
            Self::PathfindSafeSpawn => 1038u16,
            Self::Pos => 1056u16,
            Self::Power => 1073u16,
            Self::Priority => 1066u16,
            Self::QuestFlags => 9932u16,
            Self::ReadableName => 3676u16,
            Self::RespawnDelay => 1031u16,
            Self::RespawnRegionName => 10782u16,
            Self::RespawnRegionNameOverride => 10841u16,
            Self::Rot => 1067u16,
            Self::SelfRadius => 1068u16,
            Self::SpawnMethod => 6097u16,
            Self::SpawnPosition => 7833u16,
            Self::SpawnRotation => 8186u16,
            Self::Tags => 1069u16,
            Self::TeamId => 1070u16,
            Self::Ue3ClassId => 1059u16,
            Self::Ue3EdVisual => 9808u16,
            Self::VisibleOnQuestAvailable => 8586u16,
            Self::VisibleOnQuestComplete => 8583u16,
            Self::VisibleOnQuestFinished => 8584u16,
            Self::VisibleOnQuestInProgress => 8585u16,
            Self::WorldZoneObjectIndex => 1033u16,
            Self::Zone => 1072u16,
            Self::ZoneGuid => 1037u16,
            Self::BitsMultiplier => 11298u16,
            Self::Defb => 1048u16,
            Self::DropLevelVariance => 9711u16,
            Self::EvadeRadius => 6876u16,
            Self::ExperienceMultiplier => 11300u16,
            Self::FragmentsMultiplier => 11297u16,
            Self::InstanceGroup => 8041u16,
            Self::InstanceGuid => 1050u16,
            Self::InteractRadius => 4179u16,
            Self::IsUnAttackable => 4933u16,
            Self::MoveDest => 1051u16,
            Self::MoveSpeed => 1052u16,
            Self::NickName => 1053u16,
            Self::NpcStateInfo => 7922u16,
            Self::NpcType => 1054u16,
            Self::Orientation => 1055u16,
            Self::RunSpeed => 1045u16,
            Self::ShuffleSpeed => 1041u16,
            Self::SomaMultiplier => 11299u16,
            Self::SpawnLeashRadius => 5301u16,
            Self::SpeakResponse => 1057u16,
            Self::WalkSpeed => 1042u16,
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
            Self::BitsMultiplier => "BitsMultiplier",
            Self::Defb => "defb",
            Self::DropLevelVariance => "dropLevelVariance",
            Self::EvadeRadius => "evadeRadius",
            Self::ExperienceMultiplier => "ExperienceMultiplier",
            Self::FragmentsMultiplier => "FragmentsMultiplier",
            Self::InstanceGroup => "instanceGroup",
            Self::InstanceGuid => "instanceGUID",
            Self::InteractRadius => "interactRadius",
            Self::IsUnAttackable => "isUnAttackable",
            Self::MoveDest => "moveDest",
            Self::MoveSpeed => "moveSpeed",
            Self::NickName => "nickName",
            Self::NpcStateInfo => "npcStateInfo",
            Self::NpcType => "npcType",
            Self::Orientation => "orientation",
            Self::RunSpeed => "runSpeed",
            Self::ShuffleSpeed => "shuffleSpeed",
            Self::SomaMultiplier => "SOMAMultiplier",
            Self::SpawnLeashRadius => "spawnLeashRadius",
            Self::SpeakResponse => "speakResponse",
            Self::WalkSpeed => "walkSpeed",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::DisplayName => ParamType::LocalizedString,
            Self::BitsMultiplier => ParamType::Float,
            Self::Defb => ParamType::String,
            Self::DropLevelVariance => ParamType::Int,
            Self::EvadeRadius => ParamType::Float,
            Self::ExperienceMultiplier => ParamType::Float,
            Self::FragmentsMultiplier => ParamType::Float,
            Self::InstanceGroup => ParamType::InstanceGroup,
            Self::InstanceGuid => ParamType::Guid,
            Self::InteractRadius => ParamType::Float,
            Self::IsUnAttackable => ParamType::Bool,
            Self::MoveDest => ParamType::Vector3,
            Self::MoveSpeed => ParamType::Float,
            Self::NickName => ParamType::String,
            Self::NpcStateInfo => ParamType::String,
            Self::NpcType => ParamType::String,
            Self::Orientation => ParamType::Vector3,
            Self::RunSpeed => ParamType::Float,
            Self::ShuffleSpeed => ParamType::Float,
            Self::SomaMultiplier => ParamType::Float,
            Self::SpawnLeashRadius => ParamType::Float,
            Self::SpeakResponse => ParamType::String,
            Self::WalkSpeed => ParamType::Float,
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
        }
    }
    fn default(&self) -> &'static Value {
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static BITS_MULTIPLIER: Value = Value::Float(1f32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String("BaseNPC".to_string()));
        static DROP_LEVEL_VARIANCE: Value = Value::Int(0i32);
        static EVADE_RADIUS: Value = Value::Float(4000f32);
        static EXPERIENCE_MULTIPLIER: Value = Value::Float(1f32);
        static FRAGMENTS_MULTIPLIER: Value = Value::Float(1f32);
        static INSTANCE_GROUP: Lazy<Value> = Lazy::new(|| Value::InstanceGroup(
            String::default(),
        ));
        static INSTANCE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static INTERACT_RADIUS: Value = Value::Float(-1f32);
        static IS_UN_ATTACKABLE: Value = Value::Bool(false);
        static MOVE_DEST: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static MOVE_SPEED: Value = Value::Float(150f32);
        static NICK_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            "\u{7f}\u{7f}\u{7f}\u{7f}".to_string(),
        ));
        static NPC_STATE_INFO: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static NPC_TYPE: Lazy<Value> = Lazy::new(|| Value::String("Vendor".to_string()));
        static ORIENTATION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static RUN_SPEED: Value = Value::Float(337f32);
        static SHUFFLE_SPEED: Value = Value::Float(45f32);
        static SOMA_MULTIPLIER: Value = Value::Float(1f32);
        static SPAWN_LEASH_RADIUS: Value = Value::Float(2500f32);
        static SPEAK_RESPONSE: Lazy<Value> = Lazy::new(|| Value::String(
            "Hi there".to_string(),
        ));
        static WALK_SPEED: Value = Value::Float(80f32);
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
            "Engine.AtlasAvatar".to_string(),
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
        match self {
            Self::DisplayName => &DISPLAY_NAME,
            Self::BitsMultiplier => &BITS_MULTIPLIER,
            Self::Defb => &DEFB,
            Self::DropLevelVariance => &DROP_LEVEL_VARIANCE,
            Self::EvadeRadius => &EVADE_RADIUS,
            Self::ExperienceMultiplier => &EXPERIENCE_MULTIPLIER,
            Self::FragmentsMultiplier => &FRAGMENTS_MULTIPLIER,
            Self::InstanceGroup => &INSTANCE_GROUP,
            Self::InstanceGuid => &INSTANCE_GUID,
            Self::InteractRadius => &INTERACT_RADIUS,
            Self::IsUnAttackable => &IS_UN_ATTACKABLE,
            Self::MoveDest => &MOVE_DEST,
            Self::MoveSpeed => &MOVE_SPEED,
            Self::NickName => &NICK_NAME,
            Self::NpcStateInfo => &NPC_STATE_INFO,
            Self::NpcType => &NPC_TYPE,
            Self::Orientation => &ORIENTATION,
            Self::RunSpeed => &RUN_SPEED,
            Self::ShuffleSpeed => &SHUFFLE_SPEED,
            Self::SomaMultiplier => &SOMA_MULTIPLIER,
            Self::SpawnLeashRadius => &SPAWN_LEASH_RADIUS,
            Self::SpeakResponse => &SPEAK_RESPONSE,
            Self::WalkSpeed => &WALK_SPEED,
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
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::DisplayName => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BitsMultiplier => {
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
            Self::DropLevelVariance => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EvadeRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ExperienceMultiplier => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::FragmentsMultiplier => {
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
            Self::InstanceGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::InteractRadius => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::IsUnAttackable => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MoveDest => {
                &[ParamFlag::ClientOwn, ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::MoveSpeed => {
                &[
                    ParamFlag::ClientOwn,
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NickName => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::NpcStateInfo => &[ParamFlag::NodeOwn],
            Self::NpcType => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Orientation => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::RunSpeed => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ShuffleSpeed => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SomaMultiplier => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnLeashRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpeakResponse => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::Deprecated,
                ]
            }
            Self::WalkSpeed => {
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
        }
    }
}
impl FromStr for NpcBase {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NPC_BASE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for NpcBase {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            1060u16 => Ok(Self::Action0),
            1061u16 => Ok(Self::Action0Duration),
            1046u16 => Ok(Self::Action0Option),
            3492u16 => Ok(Self::AlwaysVisibleToPlayers),
            10524u16 => Ok(Self::AutoReviveDelay),
            10464u16 => Ok(Self::AutoReviveTime),
            8243u16 => Ok(Self::AwareRange),
            10935u16 => Ok(Self::BeaconRadius),
            1062u16 => Ok(Self::CollisionExtent),
            1058u16 => Ok(Self::ContentClass),
            11070u16 => Ok(Self::CycleQuestBase),
            7210u16 => Ok(Self::DefaultWeapon),
            9634u16 => Ok(Self::DespawnDelay),
            8830u16 => Ok(Self::Dialogs),
            6596u16 => Ok(Self::DisplayName),
            6822u16 => Ok(Self::EnableInGame),
            11194u16 => Ok(Self::FreedomProperties),
            1074u16 => Ok(Self::Freq),
            1063u16 => Ok(Self::GenerateInterestList),
            1064u16 => Ok(Self::HiddenFromClients),
            1040u16 => Ok(Self::HiddenFromPlayers),
            9048u16 => Ok(Self::HideAfterInteraction),
            4350u16 => Ok(Self::Icon),
            1047u16 => Ok(Self::InstanceTags),
            5559u16 => Ok(Self::InstanceZoneKey),
            11140u16 => Ok(Self::InteractionDuration),
            7471u16 => Ok(Self::InteractionRadius),
            9050u16 => Ok(Self::InteractionResetTimer),
            1032u16 => Ok(Self::IsNonSpawnedAvatar),
            7155u16 => Ok(Self::IsSelfRevivable),
            9049u16 => Ok(Self::LastInteractionTime),
            7778u16 => Ok(Self::LuaScript),
            6179u16 => Ok(Self::Lvl),
            4729u16 => Ok(Self::MaterialOverride),
            1044u16 => Ok(Self::Nodelink),
            1034u16 => Ok(Self::OriginalNodeName),
            1035u16 => Ok(Self::OriginalZoneName),
            1065u16 => Ok(Self::PartyGuid),
            1038u16 => Ok(Self::PathfindSafeSpawn),
            1056u16 => Ok(Self::Pos),
            1073u16 => Ok(Self::Power),
            1066u16 => Ok(Self::Priority),
            9932u16 => Ok(Self::QuestFlags),
            3676u16 => Ok(Self::ReadableName),
            1031u16 => Ok(Self::RespawnDelay),
            10782u16 => Ok(Self::RespawnRegionName),
            10841u16 => Ok(Self::RespawnRegionNameOverride),
            1067u16 => Ok(Self::Rot),
            1068u16 => Ok(Self::SelfRadius),
            6097u16 => Ok(Self::SpawnMethod),
            7833u16 => Ok(Self::SpawnPosition),
            8186u16 => Ok(Self::SpawnRotation),
            1069u16 => Ok(Self::Tags),
            1070u16 => Ok(Self::TeamId),
            1059u16 => Ok(Self::Ue3ClassId),
            9808u16 => Ok(Self::Ue3EdVisual),
            8586u16 => Ok(Self::VisibleOnQuestAvailable),
            8583u16 => Ok(Self::VisibleOnQuestComplete),
            8584u16 => Ok(Self::VisibleOnQuestFinished),
            8585u16 => Ok(Self::VisibleOnQuestInProgress),
            1033u16 => Ok(Self::WorldZoneObjectIndex),
            1072u16 => Ok(Self::Zone),
            1037u16 => Ok(Self::ZoneGuid),
            11298u16 => Ok(Self::BitsMultiplier),
            1048u16 => Ok(Self::Defb),
            9711u16 => Ok(Self::DropLevelVariance),
            6876u16 => Ok(Self::EvadeRadius),
            11300u16 => Ok(Self::ExperienceMultiplier),
            11297u16 => Ok(Self::FragmentsMultiplier),
            8041u16 => Ok(Self::InstanceGroup),
            1050u16 => Ok(Self::InstanceGuid),
            4179u16 => Ok(Self::InteractRadius),
            4933u16 => Ok(Self::IsUnAttackable),
            1051u16 => Ok(Self::MoveDest),
            1052u16 => Ok(Self::MoveSpeed),
            1053u16 => Ok(Self::NickName),
            7922u16 => Ok(Self::NpcStateInfo),
            1054u16 => Ok(Self::NpcType),
            1055u16 => Ok(Self::Orientation),
            1045u16 => Ok(Self::RunSpeed),
            1041u16 => Ok(Self::ShuffleSpeed),
            11299u16 => Ok(Self::SomaMultiplier),
            5301u16 => Ok(Self::SpawnLeashRadius),
            1057u16 => Ok(Self::SpeakResponse),
            1042u16 => Ok(Self::WalkSpeed),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
