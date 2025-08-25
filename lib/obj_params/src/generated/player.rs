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
pub enum Player {
    Abilities,
    AccountBankId,
    AccountBankSize,
    AccountId,
    AccountName,
    Action0,
    Action0Duration,
    AddressSlots,
    Alive,
    AttackedBy,
    AttributeAttackPowerPhys,
    AttributeAttackPowerSpell,
    AttributeConstitution,
    AttributeCrafting,
    AttributeCriticalPhys,
    AttributeCriticalSpell,
    AttributeDegenerateLevel,
    AttributeDexterity,
    AttributeDisguise,
    AttributeEnergy,
    AttributeEnergyCurrent,
    AttributeEnergyDecayStealthedPercentageNormalized,
    AttributeEnergyEquilibriumPercentageNormalized,
    AttributeEnergyGainAutoAttackHitAbsolute,
    AttributeEnergyGainWithTargetPerSecondAbsolute,
    AttributeEnergyInitialPercentageNormalized,
    AttributeEnergyMax,
    AttributeEnergyRegen,
    AttributeFocus,
    AttributeHastePhys,
    AttributeHasteSpell,
    AttributeHealth,
    AttributeHealthRegen,
    AttributeHitRatingPhys,
    AttributeHitRatingSpell,
    AttributeInCombatToEquilibriumPerSecondAbsolute,
    AttributeInCombatToEquilibriumPerSecondPercentageNormalized,
    AttributeIntuition,
    AttributeItemLevel,
    AttributeJump,
    AttributeMissRatingPhys,
    AttributeMissRatingSpell,
    AttributeMovement,
    AttributeOutOfCombatToEquilibriumPerSecondAbsolute,
    AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized,
    AttributeResilience,
    AttributeRun,
    AttributeStealthLevel,
    AttributeStrength,
    AttributeWisdom,
    AutoLootRadius,
    AvailableEdnaClones,
    AvailableOutfits,
    AwareDist,
    AwareRange,
    BgLastWordZoneGuid,
    BgLastZoneGuid,
    BgLastZonePosition,
    BgStatisticsString,
    BgTeam,
    Bling,
    BlockedAbilityEffectTypes,
    CarrierGuid,
    ChatIgnoreSet,
    ClanGuid,
    ClanHateList,
    ClanName,
    ClanPrivileges,
    ClanRank,
    ClanRatified,
    ClassData,
    ClassSkillCollection,
    ClientActionTracker,
    ClientReady,
    Cloaked,
    ClusterGuid,
    CollisionExtent,
    CombatStyle,
    ContentClass,
    CooldownManager,
    CooldownPassed,
    CooldownTrackers,
    CurrentAbilityBarReferences,
    CurrentClassSkills,
    CurrentMyLandAddress,
    CurrentOutfitSlot,
    CurrentSkin,
    CurrentTickedItemSlot,
    CustomizationBrowAngle,
    CustomizationBustSize,
    CustomizationCheek,
    CustomizationCheekBone,
    CustomizationChinPortude,
    CustomizationEarElf,
    CustomizationEarSize,
    CustomizationEyeBrowPos,
    CustomizationEyePos,
    CustomizationEyePosSpacing,
    CustomizationEyeSizeLength,
    CustomizationEyeSizeWidth,
    CustomizationEyesPretty,
    CustomizationFat,
    CustomizationGender,
    CustomizationHeight,
    CustomizationJawChubby,
    CustomizationMouthExpression,
    CustomizationMouthLowerLipThic,
    CustomizationMouthPos,
    CustomizationMouthUpperLipThic,
    CustomizationMouthWidth,
    CustomizationMuscular,
    CustomizationNosePortude,
    CustomizationNosePosLength,
    CustomizationNosePosWidth,
    CustomizationSkinny,
    DamageOutputMod,
    DamageReceivedMod,
    DeathInfo,
    DefaultItemsContentGuid,
    DepositAmount,
    DepositBankGuid,
    DepositHistory,
    DepositLevel,
    DungoneKillBoss,
    EmoteSlots,
    EmoteUsed,
    EnemyId,
    Faction,
    FactionStandings,
    FirstTimeSpawn,
    FragmentSlots,
    FreedomProperties,
    Freq,
    GameCash,
    GenerateInterestList,
    GuideToAvatar,
    GuideToLocation,
    HasAttributes,
    HeavySpecialSkillData,
    HostIp,
    HpCur,
    HpMax,
    HpMin,
    Icon,
    InGameSession,
    InInstancedBattle,
    InitialWorldTimeThisLevelThisSession,
    InitialWorldTimeThisSession,
    InMiniGame,
    InstanceZoneKey,
    InteractionRadius,
    InteractRadius,
    InventorySize,
    IsAdmin,
    IsInCombat,
    IsInPvPZone,
    IsInsideInstanceZone,
    IsInSocial,
    IsOnline,
    IsUnAttackable,
    ItemSlotsVisible,
    JumpVelocity,
    LastAttackPosition,
    LastEquippedWeapon,
    LastKnownClanLandRadius,
    LastLogoutTime,
    LastPortalUsed,
    LastResetDailyQuest,
    LastSkuSyncTime,
    LastVendorSyncTime,
    LoginCount,
    LootItemGuid,
    LootItemType,
    Lvl,
    LvlHistory,
    MaxLevelCap,
    MetamorphItemList,
    MinigameData,
    Mount,
    Mover,
    MoveSpeed,
    MyLandData,
    MyQuestTrack,
    MyShopGuid,
    MySteamDlc,
    MyUsedSteamDlc,
    NewItems,
    OutfitNames,
    OutfitSlots,
    OverrideFaction,
    PartyGuid,
    Pet,
    PhaseSelectionData,
    PlaycountMinigameBilliards,
    PlayerLoading,
    PlayerNodeState,
    PlayerUsedSteamDlc,
    PortalData,
    Pos,
    Power,
    PvpEnabled,
    PvpEnabledInMyLandServerSetting,
    PvpEnabledServerSetting,
    PvpEnableDuration,
    PvpFlag,
    PvpRank,
    PvpTimer,
    PvpXp,
    QuickUseBar,
    Race,
    RankingEdnamobsTotal,
    RankingGearTotal,
    RankingKillsPve,
    RankingKillsPvp,
    RankingMypadRooms,
    RankingSomaAdd,
    RankingSomaTotal,
    RankingTotal,
    RecentlyKilledInPvP,
    ReferenceList,
    RelativePosToCarrier,
    RequestTeleportPos,
    ResetDailyQuestList,
    Rot,
    ScoreMinigameBilliards,
    SelfRadius,
    SheathedModeActive,
    SignClanCharterItem,
    Size,
    SomaCarried,
    SomaLootRate,
    SpawnCinematicOverride,
    SpawnedOnAvatar,
    SpawnMode,
    SpectateName,
    SpectatePartyGuid,
    StatAnyDmgReduction,
    StatAoEMaxSubTargets,
    StatAoESubTargetsDamageMod,
    StatArmorRating,
    StatArmorReduction,
    StatAttackPower,
    StatAttackPowerBonus,
    StatAttackPowerRating,
    StatAttackRangePhysAdd,
    StatAttackRating,
    StatBendChance,
    StatBendRating,
    StatBlockChance,
    StatBlockedDamageMod,
    StatBlockRating,
    StatCritChance,
    StatCritDmgRating,
    StatCriticalChanceReduction,
    StatCriticalDamageMod,
    StatCriticalDamageModBonus,
    StatCritRating,
    StatDamagePercPerMeterMod,
    StatDefencePowerPhys,
    StatDefenceRatingPhys,
    StatDodgeChance,
    StatDodgeRating,
    StatEnergyCurrentH1,
    StatEnergyCurrentH2,
    StatEnergyCurrentH3,
    StatEnergyCurrentS1,
    StatEnergyCurrentS2,
    StatEnergyCurrentS3,
    StatEvadeChance,
    StatEvadeRating,
    StatExtraHealthRegen,
    StatFinalDamageMod,
    StatFinalHealingMod,
    StatFreeFallDistanceMod,
    StatHasteClassSkills,
    StatHastePhysNormal,
    StatHealingReceivedMod,
    StatHeavyBonus,
    StatHeavyEnergyPerHit,
    StatHeavyRating,
    StatHitChance,
    StatHitRating,
    StatInitialThreatMod,
    StatParryChance,
    StatParryRating,
    StatPeneBonus,
    StatPeneRating,
    StatReflectChance,
    StatReflectRating,
    StatSpecialBonus,
    StatSpecialEnergyPerHit,
    StatSpecialRating,
    StatStamina,
    StatTcMax,
    StatThreatMod,
    StatWeaponDps,
    StatWepMaxDmg,
    StatWepMinDmg,
    StatXpMod,
    StickyTargets,
    Tags,
    Target,
    TeamId,
    TimePlayedBeforeThisSession,
    TimePlayedThisLevelBeforeThisSession,
    TutorialMode,
    Ue3ClassId,
    UiHintsAvailable,
    UnassignPortals,
    UnLockedInstances,
    UnLockedPortals,
    UnlockedUiWindows,
    VisibleItemInfo,
    Weapon,
    WorldMapGuid,
    Xp,
    XpForNextLevel,
    XpTotal,
    Zone,
    ZoneGuid,
}
pub(crate) static PLAYER_ATTRIBUTES: phf::Map<&'static str, Player> = phf_map! {
    "abilities" => Player::Abilities, "accountBankID" => Player::AccountBankId,
    "accountBankSize" => Player::AccountBankSize, "accountId" => Player::AccountId,
    "accountName" => Player::AccountName, "action0" => Player::Action0, "action0Duration"
    => Player::Action0Duration, "AddressSlots" => Player::AddressSlots, "alive" =>
    Player::Alive, "attackedBy" => Player::AttackedBy, "attributeAttackPowerPhys" =>
    Player::AttributeAttackPowerPhys, "attributeAttackPowerSpell" =>
    Player::AttributeAttackPowerSpell, "attributeConstitution" =>
    Player::AttributeConstitution, "attributeCrafting" => Player::AttributeCrafting,
    "attributeCriticalPhys" => Player::AttributeCriticalPhys, "attributeCriticalSpell" =>
    Player::AttributeCriticalSpell, "attributeDegenerate_Level" =>
    Player::AttributeDegenerateLevel, "attributeDexterity" => Player::AttributeDexterity,
    "attributeDisguise" => Player::AttributeDisguise, "attributeEnergy" =>
    Player::AttributeEnergy, "attributeEnergyCurrent" => Player::AttributeEnergyCurrent,
    "attributeEnergyDecay_Stealthed_PercentageNormalized" =>
    Player::AttributeEnergyDecayStealthedPercentageNormalized,
    "attributeEnergyEquilibrium_PercentageNormalized" =>
    Player::AttributeEnergyEquilibriumPercentageNormalized,
    "attributeEnergyGain_AutoAttack_Hit_Absolute" =>
    Player::AttributeEnergyGainAutoAttackHitAbsolute,
    "attributeEnergyGain_WithTarget_PerSecond_Absolute" =>
    Player::AttributeEnergyGainWithTargetPerSecondAbsolute,
    "attributeEnergyInitial_PercentageNormalized" =>
    Player::AttributeEnergyInitialPercentageNormalized, "attributeEnergyMax" =>
    Player::AttributeEnergyMax, "attributeEnergyRegen" => Player::AttributeEnergyRegen,
    "attributeFocus" => Player::AttributeFocus, "attributeHastePhys" =>
    Player::AttributeHastePhys, "attributeHasteSpell" => Player::AttributeHasteSpell,
    "attributeHealth" => Player::AttributeHealth, "attributeHealthRegen" =>
    Player::AttributeHealthRegen, "attributeHitRatingPhys" =>
    Player::AttributeHitRatingPhys, "attributeHitRatingSpell" =>
    Player::AttributeHitRatingSpell, "attributeInCombat_ToEquilibrium_PerSecond_Absolute"
    => Player::AttributeInCombatToEquilibriumPerSecondAbsolute,
    "attributeInCombat_ToEquilibrium_PerSecond_PercentageNormalized" =>
    Player::AttributeInCombatToEquilibriumPerSecondPercentageNormalized,
    "attributeIntuition" => Player::AttributeIntuition, "attributeItem_Level" =>
    Player::AttributeItemLevel, "attributeJump" => Player::AttributeJump,
    "attributeMissRatingPhys" => Player::AttributeMissRatingPhys,
    "attributeMissRatingSpell" => Player::AttributeMissRatingSpell, "attributeMovement"
    => Player::AttributeMovement, "attributeOutOfCombat_ToEquilibrium_PerSecond_Absolute"
    => Player::AttributeOutOfCombatToEquilibriumPerSecondAbsolute,
    "attributeOutOfCombat_ToEquilibrium_PerSecond_PercentageNormalized" =>
    Player::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized,
    "attributeResilience" => Player::AttributeResilience, "attributeRun" =>
    Player::AttributeRun, "attributeStealth_Level" => Player::AttributeStealthLevel,
    "attributeStrength" => Player::AttributeStrength, "attributeWisdom" =>
    Player::AttributeWisdom, "autoLootRadius" => Player::AutoLootRadius,
    "AvailableEDNAClones" => Player::AvailableEdnaClones, "availableOutfits" =>
    Player::AvailableOutfits, "awareDist" => Player::AwareDist, "AwareRange" =>
    Player::AwareRange, "BG_LastWordZoneGUID" => Player::BgLastWordZoneGuid,
    "BG_LastZoneGUID" => Player::BgLastZoneGuid, "BG_LastZonePosition" =>
    Player::BgLastZonePosition, "BG_StatisticsString" => Player::BgStatisticsString,
    "BG_Team" => Player::BgTeam, "bling" => Player::Bling, "blockedAbilityEffectTypes" =>
    Player::BlockedAbilityEffectTypes, "carrierGuid" => Player::CarrierGuid,
    "chatIgnoreSet" => Player::ChatIgnoreSet, "clanGUID" => Player::ClanGuid,
    "clanHateList" => Player::ClanHateList, "clanName" => Player::ClanName,
    "clanPrivileges" => Player::ClanPrivileges, "clanRank" => Player::ClanRank,
    "clanRatified" => Player::ClanRatified, "classData" => Player::ClassData,
    "classSkillCollection" => Player::ClassSkillCollection, "clientActionTracker" =>
    Player::ClientActionTracker, "clientReady" => Player::ClientReady, "cloaked" =>
    Player::Cloaked, "clusterGUID" => Player::ClusterGuid, "collisionExtent" =>
    Player::CollisionExtent, "combatStyle" => Player::CombatStyle, "ContentClass" =>
    Player::ContentClass, "cooldownManager" => Player::CooldownManager, "cooldownPassed"
    => Player::CooldownPassed, "cooldownTrackers" => Player::CooldownTrackers,
    "currentAbilityBarReferences" => Player::CurrentAbilityBarReferences,
    "currentClassSkills" => Player::CurrentClassSkills, "currentMyLandAddress" =>
    Player::CurrentMyLandAddress, "currentOutfitSlot" => Player::CurrentOutfitSlot,
    "currentSkin" => Player::CurrentSkin, "currentTickedItemSlot" =>
    Player::CurrentTickedItemSlot, "customizationBrowAngle" =>
    Player::CustomizationBrowAngle, "customizationBustSize" =>
    Player::CustomizationBustSize, "customizationCheek" => Player::CustomizationCheek,
    "customizationCheekBone" => Player::CustomizationCheekBone,
    "customizationChinPortude" => Player::CustomizationChinPortude, "customizationEarElf"
    => Player::CustomizationEarElf, "customizationEarSize" =>
    Player::CustomizationEarSize, "customizationEyeBrowPos" =>
    Player::CustomizationEyeBrowPos, "customizationEyePos" =>
    Player::CustomizationEyePos, "customizationEyePosSpacing" =>
    Player::CustomizationEyePosSpacing, "customizationEyeSizeLength" =>
    Player::CustomizationEyeSizeLength, "customizationEyeSizeWidth" =>
    Player::CustomizationEyeSizeWidth, "customizationEyesPretty" =>
    Player::CustomizationEyesPretty, "customizationFat" => Player::CustomizationFat,
    "customizationGender" => Player::CustomizationGender, "customizationHeight" =>
    Player::CustomizationHeight, "customizationJawChubby" =>
    Player::CustomizationJawChubby, "customizationMouthExpression" =>
    Player::CustomizationMouthExpression, "customizationMouthLowerLipThic" =>
    Player::CustomizationMouthLowerLipThic, "customizationMouthPos" =>
    Player::CustomizationMouthPos, "customizationMouthUpperLipThic" =>
    Player::CustomizationMouthUpperLipThic, "customizationMouthWidth" =>
    Player::CustomizationMouthWidth, "customizationMuscular" =>
    Player::CustomizationMuscular, "customizationNosePortude" =>
    Player::CustomizationNosePortude, "customizationNosePosLength" =>
    Player::CustomizationNosePosLength, "customizationNosePosWidth" =>
    Player::CustomizationNosePosWidth, "customizationSkinny" =>
    Player::CustomizationSkinny, "damageOutputMod" => Player::DamageOutputMod,
    "damageReceivedMod" => Player::DamageReceivedMod, "deathInfo" => Player::DeathInfo,
    "defaultItemsContentGuid" => Player::DefaultItemsContentGuid, "DepositAmount" =>
    Player::DepositAmount, "depositBankGUID" => Player::DepositBankGuid, "DepositHistory"
    => Player::DepositHistory, "DepositLevel" => Player::DepositLevel, "DungoneKillBoss"
    => Player::DungoneKillBoss, "EmoteSlots" => Player::EmoteSlots, "EmoteUsed" =>
    Player::EmoteUsed, "enemyID" => Player::EnemyId, "Faction" => Player::Faction,
    "FactionStandings" => Player::FactionStandings, "firstTimeSpawn" =>
    Player::FirstTimeSpawn, "FragmentSlots" => Player::FragmentSlots, "FreedomProperties"
    => Player::FreedomProperties, "Freq" => Player::Freq, "gameCash" => Player::GameCash,
    "generateInterestList" => Player::GenerateInterestList, "guideToAvatar" =>
    Player::GuideToAvatar, "guideToLocation" => Player::GuideToLocation, "hasAttributes"
    => Player::HasAttributes, "heavySpecialSkillData" => Player::HeavySpecialSkillData,
    "hostIP" => Player::HostIp, "hpCur" => Player::HpCur, "hpMax" => Player::HpMax,
    "hpMin" => Player::HpMin, "Icon" => Player::Icon, "inGameSession" =>
    Player::InGameSession, "inInstancedBattle" => Player::InInstancedBattle,
    "initialWorldTimeThisLevelThisSession" =>
    Player::InitialWorldTimeThisLevelThisSession, "initialWorldTimeThisSession" =>
    Player::InitialWorldTimeThisSession, "inMiniGame" => Player::InMiniGame,
    "instanceZoneKey" => Player::InstanceZoneKey, "InteractionRadius" =>
    Player::InteractionRadius, "interactRadius" => Player::InteractRadius,
    "inventorySize" => Player::InventorySize, "isAdmin" => Player::IsAdmin, "isInCombat"
    => Player::IsInCombat, "isInPvPZone" => Player::IsInPvPZone, "isInsideInstanceZone"
    => Player::IsInsideInstanceZone, "isInSocial" => Player::IsInSocial, "isOnline" =>
    Player::IsOnline, "isUnAttackable" => Player::IsUnAttackable, "itemSlotsVisible" =>
    Player::ItemSlotsVisible, "jumpVelocity" => Player::JumpVelocity,
    "lastAttackPosition" => Player::LastAttackPosition, "lastEquippedWeapon" =>
    Player::LastEquippedWeapon, "lastKnownClanLandRadius" =>
    Player::LastKnownClanLandRadius, "lastLogoutTime" => Player::LastLogoutTime,
    "LastPortalUsed" => Player::LastPortalUsed, "lastResetDailyQuest" =>
    Player::LastResetDailyQuest, "lastSKUSyncTime" => Player::LastSkuSyncTime,
    "lastVendorSyncTime" => Player::LastVendorSyncTime, "loginCount" =>
    Player::LoginCount, "lootItemGuid" => Player::LootItemGuid, "lootItemType" =>
    Player::LootItemType, "lvl" => Player::Lvl, "lvlHistory" => Player::LvlHistory,
    "maxLevelCap" => Player::MaxLevelCap, "metamorphItemList" =>
    Player::MetamorphItemList, "minigameData" => Player::MinigameData, "mount" =>
    Player::Mount, "mover" => Player::Mover, "moveSpeed" => Player::MoveSpeed,
    "MyLandData" => Player::MyLandData, "myQuestTrack" => Player::MyQuestTrack,
    "myShopGUID" => Player::MyShopGuid, "mySteamDlc" => Player::MySteamDlc,
    "myUsedSteamDlc" => Player::MyUsedSteamDlc, "newItems" => Player::NewItems,
    "outfitNames" => Player::OutfitNames, "OutfitSlots" => Player::OutfitSlots,
    "OverrideFaction" => Player::OverrideFaction, "partyGUID" => Player::PartyGuid, "pet"
    => Player::Pet, "phaseSelectionData" => Player::PhaseSelectionData,
    "playcount_minigame_billiards" => Player::PlaycountMinigameBilliards, "playerLoading"
    => Player::PlayerLoading, "playerNodeState" => Player::PlayerNodeState,
    "playerUsedSteamDlc" => Player::PlayerUsedSteamDlc, "PortalData" =>
    Player::PortalData, "pos" => Player::Pos, "Power" => Player::Power, "pvpEnabled" =>
    Player::PvpEnabled, "pvpEnabledInMyLandServerSetting" =>
    Player::PvpEnabledInMyLandServerSetting, "pvpEnabledServerSetting" =>
    Player::PvpEnabledServerSetting, "pvpEnableDuration" => Player::PvpEnableDuration,
    "pvpFlag" => Player::PvpFlag, "pvpRank" => Player::PvpRank, "pvpTimer" =>
    Player::PvpTimer, "pvpXP" => Player::PvpXp, "quickUseBar" => Player::QuickUseBar,
    "race" => Player::Race, "ranking_ednamobs_total" => Player::RankingEdnamobsTotal,
    "ranking_gear_total" => Player::RankingGearTotal, "ranking_kills_pve" =>
    Player::RankingKillsPve, "ranking_kills_pvp" => Player::RankingKillsPvp,
    "ranking_mypad_rooms" => Player::RankingMypadRooms, "ranking_soma_add" =>
    Player::RankingSomaAdd, "ranking_soma_total" => Player::RankingSomaTotal,
    "ranking_total" => Player::RankingTotal, "recentlyKilledInPvP" =>
    Player::RecentlyKilledInPvP, "ReferenceList" => Player::ReferenceList,
    "relativePosToCarrier" => Player::RelativePosToCarrier, "requestTeleportPos" =>
    Player::RequestTeleportPos, "resetDailyQuestList" => Player::ResetDailyQuestList,
    "rot" => Player::Rot, "score_minigame_billiards" => Player::ScoreMinigameBilliards,
    "selfRadius" => Player::SelfRadius, "sheathedModeActive" =>
    Player::SheathedModeActive, "SignClanCharterItem" => Player::SignClanCharterItem,
    "Size" => Player::Size, "somaCarried" => Player::SomaCarried, "somaLootRate" =>
    Player::SomaLootRate, "spawnCinematicOverride" => Player::SpawnCinematicOverride,
    "spawnedOnAvatar" => Player::SpawnedOnAvatar, "spawnMode" => Player::SpawnMode,
    "spectateName" => Player::SpectateName, "spectatePartyGUID" =>
    Player::SpectatePartyGuid, "statAnyDmgReduction" => Player::StatAnyDmgReduction,
    "statAoE_MaxSubTargets" => Player::StatAoEMaxSubTargets,
    "statAoE_SubTargetsDamageMod" => Player::StatAoESubTargetsDamageMod,
    "statArmorRating" => Player::StatArmorRating, "statArmorReduction" =>
    Player::StatArmorReduction, "statAttackPower" => Player::StatAttackPower,
    "statAttackPowerBonus" => Player::StatAttackPowerBonus, "statAttackPowerRating" =>
    Player::StatAttackPowerRating, "statAttackRangePhysAdd" =>
    Player::StatAttackRangePhysAdd, "statAttackRating" => Player::StatAttackRating,
    "statBendChance" => Player::StatBendChance, "statBendRating" =>
    Player::StatBendRating, "statBlockChance" => Player::StatBlockChance,
    "statBlockedDamageMod" => Player::StatBlockedDamageMod, "statBlockRating" =>
    Player::StatBlockRating, "statCritChance" => Player::StatCritChance,
    "statCritDmgRating" => Player::StatCritDmgRating, "statCriticalChanceReduction" =>
    Player::StatCriticalChanceReduction, "statCriticalDamageMod" =>
    Player::StatCriticalDamageMod, "statCriticalDamageModBonus" =>
    Player::StatCriticalDamageModBonus, "statCritRating" => Player::StatCritRating,
    "statDamagePercPerMeterMod" => Player::StatDamagePercPerMeterMod,
    "statDefencePowerPhys" => Player::StatDefencePowerPhys, "statDefenceRatingPhys" =>
    Player::StatDefenceRatingPhys, "statDodgeChance" => Player::StatDodgeChance,
    "statDodgeRating" => Player::StatDodgeRating, "statEnergyCurrentH1" =>
    Player::StatEnergyCurrentH1, "statEnergyCurrentH2" => Player::StatEnergyCurrentH2,
    "statEnergyCurrentH3" => Player::StatEnergyCurrentH3, "statEnergyCurrentS1" =>
    Player::StatEnergyCurrentS1, "statEnergyCurrentS2" => Player::StatEnergyCurrentS2,
    "statEnergyCurrentS3" => Player::StatEnergyCurrentS3, "statEvadeChance" =>
    Player::StatEvadeChance, "statEvadeRating" => Player::StatEvadeRating,
    "statExtraHealthRegen" => Player::StatExtraHealthRegen, "statFinalDamageMod" =>
    Player::StatFinalDamageMod, "statFinalHealingMod" => Player::StatFinalHealingMod,
    "statFreeFallDistanceMod" => Player::StatFreeFallDistanceMod, "statHasteClassSkills"
    => Player::StatHasteClassSkills, "statHastePhysNormal" =>
    Player::StatHastePhysNormal, "statHealingReceivedMod" =>
    Player::StatHealingReceivedMod, "statHeavyBonus" => Player::StatHeavyBonus,
    "statHeavyEnergyPerHit" => Player::StatHeavyEnergyPerHit, "statHeavyRating" =>
    Player::StatHeavyRating, "statHitChance" => Player::StatHitChance, "statHitRating" =>
    Player::StatHitRating, "statInitialThreatMod" => Player::StatInitialThreatMod,
    "statParryChance" => Player::StatParryChance, "statParryRating" =>
    Player::StatParryRating, "statPeneBonus" => Player::StatPeneBonus, "statPeneRating"
    => Player::StatPeneRating, "statReflectChance" => Player::StatReflectChance,
    "statReflectRating" => Player::StatReflectRating, "statSpecialBonus" =>
    Player::StatSpecialBonus, "statSpecialEnergyPerHit" =>
    Player::StatSpecialEnergyPerHit, "statSpecialRating" => Player::StatSpecialRating,
    "statStamina" => Player::StatStamina, "statTCMax" => Player::StatTcMax,
    "statThreatMod" => Player::StatThreatMod, "statWeaponDPS" => Player::StatWeaponDps,
    "statWepMaxDmg" => Player::StatWepMaxDmg, "statWepMinDmg" => Player::StatWepMinDmg,
    "statXpMod" => Player::StatXpMod, "stickyTargets" => Player::StickyTargets, "tags" =>
    Player::Tags, "target" => Player::Target, "teamID" => Player::TeamId,
    "timePlayedBeforeThisSession" => Player::TimePlayedBeforeThisSession,
    "timePlayedThisLevelBeforeThisSession" =>
    Player::TimePlayedThisLevelBeforeThisSession, "tutorialMode" => Player::TutorialMode,
    "UE3ClassID" => Player::Ue3ClassId, "uiHintsAvailable" => Player::UiHintsAvailable,
    "UnassignPortals" => Player::UnassignPortals, "unLockedInstances" =>
    Player::UnLockedInstances, "unLockedPortals" => Player::UnLockedPortals,
    "unlockedUIWindows" => Player::UnlockedUiWindows, "visibleItemInfo" =>
    Player::VisibleItemInfo, "weapon" => Player::Weapon, "worldMapGUID" =>
    Player::WorldMapGuid, "xp" => Player::Xp, "xpForNextLevel" => Player::XpForNextLevel,
    "xpTotal" => Player::XpTotal, "zone" => Player::Zone, "ZoneGuid" => Player::ZoneGuid,
};
pub(crate) static PLAYER_ATTRIBUTES_ID: phf::Map<u16, Player> = phf_map! {
    9350u16 => Player::Abilities, 12268u16 => Player::AccountBankId, 12269u16 =>
    Player::AccountBankSize, 10603u16 => Player::AccountId, 10602u16 =>
    Player::AccountName, 2673u16 => Player::Action0, 2674u16 => Player::Action0Duration,
    5904u16 => Player::AddressSlots, 2675u16 => Player::Alive, 2676u16 =>
    Player::AttackedBy, 6258u16 => Player::AttributeAttackPowerPhys, 6256u16 =>
    Player::AttributeAttackPowerSpell, 4326u16 => Player::AttributeConstitution, 4325u16
    => Player::AttributeCrafting, 6255u16 => Player::AttributeCriticalPhys, 6254u16 =>
    Player::AttributeCriticalSpell, 6539u16 => Player::AttributeDegenerateLevel, 4327u16
    => Player::AttributeDexterity, 4324u16 => Player::AttributeDisguise, 4323u16 =>
    Player::AttributeEnergy, 4547u16 => Player::AttributeEnergyCurrent, 4549u16 =>
    Player::AttributeEnergyDecayStealthedPercentageNormalized, 4556u16 =>
    Player::AttributeEnergyEquilibriumPercentageNormalized, 4550u16 =>
    Player::AttributeEnergyGainAutoAttackHitAbsolute, 4557u16 =>
    Player::AttributeEnergyGainWithTargetPerSecondAbsolute, 4555u16 =>
    Player::AttributeEnergyInitialPercentageNormalized, 4548u16 =>
    Player::AttributeEnergyMax, 6265u16 => Player::AttributeEnergyRegen, 4322u16 =>
    Player::AttributeFocus, 6253u16 => Player::AttributeHastePhys, 6252u16 =>
    Player::AttributeHasteSpell, 6267u16 => Player::AttributeHealth, 6266u16 =>
    Player::AttributeHealthRegen, 6264u16 => Player::AttributeHitRatingPhys, 6261u16 =>
    Player::AttributeHitRatingSpell, 4554u16 =>
    Player::AttributeInCombatToEquilibriumPerSecondAbsolute, 4553u16 =>
    Player::AttributeInCombatToEquilibriumPerSecondPercentageNormalized, 4321u16 =>
    Player::AttributeIntuition, 12143u16 => Player::AttributeItemLevel, 6263u16 =>
    Player::AttributeJump, 6260u16 => Player::AttributeMissRatingPhys, 6259u16 =>
    Player::AttributeMissRatingSpell, 4320u16 => Player::AttributeMovement, 4552u16 =>
    Player::AttributeOutOfCombatToEquilibriumPerSecondAbsolute, 4551u16 =>
    Player::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized, 6268u16 =>
    Player::AttributeResilience, 6262u16 => Player::AttributeRun, 4559u16 =>
    Player::AttributeStealthLevel, 4319u16 => Player::AttributeStrength, 4318u16 =>
    Player::AttributeWisdom, 11393u16 => Player::AutoLootRadius, 10049u16 =>
    Player::AvailableEdnaClones, 10040u16 => Player::AvailableOutfits, 2678u16 =>
    Player::AwareDist, 8295u16 => Player::AwareRange, 12104u16 =>
    Player::BgLastWordZoneGuid, 12103u16 => Player::BgLastZoneGuid, 12102u16 =>
    Player::BgLastZonePosition, 12100u16 => Player::BgStatisticsString, 12101u16 =>
    Player::BgTeam, 6465u16 => Player::Bling, 2627u16 =>
    Player::BlockedAbilityEffectTypes, 2619u16 => Player::CarrierGuid, 2679u16 =>
    Player::ChatIgnoreSet, 8904u16 => Player::ClanGuid, 8910u16 => Player::ClanHateList,
    8909u16 => Player::ClanName, 10928u16 => Player::ClanPrivileges, 8903u16 =>
    Player::ClanRank, 10929u16 => Player::ClanRatified, 11080u16 => Player::ClassData,
    10518u16 => Player::ClassSkillCollection, 4942u16 => Player::ClientActionTracker,
    2647u16 => Player::ClientReady, 2668u16 => Player::Cloaked, 2680u16 =>
    Player::ClusterGuid, 2681u16 => Player::CollisionExtent, 2616u16 =>
    Player::CombatStyle, 2671u16 => Player::ContentClass, 11327u16 =>
    Player::CooldownManager, 12291u16 => Player::CooldownPassed, 4039u16 =>
    Player::CooldownTrackers, 10604u16 => Player::CurrentAbilityBarReferences, 10605u16
    => Player::CurrentClassSkills, 6655u16 => Player::CurrentMyLandAddress, 7265u16 =>
    Player::CurrentOutfitSlot, 2669u16 => Player::CurrentSkin, 10097u16 =>
    Player::CurrentTickedItemSlot, 12322u16 => Player::CustomizationBrowAngle, 8562u16 =>
    Player::CustomizationBustSize, 12304u16 => Player::CustomizationCheek, 12305u16 =>
    Player::CustomizationCheekBone, 12303u16 => Player::CustomizationChinPortude,
    12306u16 => Player::CustomizationEarElf, 12307u16 => Player::CustomizationEarSize,
    12321u16 => Player::CustomizationEyeBrowPos, 12319u16 => Player::CustomizationEyePos,
    12320u16 => Player::CustomizationEyePosSpacing, 12318u16 =>
    Player::CustomizationEyeSizeLength, 12317u16 => Player::CustomizationEyeSizeWidth,
    12316u16 => Player::CustomizationEyesPretty, 4295u16 => Player::CustomizationFat,
    4291u16 => Player::CustomizationGender, 4292u16 => Player::CustomizationHeight,
    12302u16 => Player::CustomizationJawChubby, 12311u16 =>
    Player::CustomizationMouthExpression, 12313u16 =>
    Player::CustomizationMouthLowerLipThic, 12315u16 => Player::CustomizationMouthPos,
    12312u16 => Player::CustomizationMouthUpperLipThic, 12314u16 =>
    Player::CustomizationMouthWidth, 4293u16 => Player::CustomizationMuscular, 12308u16
    => Player::CustomizationNosePortude, 12310u16 => Player::CustomizationNosePosLength,
    12309u16 => Player::CustomizationNosePosWidth, 4294u16 =>
    Player::CustomizationSkinny, 6008u16 => Player::DamageOutputMod, 6007u16 =>
    Player::DamageReceivedMod, 11229u16 => Player::DeathInfo, 6786u16 =>
    Player::DefaultItemsContentGuid, 9221u16 => Player::DepositAmount, 11582u16 =>
    Player::DepositBankGuid, 9219u16 => Player::DepositHistory, 9220u16 =>
    Player::DepositLevel, 12144u16 => Player::DungoneKillBoss, 7041u16 =>
    Player::EmoteSlots, 8040u16 => Player::EmoteUsed, 12293u16 => Player::EnemyId,
    2638u16 => Player::Faction, 2639u16 => Player::FactionStandings, 2650u16 =>
    Player::FirstTimeSpawn, 5998u16 => Player::FragmentSlots, 11204u16 =>
    Player::FreedomProperties, 2715u16 => Player::Freq, 6592u16 => Player::GameCash,
    2663u16 => Player::GenerateInterestList, 2636u16 => Player::GuideToAvatar, 2635u16 =>
    Player::GuideToLocation, 6375u16 => Player::HasAttributes, 10996u16 =>
    Player::HeavySpecialSkillData, 2687u16 => Player::HostIp, 2688u16 => Player::HpCur,
    2689u16 => Player::HpMax, 2642u16 => Player::HpMin, 4391u16 => Player::Icon, 2691u16
    => Player::InGameSession, 5151u16 => Player::InInstancedBattle, 10149u16 =>
    Player::InitialWorldTimeThisLevelThisSession, 10150u16 =>
    Player::InitialWorldTimeThisSession, 2631u16 => Player::InMiniGame, 5610u16 =>
    Player::InstanceZoneKey, 7527u16 => Player::InteractionRadius, 4191u16 =>
    Player::InteractRadius, 9881u16 => Player::InventorySize, 12141u16 =>
    Player::IsAdmin, 2607u16 => Player::IsInCombat, 9698u16 => Player::IsInPvPZone,
    5508u16 => Player::IsInsideInstanceZone, 12527u16 => Player::IsInSocial, 12324u16 =>
    Player::IsOnline, 7884u16 => Player::IsUnAttackable, 12244u16 =>
    Player::ItemSlotsVisible, 2692u16 => Player::JumpVelocity, 2632u16 =>
    Player::LastAttackPosition, 7887u16 => Player::LastEquippedWeapon, 11001u16 =>
    Player::LastKnownClanLandRadius, 11026u16 => Player::LastLogoutTime, 10386u16 =>
    Player::LastPortalUsed, 12140u16 => Player::LastResetDailyQuest, 10988u16 =>
    Player::LastSkuSyncTime, 11230u16 => Player::LastVendorSyncTime, 10144u16 =>
    Player::LoginCount, 6463u16 => Player::LootItemGuid, 7947u16 => Player::LootItemType,
    2693u16 => Player::Lvl, 10177u16 => Player::LvlHistory, 12115u16 =>
    Player::MaxLevelCap, 11095u16 => Player::MetamorphItemList, 2626u16 =>
    Player::MinigameData, 3771u16 => Player::Mount, 2649u16 => Player::Mover, 2694u16 =>
    Player::MoveSpeed, 5622u16 => Player::MyLandData, 12415u16 => Player::MyQuestTrack,
    11583u16 => Player::MyShopGuid, 12117u16 => Player::MySteamDlc, 12116u16 =>
    Player::MyUsedSteamDlc, 11231u16 => Player::NewItems, 10048u16 =>
    Player::OutfitNames, 7264u16 => Player::OutfitSlots, 2640u16 =>
    Player::OverrideFaction, 2697u16 => Player::PartyGuid, 12181u16 => Player::Pet,
    5212u16 => Player::PhaseSelectionData, 10145u16 =>
    Player::PlaycountMinigameBilliards, 4943u16 => Player::PlayerLoading, 11252u16 =>
    Player::PlayerNodeState, 12288u16 => Player::PlayerUsedSteamDlc, 10033u16 =>
    Player::PortalData, 2699u16 => Player::Pos, 2714u16 => Player::Power, 9358u16 =>
    Player::PvpEnabled, 9985u16 => Player::PvpEnabledInMyLandServerSetting, 9986u16 =>
    Player::PvpEnabledServerSetting, 12026u16 => Player::PvpEnableDuration, 12028u16 =>
    Player::PvpFlag, 12025u16 => Player::PvpRank, 12027u16 => Player::PvpTimer, 12024u16
    => Player::PvpXp, 11205u16 => Player::QuickUseBar, 8912u16 => Player::Race, 10123u16
    => Player::RankingEdnamobsTotal, 10125u16 => Player::RankingGearTotal, 10130u16 =>
    Player::RankingKillsPve, 10129u16 => Player::RankingKillsPvp, 10126u16 =>
    Player::RankingMypadRooms, 10127u16 => Player::RankingSomaAdd, 10128u16 =>
    Player::RankingSomaTotal, 10124u16 => Player::RankingTotal, 9987u16 =>
    Player::RecentlyKilledInPvP, 7526u16 => Player::ReferenceList, 2620u16 =>
    Player::RelativePosToCarrier, 6023u16 => Player::RequestTeleportPos, 12185u16 =>
    Player::ResetDailyQuestList, 2703u16 => Player::Rot, 10146u16 =>
    Player::ScoreMinigameBilliards, 2704u16 => Player::SelfRadius, 10016u16 =>
    Player::SheathedModeActive, 11867u16 => Player::SignClanCharterItem, 2705u16 =>
    Player::Size, 7946u16 => Player::SomaCarried, 6457u16 => Player::SomaLootRate,
    11011u16 => Player::SpawnCinematicOverride, 4944u16 => Player::SpawnedOnAvatar,
    2618u16 => Player::SpawnMode, 2634u16 => Player::SpectateName, 2633u16 =>
    Player::SpectatePartyGuid, 10067u16 => Player::StatAnyDmgReduction, 10024u16 =>
    Player::StatAoEMaxSubTargets, 10023u16 => Player::StatAoESubTargetsDamageMod, 9556u16
    => Player::StatArmorRating, 9555u16 => Player::StatArmorReduction, 9563u16 =>
    Player::StatAttackPower, 12098u16 => Player::StatAttackPowerBonus, 12099u16 =>
    Player::StatAttackPowerRating, 8929u16 => Player::StatAttackRangePhysAdd, 9564u16 =>
    Player::StatAttackRating, 9559u16 => Player::StatBendChance, 9560u16 =>
    Player::StatBendRating, 6897u16 => Player::StatBlockChance, 9602u16 =>
    Player::StatBlockedDamageMod, 6896u16 => Player::StatBlockRating, 9561u16 =>
    Player::StatCritChance, 12093u16 => Player::StatCritDmgRating, 8930u16 =>
    Player::StatCriticalChanceReduction, 9604u16 => Player::StatCriticalDamageMod,
    12094u16 => Player::StatCriticalDamageModBonus, 9562u16 => Player::StatCritRating,
    10050u16 => Player::StatDamagePercPerMeterMod, 6735u16 =>
    Player::StatDefencePowerPhys, 6728u16 => Player::StatDefenceRatingPhys, 6657u16 =>
    Player::StatDodgeChance, 6658u16 => Player::StatDodgeRating, 6499u16 =>
    Player::StatEnergyCurrentH1, 6498u16 => Player::StatEnergyCurrentH2, 6497u16 =>
    Player::StatEnergyCurrentH3, 6496u16 => Player::StatEnergyCurrentS1, 6495u16 =>
    Player::StatEnergyCurrentS2, 6494u16 => Player::StatEnergyCurrentS3, 6720u16 =>
    Player::StatEvadeChance, 6721u16 => Player::StatEvadeRating, 10068u16 =>
    Player::StatExtraHealthRegen, 11215u16 => Player::StatFinalDamageMod, 11216u16 =>
    Player::StatFinalHealingMod, 10051u16 => Player::StatFreeFallDistanceMod, 11243u16 =>
    Player::StatHasteClassSkills, 9356u16 => Player::StatHastePhysNormal, 11390u16 =>
    Player::StatHealingReceivedMod, 12097u16 => Player::StatHeavyBonus, 7051u16 =>
    Player::StatHeavyEnergyPerHit, 12092u16 => Player::StatHeavyRating, 9553u16 =>
    Player::StatHitChance, 9554u16 => Player::StatHitRating, 10624u16 =>
    Player::StatInitialThreatMod, 6718u16 => Player::StatParryChance, 6719u16 =>
    Player::StatParryRating, 12095u16 => Player::StatPeneBonus, 12091u16 =>
    Player::StatPeneRating, 9557u16 => Player::StatReflectChance, 9558u16 =>
    Player::StatReflectRating, 12096u16 => Player::StatSpecialBonus, 7050u16 =>
    Player::StatSpecialEnergyPerHit, 12090u16 => Player::StatSpecialRating, 9609u16 =>
    Player::StatStamina, 7053u16 => Player::StatTcMax, 9306u16 => Player::StatThreatMod,
    9589u16 => Player::StatWeaponDps, 12389u16 => Player::StatWepMaxDmg, 12388u16 =>
    Player::StatWepMinDmg, 7088u16 => Player::StatXpMod, 10630u16 =>
    Player::StickyTargets, 2706u16 => Player::Tags, 2707u16 => Player::Target, 3045u16 =>
    Player::TeamId, 10147u16 => Player::TimePlayedBeforeThisSession, 10148u16 =>
    Player::TimePlayedThisLevelBeforeThisSession, 2600u16 => Player::TutorialMode,
    2672u16 => Player::Ue3ClassId, 12444u16 => Player::UiHintsAvailable, 10136u16 =>
    Player::UnassignPortals, 6502u16 => Player::UnLockedInstances, 4038u16 =>
    Player::UnLockedPortals, 11309u16 => Player::UnlockedUiWindows, 2617u16 =>
    Player::VisibleItemInfo, 2709u16 => Player::Weapon, 10037u16 => Player::WorldMapGuid,
    2710u16 => Player::Xp, 2711u16 => Player::XpForNextLevel, 7052u16 => Player::XpTotal,
    2712u16 => Player::Zone, 2628u16 => Player::ZoneGuid,
};
impl Attribute for Player {
    fn class() -> Class {
        Class::Player
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Abilities => &Self::Abilities,
            Self::AccountBankId => &Self::AccountBankId,
            Self::AccountBankSize => &Self::AccountBankSize,
            Self::AccountId => &Self::AccountId,
            Self::AccountName => &Self::AccountName,
            Self::Action0 => &Self::Action0,
            Self::Action0Duration => &Self::Action0Duration,
            Self::AddressSlots => &Self::AddressSlots,
            Self::Alive => &Self::Alive,
            Self::AttackedBy => &Self::AttackedBy,
            Self::AttributeAttackPowerPhys => &Self::AttributeAttackPowerPhys,
            Self::AttributeAttackPowerSpell => &Self::AttributeAttackPowerSpell,
            Self::AttributeConstitution => &Self::AttributeConstitution,
            Self::AttributeCrafting => &Self::AttributeCrafting,
            Self::AttributeCriticalPhys => &Self::AttributeCriticalPhys,
            Self::AttributeCriticalSpell => &Self::AttributeCriticalSpell,
            Self::AttributeDegenerateLevel => &Self::AttributeDegenerateLevel,
            Self::AttributeDexterity => &Self::AttributeDexterity,
            Self::AttributeDisguise => &Self::AttributeDisguise,
            Self::AttributeEnergy => &Self::AttributeEnergy,
            Self::AttributeEnergyCurrent => &Self::AttributeEnergyCurrent,
            Self::AttributeEnergyDecayStealthedPercentageNormalized => {
                &Self::AttributeEnergyDecayStealthedPercentageNormalized
            }
            Self::AttributeEnergyEquilibriumPercentageNormalized => {
                &Self::AttributeEnergyEquilibriumPercentageNormalized
            }
            Self::AttributeEnergyGainAutoAttackHitAbsolute => {
                &Self::AttributeEnergyGainAutoAttackHitAbsolute
            }
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => {
                &Self::AttributeEnergyGainWithTargetPerSecondAbsolute
            }
            Self::AttributeEnergyInitialPercentageNormalized => {
                &Self::AttributeEnergyInitialPercentageNormalized
            }
            Self::AttributeEnergyMax => &Self::AttributeEnergyMax,
            Self::AttributeEnergyRegen => &Self::AttributeEnergyRegen,
            Self::AttributeFocus => &Self::AttributeFocus,
            Self::AttributeHastePhys => &Self::AttributeHastePhys,
            Self::AttributeHasteSpell => &Self::AttributeHasteSpell,
            Self::AttributeHealth => &Self::AttributeHealth,
            Self::AttributeHealthRegen => &Self::AttributeHealthRegen,
            Self::AttributeHitRatingPhys => &Self::AttributeHitRatingPhys,
            Self::AttributeHitRatingSpell => &Self::AttributeHitRatingSpell,
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => {
                &Self::AttributeInCombatToEquilibriumPerSecondAbsolute
            }
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => {
                &Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized
            }
            Self::AttributeIntuition => &Self::AttributeIntuition,
            Self::AttributeItemLevel => &Self::AttributeItemLevel,
            Self::AttributeJump => &Self::AttributeJump,
            Self::AttributeMissRatingPhys => &Self::AttributeMissRatingPhys,
            Self::AttributeMissRatingSpell => &Self::AttributeMissRatingSpell,
            Self::AttributeMovement => &Self::AttributeMovement,
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => {
                &Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute
            }
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                &Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized
            }
            Self::AttributeResilience => &Self::AttributeResilience,
            Self::AttributeRun => &Self::AttributeRun,
            Self::AttributeStealthLevel => &Self::AttributeStealthLevel,
            Self::AttributeStrength => &Self::AttributeStrength,
            Self::AttributeWisdom => &Self::AttributeWisdom,
            Self::AutoLootRadius => &Self::AutoLootRadius,
            Self::AvailableEdnaClones => &Self::AvailableEdnaClones,
            Self::AvailableOutfits => &Self::AvailableOutfits,
            Self::AwareDist => &Self::AwareDist,
            Self::AwareRange => &Self::AwareRange,
            Self::BgLastWordZoneGuid => &Self::BgLastWordZoneGuid,
            Self::BgLastZoneGuid => &Self::BgLastZoneGuid,
            Self::BgLastZonePosition => &Self::BgLastZonePosition,
            Self::BgStatisticsString => &Self::BgStatisticsString,
            Self::BgTeam => &Self::BgTeam,
            Self::Bling => &Self::Bling,
            Self::BlockedAbilityEffectTypes => &Self::BlockedAbilityEffectTypes,
            Self::CarrierGuid => &Self::CarrierGuid,
            Self::ChatIgnoreSet => &Self::ChatIgnoreSet,
            Self::ClanGuid => &Self::ClanGuid,
            Self::ClanHateList => &Self::ClanHateList,
            Self::ClanName => &Self::ClanName,
            Self::ClanPrivileges => &Self::ClanPrivileges,
            Self::ClanRank => &Self::ClanRank,
            Self::ClanRatified => &Self::ClanRatified,
            Self::ClassData => &Self::ClassData,
            Self::ClassSkillCollection => &Self::ClassSkillCollection,
            Self::ClientActionTracker => &Self::ClientActionTracker,
            Self::ClientReady => &Self::ClientReady,
            Self::Cloaked => &Self::Cloaked,
            Self::ClusterGuid => &Self::ClusterGuid,
            Self::CollisionExtent => &Self::CollisionExtent,
            Self::CombatStyle => &Self::CombatStyle,
            Self::ContentClass => &Self::ContentClass,
            Self::CooldownManager => &Self::CooldownManager,
            Self::CooldownPassed => &Self::CooldownPassed,
            Self::CooldownTrackers => &Self::CooldownTrackers,
            Self::CurrentAbilityBarReferences => &Self::CurrentAbilityBarReferences,
            Self::CurrentClassSkills => &Self::CurrentClassSkills,
            Self::CurrentMyLandAddress => &Self::CurrentMyLandAddress,
            Self::CurrentOutfitSlot => &Self::CurrentOutfitSlot,
            Self::CurrentSkin => &Self::CurrentSkin,
            Self::CurrentTickedItemSlot => &Self::CurrentTickedItemSlot,
            Self::CustomizationBrowAngle => &Self::CustomizationBrowAngle,
            Self::CustomizationBustSize => &Self::CustomizationBustSize,
            Self::CustomizationCheek => &Self::CustomizationCheek,
            Self::CustomizationCheekBone => &Self::CustomizationCheekBone,
            Self::CustomizationChinPortude => &Self::CustomizationChinPortude,
            Self::CustomizationEarElf => &Self::CustomizationEarElf,
            Self::CustomizationEarSize => &Self::CustomizationEarSize,
            Self::CustomizationEyeBrowPos => &Self::CustomizationEyeBrowPos,
            Self::CustomizationEyePos => &Self::CustomizationEyePos,
            Self::CustomizationEyePosSpacing => &Self::CustomizationEyePosSpacing,
            Self::CustomizationEyeSizeLength => &Self::CustomizationEyeSizeLength,
            Self::CustomizationEyeSizeWidth => &Self::CustomizationEyeSizeWidth,
            Self::CustomizationEyesPretty => &Self::CustomizationEyesPretty,
            Self::CustomizationFat => &Self::CustomizationFat,
            Self::CustomizationGender => &Self::CustomizationGender,
            Self::CustomizationHeight => &Self::CustomizationHeight,
            Self::CustomizationJawChubby => &Self::CustomizationJawChubby,
            Self::CustomizationMouthExpression => &Self::CustomizationMouthExpression,
            Self::CustomizationMouthLowerLipThic => &Self::CustomizationMouthLowerLipThic,
            Self::CustomizationMouthPos => &Self::CustomizationMouthPos,
            Self::CustomizationMouthUpperLipThic => &Self::CustomizationMouthUpperLipThic,
            Self::CustomizationMouthWidth => &Self::CustomizationMouthWidth,
            Self::CustomizationMuscular => &Self::CustomizationMuscular,
            Self::CustomizationNosePortude => &Self::CustomizationNosePortude,
            Self::CustomizationNosePosLength => &Self::CustomizationNosePosLength,
            Self::CustomizationNosePosWidth => &Self::CustomizationNosePosWidth,
            Self::CustomizationSkinny => &Self::CustomizationSkinny,
            Self::DamageOutputMod => &Self::DamageOutputMod,
            Self::DamageReceivedMod => &Self::DamageReceivedMod,
            Self::DeathInfo => &Self::DeathInfo,
            Self::DefaultItemsContentGuid => &Self::DefaultItemsContentGuid,
            Self::DepositAmount => &Self::DepositAmount,
            Self::DepositBankGuid => &Self::DepositBankGuid,
            Self::DepositHistory => &Self::DepositHistory,
            Self::DepositLevel => &Self::DepositLevel,
            Self::DungoneKillBoss => &Self::DungoneKillBoss,
            Self::EmoteSlots => &Self::EmoteSlots,
            Self::EmoteUsed => &Self::EmoteUsed,
            Self::EnemyId => &Self::EnemyId,
            Self::Faction => &Self::Faction,
            Self::FactionStandings => &Self::FactionStandings,
            Self::FirstTimeSpawn => &Self::FirstTimeSpawn,
            Self::FragmentSlots => &Self::FragmentSlots,
            Self::FreedomProperties => &Self::FreedomProperties,
            Self::Freq => &Self::Freq,
            Self::GameCash => &Self::GameCash,
            Self::GenerateInterestList => &Self::GenerateInterestList,
            Self::GuideToAvatar => &Self::GuideToAvatar,
            Self::GuideToLocation => &Self::GuideToLocation,
            Self::HasAttributes => &Self::HasAttributes,
            Self::HeavySpecialSkillData => &Self::HeavySpecialSkillData,
            Self::HostIp => &Self::HostIp,
            Self::HpCur => &Self::HpCur,
            Self::HpMax => &Self::HpMax,
            Self::HpMin => &Self::HpMin,
            Self::Icon => &Self::Icon,
            Self::InGameSession => &Self::InGameSession,
            Self::InInstancedBattle => &Self::InInstancedBattle,
            Self::InitialWorldTimeThisLevelThisSession => {
                &Self::InitialWorldTimeThisLevelThisSession
            }
            Self::InitialWorldTimeThisSession => &Self::InitialWorldTimeThisSession,
            Self::InMiniGame => &Self::InMiniGame,
            Self::InstanceZoneKey => &Self::InstanceZoneKey,
            Self::InteractionRadius => &Self::InteractionRadius,
            Self::InteractRadius => &Self::InteractRadius,
            Self::InventorySize => &Self::InventorySize,
            Self::IsAdmin => &Self::IsAdmin,
            Self::IsInCombat => &Self::IsInCombat,
            Self::IsInPvPZone => &Self::IsInPvPZone,
            Self::IsInsideInstanceZone => &Self::IsInsideInstanceZone,
            Self::IsInSocial => &Self::IsInSocial,
            Self::IsOnline => &Self::IsOnline,
            Self::IsUnAttackable => &Self::IsUnAttackable,
            Self::ItemSlotsVisible => &Self::ItemSlotsVisible,
            Self::JumpVelocity => &Self::JumpVelocity,
            Self::LastAttackPosition => &Self::LastAttackPosition,
            Self::LastEquippedWeapon => &Self::LastEquippedWeapon,
            Self::LastKnownClanLandRadius => &Self::LastKnownClanLandRadius,
            Self::LastLogoutTime => &Self::LastLogoutTime,
            Self::LastPortalUsed => &Self::LastPortalUsed,
            Self::LastResetDailyQuest => &Self::LastResetDailyQuest,
            Self::LastSkuSyncTime => &Self::LastSkuSyncTime,
            Self::LastVendorSyncTime => &Self::LastVendorSyncTime,
            Self::LoginCount => &Self::LoginCount,
            Self::LootItemGuid => &Self::LootItemGuid,
            Self::LootItemType => &Self::LootItemType,
            Self::Lvl => &Self::Lvl,
            Self::LvlHistory => &Self::LvlHistory,
            Self::MaxLevelCap => &Self::MaxLevelCap,
            Self::MetamorphItemList => &Self::MetamorphItemList,
            Self::MinigameData => &Self::MinigameData,
            Self::Mount => &Self::Mount,
            Self::Mover => &Self::Mover,
            Self::MoveSpeed => &Self::MoveSpeed,
            Self::MyLandData => &Self::MyLandData,
            Self::MyQuestTrack => &Self::MyQuestTrack,
            Self::MyShopGuid => &Self::MyShopGuid,
            Self::MySteamDlc => &Self::MySteamDlc,
            Self::MyUsedSteamDlc => &Self::MyUsedSteamDlc,
            Self::NewItems => &Self::NewItems,
            Self::OutfitNames => &Self::OutfitNames,
            Self::OutfitSlots => &Self::OutfitSlots,
            Self::OverrideFaction => &Self::OverrideFaction,
            Self::PartyGuid => &Self::PartyGuid,
            Self::Pet => &Self::Pet,
            Self::PhaseSelectionData => &Self::PhaseSelectionData,
            Self::PlaycountMinigameBilliards => &Self::PlaycountMinigameBilliards,
            Self::PlayerLoading => &Self::PlayerLoading,
            Self::PlayerNodeState => &Self::PlayerNodeState,
            Self::PlayerUsedSteamDlc => &Self::PlayerUsedSteamDlc,
            Self::PortalData => &Self::PortalData,
            Self::Pos => &Self::Pos,
            Self::Power => &Self::Power,
            Self::PvpEnabled => &Self::PvpEnabled,
            Self::PvpEnabledInMyLandServerSetting => {
                &Self::PvpEnabledInMyLandServerSetting
            }
            Self::PvpEnabledServerSetting => &Self::PvpEnabledServerSetting,
            Self::PvpEnableDuration => &Self::PvpEnableDuration,
            Self::PvpFlag => &Self::PvpFlag,
            Self::PvpRank => &Self::PvpRank,
            Self::PvpTimer => &Self::PvpTimer,
            Self::PvpXp => &Self::PvpXp,
            Self::QuickUseBar => &Self::QuickUseBar,
            Self::Race => &Self::Race,
            Self::RankingEdnamobsTotal => &Self::RankingEdnamobsTotal,
            Self::RankingGearTotal => &Self::RankingGearTotal,
            Self::RankingKillsPve => &Self::RankingKillsPve,
            Self::RankingKillsPvp => &Self::RankingKillsPvp,
            Self::RankingMypadRooms => &Self::RankingMypadRooms,
            Self::RankingSomaAdd => &Self::RankingSomaAdd,
            Self::RankingSomaTotal => &Self::RankingSomaTotal,
            Self::RankingTotal => &Self::RankingTotal,
            Self::RecentlyKilledInPvP => &Self::RecentlyKilledInPvP,
            Self::ReferenceList => &Self::ReferenceList,
            Self::RelativePosToCarrier => &Self::RelativePosToCarrier,
            Self::RequestTeleportPos => &Self::RequestTeleportPos,
            Self::ResetDailyQuestList => &Self::ResetDailyQuestList,
            Self::Rot => &Self::Rot,
            Self::ScoreMinigameBilliards => &Self::ScoreMinigameBilliards,
            Self::SelfRadius => &Self::SelfRadius,
            Self::SheathedModeActive => &Self::SheathedModeActive,
            Self::SignClanCharterItem => &Self::SignClanCharterItem,
            Self::Size => &Self::Size,
            Self::SomaCarried => &Self::SomaCarried,
            Self::SomaLootRate => &Self::SomaLootRate,
            Self::SpawnCinematicOverride => &Self::SpawnCinematicOverride,
            Self::SpawnedOnAvatar => &Self::SpawnedOnAvatar,
            Self::SpawnMode => &Self::SpawnMode,
            Self::SpectateName => &Self::SpectateName,
            Self::SpectatePartyGuid => &Self::SpectatePartyGuid,
            Self::StatAnyDmgReduction => &Self::StatAnyDmgReduction,
            Self::StatAoEMaxSubTargets => &Self::StatAoEMaxSubTargets,
            Self::StatAoESubTargetsDamageMod => &Self::StatAoESubTargetsDamageMod,
            Self::StatArmorRating => &Self::StatArmorRating,
            Self::StatArmorReduction => &Self::StatArmorReduction,
            Self::StatAttackPower => &Self::StatAttackPower,
            Self::StatAttackPowerBonus => &Self::StatAttackPowerBonus,
            Self::StatAttackPowerRating => &Self::StatAttackPowerRating,
            Self::StatAttackRangePhysAdd => &Self::StatAttackRangePhysAdd,
            Self::StatAttackRating => &Self::StatAttackRating,
            Self::StatBendChance => &Self::StatBendChance,
            Self::StatBendRating => &Self::StatBendRating,
            Self::StatBlockChance => &Self::StatBlockChance,
            Self::StatBlockedDamageMod => &Self::StatBlockedDamageMod,
            Self::StatBlockRating => &Self::StatBlockRating,
            Self::StatCritChance => &Self::StatCritChance,
            Self::StatCritDmgRating => &Self::StatCritDmgRating,
            Self::StatCriticalChanceReduction => &Self::StatCriticalChanceReduction,
            Self::StatCriticalDamageMod => &Self::StatCriticalDamageMod,
            Self::StatCriticalDamageModBonus => &Self::StatCriticalDamageModBonus,
            Self::StatCritRating => &Self::StatCritRating,
            Self::StatDamagePercPerMeterMod => &Self::StatDamagePercPerMeterMod,
            Self::StatDefencePowerPhys => &Self::StatDefencePowerPhys,
            Self::StatDefenceRatingPhys => &Self::StatDefenceRatingPhys,
            Self::StatDodgeChance => &Self::StatDodgeChance,
            Self::StatDodgeRating => &Self::StatDodgeRating,
            Self::StatEnergyCurrentH1 => &Self::StatEnergyCurrentH1,
            Self::StatEnergyCurrentH2 => &Self::StatEnergyCurrentH2,
            Self::StatEnergyCurrentH3 => &Self::StatEnergyCurrentH3,
            Self::StatEnergyCurrentS1 => &Self::StatEnergyCurrentS1,
            Self::StatEnergyCurrentS2 => &Self::StatEnergyCurrentS2,
            Self::StatEnergyCurrentS3 => &Self::StatEnergyCurrentS3,
            Self::StatEvadeChance => &Self::StatEvadeChance,
            Self::StatEvadeRating => &Self::StatEvadeRating,
            Self::StatExtraHealthRegen => &Self::StatExtraHealthRegen,
            Self::StatFinalDamageMod => &Self::StatFinalDamageMod,
            Self::StatFinalHealingMod => &Self::StatFinalHealingMod,
            Self::StatFreeFallDistanceMod => &Self::StatFreeFallDistanceMod,
            Self::StatHasteClassSkills => &Self::StatHasteClassSkills,
            Self::StatHastePhysNormal => &Self::StatHastePhysNormal,
            Self::StatHealingReceivedMod => &Self::StatHealingReceivedMod,
            Self::StatHeavyBonus => &Self::StatHeavyBonus,
            Self::StatHeavyEnergyPerHit => &Self::StatHeavyEnergyPerHit,
            Self::StatHeavyRating => &Self::StatHeavyRating,
            Self::StatHitChance => &Self::StatHitChance,
            Self::StatHitRating => &Self::StatHitRating,
            Self::StatInitialThreatMod => &Self::StatInitialThreatMod,
            Self::StatParryChance => &Self::StatParryChance,
            Self::StatParryRating => &Self::StatParryRating,
            Self::StatPeneBonus => &Self::StatPeneBonus,
            Self::StatPeneRating => &Self::StatPeneRating,
            Self::StatReflectChance => &Self::StatReflectChance,
            Self::StatReflectRating => &Self::StatReflectRating,
            Self::StatSpecialBonus => &Self::StatSpecialBonus,
            Self::StatSpecialEnergyPerHit => &Self::StatSpecialEnergyPerHit,
            Self::StatSpecialRating => &Self::StatSpecialRating,
            Self::StatStamina => &Self::StatStamina,
            Self::StatTcMax => &Self::StatTcMax,
            Self::StatThreatMod => &Self::StatThreatMod,
            Self::StatWeaponDps => &Self::StatWeaponDps,
            Self::StatWepMaxDmg => &Self::StatWepMaxDmg,
            Self::StatWepMinDmg => &Self::StatWepMinDmg,
            Self::StatXpMod => &Self::StatXpMod,
            Self::StickyTargets => &Self::StickyTargets,
            Self::Tags => &Self::Tags,
            Self::Target => &Self::Target,
            Self::TeamId => &Self::TeamId,
            Self::TimePlayedBeforeThisSession => &Self::TimePlayedBeforeThisSession,
            Self::TimePlayedThisLevelBeforeThisSession => {
                &Self::TimePlayedThisLevelBeforeThisSession
            }
            Self::TutorialMode => &Self::TutorialMode,
            Self::Ue3ClassId => &Self::Ue3ClassId,
            Self::UiHintsAvailable => &Self::UiHintsAvailable,
            Self::UnassignPortals => &Self::UnassignPortals,
            Self::UnLockedInstances => &Self::UnLockedInstances,
            Self::UnLockedPortals => &Self::UnLockedPortals,
            Self::UnlockedUiWindows => &Self::UnlockedUiWindows,
            Self::VisibleItemInfo => &Self::VisibleItemInfo,
            Self::Weapon => &Self::Weapon,
            Self::WorldMapGuid => &Self::WorldMapGuid,
            Self::Xp => &Self::Xp,
            Self::XpForNextLevel => &Self::XpForNextLevel,
            Self::XpTotal => &Self::XpTotal,
            Self::Zone => &Self::Zone,
            Self::ZoneGuid => &Self::ZoneGuid,
        }
    }
}
impl AttributeInfo for Player {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Abilities => 9350u16,
            Self::AccountBankId => 12268u16,
            Self::AccountBankSize => 12269u16,
            Self::AccountId => 10603u16,
            Self::AccountName => 10602u16,
            Self::Action0 => 2673u16,
            Self::Action0Duration => 2674u16,
            Self::AddressSlots => 5904u16,
            Self::Alive => 2675u16,
            Self::AttackedBy => 2676u16,
            Self::AttributeAttackPowerPhys => 6258u16,
            Self::AttributeAttackPowerSpell => 6256u16,
            Self::AttributeConstitution => 4326u16,
            Self::AttributeCrafting => 4325u16,
            Self::AttributeCriticalPhys => 6255u16,
            Self::AttributeCriticalSpell => 6254u16,
            Self::AttributeDegenerateLevel => 6539u16,
            Self::AttributeDexterity => 4327u16,
            Self::AttributeDisguise => 4324u16,
            Self::AttributeEnergy => 4323u16,
            Self::AttributeEnergyCurrent => 4547u16,
            Self::AttributeEnergyDecayStealthedPercentageNormalized => 4549u16,
            Self::AttributeEnergyEquilibriumPercentageNormalized => 4556u16,
            Self::AttributeEnergyGainAutoAttackHitAbsolute => 4550u16,
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => 4557u16,
            Self::AttributeEnergyInitialPercentageNormalized => 4555u16,
            Self::AttributeEnergyMax => 4548u16,
            Self::AttributeEnergyRegen => 6265u16,
            Self::AttributeFocus => 4322u16,
            Self::AttributeHastePhys => 6253u16,
            Self::AttributeHasteSpell => 6252u16,
            Self::AttributeHealth => 6267u16,
            Self::AttributeHealthRegen => 6266u16,
            Self::AttributeHitRatingPhys => 6264u16,
            Self::AttributeHitRatingSpell => 6261u16,
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => 4554u16,
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => 4553u16,
            Self::AttributeIntuition => 4321u16,
            Self::AttributeItemLevel => 12143u16,
            Self::AttributeJump => 6263u16,
            Self::AttributeMissRatingPhys => 6260u16,
            Self::AttributeMissRatingSpell => 6259u16,
            Self::AttributeMovement => 4320u16,
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => 4552u16,
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                4551u16
            }
            Self::AttributeResilience => 6268u16,
            Self::AttributeRun => 6262u16,
            Self::AttributeStealthLevel => 4559u16,
            Self::AttributeStrength => 4319u16,
            Self::AttributeWisdom => 4318u16,
            Self::AutoLootRadius => 11393u16,
            Self::AvailableEdnaClones => 10049u16,
            Self::AvailableOutfits => 10040u16,
            Self::AwareDist => 2678u16,
            Self::AwareRange => 8295u16,
            Self::BgLastWordZoneGuid => 12104u16,
            Self::BgLastZoneGuid => 12103u16,
            Self::BgLastZonePosition => 12102u16,
            Self::BgStatisticsString => 12100u16,
            Self::BgTeam => 12101u16,
            Self::Bling => 6465u16,
            Self::BlockedAbilityEffectTypes => 2627u16,
            Self::CarrierGuid => 2619u16,
            Self::ChatIgnoreSet => 2679u16,
            Self::ClanGuid => 8904u16,
            Self::ClanHateList => 8910u16,
            Self::ClanName => 8909u16,
            Self::ClanPrivileges => 10928u16,
            Self::ClanRank => 8903u16,
            Self::ClanRatified => 10929u16,
            Self::ClassData => 11080u16,
            Self::ClassSkillCollection => 10518u16,
            Self::ClientActionTracker => 4942u16,
            Self::ClientReady => 2647u16,
            Self::Cloaked => 2668u16,
            Self::ClusterGuid => 2680u16,
            Self::CollisionExtent => 2681u16,
            Self::CombatStyle => 2616u16,
            Self::ContentClass => 2671u16,
            Self::CooldownManager => 11327u16,
            Self::CooldownPassed => 12291u16,
            Self::CooldownTrackers => 4039u16,
            Self::CurrentAbilityBarReferences => 10604u16,
            Self::CurrentClassSkills => 10605u16,
            Self::CurrentMyLandAddress => 6655u16,
            Self::CurrentOutfitSlot => 7265u16,
            Self::CurrentSkin => 2669u16,
            Self::CurrentTickedItemSlot => 10097u16,
            Self::CustomizationBrowAngle => 12322u16,
            Self::CustomizationBustSize => 8562u16,
            Self::CustomizationCheek => 12304u16,
            Self::CustomizationCheekBone => 12305u16,
            Self::CustomizationChinPortude => 12303u16,
            Self::CustomizationEarElf => 12306u16,
            Self::CustomizationEarSize => 12307u16,
            Self::CustomizationEyeBrowPos => 12321u16,
            Self::CustomizationEyePos => 12319u16,
            Self::CustomizationEyePosSpacing => 12320u16,
            Self::CustomizationEyeSizeLength => 12318u16,
            Self::CustomizationEyeSizeWidth => 12317u16,
            Self::CustomizationEyesPretty => 12316u16,
            Self::CustomizationFat => 4295u16,
            Self::CustomizationGender => 4291u16,
            Self::CustomizationHeight => 4292u16,
            Self::CustomizationJawChubby => 12302u16,
            Self::CustomizationMouthExpression => 12311u16,
            Self::CustomizationMouthLowerLipThic => 12313u16,
            Self::CustomizationMouthPos => 12315u16,
            Self::CustomizationMouthUpperLipThic => 12312u16,
            Self::CustomizationMouthWidth => 12314u16,
            Self::CustomizationMuscular => 4293u16,
            Self::CustomizationNosePortude => 12308u16,
            Self::CustomizationNosePosLength => 12310u16,
            Self::CustomizationNosePosWidth => 12309u16,
            Self::CustomizationSkinny => 4294u16,
            Self::DamageOutputMod => 6008u16,
            Self::DamageReceivedMod => 6007u16,
            Self::DeathInfo => 11229u16,
            Self::DefaultItemsContentGuid => 6786u16,
            Self::DepositAmount => 9221u16,
            Self::DepositBankGuid => 11582u16,
            Self::DepositHistory => 9219u16,
            Self::DepositLevel => 9220u16,
            Self::DungoneKillBoss => 12144u16,
            Self::EmoteSlots => 7041u16,
            Self::EmoteUsed => 8040u16,
            Self::EnemyId => 12293u16,
            Self::Faction => 2638u16,
            Self::FactionStandings => 2639u16,
            Self::FirstTimeSpawn => 2650u16,
            Self::FragmentSlots => 5998u16,
            Self::FreedomProperties => 11204u16,
            Self::Freq => 2715u16,
            Self::GameCash => 6592u16,
            Self::GenerateInterestList => 2663u16,
            Self::GuideToAvatar => 2636u16,
            Self::GuideToLocation => 2635u16,
            Self::HasAttributes => 6375u16,
            Self::HeavySpecialSkillData => 10996u16,
            Self::HostIp => 2687u16,
            Self::HpCur => 2688u16,
            Self::HpMax => 2689u16,
            Self::HpMin => 2642u16,
            Self::Icon => 4391u16,
            Self::InGameSession => 2691u16,
            Self::InInstancedBattle => 5151u16,
            Self::InitialWorldTimeThisLevelThisSession => 10149u16,
            Self::InitialWorldTimeThisSession => 10150u16,
            Self::InMiniGame => 2631u16,
            Self::InstanceZoneKey => 5610u16,
            Self::InteractionRadius => 7527u16,
            Self::InteractRadius => 4191u16,
            Self::InventorySize => 9881u16,
            Self::IsAdmin => 12141u16,
            Self::IsInCombat => 2607u16,
            Self::IsInPvPZone => 9698u16,
            Self::IsInsideInstanceZone => 5508u16,
            Self::IsInSocial => 12527u16,
            Self::IsOnline => 12324u16,
            Self::IsUnAttackable => 7884u16,
            Self::ItemSlotsVisible => 12244u16,
            Self::JumpVelocity => 2692u16,
            Self::LastAttackPosition => 2632u16,
            Self::LastEquippedWeapon => 7887u16,
            Self::LastKnownClanLandRadius => 11001u16,
            Self::LastLogoutTime => 11026u16,
            Self::LastPortalUsed => 10386u16,
            Self::LastResetDailyQuest => 12140u16,
            Self::LastSkuSyncTime => 10988u16,
            Self::LastVendorSyncTime => 11230u16,
            Self::LoginCount => 10144u16,
            Self::LootItemGuid => 6463u16,
            Self::LootItemType => 7947u16,
            Self::Lvl => 2693u16,
            Self::LvlHistory => 10177u16,
            Self::MaxLevelCap => 12115u16,
            Self::MetamorphItemList => 11095u16,
            Self::MinigameData => 2626u16,
            Self::Mount => 3771u16,
            Self::Mover => 2649u16,
            Self::MoveSpeed => 2694u16,
            Self::MyLandData => 5622u16,
            Self::MyQuestTrack => 12415u16,
            Self::MyShopGuid => 11583u16,
            Self::MySteamDlc => 12117u16,
            Self::MyUsedSteamDlc => 12116u16,
            Self::NewItems => 11231u16,
            Self::OutfitNames => 10048u16,
            Self::OutfitSlots => 7264u16,
            Self::OverrideFaction => 2640u16,
            Self::PartyGuid => 2697u16,
            Self::Pet => 12181u16,
            Self::PhaseSelectionData => 5212u16,
            Self::PlaycountMinigameBilliards => 10145u16,
            Self::PlayerLoading => 4943u16,
            Self::PlayerNodeState => 11252u16,
            Self::PlayerUsedSteamDlc => 12288u16,
            Self::PortalData => 10033u16,
            Self::Pos => 2699u16,
            Self::Power => 2714u16,
            Self::PvpEnabled => 9358u16,
            Self::PvpEnabledInMyLandServerSetting => 9985u16,
            Self::PvpEnabledServerSetting => 9986u16,
            Self::PvpEnableDuration => 12026u16,
            Self::PvpFlag => 12028u16,
            Self::PvpRank => 12025u16,
            Self::PvpTimer => 12027u16,
            Self::PvpXp => 12024u16,
            Self::QuickUseBar => 11205u16,
            Self::Race => 8912u16,
            Self::RankingEdnamobsTotal => 10123u16,
            Self::RankingGearTotal => 10125u16,
            Self::RankingKillsPve => 10130u16,
            Self::RankingKillsPvp => 10129u16,
            Self::RankingMypadRooms => 10126u16,
            Self::RankingSomaAdd => 10127u16,
            Self::RankingSomaTotal => 10128u16,
            Self::RankingTotal => 10124u16,
            Self::RecentlyKilledInPvP => 9987u16,
            Self::ReferenceList => 7526u16,
            Self::RelativePosToCarrier => 2620u16,
            Self::RequestTeleportPos => 6023u16,
            Self::ResetDailyQuestList => 12185u16,
            Self::Rot => 2703u16,
            Self::ScoreMinigameBilliards => 10146u16,
            Self::SelfRadius => 2704u16,
            Self::SheathedModeActive => 10016u16,
            Self::SignClanCharterItem => 11867u16,
            Self::Size => 2705u16,
            Self::SomaCarried => 7946u16,
            Self::SomaLootRate => 6457u16,
            Self::SpawnCinematicOverride => 11011u16,
            Self::SpawnedOnAvatar => 4944u16,
            Self::SpawnMode => 2618u16,
            Self::SpectateName => 2634u16,
            Self::SpectatePartyGuid => 2633u16,
            Self::StatAnyDmgReduction => 10067u16,
            Self::StatAoEMaxSubTargets => 10024u16,
            Self::StatAoESubTargetsDamageMod => 10023u16,
            Self::StatArmorRating => 9556u16,
            Self::StatArmorReduction => 9555u16,
            Self::StatAttackPower => 9563u16,
            Self::StatAttackPowerBonus => 12098u16,
            Self::StatAttackPowerRating => 12099u16,
            Self::StatAttackRangePhysAdd => 8929u16,
            Self::StatAttackRating => 9564u16,
            Self::StatBendChance => 9559u16,
            Self::StatBendRating => 9560u16,
            Self::StatBlockChance => 6897u16,
            Self::StatBlockedDamageMod => 9602u16,
            Self::StatBlockRating => 6896u16,
            Self::StatCritChance => 9561u16,
            Self::StatCritDmgRating => 12093u16,
            Self::StatCriticalChanceReduction => 8930u16,
            Self::StatCriticalDamageMod => 9604u16,
            Self::StatCriticalDamageModBonus => 12094u16,
            Self::StatCritRating => 9562u16,
            Self::StatDamagePercPerMeterMod => 10050u16,
            Self::StatDefencePowerPhys => 6735u16,
            Self::StatDefenceRatingPhys => 6728u16,
            Self::StatDodgeChance => 6657u16,
            Self::StatDodgeRating => 6658u16,
            Self::StatEnergyCurrentH1 => 6499u16,
            Self::StatEnergyCurrentH2 => 6498u16,
            Self::StatEnergyCurrentH3 => 6497u16,
            Self::StatEnergyCurrentS1 => 6496u16,
            Self::StatEnergyCurrentS2 => 6495u16,
            Self::StatEnergyCurrentS3 => 6494u16,
            Self::StatEvadeChance => 6720u16,
            Self::StatEvadeRating => 6721u16,
            Self::StatExtraHealthRegen => 10068u16,
            Self::StatFinalDamageMod => 11215u16,
            Self::StatFinalHealingMod => 11216u16,
            Self::StatFreeFallDistanceMod => 10051u16,
            Self::StatHasteClassSkills => 11243u16,
            Self::StatHastePhysNormal => 9356u16,
            Self::StatHealingReceivedMod => 11390u16,
            Self::StatHeavyBonus => 12097u16,
            Self::StatHeavyEnergyPerHit => 7051u16,
            Self::StatHeavyRating => 12092u16,
            Self::StatHitChance => 9553u16,
            Self::StatHitRating => 9554u16,
            Self::StatInitialThreatMod => 10624u16,
            Self::StatParryChance => 6718u16,
            Self::StatParryRating => 6719u16,
            Self::StatPeneBonus => 12095u16,
            Self::StatPeneRating => 12091u16,
            Self::StatReflectChance => 9557u16,
            Self::StatReflectRating => 9558u16,
            Self::StatSpecialBonus => 12096u16,
            Self::StatSpecialEnergyPerHit => 7050u16,
            Self::StatSpecialRating => 12090u16,
            Self::StatStamina => 9609u16,
            Self::StatTcMax => 7053u16,
            Self::StatThreatMod => 9306u16,
            Self::StatWeaponDps => 9589u16,
            Self::StatWepMaxDmg => 12389u16,
            Self::StatWepMinDmg => 12388u16,
            Self::StatXpMod => 7088u16,
            Self::StickyTargets => 10630u16,
            Self::Tags => 2706u16,
            Self::Target => 2707u16,
            Self::TeamId => 3045u16,
            Self::TimePlayedBeforeThisSession => 10147u16,
            Self::TimePlayedThisLevelBeforeThisSession => 10148u16,
            Self::TutorialMode => 2600u16,
            Self::Ue3ClassId => 2672u16,
            Self::UiHintsAvailable => 12444u16,
            Self::UnassignPortals => 10136u16,
            Self::UnLockedInstances => 6502u16,
            Self::UnLockedPortals => 4038u16,
            Self::UnlockedUiWindows => 11309u16,
            Self::VisibleItemInfo => 2617u16,
            Self::Weapon => 2709u16,
            Self::WorldMapGuid => 10037u16,
            Self::Xp => 2710u16,
            Self::XpForNextLevel => 2711u16,
            Self::XpTotal => 7052u16,
            Self::Zone => 2712u16,
            Self::ZoneGuid => 2628u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Abilities => "abilities",
            Self::AccountBankId => "accountBankID",
            Self::AccountBankSize => "accountBankSize",
            Self::AccountId => "accountId",
            Self::AccountName => "accountName",
            Self::Action0 => "action0",
            Self::Action0Duration => "action0Duration",
            Self::AddressSlots => "AddressSlots",
            Self::Alive => "alive",
            Self::AttackedBy => "attackedBy",
            Self::AttributeAttackPowerPhys => "attributeAttackPowerPhys",
            Self::AttributeAttackPowerSpell => "attributeAttackPowerSpell",
            Self::AttributeConstitution => "attributeConstitution",
            Self::AttributeCrafting => "attributeCrafting",
            Self::AttributeCriticalPhys => "attributeCriticalPhys",
            Self::AttributeCriticalSpell => "attributeCriticalSpell",
            Self::AttributeDegenerateLevel => "attributeDegenerate_Level",
            Self::AttributeDexterity => "attributeDexterity",
            Self::AttributeDisguise => "attributeDisguise",
            Self::AttributeEnergy => "attributeEnergy",
            Self::AttributeEnergyCurrent => "attributeEnergyCurrent",
            Self::AttributeEnergyDecayStealthedPercentageNormalized => {
                "attributeEnergyDecay_Stealthed_PercentageNormalized"
            }
            Self::AttributeEnergyEquilibriumPercentageNormalized => {
                "attributeEnergyEquilibrium_PercentageNormalized"
            }
            Self::AttributeEnergyGainAutoAttackHitAbsolute => {
                "attributeEnergyGain_AutoAttack_Hit_Absolute"
            }
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => {
                "attributeEnergyGain_WithTarget_PerSecond_Absolute"
            }
            Self::AttributeEnergyInitialPercentageNormalized => {
                "attributeEnergyInitial_PercentageNormalized"
            }
            Self::AttributeEnergyMax => "attributeEnergyMax",
            Self::AttributeEnergyRegen => "attributeEnergyRegen",
            Self::AttributeFocus => "attributeFocus",
            Self::AttributeHastePhys => "attributeHastePhys",
            Self::AttributeHasteSpell => "attributeHasteSpell",
            Self::AttributeHealth => "attributeHealth",
            Self::AttributeHealthRegen => "attributeHealthRegen",
            Self::AttributeHitRatingPhys => "attributeHitRatingPhys",
            Self::AttributeHitRatingSpell => "attributeHitRatingSpell",
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => {
                "attributeInCombat_ToEquilibrium_PerSecond_Absolute"
            }
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => {
                "attributeInCombat_ToEquilibrium_PerSecond_PercentageNormalized"
            }
            Self::AttributeIntuition => "attributeIntuition",
            Self::AttributeItemLevel => "attributeItem_Level",
            Self::AttributeJump => "attributeJump",
            Self::AttributeMissRatingPhys => "attributeMissRatingPhys",
            Self::AttributeMissRatingSpell => "attributeMissRatingSpell",
            Self::AttributeMovement => "attributeMovement",
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => {
                "attributeOutOfCombat_ToEquilibrium_PerSecond_Absolute"
            }
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                "attributeOutOfCombat_ToEquilibrium_PerSecond_PercentageNormalized"
            }
            Self::AttributeResilience => "attributeResilience",
            Self::AttributeRun => "attributeRun",
            Self::AttributeStealthLevel => "attributeStealth_Level",
            Self::AttributeStrength => "attributeStrength",
            Self::AttributeWisdom => "attributeWisdom",
            Self::AutoLootRadius => "autoLootRadius",
            Self::AvailableEdnaClones => "AvailableEDNAClones",
            Self::AvailableOutfits => "availableOutfits",
            Self::AwareDist => "awareDist",
            Self::AwareRange => "AwareRange",
            Self::BgLastWordZoneGuid => "BG_LastWordZoneGUID",
            Self::BgLastZoneGuid => "BG_LastZoneGUID",
            Self::BgLastZonePosition => "BG_LastZonePosition",
            Self::BgStatisticsString => "BG_StatisticsString",
            Self::BgTeam => "BG_Team",
            Self::Bling => "bling",
            Self::BlockedAbilityEffectTypes => "blockedAbilityEffectTypes",
            Self::CarrierGuid => "carrierGuid",
            Self::ChatIgnoreSet => "chatIgnoreSet",
            Self::ClanGuid => "clanGUID",
            Self::ClanHateList => "clanHateList",
            Self::ClanName => "clanName",
            Self::ClanPrivileges => "clanPrivileges",
            Self::ClanRank => "clanRank",
            Self::ClanRatified => "clanRatified",
            Self::ClassData => "classData",
            Self::ClassSkillCollection => "classSkillCollection",
            Self::ClientActionTracker => "clientActionTracker",
            Self::ClientReady => "clientReady",
            Self::Cloaked => "cloaked",
            Self::ClusterGuid => "clusterGUID",
            Self::CollisionExtent => "collisionExtent",
            Self::CombatStyle => "combatStyle",
            Self::ContentClass => "ContentClass",
            Self::CooldownManager => "cooldownManager",
            Self::CooldownPassed => "cooldownPassed",
            Self::CooldownTrackers => "cooldownTrackers",
            Self::CurrentAbilityBarReferences => "currentAbilityBarReferences",
            Self::CurrentClassSkills => "currentClassSkills",
            Self::CurrentMyLandAddress => "currentMyLandAddress",
            Self::CurrentOutfitSlot => "currentOutfitSlot",
            Self::CurrentSkin => "currentSkin",
            Self::CurrentTickedItemSlot => "currentTickedItemSlot",
            Self::CustomizationBrowAngle => "customizationBrowAngle",
            Self::CustomizationBustSize => "customizationBustSize",
            Self::CustomizationCheek => "customizationCheek",
            Self::CustomizationCheekBone => "customizationCheekBone",
            Self::CustomizationChinPortude => "customizationChinPortude",
            Self::CustomizationEarElf => "customizationEarElf",
            Self::CustomizationEarSize => "customizationEarSize",
            Self::CustomizationEyeBrowPos => "customizationEyeBrowPos",
            Self::CustomizationEyePos => "customizationEyePos",
            Self::CustomizationEyePosSpacing => "customizationEyePosSpacing",
            Self::CustomizationEyeSizeLength => "customizationEyeSizeLength",
            Self::CustomizationEyeSizeWidth => "customizationEyeSizeWidth",
            Self::CustomizationEyesPretty => "customizationEyesPretty",
            Self::CustomizationFat => "customizationFat",
            Self::CustomizationGender => "customizationGender",
            Self::CustomizationHeight => "customizationHeight",
            Self::CustomizationJawChubby => "customizationJawChubby",
            Self::CustomizationMouthExpression => "customizationMouthExpression",
            Self::CustomizationMouthLowerLipThic => "customizationMouthLowerLipThic",
            Self::CustomizationMouthPos => "customizationMouthPos",
            Self::CustomizationMouthUpperLipThic => "customizationMouthUpperLipThic",
            Self::CustomizationMouthWidth => "customizationMouthWidth",
            Self::CustomizationMuscular => "customizationMuscular",
            Self::CustomizationNosePortude => "customizationNosePortude",
            Self::CustomizationNosePosLength => "customizationNosePosLength",
            Self::CustomizationNosePosWidth => "customizationNosePosWidth",
            Self::CustomizationSkinny => "customizationSkinny",
            Self::DamageOutputMod => "damageOutputMod",
            Self::DamageReceivedMod => "damageReceivedMod",
            Self::DeathInfo => "deathInfo",
            Self::DefaultItemsContentGuid => "defaultItemsContentGuid",
            Self::DepositAmount => "DepositAmount",
            Self::DepositBankGuid => "depositBankGUID",
            Self::DepositHistory => "DepositHistory",
            Self::DepositLevel => "DepositLevel",
            Self::DungoneKillBoss => "DungoneKillBoss",
            Self::EmoteSlots => "EmoteSlots",
            Self::EmoteUsed => "EmoteUsed",
            Self::EnemyId => "enemyID",
            Self::Faction => "Faction",
            Self::FactionStandings => "FactionStandings",
            Self::FirstTimeSpawn => "firstTimeSpawn",
            Self::FragmentSlots => "FragmentSlots",
            Self::FreedomProperties => "FreedomProperties",
            Self::Freq => "Freq",
            Self::GameCash => "gameCash",
            Self::GenerateInterestList => "generateInterestList",
            Self::GuideToAvatar => "guideToAvatar",
            Self::GuideToLocation => "guideToLocation",
            Self::HasAttributes => "hasAttributes",
            Self::HeavySpecialSkillData => "heavySpecialSkillData",
            Self::HostIp => "hostIP",
            Self::HpCur => "hpCur",
            Self::HpMax => "hpMax",
            Self::HpMin => "hpMin",
            Self::Icon => "Icon",
            Self::InGameSession => "inGameSession",
            Self::InInstancedBattle => "inInstancedBattle",
            Self::InitialWorldTimeThisLevelThisSession => {
                "initialWorldTimeThisLevelThisSession"
            }
            Self::InitialWorldTimeThisSession => "initialWorldTimeThisSession",
            Self::InMiniGame => "inMiniGame",
            Self::InstanceZoneKey => "instanceZoneKey",
            Self::InteractionRadius => "InteractionRadius",
            Self::InteractRadius => "interactRadius",
            Self::InventorySize => "inventorySize",
            Self::IsAdmin => "isAdmin",
            Self::IsInCombat => "isInCombat",
            Self::IsInPvPZone => "isInPvPZone",
            Self::IsInsideInstanceZone => "isInsideInstanceZone",
            Self::IsInSocial => "isInSocial",
            Self::IsOnline => "isOnline",
            Self::IsUnAttackable => "isUnAttackable",
            Self::ItemSlotsVisible => "itemSlotsVisible",
            Self::JumpVelocity => "jumpVelocity",
            Self::LastAttackPosition => "lastAttackPosition",
            Self::LastEquippedWeapon => "lastEquippedWeapon",
            Self::LastKnownClanLandRadius => "lastKnownClanLandRadius",
            Self::LastLogoutTime => "lastLogoutTime",
            Self::LastPortalUsed => "LastPortalUsed",
            Self::LastResetDailyQuest => "lastResetDailyQuest",
            Self::LastSkuSyncTime => "lastSKUSyncTime",
            Self::LastVendorSyncTime => "lastVendorSyncTime",
            Self::LoginCount => "loginCount",
            Self::LootItemGuid => "lootItemGuid",
            Self::LootItemType => "lootItemType",
            Self::Lvl => "lvl",
            Self::LvlHistory => "lvlHistory",
            Self::MaxLevelCap => "maxLevelCap",
            Self::MetamorphItemList => "metamorphItemList",
            Self::MinigameData => "minigameData",
            Self::Mount => "mount",
            Self::Mover => "mover",
            Self::MoveSpeed => "moveSpeed",
            Self::MyLandData => "MyLandData",
            Self::MyQuestTrack => "myQuestTrack",
            Self::MyShopGuid => "myShopGUID",
            Self::MySteamDlc => "mySteamDlc",
            Self::MyUsedSteamDlc => "myUsedSteamDlc",
            Self::NewItems => "newItems",
            Self::OutfitNames => "outfitNames",
            Self::OutfitSlots => "OutfitSlots",
            Self::OverrideFaction => "OverrideFaction",
            Self::PartyGuid => "partyGUID",
            Self::Pet => "pet",
            Self::PhaseSelectionData => "phaseSelectionData",
            Self::PlaycountMinigameBilliards => "playcount_minigame_billiards",
            Self::PlayerLoading => "playerLoading",
            Self::PlayerNodeState => "playerNodeState",
            Self::PlayerUsedSteamDlc => "playerUsedSteamDlc",
            Self::PortalData => "PortalData",
            Self::Pos => "pos",
            Self::Power => "Power",
            Self::PvpEnabled => "pvpEnabled",
            Self::PvpEnabledInMyLandServerSetting => "pvpEnabledInMyLandServerSetting",
            Self::PvpEnabledServerSetting => "pvpEnabledServerSetting",
            Self::PvpEnableDuration => "pvpEnableDuration",
            Self::PvpFlag => "pvpFlag",
            Self::PvpRank => "pvpRank",
            Self::PvpTimer => "pvpTimer",
            Self::PvpXp => "pvpXP",
            Self::QuickUseBar => "quickUseBar",
            Self::Race => "race",
            Self::RankingEdnamobsTotal => "ranking_ednamobs_total",
            Self::RankingGearTotal => "ranking_gear_total",
            Self::RankingKillsPve => "ranking_kills_pve",
            Self::RankingKillsPvp => "ranking_kills_pvp",
            Self::RankingMypadRooms => "ranking_mypad_rooms",
            Self::RankingSomaAdd => "ranking_soma_add",
            Self::RankingSomaTotal => "ranking_soma_total",
            Self::RankingTotal => "ranking_total",
            Self::RecentlyKilledInPvP => "recentlyKilledInPvP",
            Self::ReferenceList => "ReferenceList",
            Self::RelativePosToCarrier => "relativePosToCarrier",
            Self::RequestTeleportPos => "requestTeleportPos",
            Self::ResetDailyQuestList => "resetDailyQuestList",
            Self::Rot => "rot",
            Self::ScoreMinigameBilliards => "score_minigame_billiards",
            Self::SelfRadius => "selfRadius",
            Self::SheathedModeActive => "sheathedModeActive",
            Self::SignClanCharterItem => "SignClanCharterItem",
            Self::Size => "Size",
            Self::SomaCarried => "somaCarried",
            Self::SomaLootRate => "somaLootRate",
            Self::SpawnCinematicOverride => "spawnCinematicOverride",
            Self::SpawnedOnAvatar => "spawnedOnAvatar",
            Self::SpawnMode => "spawnMode",
            Self::SpectateName => "spectateName",
            Self::SpectatePartyGuid => "spectatePartyGUID",
            Self::StatAnyDmgReduction => "statAnyDmgReduction",
            Self::StatAoEMaxSubTargets => "statAoE_MaxSubTargets",
            Self::StatAoESubTargetsDamageMod => "statAoE_SubTargetsDamageMod",
            Self::StatArmorRating => "statArmorRating",
            Self::StatArmorReduction => "statArmorReduction",
            Self::StatAttackPower => "statAttackPower",
            Self::StatAttackPowerBonus => "statAttackPowerBonus",
            Self::StatAttackPowerRating => "statAttackPowerRating",
            Self::StatAttackRangePhysAdd => "statAttackRangePhysAdd",
            Self::StatAttackRating => "statAttackRating",
            Self::StatBendChance => "statBendChance",
            Self::StatBendRating => "statBendRating",
            Self::StatBlockChance => "statBlockChance",
            Self::StatBlockedDamageMod => "statBlockedDamageMod",
            Self::StatBlockRating => "statBlockRating",
            Self::StatCritChance => "statCritChance",
            Self::StatCritDmgRating => "statCritDmgRating",
            Self::StatCriticalChanceReduction => "statCriticalChanceReduction",
            Self::StatCriticalDamageMod => "statCriticalDamageMod",
            Self::StatCriticalDamageModBonus => "statCriticalDamageModBonus",
            Self::StatCritRating => "statCritRating",
            Self::StatDamagePercPerMeterMod => "statDamagePercPerMeterMod",
            Self::StatDefencePowerPhys => "statDefencePowerPhys",
            Self::StatDefenceRatingPhys => "statDefenceRatingPhys",
            Self::StatDodgeChance => "statDodgeChance",
            Self::StatDodgeRating => "statDodgeRating",
            Self::StatEnergyCurrentH1 => "statEnergyCurrentH1",
            Self::StatEnergyCurrentH2 => "statEnergyCurrentH2",
            Self::StatEnergyCurrentH3 => "statEnergyCurrentH3",
            Self::StatEnergyCurrentS1 => "statEnergyCurrentS1",
            Self::StatEnergyCurrentS2 => "statEnergyCurrentS2",
            Self::StatEnergyCurrentS3 => "statEnergyCurrentS3",
            Self::StatEvadeChance => "statEvadeChance",
            Self::StatEvadeRating => "statEvadeRating",
            Self::StatExtraHealthRegen => "statExtraHealthRegen",
            Self::StatFinalDamageMod => "statFinalDamageMod",
            Self::StatFinalHealingMod => "statFinalHealingMod",
            Self::StatFreeFallDistanceMod => "statFreeFallDistanceMod",
            Self::StatHasteClassSkills => "statHasteClassSkills",
            Self::StatHastePhysNormal => "statHastePhysNormal",
            Self::StatHealingReceivedMod => "statHealingReceivedMod",
            Self::StatHeavyBonus => "statHeavyBonus",
            Self::StatHeavyEnergyPerHit => "statHeavyEnergyPerHit",
            Self::StatHeavyRating => "statHeavyRating",
            Self::StatHitChance => "statHitChance",
            Self::StatHitRating => "statHitRating",
            Self::StatInitialThreatMod => "statInitialThreatMod",
            Self::StatParryChance => "statParryChance",
            Self::StatParryRating => "statParryRating",
            Self::StatPeneBonus => "statPeneBonus",
            Self::StatPeneRating => "statPeneRating",
            Self::StatReflectChance => "statReflectChance",
            Self::StatReflectRating => "statReflectRating",
            Self::StatSpecialBonus => "statSpecialBonus",
            Self::StatSpecialEnergyPerHit => "statSpecialEnergyPerHit",
            Self::StatSpecialRating => "statSpecialRating",
            Self::StatStamina => "statStamina",
            Self::StatTcMax => "statTCMax",
            Self::StatThreatMod => "statThreatMod",
            Self::StatWeaponDps => "statWeaponDPS",
            Self::StatWepMaxDmg => "statWepMaxDmg",
            Self::StatWepMinDmg => "statWepMinDmg",
            Self::StatXpMod => "statXpMod",
            Self::StickyTargets => "stickyTargets",
            Self::Tags => "tags",
            Self::Target => "target",
            Self::TeamId => "teamID",
            Self::TimePlayedBeforeThisSession => "timePlayedBeforeThisSession",
            Self::TimePlayedThisLevelBeforeThisSession => {
                "timePlayedThisLevelBeforeThisSession"
            }
            Self::TutorialMode => "tutorialMode",
            Self::Ue3ClassId => "UE3ClassID",
            Self::UiHintsAvailable => "uiHintsAvailable",
            Self::UnassignPortals => "UnassignPortals",
            Self::UnLockedInstances => "unLockedInstances",
            Self::UnLockedPortals => "unLockedPortals",
            Self::UnlockedUiWindows => "unlockedUIWindows",
            Self::VisibleItemInfo => "visibleItemInfo",
            Self::Weapon => "weapon",
            Self::WorldMapGuid => "worldMapGUID",
            Self::Xp => "xp",
            Self::XpForNextLevel => "xpForNextLevel",
            Self::XpTotal => "xpTotal",
            Self::Zone => "zone",
            Self::ZoneGuid => "ZoneGuid",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Abilities => ParamType::ContentRefList,
            Self::AccountBankId => ParamType::Guid,
            Self::AccountBankSize => ParamType::Int,
            Self::AccountId => ParamType::Int,
            Self::AccountName => ParamType::String,
            Self::Action0 => ParamType::StringFloatPair,
            Self::Action0Duration => ParamType::Float,
            Self::AddressSlots => ParamType::Any,
            Self::Alive => ParamType::Bool,
            Self::AttackedBy => ParamType::AvatarId,
            Self::AttributeAttackPowerPhys => ParamType::Float,
            Self::AttributeAttackPowerSpell => ParamType::Float,
            Self::AttributeConstitution => ParamType::Float,
            Self::AttributeCrafting => ParamType::Float,
            Self::AttributeCriticalPhys => ParamType::Float,
            Self::AttributeCriticalSpell => ParamType::Float,
            Self::AttributeDegenerateLevel => ParamType::Float,
            Self::AttributeDexterity => ParamType::Float,
            Self::AttributeDisguise => ParamType::Float,
            Self::AttributeEnergy => ParamType::Float,
            Self::AttributeEnergyCurrent => ParamType::Float,
            Self::AttributeEnergyDecayStealthedPercentageNormalized => ParamType::Float,
            Self::AttributeEnergyEquilibriumPercentageNormalized => ParamType::Float,
            Self::AttributeEnergyGainAutoAttackHitAbsolute => ParamType::Float,
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => ParamType::Float,
            Self::AttributeEnergyInitialPercentageNormalized => ParamType::Float,
            Self::AttributeEnergyMax => ParamType::Float,
            Self::AttributeEnergyRegen => ParamType::Float,
            Self::AttributeFocus => ParamType::Float,
            Self::AttributeHastePhys => ParamType::Float,
            Self::AttributeHasteSpell => ParamType::Float,
            Self::AttributeHealth => ParamType::Float,
            Self::AttributeHealthRegen => ParamType::Float,
            Self::AttributeHitRatingPhys => ParamType::Float,
            Self::AttributeHitRatingSpell => ParamType::Float,
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => ParamType::Float,
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => {
                ParamType::Float
            }
            Self::AttributeIntuition => ParamType::Float,
            Self::AttributeItemLevel => ParamType::Float,
            Self::AttributeJump => ParamType::Float,
            Self::AttributeMissRatingPhys => ParamType::Float,
            Self::AttributeMissRatingSpell => ParamType::Float,
            Self::AttributeMovement => ParamType::Float,
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => ParamType::Float,
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                ParamType::Float
            }
            Self::AttributeResilience => ParamType::Float,
            Self::AttributeRun => ParamType::Float,
            Self::AttributeStealthLevel => ParamType::Float,
            Self::AttributeStrength => ParamType::Float,
            Self::AttributeWisdom => ParamType::Float,
            Self::AutoLootRadius => ParamType::Float,
            Self::AvailableEdnaClones => ParamType::Any,
            Self::AvailableOutfits => ParamType::Any,
            Self::AwareDist => ParamType::Float,
            Self::AwareRange => ParamType::Float,
            Self::BgLastWordZoneGuid => ParamType::Guid,
            Self::BgLastZoneGuid => ParamType::Guid,
            Self::BgLastZonePosition => ParamType::Vector3,
            Self::BgStatisticsString => ParamType::String,
            Self::BgTeam => ParamType::String,
            Self::Bling => ParamType::Int,
            Self::BlockedAbilityEffectTypes => ParamType::Int,
            Self::CarrierGuid => ParamType::Guid,
            Self::ChatIgnoreSet => ParamType::String,
            Self::ClanGuid => ParamType::Guid,
            Self::ClanHateList => ParamType::GuidSet,
            Self::ClanName => ParamType::String,
            Self::ClanPrivileges => ParamType::Int,
            Self::ClanRank => ParamType::Int,
            Self::ClanRatified => ParamType::Bool,
            Self::ClassData => ParamType::Any,
            Self::ClassSkillCollection => ParamType::Any,
            Self::ClientActionTracker => ParamType::JsonValue,
            Self::ClientReady => ParamType::Bool,
            Self::Cloaked => ParamType::Float,
            Self::ClusterGuid => ParamType::String,
            Self::CollisionExtent => ParamType::Vector3,
            Self::CombatStyle => ParamType::Int,
            Self::ContentClass => ParamType::String,
            Self::CooldownManager => ParamType::Any,
            Self::CooldownPassed => ParamType::Bool,
            Self::CooldownTrackers => ParamType::JsonValue,
            Self::CurrentAbilityBarReferences => ParamType::Any,
            Self::CurrentClassSkills => ParamType::Any,
            Self::CurrentMyLandAddress => ParamType::String,
            Self::CurrentOutfitSlot => ParamType::Int,
            Self::CurrentSkin => ParamType::String,
            Self::CurrentTickedItemSlot => ParamType::Int,
            Self::CustomizationBrowAngle => ParamType::Float,
            Self::CustomizationBustSize => ParamType::Float,
            Self::CustomizationCheek => ParamType::Float,
            Self::CustomizationCheekBone => ParamType::Float,
            Self::CustomizationChinPortude => ParamType::Float,
            Self::CustomizationEarElf => ParamType::Float,
            Self::CustomizationEarSize => ParamType::Float,
            Self::CustomizationEyeBrowPos => ParamType::Float,
            Self::CustomizationEyePos => ParamType::Float,
            Self::CustomizationEyePosSpacing => ParamType::Float,
            Self::CustomizationEyeSizeLength => ParamType::Float,
            Self::CustomizationEyeSizeWidth => ParamType::Float,
            Self::CustomizationEyesPretty => ParamType::Float,
            Self::CustomizationFat => ParamType::Float,
            Self::CustomizationGender => ParamType::Float,
            Self::CustomizationHeight => ParamType::Float,
            Self::CustomizationJawChubby => ParamType::Float,
            Self::CustomizationMouthExpression => ParamType::Float,
            Self::CustomizationMouthLowerLipThic => ParamType::Float,
            Self::CustomizationMouthPos => ParamType::Float,
            Self::CustomizationMouthUpperLipThic => ParamType::Float,
            Self::CustomizationMouthWidth => ParamType::Float,
            Self::CustomizationMuscular => ParamType::Float,
            Self::CustomizationNosePortude => ParamType::Float,
            Self::CustomizationNosePosLength => ParamType::Float,
            Self::CustomizationNosePosWidth => ParamType::Float,
            Self::CustomizationSkinny => ParamType::Float,
            Self::DamageOutputMod => ParamType::Float,
            Self::DamageReceivedMod => ParamType::Float,
            Self::DeathInfo => ParamType::Any,
            Self::DefaultItemsContentGuid => ParamType::VectorInt,
            Self::DepositAmount => ParamType::Int,
            Self::DepositBankGuid => ParamType::Guid,
            Self::DepositHistory => ParamType::Any,
            Self::DepositLevel => ParamType::Int,
            Self::DungoneKillBoss => ParamType::VectorGuid,
            Self::EmoteSlots => ParamType::JsonValue,
            Self::EmoteUsed => ParamType::StringFloatPair,
            Self::EnemyId => ParamType::AvatarId,
            Self::Faction => ParamType::ContentRefList,
            Self::FactionStandings => ParamType::Any,
            Self::FirstTimeSpawn => ParamType::Bool,
            Self::FragmentSlots => ParamType::JsonValue,
            Self::FreedomProperties => ParamType::VectorInt,
            Self::Freq => ParamType::Int,
            Self::GameCash => ParamType::Int,
            Self::GenerateInterestList => ParamType::Bool,
            Self::GuideToAvatar => ParamType::AvatarId,
            Self::GuideToLocation => ParamType::Vector3,
            Self::HasAttributes => ParamType::Bool,
            Self::HeavySpecialSkillData => ParamType::Any,
            Self::HostIp => ParamType::String,
            Self::HpCur => ParamType::Int,
            Self::HpMax => ParamType::Int,
            Self::HpMin => ParamType::Int,
            Self::Icon => ParamType::String,
            Self::InGameSession => ParamType::Bool,
            Self::InInstancedBattle => ParamType::Bool,
            Self::InitialWorldTimeThisLevelThisSession => ParamType::Int64,
            Self::InitialWorldTimeThisSession => ParamType::Int64,
            Self::InMiniGame => ParamType::Bool,
            Self::InstanceZoneKey => ParamType::String,
            Self::InteractionRadius => ParamType::Float,
            Self::InteractRadius => ParamType::Float,
            Self::InventorySize => ParamType::Int,
            Self::IsAdmin => ParamType::Bool,
            Self::IsInCombat => ParamType::Bool,
            Self::IsInPvPZone => ParamType::Bool,
            Self::IsInsideInstanceZone => ParamType::Bool,
            Self::IsInSocial => ParamType::Bool,
            Self::IsOnline => ParamType::Bool,
            Self::IsUnAttackable => ParamType::Bool,
            Self::ItemSlotsVisible => ParamType::VectorInt,
            Self::JumpVelocity => ParamType::Float,
            Self::LastAttackPosition => ParamType::Vector3,
            Self::LastEquippedWeapon => ParamType::Guid,
            Self::LastKnownClanLandRadius => ParamType::Float,
            Self::LastLogoutTime => ParamType::Int64,
            Self::LastPortalUsed => ParamType::Any,
            Self::LastResetDailyQuest => ParamType::Int,
            Self::LastSkuSyncTime => ParamType::Int64,
            Self::LastVendorSyncTime => ParamType::Int64,
            Self::LoginCount => ParamType::Int,
            Self::LootItemGuid => ParamType::Guid,
            Self::LootItemType => ParamType::Int,
            Self::Lvl => ParamType::Int,
            Self::LvlHistory => ParamType::Any,
            Self::MaxLevelCap => ParamType::Int,
            Self::MetamorphItemList => ParamType::VectorGuid,
            Self::MinigameData => ParamType::JsonValue,
            Self::Mount => ParamType::AvatarId,
            Self::Mover => ParamType::AvatarId,
            Self::MoveSpeed => ParamType::Float,
            Self::MyLandData => ParamType::JsonValue,
            Self::MyQuestTrack => ParamType::VectorInt,
            Self::MyShopGuid => ParamType::VectorGuid,
            Self::MySteamDlc => ParamType::VectorInt,
            Self::MyUsedSteamDlc => ParamType::VectorInt,
            Self::NewItems => ParamType::VectorGuid,
            Self::OutfitNames => ParamType::VectorString,
            Self::OutfitSlots => ParamType::JsonValue,
            Self::OverrideFaction => ParamType::String,
            Self::PartyGuid => ParamType::Guid,
            Self::Pet => ParamType::AvatarId,
            Self::PhaseSelectionData => ParamType::String,
            Self::PlaycountMinigameBilliards => ParamType::Int,
            Self::PlayerLoading => ParamType::Bool,
            Self::PlayerNodeState => ParamType::Int,
            Self::PlayerUsedSteamDlc => ParamType::VectorInt,
            Self::PortalData => ParamType::JsonValue,
            Self::Pos => ParamType::Vector3,
            Self::Power => ParamType::Int,
            Self::PvpEnabled => ParamType::Bool,
            Self::PvpEnabledInMyLandServerSetting => ParamType::Bool,
            Self::PvpEnabledServerSetting => ParamType::Bool,
            Self::PvpEnableDuration => ParamType::Float,
            Self::PvpFlag => ParamType::Int,
            Self::PvpRank => ParamType::Int,
            Self::PvpTimer => ParamType::Float,
            Self::PvpXp => ParamType::Int,
            Self::QuickUseBar => ParamType::VectorGuid,
            Self::Race => ParamType::Int,
            Self::RankingEdnamobsTotal => ParamType::Float,
            Self::RankingGearTotal => ParamType::Float,
            Self::RankingKillsPve => ParamType::Float,
            Self::RankingKillsPvp => ParamType::Float,
            Self::RankingMypadRooms => ParamType::Float,
            Self::RankingSomaAdd => ParamType::VectorFloat,
            Self::RankingSomaTotal => ParamType::VectorFloat,
            Self::RankingTotal => ParamType::Float,
            Self::RecentlyKilledInPvP => ParamType::Bool,
            Self::ReferenceList => ParamType::JsonValue,
            Self::RelativePosToCarrier => ParamType::Vector3,
            Self::RequestTeleportPos => ParamType::Vector3,
            Self::ResetDailyQuestList => ParamType::VectorInt,
            Self::Rot => ParamType::Vector3,
            Self::ScoreMinigameBilliards => ParamType::Float,
            Self::SelfRadius => ParamType::Float,
            Self::SheathedModeActive => ParamType::Bool,
            Self::SignClanCharterItem => ParamType::Guid,
            Self::Size => ParamType::Float,
            Self::SomaCarried => ParamType::VectorInt,
            Self::SomaLootRate => ParamType::Float,
            Self::SpawnCinematicOverride => ParamType::String,
            Self::SpawnedOnAvatar => ParamType::AvatarId,
            Self::SpawnMode => ParamType::Int,
            Self::SpectateName => ParamType::String,
            Self::SpectatePartyGuid => ParamType::Guid,
            Self::StatAnyDmgReduction => ParamType::Float,
            Self::StatAoEMaxSubTargets => ParamType::Float,
            Self::StatAoESubTargetsDamageMod => ParamType::Float,
            Self::StatArmorRating => ParamType::Float,
            Self::StatArmorReduction => ParamType::Float,
            Self::StatAttackPower => ParamType::Float,
            Self::StatAttackPowerBonus => ParamType::Float,
            Self::StatAttackPowerRating => ParamType::Float,
            Self::StatAttackRangePhysAdd => ParamType::Float,
            Self::StatAttackRating => ParamType::Float,
            Self::StatBendChance => ParamType::Float,
            Self::StatBendRating => ParamType::Float,
            Self::StatBlockChance => ParamType::Float,
            Self::StatBlockedDamageMod => ParamType::Float,
            Self::StatBlockRating => ParamType::Float,
            Self::StatCritChance => ParamType::Float,
            Self::StatCritDmgRating => ParamType::Float,
            Self::StatCriticalChanceReduction => ParamType::Float,
            Self::StatCriticalDamageMod => ParamType::Float,
            Self::StatCriticalDamageModBonus => ParamType::Float,
            Self::StatCritRating => ParamType::Float,
            Self::StatDamagePercPerMeterMod => ParamType::Float,
            Self::StatDefencePowerPhys => ParamType::Float,
            Self::StatDefenceRatingPhys => ParamType::Float,
            Self::StatDodgeChance => ParamType::Float,
            Self::StatDodgeRating => ParamType::Float,
            Self::StatEnergyCurrentH1 => ParamType::Float,
            Self::StatEnergyCurrentH2 => ParamType::Float,
            Self::StatEnergyCurrentH3 => ParamType::Float,
            Self::StatEnergyCurrentS1 => ParamType::Float,
            Self::StatEnergyCurrentS2 => ParamType::Float,
            Self::StatEnergyCurrentS3 => ParamType::Float,
            Self::StatEvadeChance => ParamType::Float,
            Self::StatEvadeRating => ParamType::Float,
            Self::StatExtraHealthRegen => ParamType::Float,
            Self::StatFinalDamageMod => ParamType::Float,
            Self::StatFinalHealingMod => ParamType::Float,
            Self::StatFreeFallDistanceMod => ParamType::Float,
            Self::StatHasteClassSkills => ParamType::Float,
            Self::StatHastePhysNormal => ParamType::Float,
            Self::StatHealingReceivedMod => ParamType::Float,
            Self::StatHeavyBonus => ParamType::Float,
            Self::StatHeavyEnergyPerHit => ParamType::Float,
            Self::StatHeavyRating => ParamType::Float,
            Self::StatHitChance => ParamType::Float,
            Self::StatHitRating => ParamType::Float,
            Self::StatInitialThreatMod => ParamType::Float,
            Self::StatParryChance => ParamType::Float,
            Self::StatParryRating => ParamType::Float,
            Self::StatPeneBonus => ParamType::Float,
            Self::StatPeneRating => ParamType::Float,
            Self::StatReflectChance => ParamType::Float,
            Self::StatReflectRating => ParamType::Float,
            Self::StatSpecialBonus => ParamType::Float,
            Self::StatSpecialEnergyPerHit => ParamType::Float,
            Self::StatSpecialRating => ParamType::Float,
            Self::StatStamina => ParamType::Float,
            Self::StatTcMax => ParamType::Float,
            Self::StatThreatMod => ParamType::Float,
            Self::StatWeaponDps => ParamType::Float,
            Self::StatWepMaxDmg => ParamType::Float,
            Self::StatWepMinDmg => ParamType::Float,
            Self::StatXpMod => ParamType::Float,
            Self::StickyTargets => ParamType::Any,
            Self::Tags => ParamType::String,
            Self::Target => ParamType::AvatarId,
            Self::TeamId => ParamType::Int,
            Self::TimePlayedBeforeThisSession => ParamType::Int64,
            Self::TimePlayedThisLevelBeforeThisSession => ParamType::Int64,
            Self::TutorialMode => ParamType::Bool,
            Self::Ue3ClassId => ParamType::String,
            Self::UiHintsAvailable => ParamType::Bool,
            Self::UnassignPortals => ParamType::Any,
            Self::UnLockedInstances => ParamType::VectorString,
            Self::UnLockedPortals => ParamType::VectorInt64,
            Self::UnlockedUiWindows => ParamType::VectorInt,
            Self::VisibleItemInfo => ParamType::VectorInt,
            Self::Weapon => ParamType::GuidPair,
            Self::WorldMapGuid => ParamType::String,
            Self::Xp => ParamType::Int,
            Self::XpForNextLevel => ParamType::Int,
            Self::XpTotal => ParamType::Int,
            Self::Zone => ParamType::String,
            Self::ZoneGuid => ParamType::Guid,
        }
    }
    fn default(&self) -> &'static Value {
        static ABILITIES: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static ACCOUNT_BANK_ID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ACCOUNT_BANK_SIZE: Value = Value::Int(0i32);
        static ACCOUNT_ID: Value = Value::Int(0i32);
        static ACCOUNT_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ACTION_0: Lazy<Value> = Lazy::new(|| Value::StringFloatPair((
            String::default(),
            0.0,
        )));
        static ACTION_0_DURATION: Value = Value::Float(0f32);
        static ADDRESS_SLOTS: Value = Value::Any(vec![]);
        static ALIVE: Value = Value::Bool(true);
        static ATTACKED_BY: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static ATTRIBUTE_ATTACK_POWER_PHYS: Value = Value::Float(0f32);
        static ATTRIBUTE_ATTACK_POWER_SPELL: Value = Value::Float(0f32);
        static ATTRIBUTE_CONSTITUTION: Value = Value::Float(32f32);
        static ATTRIBUTE_CRAFTING: Value = Value::Float(32f32);
        static ATTRIBUTE_CRITICAL_PHYS: Value = Value::Float(0f32);
        static ATTRIBUTE_CRITICAL_SPELL: Value = Value::Float(0f32);
        static ATTRIBUTE_DEGENERATE_LEVEL: Value = Value::Float(0f32);
        static ATTRIBUTE_DEXTERITY: Value = Value::Float(32f32);
        static ATTRIBUTE_DISGUISE: Value = Value::Float(32f32);
        static ATTRIBUTE_ENERGY: Value = Value::Float(32f32);
        static ATTRIBUTE_ENERGY_CURRENT: Value = Value::Float(0f32);
        static ATTRIBUTE_ENERGY_DECAY_STEALTHED_PERCENTAGE_NORMALIZED: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_ENERGY_EQUILIBRIUM_PERCENTAGE_NORMALIZED: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_ENERGY_GAIN_AUTO_ATTACK_HIT_ABSOLUTE: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_ENERGY_GAIN_WITH_TARGET_PER_SECOND_ABSOLUTE: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_ENERGY_INITIAL_PERCENTAGE_NORMALIZED: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_ENERGY_MAX: Value = Value::Float(1f32);
        static ATTRIBUTE_ENERGY_REGEN: Value = Value::Float(0f32);
        static ATTRIBUTE_FOCUS: Value = Value::Float(32f32);
        static ATTRIBUTE_HASTE_PHYS: Value = Value::Float(0f32);
        static ATTRIBUTE_HASTE_SPELL: Value = Value::Float(0f32);
        static ATTRIBUTE_HEALTH: Value = Value::Float(0f32);
        static ATTRIBUTE_HEALTH_REGEN: Value = Value::Float(0f32);
        static ATTRIBUTE_HIT_RATING_PHYS: Value = Value::Float(0f32);
        static ATTRIBUTE_HIT_RATING_SPELL: Value = Value::Float(0f32);
        static ATTRIBUTE_IN_COMBAT_TO_EQUILIBRIUM_PER_SECOND_ABSOLUTE: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_IN_COMBAT_TO_EQUILIBRIUM_PER_SECOND_PERCENTAGE_NORMALIZED: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_INTUITION: Value = Value::Float(32f32);
        static ATTRIBUTE_ITEM_LEVEL: Value = Value::Float(0f32);
        static ATTRIBUTE_JUMP: Value = Value::Float(0f32);
        static ATTRIBUTE_MISS_RATING_PHYS: Value = Value::Float(0f32);
        static ATTRIBUTE_MISS_RATING_SPELL: Value = Value::Float(0f32);
        static ATTRIBUTE_MOVEMENT: Value = Value::Float(32f32);
        static ATTRIBUTE_OUT_OF_COMBAT_TO_EQUILIBRIUM_PER_SECOND_ABSOLUTE: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_OUT_OF_COMBAT_TO_EQUILIBRIUM_PER_SECOND_PERCENTAGE_NORMALIZED: Value = Value::Float(
            0f32,
        );
        static ATTRIBUTE_RESILIENCE: Value = Value::Float(0f32);
        static ATTRIBUTE_RUN: Value = Value::Float(0f32);
        static ATTRIBUTE_STEALTH_LEVEL: Value = Value::Float(0f32);
        static ATTRIBUTE_STRENGTH: Value = Value::Float(32f32);
        static ATTRIBUTE_WISDOM: Value = Value::Float(32f32);
        static AUTO_LOOT_RADIUS: Value = Value::Float(60f32);
        static AVAILABLE_EDNA_CLONES: Value = Value::Any(vec![]);
        static AVAILABLE_OUTFITS: Value = Value::Any(vec![]);
        static AWARE_DIST: Value = Value::Float(3900f32);
        static AWARE_RANGE: Value = Value::Float(5000f32);
        static BG_LAST_WORD_ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static BG_LAST_ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static BG_LAST_ZONE_POSITION: Value = Value::Vector3(
            Vec3::new(0f32, 0f32, 0f32),
        );
        static BG_STATISTICS_STRING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static BG_TEAM: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static BLING: Value = Value::Int(-1i32);
        static BLOCKED_ABILITY_EFFECT_TYPES: Value = Value::Int(0i32);
        static CARRIER_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static CHAT_IGNORE_SET: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CLAN_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static CLAN_HATE_LIST: Lazy<Value> = Lazy::new(|| Value::GuidSet(
            HashSet::new(),
        ));
        static CLAN_NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static CLAN_PRIVILEGES: Value = Value::Int(0i32);
        static CLAN_RANK: Value = Value::Int(0i32);
        static CLAN_RATIFIED: Value = Value::Bool(false);
        static CLASS_DATA: Value = Value::Any(vec![]);
        static CLASS_SKILL_COLLECTION: Value = Value::Any(vec![]);
        static CLIENT_ACTION_TRACKER: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static CLIENT_READY: Value = Value::Bool(false);
        static CLOAKED: Value = Value::Float(0f32);
        static CLUSTER_GUID: Lazy<Value> = Lazy::new(|| Value::String(
            "00000001-0000-0000-4000-000000000000".to_string(),
        ));
        static COLLISION_EXTENT: Value = Value::Vector3(Vec3::new(21f32, 21f32, 44f32));
        static COMBAT_STYLE: Value = Value::Int(6i32);
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static COOLDOWN_MANAGER: Value = Value::Any(vec![]);
        static COOLDOWN_PASSED: Value = Value::Bool(false);
        static COOLDOWN_TRACKERS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static CURRENT_ABILITY_BAR_REFERENCES: Value = Value::Any(vec![]);
        static CURRENT_CLASS_SKILLS: Value = Value::Any(vec![]);
        static CURRENT_MY_LAND_ADDRESS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CURRENT_OUTFIT_SLOT: Value = Value::Int(0i32);
        static CURRENT_SKIN: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CURRENT_TICKED_ITEM_SLOT: Value = Value::Int(0i32);
        static CUSTOMIZATION_BROW_ANGLE: Value = Value::Float(0f32);
        static CUSTOMIZATION_BUST_SIZE: Value = Value::Float(0.5f32);
        static CUSTOMIZATION_CHEEK: Value = Value::Float(0f32);
        static CUSTOMIZATION_CHEEK_BONE: Value = Value::Float(0f32);
        static CUSTOMIZATION_CHIN_PORTUDE: Value = Value::Float(0f32);
        static CUSTOMIZATION_EAR_ELF: Value = Value::Float(0f32);
        static CUSTOMIZATION_EAR_SIZE: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYE_BROW_POS: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYE_POS: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYE_POS_SPACING: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYE_SIZE_LENGTH: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYE_SIZE_WIDTH: Value = Value::Float(0f32);
        static CUSTOMIZATION_EYES_PRETTY: Value = Value::Float(0f32);
        static CUSTOMIZATION_FAT: Value = Value::Float(0f32);
        static CUSTOMIZATION_GENDER: Value = Value::Float(0f32);
        static CUSTOMIZATION_HEIGHT: Value = Value::Float(0.65f32);
        static CUSTOMIZATION_JAW_CHUBBY: Value = Value::Float(0f32);
        static CUSTOMIZATION_MOUTH_EXPRESSION: Value = Value::Float(0f32);
        static CUSTOMIZATION_MOUTH_LOWER_LIP_THIC: Value = Value::Float(0f32);
        static CUSTOMIZATION_MOUTH_POS: Value = Value::Float(0f32);
        static CUSTOMIZATION_MOUTH_UPPER_LIP_THIC: Value = Value::Float(0f32);
        static CUSTOMIZATION_MOUTH_WIDTH: Value = Value::Float(0f32);
        static CUSTOMIZATION_MUSCULAR: Value = Value::Float(1f32);
        static CUSTOMIZATION_NOSE_PORTUDE: Value = Value::Float(0f32);
        static CUSTOMIZATION_NOSE_POS_LENGTH: Value = Value::Float(0f32);
        static CUSTOMIZATION_NOSE_POS_WIDTH: Value = Value::Float(0f32);
        static CUSTOMIZATION_SKINNY: Value = Value::Float(0f32);
        static DAMAGE_OUTPUT_MOD: Value = Value::Float(1f32);
        static DAMAGE_RECEIVED_MOD: Value = Value::Float(1f32);
        static DEATH_INFO: Value = Value::Any(vec![]);
        static DEFAULT_ITEMS_CONTENT_GUID: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static DEPOSIT_AMOUNT: Value = Value::Int(0i32);
        static DEPOSIT_BANK_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DEPOSIT_HISTORY: Value = Value::Any(vec![]);
        static DEPOSIT_LEVEL: Value = Value::Int(0i32);
        static DUNGONE_KILL_BOSS: Value = Value::VectorGuid(vec![]);
        static EMOTE_SLOTS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"MaxSlots\":15,\"SlotLimit\":50,\"Slots\":[\"\"],\"Shortcuts\":[\"Agree\",\"Angry\",\"Applause\",\"Chicken\",\"Crying\",\"Disagree\",\"Facepalm\",\"Goodbye\",\"Greeting\",\"Help\",\"Kiss\",\"Laugh\",\"Rude\",\"Yawn\"]}",
                )
                .unwrap(),
        ));
        static EMOTE_USED: Lazy<Value> = Lazy::new(|| Value::StringFloatPair((
            String::default(),
            0.0,
        )));
        static ENEMY_ID: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static FACTION: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            "[151:be55863a-03a0-4f2a-807c-b794e84f537c]".parse().unwrap_or_default(),
        ));
        static FACTION_STANDINGS: Value = Value::Any(vec![]);
        static FIRST_TIME_SPAWN: Value = Value::Bool(true);
        static FRAGMENT_SLOTS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{\"MaxSlots\":99,\"SlotLimit\":99,\"Slots\":{}}")
                .unwrap(),
        ));
        static FREEDOM_PROPERTIES: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static FREQ: Value = Value::Int(0i32);
        static GAME_CASH: Value = Value::Int(-1i32);
        static GENERATE_INTEREST_LIST: Value = Value::Bool(true);
        static GUIDE_TO_AVATAR: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static GUIDE_TO_LOCATION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static HAS_ATTRIBUTES: Value = Value::Bool(true);
        static HEAVY_SPECIAL_SKILL_DATA: Value = Value::Any(vec![]);
        static HOST_IP: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static HP_CUR: Value = Value::Int(0i32);
        static HP_MAX: Value = Value::Int(1000i32);
        static HP_MIN: Value = Value::Int(0i32);
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(
            "UI_Common.Textures.PlaceHolderIcon".to_string(),
        ));
        static IN_GAME_SESSION: Value = Value::Bool(false);
        static IN_INSTANCED_BATTLE: Value = Value::Bool(false);
        static INITIAL_WORLD_TIME_THIS_LEVEL_THIS_SESSION: Value = Value::Int64(0i64);
        static INITIAL_WORLD_TIME_THIS_SESSION: Value = Value::Int64(0i64);
        static IN_MINI_GAME: Value = Value::Bool(false);
        static INSTANCE_ZONE_KEY: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static INTERACTION_RADIUS: Value = Value::Float(150f32);
        static INTERACT_RADIUS: Value = Value::Float(-1f32);
        static INVENTORY_SIZE: Value = Value::Int(30i32);
        static IS_ADMIN: Value = Value::Bool(false);
        static IS_IN_COMBAT: Value = Value::Bool(false);
        static IS_IN_PV_P_ZONE: Value = Value::Bool(false);
        static IS_INSIDE_INSTANCE_ZONE: Value = Value::Bool(false);
        static IS_IN_SOCIAL: Value = Value::Bool(false);
        static IS_ONLINE: Value = Value::Bool(false);
        static IS_UN_ATTACKABLE: Value = Value::Bool(false);
        static ITEM_SLOTS_VISIBLE: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static JUMP_VELOCITY: Value = Value::Float(310f32);
        static LAST_ATTACK_POSITION: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static LAST_EQUIPPED_WEAPON: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static LAST_KNOWN_CLAN_LAND_RADIUS: Value = Value::Float(0f32);
        static LAST_LOGOUT_TIME: Value = Value::Int64(0i64);
        static LAST_PORTAL_USED: Value = Value::Any(vec![]);
        static LAST_RESET_DAILY_QUEST: Value = Value::Int(0i32);
        static LAST_SKU_SYNC_TIME: Value = Value::Int64(0i64);
        static LAST_VENDOR_SYNC_TIME: Value = Value::Int64(0i64);
        static LOGIN_COUNT: Value = Value::Int(0i32);
        static LOOT_ITEM_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static LOOT_ITEM_TYPE: Value = Value::Int(0i32);
        static LVL: Value = Value::Int(1i32);
        static LVL_HISTORY: Value = Value::Any(vec![]);
        static MAX_LEVEL_CAP: Value = Value::Int(0i32);
        static METAMORPH_ITEM_LIST: Value = Value::VectorGuid(vec![]);
        static MINIGAME_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "[{\"gameName\":\"Dogfight\",\"Vehicle\":\"vehiclePlane\"},{\"gameName\":\"Roswell\",\"Vehicle\":\"vehicleUFO\"},{\"gameName\":\"Planetarium\",\"Vehicle\":\"vehicleAssaultGunboat\"},{\"gameName\":\"Racing\",\"Vehicle\":\"vehicleRacingMonsterTruck\"},{\"gameName\":\"Balloona\",\"Vehicle\":\"vehicleBalloon\"}]",
                )
                .unwrap(),
        ));
        static MOUNT: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static MOVER: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static MOVE_SPEED: Value = Value::Float(192f32);
        static MY_LAND_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{\"MaxSlot\":3}").unwrap(),
        ));
        static MY_QUEST_TRACK: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static MY_SHOP_GUID: Value = Value::VectorGuid(vec![]);
        static MY_STEAM_DLC: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static MY_USED_STEAM_DLC: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static NEW_ITEMS: Value = Value::VectorGuid(vec![]);
        static OUTFIT_NAMES: Lazy<Value> = Lazy::new(|| Value::VectorString(vec![]));
        static OUTFIT_SLOTS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"maxSlots\":5,\"Outfits\":{\"Primary\":{},\"Secondary\":{}}}",
                )
                .unwrap(),
        ));
        static OVERRIDE_FACTION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PARTY_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static PET: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static PHASE_SELECTION_DATA: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PLAYCOUNT_MINIGAME_BILLIARDS: Value = Value::Int(0i32);
        static PLAYER_LOADING: Value = Value::Bool(true);
        static PLAYER_NODE_STATE: Value = Value::Int(0i32);
        static PLAYER_USED_STEAM_DLC: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static PORTAL_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[]").unwrap(),
        ));
        static POS: Value = Value::Vector3Uts((0, Vec3::new(0f32, 0f32, 0f32)));
        static POWER: Value = Value::Int(0i32);
        static PVP_ENABLED: Value = Value::Bool(false);
        static PVP_ENABLED_IN_MY_LAND_SERVER_SETTING: Value = Value::Bool(true);
        static PVP_ENABLED_SERVER_SETTING: Value = Value::Bool(false);
        static PVP_ENABLE_DURATION: Value = Value::Float(0f32);
        static PVP_FLAG: Value = Value::Int(0i32);
        static PVP_RANK: Value = Value::Int(0i32);
        static PVP_TIMER: Value = Value::Float(0f32);
        static PVP_XP: Value = Value::Int(0i32);
        static QUICK_USE_BAR: Value = Value::VectorGuid(vec![]);
        static RACE: Value = Value::Int(0i32);
        static RANKING_EDNAMOBS_TOTAL: Value = Value::Float(0f32);
        static RANKING_GEAR_TOTAL: Value = Value::Float(0f32);
        static RANKING_KILLS_PVE: Value = Value::Float(0f32);
        static RANKING_KILLS_PVP: Value = Value::Float(0f32);
        static RANKING_MYPAD_ROOMS: Value = Value::Float(0f32);
        static RANKING_SOMA_ADD: Value = Value::VectorFloat(vec![]);
        static RANKING_SOMA_TOTAL: Value = Value::VectorFloat(vec![]);
        static RANKING_TOTAL: Value = Value::Float(0f32);
        static RECENTLY_KILLED_IN_PV_P: Value = Value::Bool(false);
        static REFERENCE_LIST: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str(
                    "{\"SlotLimit\":20,\"Slots\":[{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"},{\"type\":0,\"data\":\"\"}]}",
                )
                .unwrap(),
        ));
        static RELATIVE_POS_TO_CARRIER: Value = Value::Vector3(
            Vec3::new(0f32, 0f32, 0f32),
        );
        static REQUEST_TELEPORT_POS: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static RESET_DAILY_QUEST_LIST: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static ROT: Value = Value::Vector3(Vec3::new(0f32, 0f32, 0f32));
        static SCORE_MINIGAME_BILLIARDS: Value = Value::Float(1200f32);
        static SELF_RADIUS: Value = Value::Float(20f32);
        static SHEATHED_MODE_ACTIVE: Value = Value::Bool(false);
        static SIGN_CLAN_CHARTER_ITEM: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static SIZE: Value = Value::Float(1f32);
        static SOMA_CARRIED: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
        ));
        static SOMA_LOOT_RATE: Value = Value::Float(20f32);
        static SPAWN_CINEMATIC_OVERRIDE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SPAWNED_ON_AVATAR: Value = Value::AvatarId(AvatarId::from_u64(0));
        static SPAWN_MODE: Value = Value::Int(2i32);
        static SPECTATE_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SPECTATE_PARTY_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static STAT_ANY_DMG_REDUCTION: Value = Value::Float(0f32);
        static STAT_AO_E_MAX_SUB_TARGETS: Value = Value::Float(0f32);
        static STAT_AO_E_SUB_TARGETS_DAMAGE_MOD: Value = Value::Float(0f32);
        static STAT_ARMOR_RATING: Value = Value::Float(0f32);
        static STAT_ARMOR_REDUCTION: Value = Value::Float(0f32);
        static STAT_ATTACK_POWER: Value = Value::Float(0f32);
        static STAT_ATTACK_POWER_BONUS: Value = Value::Float(0f32);
        static STAT_ATTACK_POWER_RATING: Value = Value::Float(0f32);
        static STAT_ATTACK_RANGE_PHYS_ADD: Value = Value::Float(0f32);
        static STAT_ATTACK_RATING: Value = Value::Float(0f32);
        static STAT_BEND_CHANCE: Value = Value::Float(0f32);
        static STAT_BEND_RATING: Value = Value::Float(0f32);
        static STAT_BLOCK_CHANCE: Value = Value::Float(0f32);
        static STAT_BLOCKED_DAMAGE_MOD: Value = Value::Float(0f32);
        static STAT_BLOCK_RATING: Value = Value::Float(0f32);
        static STAT_CRIT_CHANCE: Value = Value::Float(0f32);
        static STAT_CRIT_DMG_RATING: Value = Value::Float(0f32);
        static STAT_CRITICAL_CHANCE_REDUCTION: Value = Value::Float(0f32);
        static STAT_CRITICAL_DAMAGE_MOD: Value = Value::Float(0f32);
        static STAT_CRITICAL_DAMAGE_MOD_BONUS: Value = Value::Float(0f32);
        static STAT_CRIT_RATING: Value = Value::Float(0f32);
        static STAT_DAMAGE_PERC_PER_METER_MOD: Value = Value::Float(0f32);
        static STAT_DEFENCE_POWER_PHYS: Value = Value::Float(0f32);
        static STAT_DEFENCE_RATING_PHYS: Value = Value::Float(0f32);
        static STAT_DODGE_CHANCE: Value = Value::Float(0f32);
        static STAT_DODGE_RATING: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_H_1: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_H_2: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_H_3: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_S_1: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_S_2: Value = Value::Float(0f32);
        static STAT_ENERGY_CURRENT_S_3: Value = Value::Float(0f32);
        static STAT_EVADE_CHANCE: Value = Value::Float(0f32);
        static STAT_EVADE_RATING: Value = Value::Float(0f32);
        static STAT_EXTRA_HEALTH_REGEN: Value = Value::Float(0f32);
        static STAT_FINAL_DAMAGE_MOD: Value = Value::Float(0f32);
        static STAT_FINAL_HEALING_MOD: Value = Value::Float(0f32);
        static STAT_FREE_FALL_DISTANCE_MOD: Value = Value::Float(0f32);
        static STAT_HASTE_CLASS_SKILLS: Value = Value::Float(0f32);
        static STAT_HASTE_PHYS_NORMAL: Value = Value::Float(0f32);
        static STAT_HEALING_RECEIVED_MOD: Value = Value::Float(0f32);
        static STAT_HEAVY_BONUS: Value = Value::Float(0f32);
        static STAT_HEAVY_ENERGY_PER_HIT: Value = Value::Float(0f32);
        static STAT_HEAVY_RATING: Value = Value::Float(0f32);
        static STAT_HIT_CHANCE: Value = Value::Float(0f32);
        static STAT_HIT_RATING: Value = Value::Float(0f32);
        static STAT_INITIAL_THREAT_MOD: Value = Value::Float(1f32);
        static STAT_PARRY_CHANCE: Value = Value::Float(0f32);
        static STAT_PARRY_RATING: Value = Value::Float(0f32);
        static STAT_PENE_BONUS: Value = Value::Float(0f32);
        static STAT_PENE_RATING: Value = Value::Float(0f32);
        static STAT_REFLECT_CHANCE: Value = Value::Float(0f32);
        static STAT_REFLECT_RATING: Value = Value::Float(0f32);
        static STAT_SPECIAL_BONUS: Value = Value::Float(0f32);
        static STAT_SPECIAL_ENERGY_PER_HIT: Value = Value::Float(0f32);
        static STAT_SPECIAL_RATING: Value = Value::Float(0f32);
        static STAT_STAMINA: Value = Value::Float(0f32);
        static STAT_TC_MAX: Value = Value::Float(0f32);
        static STAT_THREAT_MOD: Value = Value::Float(1f32);
        static STAT_WEAPON_DPS: Value = Value::Float(0f32);
        static STAT_WEP_MAX_DMG: Value = Value::Float(0f32);
        static STAT_WEP_MIN_DMG: Value = Value::Float(0f32);
        static STAT_XP_MOD: Value = Value::Float(0f32);
        static STICKY_TARGETS: Value = Value::Any(vec![]);
        static TAGS: Lazy<Value> = Lazy::new(|| Value::String("Player".to_string()));
        static TARGET: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static TEAM_ID: Value = Value::Int(0i32);
        static TIME_PLAYED_BEFORE_THIS_SESSION: Value = Value::Int64(0i64);
        static TIME_PLAYED_THIS_LEVEL_BEFORE_THIS_SESSION: Value = Value::Int64(0i64);
        static TUTORIAL_MODE: Value = Value::Bool(false);
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            "Engine.AtlasAvatar".to_string(),
        ));
        static UI_HINTS_AVAILABLE: Value = Value::Bool(false);
        static UNASSIGN_PORTALS: Value = Value::Any(vec![]);
        static UN_LOCKED_INSTANCES: Lazy<Value> = Lazy::new(|| Value::VectorString(
            vec![],
        ));
        static UN_LOCKED_PORTALS: Value = Value::VectorInt64(vec![]);
        static UNLOCKED_UI_WINDOWS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static VISIBLE_ITEM_INFO: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static WEAPON: Value = Value::GuidPair((UUID_NIL, UUID_NIL));
        static WORLD_MAP_GUID: Lazy<Value> = Lazy::new(|| Value::String(
            "00000000-0000-0000-0000-000000000000".to_string(),
        ));
        static XP: Value = Value::Int(0i32);
        static XP_FOR_NEXT_LEVEL: Value = Value::Int(100i32);
        static XP_TOTAL: Value = Value::Int(0i32);
        static ZONE: Lazy<Value> = Lazy::new(|| Value::String(
            "MeetingRoom".to_string(),
        ));
        static ZONE_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        match self {
            Self::Abilities => &ABILITIES,
            Self::AccountBankId => &ACCOUNT_BANK_ID,
            Self::AccountBankSize => &ACCOUNT_BANK_SIZE,
            Self::AccountId => &ACCOUNT_ID,
            Self::AccountName => &ACCOUNT_NAME,
            Self::Action0 => &ACTION_0,
            Self::Action0Duration => &ACTION_0_DURATION,
            Self::AddressSlots => &ADDRESS_SLOTS,
            Self::Alive => &ALIVE,
            Self::AttackedBy => &ATTACKED_BY,
            Self::AttributeAttackPowerPhys => &ATTRIBUTE_ATTACK_POWER_PHYS,
            Self::AttributeAttackPowerSpell => &ATTRIBUTE_ATTACK_POWER_SPELL,
            Self::AttributeConstitution => &ATTRIBUTE_CONSTITUTION,
            Self::AttributeCrafting => &ATTRIBUTE_CRAFTING,
            Self::AttributeCriticalPhys => &ATTRIBUTE_CRITICAL_PHYS,
            Self::AttributeCriticalSpell => &ATTRIBUTE_CRITICAL_SPELL,
            Self::AttributeDegenerateLevel => &ATTRIBUTE_DEGENERATE_LEVEL,
            Self::AttributeDexterity => &ATTRIBUTE_DEXTERITY,
            Self::AttributeDisguise => &ATTRIBUTE_DISGUISE,
            Self::AttributeEnergy => &ATTRIBUTE_ENERGY,
            Self::AttributeEnergyCurrent => &ATTRIBUTE_ENERGY_CURRENT,
            Self::AttributeEnergyDecayStealthedPercentageNormalized => {
                &ATTRIBUTE_ENERGY_DECAY_STEALTHED_PERCENTAGE_NORMALIZED
            }
            Self::AttributeEnergyEquilibriumPercentageNormalized => {
                &ATTRIBUTE_ENERGY_EQUILIBRIUM_PERCENTAGE_NORMALIZED
            }
            Self::AttributeEnergyGainAutoAttackHitAbsolute => {
                &ATTRIBUTE_ENERGY_GAIN_AUTO_ATTACK_HIT_ABSOLUTE
            }
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => {
                &ATTRIBUTE_ENERGY_GAIN_WITH_TARGET_PER_SECOND_ABSOLUTE
            }
            Self::AttributeEnergyInitialPercentageNormalized => {
                &ATTRIBUTE_ENERGY_INITIAL_PERCENTAGE_NORMALIZED
            }
            Self::AttributeEnergyMax => &ATTRIBUTE_ENERGY_MAX,
            Self::AttributeEnergyRegen => &ATTRIBUTE_ENERGY_REGEN,
            Self::AttributeFocus => &ATTRIBUTE_FOCUS,
            Self::AttributeHastePhys => &ATTRIBUTE_HASTE_PHYS,
            Self::AttributeHasteSpell => &ATTRIBUTE_HASTE_SPELL,
            Self::AttributeHealth => &ATTRIBUTE_HEALTH,
            Self::AttributeHealthRegen => &ATTRIBUTE_HEALTH_REGEN,
            Self::AttributeHitRatingPhys => &ATTRIBUTE_HIT_RATING_PHYS,
            Self::AttributeHitRatingSpell => &ATTRIBUTE_HIT_RATING_SPELL,
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => {
                &ATTRIBUTE_IN_COMBAT_TO_EQUILIBRIUM_PER_SECOND_ABSOLUTE
            }
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => {
                &ATTRIBUTE_IN_COMBAT_TO_EQUILIBRIUM_PER_SECOND_PERCENTAGE_NORMALIZED
            }
            Self::AttributeIntuition => &ATTRIBUTE_INTUITION,
            Self::AttributeItemLevel => &ATTRIBUTE_ITEM_LEVEL,
            Self::AttributeJump => &ATTRIBUTE_JUMP,
            Self::AttributeMissRatingPhys => &ATTRIBUTE_MISS_RATING_PHYS,
            Self::AttributeMissRatingSpell => &ATTRIBUTE_MISS_RATING_SPELL,
            Self::AttributeMovement => &ATTRIBUTE_MOVEMENT,
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => {
                &ATTRIBUTE_OUT_OF_COMBAT_TO_EQUILIBRIUM_PER_SECOND_ABSOLUTE
            }
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                &ATTRIBUTE_OUT_OF_COMBAT_TO_EQUILIBRIUM_PER_SECOND_PERCENTAGE_NORMALIZED
            }
            Self::AttributeResilience => &ATTRIBUTE_RESILIENCE,
            Self::AttributeRun => &ATTRIBUTE_RUN,
            Self::AttributeStealthLevel => &ATTRIBUTE_STEALTH_LEVEL,
            Self::AttributeStrength => &ATTRIBUTE_STRENGTH,
            Self::AttributeWisdom => &ATTRIBUTE_WISDOM,
            Self::AutoLootRadius => &AUTO_LOOT_RADIUS,
            Self::AvailableEdnaClones => &AVAILABLE_EDNA_CLONES,
            Self::AvailableOutfits => &AVAILABLE_OUTFITS,
            Self::AwareDist => &AWARE_DIST,
            Self::AwareRange => &AWARE_RANGE,
            Self::BgLastWordZoneGuid => &BG_LAST_WORD_ZONE_GUID,
            Self::BgLastZoneGuid => &BG_LAST_ZONE_GUID,
            Self::BgLastZonePosition => &BG_LAST_ZONE_POSITION,
            Self::BgStatisticsString => &BG_STATISTICS_STRING,
            Self::BgTeam => &BG_TEAM,
            Self::Bling => &BLING,
            Self::BlockedAbilityEffectTypes => &BLOCKED_ABILITY_EFFECT_TYPES,
            Self::CarrierGuid => &CARRIER_GUID,
            Self::ChatIgnoreSet => &CHAT_IGNORE_SET,
            Self::ClanGuid => &CLAN_GUID,
            Self::ClanHateList => &CLAN_HATE_LIST,
            Self::ClanName => &CLAN_NAME,
            Self::ClanPrivileges => &CLAN_PRIVILEGES,
            Self::ClanRank => &CLAN_RANK,
            Self::ClanRatified => &CLAN_RATIFIED,
            Self::ClassData => &CLASS_DATA,
            Self::ClassSkillCollection => &CLASS_SKILL_COLLECTION,
            Self::ClientActionTracker => &CLIENT_ACTION_TRACKER,
            Self::ClientReady => &CLIENT_READY,
            Self::Cloaked => &CLOAKED,
            Self::ClusterGuid => &CLUSTER_GUID,
            Self::CollisionExtent => &COLLISION_EXTENT,
            Self::CombatStyle => &COMBAT_STYLE,
            Self::ContentClass => &CONTENT_CLASS,
            Self::CooldownManager => &COOLDOWN_MANAGER,
            Self::CooldownPassed => &COOLDOWN_PASSED,
            Self::CooldownTrackers => &COOLDOWN_TRACKERS,
            Self::CurrentAbilityBarReferences => &CURRENT_ABILITY_BAR_REFERENCES,
            Self::CurrentClassSkills => &CURRENT_CLASS_SKILLS,
            Self::CurrentMyLandAddress => &CURRENT_MY_LAND_ADDRESS,
            Self::CurrentOutfitSlot => &CURRENT_OUTFIT_SLOT,
            Self::CurrentSkin => &CURRENT_SKIN,
            Self::CurrentTickedItemSlot => &CURRENT_TICKED_ITEM_SLOT,
            Self::CustomizationBrowAngle => &CUSTOMIZATION_BROW_ANGLE,
            Self::CustomizationBustSize => &CUSTOMIZATION_BUST_SIZE,
            Self::CustomizationCheek => &CUSTOMIZATION_CHEEK,
            Self::CustomizationCheekBone => &CUSTOMIZATION_CHEEK_BONE,
            Self::CustomizationChinPortude => &CUSTOMIZATION_CHIN_PORTUDE,
            Self::CustomizationEarElf => &CUSTOMIZATION_EAR_ELF,
            Self::CustomizationEarSize => &CUSTOMIZATION_EAR_SIZE,
            Self::CustomizationEyeBrowPos => &CUSTOMIZATION_EYE_BROW_POS,
            Self::CustomizationEyePos => &CUSTOMIZATION_EYE_POS,
            Self::CustomizationEyePosSpacing => &CUSTOMIZATION_EYE_POS_SPACING,
            Self::CustomizationEyeSizeLength => &CUSTOMIZATION_EYE_SIZE_LENGTH,
            Self::CustomizationEyeSizeWidth => &CUSTOMIZATION_EYE_SIZE_WIDTH,
            Self::CustomizationEyesPretty => &CUSTOMIZATION_EYES_PRETTY,
            Self::CustomizationFat => &CUSTOMIZATION_FAT,
            Self::CustomizationGender => &CUSTOMIZATION_GENDER,
            Self::CustomizationHeight => &CUSTOMIZATION_HEIGHT,
            Self::CustomizationJawChubby => &CUSTOMIZATION_JAW_CHUBBY,
            Self::CustomizationMouthExpression => &CUSTOMIZATION_MOUTH_EXPRESSION,
            Self::CustomizationMouthLowerLipThic => &CUSTOMIZATION_MOUTH_LOWER_LIP_THIC,
            Self::CustomizationMouthPos => &CUSTOMIZATION_MOUTH_POS,
            Self::CustomizationMouthUpperLipThic => &CUSTOMIZATION_MOUTH_UPPER_LIP_THIC,
            Self::CustomizationMouthWidth => &CUSTOMIZATION_MOUTH_WIDTH,
            Self::CustomizationMuscular => &CUSTOMIZATION_MUSCULAR,
            Self::CustomizationNosePortude => &CUSTOMIZATION_NOSE_PORTUDE,
            Self::CustomizationNosePosLength => &CUSTOMIZATION_NOSE_POS_LENGTH,
            Self::CustomizationNosePosWidth => &CUSTOMIZATION_NOSE_POS_WIDTH,
            Self::CustomizationSkinny => &CUSTOMIZATION_SKINNY,
            Self::DamageOutputMod => &DAMAGE_OUTPUT_MOD,
            Self::DamageReceivedMod => &DAMAGE_RECEIVED_MOD,
            Self::DeathInfo => &DEATH_INFO,
            Self::DefaultItemsContentGuid => &DEFAULT_ITEMS_CONTENT_GUID,
            Self::DepositAmount => &DEPOSIT_AMOUNT,
            Self::DepositBankGuid => &DEPOSIT_BANK_GUID,
            Self::DepositHistory => &DEPOSIT_HISTORY,
            Self::DepositLevel => &DEPOSIT_LEVEL,
            Self::DungoneKillBoss => &DUNGONE_KILL_BOSS,
            Self::EmoteSlots => &EMOTE_SLOTS,
            Self::EmoteUsed => &EMOTE_USED,
            Self::EnemyId => &ENEMY_ID,
            Self::Faction => &FACTION,
            Self::FactionStandings => &FACTION_STANDINGS,
            Self::FirstTimeSpawn => &FIRST_TIME_SPAWN,
            Self::FragmentSlots => &FRAGMENT_SLOTS,
            Self::FreedomProperties => &FREEDOM_PROPERTIES,
            Self::Freq => &FREQ,
            Self::GameCash => &GAME_CASH,
            Self::GenerateInterestList => &GENERATE_INTEREST_LIST,
            Self::GuideToAvatar => &GUIDE_TO_AVATAR,
            Self::GuideToLocation => &GUIDE_TO_LOCATION,
            Self::HasAttributes => &HAS_ATTRIBUTES,
            Self::HeavySpecialSkillData => &HEAVY_SPECIAL_SKILL_DATA,
            Self::HostIp => &HOST_IP,
            Self::HpCur => &HP_CUR,
            Self::HpMax => &HP_MAX,
            Self::HpMin => &HP_MIN,
            Self::Icon => &ICON,
            Self::InGameSession => &IN_GAME_SESSION,
            Self::InInstancedBattle => &IN_INSTANCED_BATTLE,
            Self::InitialWorldTimeThisLevelThisSession => {
                &INITIAL_WORLD_TIME_THIS_LEVEL_THIS_SESSION
            }
            Self::InitialWorldTimeThisSession => &INITIAL_WORLD_TIME_THIS_SESSION,
            Self::InMiniGame => &IN_MINI_GAME,
            Self::InstanceZoneKey => &INSTANCE_ZONE_KEY,
            Self::InteractionRadius => &INTERACTION_RADIUS,
            Self::InteractRadius => &INTERACT_RADIUS,
            Self::InventorySize => &INVENTORY_SIZE,
            Self::IsAdmin => &IS_ADMIN,
            Self::IsInCombat => &IS_IN_COMBAT,
            Self::IsInPvPZone => &IS_IN_PV_P_ZONE,
            Self::IsInsideInstanceZone => &IS_INSIDE_INSTANCE_ZONE,
            Self::IsInSocial => &IS_IN_SOCIAL,
            Self::IsOnline => &IS_ONLINE,
            Self::IsUnAttackable => &IS_UN_ATTACKABLE,
            Self::ItemSlotsVisible => &ITEM_SLOTS_VISIBLE,
            Self::JumpVelocity => &JUMP_VELOCITY,
            Self::LastAttackPosition => &LAST_ATTACK_POSITION,
            Self::LastEquippedWeapon => &LAST_EQUIPPED_WEAPON,
            Self::LastKnownClanLandRadius => &LAST_KNOWN_CLAN_LAND_RADIUS,
            Self::LastLogoutTime => &LAST_LOGOUT_TIME,
            Self::LastPortalUsed => &LAST_PORTAL_USED,
            Self::LastResetDailyQuest => &LAST_RESET_DAILY_QUEST,
            Self::LastSkuSyncTime => &LAST_SKU_SYNC_TIME,
            Self::LastVendorSyncTime => &LAST_VENDOR_SYNC_TIME,
            Self::LoginCount => &LOGIN_COUNT,
            Self::LootItemGuid => &LOOT_ITEM_GUID,
            Self::LootItemType => &LOOT_ITEM_TYPE,
            Self::Lvl => &LVL,
            Self::LvlHistory => &LVL_HISTORY,
            Self::MaxLevelCap => &MAX_LEVEL_CAP,
            Self::MetamorphItemList => &METAMORPH_ITEM_LIST,
            Self::MinigameData => &MINIGAME_DATA,
            Self::Mount => &MOUNT,
            Self::Mover => &MOVER,
            Self::MoveSpeed => &MOVE_SPEED,
            Self::MyLandData => &MY_LAND_DATA,
            Self::MyQuestTrack => &MY_QUEST_TRACK,
            Self::MyShopGuid => &MY_SHOP_GUID,
            Self::MySteamDlc => &MY_STEAM_DLC,
            Self::MyUsedSteamDlc => &MY_USED_STEAM_DLC,
            Self::NewItems => &NEW_ITEMS,
            Self::OutfitNames => &OUTFIT_NAMES,
            Self::OutfitSlots => &OUTFIT_SLOTS,
            Self::OverrideFaction => &OVERRIDE_FACTION,
            Self::PartyGuid => &PARTY_GUID,
            Self::Pet => &PET,
            Self::PhaseSelectionData => &PHASE_SELECTION_DATA,
            Self::PlaycountMinigameBilliards => &PLAYCOUNT_MINIGAME_BILLIARDS,
            Self::PlayerLoading => &PLAYER_LOADING,
            Self::PlayerNodeState => &PLAYER_NODE_STATE,
            Self::PlayerUsedSteamDlc => &PLAYER_USED_STEAM_DLC,
            Self::PortalData => &PORTAL_DATA,
            Self::Pos => &POS,
            Self::Power => &POWER,
            Self::PvpEnabled => &PVP_ENABLED,
            Self::PvpEnabledInMyLandServerSetting => {
                &PVP_ENABLED_IN_MY_LAND_SERVER_SETTING
            }
            Self::PvpEnabledServerSetting => &PVP_ENABLED_SERVER_SETTING,
            Self::PvpEnableDuration => &PVP_ENABLE_DURATION,
            Self::PvpFlag => &PVP_FLAG,
            Self::PvpRank => &PVP_RANK,
            Self::PvpTimer => &PVP_TIMER,
            Self::PvpXp => &PVP_XP,
            Self::QuickUseBar => &QUICK_USE_BAR,
            Self::Race => &RACE,
            Self::RankingEdnamobsTotal => &RANKING_EDNAMOBS_TOTAL,
            Self::RankingGearTotal => &RANKING_GEAR_TOTAL,
            Self::RankingKillsPve => &RANKING_KILLS_PVE,
            Self::RankingKillsPvp => &RANKING_KILLS_PVP,
            Self::RankingMypadRooms => &RANKING_MYPAD_ROOMS,
            Self::RankingSomaAdd => &RANKING_SOMA_ADD,
            Self::RankingSomaTotal => &RANKING_SOMA_TOTAL,
            Self::RankingTotal => &RANKING_TOTAL,
            Self::RecentlyKilledInPvP => &RECENTLY_KILLED_IN_PV_P,
            Self::ReferenceList => &REFERENCE_LIST,
            Self::RelativePosToCarrier => &RELATIVE_POS_TO_CARRIER,
            Self::RequestTeleportPos => &REQUEST_TELEPORT_POS,
            Self::ResetDailyQuestList => &RESET_DAILY_QUEST_LIST,
            Self::Rot => &ROT,
            Self::ScoreMinigameBilliards => &SCORE_MINIGAME_BILLIARDS,
            Self::SelfRadius => &SELF_RADIUS,
            Self::SheathedModeActive => &SHEATHED_MODE_ACTIVE,
            Self::SignClanCharterItem => &SIGN_CLAN_CHARTER_ITEM,
            Self::Size => &SIZE,
            Self::SomaCarried => &SOMA_CARRIED,
            Self::SomaLootRate => &SOMA_LOOT_RATE,
            Self::SpawnCinematicOverride => &SPAWN_CINEMATIC_OVERRIDE,
            Self::SpawnedOnAvatar => &SPAWNED_ON_AVATAR,
            Self::SpawnMode => &SPAWN_MODE,
            Self::SpectateName => &SPECTATE_NAME,
            Self::SpectatePartyGuid => &SPECTATE_PARTY_GUID,
            Self::StatAnyDmgReduction => &STAT_ANY_DMG_REDUCTION,
            Self::StatAoEMaxSubTargets => &STAT_AO_E_MAX_SUB_TARGETS,
            Self::StatAoESubTargetsDamageMod => &STAT_AO_E_SUB_TARGETS_DAMAGE_MOD,
            Self::StatArmorRating => &STAT_ARMOR_RATING,
            Self::StatArmorReduction => &STAT_ARMOR_REDUCTION,
            Self::StatAttackPower => &STAT_ATTACK_POWER,
            Self::StatAttackPowerBonus => &STAT_ATTACK_POWER_BONUS,
            Self::StatAttackPowerRating => &STAT_ATTACK_POWER_RATING,
            Self::StatAttackRangePhysAdd => &STAT_ATTACK_RANGE_PHYS_ADD,
            Self::StatAttackRating => &STAT_ATTACK_RATING,
            Self::StatBendChance => &STAT_BEND_CHANCE,
            Self::StatBendRating => &STAT_BEND_RATING,
            Self::StatBlockChance => &STAT_BLOCK_CHANCE,
            Self::StatBlockedDamageMod => &STAT_BLOCKED_DAMAGE_MOD,
            Self::StatBlockRating => &STAT_BLOCK_RATING,
            Self::StatCritChance => &STAT_CRIT_CHANCE,
            Self::StatCritDmgRating => &STAT_CRIT_DMG_RATING,
            Self::StatCriticalChanceReduction => &STAT_CRITICAL_CHANCE_REDUCTION,
            Self::StatCriticalDamageMod => &STAT_CRITICAL_DAMAGE_MOD,
            Self::StatCriticalDamageModBonus => &STAT_CRITICAL_DAMAGE_MOD_BONUS,
            Self::StatCritRating => &STAT_CRIT_RATING,
            Self::StatDamagePercPerMeterMod => &STAT_DAMAGE_PERC_PER_METER_MOD,
            Self::StatDefencePowerPhys => &STAT_DEFENCE_POWER_PHYS,
            Self::StatDefenceRatingPhys => &STAT_DEFENCE_RATING_PHYS,
            Self::StatDodgeChance => &STAT_DODGE_CHANCE,
            Self::StatDodgeRating => &STAT_DODGE_RATING,
            Self::StatEnergyCurrentH1 => &STAT_ENERGY_CURRENT_H_1,
            Self::StatEnergyCurrentH2 => &STAT_ENERGY_CURRENT_H_2,
            Self::StatEnergyCurrentH3 => &STAT_ENERGY_CURRENT_H_3,
            Self::StatEnergyCurrentS1 => &STAT_ENERGY_CURRENT_S_1,
            Self::StatEnergyCurrentS2 => &STAT_ENERGY_CURRENT_S_2,
            Self::StatEnergyCurrentS3 => &STAT_ENERGY_CURRENT_S_3,
            Self::StatEvadeChance => &STAT_EVADE_CHANCE,
            Self::StatEvadeRating => &STAT_EVADE_RATING,
            Self::StatExtraHealthRegen => &STAT_EXTRA_HEALTH_REGEN,
            Self::StatFinalDamageMod => &STAT_FINAL_DAMAGE_MOD,
            Self::StatFinalHealingMod => &STAT_FINAL_HEALING_MOD,
            Self::StatFreeFallDistanceMod => &STAT_FREE_FALL_DISTANCE_MOD,
            Self::StatHasteClassSkills => &STAT_HASTE_CLASS_SKILLS,
            Self::StatHastePhysNormal => &STAT_HASTE_PHYS_NORMAL,
            Self::StatHealingReceivedMod => &STAT_HEALING_RECEIVED_MOD,
            Self::StatHeavyBonus => &STAT_HEAVY_BONUS,
            Self::StatHeavyEnergyPerHit => &STAT_HEAVY_ENERGY_PER_HIT,
            Self::StatHeavyRating => &STAT_HEAVY_RATING,
            Self::StatHitChance => &STAT_HIT_CHANCE,
            Self::StatHitRating => &STAT_HIT_RATING,
            Self::StatInitialThreatMod => &STAT_INITIAL_THREAT_MOD,
            Self::StatParryChance => &STAT_PARRY_CHANCE,
            Self::StatParryRating => &STAT_PARRY_RATING,
            Self::StatPeneBonus => &STAT_PENE_BONUS,
            Self::StatPeneRating => &STAT_PENE_RATING,
            Self::StatReflectChance => &STAT_REFLECT_CHANCE,
            Self::StatReflectRating => &STAT_REFLECT_RATING,
            Self::StatSpecialBonus => &STAT_SPECIAL_BONUS,
            Self::StatSpecialEnergyPerHit => &STAT_SPECIAL_ENERGY_PER_HIT,
            Self::StatSpecialRating => &STAT_SPECIAL_RATING,
            Self::StatStamina => &STAT_STAMINA,
            Self::StatTcMax => &STAT_TC_MAX,
            Self::StatThreatMod => &STAT_THREAT_MOD,
            Self::StatWeaponDps => &STAT_WEAPON_DPS,
            Self::StatWepMaxDmg => &STAT_WEP_MAX_DMG,
            Self::StatWepMinDmg => &STAT_WEP_MIN_DMG,
            Self::StatXpMod => &STAT_XP_MOD,
            Self::StickyTargets => &STICKY_TARGETS,
            Self::Tags => &TAGS,
            Self::Target => &TARGET,
            Self::TeamId => &TEAM_ID,
            Self::TimePlayedBeforeThisSession => &TIME_PLAYED_BEFORE_THIS_SESSION,
            Self::TimePlayedThisLevelBeforeThisSession => {
                &TIME_PLAYED_THIS_LEVEL_BEFORE_THIS_SESSION
            }
            Self::TutorialMode => &TUTORIAL_MODE,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::UiHintsAvailable => &UI_HINTS_AVAILABLE,
            Self::UnassignPortals => &UNASSIGN_PORTALS,
            Self::UnLockedInstances => &UN_LOCKED_INSTANCES,
            Self::UnLockedPortals => &UN_LOCKED_PORTALS,
            Self::UnlockedUiWindows => &UNLOCKED_UI_WINDOWS,
            Self::VisibleItemInfo => &VISIBLE_ITEM_INFO,
            Self::Weapon => &WEAPON,
            Self::WorldMapGuid => &WORLD_MAP_GUID,
            Self::Xp => &XP,
            Self::XpForNextLevel => &XP_FOR_NEXT_LEVEL,
            Self::XpTotal => &XP_TOTAL,
            Self::Zone => &ZONE,
            Self::ZoneGuid => &ZONE_GUID,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Abilities => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AccountBankId => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::AccountBankSize => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::AccountId => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AccountName => &[ParamFlag::ServerOwn, ParamFlag::ClientUnknown],
            Self::Action0 => &[ParamFlag::NodeOwn, ParamFlag::DupeSetOk],
            Self::Action0Duration => &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown],
            Self::AddressSlots => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                ]
            }
            Self::Alive => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttackedBy => &[ParamFlag::NodeOwn, ParamFlag::DupeSetOk],
            Self::AttributeAttackPowerPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeAttackPowerSpell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeConstitution => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeCrafting => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeCriticalPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeCriticalSpell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeDegenerateLevel => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::AttributeDexterity => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeDisguise => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeEnergy => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyCurrent => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttributeEnergyDecayStealthedPercentageNormalized => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyEquilibriumPercentageNormalized => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyGainAutoAttackHitAbsolute => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyGainWithTargetPerSecondAbsolute => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyInitialPercentageNormalized => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeEnergyRegen => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttributeFocus => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeHastePhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeHasteSpell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeHealth => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttributeHealthRegen => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttributeHitRatingPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeHitRatingSpell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeInCombatToEquilibriumPerSecondAbsolute => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeIntuition => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeItemLevel => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeJump => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttributeMissRatingPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeMissRatingSpell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeMovement => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeResilience => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AttributeRun => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AttributeStealthLevel => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeStrength => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AttributeWisdom => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::AutoLootRadius => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AvailableEdnaClones => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::AvailableOutfits => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AwareDist => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::AwareRange => &[ParamFlag::NodeOwn, ParamFlag::PerInstanceSetting],
            Self::BgLastWordZoneGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BgLastZoneGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BgLastZonePosition => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BgStatisticsString => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::BgTeam => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Bling => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::ClientPrivileged]
            }
            Self::BlockedAbilityEffectTypes => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::ClientUnknown]
            }
            Self::CarrierGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ChatIgnoreSet => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::ClanGuid => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanHateList => &[ParamFlag::ServerOwn, ParamFlag::ExcludeFromClient],
            Self::ClanName => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent, ParamFlag::DupeSetOk]
            }
            Self::ClanPrivileges => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanRank => &[ParamFlag::ServerOwn, ParamFlag::ExcludeFromClient],
            Self::ClanRatified => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClassData => &[ParamFlag::NodeOwn],
            Self::ClassSkillCollection => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ClientActionTracker => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ClientReady => {
                &[ParamFlag::ClientOwn, ParamFlag::NodeOwn, ParamFlag::ServerOwn]
            }
            Self::Cloaked => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ClusterGuid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
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
            Self::CombatStyle => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ContentClass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CooldownManager => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CooldownPassed => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::CooldownTrackers => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::CurrentAbilityBarReferences => {
                &[ParamFlag::ClientOwn, ParamFlag::NodeOwn, ParamFlag::ClientPrivileged]
            }
            Self::CurrentClassSkills => &[ParamFlag::NodeOwn],
            Self::CurrentMyLandAddress => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::CurrentOutfitSlot => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::CurrentSkin => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CurrentTickedItemSlot => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CustomizationBrowAngle => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationBustSize => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationCheek => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationCheekBone => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationChinPortude => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationEarElf => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationEarSize => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationEyeBrowPos => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationEyePos => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationEyePosSpacing => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationEyeSizeLength => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationEyeSizeWidth => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationEyesPretty => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationFat => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationGender => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationHeight => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationJawChubby => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationMouthExpression => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationMouthLowerLipThic => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationMouthPos => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationMouthUpperLipThic => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationMouthWidth => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationMuscular => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::CustomizationNosePortude => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationNosePosLength => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationNosePosWidth => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::CustomizationSkinny => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DamageOutputMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DamageReceivedMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DeathInfo => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DefaultItemsContentGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DepositAmount => &[ParamFlag::ServerOwn],
            Self::DepositBankGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::DepositHistory => &[ParamFlag::ServerOwn, ParamFlag::ClientPrivileged],
            Self::DepositLevel => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::DungoneKillBoss => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent, ParamFlag::DupeSetOk]
            }
            Self::EmoteSlots => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::EmoteUsed => &[ParamFlag::NodeOwn],
            Self::EnemyId => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Faction => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::FactionStandings => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::FirstTimeSpawn => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::FragmentSlots => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::FreedomProperties => &[ParamFlag::NodeOwn],
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameCash => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::GenerateInterestList => {
                &[ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::GuideToAvatar => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::GuideToLocation => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::HasAttributes => &[ParamFlag::Persistent],
            Self::HeavySpecialSkillData => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::HostIp => &[ParamFlag::ClientOwn, ParamFlag::ServerOwn],
            Self::HpCur => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::HpMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::HpMin => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Icon => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InGameSession => {
                &[ParamFlag::ClientOwn, ParamFlag::NodeOwn, ParamFlag::ServerOwn]
            }
            Self::InInstancedBattle => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::InitialWorldTimeThisLevelThisSession => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InitialWorldTimeThisSession => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InMiniGame => &[ParamFlag::NodeOwn],
            Self::InstanceZoneKey => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InteractionRadius => &[ParamFlag::Persistent],
            Self::InteractRadius => &[ParamFlag::NodeOwn, ParamFlag::Content],
            Self::InventorySize => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsAdmin => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::IsInCombat => &[ParamFlag::NodeOwn],
            Self::IsInPvPZone => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                ]
            }
            Self::IsInsideInstanceZone => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::IsInSocial => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Metric,
                ]
            }
            Self::IsOnline => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::IsUnAttackable => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::ItemSlotsVisible => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::JumpVelocity => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::LastAttackPosition => &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown],
            Self::LastEquippedWeapon => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::LastKnownClanLandRadius => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LastLogoutTime => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::LastPortalUsed => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::LastResetDailyQuest => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LastSkuSyncTime => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LastVendorSyncTime => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LoginCount => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LootItemGuid => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::LootItemType => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Lvl => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LvlHistory => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::MaxLevelCap => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::MetamorphItemList => &[ParamFlag::ClientOwn, ParamFlag::Persistent],
            Self::MinigameData => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::Mount => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Mover => {
                &[
                    ParamFlag::ClientOwn,
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::MoveSpeed => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::MyLandData => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::MyQuestTrack => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::MyShopGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::MySteamDlc => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::MyUsedSteamDlc => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::NewItems => &[ParamFlag::ClientOwn, ParamFlag::Persistent],
            Self::OutfitNames => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::OutfitSlots => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                    ParamFlag::Deprecated,
                ]
            }
            Self::OverrideFaction => &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown],
            Self::PartyGuid => &[ParamFlag::NodeOwn, ParamFlag::ServerOwn],
            Self::Pet => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::PhaseSelectionData => &[ParamFlag::NodeOwn],
            Self::PlaycountMinigameBilliards => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::PlayerLoading => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PlayerNodeState => &[ParamFlag::NodeOwn],
            Self::PlayerUsedSteamDlc => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PortalData => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Pos => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::Uts,
                ]
            }
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PvpEnabled => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Deprecated]
            }
            Self::PvpEnabledInMyLandServerSetting => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                ]
            }
            Self::PvpEnabledServerSetting => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                ]
            }
            Self::PvpEnableDuration => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PvpFlag => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PvpRank => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PvpTimer => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::PvpXp => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::QuickUseBar => &[ParamFlag::ClientOwn, ParamFlag::Persistent],
            Self::Race => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::RankingEdnamobsTotal => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingGearTotal => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingKillsPve => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingKillsPvp => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingMypadRooms => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingSomaAdd => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingSomaTotal => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RankingTotal => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RecentlyKilledInPvP => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientUnknown,
                    ParamFlag::Persistent,
                ]
            }
            Self::ReferenceList => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::RelativePosToCarrier => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::RequestTeleportPos => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ResetDailyQuestList => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Rot => {
                &[ParamFlag::ServerOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::ScoreMinigameBilliards => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientPrivileged, ParamFlag::Persistent]
            }
            Self::SelfRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::SheathedModeActive => &[ParamFlag::ClientOwn],
            Self::SignClanCharterItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientInit,
                    ParamFlag::Persistent,
                ]
            }
            Self::Size => {
                &[
                    ParamFlag::ClientOwn,
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                ]
            }
            Self::SomaCarried => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SomaLootRate => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpawnCinematicOverride => &[ParamFlag::ServerOwn],
            Self::SpawnedOnAvatar => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::SpawnMode => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::SpectateName => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::SpectatePartyGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::StatAnyDmgReduction => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAoEMaxSubTargets => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAoESubTargetsDamageMod => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::StatArmorRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatArmorReduction => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAttackPower => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAttackPowerBonus => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAttackPowerRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAttackRangePhysAdd => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatAttackRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatBendChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatBendRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatBlockChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatBlockedDamageMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatBlockRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCritChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCritDmgRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCriticalChanceReduction => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::StatCriticalDamageMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatCriticalDamageModBonus => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::StatCritRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatDamagePercPerMeterMod => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::StatDefencePowerPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatDefenceRatingPhys => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatDodgeChance => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatDodgeRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatEnergyCurrentH1 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEnergyCurrentH2 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEnergyCurrentH3 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEnergyCurrentS1 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEnergyCurrentS2 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEnergyCurrentS3 => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatEvadeChance => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatEvadeRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatExtraHealthRegen => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatFinalDamageMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatFinalHealingMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatFreeFallDistanceMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHasteClassSkills => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHastePhysNormal => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatHealingReceivedMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHeavyBonus => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHeavyEnergyPerHit => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHeavyRating => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::StatHitChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatHitRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatInitialThreatMod => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatParryChance => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatParryRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::StatPeneBonus => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatPeneRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatReflectChance => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatReflectRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatSpecialBonus => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatSpecialEnergyPerHit => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatSpecialRating => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatStamina => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatTcMax => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatThreatMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatWeaponDps => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatWepMaxDmg => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatWepMinDmg => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StatXpMod => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::StickyTargets => &[ParamFlag::NodeOwn],
            Self::Tags => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Target => &[ParamFlag::NodeOwn, ParamFlag::DupeSetOk],
            Self::TeamId => &[ParamFlag::NodeOwn],
            Self::TimePlayedBeforeThisSession => {
                &[
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TimePlayedThisLevelBeforeThisSession => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TutorialMode => {
                &[ParamFlag::ClientOwn, ParamFlag::NodeOwn, ParamFlag::Persistent]
            }
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UiHintsAvailable => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::UnassignPortals => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::UnLockedInstances => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::UnLockedPortals => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::UnlockedUiWindows => &[ParamFlag::ClientOwn, ParamFlag::Persistent],
            Self::VisibleItemInfo => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Weapon => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::EquipSlot]
            }
            Self::WorldMapGuid => {
                &[ParamFlag::ServerOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::Xp => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::XpForNextLevel => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::ClientPrivileged,
                    ParamFlag::Persistent,
                ]
            }
            Self::XpTotal => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::Zone => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ZoneGuid => {
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
        }
    }
}
impl FromStr for Player {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PLAYER_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Player {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            9350u16 => Ok(Self::Abilities),
            12268u16 => Ok(Self::AccountBankId),
            12269u16 => Ok(Self::AccountBankSize),
            10603u16 => Ok(Self::AccountId),
            10602u16 => Ok(Self::AccountName),
            2673u16 => Ok(Self::Action0),
            2674u16 => Ok(Self::Action0Duration),
            5904u16 => Ok(Self::AddressSlots),
            2675u16 => Ok(Self::Alive),
            2676u16 => Ok(Self::AttackedBy),
            6258u16 => Ok(Self::AttributeAttackPowerPhys),
            6256u16 => Ok(Self::AttributeAttackPowerSpell),
            4326u16 => Ok(Self::AttributeConstitution),
            4325u16 => Ok(Self::AttributeCrafting),
            6255u16 => Ok(Self::AttributeCriticalPhys),
            6254u16 => Ok(Self::AttributeCriticalSpell),
            6539u16 => Ok(Self::AttributeDegenerateLevel),
            4327u16 => Ok(Self::AttributeDexterity),
            4324u16 => Ok(Self::AttributeDisguise),
            4323u16 => Ok(Self::AttributeEnergy),
            4547u16 => Ok(Self::AttributeEnergyCurrent),
            4549u16 => Ok(Self::AttributeEnergyDecayStealthedPercentageNormalized),
            4556u16 => Ok(Self::AttributeEnergyEquilibriumPercentageNormalized),
            4550u16 => Ok(Self::AttributeEnergyGainAutoAttackHitAbsolute),
            4557u16 => Ok(Self::AttributeEnergyGainWithTargetPerSecondAbsolute),
            4555u16 => Ok(Self::AttributeEnergyInitialPercentageNormalized),
            4548u16 => Ok(Self::AttributeEnergyMax),
            6265u16 => Ok(Self::AttributeEnergyRegen),
            4322u16 => Ok(Self::AttributeFocus),
            6253u16 => Ok(Self::AttributeHastePhys),
            6252u16 => Ok(Self::AttributeHasteSpell),
            6267u16 => Ok(Self::AttributeHealth),
            6266u16 => Ok(Self::AttributeHealthRegen),
            6264u16 => Ok(Self::AttributeHitRatingPhys),
            6261u16 => Ok(Self::AttributeHitRatingSpell),
            4554u16 => Ok(Self::AttributeInCombatToEquilibriumPerSecondAbsolute),
            4553u16 => {
                Ok(Self::AttributeInCombatToEquilibriumPerSecondPercentageNormalized)
            }
            4321u16 => Ok(Self::AttributeIntuition),
            12143u16 => Ok(Self::AttributeItemLevel),
            6263u16 => Ok(Self::AttributeJump),
            6260u16 => Ok(Self::AttributeMissRatingPhys),
            6259u16 => Ok(Self::AttributeMissRatingSpell),
            4320u16 => Ok(Self::AttributeMovement),
            4552u16 => Ok(Self::AttributeOutOfCombatToEquilibriumPerSecondAbsolute),
            4551u16 => {
                Ok(Self::AttributeOutOfCombatToEquilibriumPerSecondPercentageNormalized)
            }
            6268u16 => Ok(Self::AttributeResilience),
            6262u16 => Ok(Self::AttributeRun),
            4559u16 => Ok(Self::AttributeStealthLevel),
            4319u16 => Ok(Self::AttributeStrength),
            4318u16 => Ok(Self::AttributeWisdom),
            11393u16 => Ok(Self::AutoLootRadius),
            10049u16 => Ok(Self::AvailableEdnaClones),
            10040u16 => Ok(Self::AvailableOutfits),
            2678u16 => Ok(Self::AwareDist),
            8295u16 => Ok(Self::AwareRange),
            12104u16 => Ok(Self::BgLastWordZoneGuid),
            12103u16 => Ok(Self::BgLastZoneGuid),
            12102u16 => Ok(Self::BgLastZonePosition),
            12100u16 => Ok(Self::BgStatisticsString),
            12101u16 => Ok(Self::BgTeam),
            6465u16 => Ok(Self::Bling),
            2627u16 => Ok(Self::BlockedAbilityEffectTypes),
            2619u16 => Ok(Self::CarrierGuid),
            2679u16 => Ok(Self::ChatIgnoreSet),
            8904u16 => Ok(Self::ClanGuid),
            8910u16 => Ok(Self::ClanHateList),
            8909u16 => Ok(Self::ClanName),
            10928u16 => Ok(Self::ClanPrivileges),
            8903u16 => Ok(Self::ClanRank),
            10929u16 => Ok(Self::ClanRatified),
            11080u16 => Ok(Self::ClassData),
            10518u16 => Ok(Self::ClassSkillCollection),
            4942u16 => Ok(Self::ClientActionTracker),
            2647u16 => Ok(Self::ClientReady),
            2668u16 => Ok(Self::Cloaked),
            2680u16 => Ok(Self::ClusterGuid),
            2681u16 => Ok(Self::CollisionExtent),
            2616u16 => Ok(Self::CombatStyle),
            2671u16 => Ok(Self::ContentClass),
            11327u16 => Ok(Self::CooldownManager),
            12291u16 => Ok(Self::CooldownPassed),
            4039u16 => Ok(Self::CooldownTrackers),
            10604u16 => Ok(Self::CurrentAbilityBarReferences),
            10605u16 => Ok(Self::CurrentClassSkills),
            6655u16 => Ok(Self::CurrentMyLandAddress),
            7265u16 => Ok(Self::CurrentOutfitSlot),
            2669u16 => Ok(Self::CurrentSkin),
            10097u16 => Ok(Self::CurrentTickedItemSlot),
            12322u16 => Ok(Self::CustomizationBrowAngle),
            8562u16 => Ok(Self::CustomizationBustSize),
            12304u16 => Ok(Self::CustomizationCheek),
            12305u16 => Ok(Self::CustomizationCheekBone),
            12303u16 => Ok(Self::CustomizationChinPortude),
            12306u16 => Ok(Self::CustomizationEarElf),
            12307u16 => Ok(Self::CustomizationEarSize),
            12321u16 => Ok(Self::CustomizationEyeBrowPos),
            12319u16 => Ok(Self::CustomizationEyePos),
            12320u16 => Ok(Self::CustomizationEyePosSpacing),
            12318u16 => Ok(Self::CustomizationEyeSizeLength),
            12317u16 => Ok(Self::CustomizationEyeSizeWidth),
            12316u16 => Ok(Self::CustomizationEyesPretty),
            4295u16 => Ok(Self::CustomizationFat),
            4291u16 => Ok(Self::CustomizationGender),
            4292u16 => Ok(Self::CustomizationHeight),
            12302u16 => Ok(Self::CustomizationJawChubby),
            12311u16 => Ok(Self::CustomizationMouthExpression),
            12313u16 => Ok(Self::CustomizationMouthLowerLipThic),
            12315u16 => Ok(Self::CustomizationMouthPos),
            12312u16 => Ok(Self::CustomizationMouthUpperLipThic),
            12314u16 => Ok(Self::CustomizationMouthWidth),
            4293u16 => Ok(Self::CustomizationMuscular),
            12308u16 => Ok(Self::CustomizationNosePortude),
            12310u16 => Ok(Self::CustomizationNosePosLength),
            12309u16 => Ok(Self::CustomizationNosePosWidth),
            4294u16 => Ok(Self::CustomizationSkinny),
            6008u16 => Ok(Self::DamageOutputMod),
            6007u16 => Ok(Self::DamageReceivedMod),
            11229u16 => Ok(Self::DeathInfo),
            6786u16 => Ok(Self::DefaultItemsContentGuid),
            9221u16 => Ok(Self::DepositAmount),
            11582u16 => Ok(Self::DepositBankGuid),
            9219u16 => Ok(Self::DepositHistory),
            9220u16 => Ok(Self::DepositLevel),
            12144u16 => Ok(Self::DungoneKillBoss),
            7041u16 => Ok(Self::EmoteSlots),
            8040u16 => Ok(Self::EmoteUsed),
            12293u16 => Ok(Self::EnemyId),
            2638u16 => Ok(Self::Faction),
            2639u16 => Ok(Self::FactionStandings),
            2650u16 => Ok(Self::FirstTimeSpawn),
            5998u16 => Ok(Self::FragmentSlots),
            11204u16 => Ok(Self::FreedomProperties),
            2715u16 => Ok(Self::Freq),
            6592u16 => Ok(Self::GameCash),
            2663u16 => Ok(Self::GenerateInterestList),
            2636u16 => Ok(Self::GuideToAvatar),
            2635u16 => Ok(Self::GuideToLocation),
            6375u16 => Ok(Self::HasAttributes),
            10996u16 => Ok(Self::HeavySpecialSkillData),
            2687u16 => Ok(Self::HostIp),
            2688u16 => Ok(Self::HpCur),
            2689u16 => Ok(Self::HpMax),
            2642u16 => Ok(Self::HpMin),
            4391u16 => Ok(Self::Icon),
            2691u16 => Ok(Self::InGameSession),
            5151u16 => Ok(Self::InInstancedBattle),
            10149u16 => Ok(Self::InitialWorldTimeThisLevelThisSession),
            10150u16 => Ok(Self::InitialWorldTimeThisSession),
            2631u16 => Ok(Self::InMiniGame),
            5610u16 => Ok(Self::InstanceZoneKey),
            7527u16 => Ok(Self::InteractionRadius),
            4191u16 => Ok(Self::InteractRadius),
            9881u16 => Ok(Self::InventorySize),
            12141u16 => Ok(Self::IsAdmin),
            2607u16 => Ok(Self::IsInCombat),
            9698u16 => Ok(Self::IsInPvPZone),
            5508u16 => Ok(Self::IsInsideInstanceZone),
            12527u16 => Ok(Self::IsInSocial),
            12324u16 => Ok(Self::IsOnline),
            7884u16 => Ok(Self::IsUnAttackable),
            12244u16 => Ok(Self::ItemSlotsVisible),
            2692u16 => Ok(Self::JumpVelocity),
            2632u16 => Ok(Self::LastAttackPosition),
            7887u16 => Ok(Self::LastEquippedWeapon),
            11001u16 => Ok(Self::LastKnownClanLandRadius),
            11026u16 => Ok(Self::LastLogoutTime),
            10386u16 => Ok(Self::LastPortalUsed),
            12140u16 => Ok(Self::LastResetDailyQuest),
            10988u16 => Ok(Self::LastSkuSyncTime),
            11230u16 => Ok(Self::LastVendorSyncTime),
            10144u16 => Ok(Self::LoginCount),
            6463u16 => Ok(Self::LootItemGuid),
            7947u16 => Ok(Self::LootItemType),
            2693u16 => Ok(Self::Lvl),
            10177u16 => Ok(Self::LvlHistory),
            12115u16 => Ok(Self::MaxLevelCap),
            11095u16 => Ok(Self::MetamorphItemList),
            2626u16 => Ok(Self::MinigameData),
            3771u16 => Ok(Self::Mount),
            2649u16 => Ok(Self::Mover),
            2694u16 => Ok(Self::MoveSpeed),
            5622u16 => Ok(Self::MyLandData),
            12415u16 => Ok(Self::MyQuestTrack),
            11583u16 => Ok(Self::MyShopGuid),
            12117u16 => Ok(Self::MySteamDlc),
            12116u16 => Ok(Self::MyUsedSteamDlc),
            11231u16 => Ok(Self::NewItems),
            10048u16 => Ok(Self::OutfitNames),
            7264u16 => Ok(Self::OutfitSlots),
            2640u16 => Ok(Self::OverrideFaction),
            2697u16 => Ok(Self::PartyGuid),
            12181u16 => Ok(Self::Pet),
            5212u16 => Ok(Self::PhaseSelectionData),
            10145u16 => Ok(Self::PlaycountMinigameBilliards),
            4943u16 => Ok(Self::PlayerLoading),
            11252u16 => Ok(Self::PlayerNodeState),
            12288u16 => Ok(Self::PlayerUsedSteamDlc),
            10033u16 => Ok(Self::PortalData),
            2699u16 => Ok(Self::Pos),
            2714u16 => Ok(Self::Power),
            9358u16 => Ok(Self::PvpEnabled),
            9985u16 => Ok(Self::PvpEnabledInMyLandServerSetting),
            9986u16 => Ok(Self::PvpEnabledServerSetting),
            12026u16 => Ok(Self::PvpEnableDuration),
            12028u16 => Ok(Self::PvpFlag),
            12025u16 => Ok(Self::PvpRank),
            12027u16 => Ok(Self::PvpTimer),
            12024u16 => Ok(Self::PvpXp),
            11205u16 => Ok(Self::QuickUseBar),
            8912u16 => Ok(Self::Race),
            10123u16 => Ok(Self::RankingEdnamobsTotal),
            10125u16 => Ok(Self::RankingGearTotal),
            10130u16 => Ok(Self::RankingKillsPve),
            10129u16 => Ok(Self::RankingKillsPvp),
            10126u16 => Ok(Self::RankingMypadRooms),
            10127u16 => Ok(Self::RankingSomaAdd),
            10128u16 => Ok(Self::RankingSomaTotal),
            10124u16 => Ok(Self::RankingTotal),
            9987u16 => Ok(Self::RecentlyKilledInPvP),
            7526u16 => Ok(Self::ReferenceList),
            2620u16 => Ok(Self::RelativePosToCarrier),
            6023u16 => Ok(Self::RequestTeleportPos),
            12185u16 => Ok(Self::ResetDailyQuestList),
            2703u16 => Ok(Self::Rot),
            10146u16 => Ok(Self::ScoreMinigameBilliards),
            2704u16 => Ok(Self::SelfRadius),
            10016u16 => Ok(Self::SheathedModeActive),
            11867u16 => Ok(Self::SignClanCharterItem),
            2705u16 => Ok(Self::Size),
            7946u16 => Ok(Self::SomaCarried),
            6457u16 => Ok(Self::SomaLootRate),
            11011u16 => Ok(Self::SpawnCinematicOverride),
            4944u16 => Ok(Self::SpawnedOnAvatar),
            2618u16 => Ok(Self::SpawnMode),
            2634u16 => Ok(Self::SpectateName),
            2633u16 => Ok(Self::SpectatePartyGuid),
            10067u16 => Ok(Self::StatAnyDmgReduction),
            10024u16 => Ok(Self::StatAoEMaxSubTargets),
            10023u16 => Ok(Self::StatAoESubTargetsDamageMod),
            9556u16 => Ok(Self::StatArmorRating),
            9555u16 => Ok(Self::StatArmorReduction),
            9563u16 => Ok(Self::StatAttackPower),
            12098u16 => Ok(Self::StatAttackPowerBonus),
            12099u16 => Ok(Self::StatAttackPowerRating),
            8929u16 => Ok(Self::StatAttackRangePhysAdd),
            9564u16 => Ok(Self::StatAttackRating),
            9559u16 => Ok(Self::StatBendChance),
            9560u16 => Ok(Self::StatBendRating),
            6897u16 => Ok(Self::StatBlockChance),
            9602u16 => Ok(Self::StatBlockedDamageMod),
            6896u16 => Ok(Self::StatBlockRating),
            9561u16 => Ok(Self::StatCritChance),
            12093u16 => Ok(Self::StatCritDmgRating),
            8930u16 => Ok(Self::StatCriticalChanceReduction),
            9604u16 => Ok(Self::StatCriticalDamageMod),
            12094u16 => Ok(Self::StatCriticalDamageModBonus),
            9562u16 => Ok(Self::StatCritRating),
            10050u16 => Ok(Self::StatDamagePercPerMeterMod),
            6735u16 => Ok(Self::StatDefencePowerPhys),
            6728u16 => Ok(Self::StatDefenceRatingPhys),
            6657u16 => Ok(Self::StatDodgeChance),
            6658u16 => Ok(Self::StatDodgeRating),
            6499u16 => Ok(Self::StatEnergyCurrentH1),
            6498u16 => Ok(Self::StatEnergyCurrentH2),
            6497u16 => Ok(Self::StatEnergyCurrentH3),
            6496u16 => Ok(Self::StatEnergyCurrentS1),
            6495u16 => Ok(Self::StatEnergyCurrentS2),
            6494u16 => Ok(Self::StatEnergyCurrentS3),
            6720u16 => Ok(Self::StatEvadeChance),
            6721u16 => Ok(Self::StatEvadeRating),
            10068u16 => Ok(Self::StatExtraHealthRegen),
            11215u16 => Ok(Self::StatFinalDamageMod),
            11216u16 => Ok(Self::StatFinalHealingMod),
            10051u16 => Ok(Self::StatFreeFallDistanceMod),
            11243u16 => Ok(Self::StatHasteClassSkills),
            9356u16 => Ok(Self::StatHastePhysNormal),
            11390u16 => Ok(Self::StatHealingReceivedMod),
            12097u16 => Ok(Self::StatHeavyBonus),
            7051u16 => Ok(Self::StatHeavyEnergyPerHit),
            12092u16 => Ok(Self::StatHeavyRating),
            9553u16 => Ok(Self::StatHitChance),
            9554u16 => Ok(Self::StatHitRating),
            10624u16 => Ok(Self::StatInitialThreatMod),
            6718u16 => Ok(Self::StatParryChance),
            6719u16 => Ok(Self::StatParryRating),
            12095u16 => Ok(Self::StatPeneBonus),
            12091u16 => Ok(Self::StatPeneRating),
            9557u16 => Ok(Self::StatReflectChance),
            9558u16 => Ok(Self::StatReflectRating),
            12096u16 => Ok(Self::StatSpecialBonus),
            7050u16 => Ok(Self::StatSpecialEnergyPerHit),
            12090u16 => Ok(Self::StatSpecialRating),
            9609u16 => Ok(Self::StatStamina),
            7053u16 => Ok(Self::StatTcMax),
            9306u16 => Ok(Self::StatThreatMod),
            9589u16 => Ok(Self::StatWeaponDps),
            12389u16 => Ok(Self::StatWepMaxDmg),
            12388u16 => Ok(Self::StatWepMinDmg),
            7088u16 => Ok(Self::StatXpMod),
            10630u16 => Ok(Self::StickyTargets),
            2706u16 => Ok(Self::Tags),
            2707u16 => Ok(Self::Target),
            3045u16 => Ok(Self::TeamId),
            10147u16 => Ok(Self::TimePlayedBeforeThisSession),
            10148u16 => Ok(Self::TimePlayedThisLevelBeforeThisSession),
            2600u16 => Ok(Self::TutorialMode),
            2672u16 => Ok(Self::Ue3ClassId),
            12444u16 => Ok(Self::UiHintsAvailable),
            10136u16 => Ok(Self::UnassignPortals),
            6502u16 => Ok(Self::UnLockedInstances),
            4038u16 => Ok(Self::UnLockedPortals),
            11309u16 => Ok(Self::UnlockedUiWindows),
            2617u16 => Ok(Self::VisibleItemInfo),
            2709u16 => Ok(Self::Weapon),
            10037u16 => Ok(Self::WorldMapGuid),
            2710u16 => Ok(Self::Xp),
            2711u16 => Ok(Self::XpForNextLevel),
            7052u16 => Ok(Self::XpTotal),
            2712u16 => Ok(Self::Zone),
            2628u16 => Ok(Self::ZoneGuid),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
