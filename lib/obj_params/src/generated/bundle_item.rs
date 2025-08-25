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
pub enum BundleItem {
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
    UseParamI,
}
pub(crate) static BUNDLE_ITEM_ATTRIBUTES: phf::Map<&'static str, BundleItem> = phf_map! {
    "AdditionalItemCount1" => BundleItem::AdditionalItemCount1, "AdditionalItemCount2" =>
    BundleItem::AdditionalItemCount2, "AdditionalItemCount3" =>
    BundleItem::AdditionalItemCount3, "AdditionalItemRequired1" =>
    BundleItem::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    BundleItem::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    BundleItem::AdditionalItemRequired3, "AllowBuy" => BundleItem::AllowBuy, "AllowRent"
    => BundleItem::AllowRent, "AllowSell" => BundleItem::AllowSell, "BlackSomaRequired"
    => BundleItem::BlackSomaRequired, "blingPrice" => BundleItem::BlingPrice,
    "blingSellingPrice" => BundleItem::BlingSellingPrice, "BlueSomaRequired" =>
    BundleItem::BlueSomaRequired, "BonusSlotAmber" => BundleItem::BonusSlotAmber,
    "BonusSlotRuby" => BundleItem::BonusSlotRuby, "BonusSlotSapphire" =>
    BundleItem::BonusSlotSapphire, "BuyDiscount" => BundleItem::BuyDiscount,
    "BuyPriceBling" => BundleItem::BuyPriceBling, "BuyPriceGameCash" =>
    BundleItem::BuyPriceGameCash, "Category" => BundleItem::Category, "combos" =>
    BundleItem::Combos, "containerID" => BundleItem::ContainerId, "ContentClass" =>
    BundleItem::ContentClass, "CraftingMapping" => BundleItem::CraftingMapping,
    "CraftTime" => BundleItem::CraftTime, "creationTime" => BundleItem::CreationTime,
    "CrystaEffects" => BundleItem::CrystaEffects, "CrystalType" =>
    BundleItem::CrystalType, "CyanSomaRequired" => BundleItem::CyanSomaRequired,
    "Description" => BundleItem::Description, "DestroyMethod" =>
    BundleItem::DestroyMethod, "Dialogs" => BundleItem::Dialogs, "DisplayName" =>
    BundleItem::DisplayName, "EnableInGame" => BundleItem::EnableInGame, "equipSlot" =>
    BundleItem::EquipSlot, "expireBuyBack" => BundleItem::ExpireBuyBack, "ExpireTime" =>
    BundleItem::ExpireTime, "Freq" => BundleItem::Freq, "gameCashPrice" =>
    BundleItem::GameCashPrice, "GreenSomaRequired" => BundleItem::GreenSomaRequired,
    "Icon" => BundleItem::Icon, "InfiniteUse" => BundleItem::InfiniteUse, "InitLeftTime"
    => BundleItem::InitLeftTime, "inventorySlotIndex" => BundleItem::InventorySlotIndex,
    "isCollectFaction" => BundleItem::IsCollectFaction, "isEquiped" =>
    BundleItem::IsEquiped, "isFactionItem" => BundleItem::IsFactionItem, "isGemeCrystal"
    => BundleItem::IsGemeCrystal, "IsHotSeller" => BundleItem::IsHotSeller,
    "isInGlobalShop" => BundleItem::IsInGlobalShop, "IsInStock" => BundleItem::IsInStock,
    "IsNewToShop" => BundleItem::IsNewToShop, "isQuestItem" => BundleItem::IsQuestItem,
    "IsRecipe" => BundleItem::IsRecipe, "IsSomaSeed" => BundleItem::IsSomaSeed,
    "IsSoulBounded" => BundleItem::IsSoulBounded, "isTechApproved" =>
    BundleItem::IsTechApproved, "isTrialItem" => BundleItem::IsTrialItem, "ItemCritVar"
    => BundleItem::ItemCritVar, "ItemNormalVar" => BundleItem::ItemNormalVar,
    "LastUseTime" => BundleItem::LastUseTime, "LeftTime" => BundleItem::LeftTime,
    "lootAction" => BundleItem::LootAction, "Lua" => BundleItem::Lua, "lvl" =>
    BundleItem::Lvl, "lvlReq" => BundleItem::LvlReq, "MaterialOverride" =>
    BundleItem::MaterialOverride, "maxStackSize" => BundleItem::MaxStackSize,
    "OrangeSomaRequired" => BundleItem::OrangeSomaRequired, "Power" => BundleItem::Power,
    "quantity" => BundleItem::Quantity, "QuestTrigger" => BundleItem::QuestTrigger,
    "rarity" => BundleItem::Rarity, "RedSomaRequired" => BundleItem::RedSomaRequired,
    "RentalDurationMax" => BundleItem::RentalDurationMax, "RentalDurationMin" =>
    BundleItem::RentalDurationMin, "RentDiscount" => BundleItem::RentDiscount,
    "RentPriceBling" => BundleItem::RentPriceBling, "RentPriceGameCash" =>
    BundleItem::RentPriceGameCash, "SellPriceBling" => BundleItem::SellPriceBling,
    "slotID" => BundleItem::SlotId, "SlotMapping" => BundleItem::SlotMapping, "SomaType"
    => BundleItem::SomaType, "SoulBoundedAccountId" => BundleItem::SoulBoundedAccountId,
    "SoulBoundedAvatarId" => BundleItem::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    BundleItem::SoulBoundedToAccount, "SoulBoundType" => BundleItem::SoulBoundType,
    "stackCount" => BundleItem::StackCount, "standingReq" => BundleItem::StandingReq,
    "useAction" => BundleItem::UseAction, "UseCoolDownTimer" =>
    BundleItem::UseCoolDownTimer, "UseCount" => BundleItem::UseCount, "UseMaxCount" =>
    BundleItem::UseMaxCount, "UseRequireAvatar" => BundleItem::UseRequireAvatar,
    "UseRequireAvatarWithinRadius" => BundleItem::UseRequireAvatarWithinRadius,
    "UseRequireTarget" => BundleItem::UseRequireTarget, "UseScript" =>
    BundleItem::UseScript, "Vendorable" => BundleItem::Vendorable, "vendorAction" =>
    BundleItem::VendorAction, "VioletSomaRequired" => BundleItem::VioletSomaRequired,
    "YellowSomaRequired" => BundleItem::YellowSomaRequired, "abilities" =>
    BundleItem::Abilities, "abilityInstanceData" => BundleItem::AbilityInstanceData,
    "Agility" => BundleItem::Agility, "Armor" => BundleItem::Armor, "AttackPowerRating"
    => BundleItem::AttackPowerRating, "attributeOp1" => BundleItem::AttributeOp1,
    "attributeOp2" => BundleItem::AttributeOp2, "attributeOp3" =>
    BundleItem::AttributeOp3, "attributeOp4" => BundleItem::AttributeOp4,
    "attributeType1" => BundleItem::AttributeType1, "attributeType2" =>
    BundleItem::AttributeType2, "attributeType3" => BundleItem::AttributeType3,
    "attributeType4" => BundleItem::AttributeType4, "attributeWeight1" =>
    BundleItem::AttributeWeight1, "attributeWeight2" => BundleItem::AttributeWeight2,
    "attributeWeight3" => BundleItem::AttributeWeight3, "attributeWeight4" =>
    BundleItem::AttributeWeight4, "autoAttributeType1" => BundleItem::AutoAttributeType1,
    "autoAttributeType2" => BundleItem::AutoAttributeType2, "autoAttributeType3" =>
    BundleItem::AutoAttributeType3, "autoAttributeType4" =>
    BundleItem::AutoAttributeType4, "autoAttributeType5" =>
    BundleItem::AutoAttributeType5, "autoAttributeType6" =>
    BundleItem::AutoAttributeType6, "autoAttributeValue1" =>
    BundleItem::AutoAttributeValue1, "autoAttributeValue2" =>
    BundleItem::AutoAttributeValue2, "autoAttributeValue3" =>
    BundleItem::AutoAttributeValue3, "autoAttributeValue4" =>
    BundleItem::AutoAttributeValue4, "autoAttributeValue5" =>
    BundleItem::AutoAttributeValue5, "autoAttributeValue6" =>
    BundleItem::AutoAttributeValue6, "availableSockets" => BundleItem::AvailableSockets,
    "BlockRating" => BundleItem::BlockRating, "ClanName" => BundleItem::ClanName,
    "combatStyle" => BundleItem::CombatStyle, "CritDamageRating" =>
    BundleItem::CritDamageRating, "CritHitRating" => BundleItem::CritHitRating,
    "disguise" => BundleItem::Disguise, "DisplayName_Color" =>
    BundleItem::DisplayNameColor, "DisplayName_Number" => BundleItem::DisplayNameNumber,
    "DisplayName_Rarity" => BundleItem::DisplayNameRarity, "DisplayName_Slot" =>
    BundleItem::DisplayNameSlot, "DisplayName_Stat" => BundleItem::DisplayNameStat,
    "DodgeRating" => BundleItem::DodgeRating, "durability" => BundleItem::Durability,
    "durabilityCurrent" => BundleItem::DurabilityCurrent, "Focus" => BundleItem::Focus,
    "gender" => BundleItem::Gender, "HeavyRating" => BundleItem::HeavyRating, "HitRating"
    => BundleItem::HitRating, "isBanked" => BundleItem::IsBanked, "isExpired" =>
    BundleItem::IsExpired, "isSKU" => BundleItem::IsSku, "IsTemplate" =>
    BundleItem::IsTemplate, "itemActionGenericParam" =>
    BundleItem::ItemActionGenericParam, "levelDropVariance" =>
    BundleItem::LevelDropVariance, "MaxUse" => BundleItem::MaxUse, "NoteCaption" =>
    BundleItem::NoteCaption, "NoteCaptionValue" => BundleItem::NoteCaptionValue,
    "otherClientInterests" => BundleItem::OtherClientInterests, "ParryRating" =>
    BundleItem::ParryRating, "PeneRating" => BundleItem::PeneRating, "Prefix" =>
    BundleItem::Prefix, "QuickbarItem" => BundleItem::QuickbarItem, "repairCost" =>
    BundleItem::RepairCost, "schematic_CostToCreateItem" =>
    BundleItem::SchematicCostToCreateItem, "searchKeywords" =>
    BundleItem::SearchKeywords, "setBonuses" => BundleItem::SetBonuses, "SignOfAvatars"
    => BundleItem::SignOfAvatars, "SKUID" => BundleItem::Skuid, "socketLockedStatus" =>
    BundleItem::SocketLockedStatus, "socketOccupancyStatus" =>
    BundleItem::SocketOccupancyStatus, "socketUpgradeLevel" =>
    BundleItem::SocketUpgradeLevel, "SpecialRating" => BundleItem::SpecialRating,
    "Stamina" => BundleItem::Stamina, "Strength" => BundleItem::Strength, "Suffix" =>
    BundleItem::Suffix, "templateType" => BundleItem::TemplateType, "templateVersion" =>
    BundleItem::TemplateVersion, "timeStamp" => BundleItem::TimeStamp, "UseParamI" =>
    BundleItem::UseParamI,
};
pub(crate) static BUNDLE_ITEM_ATTRIBUTES_ID: phf::Map<u16, BundleItem> = phf_map! {
    12356u16 => BundleItem::AdditionalItemCount1, 12355u16 =>
    BundleItem::AdditionalItemCount2, 12354u16 => BundleItem::AdditionalItemCount3,
    11660u16 => BundleItem::AdditionalItemRequired1, 11659u16 =>
    BundleItem::AdditionalItemRequired2, 11658u16 => BundleItem::AdditionalItemRequired3,
    10704u16 => BundleItem::AllowBuy, 10705u16 => BundleItem::AllowRent, 10702u16 =>
    BundleItem::AllowSell, 11657u16 => BundleItem::BlackSomaRequired, 10724u16 =>
    BundleItem::BlingPrice, 10725u16 => BundleItem::BlingSellingPrice, 11656u16 =>
    BundleItem::BlueSomaRequired, 11948u16 => BundleItem::BonusSlotAmber, 11949u16 =>
    BundleItem::BonusSlotRuby, 11950u16 => BundleItem::BonusSlotSapphire, 10701u16 =>
    BundleItem::BuyDiscount, 10699u16 => BundleItem::BuyPriceBling, 10700u16 =>
    BundleItem::BuyPriceGameCash, 10707u16 => BundleItem::Category, 10688u16 =>
    BundleItem::Combos, 10768u16 => BundleItem::ContainerId, 10755u16 =>
    BundleItem::ContentClass, 12192u16 => BundleItem::CraftingMapping, 11648u16 =>
    BundleItem::CraftTime, 10769u16 => BundleItem::CreationTime, 11981u16 =>
    BundleItem::CrystaEffects, 11983u16 => BundleItem::CrystalType, 11655u16 =>
    BundleItem::CyanSomaRequired, 10720u16 => BundleItem::Description, 10727u16 =>
    BundleItem::DestroyMethod, 10687u16 => BundleItem::Dialogs, 10754u16 =>
    BundleItem::DisplayName, 10721u16 => BundleItem::EnableInGame, 10756u16 =>
    BundleItem::EquipSlot, 11608u16 => BundleItem::ExpireBuyBack, 10706u16 =>
    BundleItem::ExpireTime, 10762u16 => BundleItem::Freq, 10726u16 =>
    BundleItem::GameCashPrice, 11654u16 => BundleItem::GreenSomaRequired, 10751u16 =>
    BundleItem::Icon, 11462u16 => BundleItem::InfiniteUse, 12329u16 =>
    BundleItem::InitLeftTime, 10660u16 => BundleItem::InventorySlotIndex, 12164u16 =>
    BundleItem::IsCollectFaction, 10758u16 => BundleItem::IsEquiped, 12150u16 =>
    BundleItem::IsFactionItem, 11982u16 => BundleItem::IsGemeCrystal, 10715u16 =>
    BundleItem::IsHotSeller, 10717u16 => BundleItem::IsInGlobalShop, 10716u16 =>
    BundleItem::IsInStock, 10714u16 => BundleItem::IsNewToShop, 10657u16 =>
    BundleItem::IsQuestItem, 11649u16 => BundleItem::IsRecipe, 12397u16 =>
    BundleItem::IsSomaSeed, 10633u16 => BundleItem::IsSoulBounded, 10678u16 =>
    BundleItem::IsTechApproved, 10692u16 => BundleItem::IsTrialItem, 11661u16 =>
    BundleItem::ItemCritVar, 11662u16 => BundleItem::ItemNormalVar, 10680u16 =>
    BundleItem::LastUseTime, 12330u16 => BundleItem::LeftTime, 10747u16 =>
    BundleItem::LootAction, 10639u16 => BundleItem::Lua, 10745u16 => BundleItem::Lvl,
    10759u16 => BundleItem::LvlReq, 10750u16 => BundleItem::MaterialOverride, 10658u16 =>
    BundleItem::MaxStackSize, 11653u16 => BundleItem::OrangeSomaRequired, 10761u16 =>
    BundleItem::Power, 10730u16 => BundleItem::Quantity, 10694u16 =>
    BundleItem::QuestTrigger, 10743u16 => BundleItem::Rarity, 11652u16 =>
    BundleItem::RedSomaRequired, 10709u16 => BundleItem::RentalDurationMax, 10708u16 =>
    BundleItem::RentalDurationMin, 10698u16 => BundleItem::RentDiscount, 10696u16 =>
    BundleItem::RentPriceBling, 10697u16 => BundleItem::RentPriceGameCash, 10703u16 =>
    BundleItem::SellPriceBling, 10760u16 => BundleItem::SlotId, 10744u16 =>
    BundleItem::SlotMapping, 12396u16 => BundleItem::SomaType, 12258u16 =>
    BundleItem::SoulBoundedAccountId, 10631u16 => BundleItem::SoulBoundedAvatarId,
    12247u16 => BundleItem::SoulBoundedToAccount, 10634u16 => BundleItem::SoulBoundType,
    10659u16 => BundleItem::StackCount, 12163u16 => BundleItem::StandingReq, 10746u16 =>
    BundleItem::UseAction, 10682u16 => BundleItem::UseCoolDownTimer, 10679u16 =>
    BundleItem::UseCount, 10681u16 => BundleItem::UseMaxCount, 10684u16 =>
    BundleItem::UseRequireAvatar, 10685u16 => BundleItem::UseRequireAvatarWithinRadius,
    10683u16 => BundleItem::UseRequireTarget, 10686u16 => BundleItem::UseScript, 10728u16
    => BundleItem::Vendorable, 10748u16 => BundleItem::VendorAction, 11651u16 =>
    BundleItem::VioletSomaRequired, 11650u16 => BundleItem::YellowSomaRequired, 10763u16
    => BundleItem::Abilities, 10765u16 => BundleItem::AbilityInstanceData, 11500u16 =>
    BundleItem::Agility, 11487u16 => BundleItem::Armor, 11491u16 =>
    BundleItem::AttackPowerRating, 10735u16 => BundleItem::AttributeOp1, 10736u16 =>
    BundleItem::AttributeOp2, 10737u16 => BundleItem::AttributeOp3, 10738u16 =>
    BundleItem::AttributeOp4, 10731u16 => BundleItem::AttributeType1, 10732u16 =>
    BundleItem::AttributeType2, 10733u16 => BundleItem::AttributeType3, 10734u16 =>
    BundleItem::AttributeType4, 10739u16 => BundleItem::AttributeWeight1, 10740u16 =>
    BundleItem::AttributeWeight2, 10741u16 => BundleItem::AttributeWeight3, 10742u16 =>
    BundleItem::AttributeWeight4, 10668u16 => BundleItem::AutoAttributeType1, 10669u16 =>
    BundleItem::AutoAttributeType2, 10670u16 => BundleItem::AutoAttributeType3, 10671u16
    => BundleItem::AutoAttributeType4, 10664u16 => BundleItem::AutoAttributeType5,
    10665u16 => BundleItem::AutoAttributeType6, 10672u16 =>
    BundleItem::AutoAttributeValue1, 10673u16 => BundleItem::AutoAttributeValue2,
    10674u16 => BundleItem::AutoAttributeValue3, 10675u16 =>
    BundleItem::AutoAttributeValue4, 10666u16 => BundleItem::AutoAttributeValue5,
    10667u16 => BundleItem::AutoAttributeValue6, 10646u16 =>
    BundleItem::AvailableSockets, 11488u16 => BundleItem::BlockRating, 12031u16 =>
    BundleItem::ClanName, 10752u16 => BundleItem::CombatStyle, 11492u16 =>
    BundleItem::CritDamageRating, 11496u16 => BundleItem::CritHitRating, 10656u16 =>
    BundleItem::Disguise, 12210u16 => BundleItem::DisplayNameColor, 12209u16 =>
    BundleItem::DisplayNameNumber, 12211u16 => BundleItem::DisplayNameRarity, 12213u16 =>
    BundleItem::DisplayNameSlot, 12212u16 => BundleItem::DisplayNameStat, 11490u16 =>
    BundleItem::DodgeRating, 10649u16 => BundleItem::Durability, 10642u16 =>
    BundleItem::DurabilityCurrent, 11499u16 => BundleItem::Focus, 11016u16 =>
    BundleItem::Gender, 11494u16 => BundleItem::HeavyRating, 11497u16 =>
    BundleItem::HitRating, 10757u16 => BundleItem::IsBanked, 10766u16 =>
    BundleItem::IsExpired, 10636u16 => BundleItem::IsSku, 10661u16 =>
    BundleItem::IsTemplate, 10635u16 => BundleItem::ItemActionGenericParam, 10897u16 =>
    BundleItem::LevelDropVariance, 10749u16 => BundleItem::MaxUse, 12215u16 =>
    BundleItem::NoteCaption, 12214u16 => BundleItem::NoteCaptionValue, 10764u16 =>
    BundleItem::OtherClientInterests, 11489u16 => BundleItem::ParryRating, 11495u16 =>
    BundleItem::PeneRating, 10662u16 => BundleItem::Prefix, 12271u16 =>
    BundleItem::QuickbarItem, 10641u16 => BundleItem::RepairCost, 10640u16 =>
    BundleItem::SchematicCostToCreateItem, 10648u16 => BundleItem::SearchKeywords,
    12382u16 => BundleItem::SetBonuses, 12032u16 => BundleItem::SignOfAvatars, 10632u16
    => BundleItem::Skuid, 10643u16 => BundleItem::SocketLockedStatus, 10645u16 =>
    BundleItem::SocketOccupancyStatus, 10644u16 => BundleItem::SocketUpgradeLevel,
    11493u16 => BundleItem::SpecialRating, 11498u16 => BundleItem::Stamina, 11501u16 =>
    BundleItem::Strength, 10663u16 => BundleItem::Suffix, 10647u16 =>
    BundleItem::TemplateType, 11311u16 => BundleItem::TemplateVersion, 10767u16 =>
    BundleItem::TimeStamp, 12528u16 => BundleItem::UseParamI,
};
impl Attribute for BundleItem {
    fn class() -> Class {
        Class::BundleItem
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
            Self::UseParamI => &Self::UseParamI,
        }
    }
}
impl AttributeInfo for BundleItem {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12356u16,
            Self::AdditionalItemCount2 => 12355u16,
            Self::AdditionalItemCount3 => 12354u16,
            Self::AdditionalItemRequired1 => 11660u16,
            Self::AdditionalItemRequired2 => 11659u16,
            Self::AdditionalItemRequired3 => 11658u16,
            Self::AllowBuy => 10704u16,
            Self::AllowRent => 10705u16,
            Self::AllowSell => 10702u16,
            Self::BlackSomaRequired => 11657u16,
            Self::BlingPrice => 10724u16,
            Self::BlingSellingPrice => 10725u16,
            Self::BlueSomaRequired => 11656u16,
            Self::BonusSlotAmber => 11948u16,
            Self::BonusSlotRuby => 11949u16,
            Self::BonusSlotSapphire => 11950u16,
            Self::BuyDiscount => 10701u16,
            Self::BuyPriceBling => 10699u16,
            Self::BuyPriceGameCash => 10700u16,
            Self::Category => 10707u16,
            Self::Combos => 10688u16,
            Self::ContainerId => 10768u16,
            Self::ContentClass => 10755u16,
            Self::CraftingMapping => 12192u16,
            Self::CraftTime => 11648u16,
            Self::CreationTime => 10769u16,
            Self::CrystaEffects => 11981u16,
            Self::CrystalType => 11983u16,
            Self::CyanSomaRequired => 11655u16,
            Self::Description => 10720u16,
            Self::DestroyMethod => 10727u16,
            Self::Dialogs => 10687u16,
            Self::DisplayName => 10754u16,
            Self::EnableInGame => 10721u16,
            Self::EquipSlot => 10756u16,
            Self::ExpireBuyBack => 11608u16,
            Self::ExpireTime => 10706u16,
            Self::Freq => 10762u16,
            Self::GameCashPrice => 10726u16,
            Self::GreenSomaRequired => 11654u16,
            Self::Icon => 10751u16,
            Self::InfiniteUse => 11462u16,
            Self::InitLeftTime => 12329u16,
            Self::InventorySlotIndex => 10660u16,
            Self::IsCollectFaction => 12164u16,
            Self::IsEquiped => 10758u16,
            Self::IsFactionItem => 12150u16,
            Self::IsGemeCrystal => 11982u16,
            Self::IsHotSeller => 10715u16,
            Self::IsInGlobalShop => 10717u16,
            Self::IsInStock => 10716u16,
            Self::IsNewToShop => 10714u16,
            Self::IsQuestItem => 10657u16,
            Self::IsRecipe => 11649u16,
            Self::IsSomaSeed => 12397u16,
            Self::IsSoulBounded => 10633u16,
            Self::IsTechApproved => 10678u16,
            Self::IsTrialItem => 10692u16,
            Self::ItemCritVar => 11661u16,
            Self::ItemNormalVar => 11662u16,
            Self::LastUseTime => 10680u16,
            Self::LeftTime => 12330u16,
            Self::LootAction => 10747u16,
            Self::Lua => 10639u16,
            Self::Lvl => 10745u16,
            Self::LvlReq => 10759u16,
            Self::MaterialOverride => 10750u16,
            Self::MaxStackSize => 10658u16,
            Self::OrangeSomaRequired => 11653u16,
            Self::Power => 10761u16,
            Self::Quantity => 10730u16,
            Self::QuestTrigger => 10694u16,
            Self::Rarity => 10743u16,
            Self::RedSomaRequired => 11652u16,
            Self::RentalDurationMax => 10709u16,
            Self::RentalDurationMin => 10708u16,
            Self::RentDiscount => 10698u16,
            Self::RentPriceBling => 10696u16,
            Self::RentPriceGameCash => 10697u16,
            Self::SellPriceBling => 10703u16,
            Self::SlotId => 10760u16,
            Self::SlotMapping => 10744u16,
            Self::SomaType => 12396u16,
            Self::SoulBoundedAccountId => 12258u16,
            Self::SoulBoundedAvatarId => 10631u16,
            Self::SoulBoundedToAccount => 12247u16,
            Self::SoulBoundType => 10634u16,
            Self::StackCount => 10659u16,
            Self::StandingReq => 12163u16,
            Self::UseAction => 10746u16,
            Self::UseCoolDownTimer => 10682u16,
            Self::UseCount => 10679u16,
            Self::UseMaxCount => 10681u16,
            Self::UseRequireAvatar => 10684u16,
            Self::UseRequireAvatarWithinRadius => 10685u16,
            Self::UseRequireTarget => 10683u16,
            Self::UseScript => 10686u16,
            Self::Vendorable => 10728u16,
            Self::VendorAction => 10748u16,
            Self::VioletSomaRequired => 11651u16,
            Self::YellowSomaRequired => 11650u16,
            Self::Abilities => 10763u16,
            Self::AbilityInstanceData => 10765u16,
            Self::Agility => 11500u16,
            Self::Armor => 11487u16,
            Self::AttackPowerRating => 11491u16,
            Self::AttributeOp1 => 10735u16,
            Self::AttributeOp2 => 10736u16,
            Self::AttributeOp3 => 10737u16,
            Self::AttributeOp4 => 10738u16,
            Self::AttributeType1 => 10731u16,
            Self::AttributeType2 => 10732u16,
            Self::AttributeType3 => 10733u16,
            Self::AttributeType4 => 10734u16,
            Self::AttributeWeight1 => 10739u16,
            Self::AttributeWeight2 => 10740u16,
            Self::AttributeWeight3 => 10741u16,
            Self::AttributeWeight4 => 10742u16,
            Self::AutoAttributeType1 => 10668u16,
            Self::AutoAttributeType2 => 10669u16,
            Self::AutoAttributeType3 => 10670u16,
            Self::AutoAttributeType4 => 10671u16,
            Self::AutoAttributeType5 => 10664u16,
            Self::AutoAttributeType6 => 10665u16,
            Self::AutoAttributeValue1 => 10672u16,
            Self::AutoAttributeValue2 => 10673u16,
            Self::AutoAttributeValue3 => 10674u16,
            Self::AutoAttributeValue4 => 10675u16,
            Self::AutoAttributeValue5 => 10666u16,
            Self::AutoAttributeValue6 => 10667u16,
            Self::AvailableSockets => 10646u16,
            Self::BlockRating => 11488u16,
            Self::ClanName => 12031u16,
            Self::CombatStyle => 10752u16,
            Self::CritDamageRating => 11492u16,
            Self::CritHitRating => 11496u16,
            Self::Disguise => 10656u16,
            Self::DisplayNameColor => 12210u16,
            Self::DisplayNameNumber => 12209u16,
            Self::DisplayNameRarity => 12211u16,
            Self::DisplayNameSlot => 12213u16,
            Self::DisplayNameStat => 12212u16,
            Self::DodgeRating => 11490u16,
            Self::Durability => 10649u16,
            Self::DurabilityCurrent => 10642u16,
            Self::Focus => 11499u16,
            Self::Gender => 11016u16,
            Self::HeavyRating => 11494u16,
            Self::HitRating => 11497u16,
            Self::IsBanked => 10757u16,
            Self::IsExpired => 10766u16,
            Self::IsSku => 10636u16,
            Self::IsTemplate => 10661u16,
            Self::ItemActionGenericParam => 10635u16,
            Self::LevelDropVariance => 10897u16,
            Self::MaxUse => 10749u16,
            Self::NoteCaption => 12215u16,
            Self::NoteCaptionValue => 12214u16,
            Self::OtherClientInterests => 10764u16,
            Self::ParryRating => 11489u16,
            Self::PeneRating => 11495u16,
            Self::Prefix => 10662u16,
            Self::QuickbarItem => 12271u16,
            Self::RepairCost => 10641u16,
            Self::SchematicCostToCreateItem => 10640u16,
            Self::SearchKeywords => 10648u16,
            Self::SetBonuses => 12382u16,
            Self::SignOfAvatars => 12032u16,
            Self::Skuid => 10632u16,
            Self::SocketLockedStatus => 10643u16,
            Self::SocketOccupancyStatus => 10645u16,
            Self::SocketUpgradeLevel => 10644u16,
            Self::SpecialRating => 11493u16,
            Self::Stamina => 11498u16,
            Self::Strength => 11501u16,
            Self::Suffix => 10663u16,
            Self::TemplateType => 10647u16,
            Self::TemplateVersion => 11311u16,
            Self::TimeStamp => 10767u16,
            Self::UseParamI => 12528u16,
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
            Self::UseParamI => "UseParamI",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AllowBuy => ParamType::Bool,
            Self::AllowRent => ParamType::Bool,
            Self::AllowSell => ParamType::Bool,
            Self::BuyDiscount => ParamType::Float,
            Self::BuyPriceBling => ParamType::Int,
            Self::BuyPriceGameCash => ParamType::Int,
            Self::Description => ParamType::LocalizedString,
            Self::DisplayName => ParamType::LocalizedString,
            Self::Icon => ParamType::String,
            Self::IsHotSeller => ParamType::Bool,
            Self::IsInGlobalShop => ParamType::Bool,
            Self::IsNewToShop => ParamType::Bool,
            Self::IsQuestItem => ParamType::Bool,
            Self::IsTechApproved => ParamType::Bool,
            Self::IsTrialItem => ParamType::Bool,
            Self::Lvl => ParamType::Int,
            Self::Rarity => ParamType::String,
            Self::RentDiscount => ParamType::Float,
            Self::RentPriceBling => ParamType::Float,
            Self::RentPriceGameCash => ParamType::Float,
            Self::SellPriceBling => ParamType::Int,
            Self::SlotMapping => ParamType::String,
            Self::UseAction => ParamType::String,
            Self::VendorAction => ParamType::String,
            Self::CombatStyle => ParamType::Int,
            Self::IsSku => ParamType::Bool,
            Self::SearchKeywords => ParamType::VectorLocalizedString,
            Self::Skuid => ParamType::String,
            Self::UseParamI => ParamType::Int,
            Self::AdditionalItemCount1 => ParamType::Int,
            Self::AdditionalItemCount2 => ParamType::Int,
            Self::AdditionalItemCount3 => ParamType::Int,
            Self::AdditionalItemRequired1 => ParamType::Guid,
            Self::AdditionalItemRequired2 => ParamType::Guid,
            Self::AdditionalItemRequired3 => ParamType::Guid,
            Self::BlackSomaRequired => ParamType::Int,
            Self::BlingPrice => ParamType::Int,
            Self::BlingSellingPrice => ParamType::Int,
            Self::BlueSomaRequired => ParamType::Int,
            Self::BonusSlotAmber => ParamType::Guid,
            Self::BonusSlotRuby => ParamType::Guid,
            Self::BonusSlotSapphire => ParamType::Guid,
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
            Self::DestroyMethod => ParamType::String,
            Self::Dialogs => ParamType::VectorInt,
            Self::EnableInGame => ParamType::Bool,
            Self::EquipSlot => ParamType::String,
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
            Self::IsInStock => ParamType::Bool,
            Self::IsRecipe => ParamType::Bool,
            Self::IsSomaSeed => ParamType::Bool,
            Self::IsSoulBounded => ParamType::Bool,
            Self::ItemCritVar => ParamType::Guid,
            Self::ItemNormalVar => ParamType::Guid,
            Self::LastUseTime => ParamType::Int64,
            Self::LeftTime => ParamType::Int64,
            Self::LootAction => ParamType::String,
            Self::Lua => ParamType::String,
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
            Self::SlotId => ParamType::Int,
            Self::SomaType => ParamType::Int,
            Self::SoulBoundedAccountId => ParamType::Int,
            Self::SoulBoundedAvatarId => ParamType::AvatarId,
            Self::SoulBoundedToAccount => ParamType::Bool,
            Self::SoulBoundType => ParamType::String,
            Self::StackCount => ParamType::Int,
            Self::StandingReq => ParamType::Int,
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
            Self::SetBonuses => ParamType::String,
            Self::SignOfAvatars => ParamType::VectorInt,
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
        static ALLOW_BUY: Value = Value::Bool(true);
        static ALLOW_RENT: Value = Value::Bool(true);
        static ALLOW_SELL: Value = Value::Bool(false);
        static BUY_DISCOUNT: Value = Value::Float(1f32);
        static BUY_PRICE_BLING: Value = Value::Int(0i32);
        static BUY_PRICE_GAME_CASH: Value = Value::Int(0i32);
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
        static ICON: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static IS_HOT_SELLER: Value = Value::Bool(false);
        static IS_IN_GLOBAL_SHOP: Value = Value::Bool(true);
        static IS_NEW_TO_SHOP: Value = Value::Bool(false);
        static IS_QUEST_ITEM: Value = Value::Bool(false);
        static IS_TECH_APPROVED: Value = Value::Bool(true);
        static IS_TRIAL_ITEM: Value = Value::Bool(false);
        static LVL: Value = Value::Int(1i32);
        static RARITY: Lazy<Value> = Lazy::new(|| Value::String("Normal".to_string()));
        static RENT_DISCOUNT: Value = Value::Float(1f32);
        static RENT_PRICE_BLING: Value = Value::Float(0f32);
        static RENT_PRICE_GAME_CASH: Value = Value::Float(0f32);
        static SELL_PRICE_BLING: Value = Value::Int(0i32);
        static SLOT_MAPPING: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static USE_ACTION: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static VENDOR_ACTION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static COMBAT_STYLE: Value = Value::Int(-1i32);
        static IS_SKU: Value = Value::Bool(false);
        static SEARCH_KEYWORDS: Value = Value::VectorLocalizedString(vec![]);
        static SKUID: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static USE_PARAM_I: Value = Value::Int(0i32);
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
        static DESTROY_METHOD: Lazy<Value> = Lazy::new(|| Value::String(
            "CannotDestroy".to_string(),
        ));
        static DIALOGS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
        static ENABLE_IN_GAME: Value = Value::Bool(true);
        static EQUIP_SLOT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
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
        static IS_IN_STOCK: Value = Value::Bool(false);
        static IS_RECIPE: Value = Value::Bool(false);
        static IS_SOMA_SEED: Value = Value::Bool(false);
        static IS_SOUL_BOUNDED: Value = Value::Bool(false);
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
        static SET_BONUSES: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static SIGN_OF_AVATARS: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![]));
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
            Self::AllowBuy => &ALLOW_BUY,
            Self::AllowRent => &ALLOW_RENT,
            Self::AllowSell => &ALLOW_SELL,
            Self::BuyDiscount => &BUY_DISCOUNT,
            Self::BuyPriceBling => &BUY_PRICE_BLING,
            Self::BuyPriceGameCash => &BUY_PRICE_GAME_CASH,
            Self::Description => &DESCRIPTION,
            Self::DisplayName => &DISPLAY_NAME,
            Self::Icon => &ICON,
            Self::IsHotSeller => &IS_HOT_SELLER,
            Self::IsInGlobalShop => &IS_IN_GLOBAL_SHOP,
            Self::IsNewToShop => &IS_NEW_TO_SHOP,
            Self::IsQuestItem => &IS_QUEST_ITEM,
            Self::IsTechApproved => &IS_TECH_APPROVED,
            Self::IsTrialItem => &IS_TRIAL_ITEM,
            Self::Lvl => &LVL,
            Self::Rarity => &RARITY,
            Self::RentDiscount => &RENT_DISCOUNT,
            Self::RentPriceBling => &RENT_PRICE_BLING,
            Self::RentPriceGameCash => &RENT_PRICE_GAME_CASH,
            Self::SellPriceBling => &SELL_PRICE_BLING,
            Self::SlotMapping => &SLOT_MAPPING,
            Self::UseAction => &USE_ACTION,
            Self::VendorAction => &VENDOR_ACTION,
            Self::CombatStyle => &COMBAT_STYLE,
            Self::IsSku => &IS_SKU,
            Self::SearchKeywords => &SEARCH_KEYWORDS,
            Self::Skuid => &SKUID,
            Self::UseParamI => &USE_PARAM_I,
            Self::AdditionalItemCount1 => &ADDITIONAL_ITEM_COUNT_1,
            Self::AdditionalItemCount2 => &ADDITIONAL_ITEM_COUNT_2,
            Self::AdditionalItemCount3 => &ADDITIONAL_ITEM_COUNT_3,
            Self::AdditionalItemRequired1 => &ADDITIONAL_ITEM_REQUIRED_1,
            Self::AdditionalItemRequired2 => &ADDITIONAL_ITEM_REQUIRED_2,
            Self::AdditionalItemRequired3 => &ADDITIONAL_ITEM_REQUIRED_3,
            Self::BlackSomaRequired => &BLACK_SOMA_REQUIRED,
            Self::BlingPrice => &BLING_PRICE,
            Self::BlingSellingPrice => &BLING_SELLING_PRICE,
            Self::BlueSomaRequired => &BLUE_SOMA_REQUIRED,
            Self::BonusSlotAmber => &BONUS_SLOT_AMBER,
            Self::BonusSlotRuby => &BONUS_SLOT_RUBY,
            Self::BonusSlotSapphire => &BONUS_SLOT_SAPPHIRE,
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
            Self::DestroyMethod => &DESTROY_METHOD,
            Self::Dialogs => &DIALOGS,
            Self::EnableInGame => &ENABLE_IN_GAME,
            Self::EquipSlot => &EQUIP_SLOT,
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
            Self::IsInStock => &IS_IN_STOCK,
            Self::IsRecipe => &IS_RECIPE,
            Self::IsSomaSeed => &IS_SOMA_SEED,
            Self::IsSoulBounded => &IS_SOUL_BOUNDED,
            Self::ItemCritVar => &ITEM_CRIT_VAR,
            Self::ItemNormalVar => &ITEM_NORMAL_VAR,
            Self::LastUseTime => &LAST_USE_TIME,
            Self::LeftTime => &LEFT_TIME,
            Self::LootAction => &LOOT_ACTION,
            Self::Lua => &LUA,
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
            Self::SlotId => &SLOT_ID,
            Self::SomaType => &SOMA_TYPE,
            Self::SoulBoundedAccountId => &SOUL_BOUNDED_ACCOUNT_ID,
            Self::SoulBoundedAvatarId => &SOUL_BOUNDED_AVATAR_ID,
            Self::SoulBoundedToAccount => &SOUL_BOUNDED_TO_ACCOUNT,
            Self::SoulBoundType => &SOUL_BOUND_TYPE,
            Self::StackCount => &STACK_COUNT,
            Self::StandingReq => &STANDING_REQ,
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
            Self::SetBonuses => &SET_BONUSES,
            Self::SignOfAvatars => &SIGN_OF_AVATARS,
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
            Self::AllowBuy => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AllowRent => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AllowSell => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::BuyDiscount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
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
                &[
                    ParamFlag::NodeOwn,
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
            Self::Icon => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsHotSeller => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsInGlobalShop => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::IsNewToShop => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsQuestItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsTechApproved => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                    ParamFlag::ExcludeFromClient,
                ]
            }
            Self::IsTrialItem => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
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
            Self::Rarity => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RentDiscount => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RentPriceBling => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::RentPriceGameCash => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SellPriceBling => {
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
            Self::UseAction => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::VendorAction => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::CombatStyle => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsSku => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::SearchKeywords => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Skuid => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::UseParamI => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::AdditionalItemCount1 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemCount2 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemCount3 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired1 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired2 => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::AdditionalItemRequired3 => &[ParamFlag::Persistent, ParamFlag::Content],
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
            Self::DestroyMethod => &[ParamFlag::Persistent],
            Self::Dialogs => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
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
            Self::IsInStock => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
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
impl FromStr for BundleItem {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BUNDLE_ITEM_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for BundleItem {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12356u16 => Ok(Self::AdditionalItemCount1),
            12355u16 => Ok(Self::AdditionalItemCount2),
            12354u16 => Ok(Self::AdditionalItemCount3),
            11660u16 => Ok(Self::AdditionalItemRequired1),
            11659u16 => Ok(Self::AdditionalItemRequired2),
            11658u16 => Ok(Self::AdditionalItemRequired3),
            10704u16 => Ok(Self::AllowBuy),
            10705u16 => Ok(Self::AllowRent),
            10702u16 => Ok(Self::AllowSell),
            11657u16 => Ok(Self::BlackSomaRequired),
            10724u16 => Ok(Self::BlingPrice),
            10725u16 => Ok(Self::BlingSellingPrice),
            11656u16 => Ok(Self::BlueSomaRequired),
            11948u16 => Ok(Self::BonusSlotAmber),
            11949u16 => Ok(Self::BonusSlotRuby),
            11950u16 => Ok(Self::BonusSlotSapphire),
            10701u16 => Ok(Self::BuyDiscount),
            10699u16 => Ok(Self::BuyPriceBling),
            10700u16 => Ok(Self::BuyPriceGameCash),
            10707u16 => Ok(Self::Category),
            10688u16 => Ok(Self::Combos),
            10768u16 => Ok(Self::ContainerId),
            10755u16 => Ok(Self::ContentClass),
            12192u16 => Ok(Self::CraftingMapping),
            11648u16 => Ok(Self::CraftTime),
            10769u16 => Ok(Self::CreationTime),
            11981u16 => Ok(Self::CrystaEffects),
            11983u16 => Ok(Self::CrystalType),
            11655u16 => Ok(Self::CyanSomaRequired),
            10720u16 => Ok(Self::Description),
            10727u16 => Ok(Self::DestroyMethod),
            10687u16 => Ok(Self::Dialogs),
            10754u16 => Ok(Self::DisplayName),
            10721u16 => Ok(Self::EnableInGame),
            10756u16 => Ok(Self::EquipSlot),
            11608u16 => Ok(Self::ExpireBuyBack),
            10706u16 => Ok(Self::ExpireTime),
            10762u16 => Ok(Self::Freq),
            10726u16 => Ok(Self::GameCashPrice),
            11654u16 => Ok(Self::GreenSomaRequired),
            10751u16 => Ok(Self::Icon),
            11462u16 => Ok(Self::InfiniteUse),
            12329u16 => Ok(Self::InitLeftTime),
            10660u16 => Ok(Self::InventorySlotIndex),
            12164u16 => Ok(Self::IsCollectFaction),
            10758u16 => Ok(Self::IsEquiped),
            12150u16 => Ok(Self::IsFactionItem),
            11982u16 => Ok(Self::IsGemeCrystal),
            10715u16 => Ok(Self::IsHotSeller),
            10717u16 => Ok(Self::IsInGlobalShop),
            10716u16 => Ok(Self::IsInStock),
            10714u16 => Ok(Self::IsNewToShop),
            10657u16 => Ok(Self::IsQuestItem),
            11649u16 => Ok(Self::IsRecipe),
            12397u16 => Ok(Self::IsSomaSeed),
            10633u16 => Ok(Self::IsSoulBounded),
            10678u16 => Ok(Self::IsTechApproved),
            10692u16 => Ok(Self::IsTrialItem),
            11661u16 => Ok(Self::ItemCritVar),
            11662u16 => Ok(Self::ItemNormalVar),
            10680u16 => Ok(Self::LastUseTime),
            12330u16 => Ok(Self::LeftTime),
            10747u16 => Ok(Self::LootAction),
            10639u16 => Ok(Self::Lua),
            10745u16 => Ok(Self::Lvl),
            10759u16 => Ok(Self::LvlReq),
            10750u16 => Ok(Self::MaterialOverride),
            10658u16 => Ok(Self::MaxStackSize),
            11653u16 => Ok(Self::OrangeSomaRequired),
            10761u16 => Ok(Self::Power),
            10730u16 => Ok(Self::Quantity),
            10694u16 => Ok(Self::QuestTrigger),
            10743u16 => Ok(Self::Rarity),
            11652u16 => Ok(Self::RedSomaRequired),
            10709u16 => Ok(Self::RentalDurationMax),
            10708u16 => Ok(Self::RentalDurationMin),
            10698u16 => Ok(Self::RentDiscount),
            10696u16 => Ok(Self::RentPriceBling),
            10697u16 => Ok(Self::RentPriceGameCash),
            10703u16 => Ok(Self::SellPriceBling),
            10760u16 => Ok(Self::SlotId),
            10744u16 => Ok(Self::SlotMapping),
            12396u16 => Ok(Self::SomaType),
            12258u16 => Ok(Self::SoulBoundedAccountId),
            10631u16 => Ok(Self::SoulBoundedAvatarId),
            12247u16 => Ok(Self::SoulBoundedToAccount),
            10634u16 => Ok(Self::SoulBoundType),
            10659u16 => Ok(Self::StackCount),
            12163u16 => Ok(Self::StandingReq),
            10746u16 => Ok(Self::UseAction),
            10682u16 => Ok(Self::UseCoolDownTimer),
            10679u16 => Ok(Self::UseCount),
            10681u16 => Ok(Self::UseMaxCount),
            10684u16 => Ok(Self::UseRequireAvatar),
            10685u16 => Ok(Self::UseRequireAvatarWithinRadius),
            10683u16 => Ok(Self::UseRequireTarget),
            10686u16 => Ok(Self::UseScript),
            10728u16 => Ok(Self::Vendorable),
            10748u16 => Ok(Self::VendorAction),
            11651u16 => Ok(Self::VioletSomaRequired),
            11650u16 => Ok(Self::YellowSomaRequired),
            10763u16 => Ok(Self::Abilities),
            10765u16 => Ok(Self::AbilityInstanceData),
            11500u16 => Ok(Self::Agility),
            11487u16 => Ok(Self::Armor),
            11491u16 => Ok(Self::AttackPowerRating),
            10735u16 => Ok(Self::AttributeOp1),
            10736u16 => Ok(Self::AttributeOp2),
            10737u16 => Ok(Self::AttributeOp3),
            10738u16 => Ok(Self::AttributeOp4),
            10731u16 => Ok(Self::AttributeType1),
            10732u16 => Ok(Self::AttributeType2),
            10733u16 => Ok(Self::AttributeType3),
            10734u16 => Ok(Self::AttributeType4),
            10739u16 => Ok(Self::AttributeWeight1),
            10740u16 => Ok(Self::AttributeWeight2),
            10741u16 => Ok(Self::AttributeWeight3),
            10742u16 => Ok(Self::AttributeWeight4),
            10668u16 => Ok(Self::AutoAttributeType1),
            10669u16 => Ok(Self::AutoAttributeType2),
            10670u16 => Ok(Self::AutoAttributeType3),
            10671u16 => Ok(Self::AutoAttributeType4),
            10664u16 => Ok(Self::AutoAttributeType5),
            10665u16 => Ok(Self::AutoAttributeType6),
            10672u16 => Ok(Self::AutoAttributeValue1),
            10673u16 => Ok(Self::AutoAttributeValue2),
            10674u16 => Ok(Self::AutoAttributeValue3),
            10675u16 => Ok(Self::AutoAttributeValue4),
            10666u16 => Ok(Self::AutoAttributeValue5),
            10667u16 => Ok(Self::AutoAttributeValue6),
            10646u16 => Ok(Self::AvailableSockets),
            11488u16 => Ok(Self::BlockRating),
            12031u16 => Ok(Self::ClanName),
            10752u16 => Ok(Self::CombatStyle),
            11492u16 => Ok(Self::CritDamageRating),
            11496u16 => Ok(Self::CritHitRating),
            10656u16 => Ok(Self::Disguise),
            12210u16 => Ok(Self::DisplayNameColor),
            12209u16 => Ok(Self::DisplayNameNumber),
            12211u16 => Ok(Self::DisplayNameRarity),
            12213u16 => Ok(Self::DisplayNameSlot),
            12212u16 => Ok(Self::DisplayNameStat),
            11490u16 => Ok(Self::DodgeRating),
            10649u16 => Ok(Self::Durability),
            10642u16 => Ok(Self::DurabilityCurrent),
            11499u16 => Ok(Self::Focus),
            11016u16 => Ok(Self::Gender),
            11494u16 => Ok(Self::HeavyRating),
            11497u16 => Ok(Self::HitRating),
            10757u16 => Ok(Self::IsBanked),
            10766u16 => Ok(Self::IsExpired),
            10636u16 => Ok(Self::IsSku),
            10661u16 => Ok(Self::IsTemplate),
            10635u16 => Ok(Self::ItemActionGenericParam),
            10897u16 => Ok(Self::LevelDropVariance),
            10749u16 => Ok(Self::MaxUse),
            12215u16 => Ok(Self::NoteCaption),
            12214u16 => Ok(Self::NoteCaptionValue),
            10764u16 => Ok(Self::OtherClientInterests),
            11489u16 => Ok(Self::ParryRating),
            11495u16 => Ok(Self::PeneRating),
            10662u16 => Ok(Self::Prefix),
            12271u16 => Ok(Self::QuickbarItem),
            10641u16 => Ok(Self::RepairCost),
            10640u16 => Ok(Self::SchematicCostToCreateItem),
            10648u16 => Ok(Self::SearchKeywords),
            12382u16 => Ok(Self::SetBonuses),
            12032u16 => Ok(Self::SignOfAvatars),
            10632u16 => Ok(Self::Skuid),
            10643u16 => Ok(Self::SocketLockedStatus),
            10645u16 => Ok(Self::SocketOccupancyStatus),
            10644u16 => Ok(Self::SocketUpgradeLevel),
            11493u16 => Ok(Self::SpecialRating),
            11498u16 => Ok(Self::Stamina),
            11501u16 => Ok(Self::Strength),
            10663u16 => Ok(Self::Suffix),
            10647u16 => Ok(Self::TemplateType),
            11311u16 => Ok(Self::TemplateVersion),
            10767u16 => Ok(Self::TimeStamp),
            12528u16 => Ok(Self::UseParamI),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
