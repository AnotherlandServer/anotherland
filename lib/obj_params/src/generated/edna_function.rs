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
pub enum EdnaFunction {
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
    DefaultAttack,
    DefaultSkills,
    Faction,
    HeavyAttack,
    IsActive,
    IsConsumable,
    Level,
    Name,
    PlayrateFactor,
    SpecialAttack,
    StickyTargets,
    Ue3ClassId,
    WeaponDamageVariance,
    WeaponDelay,
    WeaponType,
    WepAttSpeed,
    WepMaxDmg,
    WepMinDmg,
}
pub(crate) static EDNA_FUNCTION_ATTRIBUTES: phf::Map<&'static str, EdnaFunction> = phf_map! {
    "AdditionalItemCount1" => EdnaFunction::AdditionalItemCount1, "AdditionalItemCount2"
    => EdnaFunction::AdditionalItemCount2, "AdditionalItemCount3" =>
    EdnaFunction::AdditionalItemCount3, "AdditionalItemRequired1" =>
    EdnaFunction::AdditionalItemRequired1, "AdditionalItemRequired2" =>
    EdnaFunction::AdditionalItemRequired2, "AdditionalItemRequired3" =>
    EdnaFunction::AdditionalItemRequired3, "AllowBuy" => EdnaFunction::AllowBuy,
    "AllowRent" => EdnaFunction::AllowRent, "AllowSell" => EdnaFunction::AllowSell,
    "BlackSomaRequired" => EdnaFunction::BlackSomaRequired, "blingPrice" =>
    EdnaFunction::BlingPrice, "blingSellingPrice" => EdnaFunction::BlingSellingPrice,
    "BlueSomaRequired" => EdnaFunction::BlueSomaRequired, "BonusSlotAmber" =>
    EdnaFunction::BonusSlotAmber, "BonusSlotRuby" => EdnaFunction::BonusSlotRuby,
    "BonusSlotSapphire" => EdnaFunction::BonusSlotSapphire, "BuyDiscount" =>
    EdnaFunction::BuyDiscount, "BuyPriceBling" => EdnaFunction::BuyPriceBling,
    "BuyPriceGameCash" => EdnaFunction::BuyPriceGameCash, "Category" =>
    EdnaFunction::Category, "combos" => EdnaFunction::Combos, "containerID" =>
    EdnaFunction::ContainerId, "ContentClass" => EdnaFunction::ContentClass,
    "CraftingMapping" => EdnaFunction::CraftingMapping, "CraftTime" =>
    EdnaFunction::CraftTime, "creationTime" => EdnaFunction::CreationTime,
    "CrystaEffects" => EdnaFunction::CrystaEffects, "CrystalType" =>
    EdnaFunction::CrystalType, "CyanSomaRequired" => EdnaFunction::CyanSomaRequired,
    "Description" => EdnaFunction::Description, "DestroyMethod" =>
    EdnaFunction::DestroyMethod, "Dialogs" => EdnaFunction::Dialogs, "DisplayName" =>
    EdnaFunction::DisplayName, "EnableInGame" => EdnaFunction::EnableInGame, "equipSlot"
    => EdnaFunction::EquipSlot, "expireBuyBack" => EdnaFunction::ExpireBuyBack,
    "ExpireTime" => EdnaFunction::ExpireTime, "Freq" => EdnaFunction::Freq,
    "gameCashPrice" => EdnaFunction::GameCashPrice, "GreenSomaRequired" =>
    EdnaFunction::GreenSomaRequired, "Icon" => EdnaFunction::Icon, "InfiniteUse" =>
    EdnaFunction::InfiniteUse, "InitLeftTime" => EdnaFunction::InitLeftTime,
    "inventorySlotIndex" => EdnaFunction::InventorySlotIndex, "isCollectFaction" =>
    EdnaFunction::IsCollectFaction, "isEquiped" => EdnaFunction::IsEquiped,
    "isFactionItem" => EdnaFunction::IsFactionItem, "isGemeCrystal" =>
    EdnaFunction::IsGemeCrystal, "IsHotSeller" => EdnaFunction::IsHotSeller,
    "isInGlobalShop" => EdnaFunction::IsInGlobalShop, "IsInStock" =>
    EdnaFunction::IsInStock, "IsNewToShop" => EdnaFunction::IsNewToShop, "isQuestItem" =>
    EdnaFunction::IsQuestItem, "IsRecipe" => EdnaFunction::IsRecipe, "IsSomaSeed" =>
    EdnaFunction::IsSomaSeed, "IsSoulBounded" => EdnaFunction::IsSoulBounded,
    "isTechApproved" => EdnaFunction::IsTechApproved, "isTrialItem" =>
    EdnaFunction::IsTrialItem, "ItemCritVar" => EdnaFunction::ItemCritVar,
    "ItemNormalVar" => EdnaFunction::ItemNormalVar, "LastUseTime" =>
    EdnaFunction::LastUseTime, "LeftTime" => EdnaFunction::LeftTime, "lootAction" =>
    EdnaFunction::LootAction, "Lua" => EdnaFunction::Lua, "lvl" => EdnaFunction::Lvl,
    "lvlReq" => EdnaFunction::LvlReq, "MaterialOverride" =>
    EdnaFunction::MaterialOverride, "maxStackSize" => EdnaFunction::MaxStackSize,
    "OrangeSomaRequired" => EdnaFunction::OrangeSomaRequired, "Power" =>
    EdnaFunction::Power, "quantity" => EdnaFunction::Quantity, "QuestTrigger" =>
    EdnaFunction::QuestTrigger, "rarity" => EdnaFunction::Rarity, "RedSomaRequired" =>
    EdnaFunction::RedSomaRequired, "RentalDurationMax" =>
    EdnaFunction::RentalDurationMax, "RentalDurationMin" =>
    EdnaFunction::RentalDurationMin, "RentDiscount" => EdnaFunction::RentDiscount,
    "RentPriceBling" => EdnaFunction::RentPriceBling, "RentPriceGameCash" =>
    EdnaFunction::RentPriceGameCash, "SellPriceBling" => EdnaFunction::SellPriceBling,
    "slotID" => EdnaFunction::SlotId, "SlotMapping" => EdnaFunction::SlotMapping,
    "SomaType" => EdnaFunction::SomaType, "SoulBoundedAccountId" =>
    EdnaFunction::SoulBoundedAccountId, "SoulBoundedAvatarId" =>
    EdnaFunction::SoulBoundedAvatarId, "SoulBoundedToAccount" =>
    EdnaFunction::SoulBoundedToAccount, "SoulBoundType" => EdnaFunction::SoulBoundType,
    "stackCount" => EdnaFunction::StackCount, "standingReq" => EdnaFunction::StandingReq,
    "useAction" => EdnaFunction::UseAction, "UseCoolDownTimer" =>
    EdnaFunction::UseCoolDownTimer, "UseCount" => EdnaFunction::UseCount, "UseMaxCount"
    => EdnaFunction::UseMaxCount, "UseRequireAvatar" => EdnaFunction::UseRequireAvatar,
    "UseRequireAvatarWithinRadius" => EdnaFunction::UseRequireAvatarWithinRadius,
    "UseRequireTarget" => EdnaFunction::UseRequireTarget, "UseScript" =>
    EdnaFunction::UseScript, "Vendorable" => EdnaFunction::Vendorable, "vendorAction" =>
    EdnaFunction::VendorAction, "VioletSomaRequired" => EdnaFunction::VioletSomaRequired,
    "YellowSomaRequired" => EdnaFunction::YellowSomaRequired, "abilities" =>
    EdnaFunction::Abilities, "abilityInstanceData" => EdnaFunction::AbilityInstanceData,
    "Agility" => EdnaFunction::Agility, "Armor" => EdnaFunction::Armor,
    "AttackPowerRating" => EdnaFunction::AttackPowerRating, "attributeOp1" =>
    EdnaFunction::AttributeOp1, "attributeOp2" => EdnaFunction::AttributeOp2,
    "attributeOp3" => EdnaFunction::AttributeOp3, "attributeOp4" =>
    EdnaFunction::AttributeOp4, "attributeType1" => EdnaFunction::AttributeType1,
    "attributeType2" => EdnaFunction::AttributeType2, "attributeType3" =>
    EdnaFunction::AttributeType3, "attributeType4" => EdnaFunction::AttributeType4,
    "attributeWeight1" => EdnaFunction::AttributeWeight1, "attributeWeight2" =>
    EdnaFunction::AttributeWeight2, "attributeWeight3" => EdnaFunction::AttributeWeight3,
    "attributeWeight4" => EdnaFunction::AttributeWeight4, "autoAttributeType1" =>
    EdnaFunction::AutoAttributeType1, "autoAttributeType2" =>
    EdnaFunction::AutoAttributeType2, "autoAttributeType3" =>
    EdnaFunction::AutoAttributeType3, "autoAttributeType4" =>
    EdnaFunction::AutoAttributeType4, "autoAttributeType5" =>
    EdnaFunction::AutoAttributeType5, "autoAttributeType6" =>
    EdnaFunction::AutoAttributeType6, "autoAttributeValue1" =>
    EdnaFunction::AutoAttributeValue1, "autoAttributeValue2" =>
    EdnaFunction::AutoAttributeValue2, "autoAttributeValue3" =>
    EdnaFunction::AutoAttributeValue3, "autoAttributeValue4" =>
    EdnaFunction::AutoAttributeValue4, "autoAttributeValue5" =>
    EdnaFunction::AutoAttributeValue5, "autoAttributeValue6" =>
    EdnaFunction::AutoAttributeValue6, "availableSockets" =>
    EdnaFunction::AvailableSockets, "BlockRating" => EdnaFunction::BlockRating,
    "ClanName" => EdnaFunction::ClanName, "combatStyle" => EdnaFunction::CombatStyle,
    "CritDamageRating" => EdnaFunction::CritDamageRating, "CritHitRating" =>
    EdnaFunction::CritHitRating, "disguise" => EdnaFunction::Disguise,
    "DisplayName_Color" => EdnaFunction::DisplayNameColor, "DisplayName_Number" =>
    EdnaFunction::DisplayNameNumber, "DisplayName_Rarity" =>
    EdnaFunction::DisplayNameRarity, "DisplayName_Slot" => EdnaFunction::DisplayNameSlot,
    "DisplayName_Stat" => EdnaFunction::DisplayNameStat, "DodgeRating" =>
    EdnaFunction::DodgeRating, "durability" => EdnaFunction::Durability,
    "durabilityCurrent" => EdnaFunction::DurabilityCurrent, "Focus" =>
    EdnaFunction::Focus, "gender" => EdnaFunction::Gender, "HeavyRating" =>
    EdnaFunction::HeavyRating, "HitRating" => EdnaFunction::HitRating, "isBanked" =>
    EdnaFunction::IsBanked, "isExpired" => EdnaFunction::IsExpired, "isSKU" =>
    EdnaFunction::IsSku, "IsTemplate" => EdnaFunction::IsTemplate,
    "itemActionGenericParam" => EdnaFunction::ItemActionGenericParam, "levelDropVariance"
    => EdnaFunction::LevelDropVariance, "MaxUse" => EdnaFunction::MaxUse, "NoteCaption"
    => EdnaFunction::NoteCaption, "NoteCaptionValue" => EdnaFunction::NoteCaptionValue,
    "otherClientInterests" => EdnaFunction::OtherClientInterests, "ParryRating" =>
    EdnaFunction::ParryRating, "PeneRating" => EdnaFunction::PeneRating, "Prefix" =>
    EdnaFunction::Prefix, "QuickbarItem" => EdnaFunction::QuickbarItem, "repairCost" =>
    EdnaFunction::RepairCost, "schematic_CostToCreateItem" =>
    EdnaFunction::SchematicCostToCreateItem, "searchKeywords" =>
    EdnaFunction::SearchKeywords, "setBonuses" => EdnaFunction::SetBonuses,
    "SignOfAvatars" => EdnaFunction::SignOfAvatars, "SKUID" => EdnaFunction::Skuid,
    "socketLockedStatus" => EdnaFunction::SocketLockedStatus, "socketOccupancyStatus" =>
    EdnaFunction::SocketOccupancyStatus, "socketUpgradeLevel" =>
    EdnaFunction::SocketUpgradeLevel, "SpecialRating" => EdnaFunction::SpecialRating,
    "Stamina" => EdnaFunction::Stamina, "Strength" => EdnaFunction::Strength, "Suffix" =>
    EdnaFunction::Suffix, "templateType" => EdnaFunction::TemplateType, "templateVersion"
    => EdnaFunction::TemplateVersion, "timeStamp" => EdnaFunction::TimeStamp, "AddBuff"
    => EdnaFunction::AddBuff, "considerForLootTables" =>
    EdnaFunction::ConsiderForLootTables, "cooldownDuration" =>
    EdnaFunction::CooldownDuration, "DefaultAttack" => EdnaFunction::DefaultAttack,
    "DefaultSkills" => EdnaFunction::DefaultSkills, "Faction" => EdnaFunction::Faction,
    "HeavyAttack" => EdnaFunction::HeavyAttack, "isActive" => EdnaFunction::IsActive,
    "IsConsumable" => EdnaFunction::IsConsumable, "level" => EdnaFunction::Level, "name"
    => EdnaFunction::Name, "playrateFactor" => EdnaFunction::PlayrateFactor,
    "SpecialAttack" => EdnaFunction::SpecialAttack, "StickyTargets" =>
    EdnaFunction::StickyTargets, "UE3ClassID" => EdnaFunction::Ue3ClassId,
    "weaponDamageVariance" => EdnaFunction::WeaponDamageVariance, "weaponDelay" =>
    EdnaFunction::WeaponDelay, "weaponType" => EdnaFunction::WeaponType, "WepAttSpeed" =>
    EdnaFunction::WepAttSpeed, "WepMaxDmg" => EdnaFunction::WepMaxDmg, "WepMinDmg" =>
    EdnaFunction::WepMinDmg,
};
pub(crate) static EDNA_FUNCTION_ATTRIBUTES_ID: phf::Map<u16, EdnaFunction> = phf_map! {
    12365u16 => EdnaFunction::AdditionalItemCount1, 12364u16 =>
    EdnaFunction::AdditionalItemCount2, 12363u16 => EdnaFunction::AdditionalItemCount3,
    11705u16 => EdnaFunction::AdditionalItemRequired1, 11704u16 =>
    EdnaFunction::AdditionalItemRequired2, 11703u16 =>
    EdnaFunction::AdditionalItemRequired3, 7579u16 => EdnaFunction::AllowBuy, 7568u16 =>
    EdnaFunction::AllowRent, 7619u16 => EdnaFunction::AllowSell, 11702u16 =>
    EdnaFunction::BlackSomaRequired, 6579u16 => EdnaFunction::BlingPrice, 6578u16 =>
    EdnaFunction::BlingSellingPrice, 11701u16 => EdnaFunction::BlueSomaRequired, 11957u16
    => EdnaFunction::BonusSlotAmber, 11958u16 => EdnaFunction::BonusSlotRuby, 11959u16 =>
    EdnaFunction::BonusSlotSapphire, 7620u16 => EdnaFunction::BuyDiscount, 7622u16 =>
    EdnaFunction::BuyPriceBling, 7621u16 => EdnaFunction::BuyPriceGameCash, 7546u16 =>
    EdnaFunction::Category, 8895u16 => EdnaFunction::Combos, 708u16 =>
    EdnaFunction::ContainerId, 747u16 => EdnaFunction::ContentClass, 12195u16 =>
    EdnaFunction::CraftingMapping, 11693u16 => EdnaFunction::CraftTime, 707u16 =>
    EdnaFunction::CreationTime, 11990u16 => EdnaFunction::CrystaEffects, 11992u16 =>
    EdnaFunction::CrystalType, 11700u16 => EdnaFunction::CyanSomaRequired, 6954u16 =>
    EdnaFunction::Description, 6487u16 => EdnaFunction::DestroyMethod, 8922u16 =>
    EdnaFunction::Dialogs, 749u16 => EdnaFunction::DisplayName, 6815u16 =>
    EdnaFunction::EnableInGame, 745u16 => EdnaFunction::EquipSlot, 11611u16 =>
    EdnaFunction::ExpireBuyBack, 7557u16 => EdnaFunction::ExpireTime, 734u16 =>
    EdnaFunction::Freq, 6577u16 => EdnaFunction::GameCashPrice, 11699u16 =>
    EdnaFunction::GreenSomaRequired, 4346u16 => EdnaFunction::Icon, 11465u16 =>
    EdnaFunction::InfiniteUse, 12335u16 => EdnaFunction::InitLeftTime, 9873u16 =>
    EdnaFunction::InventorySlotIndex, 12170u16 => EdnaFunction::IsCollectFaction, 743u16
    => EdnaFunction::IsEquiped, 12153u16 => EdnaFunction::IsFactionItem, 11991u16 =>
    EdnaFunction::IsGemeCrystal, 7367u16 => EdnaFunction::IsHotSeller, 7146u16 =>
    EdnaFunction::IsInGlobalShop, 7366u16 => EdnaFunction::IsInStock, 7368u16 =>
    EdnaFunction::IsNewToShop, 9910u16 => EdnaFunction::IsQuestItem, 11694u16 =>
    EdnaFunction::IsRecipe, 12403u16 => EdnaFunction::IsSomaSeed, 10586u16 =>
    EdnaFunction::IsSoulBounded, 9376u16 => EdnaFunction::IsTechApproved, 7748u16 =>
    EdnaFunction::IsTrialItem, 11706u16 => EdnaFunction::ItemCritVar, 11707u16 =>
    EdnaFunction::ItemNormalVar, 9014u16 => EdnaFunction::LastUseTime, 12336u16 =>
    EdnaFunction::LeftTime, 5994u16 => EdnaFunction::LootAction, 10155u16 =>
    EdnaFunction::Lua, 6174u16 => EdnaFunction::Lvl, 739u16 => EdnaFunction::LvlReq,
    4725u16 => EdnaFunction::MaterialOverride, 9891u16 => EdnaFunction::MaxStackSize,
    11698u16 => EdnaFunction::OrangeSomaRequired, 735u16 => EdnaFunction::Power, 6434u16
    => EdnaFunction::Quantity, 7718u16 => EdnaFunction::QuestTrigger, 6279u16 =>
    EdnaFunction::Rarity, 11697u16 => EdnaFunction::RedSomaRequired, 7456u16 =>
    EdnaFunction::RentalDurationMax, 7457u16 => EdnaFunction::RentalDurationMin, 7623u16
    => EdnaFunction::RentDiscount, 7625u16 => EdnaFunction::RentPriceBling, 7624u16 =>
    EdnaFunction::RentPriceGameCash, 7618u16 => EdnaFunction::SellPriceBling, 736u16 =>
    EdnaFunction::SlotId, 6248u16 => EdnaFunction::SlotMapping, 12402u16 =>
    EdnaFunction::SomaType, 12261u16 => EdnaFunction::SoulBoundedAccountId, 10614u16 =>
    EdnaFunction::SoulBoundedAvatarId, 12250u16 => EdnaFunction::SoulBoundedToAccount,
    10585u16 => EdnaFunction::SoulBoundType, 9890u16 => EdnaFunction::StackCount,
    12169u16 => EdnaFunction::StandingReq, 6019u16 => EdnaFunction::UseAction, 8996u16 =>
    EdnaFunction::UseCoolDownTimer, 9025u16 => EdnaFunction::UseCount, 8997u16 =>
    EdnaFunction::UseMaxCount, 8960u16 => EdnaFunction::UseRequireAvatar, 8959u16 =>
    EdnaFunction::UseRequireAvatarWithinRadius, 8961u16 =>
    EdnaFunction::UseRequireTarget, 8958u16 => EdnaFunction::UseScript, 6486u16 =>
    EdnaFunction::Vendorable, 5935u16 => EdnaFunction::VendorAction, 11696u16 =>
    EdnaFunction::VioletSomaRequired, 11695u16 => EdnaFunction::YellowSomaRequired,
    724u16 => EdnaFunction::Abilities, 712u16 => EdnaFunction::AbilityInstanceData,
    11548u16 => EdnaFunction::Agility, 11535u16 => EdnaFunction::Armor, 11539u16 =>
    EdnaFunction::AttackPowerRating, 6407u16 => EdnaFunction::AttributeOp1, 6406u16 =>
    EdnaFunction::AttributeOp2, 6405u16 => EdnaFunction::AttributeOp3, 6404u16 =>
    EdnaFunction::AttributeOp4, 6411u16 => EdnaFunction::AttributeType1, 6410u16 =>
    EdnaFunction::AttributeType2, 6409u16 => EdnaFunction::AttributeType3, 6408u16 =>
    EdnaFunction::AttributeType4, 6403u16 => EdnaFunction::AttributeWeight1, 6402u16 =>
    EdnaFunction::AttributeWeight2, 6401u16 => EdnaFunction::AttributeWeight3, 6400u16 =>
    EdnaFunction::AttributeWeight4, 9482u16 => EdnaFunction::AutoAttributeType1, 9481u16
    => EdnaFunction::AutoAttributeType2, 9480u16 => EdnaFunction::AutoAttributeType3,
    9479u16 => EdnaFunction::AutoAttributeType4, 9548u16 =>
    EdnaFunction::AutoAttributeType5, 9547u16 => EdnaFunction::AutoAttributeType6,
    9476u16 => EdnaFunction::AutoAttributeValue1, 9475u16 =>
    EdnaFunction::AutoAttributeValue2, 9474u16 => EdnaFunction::AutoAttributeValue3,
    9473u16 => EdnaFunction::AutoAttributeValue4, 9546u16 =>
    EdnaFunction::AutoAttributeValue5, 9545u16 => EdnaFunction::AutoAttributeValue6,
    10085u16 => EdnaFunction::AvailableSockets, 11536u16 => EdnaFunction::BlockRating,
    12037u16 => EdnaFunction::ClanName, 4249u16 => EdnaFunction::CombatStyle, 11540u16 =>
    EdnaFunction::CritDamageRating, 11544u16 => EdnaFunction::CritHitRating, 9990u16 =>
    EdnaFunction::Disguise, 12231u16 => EdnaFunction::DisplayNameColor, 12230u16 =>
    EdnaFunction::DisplayNameNumber, 12232u16 => EdnaFunction::DisplayNameRarity,
    12234u16 => EdnaFunction::DisplayNameSlot, 12233u16 => EdnaFunction::DisplayNameStat,
    11538u16 => EdnaFunction::DodgeRating, 10020u16 => EdnaFunction::Durability, 10095u16
    => EdnaFunction::DurabilityCurrent, 11547u16 => EdnaFunction::Focus, 11018u16 =>
    EdnaFunction::Gender, 11542u16 => EdnaFunction::HeavyRating, 11545u16 =>
    EdnaFunction::HitRating, 744u16 => EdnaFunction::IsBanked, 710u16 =>
    EdnaFunction::IsExpired, 10192u16 => EdnaFunction::IsSku, 9707u16 =>
    EdnaFunction::IsTemplate, 10389u16 => EdnaFunction::ItemActionGenericParam, 10899u16
    => EdnaFunction::LevelDropVariance, 4813u16 => EdnaFunction::MaxUse, 12236u16 =>
    EdnaFunction::NoteCaption, 12235u16 => EdnaFunction::NoteCaptionValue, 713u16 =>
    EdnaFunction::OtherClientInterests, 11537u16 => EdnaFunction::ParryRating, 11543u16
    => EdnaFunction::PeneRating, 9706u16 => EdnaFunction::Prefix, 12274u16 =>
    EdnaFunction::QuickbarItem, 10100u16 => EdnaFunction::RepairCost, 10105u16 =>
    EdnaFunction::SchematicCostToCreateItem, 10071u16 => EdnaFunction::SearchKeywords,
    12385u16 => EdnaFunction::SetBonuses, 12038u16 => EdnaFunction::SignOfAvatars,
    10608u16 => EdnaFunction::Skuid, 10088u16 => EdnaFunction::SocketLockedStatus,
    10086u16 => EdnaFunction::SocketOccupancyStatus, 10087u16 =>
    EdnaFunction::SocketUpgradeLevel, 11541u16 => EdnaFunction::SpecialRating, 11546u16
    => EdnaFunction::Stamina, 11549u16 => EdnaFunction::Strength, 9705u16 =>
    EdnaFunction::Suffix, 10075u16 => EdnaFunction::TemplateType, 11313u16 =>
    EdnaFunction::TemplateVersion, 709u16 => EdnaFunction::TimeStamp, 9370u16 =>
    EdnaFunction::AddBuff, 12289u16 => EdnaFunction::ConsiderForLootTables, 720u16 =>
    EdnaFunction::CooldownDuration, 10999u16 => EdnaFunction::DefaultAttack, 11002u16 =>
    EdnaFunction::DefaultSkills, 12201u16 => EdnaFunction::Faction, 10998u16 =>
    EdnaFunction::HeavyAttack, 714u16 => EdnaFunction::IsActive, 12275u16 =>
    EdnaFunction::IsConsumable, 731u16 => EdnaFunction::Level, 732u16 =>
    EdnaFunction::Name, 6653u16 => EdnaFunction::PlayrateFactor, 10997u16 =>
    EdnaFunction::SpecialAttack, 5214u16 => EdnaFunction::StickyTargets, 728u16 =>
    EdnaFunction::Ue3ClassId, 6651u16 => EdnaFunction::WeaponDamageVariance, 6650u16 =>
    EdnaFunction::WeaponDelay, 4248u16 => EdnaFunction::WeaponType, 11532u16 =>
    EdnaFunction::WepAttSpeed, 11533u16 => EdnaFunction::WepMaxDmg, 11534u16 =>
    EdnaFunction::WepMinDmg,
};
impl Attribute for EdnaFunction {
    fn class() -> Class {
        Class::EdnaFunction
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
            Self::DefaultAttack => &Self::DefaultAttack,
            Self::DefaultSkills => &Self::DefaultSkills,
            Self::Faction => &Self::Faction,
            Self::HeavyAttack => &Self::HeavyAttack,
            Self::IsActive => &Self::IsActive,
            Self::IsConsumable => &Self::IsConsumable,
            Self::Level => &Self::Level,
            Self::Name => &Self::Name,
            Self::PlayrateFactor => &Self::PlayrateFactor,
            Self::SpecialAttack => &Self::SpecialAttack,
            Self::StickyTargets => &Self::StickyTargets,
            Self::Ue3ClassId => &Self::Ue3ClassId,
            Self::WeaponDamageVariance => &Self::WeaponDamageVariance,
            Self::WeaponDelay => &Self::WeaponDelay,
            Self::WeaponType => &Self::WeaponType,
            Self::WepAttSpeed => &Self::WepAttSpeed,
            Self::WepMaxDmg => &Self::WepMaxDmg,
            Self::WepMinDmg => &Self::WepMinDmg,
        }
    }
}
impl AttributeInfo for EdnaFunction {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AdditionalItemCount1 => 12365u16,
            Self::AdditionalItemCount2 => 12364u16,
            Self::AdditionalItemCount3 => 12363u16,
            Self::AdditionalItemRequired1 => 11705u16,
            Self::AdditionalItemRequired2 => 11704u16,
            Self::AdditionalItemRequired3 => 11703u16,
            Self::AllowBuy => 7579u16,
            Self::AllowRent => 7568u16,
            Self::AllowSell => 7619u16,
            Self::BlackSomaRequired => 11702u16,
            Self::BlingPrice => 6579u16,
            Self::BlingSellingPrice => 6578u16,
            Self::BlueSomaRequired => 11701u16,
            Self::BonusSlotAmber => 11957u16,
            Self::BonusSlotRuby => 11958u16,
            Self::BonusSlotSapphire => 11959u16,
            Self::BuyDiscount => 7620u16,
            Self::BuyPriceBling => 7622u16,
            Self::BuyPriceGameCash => 7621u16,
            Self::Category => 7546u16,
            Self::Combos => 8895u16,
            Self::ContainerId => 708u16,
            Self::ContentClass => 747u16,
            Self::CraftingMapping => 12195u16,
            Self::CraftTime => 11693u16,
            Self::CreationTime => 707u16,
            Self::CrystaEffects => 11990u16,
            Self::CrystalType => 11992u16,
            Self::CyanSomaRequired => 11700u16,
            Self::Description => 6954u16,
            Self::DestroyMethod => 6487u16,
            Self::Dialogs => 8922u16,
            Self::DisplayName => 749u16,
            Self::EnableInGame => 6815u16,
            Self::EquipSlot => 745u16,
            Self::ExpireBuyBack => 11611u16,
            Self::ExpireTime => 7557u16,
            Self::Freq => 734u16,
            Self::GameCashPrice => 6577u16,
            Self::GreenSomaRequired => 11699u16,
            Self::Icon => 4346u16,
            Self::InfiniteUse => 11465u16,
            Self::InitLeftTime => 12335u16,
            Self::InventorySlotIndex => 9873u16,
            Self::IsCollectFaction => 12170u16,
            Self::IsEquiped => 743u16,
            Self::IsFactionItem => 12153u16,
            Self::IsGemeCrystal => 11991u16,
            Self::IsHotSeller => 7367u16,
            Self::IsInGlobalShop => 7146u16,
            Self::IsInStock => 7366u16,
            Self::IsNewToShop => 7368u16,
            Self::IsQuestItem => 9910u16,
            Self::IsRecipe => 11694u16,
            Self::IsSomaSeed => 12403u16,
            Self::IsSoulBounded => 10586u16,
            Self::IsTechApproved => 9376u16,
            Self::IsTrialItem => 7748u16,
            Self::ItemCritVar => 11706u16,
            Self::ItemNormalVar => 11707u16,
            Self::LastUseTime => 9014u16,
            Self::LeftTime => 12336u16,
            Self::LootAction => 5994u16,
            Self::Lua => 10155u16,
            Self::Lvl => 6174u16,
            Self::LvlReq => 739u16,
            Self::MaterialOverride => 4725u16,
            Self::MaxStackSize => 9891u16,
            Self::OrangeSomaRequired => 11698u16,
            Self::Power => 735u16,
            Self::Quantity => 6434u16,
            Self::QuestTrigger => 7718u16,
            Self::Rarity => 6279u16,
            Self::RedSomaRequired => 11697u16,
            Self::RentalDurationMax => 7456u16,
            Self::RentalDurationMin => 7457u16,
            Self::RentDiscount => 7623u16,
            Self::RentPriceBling => 7625u16,
            Self::RentPriceGameCash => 7624u16,
            Self::SellPriceBling => 7618u16,
            Self::SlotId => 736u16,
            Self::SlotMapping => 6248u16,
            Self::SomaType => 12402u16,
            Self::SoulBoundedAccountId => 12261u16,
            Self::SoulBoundedAvatarId => 10614u16,
            Self::SoulBoundedToAccount => 12250u16,
            Self::SoulBoundType => 10585u16,
            Self::StackCount => 9890u16,
            Self::StandingReq => 12169u16,
            Self::UseAction => 6019u16,
            Self::UseCoolDownTimer => 8996u16,
            Self::UseCount => 9025u16,
            Self::UseMaxCount => 8997u16,
            Self::UseRequireAvatar => 8960u16,
            Self::UseRequireAvatarWithinRadius => 8959u16,
            Self::UseRequireTarget => 8961u16,
            Self::UseScript => 8958u16,
            Self::Vendorable => 6486u16,
            Self::VendorAction => 5935u16,
            Self::VioletSomaRequired => 11696u16,
            Self::YellowSomaRequired => 11695u16,
            Self::Abilities => 724u16,
            Self::AbilityInstanceData => 712u16,
            Self::Agility => 11548u16,
            Self::Armor => 11535u16,
            Self::AttackPowerRating => 11539u16,
            Self::AttributeOp1 => 6407u16,
            Self::AttributeOp2 => 6406u16,
            Self::AttributeOp3 => 6405u16,
            Self::AttributeOp4 => 6404u16,
            Self::AttributeType1 => 6411u16,
            Self::AttributeType2 => 6410u16,
            Self::AttributeType3 => 6409u16,
            Self::AttributeType4 => 6408u16,
            Self::AttributeWeight1 => 6403u16,
            Self::AttributeWeight2 => 6402u16,
            Self::AttributeWeight3 => 6401u16,
            Self::AttributeWeight4 => 6400u16,
            Self::AutoAttributeType1 => 9482u16,
            Self::AutoAttributeType2 => 9481u16,
            Self::AutoAttributeType3 => 9480u16,
            Self::AutoAttributeType4 => 9479u16,
            Self::AutoAttributeType5 => 9548u16,
            Self::AutoAttributeType6 => 9547u16,
            Self::AutoAttributeValue1 => 9476u16,
            Self::AutoAttributeValue2 => 9475u16,
            Self::AutoAttributeValue3 => 9474u16,
            Self::AutoAttributeValue4 => 9473u16,
            Self::AutoAttributeValue5 => 9546u16,
            Self::AutoAttributeValue6 => 9545u16,
            Self::AvailableSockets => 10085u16,
            Self::BlockRating => 11536u16,
            Self::ClanName => 12037u16,
            Self::CombatStyle => 4249u16,
            Self::CritDamageRating => 11540u16,
            Self::CritHitRating => 11544u16,
            Self::Disguise => 9990u16,
            Self::DisplayNameColor => 12231u16,
            Self::DisplayNameNumber => 12230u16,
            Self::DisplayNameRarity => 12232u16,
            Self::DisplayNameSlot => 12234u16,
            Self::DisplayNameStat => 12233u16,
            Self::DodgeRating => 11538u16,
            Self::Durability => 10020u16,
            Self::DurabilityCurrent => 10095u16,
            Self::Focus => 11547u16,
            Self::Gender => 11018u16,
            Self::HeavyRating => 11542u16,
            Self::HitRating => 11545u16,
            Self::IsBanked => 744u16,
            Self::IsExpired => 710u16,
            Self::IsSku => 10192u16,
            Self::IsTemplate => 9707u16,
            Self::ItemActionGenericParam => 10389u16,
            Self::LevelDropVariance => 10899u16,
            Self::MaxUse => 4813u16,
            Self::NoteCaption => 12236u16,
            Self::NoteCaptionValue => 12235u16,
            Self::OtherClientInterests => 713u16,
            Self::ParryRating => 11537u16,
            Self::PeneRating => 11543u16,
            Self::Prefix => 9706u16,
            Self::QuickbarItem => 12274u16,
            Self::RepairCost => 10100u16,
            Self::SchematicCostToCreateItem => 10105u16,
            Self::SearchKeywords => 10071u16,
            Self::SetBonuses => 12385u16,
            Self::SignOfAvatars => 12038u16,
            Self::Skuid => 10608u16,
            Self::SocketLockedStatus => 10088u16,
            Self::SocketOccupancyStatus => 10086u16,
            Self::SocketUpgradeLevel => 10087u16,
            Self::SpecialRating => 11541u16,
            Self::Stamina => 11546u16,
            Self::Strength => 11549u16,
            Self::Suffix => 9705u16,
            Self::TemplateType => 10075u16,
            Self::TemplateVersion => 11313u16,
            Self::TimeStamp => 709u16,
            Self::AddBuff => 9370u16,
            Self::ConsiderForLootTables => 12289u16,
            Self::CooldownDuration => 720u16,
            Self::DefaultAttack => 10999u16,
            Self::DefaultSkills => 11002u16,
            Self::Faction => 12201u16,
            Self::HeavyAttack => 10998u16,
            Self::IsActive => 714u16,
            Self::IsConsumable => 12275u16,
            Self::Level => 731u16,
            Self::Name => 732u16,
            Self::PlayrateFactor => 6653u16,
            Self::SpecialAttack => 10997u16,
            Self::StickyTargets => 5214u16,
            Self::Ue3ClassId => 728u16,
            Self::WeaponDamageVariance => 6651u16,
            Self::WeaponDelay => 6650u16,
            Self::WeaponType => 4248u16,
            Self::WepAttSpeed => 11532u16,
            Self::WepMaxDmg => 11533u16,
            Self::WepMinDmg => 11534u16,
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
            Self::DefaultAttack => "DefaultAttack",
            Self::DefaultSkills => "DefaultSkills",
            Self::Faction => "Faction",
            Self::HeavyAttack => "HeavyAttack",
            Self::IsActive => "isActive",
            Self::IsConsumable => "IsConsumable",
            Self::Level => "level",
            Self::Name => "name",
            Self::PlayrateFactor => "playrateFactor",
            Self::SpecialAttack => "SpecialAttack",
            Self::StickyTargets => "StickyTargets",
            Self::Ue3ClassId => "UE3ClassID",
            Self::WeaponDamageVariance => "weaponDamageVariance",
            Self::WeaponDelay => "weaponDelay",
            Self::WeaponType => "weaponType",
            Self::WepAttSpeed => "WepAttSpeed",
            Self::WepMaxDmg => "WepMaxDmg",
            Self::WepMinDmg => "WepMinDmg",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::EquipSlot => ParamType::String,
            Self::QuickbarItem => ParamType::Bool,
            Self::AddBuff => ParamType::ContentRef,
            Self::ConsiderForLootTables => ParamType::Bool,
            Self::CooldownDuration => ParamType::Float,
            Self::DefaultAttack => ParamType::ContentRef,
            Self::DefaultSkills => ParamType::ContentRefList,
            Self::Faction => ParamType::ContentRefList,
            Self::HeavyAttack => ParamType::String,
            Self::IsActive => ParamType::ContentRefList,
            Self::IsConsumable => ParamType::Bool,
            Self::Level => ParamType::Int,
            Self::Name => ParamType::String,
            Self::PlayrateFactor => ParamType::Float,
            Self::SpecialAttack => ParamType::String,
            Self::StickyTargets => ParamType::VectorAvatarId,
            Self::Ue3ClassId => ParamType::String,
            Self::WeaponDamageVariance => ParamType::Float,
            Self::WeaponDelay => ParamType::Float,
            Self::WeaponType => ParamType::String,
            Self::WepAttSpeed => ParamType::Float,
            Self::WepMaxDmg => ParamType::Float,
            Self::WepMinDmg => ParamType::Float,
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
        static COOLDOWN_DURATION: Value = Value::Float(0f32);
        static DEFAULT_ATTACK: Lazy<Value> = Lazy::new(|| Value::ContentRef(None));
        static DEFAULT_SKILLS: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static FACTION: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static HEAVY_ATTACK: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static IS_ACTIVE: Lazy<Value> = Lazy::new(|| Value::ContentRefList(
            ContentRefList::default(),
        ));
        static IS_CONSUMABLE: Value = Value::Bool(false);
        static LEVEL: Value = Value::Int(3i32);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static PLAYRATE_FACTOR: Value = Value::Float(1f32);
        static SPECIAL_ATTACK: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static STICKY_TARGETS: Value = Value::VectorAvatarId(vec![]);
        static UE_3_CLASS_ID: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static WEAPON_DAMAGE_VARIANCE: Value = Value::Float(0.1f32);
        static WEAPON_DELAY: Value = Value::Float(2f32);
        static WEAPON_TYPE: Lazy<Value> = Lazy::new(|| Value::String(
            "Rage_2H_Club".to_string(),
        ));
        static WEP_ATT_SPEED: Value = Value::Float(0f32);
        static WEP_MAX_DMG: Value = Value::Float(0f32);
        static WEP_MIN_DMG: Value = Value::Float(0f32);
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
            Self::DefaultAttack => &DEFAULT_ATTACK,
            Self::DefaultSkills => &DEFAULT_SKILLS,
            Self::Faction => &FACTION,
            Self::HeavyAttack => &HEAVY_ATTACK,
            Self::IsActive => &IS_ACTIVE,
            Self::IsConsumable => &IS_CONSUMABLE,
            Self::Level => &LEVEL,
            Self::Name => &NAME,
            Self::PlayrateFactor => &PLAYRATE_FACTOR,
            Self::SpecialAttack => &SPECIAL_ATTACK,
            Self::StickyTargets => &STICKY_TARGETS,
            Self::Ue3ClassId => &UE_3_CLASS_ID,
            Self::WeaponDamageVariance => &WEAPON_DAMAGE_VARIANCE,
            Self::WeaponDelay => &WEAPON_DELAY,
            Self::WeaponType => &WEAPON_TYPE,
            Self::WepAttSpeed => &WEP_ATT_SPEED,
            Self::WepMaxDmg => &WEP_MAX_DMG,
            Self::WepMinDmg => &WEP_MIN_DMG,
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
            Self::CooldownDuration => {
                &[ParamFlag::NodeOwn, ParamFlag::PerInstanceSetting]
            }
            Self::DefaultAttack => {
                &[ParamFlag::Persistent, ParamFlag::Content, ParamFlag::Deprecated]
            }
            Self::DefaultSkills => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Faction => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::HeavyAttack => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::IsActive => &[ParamFlag::NodeOwn],
            Self::IsConsumable => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::Level => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Name => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::PlayrateFactor => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SpecialAttack => {
                &[
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::StickyTargets => &[ParamFlag::NodeOwn],
            Self::Ue3ClassId => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::WeaponDamageVariance => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::WeaponDelay => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WeaponType => {
                &[ParamFlag::NodeOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::WepAttSpeed => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WepMaxDmg => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                    ParamFlag::PerInstanceSetting,
                ]
            }
            Self::WepMinDmg => {
                &[
                    ParamFlag::NodeOwn,
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
impl FromStr for EdnaFunction {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EDNA_FUNCTION_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for EdnaFunction {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12365u16 => Ok(Self::AdditionalItemCount1),
            12364u16 => Ok(Self::AdditionalItemCount2),
            12363u16 => Ok(Self::AdditionalItemCount3),
            11705u16 => Ok(Self::AdditionalItemRequired1),
            11704u16 => Ok(Self::AdditionalItemRequired2),
            11703u16 => Ok(Self::AdditionalItemRequired3),
            7579u16 => Ok(Self::AllowBuy),
            7568u16 => Ok(Self::AllowRent),
            7619u16 => Ok(Self::AllowSell),
            11702u16 => Ok(Self::BlackSomaRequired),
            6579u16 => Ok(Self::BlingPrice),
            6578u16 => Ok(Self::BlingSellingPrice),
            11701u16 => Ok(Self::BlueSomaRequired),
            11957u16 => Ok(Self::BonusSlotAmber),
            11958u16 => Ok(Self::BonusSlotRuby),
            11959u16 => Ok(Self::BonusSlotSapphire),
            7620u16 => Ok(Self::BuyDiscount),
            7622u16 => Ok(Self::BuyPriceBling),
            7621u16 => Ok(Self::BuyPriceGameCash),
            7546u16 => Ok(Self::Category),
            8895u16 => Ok(Self::Combos),
            708u16 => Ok(Self::ContainerId),
            747u16 => Ok(Self::ContentClass),
            12195u16 => Ok(Self::CraftingMapping),
            11693u16 => Ok(Self::CraftTime),
            707u16 => Ok(Self::CreationTime),
            11990u16 => Ok(Self::CrystaEffects),
            11992u16 => Ok(Self::CrystalType),
            11700u16 => Ok(Self::CyanSomaRequired),
            6954u16 => Ok(Self::Description),
            6487u16 => Ok(Self::DestroyMethod),
            8922u16 => Ok(Self::Dialogs),
            749u16 => Ok(Self::DisplayName),
            6815u16 => Ok(Self::EnableInGame),
            745u16 => Ok(Self::EquipSlot),
            11611u16 => Ok(Self::ExpireBuyBack),
            7557u16 => Ok(Self::ExpireTime),
            734u16 => Ok(Self::Freq),
            6577u16 => Ok(Self::GameCashPrice),
            11699u16 => Ok(Self::GreenSomaRequired),
            4346u16 => Ok(Self::Icon),
            11465u16 => Ok(Self::InfiniteUse),
            12335u16 => Ok(Self::InitLeftTime),
            9873u16 => Ok(Self::InventorySlotIndex),
            12170u16 => Ok(Self::IsCollectFaction),
            743u16 => Ok(Self::IsEquiped),
            12153u16 => Ok(Self::IsFactionItem),
            11991u16 => Ok(Self::IsGemeCrystal),
            7367u16 => Ok(Self::IsHotSeller),
            7146u16 => Ok(Self::IsInGlobalShop),
            7366u16 => Ok(Self::IsInStock),
            7368u16 => Ok(Self::IsNewToShop),
            9910u16 => Ok(Self::IsQuestItem),
            11694u16 => Ok(Self::IsRecipe),
            12403u16 => Ok(Self::IsSomaSeed),
            10586u16 => Ok(Self::IsSoulBounded),
            9376u16 => Ok(Self::IsTechApproved),
            7748u16 => Ok(Self::IsTrialItem),
            11706u16 => Ok(Self::ItemCritVar),
            11707u16 => Ok(Self::ItemNormalVar),
            9014u16 => Ok(Self::LastUseTime),
            12336u16 => Ok(Self::LeftTime),
            5994u16 => Ok(Self::LootAction),
            10155u16 => Ok(Self::Lua),
            6174u16 => Ok(Self::Lvl),
            739u16 => Ok(Self::LvlReq),
            4725u16 => Ok(Self::MaterialOverride),
            9891u16 => Ok(Self::MaxStackSize),
            11698u16 => Ok(Self::OrangeSomaRequired),
            735u16 => Ok(Self::Power),
            6434u16 => Ok(Self::Quantity),
            7718u16 => Ok(Self::QuestTrigger),
            6279u16 => Ok(Self::Rarity),
            11697u16 => Ok(Self::RedSomaRequired),
            7456u16 => Ok(Self::RentalDurationMax),
            7457u16 => Ok(Self::RentalDurationMin),
            7623u16 => Ok(Self::RentDiscount),
            7625u16 => Ok(Self::RentPriceBling),
            7624u16 => Ok(Self::RentPriceGameCash),
            7618u16 => Ok(Self::SellPriceBling),
            736u16 => Ok(Self::SlotId),
            6248u16 => Ok(Self::SlotMapping),
            12402u16 => Ok(Self::SomaType),
            12261u16 => Ok(Self::SoulBoundedAccountId),
            10614u16 => Ok(Self::SoulBoundedAvatarId),
            12250u16 => Ok(Self::SoulBoundedToAccount),
            10585u16 => Ok(Self::SoulBoundType),
            9890u16 => Ok(Self::StackCount),
            12169u16 => Ok(Self::StandingReq),
            6019u16 => Ok(Self::UseAction),
            8996u16 => Ok(Self::UseCoolDownTimer),
            9025u16 => Ok(Self::UseCount),
            8997u16 => Ok(Self::UseMaxCount),
            8960u16 => Ok(Self::UseRequireAvatar),
            8959u16 => Ok(Self::UseRequireAvatarWithinRadius),
            8961u16 => Ok(Self::UseRequireTarget),
            8958u16 => Ok(Self::UseScript),
            6486u16 => Ok(Self::Vendorable),
            5935u16 => Ok(Self::VendorAction),
            11696u16 => Ok(Self::VioletSomaRequired),
            11695u16 => Ok(Self::YellowSomaRequired),
            724u16 => Ok(Self::Abilities),
            712u16 => Ok(Self::AbilityInstanceData),
            11548u16 => Ok(Self::Agility),
            11535u16 => Ok(Self::Armor),
            11539u16 => Ok(Self::AttackPowerRating),
            6407u16 => Ok(Self::AttributeOp1),
            6406u16 => Ok(Self::AttributeOp2),
            6405u16 => Ok(Self::AttributeOp3),
            6404u16 => Ok(Self::AttributeOp4),
            6411u16 => Ok(Self::AttributeType1),
            6410u16 => Ok(Self::AttributeType2),
            6409u16 => Ok(Self::AttributeType3),
            6408u16 => Ok(Self::AttributeType4),
            6403u16 => Ok(Self::AttributeWeight1),
            6402u16 => Ok(Self::AttributeWeight2),
            6401u16 => Ok(Self::AttributeWeight3),
            6400u16 => Ok(Self::AttributeWeight4),
            9482u16 => Ok(Self::AutoAttributeType1),
            9481u16 => Ok(Self::AutoAttributeType2),
            9480u16 => Ok(Self::AutoAttributeType3),
            9479u16 => Ok(Self::AutoAttributeType4),
            9548u16 => Ok(Self::AutoAttributeType5),
            9547u16 => Ok(Self::AutoAttributeType6),
            9476u16 => Ok(Self::AutoAttributeValue1),
            9475u16 => Ok(Self::AutoAttributeValue2),
            9474u16 => Ok(Self::AutoAttributeValue3),
            9473u16 => Ok(Self::AutoAttributeValue4),
            9546u16 => Ok(Self::AutoAttributeValue5),
            9545u16 => Ok(Self::AutoAttributeValue6),
            10085u16 => Ok(Self::AvailableSockets),
            11536u16 => Ok(Self::BlockRating),
            12037u16 => Ok(Self::ClanName),
            4249u16 => Ok(Self::CombatStyle),
            11540u16 => Ok(Self::CritDamageRating),
            11544u16 => Ok(Self::CritHitRating),
            9990u16 => Ok(Self::Disguise),
            12231u16 => Ok(Self::DisplayNameColor),
            12230u16 => Ok(Self::DisplayNameNumber),
            12232u16 => Ok(Self::DisplayNameRarity),
            12234u16 => Ok(Self::DisplayNameSlot),
            12233u16 => Ok(Self::DisplayNameStat),
            11538u16 => Ok(Self::DodgeRating),
            10020u16 => Ok(Self::Durability),
            10095u16 => Ok(Self::DurabilityCurrent),
            11547u16 => Ok(Self::Focus),
            11018u16 => Ok(Self::Gender),
            11542u16 => Ok(Self::HeavyRating),
            11545u16 => Ok(Self::HitRating),
            744u16 => Ok(Self::IsBanked),
            710u16 => Ok(Self::IsExpired),
            10192u16 => Ok(Self::IsSku),
            9707u16 => Ok(Self::IsTemplate),
            10389u16 => Ok(Self::ItemActionGenericParam),
            10899u16 => Ok(Self::LevelDropVariance),
            4813u16 => Ok(Self::MaxUse),
            12236u16 => Ok(Self::NoteCaption),
            12235u16 => Ok(Self::NoteCaptionValue),
            713u16 => Ok(Self::OtherClientInterests),
            11537u16 => Ok(Self::ParryRating),
            11543u16 => Ok(Self::PeneRating),
            9706u16 => Ok(Self::Prefix),
            12274u16 => Ok(Self::QuickbarItem),
            10100u16 => Ok(Self::RepairCost),
            10105u16 => Ok(Self::SchematicCostToCreateItem),
            10071u16 => Ok(Self::SearchKeywords),
            12385u16 => Ok(Self::SetBonuses),
            12038u16 => Ok(Self::SignOfAvatars),
            10608u16 => Ok(Self::Skuid),
            10088u16 => Ok(Self::SocketLockedStatus),
            10086u16 => Ok(Self::SocketOccupancyStatus),
            10087u16 => Ok(Self::SocketUpgradeLevel),
            11541u16 => Ok(Self::SpecialRating),
            11546u16 => Ok(Self::Stamina),
            11549u16 => Ok(Self::Strength),
            9705u16 => Ok(Self::Suffix),
            10075u16 => Ok(Self::TemplateType),
            11313u16 => Ok(Self::TemplateVersion),
            709u16 => Ok(Self::TimeStamp),
            9370u16 => Ok(Self::AddBuff),
            12289u16 => Ok(Self::ConsiderForLootTables),
            720u16 => Ok(Self::CooldownDuration),
            10999u16 => Ok(Self::DefaultAttack),
            11002u16 => Ok(Self::DefaultSkills),
            12201u16 => Ok(Self::Faction),
            10998u16 => Ok(Self::HeavyAttack),
            714u16 => Ok(Self::IsActive),
            12275u16 => Ok(Self::IsConsumable),
            731u16 => Ok(Self::Level),
            732u16 => Ok(Self::Name),
            6653u16 => Ok(Self::PlayrateFactor),
            10997u16 => Ok(Self::SpecialAttack),
            5214u16 => Ok(Self::StickyTargets),
            728u16 => Ok(Self::Ue3ClassId),
            6651u16 => Ok(Self::WeaponDamageVariance),
            6650u16 => Ok(Self::WeaponDelay),
            4248u16 => Ok(Self::WeaponType),
            11532u16 => Ok(Self::WepAttSpeed),
            11533u16 => Ok(Self::WepMaxDmg),
            11534u16 => Ok(Self::WepMinDmg),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
