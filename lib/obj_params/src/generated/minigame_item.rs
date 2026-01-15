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
pub enum MinigameItem {
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
    MinigameData,
    MinigameName,
}
pub(crate) static MINIGAME_ITEM_ATTRIBUTES: phf::Map<&'static str, MinigameItem> = phf_map! {
    "AdditionalItemCount1" => MinigameItem::AdditionalItemCount1, "AdditionalItemCount2"
    => MinigameItem::AdditionalItemCount2, "AdditionalItemCount3" =>
    MinigameItem::AdditionalItemCount3, "AdditionalItemRequired1" =>
    MinigameItem::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    MinigameItem::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    MinigameItem::AdditionalItemRequired3, "AllowBuy" => MinigameItem::AllowBuy,
    "AllowRent" => MinigameItem::AllowRent, "AllowSell" => MinigameItem::AllowSell,
    "BlackSomaRequired" => MinigameItem::BlackSomaRequired, "blingPrice" =>
    MinigameItem::BlingPrice, "blingSellingPrice" => MinigameItem::BlingSellingPrice,
    "BlueSomaRequired" => MinigameItem::BlueSomaRequired, "BonusSlotAmber" =>
    MinigameItem::BonusSlotAmber, "BonusSlotRuby" => MinigameItem::BonusSlotRuby,
    "BonusSlotSapphire" => MinigameItem::BonusSlotSapphire, "BuyDiscount" =>
    MinigameItem::BuyDiscount, "BuyPriceBling" => MinigameItem::BuyPriceBling,
    "BuyPriceGameCash" => MinigameItem::BuyPriceGameCash, "Category" =>
    MinigameItem::Category, "combos" => MinigameItem::Combos, "containerID" =>
    MinigameItem::ContainerId, "ContentClass" => MinigameItem::ContentClass,
    "CraftingMapping" => MinigameItem::CraftingMapping, "CraftTime" =>
    MinigameItem::CraftTime, "creationTime" => MinigameItem::CreationTime,
    "CrystaEffects" => MinigameItem::CrystaEffects, "CrystalType" =>
    MinigameItem::CrystalType, "CyanSomaRequired" => MinigameItem::CyanSomaRequired,
    "Description" => MinigameItem::Description, "DestroyMethod" =>
    MinigameItem::DestroyMethod, "Dialogs" => MinigameItem::Dialogs, "DisplayName" =>
    MinigameItem::DisplayName, "EnableInGame" => MinigameItem::EnableInGame, "equipSlot"
    => MinigameItem::EquipSlot, "expireBuyBack" => MinigameItem::ExpireBuyBack,
    "ExpireTime" => MinigameItem::ExpireTime, "Freq" => MinigameItem::Freq,
    "gameCashPrice" => MinigameItem::GameCashPrice, "GreenSomaRequired" =>
    MinigameItem::GreenSomaRequired, "Icon" => MinigameItem::Icon, "InfiniteUse" =>
    MinigameItem::InfiniteUse, "InitLeftTime" => MinigameItem::InitLeftTime,
    "inventorySlotIndex" => MinigameItem::InventorySlotIndex, "isCollectFaction" =>
    MinigameItem::IsCollectFaction, "isEquiped" => MinigameItem::IsEquiped,
    "isFactionItem" => MinigameItem::IsFactionItem, "isGemeCrystal" =>
    MinigameItem::IsGemeCrystal, "IsHotSeller" => MinigameItem::IsHotSeller,
    "isInGlobalShop" => MinigameItem::IsInGlobalShop, "IsInStock" =>
    MinigameItem::IsInStock, "IsNewToShop" => MinigameItem::IsNewToShop, "isQuestItem" =>
    MinigameItem::IsQuestItem, "IsRecipe" => MinigameItem::IsRecipe, "IsSomaSeed" =>
    MinigameItem::IsSomaSeed, "IsSoulBounded" => MinigameItem::IsSoulBounded,
    "isTechApproved" => MinigameItem::IsTechApproved, "isTrialItem" =>
    MinigameItem::IsTrialItem, "ItemCritVar" => MinigameItem::ItemCritVar,
    "ItemNormalVar" => MinigameItem::ItemNormalVar, "LastUseTime" =>
    MinigameItem::LastUseTime, "LeftTime" => MinigameItem::LeftTime, "lootAction" =>
    MinigameItem::LootAction, "Lua" => MinigameItem::Lua, "lvl" => MinigameItem::Lvl,
    "lvlReq" => MinigameItem::LvlReq, "MaterialOverride" =>
    MinigameItem::MaterialOverride, "maxStackSize" => MinigameItem::MaxStackSize,
    "OrangeSomaRequired" => MinigameItem::OrangeSomaRequired, "Power" =>
    MinigameItem::Power, "quantity" => MinigameItem::Quantity, "QuestTrigger" =>
    MinigameItem::QuestTrigger, "rarity" => MinigameItem::Rarity, "RedSomaRequired" =>
    MinigameItem::RedSomaRequired, "RentalDurationMax" =>
    MinigameItem::RentalDurationMax, "RentalDurationMin" =>
    MinigameItem::RentalDurationMin, "RentDiscount" => MinigameItem::RentDiscount,
    "RentPriceBling" => MinigameItem::RentPriceBling, "RentPriceGameCash" =>
    MinigameItem::RentPriceGameCash, "SellPriceBling" => MinigameItem::SellPriceBling,
    "slotID" => MinigameItem::SlotId, "SlotMapping" => MinigameItem::SlotMapping,
    "SomaType" => MinigameItem::SomaType, "SoulBoundedAccountId" =>
    MinigameItem::SoulBoundedAccountId, "SoulBoundedAvatarId" =>
    MinigameItem::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    MinigameItem::SoulBoundedToAccount, "SoulBoundType" => MinigameItem::SoulBoundType,
    "stackCount" => MinigameItem::StackCount, "standingReq" => MinigameItem::StandingReq,
    "useAction" => MinigameItem::UseAction, "UseCoolDownTimer" =>
    MinigameItem::UseCoolDownTimer, "UseCount" => MinigameItem::UseCount, "UseMaxCount"
    => MinigameItem::UseMaxCount, "UseRequireAvatar" => MinigameItem::UseRequireAvatar,
    "UseRequireAvatarWithinRadius" => MinigameItem::UseRequireAvatarWithinRadius,
    "UseRequireTarget" => MinigameItem::UseRequireTarget, "UseScript" =>
    MinigameItem::UseScript, "Vendorable" => MinigameItem::Vendorable, "vendorAction" =>
    MinigameItem::VendorAction, "VioletSomaRequired" => MinigameItem::VioletSomaRequired,
    "YellowSomaRequired" => MinigameItem::YellowSomaRequired, "minigameData" =>
    MinigameItem::MinigameData, "minigameName" => MinigameItem::MinigameName,
};
pub(crate) static MINIGAME_ITEM_ATTRIBUTES_ID: phf::Map<u16, MinigameItem> = phf_map! {
    12371u16 => MinigameItem::AdditionalItemCount1, 12370u16 =>
    MinigameItem::AdditionalItemCount2, 12369u16 => MinigameItem::AdditionalItemCount3,
    11735u16 => MinigameItem::AdditionalItemRequired1, 11734u16 =>
    MinigameItem::AdditionalItemRequired2, 11733u16 =>
    MinigameItem::AdditionalItemRequired3, 7584u16 => MinigameItem::AllowBuy, 7573u16 =>
    MinigameItem::AllowRent, 7659u16 => MinigameItem::AllowSell, 11732u16 =>
    MinigameItem::BlackSomaRequired, 6585u16 => MinigameItem::BlingPrice, 6584u16 =>
    MinigameItem::BlingSellingPrice, 11731u16 => MinigameItem::BlueSomaRequired, 11963u16
    => MinigameItem::BonusSlotAmber, 11964u16 => MinigameItem::BonusSlotRuby, 11965u16 =>
    MinigameItem::BonusSlotSapphire, 7660u16 => MinigameItem::BuyDiscount, 7662u16 =>
    MinigameItem::BuyPriceBling, 7661u16 => MinigameItem::BuyPriceGameCash, 7551u16 =>
    MinigameItem::Category, 8900u16 => MinigameItem::Combos, 4242u16 =>
    MinigameItem::ContainerId, 4237u16 => MinigameItem::ContentClass, 12197u16 =>
    MinigameItem::CraftingMapping, 11723u16 => MinigameItem::CraftTime, 4243u16 =>
    MinigameItem::CreationTime, 11996u16 => MinigameItem::CrystaEffects, 11998u16 =>
    MinigameItem::CrystalType, 11730u16 => MinigameItem::CyanSomaRequired, 6956u16 =>
    MinigameItem::Description, 6491u16 => MinigameItem::DestroyMethod, 8927u16 =>
    MinigameItem::Dialogs, 4239u16 => MinigameItem::DisplayName, 6817u16 =>
    MinigameItem::EnableInGame, 4235u16 => MinigameItem::EquipSlot, 11613u16 =>
    MinigameItem::ExpireBuyBack, 7562u16 => MinigameItem::ExpireTime, 4225u16 =>
    MinigameItem::Freq, 6583u16 => MinigameItem::GameCashPrice, 11729u16 =>
    MinigameItem::GreenSomaRequired, 4348u16 => MinigameItem::Icon, 11467u16 =>
    MinigameItem::InfiniteUse, 12339u16 => MinigameItem::InitLeftTime, 9878u16 =>
    MinigameItem::InventorySlotIndex, 12174u16 => MinigameItem::IsCollectFaction, 4234u16
    => MinigameItem::IsEquiped, 12155u16 => MinigameItem::IsFactionItem, 11997u16 =>
    MinigameItem::IsGemeCrystal, 7387u16 => MinigameItem::IsHotSeller, 7148u16 =>
    MinigameItem::IsInGlobalShop, 7386u16 => MinigameItem::IsInStock, 7388u16 =>
    MinigameItem::IsNewToShop, 9915u16 => MinigameItem::IsQuestItem, 11724u16 =>
    MinigameItem::IsRecipe, 12407u16 => MinigameItem::IsSomaSeed, 10596u16 =>
    MinigameItem::IsSoulBounded, 9381u16 => MinigameItem::IsTechApproved, 7753u16 =>
    MinigameItem::IsTrialItem, 11736u16 => MinigameItem::ItemCritVar, 11737u16 =>
    MinigameItem::ItemNormalVar, 9019u16 => MinigameItem::LastUseTime, 12340u16 =>
    MinigameItem::LeftTime, 5996u16 => MinigameItem::LootAction, 10160u16 =>
    MinigameItem::Lua, 6176u16 => MinigameItem::Lvl, 4230u16 => MinigameItem::LvlReq,
    4727u16 => MinigameItem::MaterialOverride, 9901u16 => MinigameItem::MaxStackSize,
    11728u16 => MinigameItem::OrangeSomaRequired, 4226u16 => MinigameItem::Power, 6436u16
    => MinigameItem::Quantity, 7728u16 => MinigameItem::QuestTrigger, 6281u16 =>
    MinigameItem::Rarity, 11727u16 => MinigameItem::RedSomaRequired, 7460u16 =>
    MinigameItem::RentalDurationMax, 7461u16 => MinigameItem::RentalDurationMin, 7663u16
    => MinigameItem::RentDiscount, 7665u16 => MinigameItem::RentPriceBling, 7664u16 =>
    MinigameItem::RentPriceGameCash, 7658u16 => MinigameItem::SellPriceBling, 4227u16 =>
    MinigameItem::SlotId, 6250u16 => MinigameItem::SlotMapping, 12406u16 =>
    MinigameItem::SomaType, 12263u16 => MinigameItem::SoulBoundedAccountId, 10619u16 =>
    MinigameItem::SoulBoundedAvatarId, 12252u16 => MinigameItem::SoulBoundedToAccount,
    10595u16 => MinigameItem::SoulBoundType, 9900u16 => MinigameItem::StackCount,
    12173u16 => MinigameItem::StandingReq, 6021u16 => MinigameItem::UseAction, 9006u16 =>
    MinigameItem::UseCoolDownTimer, 9030u16 => MinigameItem::UseCount, 9007u16 =>
    MinigameItem::UseMaxCount, 8980u16 => MinigameItem::UseRequireAvatar, 8979u16 =>
    MinigameItem::UseRequireAvatarWithinRadius, 8981u16 =>
    MinigameItem::UseRequireTarget, 8978u16 => MinigameItem::UseScript, 6490u16 =>
    MinigameItem::Vendorable, 5937u16 => MinigameItem::VendorAction, 11726u16 =>
    MinigameItem::VioletSomaRequired, 11725u16 => MinigameItem::YellowSomaRequired,
    4223u16 => MinigameItem::MinigameData, 4224u16 => MinigameItem::MinigameName,
};
impl Attribute for MinigameItem {
    fn class() -> Class {
        Class::MinigameItem
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
            Self::MinigameData => &Self::MinigameData,
            Self::MinigameName => &Self::MinigameName,
        }
    }
}
impl AttributeInfo for MinigameItem {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12371u16,
            Self::AdditionalItemCount2 => 12370u16,
            Self::AdditionalItemCount3 => 12369u16,
            Self::AdditionalItemRequired1 => 11735u16,
            Self::AdditionalItemRequired2 => 11734u16,
            Self::AdditionalItemRequired3 => 11733u16,
            Self::AllowBuy => 7584u16,
            Self::AllowRent => 7573u16,
            Self::AllowSell => 7659u16,
            Self::BlackSomaRequired => 11732u16,
            Self::BlingPrice => 6585u16,
            Self::BlingSellingPrice => 6584u16,
            Self::BlueSomaRequired => 11731u16,
            Self::BonusSlotAmber => 11963u16,
            Self::BonusSlotRuby => 11964u16,
            Self::BonusSlotSapphire => 11965u16,
            Self::BuyDiscount => 7660u16,
            Self::BuyPriceBling => 7662u16,
            Self::BuyPriceGameCash => 7661u16,
            Self::Category => 7551u16,
            Self::Combos => 8900u16,
            Self::ContainerId => 4242u16,
            Self::ContentClass => 4237u16,
            Self::CraftingMapping => 12197u16,
            Self::CraftTime => 11723u16,
            Self::CreationTime => 4243u16,
            Self::CrystaEffects => 11996u16,
            Self::CrystalType => 11998u16,
            Self::CyanSomaRequired => 11730u16,
            Self::Description => 6956u16,
            Self::DestroyMethod => 6491u16,
            Self::Dialogs => 8927u16,
            Self::DisplayName => 4239u16,
            Self::EnableInGame => 6817u16,
            Self::EquipSlot => 4235u16,
            Self::ExpireBuyBack => 11613u16,
            Self::ExpireTime => 7562u16,
            Self::Freq => 4225u16,
            Self::GameCashPrice => 6583u16,
            Self::GreenSomaRequired => 11729u16,
            Self::Icon => 4348u16,
            Self::InfiniteUse => 11467u16,
            Self::InitLeftTime => 12339u16,
            Self::InventorySlotIndex => 9878u16,
            Self::IsCollectFaction => 12174u16,
            Self::IsEquiped => 4234u16,
            Self::IsFactionItem => 12155u16,
            Self::IsGemeCrystal => 11997u16,
            Self::IsHotSeller => 7387u16,
            Self::IsInGlobalShop => 7148u16,
            Self::IsInStock => 7386u16,
            Self::IsNewToShop => 7388u16,
            Self::IsQuestItem => 9915u16,
            Self::IsRecipe => 11724u16,
            Self::IsSomaSeed => 12407u16,
            Self::IsSoulBounded => 10596u16,
            Self::IsTechApproved => 9381u16,
            Self::IsTrialItem => 7753u16,
            Self::ItemCritVar => 11736u16,
            Self::ItemNormalVar => 11737u16,
            Self::LastUseTime => 9019u16,
            Self::LeftTime => 12340u16,
            Self::LootAction => 5996u16,
            Self::Lua => 10160u16,
            Self::Lvl => 6176u16,
            Self::LvlReq => 4230u16,
            Self::MaterialOverride => 4727u16,
            Self::MaxStackSize => 9901u16,
            Self::OrangeSomaRequired => 11728u16,
            Self::Power => 4226u16,
            Self::Quantity => 6436u16,
            Self::QuestTrigger => 7728u16,
            Self::Rarity => 6281u16,
            Self::RedSomaRequired => 11727u16,
            Self::RentalDurationMax => 7460u16,
            Self::RentalDurationMin => 7461u16,
            Self::RentDiscount => 7663u16,
            Self::RentPriceBling => 7665u16,
            Self::RentPriceGameCash => 7664u16,
            Self::SellPriceBling => 7658u16,
            Self::SlotId => 4227u16,
            Self::SlotMapping => 6250u16,
            Self::SomaType => 12406u16,
            Self::SoulBoundedAccountId => 12263u16,
            Self::SoulBoundedAvatarId => 10619u16,
            Self::SoulBoundedToAccount => 12252u16,
            Self::SoulBoundType => 10595u16,
            Self::StackCount => 9900u16,
            Self::StandingReq => 12173u16,
            Self::UseAction => 6021u16,
            Self::UseCoolDownTimer => 9006u16,
            Self::UseCount => 9030u16,
            Self::UseMaxCount => 9007u16,
            Self::UseRequireAvatar => 8980u16,
            Self::UseRequireAvatarWithinRadius => 8979u16,
            Self::UseRequireTarget => 8981u16,
            Self::UseScript => 8978u16,
            Self::Vendorable => 6490u16,
            Self::VendorAction => 5937u16,
            Self::VioletSomaRequired => 11726u16,
            Self::YellowSomaRequired => 11725u16,
            Self::MinigameData => 4223u16,
            Self::MinigameName => 4224u16,
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
            Self::MinigameData => "minigameData",
            Self::MinigameName => "minigameName",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::MinigameData => ParamType::JsonValue,
            Self::MinigameName => ParamType::String,
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
        static MINIGAME_DATA: Lazy<Value> = Lazy::new(|| Value::JsonValue(
            JsonValue::default(),
        ));
        static MINIGAME_NAME: Lazy<Value> = Lazy::new(|| Value::String(
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
            Self::MinigameData => &MINIGAME_DATA,
            Self::MinigameName => &MINIGAME_NAME,
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
            Self::MinigameData => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::MinigameName => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
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
impl FromStr for MinigameItem {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MINIGAME_ITEM_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for MinigameItem {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12371u16 => Ok(Self::AdditionalItemCount1),
            12370u16 => Ok(Self::AdditionalItemCount2),
            12369u16 => Ok(Self::AdditionalItemCount3),
            11735u16 => Ok(Self::AdditionalItemRequired1),
            11734u16 => Ok(Self::AdditionalItemRequired2),
            11733u16 => Ok(Self::AdditionalItemRequired3),
            7584u16 => Ok(Self::AllowBuy),
            7573u16 => Ok(Self::AllowRent),
            7659u16 => Ok(Self::AllowSell),
            11732u16 => Ok(Self::BlackSomaRequired),
            6585u16 => Ok(Self::BlingPrice),
            6584u16 => Ok(Self::BlingSellingPrice),
            11731u16 => Ok(Self::BlueSomaRequired),
            11963u16 => Ok(Self::BonusSlotAmber),
            11964u16 => Ok(Self::BonusSlotRuby),
            11965u16 => Ok(Self::BonusSlotSapphire),
            7660u16 => Ok(Self::BuyDiscount),
            7662u16 => Ok(Self::BuyPriceBling),
            7661u16 => Ok(Self::BuyPriceGameCash),
            7551u16 => Ok(Self::Category),
            8900u16 => Ok(Self::Combos),
            4242u16 => Ok(Self::ContainerId),
            4237u16 => Ok(Self::ContentClass),
            12197u16 => Ok(Self::CraftingMapping),
            11723u16 => Ok(Self::CraftTime),
            4243u16 => Ok(Self::CreationTime),
            11996u16 => Ok(Self::CrystaEffects),
            11998u16 => Ok(Self::CrystalType),
            11730u16 => Ok(Self::CyanSomaRequired),
            6956u16 => Ok(Self::Description),
            6491u16 => Ok(Self::DestroyMethod),
            8927u16 => Ok(Self::Dialogs),
            4239u16 => Ok(Self::DisplayName),
            6817u16 => Ok(Self::EnableInGame),
            4235u16 => Ok(Self::EquipSlot),
            11613u16 => Ok(Self::ExpireBuyBack),
            7562u16 => Ok(Self::ExpireTime),
            4225u16 => Ok(Self::Freq),
            6583u16 => Ok(Self::GameCashPrice),
            11729u16 => Ok(Self::GreenSomaRequired),
            4348u16 => Ok(Self::Icon),
            11467u16 => Ok(Self::InfiniteUse),
            12339u16 => Ok(Self::InitLeftTime),
            9878u16 => Ok(Self::InventorySlotIndex),
            12174u16 => Ok(Self::IsCollectFaction),
            4234u16 => Ok(Self::IsEquiped),
            12155u16 => Ok(Self::IsFactionItem),
            11997u16 => Ok(Self::IsGemeCrystal),
            7387u16 => Ok(Self::IsHotSeller),
            7148u16 => Ok(Self::IsInGlobalShop),
            7386u16 => Ok(Self::IsInStock),
            7388u16 => Ok(Self::IsNewToShop),
            9915u16 => Ok(Self::IsQuestItem),
            11724u16 => Ok(Self::IsRecipe),
            12407u16 => Ok(Self::IsSomaSeed),
            10596u16 => Ok(Self::IsSoulBounded),
            9381u16 => Ok(Self::IsTechApproved),
            7753u16 => Ok(Self::IsTrialItem),
            11736u16 => Ok(Self::ItemCritVar),
            11737u16 => Ok(Self::ItemNormalVar),
            9019u16 => Ok(Self::LastUseTime),
            12340u16 => Ok(Self::LeftTime),
            5996u16 => Ok(Self::LootAction),
            10160u16 => Ok(Self::Lua),
            6176u16 => Ok(Self::Lvl),
            4230u16 => Ok(Self::LvlReq),
            4727u16 => Ok(Self::MaterialOverride),
            9901u16 => Ok(Self::MaxStackSize),
            11728u16 => Ok(Self::OrangeSomaRequired),
            4226u16 => Ok(Self::Power),
            6436u16 => Ok(Self::Quantity),
            7728u16 => Ok(Self::QuestTrigger),
            6281u16 => Ok(Self::Rarity),
            11727u16 => Ok(Self::RedSomaRequired),
            7460u16 => Ok(Self::RentalDurationMax),
            7461u16 => Ok(Self::RentalDurationMin),
            7663u16 => Ok(Self::RentDiscount),
            7665u16 => Ok(Self::RentPriceBling),
            7664u16 => Ok(Self::RentPriceGameCash),
            7658u16 => Ok(Self::SellPriceBling),
            4227u16 => Ok(Self::SlotId),
            6250u16 => Ok(Self::SlotMapping),
            12406u16 => Ok(Self::SomaType),
            12263u16 => Ok(Self::SoulBoundedAccountId),
            10619u16 => Ok(Self::SoulBoundedAvatarId),
            12252u16 => Ok(Self::SoulBoundedToAccount),
            10595u16 => Ok(Self::SoulBoundType),
            9900u16 => Ok(Self::StackCount),
            12173u16 => Ok(Self::StandingReq),
            6021u16 => Ok(Self::UseAction),
            9006u16 => Ok(Self::UseCoolDownTimer),
            9030u16 => Ok(Self::UseCount),
            9007u16 => Ok(Self::UseMaxCount),
            8980u16 => Ok(Self::UseRequireAvatar),
            8979u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8981u16 => Ok(Self::UseRequireTarget),
            8978u16 => Ok(Self::UseScript),
            6490u16 => Ok(Self::Vendorable),
            5937u16 => Ok(Self::VendorAction),
            11726u16 => Ok(Self::VioletSomaRequired),
            11725u16 => Ok(Self::YellowSomaRequired),
            4223u16 => Ok(Self::MinigameData),
            4224u16 => Ok(Self::MinigameName),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
