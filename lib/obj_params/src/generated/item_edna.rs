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
pub enum ItemEdna {
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
}
pub(crate) static ITEM_EDNA_ATTRIBUTES: phf::Map<&'static str, ItemEdna> = phf_map! {
    "AdditionalItemCount1" => ItemEdna::AdditionalItemCount1, "AdditionalItemCount2" =>
    ItemEdna::AdditionalItemCount2, "AdditionalItemCount3" =>
    ItemEdna::AdditionalItemCount3, "AdditionalItemRequired1" =>
    ItemEdna::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    ItemEdna::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    ItemEdna::AdditionalItemRequired3, "AllowBuy" => ItemEdna::AllowBuy, "AllowRent" =>
    ItemEdna::AllowRent, "AllowSell" => ItemEdna::AllowSell, "BlackSomaRequired" =>
    ItemEdna::BlackSomaRequired, "blingPrice" => ItemEdna::BlingPrice,
    "blingSellingPrice" => ItemEdna::BlingSellingPrice, "BlueSomaRequired" =>
    ItemEdna::BlueSomaRequired, "BonusSlotAmber" => ItemEdna::BonusSlotAmber,
    "BonusSlotRuby" => ItemEdna::BonusSlotRuby, "BonusSlotSapphire" =>
    ItemEdna::BonusSlotSapphire, "BuyDiscount" => ItemEdna::BuyDiscount, "BuyPriceBling"
    => ItemEdna::BuyPriceBling, "BuyPriceGameCash" => ItemEdna::BuyPriceGameCash,
    "Category" => ItemEdna::Category, "combos" => ItemEdna::Combos, "containerID" =>
    ItemEdna::ContainerId, "ContentClass" => ItemEdna::ContentClass, "CraftingMapping" =>
    ItemEdna::CraftingMapping, "CraftTime" => ItemEdna::CraftTime, "creationTime" =>
    ItemEdna::CreationTime, "CrystaEffects" => ItemEdna::CrystaEffects, "CrystalType" =>
    ItemEdna::CrystalType, "CyanSomaRequired" => ItemEdna::CyanSomaRequired,
    "Description" => ItemEdna::Description, "DestroyMethod" => ItemEdna::DestroyMethod,
    "Dialogs" => ItemEdna::Dialogs, "DisplayName" => ItemEdna::DisplayName,
    "EnableInGame" => ItemEdna::EnableInGame, "equipSlot" => ItemEdna::EquipSlot,
    "expireBuyBack" => ItemEdna::ExpireBuyBack, "ExpireTime" => ItemEdna::ExpireTime,
    "Freq" => ItemEdna::Freq, "gameCashPrice" => ItemEdna::GameCashPrice,
    "GreenSomaRequired" => ItemEdna::GreenSomaRequired, "Icon" => ItemEdna::Icon,
    "InfiniteUse" => ItemEdna::InfiniteUse, "InitLeftTime" => ItemEdna::InitLeftTime,
    "inventorySlotIndex" => ItemEdna::InventorySlotIndex, "isCollectFaction" =>
    ItemEdna::IsCollectFaction, "isEquiped" => ItemEdna::IsEquiped, "isFactionItem" =>
    ItemEdna::IsFactionItem, "isGemeCrystal" => ItemEdna::IsGemeCrystal, "IsHotSeller" =>
    ItemEdna::IsHotSeller, "isInGlobalShop" => ItemEdna::IsInGlobalShop, "IsInStock" =>
    ItemEdna::IsInStock, "IsNewToShop" => ItemEdna::IsNewToShop, "isQuestItem" =>
    ItemEdna::IsQuestItem, "IsRecipe" => ItemEdna::IsRecipe, "IsSomaSeed" =>
    ItemEdna::IsSomaSeed, "IsSoulBounded" => ItemEdna::IsSoulBounded, "isTechApproved" =>
    ItemEdna::IsTechApproved, "isTrialItem" => ItemEdna::IsTrialItem, "ItemCritVar" =>
    ItemEdna::ItemCritVar, "ItemNormalVar" => ItemEdna::ItemNormalVar, "LastUseTime" =>
    ItemEdna::LastUseTime, "LeftTime" => ItemEdna::LeftTime, "lootAction" =>
    ItemEdna::LootAction, "Lua" => ItemEdna::Lua, "lvl" => ItemEdna::Lvl, "lvlReq" =>
    ItemEdna::LvlReq, "MaterialOverride" => ItemEdna::MaterialOverride, "maxStackSize" =>
    ItemEdna::MaxStackSize, "OrangeSomaRequired" => ItemEdna::OrangeSomaRequired, "Power"
    => ItemEdna::Power, "quantity" => ItemEdna::Quantity, "QuestTrigger" =>
    ItemEdna::QuestTrigger, "rarity" => ItemEdna::Rarity, "RedSomaRequired" =>
    ItemEdna::RedSomaRequired, "RentalDurationMax" => ItemEdna::RentalDurationMax,
    "RentalDurationMin" => ItemEdna::RentalDurationMin, "RentDiscount" =>
    ItemEdna::RentDiscount, "RentPriceBling" => ItemEdna::RentPriceBling,
    "RentPriceGameCash" => ItemEdna::RentPriceGameCash, "SellPriceBling" =>
    ItemEdna::SellPriceBling, "slotID" => ItemEdna::SlotId, "SlotMapping" =>
    ItemEdna::SlotMapping, "SomaType" => ItemEdna::SomaType, "SoulBoundedAccountId" =>
    ItemEdna::SoulBoundedAccountId, "SoulBoundedAvatarId" =>
    ItemEdna::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    ItemEdna::SoulBoundedToAccount, "SoulBoundType" => ItemEdna::SoulBoundType,
    "stackCount" => ItemEdna::StackCount, "standingReq" => ItemEdna::StandingReq,
    "useAction" => ItemEdna::UseAction, "UseCoolDownTimer" => ItemEdna::UseCoolDownTimer,
    "UseCount" => ItemEdna::UseCount, "UseMaxCount" => ItemEdna::UseMaxCount,
    "UseRequireAvatar" => ItemEdna::UseRequireAvatar, "UseRequireAvatarWithinRadius" =>
    ItemEdna::UseRequireAvatarWithinRadius, "UseRequireTarget" =>
    ItemEdna::UseRequireTarget, "UseScript" => ItemEdna::UseScript, "Vendorable" =>
    ItemEdna::Vendorable, "vendorAction" => ItemEdna::VendorAction, "VioletSomaRequired"
    => ItemEdna::VioletSomaRequired, "YellowSomaRequired" =>
    ItemEdna::YellowSomaRequired, "abilities" => ItemEdna::Abilities,
    "abilityInstanceData" => ItemEdna::AbilityInstanceData, "Agility" =>
    ItemEdna::Agility, "Armor" => ItemEdna::Armor, "AttackPowerRating" =>
    ItemEdna::AttackPowerRating, "attributeOp1" => ItemEdna::AttributeOp1, "attributeOp2"
    => ItemEdna::AttributeOp2, "attributeOp3" => ItemEdna::AttributeOp3, "attributeOp4"
    => ItemEdna::AttributeOp4, "attributeType1" => ItemEdna::AttributeType1,
    "attributeType2" => ItemEdna::AttributeType2, "attributeType3" =>
    ItemEdna::AttributeType3, "attributeType4" => ItemEdna::AttributeType4,
    "attributeWeight1" => ItemEdna::AttributeWeight1, "attributeWeight2" =>
    ItemEdna::AttributeWeight2, "attributeWeight3" => ItemEdna::AttributeWeight3,
    "attributeWeight4" => ItemEdna::AttributeWeight4, "autoAttributeType1" =>
    ItemEdna::AutoAttributeType1, "autoAttributeType2" => ItemEdna::AutoAttributeType2,
    "autoAttributeType3" => ItemEdna::AutoAttributeType3, "autoAttributeType4" =>
    ItemEdna::AutoAttributeType4, "autoAttributeType5" => ItemEdna::AutoAttributeType5,
    "autoAttributeType6" => ItemEdna::AutoAttributeType6, "autoAttributeValue1" =>
    ItemEdna::AutoAttributeValue1, "autoAttributeValue2" =>
    ItemEdna::AutoAttributeValue2, "autoAttributeValue3" =>
    ItemEdna::AutoAttributeValue3, "autoAttributeValue4" =>
    ItemEdna::AutoAttributeValue4, "autoAttributeValue5" =>
    ItemEdna::AutoAttributeValue5, "autoAttributeValue6" =>
    ItemEdna::AutoAttributeValue6, "availableSockets" => ItemEdna::AvailableSockets,
    "BlockRating" => ItemEdna::BlockRating, "ClanName" => ItemEdna::ClanName,
    "combatStyle" => ItemEdna::CombatStyle, "CritDamageRating" =>
    ItemEdna::CritDamageRating, "CritHitRating" => ItemEdna::CritHitRating, "disguise" =>
    ItemEdna::Disguise, "DisplayName_Color" => ItemEdna::DisplayNameColor,
    "DisplayName_Number" => ItemEdna::DisplayNameNumber, "DisplayName_Rarity" =>
    ItemEdna::DisplayNameRarity, "DisplayName_Slot" => ItemEdna::DisplayNameSlot,
    "DisplayName_Stat" => ItemEdna::DisplayNameStat, "DodgeRating" =>
    ItemEdna::DodgeRating, "durability" => ItemEdna::Durability, "durabilityCurrent" =>
    ItemEdna::DurabilityCurrent, "Focus" => ItemEdna::Focus, "gender" =>
    ItemEdna::Gender, "HeavyRating" => ItemEdna::HeavyRating, "HitRating" =>
    ItemEdna::HitRating, "isBanked" => ItemEdna::IsBanked, "isExpired" =>
    ItemEdna::IsExpired, "isSKU" => ItemEdna::IsSku, "IsTemplate" =>
    ItemEdna::IsTemplate, "itemActionGenericParam" => ItemEdna::ItemActionGenericParam,
    "levelDropVariance" => ItemEdna::LevelDropVariance, "MaxUse" => ItemEdna::MaxUse,
    "NoteCaption" => ItemEdna::NoteCaption, "NoteCaptionValue" =>
    ItemEdna::NoteCaptionValue, "otherClientInterests" => ItemEdna::OtherClientInterests,
    "ParryRating" => ItemEdna::ParryRating, "PeneRating" => ItemEdna::PeneRating,
    "Prefix" => ItemEdna::Prefix, "QuickbarItem" => ItemEdna::QuickbarItem, "repairCost"
    => ItemEdna::RepairCost, "schematic_CostToCreateItem" =>
    ItemEdna::SchematicCostToCreateItem, "searchKeywords" => ItemEdna::SearchKeywords,
    "setBonuses" => ItemEdna::SetBonuses, "SignOfAvatars" => ItemEdna::SignOfAvatars,
    "SKUID" => ItemEdna::Skuid, "socketLockedStatus" => ItemEdna::SocketLockedStatus,
    "socketOccupancyStatus" => ItemEdna::SocketOccupancyStatus, "socketUpgradeLevel" =>
    ItemEdna::SocketUpgradeLevel, "SpecialRating" => ItemEdna::SpecialRating, "Stamina"
    => ItemEdna::Stamina, "Strength" => ItemEdna::Strength, "Suffix" => ItemEdna::Suffix,
    "templateType" => ItemEdna::TemplateType, "templateVersion" =>
    ItemEdna::TemplateVersion, "timeStamp" => ItemEdna::TimeStamp,
};
pub(crate) static ITEM_EDNA_ATTRIBUTES_ID: phf::Map<u16, ItemEdna> = phf_map! {
    12353u16 => ItemEdna::AdditionalItemCount1, 12352u16 =>
    ItemEdna::AdditionalItemCount2, 12351u16 => ItemEdna::AdditionalItemCount3, 11645u16
    => ItemEdna::AdditionalItemRequired1, 11644u16 => ItemEdna::AdditionalItemRequired2,
    11643u16 => ItemEdna::AdditionalItemRequired3, 7577u16 => ItemEdna::AllowBuy, 7566u16
    => ItemEdna::AllowRent, 7603u16 => ItemEdna::AllowSell, 11642u16 =>
    ItemEdna::BlackSomaRequired, 6573u16 => ItemEdna::BlingPrice, 6572u16 =>
    ItemEdna::BlingSellingPrice, 11641u16 => ItemEdna::BlueSomaRequired, 11945u16 =>
    ItemEdna::BonusSlotAmber, 11946u16 => ItemEdna::BonusSlotRuby, 11947u16 =>
    ItemEdna::BonusSlotSapphire, 7604u16 => ItemEdna::BuyDiscount, 7606u16 =>
    ItemEdna::BuyPriceBling, 7605u16 => ItemEdna::BuyPriceGameCash, 7544u16 =>
    ItemEdna::Category, 8893u16 => ItemEdna::Combos, 628u16 => ItemEdna::ContainerId,
    655u16 => ItemEdna::ContentClass, 12191u16 => ItemEdna::CraftingMapping, 11633u16 =>
    ItemEdna::CraftTime, 627u16 => ItemEdna::CreationTime, 11978u16 =>
    ItemEdna::CrystaEffects, 11980u16 => ItemEdna::CrystalType, 11640u16 =>
    ItemEdna::CyanSomaRequired, 6952u16 => ItemEdna::Description, 6483u16 =>
    ItemEdna::DestroyMethod, 8920u16 => ItemEdna::Dialogs, 657u16 =>
    ItemEdna::DisplayName, 6813u16 => ItemEdna::EnableInGame, 653u16 =>
    ItemEdna::EquipSlot, 11607u16 => ItemEdna::ExpireBuyBack, 7555u16 =>
    ItemEdna::ExpireTime, 642u16 => ItemEdna::Freq, 6571u16 => ItemEdna::GameCashPrice,
    11639u16 => ItemEdna::GreenSomaRequired, 4344u16 => ItemEdna::Icon, 11461u16 =>
    ItemEdna::InfiniteUse, 12327u16 => ItemEdna::InitLeftTime, 9871u16 =>
    ItemEdna::InventorySlotIndex, 12162u16 => ItemEdna::IsCollectFaction, 651u16 =>
    ItemEdna::IsEquiped, 12149u16 => ItemEdna::IsFactionItem, 11979u16 =>
    ItemEdna::IsGemeCrystal, 7347u16 => ItemEdna::IsHotSeller, 7144u16 =>
    ItemEdna::IsInGlobalShop, 7346u16 => ItemEdna::IsInStock, 7348u16 =>
    ItemEdna::IsNewToShop, 9908u16 => ItemEdna::IsQuestItem, 11634u16 =>
    ItemEdna::IsRecipe, 12395u16 => ItemEdna::IsSomaSeed, 10582u16 =>
    ItemEdna::IsSoulBounded, 9374u16 => ItemEdna::IsTechApproved, 7746u16 =>
    ItemEdna::IsTrialItem, 11646u16 => ItemEdna::ItemCritVar, 11647u16 =>
    ItemEdna::ItemNormalVar, 9012u16 => ItemEdna::LastUseTime, 12328u16 =>
    ItemEdna::LeftTime, 5992u16 => ItemEdna::LootAction, 10153u16 => ItemEdna::Lua,
    6172u16 => ItemEdna::Lvl, 647u16 => ItemEdna::LvlReq, 4723u16 =>
    ItemEdna::MaterialOverride, 9887u16 => ItemEdna::MaxStackSize, 11638u16 =>
    ItemEdna::OrangeSomaRequired, 643u16 => ItemEdna::Power, 6432u16 =>
    ItemEdna::Quantity, 7714u16 => ItemEdna::QuestTrigger, 6277u16 => ItemEdna::Rarity,
    11637u16 => ItemEdna::RedSomaRequired, 7452u16 => ItemEdna::RentalDurationMax,
    7453u16 => ItemEdna::RentalDurationMin, 7607u16 => ItemEdna::RentDiscount, 7609u16 =>
    ItemEdna::RentPriceBling, 7608u16 => ItemEdna::RentPriceGameCash, 7602u16 =>
    ItemEdna::SellPriceBling, 644u16 => ItemEdna::SlotId, 6246u16 =>
    ItemEdna::SlotMapping, 12394u16 => ItemEdna::SomaType, 12257u16 =>
    ItemEdna::SoulBoundedAccountId, 10612u16 => ItemEdna::SoulBoundedAvatarId, 12246u16
    => ItemEdna::SoulBoundedToAccount, 10581u16 => ItemEdna::SoulBoundType, 9886u16 =>
    ItemEdna::StackCount, 12161u16 => ItemEdna::StandingReq, 6017u16 =>
    ItemEdna::UseAction, 8992u16 => ItemEdna::UseCoolDownTimer, 9023u16 =>
    ItemEdna::UseCount, 8993u16 => ItemEdna::UseMaxCount, 8952u16 =>
    ItemEdna::UseRequireAvatar, 8951u16 => ItemEdna::UseRequireAvatarWithinRadius,
    8953u16 => ItemEdna::UseRequireTarget, 8950u16 => ItemEdna::UseScript, 6482u16 =>
    ItemEdna::Vendorable, 5933u16 => ItemEdna::VendorAction, 11636u16 =>
    ItemEdna::VioletSomaRequired, 11635u16 => ItemEdna::YellowSomaRequired, 639u16 =>
    ItemEdna::Abilities, 632u16 => ItemEdna::AbilityInstanceData, 11485u16 =>
    ItemEdna::Agility, 11472u16 => ItemEdna::Armor, 11476u16 =>
    ItemEdna::AttackPowerRating, 6383u16 => ItemEdna::AttributeOp1, 6382u16 =>
    ItemEdna::AttributeOp2, 6381u16 => ItemEdna::AttributeOp3, 6380u16 =>
    ItemEdna::AttributeOp4, 6387u16 => ItemEdna::AttributeType1, 6386u16 =>
    ItemEdna::AttributeType2, 6385u16 => ItemEdna::AttributeType3, 6384u16 =>
    ItemEdna::AttributeType4, 6379u16 => ItemEdna::AttributeWeight1, 6378u16 =>
    ItemEdna::AttributeWeight2, 6377u16 => ItemEdna::AttributeWeight3, 6376u16 =>
    ItemEdna::AttributeWeight4, 9458u16 => ItemEdna::AutoAttributeType1, 9457u16 =>
    ItemEdna::AutoAttributeType2, 9456u16 => ItemEdna::AutoAttributeType3, 9455u16 =>
    ItemEdna::AutoAttributeType4, 9540u16 => ItemEdna::AutoAttributeType5, 9539u16 =>
    ItemEdna::AutoAttributeType6, 9452u16 => ItemEdna::AutoAttributeValue1, 9451u16 =>
    ItemEdna::AutoAttributeValue2, 9450u16 => ItemEdna::AutoAttributeValue3, 9449u16 =>
    ItemEdna::AutoAttributeValue4, 9538u16 => ItemEdna::AutoAttributeValue5, 9537u16 =>
    ItemEdna::AutoAttributeValue6, 10077u16 => ItemEdna::AvailableSockets, 11473u16 =>
    ItemEdna::BlockRating, 12029u16 => ItemEdna::ClanName, 4246u16 =>
    ItemEdna::CombatStyle, 11477u16 => ItemEdna::CritDamageRating, 11481u16 =>
    ItemEdna::CritHitRating, 9988u16 => ItemEdna::Disguise, 12203u16 =>
    ItemEdna::DisplayNameColor, 12202u16 => ItemEdna::DisplayNameNumber, 12204u16 =>
    ItemEdna::DisplayNameRarity, 12206u16 => ItemEdna::DisplayNameSlot, 12205u16 =>
    ItemEdna::DisplayNameStat, 11475u16 => ItemEdna::DodgeRating, 10018u16 =>
    ItemEdna::Durability, 10093u16 => ItemEdna::DurabilityCurrent, 11484u16 =>
    ItemEdna::Focus, 11015u16 => ItemEdna::Gender, 11479u16 => ItemEdna::HeavyRating,
    11482u16 => ItemEdna::HitRating, 652u16 => ItemEdna::IsBanked, 630u16 =>
    ItemEdna::IsExpired, 10190u16 => ItemEdna::IsSku, 9701u16 => ItemEdna::IsTemplate,
    10387u16 => ItemEdna::ItemActionGenericParam, 10896u16 =>
    ItemEdna::LevelDropVariance, 4811u16 => ItemEdna::MaxUse, 12208u16 =>
    ItemEdna::NoteCaption, 12207u16 => ItemEdna::NoteCaptionValue, 633u16 =>
    ItemEdna::OtherClientInterests, 11474u16 => ItemEdna::ParryRating, 11480u16 =>
    ItemEdna::PeneRating, 9700u16 => ItemEdna::Prefix, 12270u16 =>
    ItemEdna::QuickbarItem, 10098u16 => ItemEdna::RepairCost, 10103u16 =>
    ItemEdna::SchematicCostToCreateItem, 10069u16 => ItemEdna::SearchKeywords, 12381u16
    => ItemEdna::SetBonuses, 12030u16 => ItemEdna::SignOfAvatars, 10606u16 =>
    ItemEdna::Skuid, 10080u16 => ItemEdna::SocketLockedStatus, 10078u16 =>
    ItemEdna::SocketOccupancyStatus, 10079u16 => ItemEdna::SocketUpgradeLevel, 11478u16
    => ItemEdna::SpecialRating, 11483u16 => ItemEdna::Stamina, 11486u16 =>
    ItemEdna::Strength, 9699u16 => ItemEdna::Suffix, 10073u16 => ItemEdna::TemplateType,
    11310u16 => ItemEdna::TemplateVersion, 629u16 => ItemEdna::TimeStamp,
};
impl Attribute for ItemEdna {
    fn class() -> Class {
        Class::ItemEdna
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
        }
    }
}
impl AttributeInfo for ItemEdna {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12353u16,
            Self::AdditionalItemCount2 => 12352u16,
            Self::AdditionalItemCount3 => 12351u16,
            Self::AdditionalItemRequired1 => 11645u16,
            Self::AdditionalItemRequired2 => 11644u16,
            Self::AdditionalItemRequired3 => 11643u16,
            Self::AllowBuy => 7577u16,
            Self::AllowRent => 7566u16,
            Self::AllowSell => 7603u16,
            Self::BlackSomaRequired => 11642u16,
            Self::BlingPrice => 6573u16,
            Self::BlingSellingPrice => 6572u16,
            Self::BlueSomaRequired => 11641u16,
            Self::BonusSlotAmber => 11945u16,
            Self::BonusSlotRuby => 11946u16,
            Self::BonusSlotSapphire => 11947u16,
            Self::BuyDiscount => 7604u16,
            Self::BuyPriceBling => 7606u16,
            Self::BuyPriceGameCash => 7605u16,
            Self::Category => 7544u16,
            Self::Combos => 8893u16,
            Self::ContainerId => 628u16,
            Self::ContentClass => 655u16,
            Self::CraftingMapping => 12191u16,
            Self::CraftTime => 11633u16,
            Self::CreationTime => 627u16,
            Self::CrystaEffects => 11978u16,
            Self::CrystalType => 11980u16,
            Self::CyanSomaRequired => 11640u16,
            Self::Description => 6952u16,
            Self::DestroyMethod => 6483u16,
            Self::Dialogs => 8920u16,
            Self::DisplayName => 657u16,
            Self::EnableInGame => 6813u16,
            Self::EquipSlot => 653u16,
            Self::ExpireBuyBack => 11607u16,
            Self::ExpireTime => 7555u16,
            Self::Freq => 642u16,
            Self::GameCashPrice => 6571u16,
            Self::GreenSomaRequired => 11639u16,
            Self::Icon => 4344u16,
            Self::InfiniteUse => 11461u16,
            Self::InitLeftTime => 12327u16,
            Self::InventorySlotIndex => 9871u16,
            Self::IsCollectFaction => 12162u16,
            Self::IsEquiped => 651u16,
            Self::IsFactionItem => 12149u16,
            Self::IsGemeCrystal => 11979u16,
            Self::IsHotSeller => 7347u16,
            Self::IsInGlobalShop => 7144u16,
            Self::IsInStock => 7346u16,
            Self::IsNewToShop => 7348u16,
            Self::IsQuestItem => 9908u16,
            Self::IsRecipe => 11634u16,
            Self::IsSomaSeed => 12395u16,
            Self::IsSoulBounded => 10582u16,
            Self::IsTechApproved => 9374u16,
            Self::IsTrialItem => 7746u16,
            Self::ItemCritVar => 11646u16,
            Self::ItemNormalVar => 11647u16,
            Self::LastUseTime => 9012u16,
            Self::LeftTime => 12328u16,
            Self::LootAction => 5992u16,
            Self::Lua => 10153u16,
            Self::Lvl => 6172u16,
            Self::LvlReq => 647u16,
            Self::MaterialOverride => 4723u16,
            Self::MaxStackSize => 9887u16,
            Self::OrangeSomaRequired => 11638u16,
            Self::Power => 643u16,
            Self::Quantity => 6432u16,
            Self::QuestTrigger => 7714u16,
            Self::Rarity => 6277u16,
            Self::RedSomaRequired => 11637u16,
            Self::RentalDurationMax => 7452u16,
            Self::RentalDurationMin => 7453u16,
            Self::RentDiscount => 7607u16,
            Self::RentPriceBling => 7609u16,
            Self::RentPriceGameCash => 7608u16,
            Self::SellPriceBling => 7602u16,
            Self::SlotId => 644u16,
            Self::SlotMapping => 6246u16,
            Self::SomaType => 12394u16,
            Self::SoulBoundedAccountId => 12257u16,
            Self::SoulBoundedAvatarId => 10612u16,
            Self::SoulBoundedToAccount => 12246u16,
            Self::SoulBoundType => 10581u16,
            Self::StackCount => 9886u16,
            Self::StandingReq => 12161u16,
            Self::UseAction => 6017u16,
            Self::UseCoolDownTimer => 8992u16,
            Self::UseCount => 9023u16,
            Self::UseMaxCount => 8993u16,
            Self::UseRequireAvatar => 8952u16,
            Self::UseRequireAvatarWithinRadius => 8951u16,
            Self::UseRequireTarget => 8953u16,
            Self::UseScript => 8950u16,
            Self::Vendorable => 6482u16,
            Self::VendorAction => 5933u16,
            Self::VioletSomaRequired => 11636u16,
            Self::YellowSomaRequired => 11635u16,
            Self::Abilities => 639u16,
            Self::AbilityInstanceData => 632u16,
            Self::Agility => 11485u16,
            Self::Armor => 11472u16,
            Self::AttackPowerRating => 11476u16,
            Self::AttributeOp1 => 6383u16,
            Self::AttributeOp2 => 6382u16,
            Self::AttributeOp3 => 6381u16,
            Self::AttributeOp4 => 6380u16,
            Self::AttributeType1 => 6387u16,
            Self::AttributeType2 => 6386u16,
            Self::AttributeType3 => 6385u16,
            Self::AttributeType4 => 6384u16,
            Self::AttributeWeight1 => 6379u16,
            Self::AttributeWeight2 => 6378u16,
            Self::AttributeWeight3 => 6377u16,
            Self::AttributeWeight4 => 6376u16,
            Self::AutoAttributeType1 => 9458u16,
            Self::AutoAttributeType2 => 9457u16,
            Self::AutoAttributeType3 => 9456u16,
            Self::AutoAttributeType4 => 9455u16,
            Self::AutoAttributeType5 => 9540u16,
            Self::AutoAttributeType6 => 9539u16,
            Self::AutoAttributeValue1 => 9452u16,
            Self::AutoAttributeValue2 => 9451u16,
            Self::AutoAttributeValue3 => 9450u16,
            Self::AutoAttributeValue4 => 9449u16,
            Self::AutoAttributeValue5 => 9538u16,
            Self::AutoAttributeValue6 => 9537u16,
            Self::AvailableSockets => 10077u16,
            Self::BlockRating => 11473u16,
            Self::ClanName => 12029u16,
            Self::CombatStyle => 4246u16,
            Self::CritDamageRating => 11477u16,
            Self::CritHitRating => 11481u16,
            Self::Disguise => 9988u16,
            Self::DisplayNameColor => 12203u16,
            Self::DisplayNameNumber => 12202u16,
            Self::DisplayNameRarity => 12204u16,
            Self::DisplayNameSlot => 12206u16,
            Self::DisplayNameStat => 12205u16,
            Self::DodgeRating => 11475u16,
            Self::Durability => 10018u16,
            Self::DurabilityCurrent => 10093u16,
            Self::Focus => 11484u16,
            Self::Gender => 11015u16,
            Self::HeavyRating => 11479u16,
            Self::HitRating => 11482u16,
            Self::IsBanked => 652u16,
            Self::IsExpired => 630u16,
            Self::IsSku => 10190u16,
            Self::IsTemplate => 9701u16,
            Self::ItemActionGenericParam => 10387u16,
            Self::LevelDropVariance => 10896u16,
            Self::MaxUse => 4811u16,
            Self::NoteCaption => 12208u16,
            Self::NoteCaptionValue => 12207u16,
            Self::OtherClientInterests => 633u16,
            Self::ParryRating => 11474u16,
            Self::PeneRating => 11480u16,
            Self::Prefix => 9700u16,
            Self::QuickbarItem => 12270u16,
            Self::RepairCost => 10098u16,
            Self::SchematicCostToCreateItem => 10103u16,
            Self::SearchKeywords => 10069u16,
            Self::SetBonuses => 12381u16,
            Self::SignOfAvatars => 12030u16,
            Self::Skuid => 10606u16,
            Self::SocketLockedStatus => 10080u16,
            Self::SocketOccupancyStatus => 10078u16,
            Self::SocketUpgradeLevel => 10079u16,
            Self::SpecialRating => 11478u16,
            Self::Stamina => 11483u16,
            Self::Strength => 11486u16,
            Self::Suffix => 9699u16,
            Self::TemplateType => 10073u16,
            Self::TemplateVersion => 11310u16,
            Self::TimeStamp => 629u16,
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
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::CreationTime => ParamType::Int64,
            Self::Description => ParamType::LocalizedString,
            Self::DisplayName => ParamType::LocalizedString,
            Self::EquipSlot => ParamType::String,
            Self::Icon => ParamType::String,
            Self::IsInGlobalShop => ParamType::Bool,
            Self::IsTechApproved => ParamType::Bool,
            Self::Rarity => ParamType::String,
            Self::SlotMapping => ParamType::String,
            Self::VendorAction => ParamType::String,
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
            Self::QuickbarItem => ParamType::Bool,
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
            Self::CrystaEffects => ParamType::JsonValue,
            Self::CrystalType => ParamType::String,
            Self::CyanSomaRequired => ParamType::Int,
            Self::DestroyMethod => ParamType::String,
            Self::Dialogs => ParamType::VectorInt,
            Self::EnableInGame => ParamType::Bool,
            Self::ExpireBuyBack => ParamType::Int64,
            Self::ExpireTime => ParamType::Int64,
            Self::Freq => ParamType::Int,
            Self::GameCashPrice => ParamType::Int,
            Self::GreenSomaRequired => ParamType::Int,
            Self::InfiniteUse => ParamType::Bool,
            Self::InitLeftTime => ParamType::Int,
            Self::InventorySlotIndex => ParamType::Int,
            Self::IsCollectFaction => ParamType::Bool,
            Self::IsEquiped => ParamType::Bool,
            Self::IsFactionItem => ParamType::Bool,
            Self::IsGemeCrystal => ParamType::Bool,
            Self::IsHotSeller => ParamType::Bool,
            Self::IsInStock => ParamType::Bool,
            Self::IsNewToShop => ParamType::Bool,
            Self::IsQuestItem => ParamType::Bool,
            Self::IsRecipe => ParamType::Bool,
            Self::IsSomaSeed => ParamType::Bool,
            Self::IsSoulBounded => ParamType::Bool,
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
            Self::RedSomaRequired => ParamType::Int,
            Self::RentalDurationMax => ParamType::Float,
            Self::RentalDurationMin => ParamType::Float,
            Self::RentDiscount => ParamType::Float,
            Self::RentPriceBling => ParamType::Float,
            Self::RentPriceGameCash => ParamType::Float,
            Self::SellPriceBling => ParamType::Int,
            Self::SlotId => ParamType::Int,
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
            Self::VioletSomaRequired => ParamType::Int,
            Self::YellowSomaRequired => ParamType::Int,
        }
    }
    fn default(&self) -> &'static Value {
        static CREATION_TIME: Value = Value::Int64(0i64);
        static DESCRIPTION: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static DISPLAY_NAME: Value = Value::LocalizedString(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static EQUIP_SLOT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static IS_IN_GLOBAL_SHOP: Value = Value::Bool(true);
        static IS_TECH_APPROVED: Value = Value::Bool(true);
        static RARITY: Lazy<Value> = Lazy::new(|| Value::String("Normal".to_string()));
        static SLOT_MAPPING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static VENDOR_ACTION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
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
        static QUICKBAR_ITEM: Value = Value::Bool(false);
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
        static CRYSTA_EFFECTS: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static CRYSTAL_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CYAN_SOMA_REQUIRED: Value = Value::Int(0i32);
        static DESTROY_METHOD: Lazy<Value> = Lazy::new(|| Value::String(
            "CannotDestroy".to_string(),
        ));
        static DIALOGS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static EXPIRE_BUY_BACK: Value = Value::Int64(0i64);
        static EXPIRE_TIME: Value = Value::Int64(0i64);
        static FREQ: Value = Value::Int(0i32);
        static GAME_CASH_PRICE: Value = Value::Int(0i32);
        static GREEN_SOMA_REQUIRED: Value = Value::Int(0i32);
        static INFINITE_USE: Value = Value::Bool(false);
        static INIT_LEFT_TIME: Value = Value::Int(0i32);
        static INVENTORY_SLOT_INDEX: Value = Value::Int(-1i32);
        static IS_COLLECT_FACTION: Value = Value::Bool(false);
        static IS_EQUIPED: Value = Value::Bool(false);
        static IS_FACTION_ITEM: Value = Value::Bool(false);
        static IS_GEME_CRYSTAL: Value = Value::Bool(false);
        static IS_HOT_SELLER: Value = Value::Bool(false);
        static IS_IN_STOCK: Value = Value::Bool(false);
        static IS_NEW_TO_SHOP: Value = Value::Bool(false);
        static IS_QUEST_ITEM: Value = Value::Bool(false);
        static IS_RECIPE: Value = Value::Bool(false);
        static IS_SOMA_SEED: Value = Value::Bool(false);
        static IS_SOUL_BOUNDED: Value = Value::Bool(false);
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
        static RED_SOMA_REQUIRED: Value = Value::Int(0i32);
        static RENTAL_DURATION_MAX: Value = Value::Float(30f32);
        static RENTAL_DURATION_MIN: Value = Value::Float(0f32);
        static RENT_DISCOUNT: Value = Value::Float(1f32);
        static RENT_PRICE_BLING: Value = Value::Float(0f32);
        static RENT_PRICE_GAME_CASH: Value = Value::Float(0f32);
        static SELL_PRICE_BLING: Value = Value::Int(0i32);
        static SLOT_ID: Value = Value::Int(-1i32);
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
        static VIOLET_SOMA_REQUIRED: Value = Value::Int(0i32);
        static YELLOW_SOMA_REQUIRED: Value = Value::Int(0i32);
        match self {
            Self::CreationTime => &CREATION_TIME,
            Self::Description => &DESCRIPTION,
            Self::DisplayName => &DISPLAY_NAME,
            Self::EquipSlot => &EQUIP_SLOT,
            Self::Icon => &ICON,
            Self::IsInGlobalShop => &IS_IN_GLOBAL_SHOP,
            Self::IsTechApproved => &IS_TECH_APPROVED,
            Self::Rarity => &RARITY,
            Self::SlotMapping => &SLOT_MAPPING,
            Self::VendorAction => &VENDOR_ACTION,
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
            Self::QuickbarItem => &QUICKBAR_ITEM,
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
            Self::CrystaEffects => &CRYSTA_EFFECTS,
            Self::CrystalType => &CRYSTAL_TYPE,
            Self::CyanSomaRequired => &CYAN_SOMA_REQUIRED,
            Self::DestroyMethod => &DESTROY_METHOD,
            Self::Dialogs => &DIALOGS,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::ExpireBuyBack => &EXPIRE_BUY_BACK,
            Self::ExpireTime => &EXPIRE_TIME,
            Self::Freq => &FREQ,
            Self::GameCashPrice => &GAME_CASH_PRICE,
            Self::GreenSomaRequired => &GREEN_SOMA_REQUIRED,
            Self::InfiniteUse => &INFINITE_USE,
            Self::InitLeftTime => &INIT_LEFT_TIME,
            Self::InventorySlotIndex => &INVENTORY_SLOT_INDEX,
            Self::IsCollectFaction => &IS_COLLECT_FACTION,
            Self::IsEquiped => &IS_EQUIPED,
            Self::IsFactionItem => &IS_FACTION_ITEM,
            Self::IsGemeCrystal => &IS_GEME_CRYSTAL,
            Self::IsHotSeller => &IS_HOT_SELLER,
            Self::IsInStock => &IS_IN_STOCK,
            Self::IsNewToShop => &IS_NEW_TO_SHOP,
            Self::IsQuestItem => &IS_QUEST_ITEM,
            Self::IsRecipe => &IS_RECIPE,
            Self::IsSomaSeed => &IS_SOMA_SEED,
            Self::IsSoulBounded => &IS_SOUL_BOUNDED,
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
            Self::RedSomaRequired => &RED_SOMA_REQUIRED,
            Self::RentalDurationMax => &RENTAL_DURATION_MAX,
            Self::RentalDurationMin => &RENTAL_DURATION_MIN,
            Self::RentDiscount => &RENT_DISCOUNT,
            Self::RentPriceBling => &RENT_PRICE_BLING,
            Self::RentPriceGameCash => &RENT_PRICE_GAME_CASH,
            Self::SellPriceBling => &SELL_PRICE_BLING,
            Self::SlotId => &SLOT_ID,
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
            Self::VioletSomaRequired => &VIOLET_SOMA_REQUIRED,
            Self::YellowSomaRequired => &YELLOW_SOMA_REQUIRED,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::CreationTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
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
            Self::DisplayName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::EquipSlot => {
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
            Self::IsInGlobalShop => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::IsTechApproved => {
                &[ParamFlag::NodeOwn, ParamFlag::Content, ParamFlag::ExcludeFromClient]
            }
            Self::Rarity => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
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
            Self::VendorAction => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
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
            Self::QuickbarItem => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
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
            Self::DestroyMethod => &[ParamFlag::Persistent],
            Self::Dialogs => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
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
        }
    }
}
impl FromStr for ItemEdna {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ITEM_EDNA_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ItemEdna {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12353u16 => Ok(Self::AdditionalItemCount1),
            12352u16 => Ok(Self::AdditionalItemCount2),
            12351u16 => Ok(Self::AdditionalItemCount3),
            11645u16 => Ok(Self::AdditionalItemRequired1),
            11644u16 => Ok(Self::AdditionalItemRequired2),
            11643u16 => Ok(Self::AdditionalItemRequired3),
            7577u16 => Ok(Self::AllowBuy),
            7566u16 => Ok(Self::AllowRent),
            7603u16 => Ok(Self::AllowSell),
            11642u16 => Ok(Self::BlackSomaRequired),
            6573u16 => Ok(Self::BlingPrice),
            6572u16 => Ok(Self::BlingSellingPrice),
            11641u16 => Ok(Self::BlueSomaRequired),
            11945u16 => Ok(Self::BonusSlotAmber),
            11946u16 => Ok(Self::BonusSlotRuby),
            11947u16 => Ok(Self::BonusSlotSapphire),
            7604u16 => Ok(Self::BuyDiscount),
            7606u16 => Ok(Self::BuyPriceBling),
            7605u16 => Ok(Self::BuyPriceGameCash),
            7544u16 => Ok(Self::Category),
            8893u16 => Ok(Self::Combos),
            628u16 => Ok(Self::ContainerId),
            655u16 => Ok(Self::ContentClass),
            12191u16 => Ok(Self::CraftingMapping),
            11633u16 => Ok(Self::CraftTime),
            627u16 => Ok(Self::CreationTime),
            11978u16 => Ok(Self::CrystaEffects),
            11980u16 => Ok(Self::CrystalType),
            11640u16 => Ok(Self::CyanSomaRequired),
            6952u16 => Ok(Self::Description),
            6483u16 => Ok(Self::DestroyMethod),
            8920u16 => Ok(Self::Dialogs),
            657u16 => Ok(Self::DisplayName),
            6813u16 => Ok(Self::EnableInGame),
            653u16 => Ok(Self::EquipSlot),
            11607u16 => Ok(Self::ExpireBuyBack),
            7555u16 => Ok(Self::ExpireTime),
            642u16 => Ok(Self::Freq),
            6571u16 => Ok(Self::GameCashPrice),
            11639u16 => Ok(Self::GreenSomaRequired),
            4344u16 => Ok(Self::Icon),
            11461u16 => Ok(Self::InfiniteUse),
            12327u16 => Ok(Self::InitLeftTime),
            9871u16 => Ok(Self::InventorySlotIndex),
            12162u16 => Ok(Self::IsCollectFaction),
            651u16 => Ok(Self::IsEquiped),
            12149u16 => Ok(Self::IsFactionItem),
            11979u16 => Ok(Self::IsGemeCrystal),
            7347u16 => Ok(Self::IsHotSeller),
            7144u16 => Ok(Self::IsInGlobalShop),
            7346u16 => Ok(Self::IsInStock),
            7348u16 => Ok(Self::IsNewToShop),
            9908u16 => Ok(Self::IsQuestItem),
            11634u16 => Ok(Self::IsRecipe),
            12395u16 => Ok(Self::IsSomaSeed),
            10582u16 => Ok(Self::IsSoulBounded),
            9374u16 => Ok(Self::IsTechApproved),
            7746u16 => Ok(Self::IsTrialItem),
            11646u16 => Ok(Self::ItemCritVar),
            11647u16 => Ok(Self::ItemNormalVar),
            9012u16 => Ok(Self::LastUseTime),
            12328u16 => Ok(Self::LeftTime),
            5992u16 => Ok(Self::LootAction),
            10153u16 => Ok(Self::Lua),
            6172u16 => Ok(Self::Lvl),
            647u16 => Ok(Self::LvlReq),
            4723u16 => Ok(Self::MaterialOverride),
            9887u16 => Ok(Self::MaxStackSize),
            11638u16 => Ok(Self::OrangeSomaRequired),
            643u16 => Ok(Self::Power),
            6432u16 => Ok(Self::Quantity),
            7714u16 => Ok(Self::QuestTrigger),
            6277u16 => Ok(Self::Rarity),
            11637u16 => Ok(Self::RedSomaRequired),
            7452u16 => Ok(Self::RentalDurationMax),
            7453u16 => Ok(Self::RentalDurationMin),
            7607u16 => Ok(Self::RentDiscount),
            7609u16 => Ok(Self::RentPriceBling),
            7608u16 => Ok(Self::RentPriceGameCash),
            7602u16 => Ok(Self::SellPriceBling),
            644u16 => Ok(Self::SlotId),
            6246u16 => Ok(Self::SlotMapping),
            12394u16 => Ok(Self::SomaType),
            12257u16 => Ok(Self::SoulBoundedAccountId),
            10612u16 => Ok(Self::SoulBoundedAvatarId),
            12246u16 => Ok(Self::SoulBoundedToAccount),
            10581u16 => Ok(Self::SoulBoundType),
            9886u16 => Ok(Self::StackCount),
            12161u16 => Ok(Self::StandingReq),
            6017u16 => Ok(Self::UseAction),
            8992u16 => Ok(Self::UseCoolDownTimer),
            9023u16 => Ok(Self::UseCount),
            8993u16 => Ok(Self::UseMaxCount),
            8952u16 => Ok(Self::UseRequireAvatar),
            8951u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8953u16 => Ok(Self::UseRequireTarget),
            8950u16 => Ok(Self::UseScript),
            6482u16 => Ok(Self::Vendorable),
            5933u16 => Ok(Self::VendorAction),
            11636u16 => Ok(Self::VioletSomaRequired),
            11635u16 => Ok(Self::YellowSomaRequired),
            639u16 => Ok(Self::Abilities),
            632u16 => Ok(Self::AbilityInstanceData),
            11485u16 => Ok(Self::Agility),
            11472u16 => Ok(Self::Armor),
            11476u16 => Ok(Self::AttackPowerRating),
            6383u16 => Ok(Self::AttributeOp1),
            6382u16 => Ok(Self::AttributeOp2),
            6381u16 => Ok(Self::AttributeOp3),
            6380u16 => Ok(Self::AttributeOp4),
            6387u16 => Ok(Self::AttributeType1),
            6386u16 => Ok(Self::AttributeType2),
            6385u16 => Ok(Self::AttributeType3),
            6384u16 => Ok(Self::AttributeType4),
            6379u16 => Ok(Self::AttributeWeight1),
            6378u16 => Ok(Self::AttributeWeight2),
            6377u16 => Ok(Self::AttributeWeight3),
            6376u16 => Ok(Self::AttributeWeight4),
            9458u16 => Ok(Self::AutoAttributeType1),
            9457u16 => Ok(Self::AutoAttributeType2),
            9456u16 => Ok(Self::AutoAttributeType3),
            9455u16 => Ok(Self::AutoAttributeType4),
            9540u16 => Ok(Self::AutoAttributeType5),
            9539u16 => Ok(Self::AutoAttributeType6),
            9452u16 => Ok(Self::AutoAttributeValue1),
            9451u16 => Ok(Self::AutoAttributeValue2),
            9450u16 => Ok(Self::AutoAttributeValue3),
            9449u16 => Ok(Self::AutoAttributeValue4),
            9538u16 => Ok(Self::AutoAttributeValue5),
            9537u16 => Ok(Self::AutoAttributeValue6),
            10077u16 => Ok(Self::AvailableSockets),
            11473u16 => Ok(Self::BlockRating),
            12029u16 => Ok(Self::ClanName),
            4246u16 => Ok(Self::CombatStyle),
            11477u16 => Ok(Self::CritDamageRating),
            11481u16 => Ok(Self::CritHitRating),
            9988u16 => Ok(Self::Disguise),
            12203u16 => Ok(Self::DisplayNameColor),
            12202u16 => Ok(Self::DisplayNameNumber),
            12204u16 => Ok(Self::DisplayNameRarity),
            12206u16 => Ok(Self::DisplayNameSlot),
            12205u16 => Ok(Self::DisplayNameStat),
            11475u16 => Ok(Self::DodgeRating),
            10018u16 => Ok(Self::Durability),
            10093u16 => Ok(Self::DurabilityCurrent),
            11484u16 => Ok(Self::Focus),
            11015u16 => Ok(Self::Gender),
            11479u16 => Ok(Self::HeavyRating),
            11482u16 => Ok(Self::HitRating),
            652u16 => Ok(Self::IsBanked),
            630u16 => Ok(Self::IsExpired),
            10190u16 => Ok(Self::IsSku),
            9701u16 => Ok(Self::IsTemplate),
            10387u16 => Ok(Self::ItemActionGenericParam),
            10896u16 => Ok(Self::LevelDropVariance),
            4811u16 => Ok(Self::MaxUse),
            12208u16 => Ok(Self::NoteCaption),
            12207u16 => Ok(Self::NoteCaptionValue),
            633u16 => Ok(Self::OtherClientInterests),
            11474u16 => Ok(Self::ParryRating),
            11480u16 => Ok(Self::PeneRating),
            9700u16 => Ok(Self::Prefix),
            12270u16 => Ok(Self::QuickbarItem),
            10098u16 => Ok(Self::RepairCost),
            10103u16 => Ok(Self::SchematicCostToCreateItem),
            10069u16 => Ok(Self::SearchKeywords),
            12381u16 => Ok(Self::SetBonuses),
            12030u16 => Ok(Self::SignOfAvatars),
            10606u16 => Ok(Self::Skuid),
            10080u16 => Ok(Self::SocketLockedStatus),
            10078u16 => Ok(Self::SocketOccupancyStatus),
            10079u16 => Ok(Self::SocketUpgradeLevel),
            11478u16 => Ok(Self::SpecialRating),
            11483u16 => Ok(Self::Stamina),
            11486u16 => Ok(Self::Strength),
            9699u16 => Ok(Self::Suffix),
            10073u16 => Ok(Self::TemplateType),
            11310u16 => Ok(Self::TemplateVersion),
            629u16 => Ok(Self::TimeStamp),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
