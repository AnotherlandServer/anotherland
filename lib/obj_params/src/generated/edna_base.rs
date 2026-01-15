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
pub enum EdnaBase {
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
    Level,
    Name,
}
pub(crate) static EDNA_BASE_ATTRIBUTES: phf::Map<&'static str, EdnaBase> = phf_map! {
    "AdditionalItemCount1" => EdnaBase::AdditionalItemCount1, "AdditionalItemCount2" =>
    EdnaBase::AdditionalItemCount2, "AdditionalItemCount3" =>
    EdnaBase::AdditionalItemCount3, "AdditionalItemRequired1" =>
    EdnaBase::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    EdnaBase::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    EdnaBase::AdditionalItemRequired3, "AllowBuy" => EdnaBase::AllowBuy, "AllowRent" =>
    EdnaBase::AllowRent, "AllowSell" => EdnaBase::AllowSell, "BlackSomaRequired" =>
    EdnaBase::BlackSomaRequired, "blingPrice" => EdnaBase::BlingPrice,
    "blingSellingPrice" => EdnaBase::BlingSellingPrice, "BlueSomaRequired" =>
    EdnaBase::BlueSomaRequired, "BonusSlotAmber" => EdnaBase::BonusSlotAmber,
    "BonusSlotRuby" => EdnaBase::BonusSlotRuby, "BonusSlotSapphire" =>
    EdnaBase::BonusSlotSapphire, "BuyDiscount" => EdnaBase::BuyDiscount, "BuyPriceBling"
    => EdnaBase::BuyPriceBling, "BuyPriceGameCash" => EdnaBase::BuyPriceGameCash,
    "Category" => EdnaBase::Category, "combos" => EdnaBase::Combos, "containerID" =>
    EdnaBase::ContainerId, "ContentClass" => EdnaBase::ContentClass, "CraftingMapping" =>
    EdnaBase::CraftingMapping, "CraftTime" => EdnaBase::CraftTime, "creationTime" =>
    EdnaBase::CreationTime, "CrystaEffects" => EdnaBase::CrystaEffects, "CrystalType" =>
    EdnaBase::CrystalType, "CyanSomaRequired" => EdnaBase::CyanSomaRequired,
    "Description" => EdnaBase::Description, "DestroyMethod" => EdnaBase::DestroyMethod,
    "Dialogs" => EdnaBase::Dialogs, "DisplayName" => EdnaBase::DisplayName,
    "EnableInGame" => EdnaBase::EnableInGame, "equipSlot" => EdnaBase::EquipSlot,
    "expireBuyBack" => EdnaBase::ExpireBuyBack, "ExpireTime" => EdnaBase::ExpireTime,
    "Freq" => EdnaBase::Freq, "gameCashPrice" => EdnaBase::GameCashPrice,
    "GreenSomaRequired" => EdnaBase::GreenSomaRequired, "Icon" => EdnaBase::Icon,
    "InfiniteUse" => EdnaBase::InfiniteUse, "InitLeftTime" => EdnaBase::InitLeftTime,
    "inventorySlotIndex" => EdnaBase::InventorySlotIndex, "isCollectFaction" =>
    EdnaBase::IsCollectFaction, "isEquiped" => EdnaBase::IsEquiped, "isFactionItem" =>
    EdnaBase::IsFactionItem, "isGemeCrystal" => EdnaBase::IsGemeCrystal, "IsHotSeller" =>
    EdnaBase::IsHotSeller, "isInGlobalShop" => EdnaBase::IsInGlobalShop, "IsInStock" =>
    EdnaBase::IsInStock, "IsNewToShop" => EdnaBase::IsNewToShop, "isQuestItem" =>
    EdnaBase::IsQuestItem, "IsRecipe" => EdnaBase::IsRecipe, "IsSomaSeed" =>
    EdnaBase::IsSomaSeed, "IsSoulBounded" => EdnaBase::IsSoulBounded, "isTechApproved" =>
    EdnaBase::IsTechApproved, "isTrialItem" => EdnaBase::IsTrialItem, "ItemCritVar" =>
    EdnaBase::ItemCritVar, "ItemNormalVar" => EdnaBase::ItemNormalVar, "LastUseTime" =>
    EdnaBase::LastUseTime, "LeftTime" => EdnaBase::LeftTime, "lootAction" =>
    EdnaBase::LootAction, "Lua" => EdnaBase::Lua, "lvl" => EdnaBase::Lvl, "lvlReq" =>
    EdnaBase::LvlReq, "MaterialOverride" => EdnaBase::MaterialOverride, "maxStackSize" =>
    EdnaBase::MaxStackSize, "OrangeSomaRequired" => EdnaBase::OrangeSomaRequired, "Power"
    => EdnaBase::Power, "quantity" => EdnaBase::Quantity, "QuestTrigger" =>
    EdnaBase::QuestTrigger, "rarity" => EdnaBase::Rarity, "RedSomaRequired" =>
    EdnaBase::RedSomaRequired, "RentalDurationMax" => EdnaBase::RentalDurationMax,
    "RentalDurationMin" => EdnaBase::RentalDurationMin, "RentDiscount" =>
    EdnaBase::RentDiscount, "RentPriceBling" => EdnaBase::RentPriceBling,
    "RentPriceGameCash" => EdnaBase::RentPriceGameCash, "SellPriceBling" =>
    EdnaBase::SellPriceBling, "slotID" => EdnaBase::SlotId, "SlotMapping" =>
    EdnaBase::SlotMapping, "SomaType" => EdnaBase::SomaType, "SoulBoundedAccountId" =>
    EdnaBase::SoulBoundedAccountId, "SoulBoundedAvatarId" =>
    EdnaBase::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    EdnaBase::SoulBoundedToAccount, "SoulBoundType" => EdnaBase::SoulBoundType,
    "stackCount" => EdnaBase::StackCount, "standingReq" => EdnaBase::StandingReq,
    "useAction" => EdnaBase::UseAction, "UseCoolDownTimer" => EdnaBase::UseCoolDownTimer,
    "UseCount" => EdnaBase::UseCount, "UseMaxCount" => EdnaBase::UseMaxCount,
    "UseRequireAvatar" => EdnaBase::UseRequireAvatar, "UseRequireAvatarWithinRadius" =>
    EdnaBase::UseRequireAvatarWithinRadius, "UseRequireTarget" =>
    EdnaBase::UseRequireTarget, "UseScript" => EdnaBase::UseScript, "Vendorable" =>
    EdnaBase::Vendorable, "vendorAction" => EdnaBase::VendorAction, "VioletSomaRequired"
    => EdnaBase::VioletSomaRequired, "YellowSomaRequired" =>
    EdnaBase::YellowSomaRequired, "abilities" => EdnaBase::Abilities,
    "abilityInstanceData" => EdnaBase::AbilityInstanceData, "Agility" =>
    EdnaBase::Agility, "Armor" => EdnaBase::Armor, "AttackPowerRating" =>
    EdnaBase::AttackPowerRating, "attributeOp1" => EdnaBase::AttributeOp1, "attributeOp2"
    => EdnaBase::AttributeOp2, "attributeOp3" => EdnaBase::AttributeOp3, "attributeOp4"
    => EdnaBase::AttributeOp4, "attributeType1" => EdnaBase::AttributeType1,
    "attributeType2" => EdnaBase::AttributeType2, "attributeType3" =>
    EdnaBase::AttributeType3, "attributeType4" => EdnaBase::AttributeType4,
    "attributeWeight1" => EdnaBase::AttributeWeight1, "attributeWeight2" =>
    EdnaBase::AttributeWeight2, "attributeWeight3" => EdnaBase::AttributeWeight3,
    "attributeWeight4" => EdnaBase::AttributeWeight4, "autoAttributeType1" =>
    EdnaBase::AutoAttributeType1, "autoAttributeType2" => EdnaBase::AutoAttributeType2,
    "autoAttributeType3" => EdnaBase::AutoAttributeType3, "autoAttributeType4" =>
    EdnaBase::AutoAttributeType4, "autoAttributeType5" => EdnaBase::AutoAttributeType5,
    "autoAttributeType6" => EdnaBase::AutoAttributeType6, "autoAttributeValue1" =>
    EdnaBase::AutoAttributeValue1, "autoAttributeValue2" =>
    EdnaBase::AutoAttributeValue2, "autoAttributeValue3" =>
    EdnaBase::AutoAttributeValue3, "autoAttributeValue4" =>
    EdnaBase::AutoAttributeValue4, "autoAttributeValue5" =>
    EdnaBase::AutoAttributeValue5, "autoAttributeValue6" =>
    EdnaBase::AutoAttributeValue6, "availableSockets" => EdnaBase::AvailableSockets,
    "BlockRating" => EdnaBase::BlockRating, "ClanName" => EdnaBase::ClanName,
    "combatStyle" => EdnaBase::CombatStyle, "CritDamageRating" =>
    EdnaBase::CritDamageRating, "CritHitRating" => EdnaBase::CritHitRating, "disguise" =>
    EdnaBase::Disguise, "DisplayName_Color" => EdnaBase::DisplayNameColor,
    "DisplayName_Number" => EdnaBase::DisplayNameNumber, "DisplayName_Rarity" =>
    EdnaBase::DisplayNameRarity, "DisplayName_Slot" => EdnaBase::DisplayNameSlot,
    "DisplayName_Stat" => EdnaBase::DisplayNameStat, "DodgeRating" =>
    EdnaBase::DodgeRating, "durability" => EdnaBase::Durability, "durabilityCurrent" =>
    EdnaBase::DurabilityCurrent, "Focus" => EdnaBase::Focus, "gender" =>
    EdnaBase::Gender, "HeavyRating" => EdnaBase::HeavyRating, "HitRating" =>
    EdnaBase::HitRating, "isBanked" => EdnaBase::IsBanked, "isExpired" =>
    EdnaBase::IsExpired, "isSKU" => EdnaBase::IsSku, "IsTemplate" =>
    EdnaBase::IsTemplate, "itemActionGenericParam" => EdnaBase::ItemActionGenericParam,
    "levelDropVariance" => EdnaBase::LevelDropVariance, "MaxUse" => EdnaBase::MaxUse,
    "NoteCaption" => EdnaBase::NoteCaption, "NoteCaptionValue" =>
    EdnaBase::NoteCaptionValue, "otherClientInterests" => EdnaBase::OtherClientInterests,
    "ParryRating" => EdnaBase::ParryRating, "PeneRating" => EdnaBase::PeneRating,
    "Prefix" => EdnaBase::Prefix, "QuickbarItem" => EdnaBase::QuickbarItem, "repairCost"
    => EdnaBase::RepairCost, "schematic_CostToCreateItem" =>
    EdnaBase::SchematicCostToCreateItem, "searchKeywords" => EdnaBase::SearchKeywords,
    "setBonuses" => EdnaBase::SetBonuses, "SignOfAvatars" => EdnaBase::SignOfAvatars,
    "SKUID" => EdnaBase::Skuid, "socketLockedStatus" => EdnaBase::SocketLockedStatus,
    "socketOccupancyStatus" => EdnaBase::SocketOccupancyStatus, "socketUpgradeLevel" =>
    EdnaBase::SocketUpgradeLevel, "SpecialRating" => EdnaBase::SpecialRating, "Stamina"
    => EdnaBase::Stamina, "Strength" => EdnaBase::Strength, "Suffix" => EdnaBase::Suffix,
    "templateType" => EdnaBase::TemplateType, "templateVersion" =>
    EdnaBase::TemplateVersion, "timeStamp" => EdnaBase::TimeStamp, "level" =>
    EdnaBase::Level, "name" => EdnaBase::Name,
};
pub(crate) static EDNA_BASE_ATTRIBUTES_ID: phf::Map<u16, EdnaBase> = phf_map! {
    12362u16 => EdnaBase::AdditionalItemCount1, 12361u16 =>
    EdnaBase::AdditionalItemCount2, 12360u16 => EdnaBase::AdditionalItemCount3, 11690u16
    => EdnaBase::AdditionalItemRequired1, 11689u16 => EdnaBase::AdditionalItemRequired2,
    11688u16 => EdnaBase::AdditionalItemRequired3, 7578u16 => EdnaBase::AllowBuy, 7567u16
    => EdnaBase::AllowRent, 7611u16 => EdnaBase::AllowSell, 11687u16 =>
    EdnaBase::BlackSomaRequired, 6576u16 => EdnaBase::BlingPrice, 6575u16 =>
    EdnaBase::BlingSellingPrice, 11686u16 => EdnaBase::BlueSomaRequired, 11954u16 =>
    EdnaBase::BonusSlotAmber, 11955u16 => EdnaBase::BonusSlotRuby, 11956u16 =>
    EdnaBase::BonusSlotSapphire, 7612u16 => EdnaBase::BuyDiscount, 7614u16 =>
    EdnaBase::BuyPriceBling, 7613u16 => EdnaBase::BuyPriceGameCash, 7545u16 =>
    EdnaBase::Category, 8894u16 => EdnaBase::Combos, 666u16 => EdnaBase::ContainerId,
    695u16 => EdnaBase::ContentClass, 12194u16 => EdnaBase::CraftingMapping, 11678u16 =>
    EdnaBase::CraftTime, 665u16 => EdnaBase::CreationTime, 11987u16 =>
    EdnaBase::CrystaEffects, 11989u16 => EdnaBase::CrystalType, 11685u16 =>
    EdnaBase::CyanSomaRequired, 6953u16 => EdnaBase::Description, 6485u16 =>
    EdnaBase::DestroyMethod, 8921u16 => EdnaBase::Dialogs, 697u16 =>
    EdnaBase::DisplayName, 6814u16 => EdnaBase::EnableInGame, 693u16 =>
    EdnaBase::EquipSlot, 11610u16 => EdnaBase::ExpireBuyBack, 7556u16 =>
    EdnaBase::ExpireTime, 682u16 => EdnaBase::Freq, 6574u16 => EdnaBase::GameCashPrice,
    11684u16 => EdnaBase::GreenSomaRequired, 4345u16 => EdnaBase::Icon, 11464u16 =>
    EdnaBase::InfiniteUse, 12333u16 => EdnaBase::InitLeftTime, 9872u16 =>
    EdnaBase::InventorySlotIndex, 12168u16 => EdnaBase::IsCollectFaction, 691u16 =>
    EdnaBase::IsEquiped, 12152u16 => EdnaBase::IsFactionItem, 11988u16 =>
    EdnaBase::IsGemeCrystal, 7357u16 => EdnaBase::IsHotSeller, 7145u16 =>
    EdnaBase::IsInGlobalShop, 7356u16 => EdnaBase::IsInStock, 7358u16 =>
    EdnaBase::IsNewToShop, 9909u16 => EdnaBase::IsQuestItem, 11679u16 =>
    EdnaBase::IsRecipe, 12401u16 => EdnaBase::IsSomaSeed, 10584u16 =>
    EdnaBase::IsSoulBounded, 9375u16 => EdnaBase::IsTechApproved, 7747u16 =>
    EdnaBase::IsTrialItem, 11691u16 => EdnaBase::ItemCritVar, 11692u16 =>
    EdnaBase::ItemNormalVar, 9013u16 => EdnaBase::LastUseTime, 12334u16 =>
    EdnaBase::LeftTime, 5993u16 => EdnaBase::LootAction, 10154u16 => EdnaBase::Lua,
    6173u16 => EdnaBase::Lvl, 687u16 => EdnaBase::LvlReq, 4724u16 =>
    EdnaBase::MaterialOverride, 9889u16 => EdnaBase::MaxStackSize, 11683u16 =>
    EdnaBase::OrangeSomaRequired, 683u16 => EdnaBase::Power, 6433u16 =>
    EdnaBase::Quantity, 7716u16 => EdnaBase::QuestTrigger, 6278u16 => EdnaBase::Rarity,
    11682u16 => EdnaBase::RedSomaRequired, 7454u16 => EdnaBase::RentalDurationMax,
    7455u16 => EdnaBase::RentalDurationMin, 7615u16 => EdnaBase::RentDiscount, 7617u16 =>
    EdnaBase::RentPriceBling, 7616u16 => EdnaBase::RentPriceGameCash, 7610u16 =>
    EdnaBase::SellPriceBling, 684u16 => EdnaBase::SlotId, 6247u16 =>
    EdnaBase::SlotMapping, 12400u16 => EdnaBase::SomaType, 12260u16 =>
    EdnaBase::SoulBoundedAccountId, 10613u16 => EdnaBase::SoulBoundedAvatarId, 12249u16
    => EdnaBase::SoulBoundedToAccount, 10583u16 => EdnaBase::SoulBoundType, 9888u16 =>
    EdnaBase::StackCount, 12167u16 => EdnaBase::StandingReq, 6018u16 =>
    EdnaBase::UseAction, 8994u16 => EdnaBase::UseCoolDownTimer, 9024u16 =>
    EdnaBase::UseCount, 8995u16 => EdnaBase::UseMaxCount, 8956u16 =>
    EdnaBase::UseRequireAvatar, 8955u16 => EdnaBase::UseRequireAvatarWithinRadius,
    8957u16 => EdnaBase::UseRequireTarget, 8954u16 => EdnaBase::UseScript, 6484u16 =>
    EdnaBase::Vendorable, 5934u16 => EdnaBase::VendorAction, 11681u16 =>
    EdnaBase::VioletSomaRequired, 11680u16 => EdnaBase::YellowSomaRequired, 677u16 =>
    EdnaBase::Abilities, 670u16 => EdnaBase::AbilityInstanceData, 11530u16 =>
    EdnaBase::Agility, 11517u16 => EdnaBase::Armor, 11521u16 =>
    EdnaBase::AttackPowerRating, 6395u16 => EdnaBase::AttributeOp1, 6394u16 =>
    EdnaBase::AttributeOp2, 6393u16 => EdnaBase::AttributeOp3, 6392u16 =>
    EdnaBase::AttributeOp4, 6399u16 => EdnaBase::AttributeType1, 6398u16 =>
    EdnaBase::AttributeType2, 6397u16 => EdnaBase::AttributeType3, 6396u16 =>
    EdnaBase::AttributeType4, 6391u16 => EdnaBase::AttributeWeight1, 6390u16 =>
    EdnaBase::AttributeWeight2, 6389u16 => EdnaBase::AttributeWeight3, 6388u16 =>
    EdnaBase::AttributeWeight4, 9470u16 => EdnaBase::AutoAttributeType1, 9469u16 =>
    EdnaBase::AutoAttributeType2, 9468u16 => EdnaBase::AutoAttributeType3, 9467u16 =>
    EdnaBase::AutoAttributeType4, 9544u16 => EdnaBase::AutoAttributeType5, 9543u16 =>
    EdnaBase::AutoAttributeType6, 9464u16 => EdnaBase::AutoAttributeValue1, 9463u16 =>
    EdnaBase::AutoAttributeValue2, 9462u16 => EdnaBase::AutoAttributeValue3, 9461u16 =>
    EdnaBase::AutoAttributeValue4, 9542u16 => EdnaBase::AutoAttributeValue5, 9541u16 =>
    EdnaBase::AutoAttributeValue6, 10081u16 => EdnaBase::AvailableSockets, 11518u16 =>
    EdnaBase::BlockRating, 12035u16 => EdnaBase::ClanName, 4247u16 =>
    EdnaBase::CombatStyle, 11522u16 => EdnaBase::CritDamageRating, 11526u16 =>
    EdnaBase::CritHitRating, 9989u16 => EdnaBase::Disguise, 12224u16 =>
    EdnaBase::DisplayNameColor, 12223u16 => EdnaBase::DisplayNameNumber, 12225u16 =>
    EdnaBase::DisplayNameRarity, 12227u16 => EdnaBase::DisplayNameSlot, 12226u16 =>
    EdnaBase::DisplayNameStat, 11520u16 => EdnaBase::DodgeRating, 10019u16 =>
    EdnaBase::Durability, 10094u16 => EdnaBase::DurabilityCurrent, 11529u16 =>
    EdnaBase::Focus, 11017u16 => EdnaBase::Gender, 11524u16 => EdnaBase::HeavyRating,
    11527u16 => EdnaBase::HitRating, 692u16 => EdnaBase::IsBanked, 668u16 =>
    EdnaBase::IsExpired, 10191u16 => EdnaBase::IsSku, 9704u16 => EdnaBase::IsTemplate,
    10388u16 => EdnaBase::ItemActionGenericParam, 10898u16 =>
    EdnaBase::LevelDropVariance, 4812u16 => EdnaBase::MaxUse, 12229u16 =>
    EdnaBase::NoteCaption, 12228u16 => EdnaBase::NoteCaptionValue, 671u16 =>
    EdnaBase::OtherClientInterests, 11519u16 => EdnaBase::ParryRating, 11525u16 =>
    EdnaBase::PeneRating, 9703u16 => EdnaBase::Prefix, 12273u16 =>
    EdnaBase::QuickbarItem, 10099u16 => EdnaBase::RepairCost, 10104u16 =>
    EdnaBase::SchematicCostToCreateItem, 10070u16 => EdnaBase::SearchKeywords, 12384u16
    => EdnaBase::SetBonuses, 12036u16 => EdnaBase::SignOfAvatars, 10607u16 =>
    EdnaBase::Skuid, 10084u16 => EdnaBase::SocketLockedStatus, 10082u16 =>
    EdnaBase::SocketOccupancyStatus, 10083u16 => EdnaBase::SocketUpgradeLevel, 11523u16
    => EdnaBase::SpecialRating, 11528u16 => EdnaBase::Stamina, 11531u16 =>
    EdnaBase::Strength, 9702u16 => EdnaBase::Suffix, 10074u16 => EdnaBase::TemplateType,
    11312u16 => EdnaBase::TemplateVersion, 667u16 => EdnaBase::TimeStamp, 698u16 =>
    EdnaBase::Level, 699u16 => EdnaBase::Name,
};
impl Attribute for EdnaBase {
    fn class() -> Class {
        Class::EdnaBase
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
            Self::Level => &Self::Level,
            Self::Name => &Self::Name,
        }
    }
}
impl AttributeInfo for EdnaBase {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12362u16,
            Self::AdditionalItemCount2 => 12361u16,
            Self::AdditionalItemCount3 => 12360u16,
            Self::AdditionalItemRequired1 => 11690u16,
            Self::AdditionalItemRequired2 => 11689u16,
            Self::AdditionalItemRequired3 => 11688u16,
            Self::AllowBuy => 7578u16,
            Self::AllowRent => 7567u16,
            Self::AllowSell => 7611u16,
            Self::BlackSomaRequired => 11687u16,
            Self::BlingPrice => 6576u16,
            Self::BlingSellingPrice => 6575u16,
            Self::BlueSomaRequired => 11686u16,
            Self::BonusSlotAmber => 11954u16,
            Self::BonusSlotRuby => 11955u16,
            Self::BonusSlotSapphire => 11956u16,
            Self::BuyDiscount => 7612u16,
            Self::BuyPriceBling => 7614u16,
            Self::BuyPriceGameCash => 7613u16,
            Self::Category => 7545u16,
            Self::Combos => 8894u16,
            Self::ContainerId => 666u16,
            Self::ContentClass => 695u16,
            Self::CraftingMapping => 12194u16,
            Self::CraftTime => 11678u16,
            Self::CreationTime => 665u16,
            Self::CrystaEffects => 11987u16,
            Self::CrystalType => 11989u16,
            Self::CyanSomaRequired => 11685u16,
            Self::Description => 6953u16,
            Self::DestroyMethod => 6485u16,
            Self::Dialogs => 8921u16,
            Self::DisplayName => 697u16,
            Self::EnableInGame => 6814u16,
            Self::EquipSlot => 693u16,
            Self::ExpireBuyBack => 11610u16,
            Self::ExpireTime => 7556u16,
            Self::Freq => 682u16,
            Self::GameCashPrice => 6574u16,
            Self::GreenSomaRequired => 11684u16,
            Self::Icon => 4345u16,
            Self::InfiniteUse => 11464u16,
            Self::InitLeftTime => 12333u16,
            Self::InventorySlotIndex => 9872u16,
            Self::IsCollectFaction => 12168u16,
            Self::IsEquiped => 691u16,
            Self::IsFactionItem => 12152u16,
            Self::IsGemeCrystal => 11988u16,
            Self::IsHotSeller => 7357u16,
            Self::IsInGlobalShop => 7145u16,
            Self::IsInStock => 7356u16,
            Self::IsNewToShop => 7358u16,
            Self::IsQuestItem => 9909u16,
            Self::IsRecipe => 11679u16,
            Self::IsSomaSeed => 12401u16,
            Self::IsSoulBounded => 10584u16,
            Self::IsTechApproved => 9375u16,
            Self::IsTrialItem => 7747u16,
            Self::ItemCritVar => 11691u16,
            Self::ItemNormalVar => 11692u16,
            Self::LastUseTime => 9013u16,
            Self::LeftTime => 12334u16,
            Self::LootAction => 5993u16,
            Self::Lua => 10154u16,
            Self::Lvl => 6173u16,
            Self::LvlReq => 687u16,
            Self::MaterialOverride => 4724u16,
            Self::MaxStackSize => 9889u16,
            Self::OrangeSomaRequired => 11683u16,
            Self::Power => 683u16,
            Self::Quantity => 6433u16,
            Self::QuestTrigger => 7716u16,
            Self::Rarity => 6278u16,
            Self::RedSomaRequired => 11682u16,
            Self::RentalDurationMax => 7454u16,
            Self::RentalDurationMin => 7455u16,
            Self::RentDiscount => 7615u16,
            Self::RentPriceBling => 7617u16,
            Self::RentPriceGameCash => 7616u16,
            Self::SellPriceBling => 7610u16,
            Self::SlotId => 684u16,
            Self::SlotMapping => 6247u16,
            Self::SomaType => 12400u16,
            Self::SoulBoundedAccountId => 12260u16,
            Self::SoulBoundedAvatarId => 10613u16,
            Self::SoulBoundedToAccount => 12249u16,
            Self::SoulBoundType => 10583u16,
            Self::StackCount => 9888u16,
            Self::StandingReq => 12167u16,
            Self::UseAction => 6018u16,
            Self::UseCoolDownTimer => 8994u16,
            Self::UseCount => 9024u16,
            Self::UseMaxCount => 8995u16,
            Self::UseRequireAvatar => 8956u16,
            Self::UseRequireAvatarWithinRadius => 8955u16,
            Self::UseRequireTarget => 8957u16,
            Self::UseScript => 8954u16,
            Self::Vendorable => 6484u16,
            Self::VendorAction => 5934u16,
            Self::VioletSomaRequired => 11681u16,
            Self::YellowSomaRequired => 11680u16,
            Self::Abilities => 677u16,
            Self::AbilityInstanceData => 670u16,
            Self::Agility => 11530u16,
            Self::Armor => 11517u16,
            Self::AttackPowerRating => 11521u16,
            Self::AttributeOp1 => 6395u16,
            Self::AttributeOp2 => 6394u16,
            Self::AttributeOp3 => 6393u16,
            Self::AttributeOp4 => 6392u16,
            Self::AttributeType1 => 6399u16,
            Self::AttributeType2 => 6398u16,
            Self::AttributeType3 => 6397u16,
            Self::AttributeType4 => 6396u16,
            Self::AttributeWeight1 => 6391u16,
            Self::AttributeWeight2 => 6390u16,
            Self::AttributeWeight3 => 6389u16,
            Self::AttributeWeight4 => 6388u16,
            Self::AutoAttributeType1 => 9470u16,
            Self::AutoAttributeType2 => 9469u16,
            Self::AutoAttributeType3 => 9468u16,
            Self::AutoAttributeType4 => 9467u16,
            Self::AutoAttributeType5 => 9544u16,
            Self::AutoAttributeType6 => 9543u16,
            Self::AutoAttributeValue1 => 9464u16,
            Self::AutoAttributeValue2 => 9463u16,
            Self::AutoAttributeValue3 => 9462u16,
            Self::AutoAttributeValue4 => 9461u16,
            Self::AutoAttributeValue5 => 9542u16,
            Self::AutoAttributeValue6 => 9541u16,
            Self::AvailableSockets => 10081u16,
            Self::BlockRating => 11518u16,
            Self::ClanName => 12035u16,
            Self::CombatStyle => 4247u16,
            Self::CritDamageRating => 11522u16,
            Self::CritHitRating => 11526u16,
            Self::Disguise => 9989u16,
            Self::DisplayNameColor => 12224u16,
            Self::DisplayNameNumber => 12223u16,
            Self::DisplayNameRarity => 12225u16,
            Self::DisplayNameSlot => 12227u16,
            Self::DisplayNameStat => 12226u16,
            Self::DodgeRating => 11520u16,
            Self::Durability => 10019u16,
            Self::DurabilityCurrent => 10094u16,
            Self::Focus => 11529u16,
            Self::Gender => 11017u16,
            Self::HeavyRating => 11524u16,
            Self::HitRating => 11527u16,
            Self::IsBanked => 692u16,
            Self::IsExpired => 668u16,
            Self::IsSku => 10191u16,
            Self::IsTemplate => 9704u16,
            Self::ItemActionGenericParam => 10388u16,
            Self::LevelDropVariance => 10898u16,
            Self::MaxUse => 4812u16,
            Self::NoteCaption => 12229u16,
            Self::NoteCaptionValue => 12228u16,
            Self::OtherClientInterests => 671u16,
            Self::ParryRating => 11519u16,
            Self::PeneRating => 11525u16,
            Self::Prefix => 9703u16,
            Self::QuickbarItem => 12273u16,
            Self::RepairCost => 10099u16,
            Self::SchematicCostToCreateItem => 10104u16,
            Self::SearchKeywords => 10070u16,
            Self::SetBonuses => 12384u16,
            Self::SignOfAvatars => 12036u16,
            Self::Skuid => 10607u16,
            Self::SocketLockedStatus => 10084u16,
            Self::SocketOccupancyStatus => 10082u16,
            Self::SocketUpgradeLevel => 10083u16,
            Self::SpecialRating => 11523u16,
            Self::Stamina => 11528u16,
            Self::Strength => 11531u16,
            Self::Suffix => 9702u16,
            Self::TemplateType => 10074u16,
            Self::TemplateVersion => 11312u16,
            Self::TimeStamp => 667u16,
            Self::Level => 698u16,
            Self::Name => 699u16,
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
            Self::Level => "level",
            Self::Name => "name",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Level => ParamType::Int,
            Self::Name => ParamType::String,
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
        static LEVEL: Value = Value::Int(3i32);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
            Self::Level => &LEVEL,
            Self::Name => &NAME,
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
            Self::Level => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
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
impl FromStr for EdnaBase {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EDNA_BASE_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for EdnaBase {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12362u16 => Ok(Self::AdditionalItemCount1),
            12361u16 => Ok(Self::AdditionalItemCount2),
            12360u16 => Ok(Self::AdditionalItemCount3),
            11690u16 => Ok(Self::AdditionalItemRequired1),
            11689u16 => Ok(Self::AdditionalItemRequired2),
            11688u16 => Ok(Self::AdditionalItemRequired3),
            7578u16 => Ok(Self::AllowBuy),
            7567u16 => Ok(Self::AllowRent),
            7611u16 => Ok(Self::AllowSell),
            11687u16 => Ok(Self::BlackSomaRequired),
            6576u16 => Ok(Self::BlingPrice),
            6575u16 => Ok(Self::BlingSellingPrice),
            11686u16 => Ok(Self::BlueSomaRequired),
            11954u16 => Ok(Self::BonusSlotAmber),
            11955u16 => Ok(Self::BonusSlotRuby),
            11956u16 => Ok(Self::BonusSlotSapphire),
            7612u16 => Ok(Self::BuyDiscount),
            7614u16 => Ok(Self::BuyPriceBling),
            7613u16 => Ok(Self::BuyPriceGameCash),
            7545u16 => Ok(Self::Category),
            8894u16 => Ok(Self::Combos),
            666u16 => Ok(Self::ContainerId),
            695u16 => Ok(Self::ContentClass),
            12194u16 => Ok(Self::CraftingMapping),
            11678u16 => Ok(Self::CraftTime),
            665u16 => Ok(Self::CreationTime),
            11987u16 => Ok(Self::CrystaEffects),
            11989u16 => Ok(Self::CrystalType),
            11685u16 => Ok(Self::CyanSomaRequired),
            6953u16 => Ok(Self::Description),
            6485u16 => Ok(Self::DestroyMethod),
            8921u16 => Ok(Self::Dialogs),
            697u16 => Ok(Self::DisplayName),
            6814u16 => Ok(Self::EnableInGame),
            693u16 => Ok(Self::EquipSlot),
            11610u16 => Ok(Self::ExpireBuyBack),
            7556u16 => Ok(Self::ExpireTime),
            682u16 => Ok(Self::Freq),
            6574u16 => Ok(Self::GameCashPrice),
            11684u16 => Ok(Self::GreenSomaRequired),
            4345u16 => Ok(Self::Icon),
            11464u16 => Ok(Self::InfiniteUse),
            12333u16 => Ok(Self::InitLeftTime),
            9872u16 => Ok(Self::InventorySlotIndex),
            12168u16 => Ok(Self::IsCollectFaction),
            691u16 => Ok(Self::IsEquiped),
            12152u16 => Ok(Self::IsFactionItem),
            11988u16 => Ok(Self::IsGemeCrystal),
            7357u16 => Ok(Self::IsHotSeller),
            7145u16 => Ok(Self::IsInGlobalShop),
            7356u16 => Ok(Self::IsInStock),
            7358u16 => Ok(Self::IsNewToShop),
            9909u16 => Ok(Self::IsQuestItem),
            11679u16 => Ok(Self::IsRecipe),
            12401u16 => Ok(Self::IsSomaSeed),
            10584u16 => Ok(Self::IsSoulBounded),
            9375u16 => Ok(Self::IsTechApproved),
            7747u16 => Ok(Self::IsTrialItem),
            11691u16 => Ok(Self::ItemCritVar),
            11692u16 => Ok(Self::ItemNormalVar),
            9013u16 => Ok(Self::LastUseTime),
            12334u16 => Ok(Self::LeftTime),
            5993u16 => Ok(Self::LootAction),
            10154u16 => Ok(Self::Lua),
            6173u16 => Ok(Self::Lvl),
            687u16 => Ok(Self::LvlReq),
            4724u16 => Ok(Self::MaterialOverride),
            9889u16 => Ok(Self::MaxStackSize),
            11683u16 => Ok(Self::OrangeSomaRequired),
            683u16 => Ok(Self::Power),
            6433u16 => Ok(Self::Quantity),
            7716u16 => Ok(Self::QuestTrigger),
            6278u16 => Ok(Self::Rarity),
            11682u16 => Ok(Self::RedSomaRequired),
            7454u16 => Ok(Self::RentalDurationMax),
            7455u16 => Ok(Self::RentalDurationMin),
            7615u16 => Ok(Self::RentDiscount),
            7617u16 => Ok(Self::RentPriceBling),
            7616u16 => Ok(Self::RentPriceGameCash),
            7610u16 => Ok(Self::SellPriceBling),
            684u16 => Ok(Self::SlotId),
            6247u16 => Ok(Self::SlotMapping),
            12400u16 => Ok(Self::SomaType),
            12260u16 => Ok(Self::SoulBoundedAccountId),
            10613u16 => Ok(Self::SoulBoundedAvatarId),
            12249u16 => Ok(Self::SoulBoundedToAccount),
            10583u16 => Ok(Self::SoulBoundType),
            9888u16 => Ok(Self::StackCount),
            12167u16 => Ok(Self::StandingReq),
            6018u16 => Ok(Self::UseAction),
            8994u16 => Ok(Self::UseCoolDownTimer),
            9024u16 => Ok(Self::UseCount),
            8995u16 => Ok(Self::UseMaxCount),
            8956u16 => Ok(Self::UseRequireAvatar),
            8955u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8957u16 => Ok(Self::UseRequireTarget),
            8954u16 => Ok(Self::UseScript),
            6484u16 => Ok(Self::Vendorable),
            5934u16 => Ok(Self::VendorAction),
            11681u16 => Ok(Self::VioletSomaRequired),
            11680u16 => Ok(Self::YellowSomaRequired),
            677u16 => Ok(Self::Abilities),
            670u16 => Ok(Self::AbilityInstanceData),
            11530u16 => Ok(Self::Agility),
            11517u16 => Ok(Self::Armor),
            11521u16 => Ok(Self::AttackPowerRating),
            6395u16 => Ok(Self::AttributeOp1),
            6394u16 => Ok(Self::AttributeOp2),
            6393u16 => Ok(Self::AttributeOp3),
            6392u16 => Ok(Self::AttributeOp4),
            6399u16 => Ok(Self::AttributeType1),
            6398u16 => Ok(Self::AttributeType2),
            6397u16 => Ok(Self::AttributeType3),
            6396u16 => Ok(Self::AttributeType4),
            6391u16 => Ok(Self::AttributeWeight1),
            6390u16 => Ok(Self::AttributeWeight2),
            6389u16 => Ok(Self::AttributeWeight3),
            6388u16 => Ok(Self::AttributeWeight4),
            9470u16 => Ok(Self::AutoAttributeType1),
            9469u16 => Ok(Self::AutoAttributeType2),
            9468u16 => Ok(Self::AutoAttributeType3),
            9467u16 => Ok(Self::AutoAttributeType4),
            9544u16 => Ok(Self::AutoAttributeType5),
            9543u16 => Ok(Self::AutoAttributeType6),
            9464u16 => Ok(Self::AutoAttributeValue1),
            9463u16 => Ok(Self::AutoAttributeValue2),
            9462u16 => Ok(Self::AutoAttributeValue3),
            9461u16 => Ok(Self::AutoAttributeValue4),
            9542u16 => Ok(Self::AutoAttributeValue5),
            9541u16 => Ok(Self::AutoAttributeValue6),
            10081u16 => Ok(Self::AvailableSockets),
            11518u16 => Ok(Self::BlockRating),
            12035u16 => Ok(Self::ClanName),
            4247u16 => Ok(Self::CombatStyle),
            11522u16 => Ok(Self::CritDamageRating),
            11526u16 => Ok(Self::CritHitRating),
            9989u16 => Ok(Self::Disguise),
            12224u16 => Ok(Self::DisplayNameColor),
            12223u16 => Ok(Self::DisplayNameNumber),
            12225u16 => Ok(Self::DisplayNameRarity),
            12227u16 => Ok(Self::DisplayNameSlot),
            12226u16 => Ok(Self::DisplayNameStat),
            11520u16 => Ok(Self::DodgeRating),
            10019u16 => Ok(Self::Durability),
            10094u16 => Ok(Self::DurabilityCurrent),
            11529u16 => Ok(Self::Focus),
            11017u16 => Ok(Self::Gender),
            11524u16 => Ok(Self::HeavyRating),
            11527u16 => Ok(Self::HitRating),
            692u16 => Ok(Self::IsBanked),
            668u16 => Ok(Self::IsExpired),
            10191u16 => Ok(Self::IsSku),
            9704u16 => Ok(Self::IsTemplate),
            10388u16 => Ok(Self::ItemActionGenericParam),
            10898u16 => Ok(Self::LevelDropVariance),
            4812u16 => Ok(Self::MaxUse),
            12229u16 => Ok(Self::NoteCaption),
            12228u16 => Ok(Self::NoteCaptionValue),
            671u16 => Ok(Self::OtherClientInterests),
            11519u16 => Ok(Self::ParryRating),
            11525u16 => Ok(Self::PeneRating),
            9703u16 => Ok(Self::Prefix),
            12273u16 => Ok(Self::QuickbarItem),
            10099u16 => Ok(Self::RepairCost),
            10104u16 => Ok(Self::SchematicCostToCreateItem),
            10070u16 => Ok(Self::SearchKeywords),
            12384u16 => Ok(Self::SetBonuses),
            12036u16 => Ok(Self::SignOfAvatars),
            10607u16 => Ok(Self::Skuid),
            10084u16 => Ok(Self::SocketLockedStatus),
            10082u16 => Ok(Self::SocketOccupancyStatus),
            10083u16 => Ok(Self::SocketUpgradeLevel),
            11523u16 => Ok(Self::SpecialRating),
            11528u16 => Ok(Self::Stamina),
            11531u16 => Ok(Self::Strength),
            9702u16 => Ok(Self::Suffix),
            10074u16 => Ok(Self::TemplateType),
            11312u16 => Ok(Self::TemplateVersion),
            667u16 => Ok(Self::TimeStamp),
            698u16 => Ok(Self::Level),
            699u16 => Ok(Self::Name),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
