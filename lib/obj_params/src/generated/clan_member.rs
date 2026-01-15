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
pub enum ClanMember {
    AccountId,
    CombatStyle,
    DailyWithdrawnBits,
    DailyWithdrawnStacks,
    DateJoined,
    DatePromoted,
    Donations,
    LastLoggedIn,
    LastWithdrawlDate,
    Level,
    OfficerNote,
    PlayerId,
    PlayerName,
    PlayerNote,
    RankGuid,
    WorldLocation,
}
pub(crate) static CLAN_MEMBER_ATTRIBUTES: phf::Map<&'static str, ClanMember> = phf_map! {
    "AccountID" => ClanMember::AccountId, "CombatStyle" => ClanMember::CombatStyle,
    "DailyWithdrawnBits" => ClanMember::DailyWithdrawnBits, "DailyWithdrawnStacks" =>
    ClanMember::DailyWithdrawnStacks, "DateJoined" => ClanMember::DateJoined,
    "DatePromoted" => ClanMember::DatePromoted, "Donations" => ClanMember::Donations,
    "LastLoggedIn" => ClanMember::LastLoggedIn, "LastWithdrawlDate" =>
    ClanMember::LastWithdrawlDate, "Level" => ClanMember::Level, "OfficerNote" =>
    ClanMember::OfficerNote, "PlayerID" => ClanMember::PlayerId, "PlayerName" =>
    ClanMember::PlayerName, "PlayerNote" => ClanMember::PlayerNote, "RankGUID" =>
    ClanMember::RankGuid, "WorldLocation" => ClanMember::WorldLocation,
};
pub(crate) static CLAN_MEMBER_ATTRIBUTES_ID: phf::Map<u16, ClanMember> = phf_map! {
    12041u16 => ClanMember::AccountId, 12118u16 => ClanMember::CombatStyle, 10902u16 =>
    ClanMember::DailyWithdrawnBits, 10901u16 => ClanMember::DailyWithdrawnStacks,
    10360u16 => ClanMember::DateJoined, 10361u16 => ClanMember::DatePromoted, 10629u16 =>
    ClanMember::Donations, 10627u16 => ClanMember::LastLoggedIn, 10910u16 =>
    ClanMember::LastWithdrawlDate, 12119u16 => ClanMember::Level, 10358u16 =>
    ClanMember::OfficerNote, 10357u16 => ClanMember::PlayerId, 10356u16 =>
    ClanMember::PlayerName, 10359u16 => ClanMember::PlayerNote, 10395u16 =>
    ClanMember::RankGuid, 10628u16 => ClanMember::WorldLocation,
};
impl Attribute for ClanMember {
    fn class() -> Class {
        Class::ClanMember
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::AccountId => &Self::AccountId,
            Self::CombatStyle => &Self::CombatStyle,
            Self::DailyWithdrawnBits => &Self::DailyWithdrawnBits,
            Self::DailyWithdrawnStacks => &Self::DailyWithdrawnStacks,
            Self::DateJoined => &Self::DateJoined,
            Self::DatePromoted => &Self::DatePromoted,
            Self::Donations => &Self::Donations,
            Self::LastLoggedIn => &Self::LastLoggedIn,
            Self::LastWithdrawlDate => &Self::LastWithdrawlDate,
            Self::Level => &Self::Level,
            Self::OfficerNote => &Self::OfficerNote,
            Self::PlayerId => &Self::PlayerId,
            Self::PlayerName => &Self::PlayerName,
            Self::PlayerNote => &Self::PlayerNote,
            Self::RankGuid => &Self::RankGuid,
            Self::WorldLocation => &Self::WorldLocation,
        }
    }
}
impl AttributeInfo for ClanMember {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::AccountId => 12041u16,
            Self::CombatStyle => 12118u16,
            Self::DailyWithdrawnBits => 10902u16,
            Self::DailyWithdrawnStacks => 10901u16,
            Self::DateJoined => 10360u16,
            Self::DatePromoted => 10361u16,
            Self::Donations => 10629u16,
            Self::LastLoggedIn => 10627u16,
            Self::LastWithdrawlDate => 10910u16,
            Self::Level => 12119u16,
            Self::OfficerNote => 10358u16,
            Self::PlayerId => 10357u16,
            Self::PlayerName => 10356u16,
            Self::PlayerNote => 10359u16,
            Self::RankGuid => 10395u16,
            Self::WorldLocation => 10628u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::AccountId => "AccountID",
            Self::CombatStyle => "CombatStyle",
            Self::DailyWithdrawnBits => "DailyWithdrawnBits",
            Self::DailyWithdrawnStacks => "DailyWithdrawnStacks",
            Self::DateJoined => "DateJoined",
            Self::DatePromoted => "DatePromoted",
            Self::Donations => "Donations",
            Self::LastLoggedIn => "LastLoggedIn",
            Self::LastWithdrawlDate => "LastWithdrawlDate",
            Self::Level => "Level",
            Self::OfficerNote => "OfficerNote",
            Self::PlayerId => "PlayerID",
            Self::PlayerName => "PlayerName",
            Self::PlayerNote => "PlayerNote",
            Self::RankGuid => "RankGUID",
            Self::WorldLocation => "WorldLocation",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::AccountId => ParamType::Int,
            Self::CombatStyle => ParamType::Int,
            Self::DailyWithdrawnBits => ParamType::Int,
            Self::DailyWithdrawnStacks => ParamType::VectorInt,
            Self::DateJoined => ParamType::Int,
            Self::DatePromoted => ParamType::Int,
            Self::Donations => ParamType::VectorInt,
            Self::LastLoggedIn => ParamType::Int,
            Self::LastWithdrawlDate => ParamType::Int,
            Self::Level => ParamType::Int,
            Self::OfficerNote => ParamType::String,
            Self::PlayerId => ParamType::Int,
            Self::PlayerName => ParamType::String,
            Self::PlayerNote => ParamType::String,
            Self::RankGuid => ParamType::Guid,
            Self::WorldLocation => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static ACCOUNT_ID: Value = Value::Int(0i32);
        static COMBAT_STYLE: Value = Value::Int(0i32);
        static DAILY_WITHDRAWN_BITS: Value = Value::Int(0i32);
        static DAILY_WITHDRAWN_STACKS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static DATE_JOINED: Value = Value::Int(0i32);
        static DATE_PROMOTED: Value = Value::Int(0i32);
        static DONATIONS: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
        ));
        static LAST_LOGGED_IN: Value = Value::Int(0i32);
        static LAST_WITHDRAWL_DATE: Value = Value::Int(0i32);
        static LEVEL: Value = Value::Int(0i32);
        static OFFICER_NOTE: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static PLAYER_ID: Value = Value::Int(0i32);
        static PLAYER_NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static PLAYER_NOTE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static RANK_GUID: Value = Value::Guid(
            Uuid::from_bytes([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8,
            ]),
        );
        static WORLD_LOCATION: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        match self {
            Self::AccountId => &ACCOUNT_ID,
            Self::CombatStyle => &COMBAT_STYLE,
            Self::DailyWithdrawnBits => &DAILY_WITHDRAWN_BITS,
            Self::DailyWithdrawnStacks => &DAILY_WITHDRAWN_STACKS,
            Self::DateJoined => &DATE_JOINED,
            Self::DatePromoted => &DATE_PROMOTED,
            Self::Donations => &DONATIONS,
            Self::LastLoggedIn => &LAST_LOGGED_IN,
            Self::LastWithdrawlDate => &LAST_WITHDRAWL_DATE,
            Self::Level => &LEVEL,
            Self::OfficerNote => &OFFICER_NOTE,
            Self::PlayerId => &PLAYER_ID,
            Self::PlayerName => &PLAYER_NAME,
            Self::PlayerNote => &PLAYER_NOTE,
            Self::RankGuid => &RANK_GUID,
            Self::WorldLocation => &WORLD_LOCATION,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::AccountId => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::CombatStyle => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DailyWithdrawnBits => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DailyWithdrawnStacks => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DateJoined => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DatePromoted => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Donations => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::LastLoggedIn => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::LastWithdrawlDate => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Level => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::OfficerNote => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PlayerId => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PlayerName => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::PlayerNote => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::RankGuid => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::WorldLocation => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for ClanMember {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CLAN_MEMBER_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ClanMember {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            12041u16 => Ok(Self::AccountId),
            12118u16 => Ok(Self::CombatStyle),
            10902u16 => Ok(Self::DailyWithdrawnBits),
            10901u16 => Ok(Self::DailyWithdrawnStacks),
            10360u16 => Ok(Self::DateJoined),
            10361u16 => Ok(Self::DatePromoted),
            10629u16 => Ok(Self::Donations),
            10627u16 => Ok(Self::LastLoggedIn),
            10910u16 => Ok(Self::LastWithdrawlDate),
            12119u16 => Ok(Self::Level),
            10358u16 => Ok(Self::OfficerNote),
            10357u16 => Ok(Self::PlayerId),
            10356u16 => Ok(Self::PlayerName),
            10359u16 => Ok(Self::PlayerNote),
            10395u16 => Ok(Self::RankGuid),
            10628u16 => Ok(Self::WorldLocation),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
