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
pub enum Mail {
    Attach,
    AttachBling,
    AttachContentCount,
    AttachContentId,
    Body,
    DeliverT,
    ExpireT,
    Freq,
    From,
    FromAccount,
    LifeToTime,
    Power,
    Subject,
    To,
    ToAccount,
}
pub(crate) static MAIL_ATTRIBUTES: phf::Map<&'static str, Mail> = phf_map! {
    "attach" => Mail::Attach, "attachBling" => Mail::AttachBling, "attachContentCount" =>
    Mail::AttachContentCount, "attachContentID" => Mail::AttachContentId, "body" =>
    Mail::Body, "deliverT" => Mail::DeliverT, "expireT" => Mail::ExpireT, "Freq" =>
    Mail::Freq, "from" => Mail::From, "fromAccount" => Mail::FromAccount, "lifeToTime" =>
    Mail::LifeToTime, "Power" => Mail::Power, "subject" => Mail::Subject, "to" =>
    Mail::To, "toAccount" => Mail::ToAccount,
};
pub(crate) static MAIL_ATTRIBUTES_ID: phf::Map<u16, Mail> = phf_map! {
    8025u16 => Mail::Attach, 11565u16 => Mail::AttachBling, 10137u16 =>
    Mail::AttachContentCount, 8026u16 => Mail::AttachContentId, 814u16 => Mail::Body,
    815u16 => Mail::DeliverT, 816u16 => Mail::ExpireT, 821u16 => Mail::Freq, 817u16 =>
    Mail::From, 10171u16 => Mail::FromAccount, 11570u16 => Mail::LifeToTime, 820u16 =>
    Mail::Power, 818u16 => Mail::Subject, 819u16 => Mail::To, 10172u16 =>
    Mail::ToAccount,
};
impl Attribute for Mail {
    fn class() -> Class {
        Class::Mail
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::Attach => &Self::Attach,
            Self::AttachBling => &Self::AttachBling,
            Self::AttachContentCount => &Self::AttachContentCount,
            Self::AttachContentId => &Self::AttachContentId,
            Self::Body => &Self::Body,
            Self::DeliverT => &Self::DeliverT,
            Self::ExpireT => &Self::ExpireT,
            Self::Freq => &Self::Freq,
            Self::From => &Self::From,
            Self::FromAccount => &Self::FromAccount,
            Self::LifeToTime => &Self::LifeToTime,
            Self::Power => &Self::Power,
            Self::Subject => &Self::Subject,
            Self::To => &Self::To,
            Self::ToAccount => &Self::ToAccount,
        }
    }
}
impl AttributeInfo for Mail {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::Attach => 8025u16,
            Self::AttachBling => 11565u16,
            Self::AttachContentCount => 10137u16,
            Self::AttachContentId => 8026u16,
            Self::Body => 814u16,
            Self::DeliverT => 815u16,
            Self::ExpireT => 816u16,
            Self::Freq => 821u16,
            Self::From => 817u16,
            Self::FromAccount => 10171u16,
            Self::LifeToTime => 11570u16,
            Self::Power => 820u16,
            Self::Subject => 818u16,
            Self::To => 819u16,
            Self::ToAccount => 10172u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Attach => "attach",
            Self::AttachBling => "attachBling",
            Self::AttachContentCount => "attachContentCount",
            Self::AttachContentId => "attachContentID",
            Self::Body => "body",
            Self::DeliverT => "deliverT",
            Self::ExpireT => "expireT",
            Self::Freq => "Freq",
            Self::From => "from",
            Self::FromAccount => "fromAccount",
            Self::LifeToTime => "lifeToTime",
            Self::Power => "Power",
            Self::Subject => "subject",
            Self::To => "to",
            Self::ToAccount => "toAccount",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::Attach => ParamType::VectorGuid,
            Self::AttachBling => ParamType::Int,
            Self::AttachContentCount => ParamType::VectorInt,
            Self::AttachContentId => ParamType::VectorGuid,
            Self::Body => ParamType::String,
            Self::DeliverT => ParamType::Float,
            Self::ExpireT => ParamType::Float,
            Self::Freq => ParamType::Int,
            Self::From => ParamType::String,
            Self::FromAccount => ParamType::String,
            Self::LifeToTime => ParamType::Int64,
            Self::Power => ParamType::Int,
            Self::Subject => ParamType::String,
            Self::To => ParamType::String,
            Self::ToAccount => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static ATTACH: Value = Value::VectorGuid(vec![]);
        static ATTACH_BLING: Value = Value::Int(0i32);
        static ATTACH_CONTENT_COUNT: Lazy<Value> = Lazy::new(|| Value::VectorInt(
            vec![],
        ));
        static ATTACH_CONTENT_ID: Value = Value::VectorGuid(vec![]);
        static BODY: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static DELIVER_T: Value = Value::Float(60f32);
        static EXPIRE_T: Value = Value::Float(0f32);
        static FREQ: Value = Value::Int(0i32);
        static FROM: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static FROM_ACCOUNT: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static LIFE_TO_TIME: Value = Value::Int64(0i64);
        static POWER: Value = Value::Int(0i32);
        static SUBJECT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static TO: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static TO_ACCOUNT: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        match self {
            Self::Attach => &ATTACH,
            Self::AttachBling => &ATTACH_BLING,
            Self::AttachContentCount => &ATTACH_CONTENT_COUNT,
            Self::AttachContentId => &ATTACH_CONTENT_ID,
            Self::Body => &BODY,
            Self::DeliverT => &DELIVER_T,
            Self::ExpireT => &EXPIRE_T,
            Self::Freq => &FREQ,
            Self::From => &FROM,
            Self::FromAccount => &FROM_ACCOUNT,
            Self::LifeToTime => &LIFE_TO_TIME,
            Self::Power => &POWER,
            Self::Subject => &SUBJECT,
            Self::To => &TO,
            Self::ToAccount => &TO_ACCOUNT,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::Attach => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::AttachBling => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::AttachContentCount => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::AttachContentId => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Body => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::DeliverT => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::ExpireT => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::From => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::FromAccount => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::LifeToTime => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::Subject => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::To => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
            Self::ToAccount => {
                &[ParamFlag::ClientOwn, ParamFlag::ServerOwn, ParamFlag::Persistent]
            }
        }
    }
}
impl FromStr for Mail {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MAIL_ATTRIBUTES.get(s).map(|v| *v).ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for Mail {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            8025u16 => Ok(Self::Attach),
            11565u16 => Ok(Self::AttachBling),
            10137u16 => Ok(Self::AttachContentCount),
            8026u16 => Ok(Self::AttachContentId),
            814u16 => Ok(Self::Body),
            815u16 => Ok(Self::DeliverT),
            816u16 => Ok(Self::ExpireT),
            821u16 => Ok(Self::Freq),
            817u16 => Ok(Self::From),
            10171u16 => Ok(Self::FromAccount),
            11570u16 => Ok(Self::LifeToTime),
            820u16 => Ok(Self::Power),
            818u16 => Ok(Self::Subject),
            819u16 => Ok(Self::To),
            10172u16 => Ok(Self::ToAccount),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
