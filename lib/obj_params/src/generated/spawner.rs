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
pub enum Spawner {
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
    ExactCount,
    InstanceGroup,
    IsUnAttackable,
    SameRespawnType,
    SpawnInWave,
    Alive,
    AttackedBy,
    Charge,
    ChildSpawner,
    ClanGuid,
    ContentClassName,
    CulledSpawnsPerNearbyPlayer,
    Defb,
    DirectorTags,
    DirectorTagsAdded,
    DoModelPopulationChange,
    EncounterFormation,
    EncounterId,
    EncounterRole,
    Faction,
    ForceSpawnOnRestart,
    GeneralDifficulty,
    GenerateEncounter,
    GroupDifficulty,
    HpCur,
    HpMax,
    IsShardObject,
    LevelOffset,
    LootTable,
    MaxCount,
    MinCount,
    NumPerSpawn,
    OverrideDifficulty,
    OverrideLootTable,
    PopulationChangePersistentSpawnTable,
    PostSpawnStateDuration,
    RunOnce,
    SpawnAccelerationFactor,
    SpawnAccelerationLinear,
    SpawnCount,
    SpawnDelay,
    SpawnedAvatarGuids,
    SpawneeBehaviorConfig,
    SpawneeCombatNotifyRadius,
    SpawneeDefb,
    SpawneeLeashRadius,
    SpawnerAvatarGuid,
    SpawnerAvatarId,
    SpawnFreqency,
    SpawnMyLandLootItems,
    SpawnNodes,
    SpawnNpc1,
    SpawnNpc10,
    SpawnNpc2,
    SpawnNpc3,
    SpawnNpc4,
    SpawnNpc5,
    SpawnNpc6,
    SpawnNpc7,
    SpawnNpc8,
    SpawnNpc9,
    SpawnPersistentAvatars,
    SpawnPointAllowOverSpawn,
    SpawnPointDistance,
    SpawnPointMaxSpawnee,
    SpawnPointPositionNoise,
    SpawnPrecise,
    SpawnRadius,
    SpawnRadiusMin,
    SpawnSleepingAvatars,
    SpawnTime,
    SpawnTimeSecondsPerLevel,
    UseSpawnTimeAcceleration,
    WaveCount,
    WaveDelay,
    WaveDelayStartMinCount,
}
pub(crate) static SPAWNER_ATTRIBUTES: phf::Map<&'static str, Spawner> = phf_map! {
    "action0" => Spawner::Action0, "action0Duration" => Spawner::Action0Duration,
    "action0Option" => Spawner::Action0Option, "alwaysVisibleToPlayers" =>
    Spawner::AlwaysVisibleToPlayers, "autoReviveDelay" => Spawner::AutoReviveDelay,
    "autoReviveTime" => Spawner::AutoReviveTime, "AwareRange" => Spawner::AwareRange,
    "BeaconRadius" => Spawner::BeaconRadius, "collisionExtent" =>
    Spawner::CollisionExtent, "ContentClass" => Spawner::ContentClass, "CycleQuestBase"
    => Spawner::CycleQuestBase, "defaultWeapon" => Spawner::DefaultWeapon, "despawnDelay"
    => Spawner::DespawnDelay, "Dialogs" => Spawner::Dialogs, "DisplayName" =>
    Spawner::DisplayName, "EnableInGame" => Spawner::EnableInGame, "FreedomProperties" =>
    Spawner::FreedomProperties, "Freq" => Spawner::Freq, "generateInterestList" =>
    Spawner::GenerateInterestList, "hiddenFromClients" => Spawner::HiddenFromClients,
    "hiddenFromPlayers" => Spawner::HiddenFromPlayers, "HideAfterInteraction" =>
    Spawner::HideAfterInteraction, "Icon" => Spawner::Icon, "instanceTags" =>
    Spawner::InstanceTags, "instanceZoneKey" => Spawner::InstanceZoneKey,
    "InteractionDuration" => Spawner::InteractionDuration, "InteractionRadius" =>
    Spawner::InteractionRadius, "InteractionResetTimer" =>
    Spawner::InteractionResetTimer, "isNonSpawnedAvatar" => Spawner::IsNonSpawnedAvatar,
    "isSelfRevivable" => Spawner::IsSelfRevivable, "LastInteractionTime" =>
    Spawner::LastInteractionTime, "LuaScript" => Spawner::LuaScript, "lvl" =>
    Spawner::Lvl, "MaterialOverride" => Spawner::MaterialOverride, "nodelink" =>
    Spawner::Nodelink, "originalNodeName" => Spawner::OriginalNodeName,
    "originalZoneName" => Spawner::OriginalZoneName, "partyGUID" => Spawner::PartyGuid,
    "pathfindSafeSpawn" => Spawner::PathfindSafeSpawn, "pos" => Spawner::Pos, "Power" =>
    Spawner::Power, "priority" => Spawner::Priority, "QuestFlags" => Spawner::QuestFlags,
    "ReadableName" => Spawner::ReadableName, "respawnDelay" => Spawner::RespawnDelay,
    "RespawnRegionName" => Spawner::RespawnRegionName, "RespawnRegionNameOverride" =>
    Spawner::RespawnRegionNameOverride, "rot" => Spawner::Rot, "selfRadius" =>
    Spawner::SelfRadius, "spawnMethod" => Spawner::SpawnMethod, "spawnPosition" =>
    Spawner::SpawnPosition, "spawnRotation" => Spawner::SpawnRotation, "tags" =>
    Spawner::Tags, "teamID" => Spawner::TeamId, "UE3ClassID" => Spawner::Ue3ClassId,
    "UE3EdVisual" => Spawner::Ue3EdVisual, "VisibleOnQuestAvailable" =>
    Spawner::VisibleOnQuestAvailable, "VisibleOnQuestComplete" =>
    Spawner::VisibleOnQuestComplete, "VisibleOnQuestFinished" =>
    Spawner::VisibleOnQuestFinished, "VisibleOnQuestInProgress" =>
    Spawner::VisibleOnQuestInProgress, "WorldZoneObjectIndex" =>
    Spawner::WorldZoneObjectIndex, "zone" => Spawner::Zone, "ZoneGuid" =>
    Spawner::ZoneGuid, "exactCount" => Spawner::ExactCount, "instanceGroup" =>
    Spawner::InstanceGroup, "isUnAttackable" => Spawner::IsUnAttackable,
    "sameRespawnType" => Spawner::SameRespawnType, "spawnInWave" => Spawner::SpawnInWave,
    "alive" => Spawner::Alive, "attackedBy" => Spawner::AttackedBy, "charge" =>
    Spawner::Charge, "childSpawner" => Spawner::ChildSpawner, "clanGUID" =>
    Spawner::ClanGuid, "contentClassName" => Spawner::ContentClassName,
    "culledSpawnsPerNearbyPlayer" => Spawner::CulledSpawnsPerNearbyPlayer, "defb" =>
    Spawner::Defb, "directorTags" => Spawner::DirectorTags, "directorTagsAdded" =>
    Spawner::DirectorTagsAdded, "doModelPopulationChange" =>
    Spawner::DoModelPopulationChange, "encounterFormation" =>
    Spawner::EncounterFormation, "encounterID" => Spawner::EncounterId, "encounterRole"
    => Spawner::EncounterRole, "Faction" => Spawner::Faction, "forceSpawnOnRestart" =>
    Spawner::ForceSpawnOnRestart, "generalDifficulty" => Spawner::GeneralDifficulty,
    "generateEncounter" => Spawner::GenerateEncounter, "groupDifficulty" =>
    Spawner::GroupDifficulty, "hpCur" => Spawner::HpCur, "hpMax" => Spawner::HpMax,
    "isShardObject" => Spawner::IsShardObject, "levelOffset" => Spawner::LevelOffset,
    "lootTable" => Spawner::LootTable, "maxCount" => Spawner::MaxCount, "minCount" =>
    Spawner::MinCount, "NumPerSpawn" => Spawner::NumPerSpawn, "overrideDifficulty" =>
    Spawner::OverrideDifficulty, "overrideLootTable" => Spawner::OverrideLootTable,
    "populationChangePersistentSpawnTable" =>
    Spawner::PopulationChangePersistentSpawnTable, "postSpawnStateDuration" =>
    Spawner::PostSpawnStateDuration, "runOnce" => Spawner::RunOnce,
    "spawnAccelerationFactor" => Spawner::SpawnAccelerationFactor,
    "spawnAccelerationLinear" => Spawner::SpawnAccelerationLinear, "spawnCount" =>
    Spawner::SpawnCount, "spawnDelay" => Spawner::SpawnDelay, "spawnedAvatarGuids" =>
    Spawner::SpawnedAvatarGuids, "spawneeBehaviorConfig" =>
    Spawner::SpawneeBehaviorConfig, "spawneeCombatNotifyRadius" =>
    Spawner::SpawneeCombatNotifyRadius, "spawneeDefb" => Spawner::SpawneeDefb,
    "spawneeLeashRadius" => Spawner::SpawneeLeashRadius, "spawnerAvatarGuid" =>
    Spawner::SpawnerAvatarGuid, "spawnerAvatarID" => Spawner::SpawnerAvatarId,
    "spawnFreqency" => Spawner::SpawnFreqency, "spawnMyLandLootItems" =>
    Spawner::SpawnMyLandLootItems, "spawnNodes" => Spawner::SpawnNodes, "spawnNPC1" =>
    Spawner::SpawnNpc1, "spawnNPC10" => Spawner::SpawnNpc10, "spawnNPC2" =>
    Spawner::SpawnNpc2, "spawnNPC3" => Spawner::SpawnNpc3, "spawnNPC4" =>
    Spawner::SpawnNpc4, "spawnNPC5" => Spawner::SpawnNpc5, "spawnNPC6" =>
    Spawner::SpawnNpc6, "spawnNPC7" => Spawner::SpawnNpc7, "spawnNPC8" =>
    Spawner::SpawnNpc8, "spawnNPC9" => Spawner::SpawnNpc9, "spawnPersistentAvatars" =>
    Spawner::SpawnPersistentAvatars, "spawnPointAllowOverSpawn" =>
    Spawner::SpawnPointAllowOverSpawn, "spawnPointDistance" =>
    Spawner::SpawnPointDistance, "spawnPointMaxSpawnee" => Spawner::SpawnPointMaxSpawnee,
    "spawnPointPositionNoise" => Spawner::SpawnPointPositionNoise, "spawnPrecise" =>
    Spawner::SpawnPrecise, "spawnRadius" => Spawner::SpawnRadius, "spawnRadiusMin" =>
    Spawner::SpawnRadiusMin, "spawnSleepingAvatars" => Spawner::SpawnSleepingAvatars,
    "spawnTime" => Spawner::SpawnTime, "spawnTimeSecondsPerLevel" =>
    Spawner::SpawnTimeSecondsPerLevel, "useSpawnTimeAcceleration" =>
    Spawner::UseSpawnTimeAcceleration, "waveCount" => Spawner::WaveCount, "waveDelay" =>
    Spawner::WaveDelay, "waveDelayStartMinCount" => Spawner::WaveDelayStartMinCount,
};
pub(crate) static SPAWNER_ATTRIBUTES_ID: phf::Map<u16, Spawner> = phf_map! {
    955u16 => Spawner::Action0, 956u16 => Spawner::Action0Duration, 944u16 =>
    Spawner::Action0Option, 3522u16 => Spawner::AlwaysVisibleToPlayers, 10522u16 =>
    Spawner::AutoReviveDelay, 10462u16 => Spawner::AutoReviveTime, 8241u16 =>
    Spawner::AwareRange, 10933u16 => Spawner::BeaconRadius, 957u16 =>
    Spawner::CollisionExtent, 953u16 => Spawner::ContentClass, 11067u16 =>
    Spawner::CycleQuestBase, 7253u16 => Spawner::DefaultWeapon, 9632u16 =>
    Spawner::DespawnDelay, 8828u16 => Spawner::Dialogs, 6639u16 => Spawner::DisplayName,
    6865u16 => Spawner::EnableInGame, 11191u16 => Spawner::FreedomProperties, 970u16 =>
    Spawner::Freq, 958u16 => Spawner::GenerateInterestList, 959u16 =>
    Spawner::HiddenFromClients, 942u16 => Spawner::HiddenFromPlayers, 9042u16 =>
    Spawner::HideAfterInteraction, 4382u16 => Spawner::Icon, 946u16 =>
    Spawner::InstanceTags, 5599u16 => Spawner::InstanceZoneKey, 11137u16 =>
    Spawner::InteractionDuration, 7514u16 => Spawner::InteractionRadius, 9044u16 =>
    Spawner::InteractionResetTimer, 916u16 => Spawner::IsNonSpawnedAvatar, 7198u16 =>
    Spawner::IsSelfRevivable, 9043u16 => Spawner::LastInteractionTime, 7776u16 =>
    Spawner::LuaScript, 6222u16 => Spawner::Lvl, 4763u16 => Spawner::MaterialOverride,
    943u16 => Spawner::Nodelink, 918u16 => Spawner::OriginalNodeName, 919u16 =>
    Spawner::OriginalZoneName, 960u16 => Spawner::PartyGuid, 939u16 =>
    Spawner::PathfindSafeSpawn, 961u16 => Spawner::Pos, 969u16 => Spawner::Power, 962u16
    => Spawner::Priority, 9930u16 => Spawner::QuestFlags, 3707u16 =>
    Spawner::ReadableName, 915u16 => Spawner::RespawnDelay, 10780u16 =>
    Spawner::RespawnRegionName, 10839u16 => Spawner::RespawnRegionNameOverride, 963u16 =>
    Spawner::Rot, 964u16 => Spawner::SelfRadius, 6140u16 => Spawner::SpawnMethod, 7831u16
    => Spawner::SpawnPosition, 8184u16 => Spawner::SpawnRotation, 965u16 =>
    Spawner::Tags, 966u16 => Spawner::TeamId, 954u16 => Spawner::Ue3ClassId, 9806u16 =>
    Spawner::Ue3EdVisual, 8578u16 => Spawner::VisibleOnQuestAvailable, 8575u16 =>
    Spawner::VisibleOnQuestComplete, 8576u16 => Spawner::VisibleOnQuestFinished, 8577u16
    => Spawner::VisibleOnQuestInProgress, 917u16 => Spawner::WorldZoneObjectIndex, 968u16
    => Spawner::Zone, 935u16 => Spawner::ZoneGuid, 5687u16 => Spawner::ExactCount,
    11388u16 => Spawner::InstanceGroup, 10384u16 => Spawner::IsUnAttackable, 5686u16 =>
    Spawner::SameRespawnType, 5619u16 => Spawner::SpawnInWave, 5898u16 => Spawner::Alive,
    11315u16 => Spawner::AttackedBy, 4855u16 => Spawner::Charge, 920u16 =>
    Spawner::ChildSpawner, 11207u16 => Spawner::ClanGuid, 945u16 =>
    Spawner::ContentClassName, 8933u16 => Spawner::CulledSpawnsPerNearbyPlayer, 947u16 =>
    Spawner::Defb, 8063u16 => Spawner::DirectorTags, 8178u16 =>
    Spawner::DirectorTagsAdded, 3667u16 => Spawner::DoModelPopulationChange, 4843u16 =>
    Spawner::EncounterFormation, 4844u16 => Spawner::EncounterId, 4842u16 =>
    Spawner::EncounterRole, 11226u16 => Spawner::Faction, 4925u16 =>
    Spawner::ForceSpawnOnRestart, 11224u16 => Spawner::GeneralDifficulty, 5902u16 =>
    Spawner::GenerateEncounter, 11225u16 => Spawner::GroupDifficulty, 5897u16 =>
    Spawner::HpCur, 5896u16 => Spawner::HpMax, 3662u16 => Spawner::IsShardObject,
    11217u16 => Spawner::LevelOffset, 8033u16 => Spawner::LootTable, 948u16 =>
    Spawner::MaxCount, 4926u16 => Spawner::MinCount, 941u16 => Spawner::NumPerSpawn,
    11223u16 => Spawner::OverrideDifficulty, 11228u16 => Spawner::OverrideLootTable,
    3663u16 => Spawner::PopulationChangePersistentSpawnTable, 10113u16 =>
    Spawner::PostSpawnStateDuration, 5939u16 => Spawner::RunOnce, 10120u16 =>
    Spawner::SpawnAccelerationFactor, 10119u16 => Spawner::SpawnAccelerationLinear,
    5909u16 => Spawner::SpawnCount, 949u16 => Spawner::SpawnDelay, 5911u16 =>
    Spawner::SpawnedAvatarGuids, 938u16 => Spawner::SpawneeBehaviorConfig, 11234u16 =>
    Spawner::SpawneeCombatNotifyRadius, 11227u16 => Spawner::SpawneeDefb, 11235u16 =>
    Spawner::SpawneeLeashRadius, 5907u16 => Spawner::SpawnerAvatarGuid, 5905u16 =>
    Spawner::SpawnerAvatarId, 922u16 => Spawner::SpawnFreqency, 9217u16 =>
    Spawner::SpawnMyLandLootItems, 921u16 => Spawner::SpawnNodes, 924u16 =>
    Spawner::SpawnNpc1, 933u16 => Spawner::SpawnNpc10, 925u16 => Spawner::SpawnNpc2,
    926u16 => Spawner::SpawnNpc3, 927u16 => Spawner::SpawnNpc4, 928u16 =>
    Spawner::SpawnNpc5, 929u16 => Spawner::SpawnNpc6, 930u16 => Spawner::SpawnNpc7,
    931u16 => Spawner::SpawnNpc8, 932u16 => Spawner::SpawnNpc9, 5915u16 =>
    Spawner::SpawnPersistentAvatars, 11292u16 => Spawner::SpawnPointAllowOverSpawn,
    11238u16 => Spawner::SpawnPointDistance, 11236u16 => Spawner::SpawnPointMaxSpawnee,
    11237u16 => Spawner::SpawnPointPositionNoise, 951u16 => Spawner::SpawnPrecise, 952u16
    => Spawner::SpawnRadius, 6160u16 => Spawner::SpawnRadiusMin, 6090u16 =>
    Spawner::SpawnSleepingAvatars, 923u16 => Spawner::SpawnTime, 10911u16 =>
    Spawner::SpawnTimeSecondsPerLevel, 10109u16 => Spawner::UseSpawnTimeAcceleration,
    11249u16 => Spawner::WaveCount, 11251u16 => Spawner::WaveDelay, 11250u16 =>
    Spawner::WaveDelayStartMinCount,
};
impl Attribute for Spawner {
    fn class() -> Class {
        Class::Spawner
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
            Self::ExactCount => &Self::ExactCount,
            Self::InstanceGroup => &Self::InstanceGroup,
            Self::IsUnAttackable => &Self::IsUnAttackable,
            Self::SameRespawnType => &Self::SameRespawnType,
            Self::SpawnInWave => &Self::SpawnInWave,
            Self::Alive => &Self::Alive,
            Self::AttackedBy => &Self::AttackedBy,
            Self::Charge => &Self::Charge,
            Self::ChildSpawner => &Self::ChildSpawner,
            Self::ClanGuid => &Self::ClanGuid,
            Self::ContentClassName => &Self::ContentClassName,
            Self::CulledSpawnsPerNearbyPlayer => &Self::CulledSpawnsPerNearbyPlayer,
            Self::Defb => &Self::Defb,
            Self::DirectorTags => &Self::DirectorTags,
            Self::DirectorTagsAdded => &Self::DirectorTagsAdded,
            Self::DoModelPopulationChange => &Self::DoModelPopulationChange,
            Self::EncounterFormation => &Self::EncounterFormation,
            Self::EncounterId => &Self::EncounterId,
            Self::EncounterRole => &Self::EncounterRole,
            Self::Faction => &Self::Faction,
            Self::ForceSpawnOnRestart => &Self::ForceSpawnOnRestart,
            Self::GeneralDifficulty => &Self::GeneralDifficulty,
            Self::GenerateEncounter => &Self::GenerateEncounter,
            Self::GroupDifficulty => &Self::GroupDifficulty,
            Self::HpCur => &Self::HpCur,
            Self::HpMax => &Self::HpMax,
            Self::IsShardObject => &Self::IsShardObject,
            Self::LevelOffset => &Self::LevelOffset,
            Self::LootTable => &Self::LootTable,
            Self::MaxCount => &Self::MaxCount,
            Self::MinCount => &Self::MinCount,
            Self::NumPerSpawn => &Self::NumPerSpawn,
            Self::OverrideDifficulty => &Self::OverrideDifficulty,
            Self::OverrideLootTable => &Self::OverrideLootTable,
            Self::PopulationChangePersistentSpawnTable => {
                &Self::PopulationChangePersistentSpawnTable
            }
            Self::PostSpawnStateDuration => &Self::PostSpawnStateDuration,
            Self::RunOnce => &Self::RunOnce,
            Self::SpawnAccelerationFactor => &Self::SpawnAccelerationFactor,
            Self::SpawnAccelerationLinear => &Self::SpawnAccelerationLinear,
            Self::SpawnCount => &Self::SpawnCount,
            Self::SpawnDelay => &Self::SpawnDelay,
            Self::SpawnedAvatarGuids => &Self::SpawnedAvatarGuids,
            Self::SpawneeBehaviorConfig => &Self::SpawneeBehaviorConfig,
            Self::SpawneeCombatNotifyRadius => &Self::SpawneeCombatNotifyRadius,
            Self::SpawneeDefb => &Self::SpawneeDefb,
            Self::SpawneeLeashRadius => &Self::SpawneeLeashRadius,
            Self::SpawnerAvatarGuid => &Self::SpawnerAvatarGuid,
            Self::SpawnerAvatarId => &Self::SpawnerAvatarId,
            Self::SpawnFreqency => &Self::SpawnFreqency,
            Self::SpawnMyLandLootItems => &Self::SpawnMyLandLootItems,
            Self::SpawnNodes => &Self::SpawnNodes,
            Self::SpawnNpc1 => &Self::SpawnNpc1,
            Self::SpawnNpc10 => &Self::SpawnNpc10,
            Self::SpawnNpc2 => &Self::SpawnNpc2,
            Self::SpawnNpc3 => &Self::SpawnNpc3,
            Self::SpawnNpc4 => &Self::SpawnNpc4,
            Self::SpawnNpc5 => &Self::SpawnNpc5,
            Self::SpawnNpc6 => &Self::SpawnNpc6,
            Self::SpawnNpc7 => &Self::SpawnNpc7,
            Self::SpawnNpc8 => &Self::SpawnNpc8,
            Self::SpawnNpc9 => &Self::SpawnNpc9,
            Self::SpawnPersistentAvatars => &Self::SpawnPersistentAvatars,
            Self::SpawnPointAllowOverSpawn => &Self::SpawnPointAllowOverSpawn,
            Self::SpawnPointDistance => &Self::SpawnPointDistance,
            Self::SpawnPointMaxSpawnee => &Self::SpawnPointMaxSpawnee,
            Self::SpawnPointPositionNoise => &Self::SpawnPointPositionNoise,
            Self::SpawnPrecise => &Self::SpawnPrecise,
            Self::SpawnRadius => &Self::SpawnRadius,
            Self::SpawnRadiusMin => &Self::SpawnRadiusMin,
            Self::SpawnSleepingAvatars => &Self::SpawnSleepingAvatars,
            Self::SpawnTime => &Self::SpawnTime,
            Self::SpawnTimeSecondsPerLevel => &Self::SpawnTimeSecondsPerLevel,
            Self::UseSpawnTimeAcceleration => &Self::UseSpawnTimeAcceleration,
            Self::WaveCount => &Self::WaveCount,
            Self::WaveDelay => &Self::WaveDelay,
            Self::WaveDelayStartMinCount => &Self::WaveDelayStartMinCount,
        }
    }
}
impl AttributeInfo for Spawner {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Action0 => 955u16,
            Self::Action0Duration => 956u16,
            Self::Action0Option => 944u16,
            Self::AlwaysVisibleToPlayers => 3522u16,
            Self::AutoReviveDelay => 10522u16,
            Self::AutoReviveTime => 10462u16,
            Self::AwareRange => 8241u16,
            Self::BeaconRadius => 10933u16,
            Self::CollisionExtent => 957u16,
            Self::ContentClass => 953u16,
            Self::CycleQuestBase => 11067u16,
            Self::DefaultWeapon => 7253u16,
            Self::DespawnDelay => 9632u16,
            Self::Dialogs => 8828u16,
            Self::DisplayName => 6639u16,
            Self::EnableInGame => 6865u16,
            Self::FreedomProperties => 11191u16,
            Self::Freq => 970u16,
            Self::GenerateInterestList => 958u16,
            Self::HiddenFromClients => 959u16,
            Self::HiddenFromPlayers => 942u16,
            Self::HideAfterInteraction => 9042u16,
            Self::Icon => 4382u16,
            Self::InstanceTags => 946u16,
            Self::InstanceZoneKey => 5599u16,
            Self::InteractionDuration => 11137u16,
            Self::InteractionRadius => 7514u16,
            Self::InteractionResetTimer => 9044u16,
            Self::IsNonSpawnedAvatar => 916u16,
            Self::IsSelfRevivable => 7198u16,
            Self::LastInteractionTime => 9043u16,
            Self::LuaScript => 7776u16,
            Self::Lvl => 6222u16,
            Self::MaterialOverride => 4763u16,
            Self::Nodelink => 943u16,
            Self::OriginalNodeName => 918u16,
            Self::OriginalZoneName => 919u16,
            Self::PartyGuid => 960u16,
            Self::PathfindSafeSpawn => 939u16,
            Self::Pos => 961u16,
            Self::Power => 969u16,
            Self::Priority => 962u16,
            Self::QuestFlags => 9930u16,
            Self::ReadableName => 3707u16,
            Self::RespawnDelay => 915u16,
            Self::RespawnRegionName => 10780u16,
            Self::RespawnRegionNameOverride => 10839u16,
            Self::Rot => 963u16,
            Self::SelfRadius => 964u16,
            Self::SpawnMethod => 6140u16,
            Self::SpawnPosition => 7831u16,
            Self::SpawnRotation => 8184u16,
            Self::Tags => 965u16,
            Self::TeamId => 966u16,
            Self::Ue3ClassId => 954u16,
            Self::Ue3EdVisual => 9806u16,
            Self::VisibleOnQuestAvailable => 8578u16,
            Self::VisibleOnQuestComplete => 8575u16,
            Self::VisibleOnQuestFinished => 8576u16,
            Self::VisibleOnQuestInProgress => 8577u16,
            Self::WorldZoneObjectIndex => 917u16,
            Self::Zone => 968u16,
            Self::ZoneGuid => 935u16,
            Self::ExactCount => 5687u16,
            Self::InstanceGroup => 11388u16,
            Self::IsUnAttackable => 10384u16,
            Self::SameRespawnType => 5686u16,
            Self::SpawnInWave => 5619u16,
            Self::Alive => 5898u16,
            Self::AttackedBy => 11315u16,
            Self::Charge => 4855u16,
            Self::ChildSpawner => 920u16,
            Self::ClanGuid => 11207u16,
            Self::ContentClassName => 945u16,
            Self::CulledSpawnsPerNearbyPlayer => 8933u16,
            Self::Defb => 947u16,
            Self::DirectorTags => 8063u16,
            Self::DirectorTagsAdded => 8178u16,
            Self::DoModelPopulationChange => 3667u16,
            Self::EncounterFormation => 4843u16,
            Self::EncounterId => 4844u16,
            Self::EncounterRole => 4842u16,
            Self::Faction => 11226u16,
            Self::ForceSpawnOnRestart => 4925u16,
            Self::GeneralDifficulty => 11224u16,
            Self::GenerateEncounter => 5902u16,
            Self::GroupDifficulty => 11225u16,
            Self::HpCur => 5897u16,
            Self::HpMax => 5896u16,
            Self::IsShardObject => 3662u16,
            Self::LevelOffset => 11217u16,
            Self::LootTable => 8033u16,
            Self::MaxCount => 948u16,
            Self::MinCount => 4926u16,
            Self::NumPerSpawn => 941u16,
            Self::OverrideDifficulty => 11223u16,
            Self::OverrideLootTable => 11228u16,
            Self::PopulationChangePersistentSpawnTable => 3663u16,
            Self::PostSpawnStateDuration => 10113u16,
            Self::RunOnce => 5939u16,
            Self::SpawnAccelerationFactor => 10120u16,
            Self::SpawnAccelerationLinear => 10119u16,
            Self::SpawnCount => 5909u16,
            Self::SpawnDelay => 949u16,
            Self::SpawnedAvatarGuids => 5911u16,
            Self::SpawneeBehaviorConfig => 938u16,
            Self::SpawneeCombatNotifyRadius => 11234u16,
            Self::SpawneeDefb => 11227u16,
            Self::SpawneeLeashRadius => 11235u16,
            Self::SpawnerAvatarGuid => 5907u16,
            Self::SpawnerAvatarId => 5905u16,
            Self::SpawnFreqency => 922u16,
            Self::SpawnMyLandLootItems => 9217u16,
            Self::SpawnNodes => 921u16,
            Self::SpawnNpc1 => 924u16,
            Self::SpawnNpc10 => 933u16,
            Self::SpawnNpc2 => 925u16,
            Self::SpawnNpc3 => 926u16,
            Self::SpawnNpc4 => 927u16,
            Self::SpawnNpc5 => 928u16,
            Self::SpawnNpc6 => 929u16,
            Self::SpawnNpc7 => 930u16,
            Self::SpawnNpc8 => 931u16,
            Self::SpawnNpc9 => 932u16,
            Self::SpawnPersistentAvatars => 5915u16,
            Self::SpawnPointAllowOverSpawn => 11292u16,
            Self::SpawnPointDistance => 11238u16,
            Self::SpawnPointMaxSpawnee => 11236u16,
            Self::SpawnPointPositionNoise => 11237u16,
            Self::SpawnPrecise => 951u16,
            Self::SpawnRadius => 952u16,
            Self::SpawnRadiusMin => 6160u16,
            Self::SpawnSleepingAvatars => 6090u16,
            Self::SpawnTime => 923u16,
            Self::SpawnTimeSecondsPerLevel => 10911u16,
            Self::UseSpawnTimeAcceleration => 10109u16,
            Self::WaveCount => 11249u16,
            Self::WaveDelay => 11251u16,
            Self::WaveDelayStartMinCount => 11250u16,
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
            Self::ExactCount => "exactCount",
            Self::InstanceGroup => "instanceGroup",
            Self::IsUnAttackable => "isUnAttackable",
            Self::SameRespawnType => "sameRespawnType",
            Self::SpawnInWave => "spawnInWave",
            Self::Alive => "alive",
            Self::AttackedBy => "attackedBy",
            Self::Charge => "charge",
            Self::ChildSpawner => "childSpawner",
            Self::ClanGuid => "clanGUID",
            Self::ContentClassName => "contentClassName",
            Self::CulledSpawnsPerNearbyPlayer => "culledSpawnsPerNearbyPlayer",
            Self::Defb => "defb",
            Self::DirectorTags => "directorTags",
            Self::DirectorTagsAdded => "directorTagsAdded",
            Self::DoModelPopulationChange => "doModelPopulationChange",
            Self::EncounterFormation => "encounterFormation",
            Self::EncounterId => "encounterID",
            Self::EncounterRole => "encounterRole",
            Self::Faction => "Faction",
            Self::ForceSpawnOnRestart => "forceSpawnOnRestart",
            Self::GeneralDifficulty => "generalDifficulty",
            Self::GenerateEncounter => "generateEncounter",
            Self::GroupDifficulty => "groupDifficulty",
            Self::HpCur => "hpCur",
            Self::HpMax => "hpMax",
            Self::IsShardObject => "isShardObject",
            Self::LevelOffset => "levelOffset",
            Self::LootTable => "lootTable",
            Self::MaxCount => "maxCount",
            Self::MinCount => "minCount",
            Self::NumPerSpawn => "NumPerSpawn",
            Self::OverrideDifficulty => "overrideDifficulty",
            Self::OverrideLootTable => "overrideLootTable",
            Self::PopulationChangePersistentSpawnTable => {
                "populationChangePersistentSpawnTable"
            }
            Self::PostSpawnStateDuration => "postSpawnStateDuration",
            Self::RunOnce => "runOnce",
            Self::SpawnAccelerationFactor => "spawnAccelerationFactor",
            Self::SpawnAccelerationLinear => "spawnAccelerationLinear",
            Self::SpawnCount => "spawnCount",
            Self::SpawnDelay => "spawnDelay",
            Self::SpawnedAvatarGuids => "spawnedAvatarGuids",
            Self::SpawneeBehaviorConfig => "spawneeBehaviorConfig",
            Self::SpawneeCombatNotifyRadius => "spawneeCombatNotifyRadius",
            Self::SpawneeDefb => "spawneeDefb",
            Self::SpawneeLeashRadius => "spawneeLeashRadius",
            Self::SpawnerAvatarGuid => "spawnerAvatarGuid",
            Self::SpawnerAvatarId => "spawnerAvatarID",
            Self::SpawnFreqency => "spawnFreqency",
            Self::SpawnMyLandLootItems => "spawnMyLandLootItems",
            Self::SpawnNodes => "spawnNodes",
            Self::SpawnNpc1 => "spawnNPC1",
            Self::SpawnNpc10 => "spawnNPC10",
            Self::SpawnNpc2 => "spawnNPC2",
            Self::SpawnNpc3 => "spawnNPC3",
            Self::SpawnNpc4 => "spawnNPC4",
            Self::SpawnNpc5 => "spawnNPC5",
            Self::SpawnNpc6 => "spawnNPC6",
            Self::SpawnNpc7 => "spawnNPC7",
            Self::SpawnNpc8 => "spawnNPC8",
            Self::SpawnNpc9 => "spawnNPC9",
            Self::SpawnPersistentAvatars => "spawnPersistentAvatars",
            Self::SpawnPointAllowOverSpawn => "spawnPointAllowOverSpawn",
            Self::SpawnPointDistance => "spawnPointDistance",
            Self::SpawnPointMaxSpawnee => "spawnPointMaxSpawnee",
            Self::SpawnPointPositionNoise => "spawnPointPositionNoise",
            Self::SpawnPrecise => "spawnPrecise",
            Self::SpawnRadius => "spawnRadius",
            Self::SpawnRadiusMin => "spawnRadiusMin",
            Self::SpawnSleepingAvatars => "spawnSleepingAvatars",
            Self::SpawnTime => "spawnTime",
            Self::SpawnTimeSecondsPerLevel => "spawnTimeSecondsPerLevel",
            Self::UseSpawnTimeAcceleration => "useSpawnTimeAcceleration",
            Self::WaveCount => "waveCount",
            Self::WaveDelay => "waveDelay",
            Self::WaveDelayStartMinCount => "waveDelayStartMinCount",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Ue3EdVisual => ParamType::String,
            Self::SpawnInWave => ParamType::Bool,
            Self::Alive => ParamType::Bool,
            Self::AttackedBy => ParamType::AvatarId,
            Self::Charge => ParamType::Int,
            Self::ChildSpawner => ParamType::String,
            Self::ClanGuid => ParamType::Guid,
            Self::ContentClassName => ParamType::String,
            Self::CulledSpawnsPerNearbyPlayer => ParamType::Float,
            Self::Defb => ParamType::String,
            Self::DirectorTags => ParamType::String,
            Self::DirectorTagsAdded => ParamType::String,
            Self::DoModelPopulationChange => ParamType::Bool,
            Self::EncounterFormation => ParamType::String,
            Self::EncounterId => ParamType::String,
            Self::EncounterRole => ParamType::String,
            Self::Faction => ParamType::ContentRefList,
            Self::ForceSpawnOnRestart => ParamType::Bool,
            Self::GeneralDifficulty => ParamType::Int,
            Self::GenerateEncounter => ParamType::Bool,
            Self::GroupDifficulty => ParamType::Int,
            Self::HpCur => ParamType::Int,
            Self::HpMax => ParamType::Int,
            Self::IsShardObject => ParamType::Bool,
            Self::LevelOffset => ParamType::Int,
            Self::LootTable => ParamType::JsonValue,
            Self::MaxCount => ParamType::Int,
            Self::MinCount => ParamType::Int,
            Self::NumPerSpawn => ParamType::Int,
            Self::OverrideDifficulty => ParamType::Bool,
            Self::OverrideLootTable => ParamType::Bool,
            Self::PopulationChangePersistentSpawnTable => ParamType::VectorInt64,
            Self::PostSpawnStateDuration => ParamType::Float,
            Self::RunOnce => ParamType::Bool,
            Self::SpawnAccelerationFactor => ParamType::Float,
            Self::SpawnAccelerationLinear => ParamType::Float,
            Self::SpawnCount => ParamType::Int,
            Self::SpawnDelay => ParamType::FloatRange,
            Self::SpawnedAvatarGuids => ParamType::VectorGuid,
            Self::SpawneeBehaviorConfig => ParamType::JsonValue,
            Self::SpawneeCombatNotifyRadius => ParamType::Float,
            Self::SpawneeDefb => ParamType::String,
            Self::SpawneeLeashRadius => ParamType::Float,
            Self::SpawnerAvatarGuid => ParamType::Guid,
            Self::SpawnerAvatarId => ParamType::AvatarId,
            Self::SpawnFreqency => ParamType::Int,
            Self::SpawnMyLandLootItems => ParamType::Bool,
            Self::SpawnNodes => ParamType::String,
            Self::SpawnNpc1 => ParamType::ContentRefAndInt,
            Self::SpawnNpc10 => ParamType::ContentRefAndInt,
            Self::SpawnNpc2 => ParamType::ContentRefAndInt,
            Self::SpawnNpc3 => ParamType::ContentRefAndInt,
            Self::SpawnNpc4 => ParamType::ContentRefAndInt,
            Self::SpawnNpc5 => ParamType::ContentRefAndInt,
            Self::SpawnNpc6 => ParamType::ContentRefAndInt,
            Self::SpawnNpc7 => ParamType::ContentRefAndInt,
            Self::SpawnNpc8 => ParamType::ContentRefAndInt,
            Self::SpawnNpc9 => ParamType::ContentRefAndInt,
            Self::SpawnPersistentAvatars => ParamType::Bool,
            Self::SpawnPointAllowOverSpawn => ParamType::Bool,
            Self::SpawnPointDistance => ParamType::Float,
            Self::SpawnPointMaxSpawnee => ParamType::Int,
            Self::SpawnPointPositionNoise => ParamType::Float,
            Self::SpawnPrecise => ParamType::ContentRefList,
            Self::SpawnRadius => ParamType::Float,
            Self::SpawnRadiusMin => ParamType::Float,
            Self::SpawnSleepingAvatars => ParamType::Bool,
            Self::SpawnTime => ParamType::String,
            Self::SpawnTimeSecondsPerLevel => ParamType::Float,
            Self::UseSpawnTimeAcceleration => ParamType::Bool,
            Self::WaveCount => ParamType::Int,
            Self::WaveDelay => ParamType::FloatRange,
            Self::WaveDelayStartMinCount => ParamType::Int,
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
            Self::VisibleOnQuestAvailable => ParamType::VectorInt,
            Self::VisibleOnQuestComplete => ParamType::VectorInt,
            Self::VisibleOnQuestFinished => ParamType::VectorInt,
            Self::VisibleOnQuestInProgress => ParamType::VectorInt,
            Self::WorldZoneObjectIndex => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
            Self::ExactCount => ParamType::Bool,
            Self::InstanceGroup => ParamType::InstanceGroup,
            Self::IsUnAttackable => ParamType::Bool,
            Self::SameRespawnType => ParamType::Bool,
        }
    }
    fn default(&self) -> &'static Value {
        static UE_3_ED_VISUAL: Lazy<Value> = Lazy::new(|| Value::String(
            "ContentInfoIcons.Textures.Spawner".to_string(),
        ));
        static SPAWN_IN_WAVE: Value = Value::Bool(false);
        static ALIVE: Value = Value::Bool(true);
        static ATTACKED_BY: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static CHARGE: Value = Value::Int(-1i32);
        static CHILD_SPAWNER: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CLAN_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static CONTENT_CLASS_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CULLED_SPAWNS_PER_NEARBY_PLAYER: Value = Value::Float(0f32);
        static DEFB: Lazy<Value> = Lazy::new(|| Value::String("spawner".to_string()));
        static DIRECTOR_TAGS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static DIRECTOR_TAGS_ADDED: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static DO_MODEL_POPULATION_CHANGE: Value = Value::Bool(false);
        static ENCOUNTER_FORMATION: Lazy<Value> = Lazy::new(|| Value::String(
            "None".to_string(),
        ));
        static ENCOUNTER_ID: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ENCOUNTER_ROLE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static FACTION: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static FORCE_SPAWN_ON_RESTART: Value = Value::Bool(false);
        static GENERAL_DIFFICULTY: Value = Value::Int(1i32);
        static GENERATE_ENCOUNTER: Value = Value::Bool(false);
        static GROUP_DIFFICULTY: Value = Value::Int(0i32);
        static HP_CUR: Value = Value::Int(0i32);
        static HP_MAX: Value = Value::Int(2000i32);
        static IS_SHARD_OBJECT: Value = Value::Bool(false);
        static LEVEL_OFFSET: Value = Value::Int(0i32);
        static LOOT_TABLE: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static MAX_COUNT: Value = Value::Int(3i32);
        static MIN_COUNT: Value = Value::Int(0i32);
        static NUM_PER_SPAWN: Value = Value::Int(1i32);
        static OVERRIDE_DIFFICULTY: Value = Value::Bool(false);
        static OVERRIDE_LOOT_TABLE: Value = Value::Bool(false);
        static POPULATION_CHANGE_PERSISTENT_SPAWN_TABLE: Value = Value::VectorInt64(
            vec![],
        );
        static POST_SPAWN_STATE_DURATION: Value = Value::Float(0f32);
        static RUN_ONCE: Value = Value::Bool(false);
        static SPAWN_ACCELERATION_FACTOR: Value = Value::Float(0.5f32);
        static SPAWN_ACCELERATION_LINEAR: Value = Value::Float(0.5f32);
        static SPAWN_COUNT: Value = Value::Int(0i32);
        static SPAWN_DELAY: Value = Value::FloatRange((0.0, 0.0));
        static SPAWNED_AVATAR_GUIDS: Value = Value::VectorGuid(vec![]);
        static SPAWNEE_BEHAVIOR_CONFIG: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static SPAWNEE_COMBAT_NOTIFY_RADIUS: Value = Value::Float(-1f32);
        static SPAWNEE_DEFB: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SPAWNEE_LEASH_RADIUS: Value = Value::Float(-1f32);
        static SPAWNER_AVATAR_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static SPAWNER_AVATAR_ID: Value = Value::AvatarId(AvatarId::from_u64(0));
        static SPAWN_FREQENCY: Value = Value::Int(-1i32);
        static SPAWN_MY_LAND_LOOT_ITEMS: Value = Value::Bool(false);
        static SPAWN_NODES: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SPAWN_NPC_1: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_10: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_2: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_3: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_4: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_5: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_6: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_7: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_8: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_NPC_9: Lazy<Value> = Lazy::new(|| Value::ContentRefAndInt(
            String::default(),
        ));
        static SPAWN_PERSISTENT_AVATARS: Value = Value::Bool(false);
        static SPAWN_POINT_ALLOW_OVER_SPAWN: Value = Value::Bool(true);
        static SPAWN_POINT_DISTANCE: Value = Value::Float(200f32);
        static SPAWN_POINT_MAX_SPAWNEE: Value = Value::Int(1i32);
        static SPAWN_POINT_POSITION_NOISE: Value = Value::Float(0.01f32);
        static SPAWN_PRECISE: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static SPAWN_RADIUS: Value = Value::Float(150f32);
        static SPAWN_RADIUS_MIN: Value = Value::Float(0f32);
        static SPAWN_SLEEPING_AVATARS: Value = Value::Bool(false);
        static SPAWN_TIME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SPAWN_TIME_SECONDS_PER_LEVEL: Value = Value::Float(0.5f32);
        static USE_SPAWN_TIME_ACCELERATION: Value = Value::Bool(false);
        static WAVE_COUNT: Value = Value::Int(-1i32);
        static WAVE_DELAY: Value = Value::FloatRange((0.0, 0.0));
        static WAVE_DELAY_START_MIN_COUNT: Value = Value::Int(0i32);
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
        static GENERATE_INTEREST_LIST: Value = Value::Bool(false);
        static HIDDEN_FROM_CLIENTS: Value = Value::Bool(false);
        static HIDDEN_FROM_PLAYERS: Value = Value::Bool(true);
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
        static LVL: Value = Value::Int(-1i32);
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
        static EXACT_COUNT: Value = Value::Bool(false);
        static INSTANCE_GROUP: Lazy<Value> = Lazy::new(|| Value::InstanceGroup(
            String::default(),
        ));
        static IS_UN_ATTACKABLE: Value = Value::Bool(true);
        static SAME_RESPAWN_TYPE: Value = Value::Bool(false);
        match self {
            Self::Ue3EdVisual => &UE_3_ED_VISUAL,
            Self::SpawnInWave => &SPAWN_IN_WAVE,
            Self::Alive => &ALIVE,
            Self::AttackedBy => &ATTACKED_BY,
            Self::Charge => &CHARGE,
            Self::ChildSpawner => &CHILD_SPAWNER,
            Self::ClanGuid => &CLAN_GUID,
            Self::ContentClassName => &CONTENT_CLASS_NAME,
            Self::CulledSpawnsPerNearbyPlayer => &CULLED_SPAWNS_PER_NEARBY_PLAYER,
            Self::Defb => &DEFB,
            Self::DirectorTags => &DIRECTOR_TAGS,
            Self::DirectorTagsAdded => &DIRECTOR_TAGS_ADDED,
            Self::DoModelPopulationChange => &DO_MODEL_POPULATION_CHANGE,
            Self::EncounterFormation => &ENCOUNTER_FORMATION,
            Self::EncounterId => &ENCOUNTER_ID,
            Self::EncounterRole => &ENCOUNTER_ROLE,
            Self::Faction => &FACTION,
            Self::ForceSpawnOnRestart => &FORCE_SPAWN_ON_RESTART,
            Self::GeneralDifficulty => &GENERAL_DIFFICULTY,
            Self::GenerateEncounter => &GENERATE_ENCOUNTER,
            Self::GroupDifficulty => &GROUP_DIFFICULTY,
            Self::HpCur => &HP_CUR,
            Self::HpMax => &HP_MAX,
            Self::IsShardObject => &IS_SHARD_OBJECT,
            Self::LevelOffset => &LEVEL_OFFSET,
            Self::LootTable => &LOOT_TABLE,
            Self::MaxCount => &MAX_COUNT,
            Self::MinCount => &MIN_COUNT,
            Self::NumPerSpawn => &NUM_PER_SPAWN,
            Self::OverrideDifficulty => &OVERRIDE_DIFFICULTY,
            Self::OverrideLootTable => &OVERRIDE_LOOT_TABLE,
            Self::PopulationChangePersistentSpawnTable => {
                &POPULATION_CHANGE_PERSISTENT_SPAWN_TABLE
            }
            Self::PostSpawnStateDuration => &POST_SPAWN_STATE_DURATION,
            Self::RunOnce => &RUN_ONCE,
            Self::SpawnAccelerationFactor => &SPAWN_ACCELERATION_FACTOR,
            Self::SpawnAccelerationLinear => &SPAWN_ACCELERATION_LINEAR,
            Self::SpawnCount => &SPAWN_COUNT,
            Self::SpawnDelay => &SPAWN_DELAY,
            Self::SpawnedAvatarGuids => &SPAWNED_AVATAR_GUIDS,
            Self::SpawneeBehaviorConfig => &SPAWNEE_BEHAVIOR_CONFIG,
            Self::SpawneeCombatNotifyRadius => &SPAWNEE_COMBAT_NOTIFY_RADIUS,
            Self::SpawneeDefb => &SPAWNEE_DEFB,
            Self::SpawneeLeashRadius => &SPAWNEE_LEASH_RADIUS,
            Self::SpawnerAvatarGuid => &SPAWNER_AVATAR_GUID,
            Self::SpawnerAvatarId => &SPAWNER_AVATAR_ID,
            Self::SpawnFreqency => &SPAWN_FREQENCY,
            Self::SpawnMyLandLootItems => &SPAWN_MY_LAND_LOOT_ITEMS,
            Self::SpawnNodes => &SPAWN_NODES,
            Self::SpawnNpc1 => &SPAWN_NPC_1,
            Self::SpawnNpc10 => &SPAWN_NPC_10,
            Self::SpawnNpc2 => &SPAWN_NPC_2,
            Self::SpawnNpc3 => &SPAWN_NPC_3,
            Self::SpawnNpc4 => &SPAWN_NPC_4,
            Self::SpawnNpc5 => &SPAWN_NPC_5,
            Self::SpawnNpc6 => &SPAWN_NPC_6,
            Self::SpawnNpc7 => &SPAWN_NPC_7,
            Self::SpawnNpc8 => &SPAWN_NPC_8,
            Self::SpawnNpc9 => &SPAWN_NPC_9,
            Self::SpawnPersistentAvatars => &SPAWN_PERSISTENT_AVATARS,
            Self::SpawnPointAllowOverSpawn => &SPAWN_POINT_ALLOW_OVER_SPAWN,
            Self::SpawnPointDistance => &SPAWN_POINT_DISTANCE,
            Self::SpawnPointMaxSpawnee => &SPAWN_POINT_MAX_SPAWNEE,
            Self::SpawnPointPositionNoise => &SPAWN_POINT_POSITION_NOISE,
            Self::SpawnPrecise => &SPAWN_PRECISE,
            Self::SpawnRadius => &SPAWN_RADIUS,
            Self::SpawnRadiusMin => &SPAWN_RADIUS_MIN,
            Self::SpawnSleepingAvatars => &SPAWN_SLEEPING_AVATARS,
            Self::SpawnTime => &SPAWN_TIME,
            Self::SpawnTimeSecondsPerLevel => &SPAWN_TIME_SECONDS_PER_LEVEL,
            Self::UseSpawnTimeAcceleration => &USE_SPAWN_TIME_ACCELERATION,
            Self::WaveCount => &WAVE_COUNT,
            Self::WaveDelay => &WAVE_DELAY,
            Self::WaveDelayStartMinCount => &WAVE_DELAY_START_MIN_COUNT,
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
            Self::VisibleOnQuestAvailable => &VISIBLE_ON_QUEST_AVAILABLE,
            Self::VisibleOnQuestComplete => &VISIBLE_ON_QUEST_COMPLETE,
            Self::VisibleOnQuestFinished => &VISIBLE_ON_QUEST_FINISHED,
            Self::VisibleOnQuestInProgress => &VISIBLE_ON_QUEST_IN_PROGRESS,
            Self::WorldZoneObjectIndex => &WORLD_ZONE_OBJECT_INDEX,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
            Self::ExactCount => &EXACT_COUNT,
            Self::InstanceGroup => &INSTANCE_GROUP,
            Self::IsUnAttackable => &IS_UN_ATTACKABLE,
            Self::SameRespawnType => &SAME_RESPAWN_TYPE,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Ue3EdVisual => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
            Self::SpawnInWave => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Alive => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttackedBy => &[ParamFlag::NodeOwn],
            Self::Charge => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ChildSpawner => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::Deprecated,
                ]
            }
            Self::ClanGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ContentClassName => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CulledSpawnsPerNearbyPlayer => {
                &[ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::Defb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DirectorTags => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DirectorTagsAdded => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DoModelPopulationChange => {
                &[
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::EncounterFormation => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EncounterId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EncounterRole => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Faction => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ForceSpawnOnRestart => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::GeneralDifficulty => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::GenerateEncounter => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::GroupDifficulty => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HpCur => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::HpMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsShardObject => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LevelOffset => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LootTable => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MaxCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MinCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NumPerSpawn => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::OverrideDifficulty => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::OverrideLootTable => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PopulationChangePersistentSpawnTable => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PostSpawnStateDuration => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RunOnce => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SpawnAccelerationFactor => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnAccelerationLinear => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnCount => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Deprecated]
            }
            Self::SpawnDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnedAvatarGuids => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Deprecated]
            }
            Self::SpawneeBehaviorConfig => {
                &[
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawneeCombatNotifyRadius => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawneeDefb => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawneeLeashRadius => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnerAvatarGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SpawnerAvatarId => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::SpawnFreqency => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnMyLandLootItems => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SpawnNodes => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnNpc1 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc10 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc2 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc3 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc4 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc5 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc6 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc7 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc8 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnNpc9 => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::PerInstanceSetting]
            }
            Self::SpawnPersistentAvatars => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::SpawnPointAllowOverSpawn => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnPointDistance => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnPointMaxSpawnee => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnPointPositionNoise => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnPrecise => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnRadiusMin => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnSleepingAvatars => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SpawnTime => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SpawnTimeSecondsPerLevel => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::UseSpawnTimeAcceleration => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WaveCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WaveDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WaveDelayStartMinCount => {
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
            Self::ExactCount => {
                &[
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
            Self::SameRespawnType => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
        }
    }
}
impl FromStr for Spawner {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SPAWNER_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Spawner {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            955u16 => Ok(Self::Action0),
            956u16 => Ok(Self::Action0Duration),
            944u16 => Ok(Self::Action0Option),
            3522u16 => Ok(Self::AlwaysVisibleToPlayers),
            10522u16 => Ok(Self::AutoReviveDelay),
            10462u16 => Ok(Self::AutoReviveTime),
            8241u16 => Ok(Self::AwareRange),
            10933u16 => Ok(Self::BeaconRadius),
            957u16 => Ok(Self::CollisionExtent),
            953u16 => Ok(Self::ContentClass),
            11067u16 => Ok(Self::CycleQuestBase),
            7253u16 => Ok(Self::DefaultWeapon),
            9632u16 => Ok(Self::DespawnDelay),
            8828u16 => Ok(Self::Dialogs),
            6639u16 => Ok(Self::DisplayName),
            6865u16 => Ok(Self::EnableInGame),
            11191u16 => Ok(Self::FreedomProperties),
            970u16 => Ok(Self::Freq),
            958u16 => Ok(Self::GenerateInterestList),
            959u16 => Ok(Self::HiddenFromClients),
            942u16 => Ok(Self::HiddenFromPlayers),
            9042u16 => Ok(Self::HideAfterInteraction),
            4382u16 => Ok(Self::Icon),
            946u16 => Ok(Self::InstanceTags),
            5599u16 => Ok(Self::InstanceZoneKey),
            11137u16 => Ok(Self::InteractionDuration),
            7514u16 => Ok(Self::InteractionRadius),
            9044u16 => Ok(Self::InteractionResetTimer),
            916u16 => Ok(Self::IsNonSpawnedAvatar),
            7198u16 => Ok(Self::IsSelfRevivable),
            9043u16 => Ok(Self::LastInteractionTime),
            7776u16 => Ok(Self::LuaScript),
            6222u16 => Ok(Self::Lvl),
            4763u16 => Ok(Self::MaterialOverride),
            943u16 => Ok(Self::Nodelink),
            918u16 => Ok(Self::OriginalNodeName),
            919u16 => Ok(Self::OriginalZoneName),
            960u16 => Ok(Self::PartyGuid),
            939u16 => Ok(Self::PathfindSafeSpawn),
            961u16 => Ok(Self::Pos),
            969u16 => Ok(Self::Power),
            962u16 => Ok(Self::Priority),
            9930u16 => Ok(Self::QuestFlags),
            3707u16 => Ok(Self::ReadableName),
            915u16 => Ok(Self::RespawnDelay),
            10780u16 => Ok(Self::RespawnRegionName),
            10839u16 => Ok(Self::RespawnRegionNameOverride),
            963u16 => Ok(Self::Rot),
            964u16 => Ok(Self::SelfRadius),
            6140u16 => Ok(Self::SpawnMethod),
            7831u16 => Ok(Self::SpawnPosition),
            8184u16 => Ok(Self::SpawnRotation),
            965u16 => Ok(Self::Tags),
            966u16 => Ok(Self::TeamId),
            954u16 => Ok(Self::Ue3ClassId),
            9806u16 => Ok(Self::Ue3EdVisual),
            8578u16 => Ok(Self::VisibleOnQuestAvailable),
            8575u16 => Ok(Self::VisibleOnQuestComplete),
            8576u16 => Ok(Self::VisibleOnQuestFinished),
            8577u16 => Ok(Self::VisibleOnQuestInProgress),
            917u16 => Ok(Self::WorldZoneObjectIndex),
            968u16 => Ok(Self::Zone),
            935u16 => Ok(Self::ZoneGuid),
            5687u16 => Ok(Self::ExactCount),
            11388u16 => Ok(Self::InstanceGroup),
            10384u16 => Ok(Self::IsUnAttackable),
            5686u16 => Ok(Self::SameRespawnType),
            5619u16 => Ok(Self::SpawnInWave),
            5898u16 => Ok(Self::Alive),
            11315u16 => Ok(Self::AttackedBy),
            4855u16 => Ok(Self::Charge),
            920u16 => Ok(Self::ChildSpawner),
            11207u16 => Ok(Self::ClanGuid),
            945u16 => Ok(Self::ContentClassName),
            8933u16 => Ok(Self::CulledSpawnsPerNearbyPlayer),
            947u16 => Ok(Self::Defb),
            8063u16 => Ok(Self::DirectorTags),
            8178u16 => Ok(Self::DirectorTagsAdded),
            3667u16 => Ok(Self::DoModelPopulationChange),
            4843u16 => Ok(Self::EncounterFormation),
            4844u16 => Ok(Self::EncounterId),
            4842u16 => Ok(Self::EncounterRole),
            11226u16 => Ok(Self::Faction),
            4925u16 => Ok(Self::ForceSpawnOnRestart),
            11224u16 => Ok(Self::GeneralDifficulty),
            5902u16 => Ok(Self::GenerateEncounter),
            11225u16 => Ok(Self::GroupDifficulty),
            5897u16 => Ok(Self::HpCur),
            5896u16 => Ok(Self::HpMax),
            3662u16 => Ok(Self::IsShardObject),
            11217u16 => Ok(Self::LevelOffset),
            8033u16 => Ok(Self::LootTable),
            948u16 => Ok(Self::MaxCount),
            4926u16 => Ok(Self::MinCount),
            941u16 => Ok(Self::NumPerSpawn),
            11223u16 => Ok(Self::OverrideDifficulty),
            11228u16 => Ok(Self::OverrideLootTable),
            3663u16 => Ok(Self::PopulationChangePersistentSpawnTable),
            10113u16 => Ok(Self::PostSpawnStateDuration),
            5939u16 => Ok(Self::RunOnce),
            10120u16 => Ok(Self::SpawnAccelerationFactor),
            10119u16 => Ok(Self::SpawnAccelerationLinear),
            5909u16 => Ok(Self::SpawnCount),
            949u16 => Ok(Self::SpawnDelay),
            5911u16 => Ok(Self::SpawnedAvatarGuids),
            938u16 => Ok(Self::SpawneeBehaviorConfig),
            11234u16 => Ok(Self::SpawneeCombatNotifyRadius),
            11227u16 => Ok(Self::SpawneeDefb),
            11235u16 => Ok(Self::SpawneeLeashRadius),
            5907u16 => Ok(Self::SpawnerAvatarGuid),
            5905u16 => Ok(Self::SpawnerAvatarId),
            922u16 => Ok(Self::SpawnFreqency),
            9217u16 => Ok(Self::SpawnMyLandLootItems),
            921u16 => Ok(Self::SpawnNodes),
            924u16 => Ok(Self::SpawnNpc1),
            933u16 => Ok(Self::SpawnNpc10),
            925u16 => Ok(Self::SpawnNpc2),
            926u16 => Ok(Self::SpawnNpc3),
            927u16 => Ok(Self::SpawnNpc4),
            928u16 => Ok(Self::SpawnNpc5),
            929u16 => Ok(Self::SpawnNpc6),
            930u16 => Ok(Self::SpawnNpc7),
            931u16 => Ok(Self::SpawnNpc8),
            932u16 => Ok(Self::SpawnNpc9),
            5915u16 => Ok(Self::SpawnPersistentAvatars),
            11292u16 => Ok(Self::SpawnPointAllowOverSpawn),
            11238u16 => Ok(Self::SpawnPointDistance),
            11236u16 => Ok(Self::SpawnPointMaxSpawnee),
            11237u16 => Ok(Self::SpawnPointPositionNoise),
            951u16 => Ok(Self::SpawnPrecise),
            952u16 => Ok(Self::SpawnRadius),
            6160u16 => Ok(Self::SpawnRadiusMin),
            6090u16 => Ok(Self::SpawnSleepingAvatars),
            923u16 => Ok(Self::SpawnTime),
            10911u16 => Ok(Self::SpawnTimeSecondsPerLevel),
            10109u16 => Ok(Self::UseSpawnTimeAcceleration),
            11249u16 => Ok(Self::WaveCount),
            11251u16 => Ok(Self::WaveDelay),
            11250u16 => Ok(Self::WaveDelayStartMinCount),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
