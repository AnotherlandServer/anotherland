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
pub enum GameSession {
    CurrentPlayers,
    DisplayName,
    Freq,
    HostIp,
    M1,
    M10,
    M11,
    M12,
    M13,
    M14,
    M15,
    M16,
    M17,
    M18,
    M19,
    M2,
    M20,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,
    MaxPlayersPerTeam,
    MaxTeams,
    MinPlayersPerTeam,
    MinTeams,
    Power,
    SrcAvatar,
    StartDelaySeconds,
}
pub(crate) static GAME_SESSION_ATTRIBUTES: phf::Map<&'static str, GameSession> = phf_map! {
    "currentPlayers" => GameSession::CurrentPlayers, "DisplayName" =>
    GameSession::DisplayName, "Freq" => GameSession::Freq, "hostIP" =>
    GameSession::HostIp, "m1" => GameSession::M1, "m10" => GameSession::M10, "m11" =>
    GameSession::M11, "m12" => GameSession::M12, "m13" => GameSession::M13, "m14" =>
    GameSession::M14, "m15" => GameSession::M15, "m16" => GameSession::M16, "m17" =>
    GameSession::M17, "m18" => GameSession::M18, "m19" => GameSession::M19, "m2" =>
    GameSession::M2, "m20" => GameSession::M20, "m3" => GameSession::M3, "m4" =>
    GameSession::M4, "m5" => GameSession::M5, "m6" => GameSession::M6, "m7" =>
    GameSession::M7, "m8" => GameSession::M8, "m9" => GameSession::M9,
    "maxPlayersPerTeam" => GameSession::MaxPlayersPerTeam, "maxTeams" =>
    GameSession::MaxTeams, "minPlayersPerTeam" => GameSession::MinPlayersPerTeam,
    "minTeams" => GameSession::MinTeams, "Power" => GameSession::Power, "srcAvatar" =>
    GameSession::SrcAvatar, "startDelaySeconds" => GameSession::StartDelaySeconds,
};
pub(crate) static GAME_SESSION_ATTRIBUTES_ID: phf::Map<u16, GameSession> = phf_map! {
    276u16 => GameSession::CurrentPlayers, 275u16 => GameSession::DisplayName, 301u16 =>
    GameSession::Freq, 277u16 => GameSession::HostIp, 278u16 => GameSession::M1, 279u16
    => GameSession::M10, 280u16 => GameSession::M11, 281u16 => GameSession::M12, 282u16
    => GameSession::M13, 283u16 => GameSession::M14, 284u16 => GameSession::M15, 285u16
    => GameSession::M16, 286u16 => GameSession::M17, 287u16 => GameSession::M18, 288u16
    => GameSession::M19, 289u16 => GameSession::M2, 290u16 => GameSession::M20, 291u16 =>
    GameSession::M3, 292u16 => GameSession::M4, 293u16 => GameSession::M5, 294u16 =>
    GameSession::M6, 295u16 => GameSession::M7, 296u16 => GameSession::M8, 297u16 =>
    GameSession::M9, 271u16 => GameSession::MaxPlayersPerTeam, 269u16 =>
    GameSession::MaxTeams, 272u16 => GameSession::MinPlayersPerTeam, 270u16 =>
    GameSession::MinTeams, 300u16 => GameSession::Power, 299u16 =>
    GameSession::SrcAvatar, 273u16 => GameSession::StartDelaySeconds,
};
impl Attribute for GameSession {
    fn class() -> Class {
        Class::GameSession
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::CurrentPlayers => &Self::CurrentPlayers,
            Self::DisplayName => &Self::DisplayName,
            Self::Freq => &Self::Freq,
            Self::HostIp => &Self::HostIp,
            Self::M1 => &Self::M1,
            Self::M10 => &Self::M10,
            Self::M11 => &Self::M11,
            Self::M12 => &Self::M12,
            Self::M13 => &Self::M13,
            Self::M14 => &Self::M14,
            Self::M15 => &Self::M15,
            Self::M16 => &Self::M16,
            Self::M17 => &Self::M17,
            Self::M18 => &Self::M18,
            Self::M19 => &Self::M19,
            Self::M2 => &Self::M2,
            Self::M20 => &Self::M20,
            Self::M3 => &Self::M3,
            Self::M4 => &Self::M4,
            Self::M5 => &Self::M5,
            Self::M6 => &Self::M6,
            Self::M7 => &Self::M7,
            Self::M8 => &Self::M8,
            Self::M9 => &Self::M9,
            Self::MaxPlayersPerTeam => &Self::MaxPlayersPerTeam,
            Self::MaxTeams => &Self::MaxTeams,
            Self::MinPlayersPerTeam => &Self::MinPlayersPerTeam,
            Self::MinTeams => &Self::MinTeams,
            Self::Power => &Self::Power,
            Self::SrcAvatar => &Self::SrcAvatar,
            Self::StartDelaySeconds => &Self::StartDelaySeconds,
        }
    }
}
impl AttributeInfo for GameSession {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::CurrentPlayers => 276u16,
            Self::DisplayName => 275u16,
            Self::Freq => 301u16,
            Self::HostIp => 277u16,
            Self::M1 => 278u16,
            Self::M10 => 279u16,
            Self::M11 => 280u16,
            Self::M12 => 281u16,
            Self::M13 => 282u16,
            Self::M14 => 283u16,
            Self::M15 => 284u16,
            Self::M16 => 285u16,
            Self::M17 => 286u16,
            Self::M18 => 287u16,
            Self::M19 => 288u16,
            Self::M2 => 289u16,
            Self::M20 => 290u16,
            Self::M3 => 291u16,
            Self::M4 => 292u16,
            Self::M5 => 293u16,
            Self::M6 => 294u16,
            Self::M7 => 295u16,
            Self::M8 => 296u16,
            Self::M9 => 297u16,
            Self::MaxPlayersPerTeam => 271u16,
            Self::MaxTeams => 269u16,
            Self::MinPlayersPerTeam => 272u16,
            Self::MinTeams => 270u16,
            Self::Power => 300u16,
            Self::SrcAvatar => 299u16,
            Self::StartDelaySeconds => 273u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::CurrentPlayers => "currentPlayers",
            Self::DisplayName => "DisplayName",
            Self::Freq => "Freq",
            Self::HostIp => "hostIP",
            Self::M1 => "m1",
            Self::M10 => "m10",
            Self::M11 => "m11",
            Self::M12 => "m12",
            Self::M13 => "m13",
            Self::M14 => "m14",
            Self::M15 => "m15",
            Self::M16 => "m16",
            Self::M17 => "m17",
            Self::M18 => "m18",
            Self::M19 => "m19",
            Self::M2 => "m2",
            Self::M20 => "m20",
            Self::M3 => "m3",
            Self::M4 => "m4",
            Self::M5 => "m5",
            Self::M6 => "m6",
            Self::M7 => "m7",
            Self::M8 => "m8",
            Self::M9 => "m9",
            Self::MaxPlayersPerTeam => "maxPlayersPerTeam",
            Self::MaxTeams => "maxTeams",
            Self::MinPlayersPerTeam => "minPlayersPerTeam",
            Self::MinTeams => "minTeams",
            Self::Power => "Power",
            Self::SrcAvatar => "srcAvatar",
            Self::StartDelaySeconds => "startDelaySeconds",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::CurrentPlayers => ParamType::Int,
            Self::DisplayName => ParamType::String,
            Self::Freq => ParamType::Int,
            Self::HostIp => ParamType::String,
            Self::M1 => ParamType::String,
            Self::M10 => ParamType::String,
            Self::M11 => ParamType::String,
            Self::M12 => ParamType::String,
            Self::M13 => ParamType::String,
            Self::M14 => ParamType::String,
            Self::M15 => ParamType::String,
            Self::M16 => ParamType::String,
            Self::M17 => ParamType::String,
            Self::M18 => ParamType::String,
            Self::M19 => ParamType::String,
            Self::M2 => ParamType::String,
            Self::M20 => ParamType::String,
            Self::M3 => ParamType::String,
            Self::M4 => ParamType::String,
            Self::M5 => ParamType::String,
            Self::M6 => ParamType::String,
            Self::M7 => ParamType::String,
            Self::M8 => ParamType::String,
            Self::M9 => ParamType::String,
            Self::MaxPlayersPerTeam => ParamType::Int,
            Self::MaxTeams => ParamType::Int,
            Self::MinPlayersPerTeam => ParamType::Int,
            Self::MinTeams => ParamType::Int,
            Self::Power => ParamType::Int,
            Self::SrcAvatar => ParamType::AvatarId,
            Self::StartDelaySeconds => ParamType::Int,
        }
    }
    fn default(&self) -> &'static Value {
        static CURRENT_PLAYERS: Value = Value::Int(0i32);
        static DISPLAY_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            "DefaultDisplayName".to_string(),
        ));
        static FREQ: Value = Value::Int(0i32);
        static HOST_IP: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_1: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_10: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_11: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_12: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_13: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_14: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_15: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_16: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_17: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_18: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_19: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_2: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_20: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_3: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_4: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_5: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_6: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_7: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_8: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static M_9: Lazy<Value> = Lazy::new(|| Value::String(String::default()));
        static MAX_PLAYERS_PER_TEAM: Value = Value::Int(0i32);
        static MAX_TEAMS: Value = Value::Int(0i32);
        static MIN_PLAYERS_PER_TEAM: Value = Value::Int(0i32);
        static MIN_TEAMS: Value = Value::Int(0i32);
        static POWER: Value = Value::Int(0i32);
        static SRC_AVATAR: Value = Value::AvatarId(AvatarId::from_u64(0u64));
        static START_DELAY_SECONDS: Value = Value::Int(0i32);
        match self {
            Self::CurrentPlayers => &CURRENT_PLAYERS,
            Self::DisplayName => &DISPLAY_NAME,
            Self::Freq => &FREQ,
            Self::HostIp => &HOST_IP,
            Self::M1 => &M_1,
            Self::M10 => &M_10,
            Self::M11 => &M_11,
            Self::M12 => &M_12,
            Self::M13 => &M_13,
            Self::M14 => &M_14,
            Self::M15 => &M_15,
            Self::M16 => &M_16,
            Self::M17 => &M_17,
            Self::M18 => &M_18,
            Self::M19 => &M_19,
            Self::M2 => &M_2,
            Self::M20 => &M_20,
            Self::M3 => &M_3,
            Self::M4 => &M_4,
            Self::M5 => &M_5,
            Self::M6 => &M_6,
            Self::M7 => &M_7,
            Self::M8 => &M_8,
            Self::M9 => &M_9,
            Self::MaxPlayersPerTeam => &MAX_PLAYERS_PER_TEAM,
            Self::MaxTeams => &MAX_TEAMS,
            Self::MinPlayersPerTeam => &MIN_PLAYERS_PER_TEAM,
            Self::MinTeams => &MIN_TEAMS,
            Self::Power => &POWER,
            Self::SrcAvatar => &SRC_AVATAR,
            Self::StartDelaySeconds => &START_DELAY_SECONDS,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::CurrentPlayers => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::DisplayName => {
                &[ParamFlag::ServerOwn, ParamFlag::Persistent, ParamFlag::Content]
            }
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::HostIp => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M1 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M10 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M11 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M12 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M13 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M14 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M15 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M16 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M17 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M18 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M19 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M2 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M20 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M3 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M4 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M5 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M6 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M7 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M8 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::M9 => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MaxPlayersPerTeam => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MaxTeams => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MinPlayersPerTeam => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::MinTeams => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::SrcAvatar => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
            Self::StartDelaySeconds => &[ParamFlag::ServerOwn, ParamFlag::Persistent],
        }
    }
}
impl FromStr for GameSession {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GAME_SESSION_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for GameSession {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            276u16 => Ok(Self::CurrentPlayers),
            275u16 => Ok(Self::DisplayName),
            301u16 => Ok(Self::Freq),
            277u16 => Ok(Self::HostIp),
            278u16 => Ok(Self::M1),
            279u16 => Ok(Self::M10),
            280u16 => Ok(Self::M11),
            281u16 => Ok(Self::M12),
            282u16 => Ok(Self::M13),
            283u16 => Ok(Self::M14),
            284u16 => Ok(Self::M15),
            285u16 => Ok(Self::M16),
            286u16 => Ok(Self::M17),
            287u16 => Ok(Self::M18),
            288u16 => Ok(Self::M19),
            289u16 => Ok(Self::M2),
            290u16 => Ok(Self::M20),
            291u16 => Ok(Self::M3),
            292u16 => Ok(Self::M4),
            293u16 => Ok(Self::M5),
            294u16 => Ok(Self::M6),
            295u16 => Ok(Self::M7),
            296u16 => Ok(Self::M8),
            297u16 => Ok(Self::M9),
            271u16 => Ok(Self::MaxPlayersPerTeam),
            269u16 => Ok(Self::MaxTeams),
            272u16 => Ok(Self::MinPlayersPerTeam),
            270u16 => Ok(Self::MinTeams),
            300u16 => Ok(Self::Power),
            299u16 => Ok(Self::SrcAvatar),
            273u16 => Ok(Self::StartDelaySeconds),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
