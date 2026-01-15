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
pub enum ItemMyLandTheme {
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
}
pub(crate) static ITEM_MY_LAND_THEME_ATTRIBUTES: phf::Map<
    &'static str,
    ItemMyLandTheme,
> = phf_map! {
    "AdditionalItemCount1" => ItemMyLandTheme::AdditionalItemCount1,
    "AdditionalItemCount2" => ItemMyLandTheme::AdditionalItemCount2,
    "AdditionalItemCount3" => ItemMyLandTheme::AdditionalItemCount3,
    "AdditionalItemRequired1" => ItemMyLandTheme::AdditionalItemRequired1,
    "AdditionalItemRequired2" => ItemMyLandTheme::AdditionalItemRequired2,
    "AdditionalItemRequired3" => ItemMyLandTheme::AdditionalItemRequired3, "AllowBuy" =>
    ItemMyLandTheme::AllowBuy, "AllowRent" => ItemMyLandTheme::AllowRent, "AllowSell" =>
    ItemMyLandTheme::AllowSell, "BlackSomaRequired" =>
    ItemMyLandTheme::BlackSomaRequired, "blingPrice" => ItemMyLandTheme::BlingPrice,
    "blingSellingPrice" => ItemMyLandTheme::BlingSellingPrice, "BlueSomaRequired" =>
    ItemMyLandTheme::BlueSomaRequired, "BonusSlotAmber" =>
    ItemMyLandTheme::BonusSlotAmber, "BonusSlotRuby" => ItemMyLandTheme::BonusSlotRuby,
    "BonusSlotSapphire" => ItemMyLandTheme::BonusSlotSapphire, "BuyDiscount" =>
    ItemMyLandTheme::BuyDiscount, "BuyPriceBling" => ItemMyLandTheme::BuyPriceBling,
    "BuyPriceGameCash" => ItemMyLandTheme::BuyPriceGameCash, "Category" =>
    ItemMyLandTheme::Category, "combos" => ItemMyLandTheme::Combos, "containerID" =>
    ItemMyLandTheme::ContainerId, "ContentClass" => ItemMyLandTheme::ContentClass,
    "CraftingMapping" => ItemMyLandTheme::CraftingMapping, "CraftTime" =>
    ItemMyLandTheme::CraftTime, "creationTime" => ItemMyLandTheme::CreationTime,
    "CrystaEffects" => ItemMyLandTheme::CrystaEffects, "CrystalType" =>
    ItemMyLandTheme::CrystalType, "CyanSomaRequired" =>
    ItemMyLandTheme::CyanSomaRequired, "Description" => ItemMyLandTheme::Description,
    "DestroyMethod" => ItemMyLandTheme::DestroyMethod, "Dialogs" =>
    ItemMyLandTheme::Dialogs, "DisplayName" => ItemMyLandTheme::DisplayName,
    "EnableInGame" => ItemMyLandTheme::EnableInGame, "equipSlot" =>
    ItemMyLandTheme::EquipSlot, "expireBuyBack" => ItemMyLandTheme::ExpireBuyBack,
    "ExpireTime" => ItemMyLandTheme::ExpireTime, "Freq" => ItemMyLandTheme::Freq,
    "gameCashPrice" => ItemMyLandTheme::GameCashPrice, "GreenSomaRequired" =>
    ItemMyLandTheme::GreenSomaRequired, "Icon" => ItemMyLandTheme::Icon, "InfiniteUse" =>
    ItemMyLandTheme::InfiniteUse, "InitLeftTime" => ItemMyLandTheme::InitLeftTime,
    "inventorySlotIndex" => ItemMyLandTheme::InventorySlotIndex, "isCollectFaction" =>
    ItemMyLandTheme::IsCollectFaction, "isEquiped" => ItemMyLandTheme::IsEquiped,
    "isFactionItem" => ItemMyLandTheme::IsFactionItem, "isGemeCrystal" =>
    ItemMyLandTheme::IsGemeCrystal, "IsHotSeller" => ItemMyLandTheme::IsHotSeller,
    "isInGlobalShop" => ItemMyLandTheme::IsInGlobalShop, "IsInStock" =>
    ItemMyLandTheme::IsInStock, "IsNewToShop" => ItemMyLandTheme::IsNewToShop,
    "isQuestItem" => ItemMyLandTheme::IsQuestItem, "IsRecipe" =>
    ItemMyLandTheme::IsRecipe, "IsSomaSeed" => ItemMyLandTheme::IsSomaSeed,
    "IsSoulBounded" => ItemMyLandTheme::IsSoulBounded, "isTechApproved" =>
    ItemMyLandTheme::IsTechApproved, "isTrialItem" => ItemMyLandTheme::IsTrialItem,
    "ItemCritVar" => ItemMyLandTheme::ItemCritVar, "ItemNormalVar" =>
    ItemMyLandTheme::ItemNormalVar, "LastUseTime" => ItemMyLandTheme::LastUseTime,
    "LeftTime" => ItemMyLandTheme::LeftTime, "lootAction" => ItemMyLandTheme::LootAction,
    "Lua" => ItemMyLandTheme::Lua, "lvl" => ItemMyLandTheme::Lvl, "lvlReq" =>
    ItemMyLandTheme::LvlReq, "MaterialOverride" => ItemMyLandTheme::MaterialOverride,
    "maxStackSize" => ItemMyLandTheme::MaxStackSize, "OrangeSomaRequired" =>
    ItemMyLandTheme::OrangeSomaRequired, "Power" => ItemMyLandTheme::Power, "quantity" =>
    ItemMyLandTheme::Quantity, "QuestTrigger" => ItemMyLandTheme::QuestTrigger, "rarity"
    => ItemMyLandTheme::Rarity, "RedSomaRequired" => ItemMyLandTheme::RedSomaRequired,
    "RentalDurationMax" => ItemMyLandTheme::RentalDurationMax, "RentalDurationMin" =>
    ItemMyLandTheme::RentalDurationMin, "RentDiscount" => ItemMyLandTheme::RentDiscount,
    "RentPriceBling" => ItemMyLandTheme::RentPriceBling, "RentPriceGameCash" =>
    ItemMyLandTheme::RentPriceGameCash, "SellPriceBling" =>
    ItemMyLandTheme::SellPriceBling, "slotID" => ItemMyLandTheme::SlotId, "SlotMapping"
    => ItemMyLandTheme::SlotMapping, "SomaType" => ItemMyLandTheme::SomaType,
    "SoulBoundedAccountId" => ItemMyLandTheme::SoulBoundedAccountId,
    "SoulBoundedAvatarId" => ItemMyLandTheme::SoulBoundedAvatarId, "SoulBoundedToAccount"
    => ItemMyLandTheme::SoulBoundedToAccount, "SoulBoundType" =>
    ItemMyLandTheme::SoulBoundType, "stackCount" => ItemMyLandTheme::StackCount,
    "standingReq" => ItemMyLandTheme::StandingReq, "useAction" =>
    ItemMyLandTheme::UseAction, "UseCoolDownTimer" => ItemMyLandTheme::UseCoolDownTimer,
    "UseCount" => ItemMyLandTheme::UseCount, "UseMaxCount" =>
    ItemMyLandTheme::UseMaxCount, "UseRequireAvatar" =>
    ItemMyLandTheme::UseRequireAvatar, "UseRequireAvatarWithinRadius" =>
    ItemMyLandTheme::UseRequireAvatarWithinRadius, "UseRequireTarget" =>
    ItemMyLandTheme::UseRequireTarget, "UseScript" => ItemMyLandTheme::UseScript,
    "Vendorable" => ItemMyLandTheme::Vendorable, "vendorAction" =>
    ItemMyLandTheme::VendorAction, "VioletSomaRequired" =>
    ItemMyLandTheme::VioletSomaRequired, "YellowSomaRequired" =>
    ItemMyLandTheme::YellowSomaRequired,
};
pub(crate) static ITEM_MY_LAND_THEME_ATTRIBUTES_ID: phf::Map<u16, ItemMyLandTheme> = phf_map! {
    12374u16 => ItemMyLandTheme::AdditionalItemCount1, 12373u16 =>
    ItemMyLandTheme::AdditionalItemCount2, 12372u16 =>
    ItemMyLandTheme::AdditionalItemCount3, 11750u16 =>
    ItemMyLandTheme::AdditionalItemRequired1, 11749u16 =>
    ItemMyLandTheme::AdditionalItemRequired2, 11748u16 =>
    ItemMyLandTheme::AdditionalItemRequired3, 7583u16 => ItemMyLandTheme::AllowBuy,
    7572u16 => ItemMyLandTheme::AllowRent, 7651u16 => ItemMyLandTheme::AllowSell,
    11747u16 => ItemMyLandTheme::BlackSomaRequired, 6588u16 =>
    ItemMyLandTheme::BlingPrice, 6587u16 => ItemMyLandTheme::BlingSellingPrice, 11746u16
    => ItemMyLandTheme::BlueSomaRequired, 11966u16 => ItemMyLandTheme::BonusSlotAmber,
    11967u16 => ItemMyLandTheme::BonusSlotRuby, 11968u16 =>
    ItemMyLandTheme::BonusSlotSapphire, 7652u16 => ItemMyLandTheme::BuyDiscount, 7654u16
    => ItemMyLandTheme::BuyPriceBling, 7653u16 => ItemMyLandTheme::BuyPriceGameCash,
    7550u16 => ItemMyLandTheme::Category, 8899u16 => ItemMyLandTheme::Combos, 5853u16 =>
    ItemMyLandTheme::ContainerId, 5848u16 => ItemMyLandTheme::ContentClass, 12198u16 =>
    ItemMyLandTheme::CraftingMapping, 11738u16 => ItemMyLandTheme::CraftTime, 5854u16 =>
    ItemMyLandTheme::CreationTime, 11999u16 => ItemMyLandTheme::CrystaEffects, 12001u16
    => ItemMyLandTheme::CrystalType, 11745u16 => ItemMyLandTheme::CyanSomaRequired,
    6959u16 => ItemMyLandTheme::Description, 6493u16 => ItemMyLandTheme::DestroyMethod,
    8926u16 => ItemMyLandTheme::Dialogs, 5850u16 => ItemMyLandTheme::DisplayName, 6818u16
    => ItemMyLandTheme::EnableInGame, 5846u16 => ItemMyLandTheme::EquipSlot, 11614u16 =>
    ItemMyLandTheme::ExpireBuyBack, 7561u16 => ItemMyLandTheme::ExpireTime, 5836u16 =>
    ItemMyLandTheme::Freq, 6586u16 => ItemMyLandTheme::GameCashPrice, 11744u16 =>
    ItemMyLandTheme::GreenSomaRequired, 5835u16 => ItemMyLandTheme::Icon, 11468u16 =>
    ItemMyLandTheme::InfiniteUse, 12341u16 => ItemMyLandTheme::InitLeftTime, 9877u16 =>
    ItemMyLandTheme::InventorySlotIndex, 12176u16 => ItemMyLandTheme::IsCollectFaction,
    5845u16 => ItemMyLandTheme::IsEquiped, 12156u16 => ItemMyLandTheme::IsFactionItem,
    12000u16 => ItemMyLandTheme::IsGemeCrystal, 7427u16 => ItemMyLandTheme::IsHotSeller,
    7151u16 => ItemMyLandTheme::IsInGlobalShop, 7426u16 => ItemMyLandTheme::IsInStock,
    7428u16 => ItemMyLandTheme::IsNewToShop, 9914u16 => ItemMyLandTheme::IsQuestItem,
    11739u16 => ItemMyLandTheme::IsRecipe, 12409u16 => ItemMyLandTheme::IsSomaSeed,
    10594u16 => ItemMyLandTheme::IsSoulBounded, 9380u16 =>
    ItemMyLandTheme::IsTechApproved, 7752u16 => ItemMyLandTheme::IsTrialItem, 11751u16 =>
    ItemMyLandTheme::ItemCritVar, 11752u16 => ItemMyLandTheme::ItemNormalVar, 9018u16 =>
    ItemMyLandTheme::LastUseTime, 12342u16 => ItemMyLandTheme::LeftTime, 5997u16 =>
    ItemMyLandTheme::LootAction, 10159u16 => ItemMyLandTheme::Lua, 6177u16 =>
    ItemMyLandTheme::Lvl, 5841u16 => ItemMyLandTheme::LvlReq, 5834u16 =>
    ItemMyLandTheme::MaterialOverride, 9899u16 => ItemMyLandTheme::MaxStackSize, 11743u16
    => ItemMyLandTheme::OrangeSomaRequired, 5837u16 => ItemMyLandTheme::Power, 6437u16 =>
    ItemMyLandTheme::Quantity, 7726u16 => ItemMyLandTheme::QuestTrigger, 6282u16 =>
    ItemMyLandTheme::Rarity, 11742u16 => ItemMyLandTheme::RedSomaRequired, 7468u16 =>
    ItemMyLandTheme::RentalDurationMax, 7469u16 => ItemMyLandTheme::RentalDurationMin,
    7655u16 => ItemMyLandTheme::RentDiscount, 7657u16 => ItemMyLandTheme::RentPriceBling,
    7656u16 => ItemMyLandTheme::RentPriceGameCash, 7650u16 =>
    ItemMyLandTheme::SellPriceBling, 5838u16 => ItemMyLandTheme::SlotId, 6251u16 =>
    ItemMyLandTheme::SlotMapping, 12408u16 => ItemMyLandTheme::SomaType, 12264u16 =>
    ItemMyLandTheme::SoulBoundedAccountId, 10618u16 =>
    ItemMyLandTheme::SoulBoundedAvatarId, 12253u16 =>
    ItemMyLandTheme::SoulBoundedToAccount, 10593u16 => ItemMyLandTheme::SoulBoundType,
    9898u16 => ItemMyLandTheme::StackCount, 12175u16 => ItemMyLandTheme::StandingReq,
    6022u16 => ItemMyLandTheme::UseAction, 9004u16 => ItemMyLandTheme::UseCoolDownTimer,
    9029u16 => ItemMyLandTheme::UseCount, 9005u16 => ItemMyLandTheme::UseMaxCount,
    8976u16 => ItemMyLandTheme::UseRequireAvatar, 8975u16 =>
    ItemMyLandTheme::UseRequireAvatarWithinRadius, 8977u16 =>
    ItemMyLandTheme::UseRequireTarget, 8974u16 => ItemMyLandTheme::UseScript, 6492u16 =>
    ItemMyLandTheme::Vendorable, 5938u16 => ItemMyLandTheme::VendorAction, 11741u16 =>
    ItemMyLandTheme::VioletSomaRequired, 11740u16 => ItemMyLandTheme::YellowSomaRequired,
};
impl Attribute for ItemMyLandTheme {
    fn class() -> Class {
        Class::ItemMyLandTheme
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
        }
    }
}
impl AttributeInfo for ItemMyLandTheme {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12374u16,
            Self::AdditionalItemCount2 => 12373u16,
            Self::AdditionalItemCount3 => 12372u16,
            Self::AdditionalItemRequired1 => 11750u16,
            Self::AdditionalItemRequired2 => 11749u16,
            Self::AdditionalItemRequired3 => 11748u16,
            Self::AllowBuy => 7583u16,
            Self::AllowRent => 7572u16,
            Self::AllowSell => 7651u16,
            Self::BlackSomaRequired => 11747u16,
            Self::BlingPrice => 6588u16,
            Self::BlingSellingPrice => 6587u16,
            Self::BlueSomaRequired => 11746u16,
            Self::BonusSlotAmber => 11966u16,
            Self::BonusSlotRuby => 11967u16,
            Self::BonusSlotSapphire => 11968u16,
            Self::BuyDiscount => 7652u16,
            Self::BuyPriceBling => 7654u16,
            Self::BuyPriceGameCash => 7653u16,
            Self::Category => 7550u16,
            Self::Combos => 8899u16,
            Self::ContainerId => 5853u16,
            Self::ContentClass => 5848u16,
            Self::CraftingMapping => 12198u16,
            Self::CraftTime => 11738u16,
            Self::CreationTime => 5854u16,
            Self::CrystaEffects => 11999u16,
            Self::CrystalType => 12001u16,
            Self::CyanSomaRequired => 11745u16,
            Self::Description => 6959u16,
            Self::DestroyMethod => 6493u16,
            Self::Dialogs => 8926u16,
            Self::DisplayName => 5850u16,
            Self::EnableInGame => 6818u16,
            Self::EquipSlot => 5846u16,
            Self::ExpireBuyBack => 11614u16,
            Self::ExpireTime => 7561u16,
            Self::Freq => 5836u16,
            Self::GameCashPrice => 6586u16,
            Self::GreenSomaRequired => 11744u16,
            Self::Icon => 5835u16,
            Self::InfiniteUse => 11468u16,
            Self::InitLeftTime => 12341u16,
            Self::InventorySlotIndex => 9877u16,
            Self::IsCollectFaction => 12176u16,
            Self::IsEquiped => 5845u16,
            Self::IsFactionItem => 12156u16,
            Self::IsGemeCrystal => 12000u16,
            Self::IsHotSeller => 7427u16,
            Self::IsInGlobalShop => 7151u16,
            Self::IsInStock => 7426u16,
            Self::IsNewToShop => 7428u16,
            Self::IsQuestItem => 9914u16,
            Self::IsRecipe => 11739u16,
            Self::IsSomaSeed => 12409u16,
            Self::IsSoulBounded => 10594u16,
            Self::IsTechApproved => 9380u16,
            Self::IsTrialItem => 7752u16,
            Self::ItemCritVar => 11751u16,
            Self::ItemNormalVar => 11752u16,
            Self::LastUseTime => 9018u16,
            Self::LeftTime => 12342u16,
            Self::LootAction => 5997u16,
            Self::Lua => 10159u16,
            Self::Lvl => 6177u16,
            Self::LvlReq => 5841u16,
            Self::MaterialOverride => 5834u16,
            Self::MaxStackSize => 9899u16,
            Self::OrangeSomaRequired => 11743u16,
            Self::Power => 5837u16,
            Self::Quantity => 6437u16,
            Self::QuestTrigger => 7726u16,
            Self::Rarity => 6282u16,
            Self::RedSomaRequired => 11742u16,
            Self::RentalDurationMax => 7468u16,
            Self::RentalDurationMin => 7469u16,
            Self::RentDiscount => 7655u16,
            Self::RentPriceBling => 7657u16,
            Self::RentPriceGameCash => 7656u16,
            Self::SellPriceBling => 7650u16,
            Self::SlotId => 5838u16,
            Self::SlotMapping => 6251u16,
            Self::SomaType => 12408u16,
            Self::SoulBoundedAccountId => 12264u16,
            Self::SoulBoundedAvatarId => 10618u16,
            Self::SoulBoundedToAccount => 12253u16,
            Self::SoulBoundType => 10593u16,
            Self::StackCount => 9898u16,
            Self::StandingReq => 12175u16,
            Self::UseAction => 6022u16,
            Self::UseCoolDownTimer => 9004u16,
            Self::UseCount => 9029u16,
            Self::UseMaxCount => 9005u16,
            Self::UseRequireAvatar => 8976u16,
            Self::UseRequireAvatarWithinRadius => 8975u16,
            Self::UseRequireTarget => 8977u16,
            Self::UseScript => 8974u16,
            Self::Vendorable => 6492u16,
            Self::VendorAction => 5938u16,
            Self::VioletSomaRequired => 11741u16,
            Self::YellowSomaRequired => 11740u16,
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
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
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
        }
    }
    fn default(&self) -> &'static Value {
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
        match self {
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
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
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
                &[ParamFlag::NodeOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
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
            Self::IsInGlobalShop => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
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
            Self::IsTechApproved => &[ParamFlag::Content, ParamFlag::ExcludeFromClient],
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
            Self::VendorAction => &[ParamFlag::Persistent, ParamFlag::Content],
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
impl FromStr for ItemMyLandTheme {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ITEM_MY_LAND_THEME_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ItemMyLandTheme {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12374u16 => Ok(Self::AdditionalItemCount1),
            12373u16 => Ok(Self::AdditionalItemCount2),
            12372u16 => Ok(Self::AdditionalItemCount3),
            11750u16 => Ok(Self::AdditionalItemRequired1),
            11749u16 => Ok(Self::AdditionalItemRequired2),
            11748u16 => Ok(Self::AdditionalItemRequired3),
            7583u16 => Ok(Self::AllowBuy),
            7572u16 => Ok(Self::AllowRent),
            7651u16 => Ok(Self::AllowSell),
            11747u16 => Ok(Self::BlackSomaRequired),
            6588u16 => Ok(Self::BlingPrice),
            6587u16 => Ok(Self::BlingSellingPrice),
            11746u16 => Ok(Self::BlueSomaRequired),
            11966u16 => Ok(Self::BonusSlotAmber),
            11967u16 => Ok(Self::BonusSlotRuby),
            11968u16 => Ok(Self::BonusSlotSapphire),
            7652u16 => Ok(Self::BuyDiscount),
            7654u16 => Ok(Self::BuyPriceBling),
            7653u16 => Ok(Self::BuyPriceGameCash),
            7550u16 => Ok(Self::Category),
            8899u16 => Ok(Self::Combos),
            5853u16 => Ok(Self::ContainerId),
            5848u16 => Ok(Self::ContentClass),
            12198u16 => Ok(Self::CraftingMapping),
            11738u16 => Ok(Self::CraftTime),
            5854u16 => Ok(Self::CreationTime),
            11999u16 => Ok(Self::CrystaEffects),
            12001u16 => Ok(Self::CrystalType),
            11745u16 => Ok(Self::CyanSomaRequired),
            6959u16 => Ok(Self::Description),
            6493u16 => Ok(Self::DestroyMethod),
            8926u16 => Ok(Self::Dialogs),
            5850u16 => Ok(Self::DisplayName),
            6818u16 => Ok(Self::EnableInGame),
            5846u16 => Ok(Self::EquipSlot),
            11614u16 => Ok(Self::ExpireBuyBack),
            7561u16 => Ok(Self::ExpireTime),
            5836u16 => Ok(Self::Freq),
            6586u16 => Ok(Self::GameCashPrice),
            11744u16 => Ok(Self::GreenSomaRequired),
            5835u16 => Ok(Self::Icon),
            11468u16 => Ok(Self::InfiniteUse),
            12341u16 => Ok(Self::InitLeftTime),
            9877u16 => Ok(Self::InventorySlotIndex),
            12176u16 => Ok(Self::IsCollectFaction),
            5845u16 => Ok(Self::IsEquiped),
            12156u16 => Ok(Self::IsFactionItem),
            12000u16 => Ok(Self::IsGemeCrystal),
            7427u16 => Ok(Self::IsHotSeller),
            7151u16 => Ok(Self::IsInGlobalShop),
            7426u16 => Ok(Self::IsInStock),
            7428u16 => Ok(Self::IsNewToShop),
            9914u16 => Ok(Self::IsQuestItem),
            11739u16 => Ok(Self::IsRecipe),
            12409u16 => Ok(Self::IsSomaSeed),
            10594u16 => Ok(Self::IsSoulBounded),
            9380u16 => Ok(Self::IsTechApproved),
            7752u16 => Ok(Self::IsTrialItem),
            11751u16 => Ok(Self::ItemCritVar),
            11752u16 => Ok(Self::ItemNormalVar),
            9018u16 => Ok(Self::LastUseTime),
            12342u16 => Ok(Self::LeftTime),
            5997u16 => Ok(Self::LootAction),
            10159u16 => Ok(Self::Lua),
            6177u16 => Ok(Self::Lvl),
            5841u16 => Ok(Self::LvlReq),
            5834u16 => Ok(Self::MaterialOverride),
            9899u16 => Ok(Self::MaxStackSize),
            11743u16 => Ok(Self::OrangeSomaRequired),
            5837u16 => Ok(Self::Power),
            6437u16 => Ok(Self::Quantity),
            7726u16 => Ok(Self::QuestTrigger),
            6282u16 => Ok(Self::Rarity),
            11742u16 => Ok(Self::RedSomaRequired),
            7468u16 => Ok(Self::RentalDurationMax),
            7469u16 => Ok(Self::RentalDurationMin),
            7655u16 => Ok(Self::RentDiscount),
            7657u16 => Ok(Self::RentPriceBling),
            7656u16 => Ok(Self::RentPriceGameCash),
            7650u16 => Ok(Self::SellPriceBling),
            5838u16 => Ok(Self::SlotId),
            6251u16 => Ok(Self::SlotMapping),
            12408u16 => Ok(Self::SomaType),
            12264u16 => Ok(Self::SoulBoundedAccountId),
            10618u16 => Ok(Self::SoulBoundedAvatarId),
            12253u16 => Ok(Self::SoulBoundedToAccount),
            10593u16 => Ok(Self::SoulBoundType),
            9898u16 => Ok(Self::StackCount),
            12175u16 => Ok(Self::StandingReq),
            6022u16 => Ok(Self::UseAction),
            9004u16 => Ok(Self::UseCoolDownTimer),
            9029u16 => Ok(Self::UseCount),
            9005u16 => Ok(Self::UseMaxCount),
            8976u16 => Ok(Self::UseRequireAvatar),
            8975u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8977u16 => Ok(Self::UseRequireTarget),
            8974u16 => Ok(Self::UseScript),
            6492u16 => Ok(Self::Vendorable),
            5938u16 => Ok(Self::VendorAction),
            11741u16 => Ok(Self::VioletSomaRequired),
            11740u16 => Ok(Self::YellowSomaRequired),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
