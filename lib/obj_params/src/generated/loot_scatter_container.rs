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
pub enum LootScatterContainer {
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
    AllowAvatar,
    AllowParty,
    BlingAmount,
    IsQuestItem,
    ItemContentGuid,
    ItemContentName,
    ItemCount,
    Rarity,
    RarityOverride,
    ScatterLootVisualType,
    SomaAmount,
    SomaType,
}
pub(crate) static LOOT_SCATTER_CONTAINER_ATTRIBUTES: phf::Map<
    &'static str,
    LootScatterContainer,
> = phf_map! {
    "action0" => LootScatterContainer::Action0, "action0Duration" =>
    LootScatterContainer::Action0Duration, "action0Option" =>
    LootScatterContainer::Action0Option, "alwaysVisibleToPlayers" =>
    LootScatterContainer::AlwaysVisibleToPlayers, "autoReviveDelay" =>
    LootScatterContainer::AutoReviveDelay, "autoReviveTime" =>
    LootScatterContainer::AutoReviveTime, "AwareRange" =>
    LootScatterContainer::AwareRange, "BeaconRadius" =>
    LootScatterContainer::BeaconRadius, "collisionExtent" =>
    LootScatterContainer::CollisionExtent, "ContentClass" =>
    LootScatterContainer::ContentClass, "CycleQuestBase" =>
    LootScatterContainer::CycleQuestBase, "defaultWeapon" =>
    LootScatterContainer::DefaultWeapon, "despawnDelay" =>
    LootScatterContainer::DespawnDelay, "Dialogs" => LootScatterContainer::Dialogs,
    "DisplayName" => LootScatterContainer::DisplayName, "EnableInGame" =>
    LootScatterContainer::EnableInGame, "FreedomProperties" =>
    LootScatterContainer::FreedomProperties, "Freq" => LootScatterContainer::Freq,
    "generateInterestList" => LootScatterContainer::GenerateInterestList,
    "hiddenFromClients" => LootScatterContainer::HiddenFromClients, "hiddenFromPlayers"
    => LootScatterContainer::HiddenFromPlayers, "HideAfterInteraction" =>
    LootScatterContainer::HideAfterInteraction, "Icon" => LootScatterContainer::Icon,
    "instanceTags" => LootScatterContainer::InstanceTags, "instanceZoneKey" =>
    LootScatterContainer::InstanceZoneKey, "InteractionDuration" =>
    LootScatterContainer::InteractionDuration, "InteractionRadius" =>
    LootScatterContainer::InteractionRadius, "InteractionResetTimer" =>
    LootScatterContainer::InteractionResetTimer, "isNonSpawnedAvatar" =>
    LootScatterContainer::IsNonSpawnedAvatar, "isSelfRevivable" =>
    LootScatterContainer::IsSelfRevivable, "LastInteractionTime" =>
    LootScatterContainer::LastInteractionTime, "LuaScript" =>
    LootScatterContainer::LuaScript, "lvl" => LootScatterContainer::Lvl,
    "MaterialOverride" => LootScatterContainer::MaterialOverride, "nodelink" =>
    LootScatterContainer::Nodelink, "originalNodeName" =>
    LootScatterContainer::OriginalNodeName, "originalZoneName" =>
    LootScatterContainer::OriginalZoneName, "partyGUID" =>
    LootScatterContainer::PartyGuid, "pathfindSafeSpawn" =>
    LootScatterContainer::PathfindSafeSpawn, "pos" => LootScatterContainer::Pos, "Power"
    => LootScatterContainer::Power, "priority" => LootScatterContainer::Priority,
    "QuestFlags" => LootScatterContainer::QuestFlags, "ReadableName" =>
    LootScatterContainer::ReadableName, "respawnDelay" =>
    LootScatterContainer::RespawnDelay, "RespawnRegionName" =>
    LootScatterContainer::RespawnRegionName, "RespawnRegionNameOverride" =>
    LootScatterContainer::RespawnRegionNameOverride, "rot" => LootScatterContainer::Rot,
    "selfRadius" => LootScatterContainer::SelfRadius, "spawnMethod" =>
    LootScatterContainer::SpawnMethod, "spawnPosition" =>
    LootScatterContainer::SpawnPosition, "spawnRotation" =>
    LootScatterContainer::SpawnRotation, "tags" => LootScatterContainer::Tags, "teamID"
    => LootScatterContainer::TeamId, "UE3ClassID" => LootScatterContainer::Ue3ClassId,
    "UE3EdVisual" => LootScatterContainer::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    LootScatterContainer::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    LootScatterContainer::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    LootScatterContainer::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    LootScatterContainer::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    LootScatterContainer::WorldZoneObjectIndex, "zone" => LootScatterContainer::Zone,
    "ZoneGuid" => LootScatterContainer::ZoneGuid, "awareDist" =>
    LootScatterContainer::AwareDist, "defb" => LootScatterContainer::Defb,
    "instanceGroup" => LootScatterContainer::InstanceGroup, "isUnAttackable" =>
    LootScatterContainer::IsUnAttackable, "abilities" => LootScatterContainer::Abilities,
    "alive" => LootScatterContainer::Alive, "attackedBy" =>
    LootScatterContainer::AttackedBy, "carrierGuid" => LootScatterContainer::CarrierGuid,
    "clientLoadingPriority" => LootScatterContainer::ClientLoadingPriority,
    "directorTags" => LootScatterContainer::DirectorTags, "forceSpawnOnClient" =>
    LootScatterContainer::ForceSpawnOnClient, "hpCur" => LootScatterContainer::HpCur,
    "hpMax" => LootScatterContainer::HpMax, "isLocked" => LootScatterContainer::IsLocked,
    "spawnerAvatarGuid" => LootScatterContainer::SpawnerAvatarGuid, "spawnerAvatarID" =>
    LootScatterContainer::SpawnerAvatarId, "AllowAvatar" =>
    LootScatterContainer::AllowAvatar, "AllowParty" => LootScatterContainer::AllowParty,
    "BlingAmount" => LootScatterContainer::BlingAmount, "IsQuestItem" =>
    LootScatterContainer::IsQuestItem, "ItemContentGuid" =>
    LootScatterContainer::ItemContentGuid, "ItemContentName" =>
    LootScatterContainer::ItemContentName, "ItemCount" =>
    LootScatterContainer::ItemCount, "Rarity" => LootScatterContainer::Rarity,
    "rarityOverride" => LootScatterContainer::RarityOverride, "ScatterLootVisualType" =>
    LootScatterContainer::ScatterLootVisualType, "SomaAmount" =>
    LootScatterContainer::SomaAmount, "SomaType" => LootScatterContainer::SomaType,
};
pub(crate) static LOOT_SCATTER_CONTAINER_ATTRIBUTES_ID: phf::Map<
    u16,
    LootScatterContainer,
