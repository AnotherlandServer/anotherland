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
pub enum Clan {
    BankItemsTab,
    BankLogEntries,
    BankLogEntryDates,
    BankLogEntryUsers,
    CharterDate,
    ClanFounder,
    ClanFounderCharacterId,
    ClanLevel,
    ClanNotes,
    ClanXp,
    DailyXp,
    DisplayName,
    LeaderLastLoggedIn,
    MemberCombatStyles,
    MemberlastLoggedIn,
    MemberLevels,
    MemberOnlineStatuses,
    Members,
    MembersRankHash,
    Motd,
    OwnedTrophies,
    RankOrder,
    Ratified,
    RatifiedDate,
    Rating,
    UpdateClanXp,
}
pub(crate) static CLAN_ATTRIBUTES: phf::Map<&'static str, Clan> = phf_map! {
    "BankItemsTab" => Clan::BankItemsTab, "BankLogEntries" => Clan::BankLogEntries,
    "BankLogEntryDates" => Clan::BankLogEntryDates, "BankLogEntryUsers" =>
    Clan::BankLogEntryUsers, "CharterDate" => Clan::CharterDate, "ClanFounder" =>
    Clan::ClanFounder, "ClanFounderCharacterID" => Clan::ClanFounderCharacterId,
    "ClanLevel" => Clan::ClanLevel, "ClanNotes" => Clan::ClanNotes, "ClanXp" =>
    Clan::ClanXp, "DailyXp" => Clan::DailyXp, "DisplayName" => Clan::DisplayName,
    "LeaderLastLoggedIn" => Clan::LeaderLastLoggedIn, "memberCombatStyles" =>
    Clan::MemberCombatStyles, "memberlastLoggedIn" => Clan::MemberlastLoggedIn,
    "memberLevels" => Clan::MemberLevels, "memberOnlineStatuses" =>
    Clan::MemberOnlineStatuses, "members" => Clan::Members, "membersRankHash" =>
    Clan::MembersRankHash, "MOTD" => Clan::Motd, "OwnedTrophies" => Clan::OwnedTrophies,
    "RankOrder" => Clan::RankOrder, "Ratified" => Clan::Ratified, "RatifiedDate" =>
    Clan::RatifiedDate, "Rating" => Clan::Rating, "UpdateClanXp" => Clan::UpdateClanXp,
};
pub(crate) static CLAN_ATTRIBUTES_ID: phf::Map<u16, Clan> = phf_map! {
    11940u16 => Clan::BankItemsTab, 10909u16 => Clan::BankLogEntries, 10908u16 =>
    Clan::BankLogEntryDates, 10907u16 => Clan::BankLogEntryUsers, 10373u16 =>
    Clan::CharterDate, 10372u16 => Clan::ClanFounder, 11094u16 =>
    Clan::ClanFounderCharacterId, 12019u16 => Clan::ClanLevel, 11082u16 =>
    Clan::ClanNotes, 12021u16 => Clan::ClanXp, 12020u16 => Clan::DailyXp, 8902u16 =>
    Clan::DisplayName, 12023u16 => Clan::LeaderLastLoggedIn, 10399u16 =>
    Clan::MemberCombatStyles, 12120u16 => Clan::MemberlastLoggedIn, 10400u16 =>
    Clan::MemberLevels, 10398u16 => Clan::MemberOnlineStatuses, 8905u16 => Clan::Members,
    10393u16 => Clan::MembersRankHash, 10371u16 => Clan::Motd, 11220u16 =>
    Clan::OwnedTrophies, 10394u16 => Clan::RankOrder, 10369u16 => Clan::Ratified,
    10368u16 => Clan::RatifiedDate, 10625u16 => Clan::Rating, 12022u16 =>
    Clan::UpdateClanXp,
};
impl Attribute for Clan {
    fn class() -> Class {
        Class::Clan
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::BankItemsTab => &Self::BankItemsTab,
            Self::BankLogEntries => &Self::BankLogEntries,
            Self::BankLogEntryDates => &Self::BankLogEntryDates,
            Self::BankLogEntryUsers => &Self::BankLogEntryUsers,
            Self::CharterDate => &Self::CharterDate,
            Self::ClanFounder => &Self::ClanFounder,
            Self::ClanFounderCharacterId => &Self::ClanFounderCharacterId,
            Self::ClanLevel => &Self::ClanLevel,
            Self::ClanNotes => &Self::ClanNotes,
            Self::ClanXp => &Self::ClanXp,
            Self::DailyXp => &Self::DailyXp,
            Self::DisplayName => &Self::DisplayName,
            Self::LeaderLastLoggedIn => &Self::LeaderLastLoggedIn,
            Self::MemberCombatStyles => &Self::MemberCombatStyles,
            Self::MemberlastLoggedIn => &Self::MemberlastLoggedIn,
            Self::MemberLevels => &Self::MemberLevels,
            Self::MemberOnlineStatuses => &Self::MemberOnlineStatuses,
            Self::Members => &Self::Members,
            Self::MembersRankHash => &Self::MembersRankHash,
            Self::Motd => &Self::Motd,
            Self::OwnedTrophies => &Self::OwnedTrophies,
            Self::RankOrder => &Self::RankOrder,
            Self::Ratified => &Self::Ratified,
            Self::RatifiedDate => &Self::RatifiedDate,
            Self::Rating => &Self::Rating,
            Self::UpdateClanXp => &Self::UpdateClanXp,
        }
    }
}
impl AttributeInfo for Clan {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BankItemsTab => 11940u16,
            Self::BankLogEntries => 10909u16,
            Self::BankLogEntryDates => 10908u16,
            Self::BankLogEntryUsers => 10907u16,
            Self::CharterDate => 10373u16,
            Self::ClanFounder => 10372u16,
            Self::ClanFounderCharacterId => 11094u16,
            Self::ClanLevel => 12019u16,
            Self::ClanNotes => 11082u16,
            Self::ClanXp => 12021u16,
            Self::DailyXp => 12020u16,
            Self::DisplayName => 8902u16,
            Self::LeaderLastLoggedIn => 12023u16,
            Self::MemberCombatStyles => 10399u16,
            Self::MemberlastLoggedIn => 12120u16,
            Self::MemberLevels => 10400u16,
            Self::MemberOnlineStatuses => 10398u16,
            Self::Members => 8905u16,
            Self::MembersRankHash => 10393u16,
            Self::Motd => 10371u16,
            Self::OwnedTrophies => 11220u16,
            Self::RankOrder => 10394u16,
            Self::Ratified => 10369u16,
            Self::RatifiedDate => 10368u16,
            Self::Rating => 10625u16,
            Self::UpdateClanXp => 12022u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::BankItemsTab => "BankItemsTab",
            Self::BankLogEntries => "BankLogEntries",
            Self::BankLogEntryDates => "BankLogEntryDates",
            Self::BankLogEntryUsers => "BankLogEntryUsers",
            Self::CharterDate => "CharterDate",
            Self::ClanFounder => "ClanFounder",
            Self::ClanFounderCharacterId => "ClanFounderCharacterID",
            Self::ClanLevel => "ClanLevel",
            Self::ClanNotes => "ClanNotes",
            Self::ClanXp => "ClanXp",
            Self::DailyXp => "DailyXp",
            Self::DisplayName => "DisplayName",
            Self::LeaderLastLoggedIn => "LeaderLastLoggedIn",
            Self::MemberCombatStyles => "memberCombatStyles",
            Self::MemberlastLoggedIn => "memberlastLoggedIn",
            Self::MemberLevels => "memberLevels",
            Self::MemberOnlineStatuses => "memberOnlineStatuses",
            Self::Members => "members",
            Self::MembersRankHash => "membersRankHash",
            Self::Motd => "MOTD",
            Self::OwnedTrophies => "OwnedTrophies",
            Self::RankOrder => "RankOrder",
            Self::Ratified => "Ratified",
            Self::RatifiedDate => "RatifiedDate",
            Self::Rating => "Rating",
            Self::UpdateClanXp => "UpdateClanXp",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::BankItemsTab => ParamType::VectorInt,
            Self::BankLogEntries => ParamType::VectorString,
            Self::BankLogEntryDates => ParamType::VectorInt,
            Self::BankLogEntryUsers => ParamType::VectorAvatarId,
            Self::CharterDate => ParamType::Int,
            Self::ClanFounder => ParamType::String,
            Self::ClanFounderCharacterId => ParamType::Int64,
            Self::ClanLevel => ParamType::Int,
            Self::ClanNotes => ParamType::String,
            Self::ClanXp => ParamType::Int64,
            Self::DailyXp => ParamType::Int,
            Self::DisplayName => ParamType::String,
            Self::LeaderLastLoggedIn => ParamType::Int64,
            Self::MemberCombatStyles => ParamType::HashmapStringInt,
            Self::MemberlastLoggedIn => ParamType::HashmapStringInt,
            Self::MemberLevels => ParamType::HashmapStringInt,
            Self::MemberOnlineStatuses => ParamType::HashmapStringInt,
            Self::Members => ParamType::HashmapStringInt,
            Self::MembersRankHash => ParamType::HashmapStringString,
            Self::Motd => ParamType::String,
            Self::OwnedTrophies => ParamType::VectorString,
            Self::RankOrder => ParamType::VectorString,
            Self::Ratified => ParamType::Bool,
            Self::RatifiedDate => ParamType::Int,
            Self::Rating => ParamType::Int,
            Self::UpdateClanXp => ParamType::Int64,
        }
    }
    fn default(&self) -> &'static Value {
        static BANK_ITEMS_TAB: Lazy<Value> = Lazy::new(|| Value::VectorInt(vec![30i32]));
        static BANK_LOG_ENTRIES: Lazy<Value> = Lazy::new(|| Value::VectorString(vec![]));
        static BANK_LOG_ENTRY_DATES: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static BANK_LOG_ENTRY_USERS: Value = Value::VectorAvatarId(vec![]);
        static CHARTER_DATE: Value = Value::Int(1i32);
        static CLAN_FOUNDER: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static CLAN_FOUNDER_CHARACTER_ID: Value = Value::Int64(0i64);
        static CLAN_LEVEL: Value = Value::Int(0i32);
        static CLAN_NOTES: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static CLAN_XP: Value = Value::Int64(0i64);
        static DAILY_XP: Value = Value::Int(0i32);
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static LEADER_LAST_LOGGED_IN: Value = Value::Int64(0i64);
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
        static MEMBERS: Lazy<Value> = Lazy::new(|| Value::HashmapStringInt(
            HashMap::new(),
        ));
        static MEMBERS_RANK_HASH: Lazy<Value> = Lazy::new(|| Value::HashmapStringString(
            HashMap::new(),
        ));
        static MOTD: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static OWNED_TROPHIES: Lazy<Value> = Lazy::new(|| Value::VectorString(vec![]));
        static RANK_ORDER: Lazy<Value> = Lazy::new(|| Value::VectorString(vec![]));
        static RATIFIED: Value = Value::Bool(false);
        static RATIFIED_DATE: Value = Value::Int(0i32);
        static RATING: Value = Value::Int(0i32);
        static UPDATE_CLAN_XP: Value = Value::Int64(0i64);
        match self {
            Self::BankItemsTab => &BANK_ITEMS_TAB,
            Self::BankLogEntries => &BANK_LOG_ENTRIES,
            Self::BankLogEntryDates => &BANK_LOG_ENTRY_DATES,
            Self::BankLogEntryUsers => &BANK_LOG_ENTRY_USERS,
            Self::CharterDate => &CHARTER_DATE,
            Self::ClanFounder => &CLAN_FOUNDER,
            Self::ClanFounderCharacterId => &CLAN_FOUNDER_CHARACTER_ID,
            Self::ClanLevel => &CLAN_LEVEL,
            Self::ClanNotes => &CLAN_NOTES,
            Self::ClanXp => &CLAN_XP,
            Self::DailyXp => &DAILY_XP,
            Self::DisplayName => &DISPLAY_NAME,
            Self::LeaderLastLoggedIn => &LEADER_LAST_LOGGED_IN,
            Self::MemberCombatStyles => &MEMBER_COMBAT_STYLES,
            Self::MemberlastLoggedIn => &MEMBERLAST_LOGGED_IN,
            Self::MemberLevels => &MEMBER_LEVELS,
            Self::MemberOnlineStatuses => &MEMBER_ONLINE_STATUSES,
            Self::Members => &MEMBERS,
            Self::MembersRankHash => &MEMBERS_RANK_HASH,
            Self::Motd => &MOTD,
            Self::OwnedTrophies => &OWNED_TROPHIES,
            Self::RankOrder => &RANK_ORDER,
            Self::Ratified => &RATIFIED,
            Self::RatifiedDate => &RATIFIED_DATE,
            Self::Rating => &RATING,
            Self::UpdateClanXp => &UPDATE_CLAN_XP,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::BankItemsTab => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::BankLogEntries => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::BankLogEntryDates => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::BankLogEntryUsers => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::CharterDate => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanFounder => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanFounderCharacterId => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ClanLevel => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanNotes => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ClanXp => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DailyXp => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DisplayName => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::LeaderLastLoggedIn => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberCombatStyles => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberlastLoggedIn => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberLevels => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MemberOnlineStatuses => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Members => &[ParamFlag::ServerOwn],
            Self::MembersRankHash => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Motd => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::OwnedTrophies => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::RankOrder => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Ratified => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::RatifiedDate => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Rating => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::UpdateClanXp => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for Clan {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CLAN_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Clan {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11940u16 => Ok(Self::BankItemsTab),
            10909u16 => Ok(Self::BankLogEntries),
            10908u16 => Ok(Self::BankLogEntryDates),
            10907u16 => Ok(Self::BankLogEntryUsers),
            10373u16 => Ok(Self::CharterDate),
            10372u16 => Ok(Self::ClanFounder),
            11094u16 => Ok(Self::ClanFounderCharacterId),
            12019u16 => Ok(Self::ClanLevel),
            11082u16 => Ok(Self::ClanNotes),
            12021u16 => Ok(Self::ClanXp),
            12020u16 => Ok(Self::DailyXp),
            8902u16 => Ok(Self::DisplayName),
            12023u16 => Ok(Self::LeaderLastLoggedIn),
            10399u16 => Ok(Self::MemberCombatStyles),
            12120u16 => Ok(Self::MemberlastLoggedIn),
            10400u16 => Ok(Self::MemberLevels),
            10398u16 => Ok(Self::MemberOnlineStatuses),
            8905u16 => Ok(Self::Members),
            10393u16 => Ok(Self::MembersRankHash),
            10371u16 => Ok(Self::Motd),
            11220u16 => Ok(Self::OwnedTrophies),
            10394u16 => Ok(Self::RankOrder),
            10369u16 => Ok(Self::Ratified),
            10368u16 => Ok(Self::RatifiedDate),
            10625u16 => Ok(Self::Rating),
            12022u16 => Ok(Self::UpdateClanXp),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
