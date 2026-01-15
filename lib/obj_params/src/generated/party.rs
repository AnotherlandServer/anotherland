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
pub enum Party {
    Freq,
    IsRaid,
    LootItemRarity,
    LootType,
    MemberCombatStyles,
    MemberlastLoggedIn,
    MemberLevels,
    MemberOnlineStatuses,
    Members,
    MembersPassOnLoot,
    OfflineMembers,
    PartyAssistant,
    PartyLeader,
    PartyLimit,
    Power,
    Range,
    UnLockedPortals,
}
pub(crate) static PARTY_ATTRIBUTES: phf::Map<&'static str, Party> = phf_map! {
    "Freq" => Party::Freq, "isRaid" => Party::IsRaid, "LootItemRarity" =>
    Party::LootItemRarity, "LootType" => Party::LootType, "memberCombatStyles" =>
    Party::MemberCombatStyles, "memberlastLoggedIn" => Party::MemberlastLoggedIn,
    "memberLevels" => Party::MemberLevels, "memberOnlineStatuses" =>
    Party::MemberOnlineStatuses, "members" => Party::Members, "membersPassOnLoot" =>
    Party::MembersPassOnLoot, "offlineMembers" => Party::OfflineMembers, "PartyAssistant"
    => Party::PartyAssistant, "PartyLeader" => Party::PartyLeader, "partyLimit" =>
    Party::PartyLimit, "Power" => Party::Power, "range" => Party::Range,
    "unLockedPortals" => Party::UnLockedPortals,
};
pub(crate) static PARTY_ATTRIBUTES_ID: phf::Map<u16, Party> = phf_map! {
    2598u16 => Party::Freq, 12187u16 => Party::IsRaid, 12287u16 => Party::LootItemRarity,
    12285u16 => Party::LootType, 12281u16 => Party::MemberCombatStyles, 12279u16 =>
    Party::MemberlastLoggedIn, 12282u16 => Party::MemberLevels, 12280u16 =>
    Party::MemberOnlineStatuses, 8050u16 => Party::Members, 12286u16 =>
    Party::MembersPassOnLoot, 12283u16 => Party::OfflineMembers, 12188u16 =>
    Party::PartyAssistant, 3460u16 => Party::PartyLeader, 8049u16 => Party::PartyLimit,
    2597u16 => Party::Power, 2596u16 => Party::Range, 8333u16 => Party::UnLockedPortals,
};
impl Attribute for Party {
    fn class() -> Class {
        Class::Party
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Freq => &Self::Freq,
            Self::IsRaid => &Self::IsRaid,
            Self::LootItemRarity => &Self::LootItemRarity,
            Self::LootType => &Self::LootType,
            Self::MemberCombatStyles => &Self::MemberCombatStyles,
            Self::MemberlastLoggedIn => &Self::MemberlastLoggedIn,
            Self::MemberLevels => &Self::MemberLevels,
            Self::MemberOnlineStatuses => &Self::MemberOnlineStatuses,
            Self::Members => &Self::Members,
            Self::MembersPassOnLoot => &Self::MembersPassOnLoot,
            Self::OfflineMembers => &Self::OfflineMembers,
            Self::PartyAssistant => &Self::PartyAssistant,
            Self::PartyLeader => &Self::PartyLeader,
            Self::PartyLimit => &Self::PartyLimit,
            Self::Power => &Self::Power,
            Self::Range => &Self::Range,
            Self::UnLockedPortals => &Self::UnLockedPortals,
        }
    }
}
impl AttributeInfo for Party {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Freq => 2598u16,
            Self::IsRaid => 12187u16,
            Self::LootItemRarity => 12287u16,
            Self::LootType => 12285u16,
            Self::MemberCombatStyles => 12281u16,
            Self::MemberlastLoggedIn => 12279u16,
            Self::MemberLevels => 12282u16,
            Self::MemberOnlineStatuses => 12280u16,
            Self::Members => 8050u16,
            Self::MembersPassOnLoot => 12286u16,
            Self::OfflineMembers => 12283u16,
            Self::PartyAssistant => 12188u16,
            Self::PartyLeader => 3460u16,
            Self::PartyLimit => 8049u16,
            Self::Power => 2597u16,
            Self::Range => 2596u16,
            Self::UnLockedPortals => 8333u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Freq => "Freq",
            Self::IsRaid => "isRaid",
            Self::LootItemRarity => "LootItemRarity",
            Self::LootType => "LootType",
            Self::MemberCombatStyles => "memberCombatStyles",
            Self::MemberlastLoggedIn => "memberlastLoggedIn",
            Self::MemberLevels => "memberLevels",
            Self::MemberOnlineStatuses => "memberOnlineStatuses",
            Self::Members => "members",
            Self::MembersPassOnLoot => "membersPassOnLoot",
            Self::OfflineMembers => "offlineMembers",
            Self::PartyAssistant => "PartyAssistant",
            Self::PartyLeader => "PartyLeader",
            Self::PartyLimit => "partyLimit",
            Self::Power => "Power",
            Self::Range => "range",
            Self::UnLockedPortals => "unLockedPortals",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Freq => ParamType::Int,
            Self::IsRaid => ParamType::Bool,
            Self::LootItemRarity => ParamType::String,
            Self::LootType => ParamType::Int,
            Self::MemberCombatStyles => ParamType::HashmapStringInt,
            Self::MemberlastLoggedIn => ParamType::HashmapStringInt,
            Self::MemberLevels => ParamType::HashmapStringInt,
            Self::MemberOnlineStatuses => ParamType::HashmapStringInt,
            Self::Members => ParamType::AvatarIdSet,
            Self::MembersPassOnLoot => ParamType::AvatarIdSet,
            Self::OfflineMembers => ParamType::HashmapStringInt,
            Self::PartyAssistant => ParamType::AvatarId,
            Self::PartyLeader => ParamType::AvatarId,
            Self::PartyLimit => ParamType::Int,
            Self::Power => ParamType::Int,
            Self::Range => ParamType::Float,
            Self::UnLockedPortals => ParamType::VectorInt64,
        }
    }
    fn default(&self) -> &'static Value {
        static FREQ: Value = Value::Int(0i32);
        static IS_RAID: Value = Value::Bool(false);
        static LOOT_ITEM_RARITY: Lazy<Value> = Lazy::new(|| Value::String(
            "Uncommon".to_string(),
        ));
        static LOOT_TYPE: Value = Value::Int(0i32);
        static MEMBER_COMBAT_STYLES: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static MEMBERLAST_LOGGED_IN: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static MEMBER_LEVELS: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static MEMBER_ONLINE_STATUSES: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static MEMBERS: Lazy<Value> = Lazy::new(|| Value::AvatarIdSet(HashSet::new()));
        static MEMBERS_PASS_ON_LOOT: Lazy<Value> = Lazy::new(|| Value::AvatarIdSet(
            HashSet::new(),
        ));
        static OFFLINE_MEMBERS: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static PARTY_ASSISTANT: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static PARTY_LEADER: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static PARTY_LIMIT: Value = Value::Int(5i32);
        static POWER: Value = Value::Int(0i32);
        static RANGE: Value = Value::Float(1000f32);
        static UN_LOCKED_PORTALS: Value = Value::VectorInt64(vec![]);
        match self {
            Self::Freq => &FREQ,
            Self::IsRaid => &IS_RAID,
            Self::LootItemRarity => &LOOT_ITEM_RARITY,
            Self::LootType => &LOOT_TYPE,
            Self::MemberCombatStyles => &MEMBER_COMBAT_STYLES,
            Self::MemberlastLoggedIn => &MEMBERLAST_LOGGED_IN,
            Self::MemberLevels => &MEMBER_LEVELS,
            Self::MemberOnlineStatuses => &MEMBER_ONLINE_STATUSES,
            Self::Members => &MEMBERS,
            Self::MembersPassOnLoot => &MEMBERS_PASS_ON_LOOT,
            Self::OfflineMembers => &OFFLINE_MEMBERS,
            Self::PartyAssistant => &PARTY_ASSISTANT,
            Self::PartyLeader => &PARTY_LEADER,
            Self::PartyLimit => &PARTY_LIMIT,
            Self::Power => &POWER,
            Self::Range => &RANGE,
            Self::UnLockedPortals => &UN_LOCKED_PORTALS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::IsRaid => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::LootItemRarity => &[ParamFlag::ServerOwn],
            Self::LootType => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberCombatStyles => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberlastLoggedIn => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberLevels => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberOnlineStatuses => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Members => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MembersPassOnLoot => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::OfflineMembers => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PartyAssistant => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PartyLeader => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PartyLimit => &[ParamFlag::Persistent],
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Range => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::UnLockedPortals => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for Party {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PARTY_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Party {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            2598u16 => Ok(Self::Freq),
            12187u16 => Ok(Self::IsRaid),
            12287u16 => Ok(Self::LootItemRarity),
            12285u16 => Ok(Self::LootType),
            12281u16 => Ok(Self::MemberCombatStyles),
            12279u16 => Ok(Self::MemberlastLoggedIn),
            12282u16 => Ok(Self::MemberLevels),
            12280u16 => Ok(Self::MemberOnlineStatuses),
            8050u16 => Ok(Self::Members),
            12286u16 => Ok(Self::MembersPassOnLoot),
            12283u16 => Ok(Self::OfflineMembers),
            12188u16 => Ok(Self::PartyAssistant),
            3460u16 => Ok(Self::PartyLeader),
            8049u16 => Ok(Self::PartyLimit),
            2597u16 => Ok(Self::Power),
            2596u16 => Ok(Self::Range),
            8333u16 => Ok(Self::UnLockedPortals),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