> = phf_map! {
    7996u16 => LootScatterContainer::Action0, 7995u16 =>
    LootScatterContainer::Action0Duration, 8006u16 =>
    LootScatterContainer::Action0Option, 7979u16 =>
    LootScatterContainer::AlwaysVisibleToPlayers, 10565u16 =>
    LootScatterContainer::AutoReviveDelay, 10505u16 =>
    LootScatterContainer::AutoReviveTime, 8284u16 => LootScatterContainer::AwareRange,
    10977u16 => LootScatterContainer::BeaconRadius, 7994u16 =>
    LootScatterContainer::CollisionExtent, 7998u16 => LootScatterContainer::ContentClass,
    11063u16 => LootScatterContainer::CycleQuestBase, 7956u16 =>
    LootScatterContainer::DefaultWeapon, 9675u16 => LootScatterContainer::DespawnDelay,
    8871u16 => LootScatterContainer::Dialogs, 7959u16 =>
    LootScatterContainer::DisplayName, 7958u16 => LootScatterContainer::EnableInGame,
    11187u16 => LootScatterContainer::FreedomProperties, 7981u16 =>
    LootScatterContainer::Freq, 7993u16 => LootScatterContainer::GenerateInterestList,
    7992u16 => LootScatterContainer::HiddenFromClients, 8008u16 =>
    LootScatterContainer::HiddenFromPlayers, 9171u16 =>
    LootScatterContainer::HideAfterInteraction, 7976u16 => LootScatterContainer::Icon,
    8005u16 => LootScatterContainer::InstanceTags, 7965u16 =>
    LootScatterContainer::InstanceZoneKey, 11133u16 =>
    LootScatterContainer::InteractionDuration, 7955u16 =>
    LootScatterContainer::InteractionRadius, 9173u16 =>
    LootScatterContainer::InteractionResetTimer, 8018u16 =>
    LootScatterContainer::IsNonSpawnedAvatar, 7957u16 =>
    LootScatterContainer::IsSelfRevivable, 9172u16 =>
    LootScatterContainer::LastInteractionTime, 7953u16 =>
    LootScatterContainer::LuaScript, 7960u16 => LootScatterContainer::Lvl, 7969u16 =>
    LootScatterContainer::MaterialOverride, 8007u16 => LootScatterContainer::Nodelink,
    8016u16 => LootScatterContainer::OriginalNodeName, 8015u16 =>
    LootScatterContainer::OriginalZoneName, 7991u16 => LootScatterContainer::PartyGuid,
    8009u16 => LootScatterContainer::PathfindSafeSpawn, 7990u16 =>
    LootScatterContainer::Pos, 7982u16 => LootScatterContainer::Power, 7989u16 =>
    LootScatterContainer::Priority, 9973u16 => LootScatterContainer::QuestFlags, 7978u16
    => LootScatterContainer::ReadableName, 8019u16 => LootScatterContainer::RespawnDelay,
    10824u16 => LootScatterContainer::RespawnRegionName, 10883u16 =>
    LootScatterContainer::RespawnRegionNameOverride, 7988u16 =>
    LootScatterContainer::Rot, 7987u16 => LootScatterContainer::SelfRadius, 7961u16 =>
    LootScatterContainer::SpawnMethod, 7952u16 => LootScatterContainer::SpawnPosition,
    8227u16 => LootScatterContainer::SpawnRotation, 7986u16 =>
    LootScatterContainer::Tags, 7985u16 => LootScatterContainer::TeamId, 7997u16 =>
    LootScatterContainer::Ue3ClassId, 9849u16 => LootScatterContainer::Ue3EdVisual,
    8750u16 => LootScatterContainer::VisibleOnQuestAvailable, 8747u16 =>
    LootScatterContainer::VisibleOnQuestComplete, 8748u16 =>
    LootScatterContainer::VisibleOnQuestFinished, 8749u16 =>
    LootScatterContainer::VisibleOnQuestInProgress, 8017u16 =>
    LootScatterContainer::WorldZoneObjectIndex, 7983u16 => LootScatterContainer::Zone,
    8010u16 => LootScatterContainer::ZoneGuid, 8013u16 =>
    LootScatterContainer::AwareDist, 8000u16 => LootScatterContainer::Defb, 11383u16 =>
    LootScatterContainer::InstanceGroup, 12441u16 =>
    LootScatterContainer::IsUnAttackable, 9344u16 => LootScatterContainer::Abilities,
    8004u16 => LootScatterContainer::Alive, 8003u16 => LootScatterContainer::AttackedBy,
    8011u16 => LootScatterContainer::CarrierGuid, 11286u16 =>
    LootScatterContainer::ClientLoadingPriority, 8101u16 =>
    LootScatterContainer::DirectorTags, 8012u16 =>
    LootScatterContainer::ForceSpawnOnClient, 8002u16 => LootScatterContainer::HpCur,
    8001u16 => LootScatterContainer::HpMax, 7966u16 => LootScatterContainer::IsLocked,
    7962u16 => LootScatterContainer::SpawnerAvatarGuid, 7954u16 =>
    LootScatterContainer::SpawnerAvatarId, 8024u16 => LootScatterContainer::AllowAvatar,
    8023u16 => LootScatterContainer::AllowParty, 7949u16 =>
    LootScatterContainer::BlingAmount, 9223u16 => LootScatterContainer::IsQuestItem,
    11004u16 => LootScatterContainer::ItemContentGuid, 7951u16 =>
    LootScatterContainer::ItemContentName, 7950u16 => LootScatterContainer::ItemCount,
    11150u16 => LootScatterContainer::Rarity, 11014u16 =>
    LootScatterContainer::RarityOverride, 11027u16 =>
    LootScatterContainer::ScatterLootVisualType, 11097u16 =>
    LootScatterContainer::SomaAmount, 11096u16 => LootScatterContainer::SomaType,
};
impl Attribute for LootScatterContainer {
    fn class() -> Class {
        Class::LootScatterContainer
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
            Self::AllowAvatar => &Self::AllowAvatar,
            Self::AllowParty => &Self::AllowParty,
            Self::BlingAmount => &Self::BlingAmount,
            Self::IsQuestItem => &Self::IsQuestItem,
            Self::ItemContentGuid => &Self::ItemContentGuid,
            Self::ItemContentName => &Self::ItemContentName,
            Self::ItemCount => &Self::ItemCount,
            Self::Rarity => &Self::Rarity,
            Self::RarityOverride => &Self::RarityOverride,
            Self::ScatterLootVisualType => &Self::ScatterLootVisualType,
            Self::SomaAmount => &Self::SomaAmount,
            Self::SomaType => &Self::SomaType,
        }
    }
}
impl AttributeInfo for LootScatterContainer {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 7996u16,
            Self::Action0Duration => 7995u16,
            Self::Action0Option => 8006u16,
            Self::AlwaysVisibleToPlayers => 7979u16,
            Self::AutoReviveDelay => 10565u16,
            Self::AutoReviveTime => 10505u16,
            Self::AwareRange => 8284u16,
            Self::BeaconRadius => 10977u16,
            Self::CollisionExtent => 7994u16,
            Self::ContentClass => 7998u16,
            Self::CycleQuestBase => 11063u16,
            Self::DefaultWeapon => 7956u16,
            Self::DespawnDelay => 9675u16,
            Self::Dialogs => 8871u16,
            Self::DisplayName => 7959u16,
            Self::EnableInGame => 7958u16,
            Self::FreedomProperties => 11187u16,
            Self::Freq => 7981u16,
            Self::GenerateInterestList => 7993u16,
            Self::HiddenFromClients => 7992u16,
            Self::HiddenFromPlayers => 8008u16,
            Self::HideAfterInteraction => 9171u16,
            Self::Icon => 7976u16,
            Self::InstanceTags => 8005u16,
            Self::InstanceZoneKey => 7965u16,
            Self::InteractionDuration => 11133u16,
            Self::InteractionRadius => 7955u16,
            Self::InteractionResetTimer => 9173u16,
            Self::IsNonSpawnedAvatar => 8018u16,
            Self::IsSelfRevivable => 7957u16,
            Self::LastInteractionTime => 9172u16,
            Self::LuaScript => 7953u16,
            Self::Lvl => 7960u16,
            Self::MaterialOverride => 7969u16,
            Self::Nodelink => 8007u16,
            Self::OriginalNodeName => 8016u16,
            Self::OriginalZoneName => 8015u16,
            Self::PartyGuid => 7991u16,
            Self::PathfindSafeSpawn => 8009u16,
            Self::Pos => 7990u16,
            Self::Power => 7982u16,
            Self::Priority => 7989u16,
            Self::QuestFlags => 9973u16,
            Self::ReadableName => 7978u16,
            Self::RespawnDelay => 8019u16,
            Self::RespawnRegionName => 10824u16,
            Self::RespawnRegionNameOverride => 10883u16,
            Self::Rot => 7988u16,
            Self::SelfRadius => 7987u16,
            Self::SpawnMethod => 7961u16,
            Self::SpawnPosition => 7952u16,
            Self::SpawnRotation => 8227u16,
            Self::Tags => 7986u16,
            Self::TeamId => 7985u16,
            Self::Ue3ClassId => 7997u16,
            Self::Ue3EdVisual => 9849u16,
            Self::VisibleOnQuestAvailable => 8750u16,
            Self::VisibleOnQuestComplete => 8747u16,
            Self::VisibleOnQuestFinished => 8748u16,
            Self::VisibleOnQuestInProgress => 8749u16,
            Self::WorldZoneObjectIndex => 8017u16,
            Self::Zone => 7983u16,
            Self::ZoneGuid => 8010u16,
            Self::AwareDist => 8013u16,
            Self::Defb => 8000u16,
            Self::InstanceGroup => 11383u16,
            Self::IsUnAttackable => 12441u16,
            Self::Abilities => 9344u16,
            Self::Alive => 8004u16,
            Self::AttackedBy => 8003u16,
            Self::CarrierGuid => 8011u16,
            Self::ClientLoadingPriority => 11286u16,
            Self::DirectorTags => 8101u16,
            Self::ForceSpawnOnClient => 8012u16,
            Self::HpCur => 8002u16,
            Self::HpMax => 8001u16,
            Self::IsLocked => 7966u16,
            Self::SpawnerAvatarGuid => 7962u16,
            Self::SpawnerAvatarId => 7954u16,
            Self::AllowAvatar => 8024u16,
            Self::AllowParty => 8023u16,
            Self::BlingAmount => 7949u16,
            Self::IsQuestItem => 9223u16,
            Self::ItemContentGuid => 11004u16,
            Self::ItemContentName => 7951u16,
            Self::ItemCount => 7950u16,
            Self::Rarity => 11150u16,
            Self::RarityOverride => 11014u16,
            Self::ScatterLootVisualType => 11027u16,
            Self::SomaAmount => 11097u16,
            Self::SomaType => 11096u16,
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
            Self::AllowAvatar => "AllowAvatar",
            Self::AllowParty => "AllowParty",
            Self::BlingAmount => "BlingAmount",
            Self::IsQuestItem => "IsQuestItem",
            Self::ItemContentGuid => "ItemContentGuid",
            Self::ItemContentName => "ItemContentName",
            Self::ItemCount => "ItemCount",
            Self::Rarity => "Rarity",
            Self::RarityOverride => "rarityOverride",
            Self::ScatterLootVisualType => "ScatterLootVisualType",
            Self::SomaAmount => "SomaAmount",
            Self::SomaType => "SomaType",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Ue3ClassId => ParamType::String,
            Self::Defb => ParamType::String,
            Self::AllowAvatar => ParamType::AvatarId,
            Self::AllowParty => ParamType::Guid,
            Self::BlingAmount => ParamType::Int,
            Self::IsQuestItem => ParamType::Bool,
            Self::ItemContentGuid => ParamType::Guid,
            Self::ItemContentName => ParamType::String,
            Self::ItemCount => ParamType::Int,
            Self::Rarity => ParamType::Int,
            Self::RarityOverride => ParamType::String,
            Self::ScatterLootVisualType => ParamType::Int,
            Self::SomaAmount => ParamType::Int,
            Self::SomaType => ParamType::Int,
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
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Otherland.OLStructureAvatarScatterContainer".to_string(),
        ));
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String(
            "LootScatterContainer".to_string(),
        ));
        static ALLOW_AVATAR: Value = Value::AvatarId(AvatarId::from_u64(0));
        static ALLOW_PARTY: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static BLING_AMOUNT: Value = Value::Int(0i32);
        static IS_QUEST_ITEM: Value = Value::Bool(false);
        static ITEM_CONTENT_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ITEM_CONTENT_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ITEM_COUNT: Value = Value::Int(0i32);
        static RARITY: Value = Value::Int(0i32);
        static RARITY_OVERRIDE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SCATTER_LOOT_VISUAL_TYPE: Value = Value::Int(0i32);
        static SOMA_AMOUNT: Value = Value::Int(0i32);
        static SOMA_TYPE: Value = Value::Int(0i32);
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
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::Defb => &DEFB,
            Self::AllowAvatar => &ALLOW_AVATAR,
            Self::AllowParty => &ALLOW_PARTY,
            Self::BlingAmount => &BLING_AMOUNT,
            Self::IsQuestItem => &IS_QUEST_ITEM,
            Self::ItemContentGuid => &ITEM_CONTENT_GUID,
            Self::ItemContentName => &ITEM_CONTENT_NAME,
            Self::ItemCount => &ITEM_COUNT,
            Self::Rarity => &RARITY,
            Self::RarityOverride => &RARITY_OVERRIDE,
            Self::ScatterLootVisualType => &SCATTER_LOOT_VISUAL_TYPE,
            Self::SomaAmount => &SOMA_AMOUNT,
            Self::SomaType => &SOMA_TYPE,
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
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Defb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AllowAvatar => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AllowParty => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::BlingAmount => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsQuestItem => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ItemContentGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ItemContentName => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ItemCount => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Rarity => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::RarityOverride => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ScatterLootVisualType => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SomaAmount => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SomaType => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
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
impl FromStr for LootScatterContainer {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LOOT_SCATTER_CONTAINER_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for LootScatterContainer {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            7996u16 => Ok(Self::Action0),
            7995u16 => Ok(Self::Action0Duration),
            8006u16 => Ok(Self::Action0Option),
            7979u16 => Ok(Self::AlwaysVisibleToPlayers),
            10565u16 => Ok(Self::AutoReviveDelay),
            10505u16 => Ok(Self::AutoReviveTime),
            8284u16 => Ok(Self::AwareRange),
            10977u16 => Ok(Self::BeaconRadius),
            7994u16 => Ok(Self::CollisionExtent),
            7998u16 => Ok(Self::ContentClass),
            11063u16 => Ok(Self::CycleQuestBase),
            7956u16 => Ok(Self::DefaultWeapon),
            9675u16 => Ok(Self::DespawnDelay),
            8871u16 => Ok(Self::Dialogs),
            7959u16 => Ok(Self::DisplayName),
            7958u16 => Ok(Self::EnableInGame),
            11187u16 => Ok(Self::FreedomProperties),
            7981u16 => Ok(Self::Freq),
            7993u16 => Ok(Self::GenerateInterestList),
            7992u16 => Ok(Self::HiddenFromClients),
            8008u16 => Ok(Self::HiddenFromPlayers),
            9171u16 => Ok(Self::HideAfterInteraction),
            7976u16 => Ok(Self::Icon),
            8005u16 => Ok(Self::InstanceTags),
            7965u16 => Ok(Self::InstanceZoneKey),
            11133u16 => Ok(Self::InteractionDuration),
            7955u16 => Ok(Self::InteractionRadius),
            9173u16 => Ok(Self::InteractionResetTimer),
            8018u16 => Ok(Self::IsNonSpawnedAvatar),
            7957u16 => Ok(Self::IsSelfRevivable),
            9172u16 => Ok(Self::LastInteractionTime),
            7953u16 => Ok(Self::LuaScript),
            7960u16 => Ok(Self::Lvl),
            7969u16 => Ok(Self::MaterialOverride),
            8007u16 => Ok(Self::Nodelink),
            8016u16 => Ok(Self::OriginalNodeName),
            8015u16 => Ok(Self::OriginalZoneName),
            7991u16 => Ok(Self::PartyGuid),
            8009u16 => Ok(Self::PathfindSafeSpawn),
            7990u16 => Ok(Self::Pos),
            7982u16 => Ok(Self::Power),
            7989u16 => Ok(Self::Priority),
            9973u16 => Ok(Self::QuestFlags),
            7978u16 => Ok(Self::ReadableName),
            8019u16 => Ok(Self::RespawnDelay),
            10824u16 => Ok(Self::RespawnRegionName),
            10883u16 => Ok(Self::RespawnRegionNameOverride),
            7988u16 => Ok(Self::Rot),
            7987u16 => Ok(Self::SelfRadius),
            7961u16 => Ok(Self::SpawnMethod),
            7952u16 => Ok(Self::SpawnPosition),
            8227u16 => Ok(Self::SpawnRotation),
            7986u16 => Ok(Self::Tags),
            7985u16 => Ok(Self::TeamId),
            7997u16 => Ok(Self::Ue3ClassId),
            9849u16 => Ok(Self::Ue3EdVisual),
            8750u16 => Ok(Self::VisibleOnQuestAvailable),
            8747u16 => Ok(Self::VisibleOnQuestComplete),
            8748u16 => Ok(Self::VisibleOnQuestFinished),
            8749u16 => Ok(Self::VisibleOnQuestInProgress),
            8017u16 => Ok(Self::WorldZoneObjectIndex),
            7983u16 => Ok(Self::Zone),
            8010u16 => Ok(Self::ZoneGuid),
            8013u16 => Ok(Self::AwareDist),
            8000u16 => Ok(Self::Defb),
            11383u16 => Ok(Self::InstanceGroup),
            12441u16 => Ok(Self::IsUnAttackable),
            9344u16 => Ok(Self::Abilities),
            8004u16 => Ok(Self::Alive),
            8003u16 => Ok(Self::AttackedBy),
            8011u16 => Ok(Self::CarrierGuid),
            11286u16 => Ok(Self::ClientLoadingPriority),
            8101u16 => Ok(Self::DirectorTags),
            8012u16 => Ok(Self::ForceSpawnOnClient),
            8002u16 => Ok(Self::HpCur),
            8001u16 => Ok(Self::HpMax),
            7966u16 => Ok(Self::IsLocked),
            7962u16 => Ok(Self::SpawnerAvatarGuid),
            7954u16 => Ok(Self::SpawnerAvatarId),
            8024u16 => Ok(Self::AllowAvatar),
            8023u16 => Ok(Self::AllowParty),
            7949u16 => Ok(Self::BlingAmount),
            9223u16 => Ok(Self::IsQuestItem),
            11004u16 => Ok(Self::ItemContentGuid),
            7951u16 => Ok(Self::ItemContentName),
            7950u16 => Ok(Self::ItemCount),
            11150u16 => Ok(Self::Rarity),
            11014u16 => Ok(Self::RarityOverride),
            11027u16 => Ok(Self::ScatterLootVisualType),
            11097u16 => Ok(Self::SomaAmount),
            11096u16 => Ok(Self::SomaType),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
