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
pub enum SomaforgeItem {
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
    SomaforgeAttributes,
    Type,
}
pub(crate) static SOMAFORGE_ITEM_ATTRIBUTES: phf::Map<&'static str, SomaforgeItem> = phf_map! {
    "AdditionalItemCount1" => SomaforgeItem::AdditionalItemCount1, "AdditionalItemCount2"
    => SomaforgeItem::AdditionalItemCount2, "AdditionalItemCount3" =>
    SomaforgeItem::AdditionalItemCount3, "AdditionalItemRequired1" =>
    SomaforgeItem::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    SomaforgeItem::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    SomaforgeItem::AdditionalItemRequired3, "AllowBuy" => SomaforgeItem::AllowBuy,
    "AllowRent" => SomaforgeItem::AllowRent, "AllowSell" => SomaforgeItem::AllowSell,
    "BlackSomaRequired" => SomaforgeItem::BlackSomaRequired, "blingPrice" =>
    SomaforgeItem::BlingPrice, "blingSellingPrice" => SomaforgeItem::BlingSellingPrice,
    "BlueSomaRequired" => SomaforgeItem::BlueSomaRequired, "BonusSlotAmber" =>
    SomaforgeItem::BonusSlotAmber, "BonusSlotRuby" => SomaforgeItem::BonusSlotRuby,
    "BonusSlotSapphire" => SomaforgeItem::BonusSlotSapphire, "BuyDiscount" =>
    SomaforgeItem::BuyDiscount, "BuyPriceBling" => SomaforgeItem::BuyPriceBling,
    "BuyPriceGameCash" => SomaforgeItem::BuyPriceGameCash, "Category" =>
    SomaforgeItem::Category, "combos" => SomaforgeItem::Combos, "containerID" =>
    SomaforgeItem::ContainerId, "ContentClass" => SomaforgeItem::ContentClass,
    "CraftingMapping" => SomaforgeItem::CraftingMapping, "CraftTime" =>
    SomaforgeItem::CraftTime, "creationTime" => SomaforgeItem::CreationTime,
    "CrystaEffects" => SomaforgeItem::CrystaEffects, "CrystalType" =>
    SomaforgeItem::CrystalType, "CyanSomaRequired" => SomaforgeItem::CyanSomaRequired,
    "Description" => SomaforgeItem::Description, "DestroyMethod" =>
    SomaforgeItem::DestroyMethod, "Dialogs" => SomaforgeItem::Dialogs, "DisplayName" =>
    SomaforgeItem::DisplayName, "EnableInGame" => SomaforgeItem::EnableInGame,
    "equipSlot" => SomaforgeItem::EquipSlot, "expireBuyBack" =>
    SomaforgeItem::ExpireBuyBack, "ExpireTime" => SomaforgeItem::ExpireTime, "Freq" =>
    SomaforgeItem::Freq, "gameCashPrice" => SomaforgeItem::GameCashPrice,
    "GreenSomaRequired" => SomaforgeItem::GreenSomaRequired, "Icon" =>
    SomaforgeItem::Icon, "InfiniteUse" => SomaforgeItem::InfiniteUse, "InitLeftTime" =>
    SomaforgeItem::InitLeftTime, "inventorySlotIndex" =>
    SomaforgeItem::InventorySlotIndex, "isCollectFaction" =>
    SomaforgeItem::IsCollectFaction, "isEquiped" => SomaforgeItem::IsEquiped,
    "isFactionItem" => SomaforgeItem::IsFactionItem, "isGemeCrystal" =>
    SomaforgeItem::IsGemeCrystal, "IsHotSeller" => SomaforgeItem::IsHotSeller,
    "isInGlobalShop" => SomaforgeItem::IsInGlobalShop, "IsInStock" =>
    SomaforgeItem::IsInStock, "IsNewToShop" => SomaforgeItem::IsNewToShop, "isQuestItem"
    => SomaforgeItem::IsQuestItem, "IsRecipe" => SomaforgeItem::IsRecipe, "IsSomaSeed" =>
    SomaforgeItem::IsSomaSeed, "IsSoulBounded" => SomaforgeItem::IsSoulBounded,
    "isTechApproved" => SomaforgeItem::IsTechApproved, "isTrialItem" =>
    SomaforgeItem::IsTrialItem, "ItemCritVar" => SomaforgeItem::ItemCritVar,
    "ItemNormalVar" => SomaforgeItem::ItemNormalVar, "LastUseTime" =>
    SomaforgeItem::LastUseTime, "LeftTime" => SomaforgeItem::LeftTime, "lootAction" =>
    SomaforgeItem::LootAction, "Lua" => SomaforgeItem::Lua, "lvl" => SomaforgeItem::Lvl,
    "lvlReq" => SomaforgeItem::LvlReq, "MaterialOverride" =>
    SomaforgeItem::MaterialOverride, "maxStackSize" => SomaforgeItem::MaxStackSize,
    "OrangeSomaRequired" => SomaforgeItem::OrangeSomaRequired, "Power" =>
    SomaforgeItem::Power, "quantity" => SomaforgeItem::Quantity, "QuestTrigger" =>
    SomaforgeItem::QuestTrigger, "rarity" => SomaforgeItem::Rarity, "RedSomaRequired" =>
    SomaforgeItem::RedSomaRequired, "RentalDurationMax" =>
    SomaforgeItem::RentalDurationMax, "RentalDurationMin" =>
    SomaforgeItem::RentalDurationMin, "RentDiscount" => SomaforgeItem::RentDiscount,
    "RentPriceBling" => SomaforgeItem::RentPriceBling, "RentPriceGameCash" =>
    SomaforgeItem::RentPriceGameCash, "SellPriceBling" => SomaforgeItem::SellPriceBling,
    "slotID" => SomaforgeItem::SlotId, "SlotMapping" => SomaforgeItem::SlotMapping,
    "SomaType" => SomaforgeItem::SomaType, "SoulBoundedAccountId" =>
    SomaforgeItem::SoulBoundedAccountId, "SoulBoundedAvatarId" =>
    SomaforgeItem::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    SomaforgeItem::SoulBoundedToAccount, "SoulBoundType" => SomaforgeItem::SoulBoundType,
    "stackCount" => SomaforgeItem::StackCount, "standingReq" =>
    SomaforgeItem::StandingReq, "useAction" => SomaforgeItem::UseAction,
    "UseCoolDownTimer" => SomaforgeItem::UseCoolDownTimer, "UseCount" =>
    SomaforgeItem::UseCount, "UseMaxCount" => SomaforgeItem::UseMaxCount,
    "UseRequireAvatar" => SomaforgeItem::UseRequireAvatar, "UseRequireAvatarWithinRadius"
    => SomaforgeItem::UseRequireAvatarWithinRadius, "UseRequireTarget" =>
    SomaforgeItem::UseRequireTarget, "UseScript" => SomaforgeItem::UseScript,
    "Vendorable" => SomaforgeItem::Vendorable, "vendorAction" =>
    SomaforgeItem::VendorAction, "VioletSomaRequired" =>
    SomaforgeItem::VioletSomaRequired, "YellowSomaRequired" =>
    SomaforgeItem::YellowSomaRequired, "abilities" => SomaforgeItem::Abilities,
    "abilityInstanceData" => SomaforgeItem::AbilityInstanceData, "Agility" =>
    SomaforgeItem::Agility, "Armor" => SomaforgeItem::Armor, "AttackPowerRating" =>
    SomaforgeItem::AttackPowerRating, "attributeOp1" => SomaforgeItem::AttributeOp1,
    "attributeOp2" => SomaforgeItem::AttributeOp2, "attributeOp3" =>
    SomaforgeItem::AttributeOp3, "attributeOp4" => SomaforgeItem::AttributeOp4,
    "attributeType1" => SomaforgeItem::AttributeType1, "attributeType2" =>
    SomaforgeItem::AttributeType2, "attributeType3" => SomaforgeItem::AttributeType3,
    "attributeType4" => SomaforgeItem::AttributeType4, "attributeWeight1" =>
    SomaforgeItem::AttributeWeight1, "attributeWeight2" =>
    SomaforgeItem::AttributeWeight2, "attributeWeight3" =>
    SomaforgeItem::AttributeWeight3, "attributeWeight4" =>
    SomaforgeItem::AttributeWeight4, "autoAttributeType1" =>
    SomaforgeItem::AutoAttributeType1, "autoAttributeType2" =>
    SomaforgeItem::AutoAttributeType2, "autoAttributeType3" =>
    SomaforgeItem::AutoAttributeType3, "autoAttributeType4" =>
    SomaforgeItem::AutoAttributeType4, "autoAttributeType5" =>
    SomaforgeItem::AutoAttributeType5, "autoAttributeType6" =>
    SomaforgeItem::AutoAttributeType6, "autoAttributeValue1" =>
    SomaforgeItem::AutoAttributeValue1, "autoAttributeValue2" =>
    SomaforgeItem::AutoAttributeValue2, "autoAttributeValue3" =>
    SomaforgeItem::AutoAttributeValue3, "autoAttributeValue4" =>
    SomaforgeItem::AutoAttributeValue4, "autoAttributeValue5" =>
    SomaforgeItem::AutoAttributeValue5, "autoAttributeValue6" =>
    SomaforgeItem::AutoAttributeValue6, "availableSockets" =>
    SomaforgeItem::AvailableSockets, "BlockRating" => SomaforgeItem::BlockRating,
    "ClanName" => SomaforgeItem::ClanName, "combatStyle" => SomaforgeItem::CombatStyle,
    "CritDamageRating" => SomaforgeItem::CritDamageRating, "CritHitRating" =>
    SomaforgeItem::CritHitRating, "disguise" => SomaforgeItem::Disguise,
    "DisplayName_Color" => SomaforgeItem::DisplayNameColor, "DisplayName_Number" =>
    SomaforgeItem::DisplayNameNumber, "DisplayName_Rarity" =>
    SomaforgeItem::DisplayNameRarity, "DisplayName_Slot" =>
    SomaforgeItem::DisplayNameSlot, "DisplayName_Stat" => SomaforgeItem::DisplayNameStat,
    "DodgeRating" => SomaforgeItem::DodgeRating, "durability" =>
    SomaforgeItem::Durability, "durabilityCurrent" => SomaforgeItem::DurabilityCurrent,
    "Focus" => SomaforgeItem::Focus, "gender" => SomaforgeItem::Gender, "HeavyRating" =>
    SomaforgeItem::HeavyRating, "HitRating" => SomaforgeItem::HitRating, "isBanked" =>
    SomaforgeItem::IsBanked, "isExpired" => SomaforgeItem::IsExpired, "isSKU" =>
    SomaforgeItem::IsSku, "IsTemplate" => SomaforgeItem::IsTemplate,
    "itemActionGenericParam" => SomaforgeItem::ItemActionGenericParam,
    "levelDropVariance" => SomaforgeItem::LevelDropVariance, "MaxUse" =>
    SomaforgeItem::MaxUse, "NoteCaption" => SomaforgeItem::NoteCaption,
    "NoteCaptionValue" => SomaforgeItem::NoteCaptionValue, "otherClientInterests" =>
    SomaforgeItem::OtherClientInterests, "ParryRating" => SomaforgeItem::ParryRating,
    "PeneRating" => SomaforgeItem::PeneRating, "Prefix" => SomaforgeItem::Prefix,
    "QuickbarItem" => SomaforgeItem::QuickbarItem, "repairCost" =>
    SomaforgeItem::RepairCost, "schematic_CostToCreateItem" =>
    SomaforgeItem::SchematicCostToCreateItem, "searchKeywords" =>
    SomaforgeItem::SearchKeywords, "setBonuses" => SomaforgeItem::SetBonuses,
    "SignOfAvatars" => SomaforgeItem::SignOfAvatars, "SKUID" => SomaforgeItem::Skuid,
    "socketLockedStatus" => SomaforgeItem::SocketLockedStatus, "socketOccupancyStatus" =>
    SomaforgeItem::SocketOccupancyStatus, "socketUpgradeLevel" =>
    SomaforgeItem::SocketUpgradeLevel, "SpecialRating" => SomaforgeItem::SpecialRating,
    "Stamina" => SomaforgeItem::Stamina, "Strength" => SomaforgeItem::Strength, "Suffix"
    => SomaforgeItem::Suffix, "templateType" => SomaforgeItem::TemplateType,
    "templateVersion" => SomaforgeItem::TemplateVersion, "timeStamp" =>
    SomaforgeItem::TimeStamp, "SomaforgeAttributes" =>
    SomaforgeItem::SomaforgeAttributes, "Type" => SomaforgeItem::Type,
};
pub(crate) static SOMAFORGE_ITEM_ATTRIBUTES_ID: phf::Map<u16, SomaforgeItem> = phf_map! {
    12359u16 => SomaforgeItem::AdditionalItemCount1, 12358u16 =>
    SomaforgeItem::AdditionalItemCount2, 12357u16 => SomaforgeItem::AdditionalItemCount3,
    11675u16 => SomaforgeItem::AdditionalItemRequired1, 11674u16 =>
    SomaforgeItem::AdditionalItemRequired2, 11673u16 =>
    SomaforgeItem::AdditionalItemRequired3, 9249u16 => SomaforgeItem::AllowBuy, 9250u16
    => SomaforgeItem::AllowRent, 9247u16 => SomaforgeItem::AllowSell, 11672u16 =>
    SomaforgeItem::BlackSomaRequired, 9268u16 => SomaforgeItem::BlingPrice, 9269u16 =>
    SomaforgeItem::BlingSellingPrice, 11671u16 => SomaforgeItem::BlueSomaRequired,
    11951u16 => SomaforgeItem::BonusSlotAmber, 11952u16 => SomaforgeItem::BonusSlotRuby,
    11953u16 => SomaforgeItem::BonusSlotSapphire, 9246u16 => SomaforgeItem::BuyDiscount,
    9244u16 => SomaforgeItem::BuyPriceBling, 9245u16 => SomaforgeItem::BuyPriceGameCash,
    9252u16 => SomaforgeItem::Category, 9233u16 => SomaforgeItem::Combos, 9291u16 =>
    SomaforgeItem::ContainerId, 9289u16 => SomaforgeItem::ContentClass, 12193u16 =>
    SomaforgeItem::CraftingMapping, 11663u16 => SomaforgeItem::CraftTime, 9292u16 =>
    SomaforgeItem::CreationTime, 11984u16 => SomaforgeItem::CrystaEffects, 11986u16 =>
    SomaforgeItem::CrystalType, 11670u16 => SomaforgeItem::CyanSomaRequired, 9264u16 =>
    SomaforgeItem::Description, 9271u16 => SomaforgeItem::DestroyMethod, 9232u16 =>
    SomaforgeItem::Dialogs, 9290u16 => SomaforgeItem::DisplayName, 9265u16 =>
    SomaforgeItem::EnableInGame, 9288u16 => SomaforgeItem::EquipSlot, 11609u16 =>
    SomaforgeItem::ExpireBuyBack, 9251u16 => SomaforgeItem::ExpireTime, 9283u16 =>
    SomaforgeItem::Freq, 9270u16 => SomaforgeItem::GameCashPrice, 11669u16 =>
    SomaforgeItem::GreenSomaRequired, 9282u16 => SomaforgeItem::Icon, 11463u16 =>
    SomaforgeItem::InfiniteUse, 12331u16 => SomaforgeItem::InitLeftTime, 9879u16 =>
    SomaforgeItem::InventorySlotIndex, 12166u16 => SomaforgeItem::IsCollectFaction,
    9287u16 => SomaforgeItem::IsEquiped, 12151u16 => SomaforgeItem::IsFactionItem,
    11985u16 => SomaforgeItem::IsGemeCrystal, 9260u16 => SomaforgeItem::IsHotSeller,
    9262u16 => SomaforgeItem::IsInGlobalShop, 9261u16 => SomaforgeItem::IsInStock,
    9259u16 => SomaforgeItem::IsNewToShop, 9916u16 => SomaforgeItem::IsQuestItem,
    11664u16 => SomaforgeItem::IsRecipe, 12399u16 => SomaforgeItem::IsSomaSeed, 10598u16
    => SomaforgeItem::IsSoulBounded, 9382u16 => SomaforgeItem::IsTechApproved, 9237u16 =>
    SomaforgeItem::IsTrialItem, 11676u16 => SomaforgeItem::ItemCritVar, 11677u16 =>
    SomaforgeItem::ItemNormalVar, 9225u16 => SomaforgeItem::LastUseTime, 12332u16 =>
    SomaforgeItem::LeftTime, 9279u16 => SomaforgeItem::LootAction, 10161u16 =>
    SomaforgeItem::Lua, 9277u16 => SomaforgeItem::Lvl, 9286u16 => SomaforgeItem::LvlReq,
    9281u16 => SomaforgeItem::MaterialOverride, 9903u16 => SomaforgeItem::MaxStackSize,
    11668u16 => SomaforgeItem::OrangeSomaRequired, 9284u16 => SomaforgeItem::Power,
    9274u16 => SomaforgeItem::Quantity, 9239u16 => SomaforgeItem::QuestTrigger, 9275u16
    => SomaforgeItem::Rarity, 11667u16 => SomaforgeItem::RedSomaRequired, 9254u16 =>
    SomaforgeItem::RentalDurationMax, 9253u16 => SomaforgeItem::RentalDurationMin,
    9243u16 => SomaforgeItem::RentDiscount, 9241u16 => SomaforgeItem::RentPriceBling,
    9242u16 => SomaforgeItem::RentPriceGameCash, 9248u16 =>
    SomaforgeItem::SellPriceBling, 9285u16 => SomaforgeItem::SlotId, 9276u16 =>
    SomaforgeItem::SlotMapping, 12398u16 => SomaforgeItem::SomaType, 12259u16 =>
    SomaforgeItem::SoulBoundedAccountId, 10620u16 => SomaforgeItem::SoulBoundedAvatarId,
    12248u16 => SomaforgeItem::SoulBoundedToAccount, 10597u16 =>
    SomaforgeItem::SoulBoundType, 9902u16 => SomaforgeItem::StackCount, 12165u16 =>
    SomaforgeItem::StandingReq, 9278u16 => SomaforgeItem::UseAction, 9227u16 =>
    SomaforgeItem::UseCoolDownTimer, 9224u16 => SomaforgeItem::UseCount, 9226u16 =>
    SomaforgeItem::UseMaxCount, 9229u16 => SomaforgeItem::UseRequireAvatar, 9230u16 =>
    SomaforgeItem::UseRequireAvatarWithinRadius, 9228u16 =>
    SomaforgeItem::UseRequireTarget, 9231u16 => SomaforgeItem::UseScript, 9272u16 =>
    SomaforgeItem::Vendorable, 9280u16 => SomaforgeItem::VendorAction, 11666u16 =>
    SomaforgeItem::VioletSomaRequired, 11665u16 => SomaforgeItem::YellowSomaRequired,
    11406u16 => SomaforgeItem::Abilities, 11407u16 => SomaforgeItem::AbilityInstanceData,
    11515u16 => SomaforgeItem::Agility, 11502u16 => SomaforgeItem::Armor, 11506u16 =>
    SomaforgeItem::AttackPowerRating, 11408u16 => SomaforgeItem::AttributeOp1, 11409u16
    => SomaforgeItem::AttributeOp2, 11410u16 => SomaforgeItem::AttributeOp3, 11411u16 =>
    SomaforgeItem::AttributeOp4, 11412u16 => SomaforgeItem::AttributeType1, 11413u16 =>
    SomaforgeItem::AttributeType2, 11414u16 => SomaforgeItem::AttributeType3, 11415u16 =>
    SomaforgeItem::AttributeType4, 11416u16 => SomaforgeItem::AttributeWeight1, 11417u16
    => SomaforgeItem::AttributeWeight2, 11418u16 => SomaforgeItem::AttributeWeight3,
    11419u16 => SomaforgeItem::AttributeWeight4, 11420u16 =>
    SomaforgeItem::AutoAttributeType1, 11421u16 => SomaforgeItem::AutoAttributeType2,
    11422u16 => SomaforgeItem::AutoAttributeType3, 11423u16 =>
    SomaforgeItem::AutoAttributeType4, 11424u16 => SomaforgeItem::AutoAttributeType5,
    11425u16 => SomaforgeItem::AutoAttributeType6, 11426u16 =>
    SomaforgeItem::AutoAttributeValue1, 11427u16 => SomaforgeItem::AutoAttributeValue2,
    11428u16 => SomaforgeItem::AutoAttributeValue3, 11429u16 =>
    SomaforgeItem::AutoAttributeValue4, 11430u16 => SomaforgeItem::AutoAttributeValue5,
    11431u16 => SomaforgeItem::AutoAttributeValue6, 11432u16 =>
    SomaforgeItem::AvailableSockets, 11503u16 => SomaforgeItem::BlockRating, 12033u16 =>
    SomaforgeItem::ClanName, 11433u16 => SomaforgeItem::CombatStyle, 11507u16 =>
    SomaforgeItem::CritDamageRating, 11511u16 => SomaforgeItem::CritHitRating, 11434u16
    => SomaforgeItem::Disguise, 12217u16 => SomaforgeItem::DisplayNameColor, 12216u16 =>
    SomaforgeItem::DisplayNameNumber, 12218u16 => SomaforgeItem::DisplayNameRarity,
    12220u16 => SomaforgeItem::DisplayNameSlot, 12219u16 =>
    SomaforgeItem::DisplayNameStat, 11505u16 => SomaforgeItem::DodgeRating, 11435u16 =>
    SomaforgeItem::Durability, 11436u16 => SomaforgeItem::DurabilityCurrent, 11514u16 =>
    SomaforgeItem::Focus, 11437u16 => SomaforgeItem::Gender, 11509u16 =>
    SomaforgeItem::HeavyRating, 11512u16 => SomaforgeItem::HitRating, 11438u16 =>
    SomaforgeItem::IsBanked, 11439u16 => SomaforgeItem::IsExpired, 11440u16 =>
    SomaforgeItem::IsSku, 11441u16 => SomaforgeItem::IsTemplate, 11442u16 =>
    SomaforgeItem::ItemActionGenericParam, 11443u16 => SomaforgeItem::LevelDropVariance,
    11444u16 => SomaforgeItem::MaxUse, 12222u16 => SomaforgeItem::NoteCaption, 12221u16
    => SomaforgeItem::NoteCaptionValue, 11445u16 => SomaforgeItem::OtherClientInterests,
    11504u16 => SomaforgeItem::ParryRating, 11510u16 => SomaforgeItem::PeneRating,
    11446u16 => SomaforgeItem::Prefix, 12272u16 => SomaforgeItem::QuickbarItem, 11447u16
    => SomaforgeItem::RepairCost, 11448u16 => SomaforgeItem::SchematicCostToCreateItem,
    11449u16 => SomaforgeItem::SearchKeywords, 12383u16 => SomaforgeItem::SetBonuses,
    12034u16 => SomaforgeItem::SignOfAvatars, 11450u16 => SomaforgeItem::Skuid, 11451u16
    => SomaforgeItem::SocketLockedStatus, 11452u16 =>
    SomaforgeItem::SocketOccupancyStatus, 11453u16 => SomaforgeItem::SocketUpgradeLevel,
    11508u16 => SomaforgeItem::SpecialRating, 11513u16 => SomaforgeItem::Stamina,
    11516u16 => SomaforgeItem::Strength, 11454u16 => SomaforgeItem::Suffix, 11455u16 =>
    SomaforgeItem::TemplateType, 11456u16 => SomaforgeItem::TemplateVersion, 11457u16 =>
    SomaforgeItem::TimeStamp, 9620u16 => SomaforgeItem::SomaforgeAttributes, 9387u16 =>
    SomaforgeItem::Type,
};
impl Attribute for SomaforgeItem {
    fn class() -> Class {
        Class::SomaforgeItem
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
            Self::SomaforgeAttributes => &Self::SomaforgeAttributes,
            Self::Type => &Self::Type,
        }
    }
}
impl AttributeInfo for SomaforgeItem {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12359u16,
            Self::AdditionalItemCount2 => 12358u16,
            Self::AdditionalItemCount3 => 12357u16,
            Self::AdditionalItemRequired1 => 11675u16,
            Self::AdditionalItemRequired2 => 11674u16,
            Self::AdditionalItemRequired3 => 11673u16,
            Self::AllowBuy => 9249u16,
            Self::AllowRent => 9250u16,
            Self::AllowSell => 9247u16,
            Self::BlackSomaRequired => 11672u16,
            Self::BlingPrice => 9268u16,
            Self::BlingSellingPrice => 9269u16,
            Self::BlueSomaRequired => 11671u16,
            Self::BonusSlotAmber => 11951u16,
            Self::BonusSlotRuby => 11952u16,
            Self::BonusSlotSapphire => 11953u16,
            Self::BuyDiscount => 9246u16,
            Self::BuyPriceBling => 9244u16,
            Self::BuyPriceGameCash => 9245u16,
            Self::Category => 9252u16,
            Self::Combos => 9233u16,
            Self::ContainerId => 9291u16,
            Self::ContentClass => 9289u16,
            Self::CraftingMapping => 12193u16,
            Self::CraftTime => 11663u16,
            Self::CreationTime => 9292u16,
            Self::CrystaEffects => 11984u16,
            Self::CrystalType => 11986u16,
            Self::CyanSomaRequired => 11670u16,
            Self::Description => 9264u16,
            Self::DestroyMethod => 9271u16,
            Self::Dialogs => 9232u16,
            Self::DisplayName => 9290u16,
            Self::EnableInGame => 9265u16,
            Self::EquipSlot => 9288u16,
            Self::ExpireBuyBack => 11609u16,
            Self::ExpireTime => 9251u16,
            Self::Freq => 9283u16,
            Self::GameCashPrice => 9270u16,
            Self::GreenSomaRequired => 11669u16,
            Self::Icon => 9282u16,
            Self::InfiniteUse => 11463u16,
            Self::InitLeftTime => 12331u16,
            Self::InventorySlotIndex => 9879u16,
            Self::IsCollectFaction => 12166u16,
            Self::IsEquiped => 9287u16,
            Self::IsFactionItem => 12151u16,
            Self::IsGemeCrystal => 11985u16,
            Self::IsHotSeller => 9260u16,
            Self::IsInGlobalShop => 9262u16,
            Self::IsInStock => 9261u16,
            Self::IsNewToShop => 9259u16,
            Self::IsQuestItem => 9916u16,
            Self::IsRecipe => 11664u16,
            Self::IsSomaSeed => 12399u16,
            Self::IsSoulBounded => 10598u16,
            Self::IsTechApproved => 9382u16,
            Self::IsTrialItem => 9237u16,
            Self::ItemCritVar => 11676u16,
            Self::ItemNormalVar => 11677u16,
            Self::LastUseTime => 9225u16,
            Self::LeftTime => 12332u16,
            Self::LootAction => 9279u16,
            Self::Lua => 10161u16,
            Self::Lvl => 9277u16,
            Self::LvlReq => 9286u16,
            Self::MaterialOverride => 9281u16,
            Self::MaxStackSize => 9903u16,
            Self::OrangeSomaRequired => 11668u16,
            Self::Power => 9284u16,
            Self::Quantity => 9274u16,
            Self::QuestTrigger => 9239u16,
            Self::Rarity => 9275u16,
            Self::RedSomaRequired => 11667u16,
            Self::RentalDurationMax => 9254u16,
            Self::RentalDurationMin => 9253u16,
            Self::RentDiscount => 9243u16,
            Self::RentPriceBling => 9241u16,
            Self::RentPriceGameCash => 9242u16,
            Self::SellPriceBling => 9248u16,
            Self::SlotId => 9285u16,
            Self::SlotMapping => 9276u16,
            Self::SomaType => 12398u16,
            Self::SoulBoundedAccountId => 12259u16,
            Self::SoulBoundedAvatarId => 10620u16,
            Self::SoulBoundedToAccount => 12248u16,
            Self::SoulBoundType => 10597u16,
            Self::StackCount => 9902u16,
            Self::StandingReq => 12165u16,
            Self::UseAction => 9278u16,
            Self::UseCoolDownTimer => 9227u16,
            Self::UseCount => 9224u16,
            Self::UseMaxCount => 9226u16,
            Self::UseRequireAvatar => 9229u16,
            Self::UseRequireAvatarWithinRadius => 9230u16,
            Self::UseRequireTarget => 9228u16,
            Self::UseScript => 9231u16,
            Self::Vendorable => 9272u16,
            Self::VendorAction => 9280u16,
            Self::VioletSomaRequired => 11666u16,
            Self::YellowSomaRequired => 11665u16,
            Self::Abilities => 11406u16,
            Self::AbilityInstanceData => 11407u16,
            Self::Agility => 11515u16,
            Self::Armor => 11502u16,
            Self::AttackPowerRating => 11506u16,
            Self::AttributeOp1 => 11408u16,
            Self::AttributeOp2 => 11409u16,
            Self::AttributeOp3 => 11410u16,
            Self::AttributeOp4 => 11411u16,
            Self::AttributeType1 => 11412u16,
            Self::AttributeType2 => 11413u16,
            Self::AttributeType3 => 11414u16,
            Self::AttributeType4 => 11415u16,
            Self::AttributeWeight1 => 11416u16,
            Self::AttributeWeight2 => 11417u16,
            Self::AttributeWeight3 => 11418u16,
            Self::AttributeWeight4 => 11419u16,
            Self::AutoAttributeType1 => 11420u16,
            Self::AutoAttributeType2 => 11421u16,
            Self::AutoAttributeType3 => 11422u16,
            Self::AutoAttributeType4 => 11423u16,
            Self::AutoAttributeType5 => 11424u16,
            Self::AutoAttributeType6 => 11425u16,
            Self::AutoAttributeValue1 => 11426u16,
            Self::AutoAttributeValue2 => 11427u16,
            Self::AutoAttributeValue3 => 11428u16,
            Self::AutoAttributeValue4 => 11429u16,
            Self::AutoAttributeValue5 => 11430u16,
            Self::AutoAttributeValue6 => 11431u16,
            Self::AvailableSockets => 11432u16,
            Self::BlockRating => 11503u16,
            Self::ClanName => 12033u16,
            Self::CombatStyle => 11433u16,
            Self::CritDamageRating => 11507u16,
            Self::CritHitRating => 11511u16,
            Self::Disguise => 11434u16,
            Self::DisplayNameColor => 12217u16,
            Self::DisplayNameNumber => 12216u16,
            Self::DisplayNameRarity => 12218u16,
            Self::DisplayNameSlot => 12220u16,
            Self::DisplayNameStat => 12219u16,
            Self::DodgeRating => 11505u16,
            Self::Durability => 11435u16,
            Self::DurabilityCurrent => 11436u16,
            Self::Focus => 11514u16,
            Self::Gender => 11437u16,
            Self::HeavyRating => 11509u16,
            Self::HitRating => 11512u16,
            Self::IsBanked => 11438u16,
            Self::IsExpired => 11439u16,
            Self::IsSku => 11440u16,
            Self::IsTemplate => 11441u16,
            Self::ItemActionGenericParam => 11442u16,
            Self::LevelDropVariance => 11443u16,
            Self::MaxUse => 11444u16,
            Self::NoteCaption => 12222u16,
            Self::NoteCaptionValue => 12221u16,
            Self::OtherClientInterests => 11445u16,
            Self::ParryRating => 11504u16,
            Self::PeneRating => 11510u16,
            Self::Prefix => 11446u16,
            Self::QuickbarItem => 12272u16,
            Self::RepairCost => 11447u16,
            Self::SchematicCostToCreateItem => 11448u16,
            Self::SearchKeywords => 11449u16,
            Self::SetBonuses => 12383u16,
            Self::SignOfAvatars => 12034u16,
            Self::Skuid => 11450u16,
            Self::SocketLockedStatus => 11451u16,
            Self::SocketOccupancyStatus => 11452u16,
            Self::SocketUpgradeLevel => 11453u16,
            Self::SpecialRating => 11508u16,
            Self::Stamina => 11513u16,
            Self::Strength => 11516u16,
            Self::Suffix => 11454u16,
            Self::TemplateType => 11455u16,
            Self::TemplateVersion => 11456u16,
            Self::TimeStamp => 11457u16,
            Self::SomaforgeAttributes => 9620u16,
            Self::Type => 9387u16,
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
            Self::SomaforgeAttributes => "SomaforgeAttributes",
            Self::Type => "Type",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::SomaforgeAttributes => ParamType::JsonValue,
            Self::Type => ParamType::String,
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
            Self::EquipSlot => ParamType::String,
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
        }
    }
    fn default(&self) -> &'static Value {
        static SOMAFORGE_ATTRIBUTES: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static TYPE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
        static EQUIP_SLOT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
        match self {
            Self::SomaforgeAttributes => &SOMAFORGE_ATTRIBUTES,
            Self::Type => &TYPE,
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
            Self::EquipSlot => &EQUIP_SLOT,
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
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::SomaforgeAttributes => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Type => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
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
            Self::EquipSlot => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
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
        }
    }
}
impl FromStr for SomaforgeItem {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SOMAFORGE_ITEM_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for SomaforgeItem {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12359u16 => Ok(Self::AdditionalItemCount1),
            12358u16 => Ok(Self::AdditionalItemCount2),
            12357u16 => Ok(Self::AdditionalItemCount3),
            11675u16 => Ok(Self::AdditionalItemRequired1),
            11674u16 => Ok(Self::AdditionalItemRequired2),
            11673u16 => Ok(Self::AdditionalItemRequired3),
            9249u16 => Ok(Self::AllowBuy),
            9250u16 => Ok(Self::AllowRent),
            9247u16 => Ok(Self::AllowSell),
            11672u16 => Ok(Self::BlackSomaRequired),
            9268u16 => Ok(Self::BlingPrice),
            9269u16 => Ok(Self::BlingSellingPrice),
            11671u16 => Ok(Self::BlueSomaRequired),
            11951u16 => Ok(Self::BonusSlotAmber),
            11952u16 => Ok(Self::BonusSlotRuby),
            11953u16 => Ok(Self::BonusSlotSapphire),
            9246u16 => Ok(Self::BuyDiscount),
            9244u16 => Ok(Self::BuyPriceBling),
            9245u16 => Ok(Self::BuyPriceGameCash),
            9252u16 => Ok(Self::Category),
            9233u16 => Ok(Self::Combos),
            9291u16 => Ok(Self::ContainerId),
            9289u16 => Ok(Self::ContentClass),
            12193u16 => Ok(Self::CraftingMapping),
            11663u16 => Ok(Self::CraftTime),
            9292u16 => Ok(Self::CreationTime),
            11984u16 => Ok(Self::CrystaEffects),
            11986u16 => Ok(Self::CrystalType),
            11670u16 => Ok(Self::CyanSomaRequired),
            9264u16 => Ok(Self::Description),
            9271u16 => Ok(Self::DestroyMethod),
            9232u16 => Ok(Self::Dialogs),
            9290u16 => Ok(Self::DisplayName),
            9265u16 => Ok(Self::EnableInGame),
            9288u16 => Ok(Self::EquipSlot),
            11609u16 => Ok(Self::ExpireBuyBack),
            9251u16 => Ok(Self::ExpireTime),
            9283u16 => Ok(Self::Freq),
            9270u16 => Ok(Self::GameCashPrice),
            11669u16 => Ok(Self::GreenSomaRequired),
            9282u16 => Ok(Self::Icon),
            11463u16 => Ok(Self::InfiniteUse),
            12331u16 => Ok(Self::InitLeftTime),
            9879u16 => Ok(Self::InventorySlotIndex),
            12166u16 => Ok(Self::IsCollectFaction),
            9287u16 => Ok(Self::IsEquiped),
            12151u16 => Ok(Self::IsFactionItem),
            11985u16 => Ok(Self::IsGemeCrystal),
            9260u16 => Ok(Self::IsHotSeller),
            9262u16 => Ok(Self::IsInGlobalShop),
            9261u16 => Ok(Self::IsInStock),
            9259u16 => Ok(Self::IsNewToShop),
            9916u16 => Ok(Self::IsQuestItem),
            11664u16 => Ok(Self::IsRecipe),
            12399u16 => Ok(Self::IsSomaSeed),
            10598u16 => Ok(Self::IsSoulBounded),
            9382u16 => Ok(Self::IsTechApproved),
            9237u16 => Ok(Self::IsTrialItem),
            11676u16 => Ok(Self::ItemCritVar),
            11677u16 => Ok(Self::ItemNormalVar),
            9225u16 => Ok(Self::LastUseTime),
            12332u16 => Ok(Self::LeftTime),
            9279u16 => Ok(Self::LootAction),
            10161u16 => Ok(Self::Lua),
            9277u16 => Ok(Self::Lvl),
            9286u16 => Ok(Self::LvlReq),
            9281u16 => Ok(Self::MaterialOverride),
            9903u16 => Ok(Self::MaxStackSize),
            11668u16 => Ok(Self::OrangeSomaRequired),
            9284u16 => Ok(Self::Power),
            9274u16 => Ok(Self::Quantity),
            9239u16 => Ok(Self::QuestTrigger),
            9275u16 => Ok(Self::Rarity),
            11667u16 => Ok(Self::RedSomaRequired),
            9254u16 => Ok(Self::RentalDurationMax),
            9253u16 => Ok(Self::RentalDurationMin),
            9243u16 => Ok(Self::RentDiscount),
            9241u16 => Ok(Self::RentPriceBling),
            9242u16 => Ok(Self::RentPriceGameCash),
            9248u16 => Ok(Self::SellPriceBling),
            9285u16 => Ok(Self::SlotId),
            9276u16 => Ok(Self::SlotMapping),
            12398u16 => Ok(Self::SomaType),
            12259u16 => Ok(Self::SoulBoundedAccountId),
            10620u16 => Ok(Self::SoulBoundedAvatarId),
            12248u16 => Ok(Self::SoulBoundedToAccount),
            10597u16 => Ok(Self::SoulBoundType),
            9902u16 => Ok(Self::StackCount),
            12165u16 => Ok(Self::StandingReq),
            9278u16 => Ok(Self::UseAction),
            9227u16 => Ok(Self::UseCoolDownTimer),
            9224u16 => Ok(Self::UseCount),
            9226u16 => Ok(Self::UseMaxCount),
            9229u16 => Ok(Self::UseRequireAvatar),
            9230u16 => Ok(Self::UseRequireAvatarWithinRadius),
            9228u16 => Ok(Self::UseRequireTarget),
            9231u16 => Ok(Self::UseScript),
            9272u16 => Ok(Self::Vendorable),
            9280u16 => Ok(Self::VendorAction),
            11666u16 => Ok(Self::VioletSomaRequired),
            11665u16 => Ok(Self::YellowSomaRequired),
            11406u16 => Ok(Self::Abilities),
            11407u16 => Ok(Self::AbilityInstanceData),
            11515u16 => Ok(Self::Agility),
            11502u16 => Ok(Self::Armor),
            11506u16 => Ok(Self::AttackPowerRating),
            11408u16 => Ok(Self::AttributeOp1),
            11409u16 => Ok(Self::AttributeOp2),
            11410u16 => Ok(Self::AttributeOp3),
            11411u16 => Ok(Self::AttributeOp4),
            11412u16 => Ok(Self::AttributeType1),
            11413u16 => Ok(Self::AttributeType2),
            11414u16 => Ok(Self::AttributeType3),
            11415u16 => Ok(Self::AttributeType4),
            11416u16 => Ok(Self::AttributeWeight1),
            11417u16 => Ok(Self::AttributeWeight2),
            11418u16 => Ok(Self::AttributeWeight3),
            11419u16 => Ok(Self::AttributeWeight4),
            11420u16 => Ok(Self::AutoAttributeType1),
            11421u16 => Ok(Self::AutoAttributeType2),
            11422u16 => Ok(Self::AutoAttributeType3),
            11423u16 => Ok(Self::AutoAttributeType4),
            11424u16 => Ok(Self::AutoAttributeType5),
            11425u16 => Ok(Self::AutoAttributeType6),
            11426u16 => Ok(Self::AutoAttributeValue1),
            11427u16 => Ok(Self::AutoAttributeValue2),
            11428u16 => Ok(Self::AutoAttributeValue3),
            11429u16 => Ok(Self::AutoAttributeValue4),
            11430u16 => Ok(Self::AutoAttributeValue5),
            11431u16 => Ok(Self::AutoAttributeValue6),
            11432u16 => Ok(Self::AvailableSockets),
            11503u16 => Ok(Self::BlockRating),
            12033u16 => Ok(Self::ClanName),
            11433u16 => Ok(Self::CombatStyle),
            11507u16 => Ok(Self::CritDamageRating),
            11511u16 => Ok(Self::CritHitRating),
            11434u16 => Ok(Self::Disguise),
            12217u16 => Ok(Self::DisplayNameColor),
            12216u16 => Ok(Self::DisplayNameNumber),
            12218u16 => Ok(Self::DisplayNameRarity),
            12220u16 => Ok(Self::DisplayNameSlot),
            12219u16 => Ok(Self::DisplayNameStat),
            11505u16 => Ok(Self::DodgeRating),
            11435u16 => Ok(Self::Durability),
            11436u16 => Ok(Self::DurabilityCurrent),
            11514u16 => Ok(Self::Focus),
            11437u16 => Ok(Self::Gender),
            11509u16 => Ok(Self::HeavyRating),
            11512u16 => Ok(Self::HitRating),
            11438u16 => Ok(Self::IsBanked),
            11439u16 => Ok(Self::IsExpired),
            11440u16 => Ok(Self::IsSku),
            11441u16 => Ok(Self::IsTemplate),
            11442u16 => Ok(Self::ItemActionGenericParam),
            11443u16 => Ok(Self::LevelDropVariance),
            11444u16 => Ok(Self::MaxUse),
            12222u16 => Ok(Self::NoteCaption),
            12221u16 => Ok(Self::NoteCaptionValue),
            11445u16 => Ok(Self::OtherClientInterests),
            11504u16 => Ok(Self::ParryRating),
            11510u16 => Ok(Self::PeneRating),
            11446u16 => Ok(Self::Prefix),
            12272u16 => Ok(Self::QuickbarItem),
            11447u16 => Ok(Self::RepairCost),
            11448u16 => Ok(Self::SchematicCostToCreateItem),
            11449u16 => Ok(Self::SearchKeywords),
            12383u16 => Ok(Self::SetBonuses),
            12034u16 => Ok(Self::SignOfAvatars),
            11450u16 => Ok(Self::Skuid),
            11451u16 => Ok(Self::SocketLockedStatus),
            11452u16 => Ok(Self::SocketOccupancyStatus),
            11453u16 => Ok(Self::SocketUpgradeLevel),
            11508u16 => Ok(Self::SpecialRating),
            11513u16 => Ok(Self::Stamina),
            11516u16 => Ok(Self::Strength),
            11454u16 => Ok(Self::Suffix),
            11455u16 => Ok(Self::TemplateType),
            11456u16 => Ok(Self::TemplateVersion),
            11457u16 => Ok(Self::TimeStamp),
            9620u16 => Ok(Self::SomaforgeAttributes),
            9387u16 => Ok(Self::Type),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
