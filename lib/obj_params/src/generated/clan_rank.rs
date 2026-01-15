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
pub enum ClanRank {
    BankPermissionViewItemsTab,
    BankPrivilegeDepositTab,
    BankPrivilegeViewTab,
    BankPrivilegeWithdrawAmtPerTab,
    BooleanPrivileges,
    Name,
    Note,
    RankOrder,
    ResourceWithdrawalPerDay,
}
pub(crate) static CLAN_RANK_ATTRIBUTES: phf::Map<&'static str, ClanRank> = phf_map! {
    "BankPermissionViewItemsTab" => ClanRank::BankPermissionViewItemsTab,
    "BankPrivilegeDepositTab" => ClanRank::BankPrivilegeDepositTab,
    "BankPrivilegeViewTab" => ClanRank::BankPrivilegeViewTab,
    "BankPrivilegeWithdrawAmtPerTab" => ClanRank::BankPrivilegeWithdrawAmtPerTab,
    "BooleanPrivileges" => ClanRank::BooleanPrivileges, "Name" => ClanRank::Name, "Note"
    => ClanRank::Note, "RankOrder" => ClanRank::RankOrder, "ResourceWithdrawalPerDay" =>
    ClanRank::ResourceWithdrawalPerDay,
};
pub(crate) static CLAN_RANK_ATTRIBUTES_ID: phf::Map<u16, ClanRank> = phf_map! {
    11941u16 => ClanRank::BankPermissionViewItemsTab, 10906u16 =>
    ClanRank::BankPrivilegeDepositTab, 10905u16 => ClanRank::BankPrivilegeViewTab,
    10904u16 => ClanRank::BankPrivilegeWithdrawAmtPerTab, 10365u16 =>
    ClanRank::BooleanPrivileges, 10367u16 => ClanRank::Name, 10363u16 => ClanRank::Note,
    11389u16 => ClanRank::RankOrder, 10364u16 => ClanRank::ResourceWithdrawalPerDay,
};
impl Attribute for ClanRank {
    fn class() -> Class {
        Class::ClanRank
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::BankPermissionViewItemsTab => &Self::BankPermissionViewItemsTab,
            Self::BankPrivilegeDepositTab => &Self::BankPrivilegeDepositTab,
            Self::BankPrivilegeViewTab => &Self::BankPrivilegeViewTab,
            Self::BankPrivilegeWithdrawAmtPerTab => &Self::BankPrivilegeWithdrawAmtPerTab,
            Self::BooleanPrivileges => &Self::BooleanPrivileges,
            Self::Name => &Self::Name,
            Self::Note => &Self::Note,
            Self::RankOrder => &Self::RankOrder,
            Self::ResourceWithdrawalPerDay => &Self::ResourceWithdrawalPerDay,
        }
    }
}
impl AttributeInfo for ClanRank {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BankPermissionViewItemsTab => 11941u16,
            Self::BankPrivilegeDepositTab => 10906u16,
            Self::BankPrivilegeViewTab => 10905u16,
            Self::BankPrivilegeWithdrawAmtPerTab => 10904u16,
            Self::BooleanPrivileges => 10365u16,
            Self::Name => 10367u16,
            Self::Note => 10363u16,
            Self::RankOrder => 11389u16,
            Self::ResourceWithdrawalPerDay => 10364u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::BankPermissionViewItemsTab => "BankPermissionViewItemsTab",
            Self::BankPrivilegeDepositTab => "BankPrivilegeDepositTab",
            Self::BankPrivilegeViewTab => "BankPrivilegeViewTab",
            Self::BankPrivilegeWithdrawAmtPerTab => "BankPrivilegeWithdrawAmtPerTab",
            Self::BooleanPrivileges => "BooleanPrivileges",
            Self::Name => "Name",
            Self::Note => "Note",
            Self::RankOrder => "RankOrder",
            Self::ResourceWithdrawalPerDay => "ResourceWithdrawalPerDay",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::BankPermissionViewItemsTab => ParamType::VectorInt,
            Self::BankPrivilegeDepositTab => ParamType::VectorInt,
            Self::BankPrivilegeViewTab => ParamType::VectorInt,
            Self::BankPrivilegeWithdrawAmtPerTab => ParamType::VectorInt,
            Self::BooleanPrivileges => ParamType::BitSetFilter,
            Self::Name => ParamType::String,
            Self::Note => ParamType::String,
            Self::RankOrder => ParamType::Int,
            Self::ResourceWithdrawalPerDay => ParamType::VectorInt,
        }
    }
    fn default(&self) -> &'static Value {
        static BANK_PERMISSION_VIEW_ITEMS_TAB: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static BANK_PRIVILEGE_DEPOSIT_TAB: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static BANK_PRIVILEGE_VIEW_TAB: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static BANK_PRIVILEGE_WITHDRAW_AMT_PER_TAB: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static BOOLEAN_PRIVILEGES: Value = Value::BitSetFilter(0);
        static NAME: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static NOTE: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static RANK_ORDER: Value = Value::Int(0i32);
        static RESOURCE_WITHDRAWAL_PER_DAY: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
        ));
        match self {
            Self::BankPermissionViewItemsTab => &BANK_PERMISSION_VIEW_ITEMS_TAB,
            Self::BankPrivilegeDepositTab => &BANK_PRIVILEGE_DEPOSIT_TAB,
            Self::BankPrivilegeViewTab => &BANK_PRIVILEGE_VIEW_TAB,
            Self::BankPrivilegeWithdrawAmtPerTab => &BANK_PRIVILEGE_WITHDRAW_AMT_PER_TAB,
            Self::BooleanPrivileges => &BOOLEAN_PRIVILEGES,
            Self::Name => &NAME,
            Self::Note => &NOTE,
            Self::RankOrder => &RANK_ORDER,
            Self::ResourceWithdrawalPerDay => &RESOURCE_WITHDRAWAL_PER_DAY,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::BankPermissionViewItemsTab => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BankPrivilegeDepositTab => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BankPrivilegeViewTab => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::BankPrivilegeWithdrawAmtPerTab => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::BooleanPrivileges => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Name => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Note => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::RankOrder => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ResourceWithdrawalPerDay => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
        }
    }
}
impl FromStr for ClanRank {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CLAN_RANK_ATTRIBUTES.get(s).copied().ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for ClanRank {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            11941u16 => Ok(Self::BankPermissionViewItemsTab),
            10906u16 => Ok(Self::BankPrivilegeDepositTab),
            10905u16 => Ok(Self::BankPrivilegeViewTab),
            10904u16 => Ok(Self::BankPrivilegeWithdrawAmtPerTab),
            10365u16 => Ok(Self::BooleanPrivileges),
            10367u16 => Ok(Self::Name),
            10363u16 => Ok(Self::Note),
            11389u16 => Ok(Self::RankOrder),
            10364u16 => Ok(Self::ResourceWithdrawalPerDay),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
