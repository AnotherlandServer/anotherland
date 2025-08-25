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
pub enum EdnaModule {
    AdditionalItemCount1,
    AdditionalItemCount2,
    AdditionalItemCount3,
    AdditionalItemRequired1,
    AdditionalItemRequired2,
    AdditionalItemRequired3,
    AllowBuy,
    AllowRent,
    AllowSell,
    BlackSomaRequired,
    BlingPrice,
    BlingSellingPrice,
    BlueSomaRequired,
    BonusSlotAmber,
    BonusSlotRuby,
    BonusSlotSapphire,
    BuyDiscount,
    BuyPriceBling,
    BuyPriceGameCash,
    Category,
    Combos,
    ContainerId,
    ContentClass,
    CraftingMapping,
    CraftTime,
    CreationTime,
    CrystaEffects,
    CrystalType,
    CyanSomaRequired,
    Description,
    DestroyMethod,
    Dialogs,
    DisplayName,
    EnableInGame,
    EquipSlot,
    ExpireBuyBack,
    ExpireTime,
    Freq,
    GameCashPrice,
    GreenSomaRequired,
    Icon,
    InfiniteUse,
    InitLeftTime,
    InventorySlotIndex,
    IsCollectFaction,
    IsEquiped,
    IsFactionItem,
    IsGemeCrystal,
    IsHotSeller,
    IsInGlobalShop,
    IsInStock,
    IsNewToShop,
    IsQuestItem,
    IsRecipe,
    IsSomaSeed,
    IsSoulBounded,
    IsTechApproved,
    IsTrialItem,
    ItemCritVar,
    ItemNormalVar,
    LastUseTime,
    LeftTime,
    LootAction,
    Lua,
    Lvl,
    LvlReq,
    MaterialOverride,
    MaxStackSize,
    OrangeSomaRequired,
    Power,
    Quantity,
    QuestTrigger,
    Rarity,
    RedSomaRequired,
    RentalDurationMax,
    RentalDurationMin,
    RentDiscount,
    RentPriceBling,
    RentPriceGameCash,
    SellPriceBling,
    SlotId,
    SlotMapping,
    SomaType,
    SoulBoundedAccountId,
    SoulBoundedAvatarId,
    SoulBoundedToAccount,
    SoulBoundType,
    StackCount,
    StandingReq,
    UseAction,
    UseCoolDownTimer,
    UseCount,
    UseMaxCount,
    UseRequireAvatar,
    UseRequireAvatarWithinRadius,
    UseRequireTarget,
    UseScript,
    Vendorable,
    VendorAction,
    VioletSomaRequired,
    YellowSomaRequired,
    Abilities,
    AbilityInstanceData,
    Agility,
    Armor,
    AttackPowerRating,
    AttributeOp1,
    AttributeOp2,
    AttributeOp3,
    AttributeOp4,
    AttributeType1,
    AttributeType2,
    AttributeType3,
    AttributeType4,
    AttributeWeight1,
    AttributeWeight2,
    AttributeWeight3,
    AttributeWeight4,
    AutoAttributeType1,
    AutoAttributeType2,
    AutoAttributeType3,
    AutoAttributeType4,
    AutoAttributeType5,
    AutoAttributeType6,
    AutoAttributeValue1,
    AutoAttributeValue2,
    AutoAttributeValue3,
    AutoAttributeValue4,
    AutoAttributeValue5,
    AutoAttributeValue6,
    AvailableSockets,
    BlockRating,
    ClanName,
    CombatStyle,
    CritDamageRating,
    CritHitRating,
    Disguise,
    DisplayNameColor,
    DisplayNameNumber,
    DisplayNameRarity,
    DisplayNameSlot,
    DisplayNameStat,
    DodgeRating,
    Durability,
    DurabilityCurrent,
    Focus,
    Gender,
    HeavyRating,
    HitRating,
    IsBanked,
    IsExpired,
    IsSku,
    IsTemplate,
    ItemActionGenericParam,
    LevelDropVariance,
    MaxUse,
    NoteCaption,
    NoteCaptionValue,
    OtherClientInterests,
    ParryRating,
    PeneRating,
    Prefix,
    QuickbarItem,
    RepairCost,
    SchematicCostToCreateItem,
    SearchKeywords,
    SetBonuses,
    SignOfAvatars,
    Skuid,
    SocketLockedStatus,
    SocketOccupancyStatus,
    SocketUpgradeLevel,
    SpecialRating,
    Stamina,
    Strength,
    Suffix,
    TemplateType,
    TemplateVersion,
    TimeStamp,
    AddBuff,
    ConsiderForLootTables,
    CooldownDuration,
    ExtraData,
    Faction,
    Level,
    Name,
    RecipeContentReferences,
    Ue3ClassId,
}
pub(crate) static EDNA_MODULE_ATTRIBUTES: phf::Map<&'static str, EdnaModule> = phf_map! {
    "AdditionalItemCount1" => EdnaModule::AdditionalItemCount1, "AdditionalItemCount2" =>
    EdnaModule::AdditionalItemCount2, "AdditionalItemCount3" =>
    EdnaModule::AdditionalItemCount3, "AdditionalItemRequired1" =>
    EdnaModule::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    EdnaModule::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    EdnaModule::AdditionalItemRequired3, "AllowBuy" => EdnaModule::AllowBuy, "AllowRent"
    => EdnaModule::AllowRent, "AllowSell" => EdnaModule::AllowSell, "BlackSomaRequired"
    => EdnaModule::BlackSomaRequired, "blingPrice" => EdnaModule::BlingPrice,
    "blingSellingPrice" => EdnaModule::BlingSellingPrice, "BlueSomaRequired" =>
    EdnaModule::BlueSomaRequired, "BonusSlotAmber" => EdnaModule::BonusSlotAmber,
    "BonusSlotRuby" => EdnaModule::BonusSlotRuby, "BonusSlotSapphire" =>
    EdnaModule::BonusSlotSapphire, "BuyDiscount" => EdnaModule::BuyDiscount,
    "BuyPriceBling" => EdnaModule::BuyPriceBling, "BuyPriceGameCash" =>
    EdnaModule::BuyPriceGameCash, "Category" => EdnaModule::Category, "combos" =>
    EdnaModule::Combos, "containerID" => EdnaModule::ContainerId, "ContentClass" =>
    EdnaModule::ContentClass, "CraftingMapping" => EdnaModule::CraftingMapping,
    "CraftTime" => EdnaModule::CraftTime, "creationTime" => EdnaModule::CreationTime,
    "CrystaEffects" => EdnaModule::CrystaEffects, "CrystalType" =>
    EdnaModule::CrystalType, "CyanSomaRequired" => EdnaModule::CyanSomaRequired,
    "Description" => EdnaModule::Description, "DestroyMethod" =>
    EdnaModule::DestroyMethod, "Dialogs" => EdnaModule::Dialogs, "DisplayName" =>
    EdnaModule::DisplayName, "EnableInGame" => EdnaModule::EnableInGame, "equipSlot" =>
    EdnaModule::EquipSlot, "expireBuyBack" => EdnaModule::ExpireBuyBack, "ExpireTime" =>
    EdnaModule::ExpireTime, "Freq" => EdnaModule::Freq, "gameCashPrice" =>
    EdnaModule::GameCashPrice, "GreenSomaRequired" => EdnaModule::GreenSomaRequired,
    "Icon" => EdnaModule::Icon, "InfiniteUse" => EdnaModule::InfiniteUse, "InitLeftTime"
    => EdnaModule::InitLeftTime, "inventorySlotIndex" => EdnaModule::InventorySlotIndex,
    "isCollectFaction" => EdnaModule::IsCollectFaction, "isEquiped" =>
    EdnaModule::IsEquiped, "isFactionItem" => EdnaModule::IsFactionItem, "isGemeCrystal"
    => EdnaModule::IsGemeCrystal, "IsHotSeller" => EdnaModule::IsHotSeller,
    "isInGlobalShop" => EdnaModule::IsInGlobalShop, "IsInStock" => EdnaModule::IsInStock,
    "IsNewToShop" => EdnaModule::IsNewToShop, "isQuestItem" => EdnaModule::IsQuestItem,
    "IsRecipe" => EdnaModule::IsRecipe, "IsSomaSeed" => EdnaModule::IsSomaSeed,
    "IsSoulBounded" => EdnaModule::IsSoulBounded, "isTechApproved" =>
    EdnaModule::IsTechApproved, "isTrialItem" => EdnaModule::IsTrialItem, "ItemCritVar"
    => EdnaModule::ItemCritVar, "ItemNormalVar" => EdnaModule::ItemNormalVar,
    "LastUseTime" => EdnaModule::LastUseTime, "LeftTime" => EdnaModule::LeftTime,
    "lootAction" => EdnaModule::LootAction, "Lua" => EdnaModule::Lua, "lvl" =>
    EdnaModule::Lvl, "lvlReq" => EdnaModule::LvlReq, "MaterialOverride" =>
    EdnaModule::MaterialOverride, "maxStackSize" => EdnaModule::MaxStackSize,
    "OrangeSomaRequired" => EdnaModule::OrangeSomaRequired, "Power" => EdnaModule::Power,
    "quantity" => EdnaModule::Quantity, "QuestTrigger" => EdnaModule::QuestTrigger,
    "rarity" => EdnaModule::Rarity, "RedSomaRequired" => EdnaModule::RedSomaRequired,
    "RentalDurationMax" => EdnaModule::RentalDurationMax, "RentalDurationMin" =>
    EdnaModule::RentalDurationMin, "RentDiscount" => EdnaModule::RentDiscount,
    "RentPriceBling" => EdnaModule::RentPriceBling, "RentPriceGameCash" =>
    EdnaModule::RentPriceGameCash, "SellPriceBling" => EdnaModule::SellPriceBling,
    "slotID" => EdnaModule::SlotId, "SlotMapping" => EdnaModule::SlotMapping, "SomaType"
    => EdnaModule::SomaType, "SoulBoundedAccountId" => EdnaModule::SoulBoundedAccountId,
    "SoulBoundedAvatarId" => EdnaModule::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    EdnaModule::SoulBoundedToAccount, "SoulBoundType" => EdnaModule::SoulBoundType,
    "stackCount" => EdnaModule::StackCount, "standingReq" => EdnaModule::StandingReq,
    "useAction" => EdnaModule::UseAction, "UseCoolDownTimer" =>
    EdnaModule::UseCoolDownTimer, "UseCount" => EdnaModule::UseCount, "UseMaxCount" =>
    EdnaModule::UseMaxCount, "UseRequireAvatar" => EdnaModule::UseRequireAvatar,
    "UseRequireAvatarWithinRadius" => EdnaModule::UseRequireAvatarWithinRadius,
    "UseRequireTarget" => EdnaModule::UseRequireTarget, "UseScript" =>
    EdnaModule::UseScript, "Vendorable" => EdnaModule::Vendorable, "vendorAction" =>
    EdnaModule::VendorAction, "VioletSomaRequired" => EdnaModule::VioletSomaRequired,
    "YellowSomaRequired" => EdnaModule::YellowSomaRequired, "abilities" =>
    EdnaModule::Abilities, "abilityInstanceData" => EdnaModule::AbilityInstanceData,
    "Agility" => EdnaModule::Agility, "Armor" => EdnaModule::Armor, "AttackPowerRating"
    => EdnaModule::AttackPowerRating, "attributeOp1" => EdnaModule::AttributeOp1,
    "attributeOp2" => EdnaModule::AttributeOp2, "attributeOp3" =>
    EdnaModule::AttributeOp3, "attributeOp4" => EdnaModule::AttributeOp4,
    "attributeType1" => EdnaModule::AttributeType1, "attributeType2" =>
    EdnaModule::AttributeType2, "attributeType3" => EdnaModule::AttributeType3,
    "attributeType4" => EdnaModule::AttributeType4, "attributeWeight1" =>
    EdnaModule::AttributeWeight1, "attributeWeight2" => EdnaModule::AttributeWeight2,
    "attributeWeight3" => EdnaModule::AttributeWeight3, "attributeWeight4" =>
    EdnaModule::AttributeWeight4, "autoAttributeType1" => EdnaModule::AutoAttributeType1,
    "autoAttributeType2" => EdnaModule::AutoAttributeType2, "autoAttributeType3" =>
    EdnaModule::AutoAttributeType3, "autoAttributeType4" =>
    EdnaModule::AutoAttributeType4, "autoAttributeType5" =>
    EdnaModule::AutoAttributeType5, "autoAttributeType6" =>
    EdnaModule::AutoAttributeType6, "autoAttributeValue1" =>
    EdnaModule::AutoAttributeValue1, "autoAttributeValue2" =>
    EdnaModule::AutoAttributeValue2, "autoAttributeValue3" =>
    EdnaModule::AutoAttributeValue3, "autoAttributeValue4" =>
    EdnaModule::AutoAttributeValue4, "autoAttributeValue5" =>
    EdnaModule::AutoAttributeValue5, "autoAttributeValue6" =>
    EdnaModule::AutoAttributeValue6, "availableSockets" => EdnaModule::AvailableSockets,
    "BlockRating" => EdnaModule::BlockRating, "ClanName" => EdnaModule::ClanName,
    "combatStyle" => EdnaModule::CombatStyle, "CritDamageRating" =>
    EdnaModule::CritDamageRating, "CritHitRating" => EdnaModule::CritHitRating,
    "disguise" => EdnaModule::Disguise, "DisplayName_Color" =>
    EdnaModule::DisplayNameColor, "DisplayName_Number" => EdnaModule::DisplayNameNumber,
    "DisplayName_Rarity" => EdnaModule::DisplayNameRarity, "DisplayName_Slot" =>
    EdnaModule::DisplayNameSlot, "DisplayName_Stat" => EdnaModule::DisplayNameStat,
    "DodgeRating" => EdnaModule::DodgeRating, "durability" => EdnaModule::Durability,
    "durabilityCurrent" => EdnaModule::DurabilityCurrent, "Focus" => EdnaModule::Focus,
    "gender" => EdnaModule::Gender, "HeavyRating" => EdnaModule::HeavyRating, "HitRating"
    => EdnaModule::HitRating, "isBanked" => EdnaModule::IsBanked, "isExpired" =>
    EdnaModule::IsExpired, "isSKU" => EdnaModule::IsSku, "IsTemplate" =>
    EdnaModule::IsTemplate, "itemActionGenericParam" =>
    EdnaModule::ItemActionGenericParam, "levelDropVariance" =>
    EdnaModule::LevelDropVariance, "MaxUse" => EdnaModule::MaxUse, "NoteCaption" =>
    EdnaModule::NoteCaption, "NoteCaptionValue" => EdnaModule::NoteCaptionValue,
    "otherClientInterests" => EdnaModule::OtherClientInterests, "ParryRating" =>
    EdnaModule::ParryRating, "PeneRating" => EdnaModule::PeneRating, "Prefix" =>
    EdnaModule::Prefix, "QuickbarItem" => EdnaModule::QuickbarItem, "repairCost" =>
    EdnaModule::RepairCost, "schematic_CostToCreateItem" =>
    EdnaModule::SchematicCostToCreateItem, "searchKeywords" =>
    EdnaModule::SearchKeywords, "setBonuses" => EdnaModule::SetBonuses, "SignOfAvatars"
    => EdnaModule::SignOfAvatars, "SKUID" => EdnaModule::Skuid, "socketLockedStatus" =>
    EdnaModule::SocketLockedStatus, "socketOccupancyStatus" =>
    EdnaModule::SocketOccupancyStatus, "socketUpgradeLevel" =>
    EdnaModule::SocketUpgradeLevel, "SpecialRating" => EdnaModule::SpecialRating,
    "Stamina" => EdnaModule::Stamina, "Strength" => EdnaModule::Strength, "Suffix" =>
    EdnaModule::Suffix, "templateType" => EdnaModule::TemplateType, "templateVersion" =>
    EdnaModule::TemplateVersion, "timeStamp" => EdnaModule::TimeStamp, "AddBuff" =>
    EdnaModule::AddBuff, "considerForLootTables" => EdnaModule::ConsiderForLootTables,
    "cooldownDuration" => EdnaModule::CooldownDuration, "extraData" =>
    EdnaModule::ExtraData, "Faction" => EdnaModule::Faction, "level" =>
    EdnaModule::Level, "name" => EdnaModule::Name, "RecipeContentReferences" =>
    EdnaModule::RecipeContentReferences, "UE3ClassID" => EdnaModule::Ue3ClassId,
};
pub(crate) static EDNA_MODULE_ATTRIBUTES_ID: phf::Map<u16, EdnaModule> = phf_map! {
    12368u16 => EdnaModule::AdditionalItemCount1, 12367u16 =>
    EdnaModule::AdditionalItemCount2, 12366u16 => EdnaModule::AdditionalItemCount3,
    11720u16 => EdnaModule::AdditionalItemRequired1, 11719u16 =>
    EdnaModule::AdditionalItemRequired2, 11718u16 => EdnaModule::AdditionalItemRequired3,
    7580u16 => EdnaModule::AllowBuy, 7569u16 => EdnaModule::AllowRent, 7627u16 =>
    EdnaModule::AllowSell, 11717u16 => EdnaModule::BlackSomaRequired, 6582u16 =>
    EdnaModule::BlingPrice, 6581u16 => EdnaModule::BlingSellingPrice, 11716u16 =>
    EdnaModule::BlueSomaRequired, 11960u16 => EdnaModule::BonusSlotAmber, 11961u16 =>
    EdnaModule::BonusSlotRuby, 11962u16 => EdnaModule::BonusSlotSapphire, 7628u16 =>
    EdnaModule::BuyDiscount, 7630u16 => EdnaModule::BuyPriceBling, 7629u16 =>
    EdnaModule::BuyPriceGameCash, 7547u16 => EdnaModule::Category, 8896u16 =>
    EdnaModule::Combos, 762u16 => EdnaModule::ContainerId, 793u16 =>
    EdnaModule::ContentClass, 12196u16 => EdnaModule::CraftingMapping, 11708u16 =>
    EdnaModule::CraftTime, 761u16 => EdnaModule::CreationTime, 11993u16 =>
    EdnaModule::CrystaEffects, 11995u16 => EdnaModule::CrystalType, 11715u16 =>
    EdnaModule::CyanSomaRequired, 6955u16 => EdnaModule::Description, 6489u16 =>
    EdnaModule::DestroyMethod, 8923u16 => EdnaModule::Dialogs, 795u16 =>
    EdnaModule::DisplayName, 6816u16 => EdnaModule::EnableInGame, 791u16 =>
    EdnaModule::EquipSlot, 11612u16 => EdnaModule::ExpireBuyBack, 7558u16 =>
    EdnaModule::ExpireTime, 780u16 => EdnaModule::Freq, 6580u16 =>
    EdnaModule::GameCashPrice, 11714u16 => EdnaModule::GreenSomaRequired, 4347u16 =>
    EdnaModule::Icon, 11466u16 => EdnaModule::InfiniteUse, 12337u16 =>
    EdnaModule::InitLeftTime, 9874u16 => EdnaModule::InventorySlotIndex, 12172u16 =>
    EdnaModule::IsCollectFaction, 789u16 => EdnaModule::IsEquiped, 12154u16 =>
    EdnaModule::IsFactionItem, 11994u16 => EdnaModule::IsGemeCrystal, 7377u16 =>
    EdnaModule::IsHotSeller, 7147u16 => EdnaModule::IsInGlobalShop, 7376u16 =>
    EdnaModule::IsInStock, 7378u16 => EdnaModule::IsNewToShop, 9911u16 =>
    EdnaModule::IsQuestItem, 11709u16 => EdnaModule::IsRecipe, 12405u16 =>
    EdnaModule::IsSomaSeed, 10588u16 => EdnaModule::IsSoulBounded, 9377u16 =>
    EdnaModule::IsTechApproved, 7749u16 => EdnaModule::IsTrialItem, 11721u16 =>
    EdnaModule::ItemCritVar, 11722u16 => EdnaModule::ItemNormalVar, 9015u16 =>
    EdnaModule::LastUseTime, 12338u16 => EdnaModule::LeftTime, 5995u16 =>
    EdnaModule::LootAction, 10156u16 => EdnaModule::Lua, 6175u16 => EdnaModule::Lvl,
    785u16 => EdnaModule::LvlReq, 4726u16 => EdnaModule::MaterialOverride, 9893u16 =>
    EdnaModule::MaxStackSize, 11713u16 => EdnaModule::OrangeSomaRequired, 781u16 =>
    EdnaModule::Power, 6435u16 => EdnaModule::Quantity, 7720u16 =>
    EdnaModule::QuestTrigger, 6280u16 => EdnaModule::Rarity, 11712u16 =>
    EdnaModule::RedSomaRequired, 7458u16 => EdnaModule::RentalDurationMax, 7459u16 =>
    EdnaModule::RentalDurationMin, 7631u16 => EdnaModule::RentDiscount, 7633u16 =>
    EdnaModule::RentPriceBling, 7632u16 => EdnaModule::RentPriceGameCash, 7626u16 =>
    EdnaModule::SellPriceBling, 782u16 => EdnaModule::SlotId, 6249u16 =>
    EdnaModule::SlotMapping, 12404u16 => EdnaModule::SomaType, 12262u16 =>
    EdnaModule::SoulBoundedAccountId, 10615u16 => EdnaModule::SoulBoundedAvatarId,
    12251u16 => EdnaModule::SoulBoundedToAccount, 10587u16 => EdnaModule::SoulBoundType,
    9892u16 => EdnaModule::StackCount, 12171u16 => EdnaModule::StandingReq, 6020u16 =>
    EdnaModule::UseAction, 8998u16 => EdnaModule::UseCoolDownTimer, 9026u16 =>
    EdnaModule::UseCount, 8999u16 => EdnaModule::UseMaxCount, 8964u16 =>
    EdnaModule::UseRequireAvatar, 8963u16 => EdnaModule::UseRequireAvatarWithinRadius,
    8965u16 => EdnaModule::UseRequireTarget, 8962u16 => EdnaModule::UseScript, 6488u16 =>
    EdnaModule::Vendorable, 5936u16 => EdnaModule::VendorAction, 11711u16 =>
    EdnaModule::VioletSomaRequired, 11710u16 => EdnaModule::YellowSomaRequired, 774u16 =>
    EdnaModule::Abilities, 766u16 => EdnaModule::AbilityInstanceData, 11563u16 =>
    EdnaModule::Agility, 11550u16 => EdnaModule::Armor, 11554u16 =>
    EdnaModule::AttackPowerRating, 6419u16 => EdnaModule::AttributeOp1, 6418u16 =>
    EdnaModule::AttributeOp2, 6417u16 => EdnaModule::AttributeOp3, 6416u16 =>
    EdnaModule::AttributeOp4, 6423u16 => EdnaModule::AttributeType1, 6422u16 =>
    EdnaModule::AttributeType2, 6421u16 => EdnaModule::AttributeType3, 6420u16 =>
    EdnaModule::AttributeType4, 6415u16 => EdnaModule::AttributeWeight1, 6414u16 =>
    EdnaModule::AttributeWeight2, 6413u16 => EdnaModule::AttributeWeight3, 6412u16 =>
    EdnaModule::AttributeWeight4, 9494u16 => EdnaModule::AutoAttributeType1, 9493u16 =>
    EdnaModule::AutoAttributeType2, 9492u16 => EdnaModule::AutoAttributeType3, 9491u16 =>
    EdnaModule::AutoAttributeType4, 9552u16 => EdnaModule::AutoAttributeType5, 9551u16 =>
    EdnaModule::AutoAttributeType6, 9488u16 => EdnaModule::AutoAttributeValue1, 9487u16
    => EdnaModule::AutoAttributeValue2, 9486u16 => EdnaModule::AutoAttributeValue3,
    9485u16 => EdnaModule::AutoAttributeValue4, 9550u16 =>
    EdnaModule::AutoAttributeValue5, 9549u16 => EdnaModule::AutoAttributeValue6, 10089u16
    => EdnaModule::AvailableSockets, 11551u16 => EdnaModule::BlockRating, 12039u16 =>
    EdnaModule::ClanName, 4250u16 => EdnaModule::CombatStyle, 11555u16 =>
    EdnaModule::CritDamageRating, 11559u16 => EdnaModule::CritHitRating, 9991u16 =>
    EdnaModule::Disguise, 12238u16 => EdnaModule::DisplayNameColor, 12237u16 =>
    EdnaModule::DisplayNameNumber, 12239u16 => EdnaModule::DisplayNameRarity, 12241u16 =>
    EdnaModule::DisplayNameSlot, 12240u16 => EdnaModule::DisplayNameStat, 11553u16 =>
    EdnaModule::DodgeRating, 10021u16 => EdnaModule::Durability, 10096u16 =>
    EdnaModule::DurabilityCurrent, 11562u16 => EdnaModule::Focus, 11019u16 =>
    EdnaModule::Gender, 11557u16 => EdnaModule::HeavyRating, 11560u16 =>
    EdnaModule::HitRating, 790u16 => EdnaModule::IsBanked, 764u16 =>
    EdnaModule::IsExpired, 10193u16 => EdnaModule::IsSku, 9710u16 =>
    EdnaModule::IsTemplate, 10390u16 => EdnaModule::ItemActionGenericParam, 10900u16 =>
    EdnaModule::LevelDropVariance, 4814u16 => EdnaModule::MaxUse, 12243u16 =>
    EdnaModule::NoteCaption, 12242u16 => EdnaModule::NoteCaptionValue, 767u16 =>
    EdnaModule::OtherClientInterests, 11552u16 => EdnaModule::ParryRating, 11558u16 =>
    EdnaModule::PeneRating, 9709u16 => EdnaModule::Prefix, 12276u16 =>
    EdnaModule::QuickbarItem, 10101u16 => EdnaModule::RepairCost, 10106u16 =>
    EdnaModule::SchematicCostToCreateItem, 10072u16 => EdnaModule::SearchKeywords,
    12386u16 => EdnaModule::SetBonuses, 12040u16 => EdnaModule::SignOfAvatars, 10609u16
    => EdnaModule::Skuid, 10092u16 => EdnaModule::SocketLockedStatus, 10090u16 =>
    EdnaModule::SocketOccupancyStatus, 10091u16 => EdnaModule::SocketUpgradeLevel,
    11556u16 => EdnaModule::SpecialRating, 11561u16 => EdnaModule::Stamina, 11564u16 =>
    EdnaModule::Strength, 9708u16 => EdnaModule::Suffix, 10076u16 =>
    EdnaModule::TemplateType, 11314u16 => EdnaModule::TemplateVersion, 763u16 =>
    EdnaModule::TimeStamp, 9371u16 => EdnaModule::AddBuff, 12290u16 =>
    EdnaModule::ConsiderForLootTables, 754u16 => EdnaModule::CooldownDuration, 768u16 =>
    EdnaModule::ExtraData, 12146u16 => EdnaModule::Faction, 4941u16 => EdnaModule::Level,
    779u16 => EdnaModule::Name, 12277u16 => EdnaModule::RecipeContentReferences, 777u16
    => EdnaModule::Ue3ClassId,
};
impl Attribute for EdnaModule {
    fn class() -> Class {
        Class::EdnaModule
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AdditionalItemCount1 => &Self::AdditionalItemCount1,
            Self::AdditionalItemCount2 => &Self::AdditionalItemCount2,
            Self::AdditionalItemCount3 => &Self::AdditionalItemCount3,
            Self::AdditionalItemRequired1 => &Self::AdditionalItemRequired1,
            Self::AdditionalItemRequired2 => &Self::AdditionalItemRequired2,
            Self::AdditionalItemRequired3 => &Self::AdditionalItemRequired3,
            Self::AllowBuy => &Self::AllowBuy,
            Self::AllowRent => &Self::AllowRent,
            Self::AllowSell => &Self::AllowSell,
            Self::BlackSomaRequired => &Self::BlackSomaRequired,
            Self::BlingPrice => &Self::BlingPrice,
            Self::BlingSellingPrice => &Self::BlingSellingPrice,
            Self::BlueSomaRequired => &Self::BlueSomaRequired,
            Self::BonusSlotAmber => &Self::BonusSlotAmber,
            Self::BonusSlotRuby => &Self::BonusSlotRuby,
            Self::BonusSlotSapphire => &Self::BonusSlotSapphire,
            Self::BuyDiscount => &Self::BuyDiscount,
            Self::BuyPriceBling => &Self::BuyPriceBling,
            Self::BuyPriceGameCash => &Self::BuyPriceGameCash,
            Self::Category => &Self::Category,
            Self::Combos => &Self::Combos,
            Self::ContainerId => &Self::ContainerId,
            Self::ContentClass => &Self::ContentClass,
            Self::CraftingMapping => &Self::CraftingMapping,
            Self::CraftTime => &Self::CraftTime,
            Self::CreationTime => &Self::CreationTime,
            Self::CrystaEffects => &Self::CrystaEffects,
            Self::CrystalType => &Self::CrystalType,
            Self::CyanSomaRequired => &Self::CyanSomaRequired,
            Self::Description => &Self::Description,
            Self::DestroyMethod => &Self::DestroyMethod,
            Self::Dialogs => &Self::Dialogs,
            Self::DisplayName => &Self::DisplayName,
            Self::EnableInGame => &Self::EnableInGame,
            Self::EquipSlot => &Self::EquipSlot,
            Self::ExpireBuyBack => &Self::ExpireBuyBack,
            Self::ExpireTime => &Self::ExpireTime,
            Self::Freq => &Self::Freq,
            Self::GameCashPrice => &Self::GameCashPrice,
            Self::GreenSomaRequired => &Self::GreenSomaRequired,
            Self::Icon => &Self::Icon,
            Self::InfiniteUse => &Self::InfiniteUse,
            Self::InitLeftTime => &Self::InitLeftTime,
            Self::InventorySlotIndex => &Self::InventorySlotIndex,
            Self::IsCollectFaction => &Self::IsCollectFaction,
            Self::IsEquiped => &Self::IsEquiped,
            Self::IsFactionItem => &Self::IsFactionItem,
            Self::IsGemeCrystal => &Self::IsGemeCrystal,
            Self::IsHotSeller => &Self::IsHotSeller,
            Self::IsInGlobalShop => &Self::IsInGlobalShop,
            Self::IsInStock => &Self::IsInStock,
            Self::IsNewToShop => &Self::IsNewToShop,
            Self::IsQuestItem => &Self::IsQuestItem,
            Self::IsRecipe => &Self::IsRecipe,
            Self::IsSomaSeed => &Self::IsSomaSeed,
            Self::IsSoulBounded => &Self::IsSoulBounded,
            Self::IsTechApproved => &Self::IsTechApproved,
            Self::IsTrialItem => &Self::IsTrialItem,
            Self::ItemCritVar => &Self::ItemCritVar,
            Self::ItemNormalVar => &Self::ItemNormalVar,
            Self::LastUseTime => &Self::LastUseTime,
            Self::LeftTime => &Self::LeftTime,
            Self::LootAction => &Self::LootAction,
            Self::Lua => &Self::Lua,
            Self::Lvl => &Self::Lvl,
            Self::LvlReq => &Self::LvlReq,
            Self::MaterialOverride => &Self::MaterialOverride,
            Self::MaxStackSize => &Self::MaxStackSize,
            Self::OrangeSomaRequired => &Self::OrangeSomaRequired,
            Self::Power => &Self::Power,
            Self::Quantity => &Self::Quantity,
            Self::QuestTrigger => &Self::QuestTrigger,
            Self::Rarity => &Self::Rarity,
            Self::RedSomaRequired => &Self::RedSomaRequired,
            Self::RentalDurationMax => &Self::RentalDurationMax,
            Self::RentalDurationMin => &Self::RentalDurationMin,
            Self::RentDiscount => &Self::RentDiscount,
            Self::RentPriceBling => &Self::RentPriceBling,
            Self::RentPriceGameCash => &Self::RentPriceGameCash,
            Self::SellPriceBling => &Self::SellPriceBling,
            Self::SlotId => &Self::SlotId,
            Self::SlotMapping => &Self::SlotMapping,
            Self::SomaType => &Self::SomaType,
            Self::SoulBoundedAccountId => &Self::SoulBoundedAccountId,
            Self::SoulBoundedAvatarId => &Self::SoulBoundedAvatarId,
            Self::SoulBoundedToAccount => &Self::SoulBoundedToAccount,
            Self::SoulBoundType => &Self::SoulBoundType,
            Self::StackCount => &Self::StackCount,
            Self::StandingReq => &Self::StandingReq,
            Self::UseAction => &Self::UseAction,
            Self::UseCoolDownTimer => &Self::UseCoolDownTimer,
            Self::UseCount => &Self::UseCount,
            Self::UseMaxCount => &Self::UseMaxCount,
            Self::UseRequireAvatar => &Self::UseRequireAvatar,
            Self::UseRequireAvatarWithinRadius => &Self::UseRequireAvatarWithinRadius,
            Self::UseRequireTarget => &Self::UseRequireTarget,
            Self::UseScript => &Self::UseScript,
            Self::Vendorable => &Self::Vendorable,
            Self::VendorAction => &Self::VendorAction,
            Self::VioletSomaRequired => &Self::VioletSomaRequired,
            Self::YellowSomaRequired => &Self::YellowSomaRequired,
            Self::Abilities => &Self::Abilities,
            Self::AbilityInstanceData => &Self::AbilityInstanceData,
            Self::Agility => &Self::Agility,
            Self::Armor => &Self::Armor,
            Self::AttackPowerRating => &Self::AttackPowerRating,
            Self::AttributeOp1 => &Self::AttributeOp1,
            Self::AttributeOp2 => &Self::AttributeOp2,
            Self::AttributeOp3 => &Self::AttributeOp3,
            Self::AttributeOp4 => &Self::AttributeOp4,
            Self::AttributeType1 => &Self::AttributeType1,
            Self::AttributeType2 => &Self::AttributeType2,
            Self::AttributeType3 => &Self::AttributeType3,
            Self::AttributeType4 => &Self::AttributeType4,
            Self::AttributeWeight1 => &Self::AttributeWeight1,
            Self::AttributeWeight2 => &Self::AttributeWeight2,
            Self::AttributeWeight3 => &Self::AttributeWeight3,
            Self::AttributeWeight4 => &Self::AttributeWeight4,
            Self::AutoAttributeType1 => &Self::AutoAttributeType1,
            Self::AutoAttributeType2 => &Self::AutoAttributeType2,
            Self::AutoAttributeType3 => &Self::AutoAttributeType3,
            Self::AutoAttributeType4 => &Self::AutoAttributeType4,
            Self::AutoAttributeType5 => &Self::AutoAttributeType5,
            Self::AutoAttributeType6 => &Self::AutoAttributeType6,
            Self::AutoAttributeValue1 => &Self::AutoAttributeValue1,
            Self::AutoAttributeValue2 => &Self::AutoAttributeValue2,
            Self::AutoAttributeValue3 => &Self::AutoAttributeValue3,
            Self::AutoAttributeValue4 => &Self::AutoAttributeValue4,
            Self::AutoAttributeValue5 => &Self::AutoAttributeValue5,
            Self::AutoAttributeValue6 => &Self::AutoAttributeValue6,
            Self::AvailableSockets => &Self::AvailableSockets,
            Self::BlockRating => &Self::BlockRating,
            Self::ClanName => &Self::ClanName,
            Self::CombatStyle => &Self::CombatStyle,
            Self::CritDamageRating => &Self::CritDamageRating,
            Self::CritHitRating => &Self::CritHitRating,
            Self::Disguise => &Self::Disguise,
            Self::DisplayNameColor => &Self::DisplayNameColor,
            Self::DisplayNameNumber => &Self::DisplayNameNumber,
            Self::DisplayNameRarity => &Self::DisplayNameRarity,
            Self::DisplayNameSlot => &Self::DisplayNameSlot,
            Self::DisplayNameStat => &Self::DisplayNameStat,
            Self::DodgeRating => &Self::DodgeRating,
            Self::Durability => &Self::Durability,
            Self::DurabilityCurrent => &Self::DurabilityCurrent,
            Self::Focus => &Self::Focus,
            Self::Gender => &Self::Gender,
            Self::HeavyRating => &Self::HeavyRating,
            Self::HitRating => &Self::HitRating,
            Self::IsBanked => &Self::IsBanked,
            Self::IsExpired => &Self::IsExpired,
            Self::IsSku => &Self::IsSku,
            Self::IsTemplate => &Self::IsTemplate,
            Self::ItemActionGenericParam => &Self::ItemActionGenericParam,
            Self::LevelDropVariance => &Self::LevelDropVariance,
            Self::MaxUse => &Self::MaxUse,
            Self::NoteCaption => &Self::NoteCaption,
            Self::NoteCaptionValue => &Self::NoteCaptionValue,
            Self::OtherClientInterests => &Self::OtherClientInterests,
            Self::ParryRating => &Self::ParryRating,
            Self::PeneRating => &Self::PeneRating,
            Self::Prefix => &Self::Prefix,
            Self::QuickbarItem => &Self::QuickbarItem,
            Self::RepairCost => &Self::RepairCost,
            Self::SchematicCostToCreateItem => &Self::SchematicCostToCreateItem,
            Self::SearchKeywords => &Self::SearchKeywords,
            Self::SetBonuses => &Self::SetBonuses,
            Self::SignOfAvatars => &Self::SignOfAvatars,
            Self::Skuid => &Self::Skuid,
            Self::SocketLockedStatus => &Self::SocketLockedStatus,
            Self::SocketOccupancyStatus => &Self::SocketOccupancyStatus,
            Self::SocketUpgradeLevel => &Self::SocketUpgradeLevel,
            Self::SpecialRating => &Self::SpecialRating,
            Self::Stamina => &Self::Stamina,
            Self::Strength => &Self::Strength,
            Self::Suffix => &Self::Suffix,
            Self::TemplateType => &Self::TemplateType,
            Self::TemplateVersion => &Self::TemplateVersion,
            Self::TimeStamp => &Self::TimeStamp,
            Self::AddBuff => &Self::AddBuff,
            Self::ConsiderForLootTables => &Self::ConsiderForLootTables,
            Self::CooldownDuration => &Self::CooldownDuration,
            Self::ExtraData => &Self::ExtraData,
            Self::Faction => &Self::Faction,
            Self::Level => &Self::Level,
            Self::Name => &Self::Name,
            Self::RecipeContentReferences => &Self::RecipeContentReferences,
            Self::Ue3ClassId => &Self::Ue3ClassId,
        }
    }
}
impl AttributeInfo for EdnaModule {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12368u16,
            Self::AdditionalItemCount2 => 12367u16,
            Self::AdditionalItemCount3 => 12366u16,
            Self::AdditionalItemRequired1 => 11720u16,
            Self::AdditionalItemRequired2 => 11719u16,
            Self::AdditionalItemRequired3 => 11718u16,
            Self::AllowBuy => 7580u16,
            Self::AllowRent => 7569u16,
            Self::AllowSell => 7627u16,
            Self::BlackSomaRequired => 11717u16,
            Self::BlingPrice => 6582u16,
            Self::BlingSellingPrice => 6581u16,
            Self::BlueSomaRequired => 11716u16,
            Self::BonusSlotAmber => 11960u16,
            Self::BonusSlotRuby => 11961u16,
            Self::BonusSlotSapphire => 11962u16,
            Self::BuyDiscount => 7628u16,
            Self::BuyPriceBling => 7630u16,
            Self::BuyPriceGameCash => 7629u16,
            Self::Category => 7547u16,
            Self::Combos => 8896u16,
            Self::ContainerId => 762u16,
            Self::ContentClass => 793u16,
            Self::CraftingMapping => 12196u16,
            Self::CraftTime => 11708u16,
            Self::CreationTime => 761u16,
            Self::CrystaEffects => 11993u16,
            Self::CrystalType => 11995u16,
            Self::CyanSomaRequired => 11715u16,
            Self::Description => 6955u16,
            Self::DestroyMethod => 6489u16,
            Self::Dialogs => 8923u16,
            Self::DisplayName => 795u16,
            Self::EnableInGame => 6816u16,
            Self::EquipSlot => 791u16,
            Self::ExpireBuyBack => 11612u16,
            Self::ExpireTime => 7558u16,
            Self::Freq => 780u16,
            Self::GameCashPrice => 6580u16,
            Self::GreenSomaRequired => 11714u16,
            Self::Icon => 4347u16,
            Self::InfiniteUse => 11466u16,
            Self::InitLeftTime => 12337u16,
            Self::InventorySlotIndex => 9874u16,
            Self::IsCollectFaction => 12172u16,
            Self::IsEquiped => 789u16,
            Self::IsFactionItem => 12154u16,
            Self::IsGemeCrystal => 11994u16,
            Self::IsHotSeller => 7377u16,
            Self::IsInGlobalShop => 7147u16,
            Self::IsInStock => 7376u16,
            Self::IsNewToShop => 7378u16,
            Self::IsQuestItem => 9911u16,
            Self::IsRecipe => 11709u16,
            Self::IsSomaSeed => 12405u16,
            Self::IsSoulBounded => 10588u16,
            Self::IsTechApproved => 9377u16,
            Self::IsTrialItem => 7749u16,
            Self::ItemCritVar => 11721u16,
            Self::ItemNormalVar => 11722u16,
            Self::LastUseTime => 9015u16,
            Self::LeftTime => 12338u16,
            Self::LootAction => 5995u16,
            Self::Lua => 10156u16,
            Self::Lvl => 6175u16,
            Self::LvlReq => 785u16,
            Self::MaterialOverride => 4726u16,
            Self::MaxStackSize => 9893u16,
            Self::OrangeSomaRequired => 11713u16,
            Self::Power => 781u16,
            Self::Quantity => 6435u16,
            Self::QuestTrigger => 7720u16,
            Self::Rarity => 6280u16,
            Self::RedSomaRequired => 11712u16,
            Self::RentalDurationMax => 7458u16,
            Self::RentalDurationMin => 7459u16,
            Self::RentDiscount => 7631u16,
            Self::RentPriceBling => 7633u16,
            Self::RentPriceGameCash => 7632u16,
            Self::SellPriceBling => 7626u16,
            Self::SlotId => 782u16,
            Self::SlotMapping => 6249u16,
            Self::SomaType => 12404u16,
            Self::SoulBoundedAccountId => 12262u16,
            Self::SoulBoundedAvatarId => 10615u16,
            Self::SoulBoundedToAccount => 12251u16,
            Self::SoulBoundType => 10587u16,
            Self::StackCount => 9892u16,
            Self::StandingReq => 12171u16,
            Self::UseAction => 6020u16,
            Self::UseCoolDownTimer => 8998u16,
            Self::UseCount => 9026u16,
            Self::UseMaxCount => 8999u16,
            Self::UseRequireAvatar => 8964u16,
            Self::UseRequireAvatarWithinRadius => 8963u16,
            Self::UseRequireTarget => 8965u16,
            Self::UseScript => 8962u16,
            Self::Vendorable => 6488u16,
            Self::VendorAction => 5936u16,
            Self::VioletSomaRequired => 11711u16,
            Self::YellowSomaRequired => 11710u16,
            Self::Abilities => 774u16,
            Self::AbilityInstanceData => 766u16,
            Self::Agility => 11563u16,
            Self::Armor => 11550u16,
            Self::AttackPowerRating => 11554u16,
            Self::AttributeOp1 => 6419u16,
            Self::AttributeOp2 => 6418u16,
            Self::AttributeOp3 => 6417u16,
            Self::AttributeOp4 => 6416u16,
            Self::AttributeType1 => 6423u16,
            Self::AttributeType2 => 6422u16,
            Self::AttributeType3 => 6421u16,
            Self::AttributeType4 => 6420u16,
            Self::AttributeWeight1 => 6415u16,
            Self::AttributeWeight2 => 6414u16,
            Self::AttributeWeight3 => 6413u16,
            Self::AttributeWeight4 => 6412u16,
            Self::AutoAttributeType1 => 9494u16,
            Self::AutoAttributeType2 => 9493u16,
            Self::AutoAttributeType3 => 9492u16,
            Self::AutoAttributeType4 => 9491u16,
            Self::AutoAttributeType5 => 9552u16,
            Self::AutoAttributeType6 => 9551u16,
            Self::AutoAttributeValue1 => 9488u16,
            Self::AutoAttributeValue2 => 9487u16,
            Self::AutoAttributeValue3 => 9486u16,
            Self::AutoAttributeValue4 => 9485u16,
            Self::AutoAttributeValue5 => 9550u16,
            Self::AutoAttributeValue6 => 9549u16,
            Self::AvailableSockets => 10089u16,
            Self::BlockRating => 11551u16,
            Self::ClanName => 12039u16,
            Self::CombatStyle => 4250u16,
            Self::CritDamageRating => 11555u16,
            Self::CritHitRating => 11559u16,
            Self::Disguise => 9991u16,
            Self::DisplayNameColor => 12238u16,
            Self::DisplayNameNumber => 12237u16,
            Self::DisplayNameRarity => 12239u16,
            Self::DisplayNameSlot => 12241u16,
            Self::DisplayNameStat => 12240u16,
            Self::DodgeRating => 11553u16,
            Self::Durability => 10021u16,
            Self::DurabilityCurrent => 10096u16,
            Self::Focus => 11562u16,
            Self::Gender => 11019u16,
            Self::HeavyRating => 11557u16,
            Self::HitRating => 11560u16,
            Self::IsBanked => 790u16,
            Self::IsExpired => 764u16,
            Self::IsSku => 10193u16,
            Self::IsTemplate => 9710u16,
            Self::ItemActionGenericParam => 10390u16,
            Self::LevelDropVariance => 10900u16,
            Self::MaxUse => 4814u16,
            Self::NoteCaption => 12243u16,
            Self::NoteCaptionValue => 12242u16,
            Self::OtherClientInterests => 767u16,
            Self::ParryRating => 11552u16,
            Self::PeneRating => 11558u16,
            Self::Prefix => 9709u16,
            Self::QuickbarItem => 12276u16,
            Self::RepairCost => 10101u16,
            Self::SchematicCostToCreateItem => 10106u16,
            Self::SearchKeywords => 10072u16,
            Self::SetBonuses => 12386u16,
            Self::SignOfAvatars => 12040u16,
            Self::Skuid => 10609u16,
            Self::SocketLockedStatus => 10092u16,
            Self::SocketOccupancyStatus => 10090u16,
            Self::SocketUpgradeLevel => 10091u16,
            Self::SpecialRating => 11556u16,
            Self::Stamina => 11561u16,
            Self::Strength => 11564u16,
            Self::Suffix => 9708u16,
            Self::TemplateType => 10076u16,
            Self::TemplateVersion => 11314u16,
            Self::TimeStamp => 763u16,
            Self::AddBuff => 9371u16,
            Self::ConsiderForLootTables => 12290u16,
            Self::CooldownDuration => 754u16,
            Self::ExtraData => 768u16,
            Self::Faction => 12146u16,
            Self::Level => 4941u16,
            Self::Name => 779u16,
            Self::RecipeContentReferences => 12277u16,
            Self::Ue3ClassId => 777u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AdditionalItemCount1 => "AdditionalItemCount1",
            Self::AdditionalItemCount2 => "AdditionalItemCount2",
            Self::AdditionalItemCount3 => "AdditionalItemCount3",
            Self::AdditionalItemRequired1 => "AdditionalItemRequired1",
            Self::AdditionalItemRequired2 => "AdditionalItemRequired2",
            Self::AdditionalItemRequired3 => "AdditionalItemRequired3",
            Self::AllowBuy => "AllowBuy",
            Self::AllowRent => "AllowRent",
            Self::AllowSell => "AllowSell",
            Self::BlackSomaRequired => "BlackSomaRequired",
            Self::BlingPrice => "blingPrice",
            Self::BlingSellingPrice => "blingSellingPrice",
            Self::BlueSomaRequired => "BlueSomaRequired",
            Self::BonusSlotAmber => "BonusSlotAmber",
            Self::BonusSlotRuby => "BonusSlotRuby",
            Self::BonusSlotSapphire => "BonusSlotSapphire",
            Self::BuyDiscount => "BuyDiscount",
            Self::BuyPriceBling => "BuyPriceBling",
            Self::BuyPriceGameCash => "BuyPriceGameCash",
            Self::Category => "Category",
            Self::Combos => "combos",
            Self::ContainerId => "containerID",
            Self::ContentClass => "ContentClass",
            Self::CraftingMapping => "CraftingMapping",
            Self::CraftTime => "CraftTime",
            Self::CreationTime => "creationTime",
            Self::CrystaEffects => "CrystaEffects",
            Self::CrystalType => "CrystalType",
            Self::CyanSomaRequired => "CyanSomaRequired",
            Self::Description => "Description",
            Self::DestroyMethod => "DestroyMethod",
            Self::Dialogs => "Dialogs",
            Self::DisplayName => "DisplayName",
            Self::EnableInGame => "EnableInGame",
            Self::EquipSlot => "equipSlot",
            Self::ExpireBuyBack => "expireBuyBack",
            Self::ExpireTime => "ExpireTime",
            Self::Freq => "Freq",
            Self::GameCashPrice => "gameCashPrice",
            Self::GreenSomaRequired => "GreenSomaRequired",
            Self::Icon => "Icon",
            Self::InfiniteUse => "InfiniteUse",
            Self::InitLeftTime => "InitLeftTime",
            Self::InventorySlotIndex => "inventorySlotIndex",
            Self::IsCollectFaction => "isCollectFaction",
            Self::IsEquiped => "isEquiped",
            Self::IsFactionItem => "isFactionItem",
            Self::IsGemeCrystal => "isGemeCrystal",
            Self::IsHotSeller => "IsHotSeller",
            Self::IsInGlobalShop => "isInGlobalShop",
            Self::IsInStock => "IsInStock",
            Self::IsNewToShop => "IsNewToShop",
            Self::IsQuestItem => "isQuestItem",
            Self::IsRecipe => "IsRecipe",
            Self::IsSomaSeed => "IsSomaSeed",
            Self::IsSoulBounded => "IsSoulBounded",
            Self::IsTechApproved => "isTechApproved",
            Self::IsTrialItem => "isTrialItem",
            Self::ItemCritVar => "ItemCritVar",
            Self::ItemNormalVar => "ItemNormalVar",
            Self::LastUseTime => "LastUseTime",
            Self::LeftTime => "LeftTime",
            Self::LootAction => "lootAction",
            Self::Lua => "Lua",
            Self::Lvl => "lvl",
            Self::LvlReq => "lvlReq",
            Self::MaterialOverride => "MaterialOverride",
            Self::MaxStackSize => "maxStackSize",
            Self::OrangeSomaRequired => "OrangeSomaRequired",
            Self::Power => "Power",
            Self::Quantity => "quantity",
            Self::QuestTrigger => "QuestTrigger",
            Self::Rarity => "rarity",
            Self::RedSomaRequired => "RedSomaRequired",
            Self::RentalDurationMax => "RentalDurationMax",
            Self::RentalDurationMin => "RentalDurationMin",
            Self::RentDiscount => "RentDiscount",
            Self::RentPriceBling => "RentPriceBling",
            Self::RentPriceGameCash => "RentPriceGameCash",
            Self::SellPriceBling => "SellPriceBling",
            Self::SlotId => "slotID",
            Self::SlotMapping => "SlotMapping",
            Self::SomaType => "SomaType",
            Self::SoulBoundedAccountId => "SoulBoundedAccountId",
            Self::SoulBoundedAvatarId => "SoulBoundedAvatarId",
            Self::SoulBoundedToAccount => "SoulBoundedToAccount",
            Self::SoulBoundType => "SoulBoundType",
            Self::StackCount => "stackCount",
            Self::StandingReq => "standingReq",
            Self::UseAction => "useAction",
            Self::UseCoolDownTimer => "UseCoolDownTimer",
            Self::UseCount => "UseCount",
            Self::UseMaxCount => "UseMaxCount",
            Self::UseRequireAvatar => "UseRequireAvatar",
            Self::UseRequireAvatarWithinRadius => "UseRequireAvatarWithinRadius",
            Self::UseRequireTarget => "UseRequireTarget",
            Self::UseScript => "UseScript",
            Self::Vendorable => "Vendorable",
            Self::VendorAction => "vendorAction",
            Self::VioletSomaRequired => "VioletSomaRequired",
            Self::YellowSomaRequired => "YellowSomaRequired",
            Self::Abilities => "abilities",
            Self::AbilityInstanceData => "abilityInstanceData",
            Self::Agility => "Agility",
            Self::Armor => "Armor",
            Self::AttackPowerRating => "AttackPowerRating",
            Self::AttributeOp1 => "attributeOp1",
            Self::AttributeOp2 => "attributeOp2",
            Self::AttributeOp3 => "attributeOp3",
            Self::AttributeOp4 => "attributeOp4",
            Self::AttributeType1 => "attributeType1",
            Self::AttributeType2 => "attributeType2",
            Self::AttributeType3 => "attributeType3",
            Self::AttributeType4 => "attributeType4",
            Self::AttributeWeight1 => "attributeWeight1",
            Self::AttributeWeight2 => "attributeWeight2",
            Self::AttributeWeight3 => "attributeWeight3",
            Self::AttributeWeight4 => "attributeWeight4",
            Self::AutoAttributeType1 => "autoAttributeType1",
            Self::AutoAttributeType2 => "autoAttributeType2",
            Self::AutoAttributeType3 => "autoAttributeType3",
            Self::AutoAttributeType4 => "autoAttributeType4",
            Self::AutoAttributeType5 => "autoAttributeType5",
            Self::AutoAttributeType6 => "autoAttributeType6",
            Self::AutoAttributeValue1 => "autoAttributeValue1",
            Self::AutoAttributeValue2 => "autoAttributeValue2",
            Self::AutoAttributeValue3 => "autoAttributeValue3",
            Self::AutoAttributeValue4 => "autoAttributeValue4",
            Self::AutoAttributeValue5 => "autoAttributeValue5",
            Self::AutoAttributeValue6 => "autoAttributeValue6",
            Self::AvailableSockets => "availableSockets",
            Self::BlockRating => "BlockRating",
            Self::ClanName => "ClanName",
            Self::CombatStyle => "combatStyle",
            Self::CritDamageRating => "CritDamageRating",
            Self::CritHitRating => "CritHitRating",
            Self::Disguise => "disguise",
            Self::DisplayNameColor => "DisplayName_Color",
            Self::DisplayNameNumber => "DisplayName_Number",
            Self::DisplayNameRarity => "DisplayName_Rarity",
            Self::DisplayNameSlot => "DisplayName_Slot",
            Self::DisplayNameStat => "DisplayName_Stat",
            Self::DodgeRating => "DodgeRating",
            Self::Durability => "durability",
            Self::DurabilityCurrent => "durabilityCurrent",
            Self::Focus => "Focus",
            Self::Gender => "gender",
            Self::HeavyRating => "HeavyRating",
            Self::HitRating => "HitRating",
            Self::IsBanked => "isBanked",
            Self::IsExpired => "isExpired",
            Self::IsSku => "isSKU",
            Self::IsTemplate => "IsTemplate",
            Self::ItemActionGenericParam => "itemActionGenericParam",
            Self::LevelDropVariance => "levelDropVariance",
            Self::MaxUse => "MaxUse",
            Self::NoteCaption => "NoteCaption",
            Self::NoteCaptionValue => "NoteCaptionValue",
            Self::OtherClientInterests => "otherClientInterests",
            Self::ParryRating => "ParryRating",
            Self::PeneRating => "PeneRating",
            Self::Prefix => "Prefix",
            Self::QuickbarItem => "QuickbarItem",
            Self::RepairCost => "repairCost",
            Self::SchematicCostToCreateItem => "schematic_CostToCreateItem",
            Self::SearchKeywords => "searchKeywords",
            Self::SetBonuses => "setBonuses",
            Self::SignOfAvatars => "SignOfAvatars",
            Self::Skuid => "SKUID",
            Self::SocketLockedStatus => "socketLockedStatus",
            Self::SocketOccupancyStatus => "socketOccupancyStatus",
            Self::SocketUpgradeLevel => "socketUpgradeLevel",
            Self::SpecialRating => "SpecialRating",
            Self::Stamina => "Stamina",
            Self::Strength => "Strength",
            Self::Suffix => "Suffix",
            Self::TemplateType => "templateType",
            Self::TemplateVersion => "templateVersion",
            Self::TimeStamp => "timeStamp",
            Self::AddBuff => "AddBuff",
            Self::ConsiderForLootTables => "considerForLootTables",
            Self::CooldownDuration => "cooldownDuration",
            Self::ExtraData => "extraData",
            Self::Faction => "Faction",
            Self::Level => "level",
            Self::Name => "name",
            Self::RecipeContentReferences => "RecipeContentReferences",
            Self::Ue3ClassId => "UE3ClassID",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::EquipSlot => ParamType::String,
            Self::QuickbarItem => ParamType::Bool,
            Self::AddBuff => ParamType::ContentRef,
            Self::ConsiderForLootTables => ParamType::Bool,
            Self::CooldownDuration => ParamType::String,
            Self::ExtraData => ParamType::String,
            Self::Faction => ParamType::ContentRefList,
            Self::Level => ParamType::Int,
            Self::Name => ParamType::String,
            Self::RecipeContentReferences => ParamType::JsonValue,
            Self::Ue3ClassId => ParamType::String,
            Self::AdditionalItemCount1 => ParamType::Int,
            Self::AdditionalItemCount2 => ParamType::Int,
            Self::AdditionalItemCount3 => ParamType::Int,
            Self::AdditionalItemRequired1 => ParamType::Guid,
            Self::AdditionalItemRequired2 => ParamType::Guid,
            Self::AdditionalItemRequired3 => ParamType::Guid,
            Self::AllowBuy => ParamType::Bool,
            Self::AllowRent => ParamType::Bool,
            Self::AllowSell => ParamType::Bool,
            Self::BlackSomaRequired => ParamType::Int,
            Self::BlingPrice => ParamType::Int,
            Self::BlingSellingPrice => ParamType::Int,
            Self::BlueSomaRequired => ParamType::Int,
            Self::BonusSlotAmber => ParamType::Guid,
            Self::BonusSlotRuby => ParamType::Guid,
            Self::BonusSlotSapphire => ParamType::Guid,
            Self::BuyDiscount => ParamType::Float,
            Self::BuyPriceBling => ParamType::Int,
            Self::BuyPriceGameCash => ParamType::Int,
            Self::Category => ParamType::Guid,
            Self::Combos => ParamType::JsonValue,
            Self::ContainerId => ParamType::Int,
            Self::ContentClass => ParamType::String,
            Self::CraftingMapping => ParamType::String,
            Self::CraftTime => ParamType::Float,
            Self::CreationTime => ParamType::Int64,
            Self::CrystaEffects => ParamType::JsonValue,
            Self::CrystalType => ParamType::String,
            Self::CyanSomaRequired => ParamType::Int,
            Self::Description => ParamType::LocalizedString,
            Self::DestroyMethod => ParamType::String,
            Self::Dialogs => ParamType::VectorInt,
            Self::DisplayName => ParamType::LocalizedString,
            Self::EnableInGame => ParamType::Bool,
            Self::ExpireBuyBack => ParamType::Int64,
            Self::ExpireTime => ParamType::Int64,
            Self::Freq => ParamType::Int,
            Self::GameCashPrice => ParamType::Int,
            Self::GreenSomaRequired => ParamType::Int,
            Self::Icon => ParamType::String,
            Self::InfiniteUse => ParamType::Bool,
            Self::InitLeftTime => ParamType::Int,
            Self::InventorySlotIndex => ParamType::Int,
            Self::IsCollectFaction => ParamType::Bool,
            Self::IsEquiped => ParamType::Bool,
            Self::IsFactionItem => ParamType::Bool,
            Self::IsGemeCrystal => ParamType::Bool,
            Self::IsHotSeller => ParamType::Bool,
            Self::IsInGlobalShop => ParamType::Bool,
            Self::IsInStock => ParamType::Bool,
            Self::IsNewToShop => ParamType::Bool,
            Self::IsQuestItem => ParamType::Bool,
            Self::IsRecipe => ParamType::Bool,
            Self::IsSomaSeed => ParamType::Bool,
            Self::IsSoulBounded => ParamType::Bool,
            Self::IsTechApproved => ParamType::Bool,
            Self::IsTrialItem => ParamType::Bool,
            Self::ItemCritVar => ParamType::Guid,
            Self::ItemNormalVar => ParamType::Guid,
            Self::LastUseTime => ParamType::Int64,
            Self::LeftTime => ParamType::Int64,
            Self::LootAction => ParamType::String,
            Self::Lua => ParamType::String,
            Self::Lvl => ParamType::Int,
            Self::LvlReq => ParamType::Int,
            Self::MaterialOverride => ParamType::Int,
            Self::MaxStackSize => ParamType::Int,
            Self::OrangeSomaRequired => ParamType::Int,
            Self::Power => ParamType::Int,
            Self::Quantity => ParamType::Int,
            Self::QuestTrigger => ParamType::Int,
            Self::Rarity => ParamType::String,
            Self::RedSomaRequired => ParamType::Int,
            Self::RentalDurationMax => ParamType::Float,
            Self::RentalDurationMin => ParamType::Float,
            Self::RentDiscount => ParamType::Float,
            Self::RentPriceBling => ParamType::Float,
            Self::RentPriceGameCash => ParamType::Float,
            Self::SellPriceBling => ParamType::Int,
            Self::SlotId => ParamType::Int,
            Self::SlotMapping => ParamType::String,
            Self::SomaType => ParamType::Int,
            Self::SoulBoundedAccountId => ParamType::Int,
            Self::SoulBoundedAvatarId => ParamType::AvatarId,
            Self::SoulBoundedToAccount => ParamType::Bool,
            Self::SoulBoundType => ParamType::String,
            Self::StackCount => ParamType::Int,
            Self::StandingReq => ParamType::Int,
            Self::UseAction => ParamType::String,
            Self::UseCoolDownTimer => ParamType::Int,
            Self::UseCount => ParamType::Int,
            Self::UseMaxCount => ParamType::Int,
            Self::UseRequireAvatar => ParamType::ContentRef,
            Self::UseRequireAvatarWithinRadius => ParamType::Float,
            Self::UseRequireTarget => ParamType::ContentRef,
            Self::UseScript => ParamType::String,
            Self::Vendorable => ParamType::Bool,
            Self::VendorAction => ParamType::String,
            Self::VioletSomaRequired => ParamType::Int,
            Self::YellowSomaRequired => ParamType::Int,
            Self::Abilities => ParamType::JsonValue,
            Self::AbilityInstanceData => ParamType::JsonValue,
            Self::Agility => ParamType::Float,
            Self::Armor => ParamType::Float,
            Self::AttackPowerRating => ParamType::Float,
            Self::AttributeOp1 => ParamType::String,
            Self::AttributeOp2 => ParamType::String,
            Self::AttributeOp3 => ParamType::String,
            Self::AttributeOp4 => ParamType::String,
            Self::AttributeType1 => ParamType::String,
            Self::AttributeType2 => ParamType::String,
            Self::AttributeType3 => ParamType::String,
            Self::AttributeType4 => ParamType::String,
            Self::AttributeWeight1 => ParamType::Float,
            Self::AttributeWeight2 => ParamType::Float,
            Self::AttributeWeight3 => ParamType::Float,
            Self::AttributeWeight4 => ParamType::Float,
            Self::AutoAttributeType1 => ParamType::String,
            Self::AutoAttributeType2 => ParamType::String,
            Self::AutoAttributeType3 => ParamType::String,
            Self::AutoAttributeType4 => ParamType::String,
            Self::AutoAttributeType5 => ParamType::String,
            Self::AutoAttributeType6 => ParamType::String,
            Self::AutoAttributeValue1 => ParamType::Float,
            Self::AutoAttributeValue2 => ParamType::Float,
            Self::AutoAttributeValue3 => ParamType::Float,
            Self::AutoAttributeValue4 => ParamType::Float,
            Self::AutoAttributeValue5 => ParamType::Float,
            Self::AutoAttributeValue6 => ParamType::Float,
            Self::AvailableSockets => ParamType::VectorString,
            Self::BlockRating => ParamType::Float,
            Self::ClanName => ParamType::String,
            Self::CombatStyle => ParamType::Int,
            Self::CritDamageRating => ParamType::Float,
            Self::CritHitRating => ParamType::Float,
            Self::Disguise => ParamType::Int,
            Self::DisplayNameColor => ParamType::LocalizedString,
            Self::DisplayNameNumber => ParamType::String,
            Self::DisplayNameRarity => ParamType::LocalizedString,
            Self::DisplayNameSlot => ParamType::LocalizedString,
            Self::DisplayNameStat => ParamType::LocalizedString,
            Self::DodgeRating => ParamType::Float,
            Self::Durability => ParamType::Float,
            Self::DurabilityCurrent => ParamType::Float,
            Self::Focus => ParamType::Float,
            Self::Gender => ParamType::String,
            Self::HeavyRating => ParamType::Float,
            Self::HitRating => ParamType::Float,
            Self::IsBanked => ParamType::Bool,
            Self::IsExpired => ParamType::Bool,
            Self::IsSku => ParamType::Bool,
            Self::IsTemplate => ParamType::Bool,
            Self::ItemActionGenericParam => ParamType::Any,
            Self::LevelDropVariance => ParamType::Int,
            Self::MaxUse => ParamType::Int,
            Self::NoteCaption => ParamType::LocalizedString,
            Self::NoteCaptionValue => ParamType::String,
            Self::OtherClientInterests => ParamType::AvatarIdSet,
            Self::ParryRating => ParamType::Float,
            Self::PeneRating => ParamType::Float,
            Self::Prefix => ParamType::String,
            Self::RepairCost => ParamType::Int,
            Self::SchematicCostToCreateItem => ParamType::VectorInt,
            Self::SearchKeywords => ParamType::VectorLocalizedString,
            Self::SetBonuses => ParamType::String,
            Self::SignOfAvatars => ParamType::VectorInt,
            Self::Skuid => ParamType::String,
            Self::SocketLockedStatus => ParamType::VectorInt,
            Self::SocketOccupancyStatus => ParamType::VectorGuid,
            Self::SocketUpgradeLevel => ParamType::VectorInt,
            Self::SpecialRating => ParamType::Float,
            Self::Stamina => ParamType::Float,
            Self::Strength => ParamType::Float,
            Self::Suffix => ParamType::String,
            Self::TemplateType => ParamType::Int,
            Self::TemplateVersion => ParamType::Int,
            Self::TimeStamp => ParamType::Int64,
        }
    }
    fn default(&self) -> &'static Value {
        static EQUIP_SLOT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static QUICKBAR_ITEM: Value = Value::Bool(false);
        static ADD_BUFF: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static CONSIDER_FOR_LOOT_TABLES: Value = Value::Bool(true);
        static COOLDOWN_DURATION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static EXTRA_DATA: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static FACTION: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static LEVEL: Value = Value::Int(3i32);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static RECIPE_CONTENT_REFERENCES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ADDITIONAL_ITEM_COUNT_1: Value = Value::Int(0i32);
        static ADDITIONAL_ITEM_COUNT_2: Value = Value::Int(0i32);
        static ADDITIONAL_ITEM_COUNT_3: Value = Value::Int(0i32);
        static ADDITIONAL_ITEM_REQUIRED_1: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ADDITIONAL_ITEM_REQUIRED_2: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ADDITIONAL_ITEM_REQUIRED_3: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ALLOW_BUY: Value = Value::Bool(true);
        static ALLOW_RENT: Value = Value::Bool(true);
        static ALLOW_SELL: Value = Value::Bool(false);
        static BLACK_SOMA_REQUIRED: Value = Value::Int(0i32);
        static BLING_PRICE: Value = Value::Int(0i32);
        static BLING_SELLING_PRICE: Value = Value::Int(0i32);
        static BLUE_SOMA_REQUIRED: Value = Value::Int(0i32);
        static BONUS_SLOT_AMBER: Value = Value::Guid(
            Uuid::from_bytes([
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
            ]),
        );
        static BONUS_SLOT_RUBY: Value = Value::Guid(
            Uuid::from_bytes([
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
            ]),
        );
        static BONUS_SLOT_SAPPHIRE: Value = Value::Guid(
            Uuid::from_bytes([
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
            ]),
        );
        static BUY_DISCOUNT: Value = Value::Float(0f32);
        static BUY_PRICE_BLING: Value = Value::Int(0i32);
        static BUY_PRICE_GAME_CASH: Value = Value::Int(0i32);
        static CATEGORY: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static COMBOS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static CONTAINER_ID: Value = Value::Int(-1i32);
        static CONTENT_CLASS: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CRAFTING_MAPPING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CRAFT_TIME: Value = Value::Float(0f32);
        static CREATION_TIME: Value = Value::Int64(0i64);
        static CRYSTA_EFFECTS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static CRYSTAL_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CYAN_SOMA_REQUIRED: Value = Value::Int(0i32);
        static DESCRIPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DESTROY_METHOD: Lazy<Value> = Lazy::new(|| Value::String(
            "CannotDestroy".to_string(),
        ));
        static DIALOGS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static EXPIRE_BUY_BACK: Value = Value::Int64(0i64);
        static EXPIRE_TIME: Value = Value::Int64(0i64);
        static FREQ: Value = Value::Int(0i32);
        static GAME_CASH_PRICE: Value = Value::Int(0i32);
        static GREEN_SOMA_REQUIRED: Value = Value::Int(0i32);
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static INFINITE_USE: Value = Value::Bool(false);
        static INIT_LEFT_TIME: Value = Value::Int(0i32);
        static INVENTORY_SLOT_INDEX: Value = Value::Int(-1i32);
        static IS_COLLECT_FACTION: Value = Value::Bool(false);
        static IS_EQUIPED: Value = Value::Bool(false);
        static IS_FACTION_ITEM: Value = Value::Bool(false);
        static IS_GEME_CRYSTAL: Value = Value::Bool(false);
        static IS_HOT_SELLER: Value = Value::Bool(false);
        static IS_IN_GLOBAL_SHOP: Value = Value::Bool(true);
        static IS_IN_STOCK: Value = Value::Bool(false);
        static IS_NEW_TO_SHOP: Value = Value::Bool(false);
        static IS_QUEST_ITEM: Value = Value::Bool(false);
        static IS_RECIPE: Value = Value::Bool(false);
        static IS_SOMA_SEED: Value = Value::Bool(false);
        static IS_SOUL_BOUNDED: Value = Value::Bool(false);
        static IS_TECH_APPROVED: Value = Value::Bool(true);
        static IS_TRIAL_ITEM: Value = Value::Bool(false);
        static ITEM_CRIT_VAR: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static ITEM_NORMAL_VAR: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static LAST_USE_TIME: Value = Value::Int64(0i64);
        static LEFT_TIME: Value = Value::Int64(0i64);
        static LOOT_ACTION: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static LUA: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static LVL: Value = Value::Int(1i32);
        static LVL_REQ: Value = Value::Int(-1i32);
        static MATERIAL_OVERRIDE: Value = Value::Int(0i32);
        static MAX_STACK_SIZE: Value = Value::Int(1i32);
        static ORANGE_SOMA_REQUIRED: Value = Value::Int(0i32);
        static POWER: Value = Value::Int(0i32);
        static QUANTITY: Value = Value::Int(1i32);
        static QUEST_TRIGGER: Value = Value::Int(0i32);
        static RARITY: Lazy<Value> = Lazy::new(|| Value::String("Normal".to_string()));
        static RED_SOMA_REQUIRED: Value = Value::Int(0i32);
        static RENTAL_DURATION_MAX: Value = Value::Float(30f32);
        static RENTAL_DURATION_MIN: Value = Value::Float(0f32);
        static RENT_DISCOUNT: Value = Value::Float(1f32);
        static RENT_PRICE_BLING: Value = Value::Float(0f32);
        static RENT_PRICE_GAME_CASH: Value = Value::Float(0f32);
        static SELL_PRICE_BLING: Value = Value::Int(0i32);
        static SLOT_ID: Value = Value::Int(-1i32);
        static SLOT_MAPPING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SOMA_TYPE: Value = Value::Int(0i32);
        static SOUL_BOUNDED_ACCOUNT_ID: Value = Value::Int(0i32);
        static SOUL_BOUNDED_AVATAR_ID: Value = Value::AvatarId(AvatarId::from_u64(0));
        static SOUL_BOUNDED_TO_ACCOUNT: Value = Value::Bool(false);
        static SOUL_BOUND_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static STACK_COUNT: Value = Value::Int(1i32);
        static STANDING_REQ: Value = Value::Int(-1i32);
        static USE_ACTION: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static USE_COOL_DOWN_TIMER: Value = Value::Int(1i32);
        static USE_COUNT: Value = Value::Int(0i32);
        static USE_MAX_COUNT: Value = Value::Int(1i32);
        static USE_REQUIRE_AVATAR: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static USE_REQUIRE_AVATAR_WITHIN_RADIUS: Value = Value::Float(0f32);
        static USE_REQUIRE_TARGET: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static USE_SCRIPT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static VENDORABLE: Value = Value::Bool(false);
        static VENDOR_ACTION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static VIOLET_SOMA_REQUIRED: Value = Value::Int(0i32);
        static YELLOW_SOMA_REQUIRED: Value = Value::Int(0i32);
        static ABILITIES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("[]").unwrap(),
        ));
        static ABILITY_INSTANCE_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            serde_json::from_str("{}").unwrap(),
        ));
        static AGILITY: Value = Value::Float(0f32);
        static ARMOR: Value = Value::Float(0f32);
        static ATTACK_POWER_RATING: Value = Value::Float(0f32);
        static ATTRIBUTE_OP_1: Lazy<Value> = Lazy::new(|| Value::String(
            "Add".to_string(),
        ));
        static ATTRIBUTE_OP_2: Lazy<Value> = Lazy::new(|| Value::String(
            "Add".to_string(),
        ));
        static ATTRIBUTE_OP_3: Lazy<Value> = Lazy::new(|| Value::String(
            "Add".to_string(),
        ));
        static ATTRIBUTE_OP_4: Lazy<Value> = Lazy::new(|| Value::String(
            "Add".to_string(),
        ));
        static ATTRIBUTE_TYPE_1: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_2: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_3: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_TYPE_4: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static ATTRIBUTE_WEIGHT_1: Value = Value::Float(1f32);
        static ATTRIBUTE_WEIGHT_2: Value = Value::Float(1f32);
        static ATTRIBUTE_WEIGHT_3: Value = Value::Float(1f32);
        static ATTRIBUTE_WEIGHT_4: Value = Value::Float(1f32);
        static AUTO_ATTRIBUTE_TYPE_1: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_TYPE_2: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_TYPE_3: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_TYPE_4: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_TYPE_5: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_TYPE_6: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static AUTO_ATTRIBUTE_VALUE_1: Value = Value::Float(0f32);
        static AUTO_ATTRIBUTE_VALUE_2: Value = Value::Float(0f32);
        static AUTO_ATTRIBUTE_VALUE_3: Value = Value::Float(0f32);
        static AUTO_ATTRIBUTE_VALUE_4: Value = Value::Float(0f32);
        static AUTO_ATTRIBUTE_VALUE_5: Value = Value::Float(0f32);
        static AUTO_ATTRIBUTE_VALUE_6: Value = Value::Float(0f32);
        static AVAILABLE_SOCKETS: Lazy<Value> = Lazy::new(|| Value::VectorString(
            vec![],
        ));
        static BLOCK_RATING: Value = Value::Float(0f32);
        static CLAN_NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static COMBAT_STYLE: Value = Value::Int(-1i32);
        static CRIT_DAMAGE_RATING: Value = Value::Float(0f32);
        static CRIT_HIT_RATING: Value = Value::Float(0f32);
        static DISGUISE: Value = Value::Int(1i32);
        static DISPLAY_NAME_COLOR: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME_NUMBER: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static DISPLAY_NAME_RARITY: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME_SLOT: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME_STAT: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DODGE_RATING: Value = Value::Float(0f32);
        static DURABILITY: Value = Value::Float(0f32);
        static DURABILITY_CURRENT: Value = Value::Float(-1f32);
        static FOCUS: Value = Value::Float(0f32);
        static GENDER: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static HEAVY_RATING: Value = Value::Float(0f32);
        static HIT_RATING: Value = Value::Float(0f32);
        static IS_BANKED: Value = Value::Bool(false);
        static IS_EXPIRED: Value = Value::Bool(false);
        static IS_SKU: Value = Value::Bool(false);
        static IS_TEMPLATE: Value = Value::Bool(false);
        static ITEM_ACTION_GENERIC_PARAM: Value = Value::Any(vec![]);
        static LEVEL_DROP_VARIANCE: Value = Value::Int(1i32);
        static MAX_USE: Value = Value::Int(0i32);
        static NOTE_CAPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static NOTE_CAPTION_VALUE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static OTHER_CLIENT_INTERESTS: Lazy<Value> = Lazy::new(|| Value::AvatarIdSet(
            HashSet::new(),
        ));
        static PARRY_RATING: Value = Value::Float(0f32);
        static PENE_RATING: Value = Value::Float(0f32);
        static PREFIX: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static REPAIR_COST: Value = Value::Int(0i32);
        static SCHEMATIC_COST_TO_CREATE_ITEM: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
        ));
        static SEARCH_KEYWORDS: Value = Value::VectorLocalizedString(vec![]);
        static SET_BONUSES: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SIGN_OF_AVATARS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static SKUID: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SOCKET_LOCKED_STATUS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static SOCKET_OCCUPANCY_STATUS: Value = Value::VectorGuid(vec![]);
        static SOCKET_UPGRADE_LEVEL: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static SPECIAL_RATING: Value = Value::Float(0f32);
        static STAMINA: Value = Value::Float(0f32);
        static STRENGTH: Value = Value::Float(0f32);
        static SUFFIX: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static TEMPLATE_TYPE: Value = Value::Int(0i32);
        static TEMPLATE_VERSION: Value = Value::Int(0i32);
        static TIME_STAMP: Value = Value::Int64(0i64);
        match self {
            Self::EquipSlot => &EQUIP_SLOT,
            Self::QuickbarItem => &QUICKBAR_ITEM,
            Self::AddBuff => &ADD_BUFF,
            Self::ConsiderForLootTables => &CONSIDER_FOR_LOOT_TABLES,
            Self::CooldownDuration => &COOLDOWN_DURATION,
            Self::ExtraData => &EXTRA_DATA,
            Self::Faction => &FACTION,
            Self::Level => &LEVEL,
            Self::Name => &NAME,
            Self::RecipeContentReferences => &RECIPE_CONTENT_REFERENCES,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::AdditionalItemCount1 => &ADDITIONAL_ITEM_COUNT_1,
            Self::AdditionalItemCount2 => &ADDITIONAL_ITEM_COUNT_2,
            Self::AdditionalItemCount3 => &ADDITIONAL_ITEM_COUNT_3,
            Self::AdditionalItemRequired1 => &ADDITIONAL_ITEM_REQUIRED_1,
            Self::AdditionalItemRequired2 => &ADDITIONAL_ITEM_REQUIRED_2,
            Self::AdditionalItemRequired3 => &ADDITIONAL_ITEM_REQUIRED_3,
            Self::AllowBuy => &ALLOW_BUY,
            Self::AllowRent => &ALLOW_RENT,
            Self::AllowSell => &ALLOW_SELL,
            Self::BlackSomaRequired => &BLACK_SOMA_REQUIRED,
            Self::BlingPrice => &BLING_PRICE,
            Self::BlingSellingPrice => &BLING_SELLING_PRICE,
            Self::BlueSomaRequired => &BLUE_SOMA_REQUIRED,
            Self::BonusSlotAmber => &BONUS_SLOT_AMBER,
            Self::BonusSlotRuby => &BONUS_SLOT_RUBY,
            Self::BonusSlotSapphire => &BONUS_SLOT_SAPPHIRE,
            Self::BuyDiscount => &BUY_DISCOUNT,
            Self::BuyPriceBling => &BUY_PRICE_BLING,
            Self::BuyPriceGameCash => &BUY_PRICE_GAME_CASH,
            Self::Category => &CATEGORY,
            Self::Combos => &COMBOS,
            Self::ContainerId => &CONTAINER_ID,
            Self::ContentClass => &CONTENT_CLASS,
            Self::CraftingMapping => &CRAFTING_MAPPING,
            Self::CraftTime => &CRAFT_TIME,
            Self::CreationTime => &CREATION_TIME,
            Self::CrystaEffects => &CRYSTA_EFFECTS,
            Self::CrystalType => &CRYSTAL_TYPE,
            Self::CyanSomaRequired => &CYAN_SOMA_REQUIRED,
            Self::Description => &DESCRIPTION,
            Self::DestroyMethod => &DESTROY_METHOD,
            Self::Dialogs => &DIALOGS,
            Self::DisplayName => &DISPLAY_NAME,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::ExpireBuyBack => &EXPIRE_BUY_BACK,
            Self::ExpireTime => &EXPIRE_TIME,
            Self::Freq => &FREQ,
            Self::GameCashPrice => &GAME_CASH_PRICE,
            Self::GreenSomaRequired => &GREEN_SOMA_REQUIRED,
            Self::Icon => &ICON,
            Self::InfiniteUse => &INFINITE_USE,
            Self::InitLeftTime => &INIT_LEFT_TIME,
            Self::InventorySlotIndex => &INVENTORY_SLOT_INDEX,
            Self::IsCollectFaction => &IS_COLLECT_FACTION,
            Self::IsEquiped => &IS_EQUIPED,
            Self::IsFactionItem => &IS_FACTION_ITEM,
            Self::IsGemeCrystal => &IS_GEME_CRYSTAL,
            Self::IsHotSeller => &IS_HOT_SELLER,
            Self::IsInGlobalShop => &IS_IN_GLOBAL_SHOP,
            Self::IsInStock => &IS_IN_STOCK,
            Self::IsNewToShop => &IS_NEW_TO_SHOP,
            Self::IsQuestItem => &IS_QUEST_ITEM,
            Self::IsRecipe => &IS_RECIPE,
            Self::IsSomaSeed => &IS_SOMA_SEED,
            Self::IsSoulBounded => &IS_SOUL_BOUNDED,
            Self::IsTechApproved => &IS_TECH_APPROVED,
            Self::IsTrialItem => &IS_TRIAL_ITEM,
            Self::ItemCritVar => &ITEM_CRIT_VAR,
            Self::ItemNormalVar => &ITEM_NORMAL_VAR,
            Self::LastUseTime => &LAST_USE_TIME,
            Self::LeftTime => &LEFT_TIME,
            Self::LootAction => &LOOT_ACTION,
            Self::Lua => &LUA,
            Self::Lvl => &LVL,
            Self::LvlReq => &LVL_REQ,
            Self::MaterialOverride => &MATERIAL_OVERRIDE,
            Self::MaxStackSize => &MAX_STACK_SIZE,
            Self::OrangeSomaRequired => &ORANGE_SOMA_REQUIRED,
            Self::Power => &POWER,
            Self::Quantity => &QUANTITY,
            Self::QuestTrigger => &QUEST_TRIGGER,
            Self::Rarity => &RARITY,
            Self::RedSomaRequired => &RED_SOMA_REQUIRED,
            Self::RentalDurationMax => &RENTAL_DURATION_MAX,
            Self::RentalDurationMin => &RENTAL_DURATION_MIN,
            Self::RentDiscount => &RENT_DISCOUNT,
            Self::RentPriceBling => &RENT_PRICE_BLING,
            Self::RentPriceGameCash => &RENT_PRICE_GAME_CASH,
            Self::SellPriceBling => &SELL_PRICE_BLING,
            Self::SlotId => &SLOT_ID,
            Self::SlotMapping => &SLOT_MAPPING,
            Self::SomaType => &SOMA_TYPE,
            Self::SoulBoundedAccountId => &SOUL_BOUNDED_ACCOUNT_ID,
            Self::SoulBoundedAvatarId => &SOUL_BOUNDED_AVATAR_ID,
            Self::SoulBoundedToAccount => &SOUL_BOUNDED_TO_ACCOUNT,
            Self::SoulBoundType => &SOUL_BOUND_TYPE,
            Self::StackCount => &STACK_COUNT,
            Self::StandingReq => &STANDING_REQ,
            Self::UseAction => &USE_ACTION,
            Self::UseCoolDownTimer => &USE_COOL_DOWN_TIMER,
            Self::UseCount => &USE_COUNT,
            Self::UseMaxCount => &USE_MAX_COUNT,
            Self::UseRequireAvatar => &USE_REQUIRE_AVATAR,
            Self::UseRequireAvatarWithinRadius => &USE_REQUIRE_AVATAR_WITHIN_RADIUS,
            Self::UseRequireTarget => &USE_REQUIRE_TARGET,
            Self::UseScript => &USE_SCRIPT,
            Self::Vendorable => &VENDORABLE,
            Self::VendorAction => &VENDOR_ACTION,
            Self::VioletSomaRequired => &VIOLET_SOMA_REQUIRED,
            Self::YellowSomaRequired => &YELLOW_SOMA_REQUIRED,
            Self::Abilities => &ABILITIES,
            Self::AbilityInstanceData => &ABILITY_INSTANCE_DATA,
            Self::Agility => &AGILITY,
            Self::Armor => &ARMOR,
            Self::AttackPowerRating => &ATTACK_POWER_RATING,
            Self::AttributeOp1 => &ATTRIBUTE_OP_1,
            Self::AttributeOp2 => &ATTRIBUTE_OP_2,
            Self::AttributeOp3 => &ATTRIBUTE_OP_3,
            Self::AttributeOp4 => &ATTRIBUTE_OP_4,
            Self::AttributeType1 => &ATTRIBUTE_TYPE_1,
            Self::AttributeType2 => &ATTRIBUTE_TYPE_2,
            Self::AttributeType3 => &ATTRIBUTE_TYPE_3,
            Self::AttributeType4 => &ATTRIBUTE_TYPE_4,
            Self::AttributeWeight1 => &ATTRIBUTE_WEIGHT_1,
            Self::AttributeWeight2 => &ATTRIBUTE_WEIGHT_2,
            Self::AttributeWeight3 => &ATTRIBUTE_WEIGHT_3,
            Self::AttributeWeight4 => &ATTRIBUTE_WEIGHT_4,
            Self::AutoAttributeType1 => &AUTO_ATTRIBUTE_TYPE_1,
            Self::AutoAttributeType2 => &AUTO_ATTRIBUTE_TYPE_2,
            Self::AutoAttributeType3 => &AUTO_ATTRIBUTE_TYPE_3,
            Self::AutoAttributeType4 => &AUTO_ATTRIBUTE_TYPE_4,
            Self::AutoAttributeType5 => &AUTO_ATTRIBUTE_TYPE_5,
            Self::AutoAttributeType6 => &AUTO_ATTRIBUTE_TYPE_6,
            Self::AutoAttributeValue1 => &AUTO_ATTRIBUTE_VALUE_1,
            Self::AutoAttributeValue2 => &AUTO_ATTRIBUTE_VALUE_2,
            Self::AutoAttributeValue3 => &AUTO_ATTRIBUTE_VALUE_3,
            Self::AutoAttributeValue4 => &AUTO_ATTRIBUTE_VALUE_4,
            Self::AutoAttributeValue5 => &AUTO_ATTRIBUTE_VALUE_5,
            Self::AutoAttributeValue6 => &AUTO_ATTRIBUTE_VALUE_6,
            Self::AvailableSockets => &AVAILABLE_SOCKETS,
            Self::BlockRating => &BLOCK_RATING,
            Self::ClanName => &CLAN_NAME,
            Self::CombatStyle => &COMBAT_STYLE,
            Self::CritDamageRating => &CRIT_DAMAGE_RATING,
            Self::CritHitRating => &CRIT_HIT_RATING,
            Self::Disguise => &DISGUISE,
            Self::DisplayNameColor => &DISPLAY_NAME_COLOR,
            Self::DisplayNameNumber => &DISPLAY_NAME_NUMBER,
            Self::DisplayNameRarity => &DISPLAY_NAME_RARITY,
            Self::DisplayNameSlot => &DISPLAY_NAME_SLOT,
            Self::DisplayNameStat => &DISPLAY_NAME_STAT,
            Self::DodgeRating => &DODGE_RATING,
            Self::Durability => &DURABILITY,
            Self::DurabilityCurrent => &DURABILITY_CURRENT,
            Self::Focus => &FOCUS,
            Self::Gender => &GENDER,
            Self::HeavyRating => &HEAVY_RATING,
            Self::HitRating => &HIT_RATING,
            Self::IsBanked => &IS_BANKED,
            Self::IsExpired => &IS_EXPIRED,
            Self::IsSku => &IS_SKU,
            Self::IsTemplate => &IS_TEMPLATE,
            Self::ItemActionGenericParam => &ITEM_ACTION_GENERIC_PARAM,
            Self::LevelDropVariance => &LEVEL_DROP_VARIANCE,
            Self::MaxUse => &MAX_USE,
            Self::NoteCaption => &NOTE_CAPTION,
            Self::NoteCaptionValue => &NOTE_CAPTION_VALUE,
            Self::OtherClientInterests => &OTHER_CLIENT_INTERESTS,
            Self::ParryRating => &PARRY_RATING,
            Self::PeneRating => &PENE_RATING,
            Self::Prefix => &PREFIX,
            Self::RepairCost => &REPAIR_COST,
            Self::SchematicCostToCreateItem => &SCHEMATIC_COST_TO_CREATE_ITEM,
            Self::SearchKeywords => &SEARCH_KEYWORDS,
            Self::SetBonuses => &SET_BONUSES,
            Self::SignOfAvatars => &SIGN_OF_AVATARS,
            Self::Skuid => &SKUID,
            Self::SocketLockedStatus => &SOCKET_LOCKED_STATUS,
            Self::SocketOccupancyStatus => &SOCKET_OCCUPANCY_STATUS,
            Self::SocketUpgradeLevel => &SOCKET_UPGRADE_LEVEL,
            Self::SpecialRating => &SPECIAL_RATING,
            Self::Stamina => &STAMINA,
            Self::Strength => &STRENGTH,
            Self::Suffix => &SUFFIX,
            Self::TemplateType => &TEMPLATE_TYPE,
            Self::TemplateVersion => &TEMPLATE_VERSION,
            Self::TimeStamp => &TIME_STAMP,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::EquipSlot => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::QuickbarItem => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AddBuff => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ConsiderForLootTables => &[ParamFlag::Content],
            Self::CooldownDuration => &[ParamFlag::Persistent],
            Self::ExtraData => {
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
            Self::Level => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::RecipeContentReferences => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemCount1 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemCount2 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemCount3 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired1 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired2 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired3 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AllowBuy => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AllowRent => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::AllowSell => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::BlackSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::BlingPrice => &[ParamFlag::NodeOwn, ParamFlag::Persistent],
            Self::BlingSellingPrice => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BlueSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::BonusSlotAmber => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BonusSlotRuby => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BonusSlotSapphire => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BuyDiscount => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::BuyPriceBling => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BuyPriceGameCash => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Category => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Combos => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ContainerId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ContentClass => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CraftingMapping => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::CraftTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::CreationTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CrystaEffects => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::CrystalType => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::CyanSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Description => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DestroyMethod => &[ParamFlag::Persistent],
            Self::Dialogs => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::DisplayName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EnableInGame => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::ExpireBuyBack => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ExpireTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameCashPrice => &[ParamFlag::Persistent],
            Self::GreenSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Icon => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::InfiniteUse => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::InitLeftTime => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::InventorySlotIndex => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsCollectFaction => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsEquiped => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsFactionItem => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsGemeCrystal => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::IsHotSeller => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsInGlobalShop => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::IsInStock => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsNewToShop => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsQuestItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsRecipe => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsSomaSeed => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsSoulBounded => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsTechApproved => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::IsTrialItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ItemCritVar => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::ItemNormalVar => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::LastUseTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LeftTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LootAction => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Lua => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Lvl => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LvlReq => {
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
            Self::MaxStackSize => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::OrangeSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Quantity => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::QuestTrigger => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Rarity => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RedSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::RentalDurationMax => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RentalDurationMin => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RentDiscount => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RentPriceBling => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::RentPriceGameCash => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SellPriceBling => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SlotId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SlotMapping => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SomaType => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SoulBoundedAccountId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SoulBoundedAvatarId => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SoulBoundedToAccount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SoulBoundType => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::StackCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::StandingReq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::UseAction => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseCoolDownTimer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseMaxCount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseRequireAvatar => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseRequireAvatarWithinRadius => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseRequireTarget => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseScript => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Vendorable => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::VendorAction => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::VioletSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::YellowSomaRequired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Abilities => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AbilityInstanceData => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Agility => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Armor => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttackPowerRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AttributeOp1 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeOp2 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeOp3 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeOp4 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeType1 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeType2 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeType3 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeType4 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeWeight1 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeWeight2 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeWeight3 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AttributeWeight4 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::Deprecated,
                ]
            }
            Self::AutoAttributeType1 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeType2 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeType3 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeType4 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeType5 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeType6 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue1 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue2 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue3 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue4 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue5 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AutoAttributeValue6 => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AvailableSockets => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BlockRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::ClanName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CombatStyle => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::CritDamageRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CritHitRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Disguise => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayNameColor => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayNameNumber => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayNameRarity => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayNameSlot => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DisplayNameStat => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DodgeRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Durability => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::DurabilityCurrent => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Focus => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Gender => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HeavyRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HitRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsBanked => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsExpired => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsSku => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::IsTemplate => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::ItemActionGenericParam => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::LevelDropVariance => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::MaxUse => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::NoteCaption => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::NoteCaptionValue => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::OtherClientInterests => &[ParamFlag::NodeOwn],
            Self::ParryRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::PeneRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Prefix => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RepairCost => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SchematicCostToCreateItem => {
                &[ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SearchKeywords => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SetBonuses => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SignOfAvatars => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Skuid => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::SocketLockedStatus => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SocketOccupancyStatus => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SocketUpgradeLevel => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SpecialRating => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Stamina => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Strength => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Suffix => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TemplateType => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::TemplateVersion => {
                &[ParamFlag::NodeOwn, ParamFlag::ClientUnknown, ParamFlag::Persistent]
            }
            Self::TimeStamp => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::PerInstanceSetting,
                ]
            }
        }
    }
}
impl FromStr for EdnaModule {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EDNA_MODULE_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for EdnaModule {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12368u16 => Ok(Self::AdditionalItemCount1),
            12367u16 => Ok(Self::AdditionalItemCount2),
            12366u16 => Ok(Self::AdditionalItemCount3),
            11720u16 => Ok(Self::AdditionalItemRequired1),
            11719u16 => Ok(Self::AdditionalItemRequired2),
            11718u16 => Ok(Self::AdditionalItemRequired3),
            7580u16 => Ok(Self::AllowBuy),
            7569u16 => Ok(Self::AllowRent),
            7627u16 => Ok(Self::AllowSell),
            11717u16 => Ok(Self::BlackSomaRequired),
            6582u16 => Ok(Self::BlingPrice),
            6581u16 => Ok(Self::BlingSellingPrice),
            11716u16 => Ok(Self::BlueSomaRequired),
            11960u16 => Ok(Self::BonusSlotAmber),
            11961u16 => Ok(Self::BonusSlotRuby),
            11962u16 => Ok(Self::BonusSlotSapphire),
            7628u16 => Ok(Self::BuyDiscount),
            7630u16 => Ok(Self::BuyPriceBling),
            7629u16 => Ok(Self::BuyPriceGameCash),
            7547u16 => Ok(Self::Category),
            8896u16 => Ok(Self::Combos),
            762u16 => Ok(Self::ContainerId),
            793u16 => Ok(Self::ContentClass),
            12196u16 => Ok(Self::CraftingMapping),
            11708u16 => Ok(Self::CraftTime),
            761u16 => Ok(Self::CreationTime),
            11993u16 => Ok(Self::CrystaEffects),
            11995u16 => Ok(Self::CrystalType),
            11715u16 => Ok(Self::CyanSomaRequired),
            6955u16 => Ok(Self::Description),
            6489u16 => Ok(Self::DestroyMethod),
            8923u16 => Ok(Self::Dialogs),
            795u16 => Ok(Self::DisplayName),
            6816u16 => Ok(Self::EnableInGame),
            791u16 => Ok(Self::EquipSlot),
            11612u16 => Ok(Self::ExpireBuyBack),
            7558u16 => Ok(Self::ExpireTime),
            780u16 => Ok(Self::Freq),
            6580u16 => Ok(Self::GameCashPrice),
            11714u16 => Ok(Self::GreenSomaRequired),
            4347u16 => Ok(Self::Icon),
            11466u16 => Ok(Self::InfiniteUse),
            12337u16 => Ok(Self::InitLeftTime),
            9874u16 => Ok(Self::InventorySlotIndex),
            12172u16 => Ok(Self::IsCollectFaction),
            789u16 => Ok(Self::IsEquiped),
            12154u16 => Ok(Self::IsFactionItem),
            11994u16 => Ok(Self::IsGemeCrystal),
            7377u16 => Ok(Self::IsHotSeller),
            7147u16 => Ok(Self::IsInGlobalShop),
            7376u16 => Ok(Self::IsInStock),
            7378u16 => Ok(Self::IsNewToShop),
            9911u16 => Ok(Self::IsQuestItem),
            11709u16 => Ok(Self::IsRecipe),
            12405u16 => Ok(Self::IsSomaSeed),
            10588u16 => Ok(Self::IsSoulBounded),
            9377u16 => Ok(Self::IsTechApproved),
            7749u16 => Ok(Self::IsTrialItem),
            11721u16 => Ok(Self::ItemCritVar),
            11722u16 => Ok(Self::ItemNormalVar),
            9015u16 => Ok(Self::LastUseTime),
            12338u16 => Ok(Self::LeftTime),
            5995u16 => Ok(Self::LootAction),
            10156u16 => Ok(Self::Lua),
            6175u16 => Ok(Self::Lvl),
            785u16 => Ok(Self::LvlReq),
            4726u16 => Ok(Self::MaterialOverride),
            9893u16 => Ok(Self::MaxStackSize),
            11713u16 => Ok(Self::OrangeSomaRequired),
            781u16 => Ok(Self::Power),
            6435u16 => Ok(Self::Quantity),
            7720u16 => Ok(Self::QuestTrigger),
            6280u16 => Ok(Self::Rarity),
            11712u16 => Ok(Self::RedSomaRequired),
            7458u16 => Ok(Self::RentalDurationMax),
            7459u16 => Ok(Self::RentalDurationMin),
            7631u16 => Ok(Self::RentDiscount),
            7633u16 => Ok(Self::RentPriceBling),
            7632u16 => Ok(Self::RentPriceGameCash),
            7626u16 => Ok(Self::SellPriceBling),
            782u16 => Ok(Self::SlotId),
            6249u16 => Ok(Self::SlotMapping),
            12404u16 => Ok(Self::SomaType),
            12262u16 => Ok(Self::SoulBoundedAccountId),
            10615u16 => Ok(Self::SoulBoundedAvatarId),
            12251u16 => Ok(Self::SoulBoundedToAccount),
            10587u16 => Ok(Self::SoulBoundType),
            9892u16 => Ok(Self::StackCount),
            12171u16 => Ok(Self::StandingReq),
            6020u16 => Ok(Self::UseAction),
            8998u16 => Ok(Self::UseCoolDownTimer),
            9026u16 => Ok(Self::UseCount),
            8999u16 => Ok(Self::UseMaxCount),
            8964u16 => Ok(Self::UseRequireAvatar),
            8963u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8965u16 => Ok(Self::UseRequireTarget),
            8962u16 => Ok(Self::UseScript),
            6488u16 => Ok(Self::Vendorable),
            5936u16 => Ok(Self::VendorAction),
            11711u16 => Ok(Self::VioletSomaRequired),
            11710u16 => Ok(Self::YellowSomaRequired),
            774u16 => Ok(Self::Abilities),
            766u16 => Ok(Self::AbilityInstanceData),
            11563u16 => Ok(Self::Agility),
            11550u16 => Ok(Self::Armor),
            11554u16 => Ok(Self::AttackPowerRating),
            6419u16 => Ok(Self::AttributeOp1),
            6418u16 => Ok(Self::AttributeOp2),
            6417u16 => Ok(Self::AttributeOp3),
            6416u16 => Ok(Self::AttributeOp4),
            6423u16 => Ok(Self::AttributeType1),
            6422u16 => Ok(Self::AttributeType2),
            6421u16 => Ok(Self::AttributeType3),
            6420u16 => Ok(Self::AttributeType4),
            6415u16 => Ok(Self::AttributeWeight1),
            6414u16 => Ok(Self::AttributeWeight2),
            6413u16 => Ok(Self::AttributeWeight3),
            6412u16 => Ok(Self::AttributeWeight4),
            9494u16 => Ok(Self::AutoAttributeType1),
            9493u16 => Ok(Self::AutoAttributeType2),
            9492u16 => Ok(Self::AutoAttributeType3),
            9491u16 => Ok(Self::AutoAttributeType4),
            9552u16 => Ok(Self::AutoAttributeType5),
            9551u16 => Ok(Self::AutoAttributeType6),
            9488u16 => Ok(Self::AutoAttributeValue1),
            9487u16 => Ok(Self::AutoAttributeValue2),
            9486u16 => Ok(Self::AutoAttributeValue3),
            9485u16 => Ok(Self::AutoAttributeValue4),
            9550u16 => Ok(Self::AutoAttributeValue5),
            9549u16 => Ok(Self::AutoAttributeValue6),
            10089u16 => Ok(Self::AvailableSockets),
            11551u16 => Ok(Self::BlockRating),
            12039u16 => Ok(Self::ClanName),
            4250u16 => Ok(Self::CombatStyle),
            11555u16 => Ok(Self::CritDamageRating),
            11559u16 => Ok(Self::CritHitRating),
            9991u16 => Ok(Self::Disguise),
            12238u16 => Ok(Self::DisplayNameColor),
            12237u16 => Ok(Self::DisplayNameNumber),
            12239u16 => Ok(Self::DisplayNameRarity),
            12241u16 => Ok(Self::DisplayNameSlot),
            12240u16 => Ok(Self::DisplayNameStat),
            11553u16 => Ok(Self::DodgeRating),
            10021u16 => Ok(Self::Durability),
            10096u16 => Ok(Self::DurabilityCurrent),
            11562u16 => Ok(Self::Focus),
            11019u16 => Ok(Self::Gender),
            11557u16 => Ok(Self::HeavyRating),
            11560u16 => Ok(Self::HitRating),
            790u16 => Ok(Self::IsBanked),
            764u16 => Ok(Self::IsExpired),
            10193u16 => Ok(Self::IsSku),
            9710u16 => Ok(Self::IsTemplate),
            10390u16 => Ok(Self::ItemActionGenericParam),
            10900u16 => Ok(Self::LevelDropVariance),
            4814u16 => Ok(Self::MaxUse),
            12243u16 => Ok(Self::NoteCaption),
            12242u16 => Ok(Self::NoteCaptionValue),
            767u16 => Ok(Self::OtherClientInterests),
            11552u16 => Ok(Self::ParryRating),
            11558u16 => Ok(Self::PeneRating),
            9709u16 => Ok(Self::Prefix),
            12276u16 => Ok(Self::QuickbarItem),
            10101u16 => Ok(Self::RepairCost),
            10106u16 => Ok(Self::SchematicCostToCreateItem),
            10072u16 => Ok(Self::SearchKeywords),
            12386u16 => Ok(Self::SetBonuses),
            12040u16 => Ok(Self::SignOfAvatars),
            10609u16 => Ok(Self::Skuid),
            10092u16 => Ok(Self::SocketLockedStatus),
            10090u16 => Ok(Self::SocketOccupancyStatus),
            10091u16 => Ok(Self::SocketUpgradeLevel),
            11556u16 => Ok(Self::SpecialRating),
            11561u16 => Ok(Self::Stamina),
            11564u16 => Ok(Self::Strength),
            9708u16 => Ok(Self::Suffix),
            10076u16 => Ok(Self::TemplateType),
            11314u16 => Ok(Self::TemplateVersion),
            763u16 => Ok(Self::TimeStamp),
            9371u16 => Ok(Self::AddBuff),
            12290u16 => Ok(Self::ConsiderForLootTables),
            754u16 => Ok(Self::CooldownDuration),
            768u16 => Ok(Self::ExtraData),
            12146u16 => Ok(Self::Faction),
            4941u16 => Ok(Self::Level),
            779u16 => Ok(Self::Name),
            12277u16 => Ok(Self::RecipeContentReferences),
            777u16 => Ok(Self::Ue3ClassId),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
