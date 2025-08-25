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
pub enum CompinstLoginsrv {
    BytesReceivedPeer,
    BytesReceivedServer,
    BytesSentPeer,
    BytesSentServer,
    CpuUsage,
    Freq,
    GameDll,
    MemoryUsage,
    MemoryUsagePeak,
    PageFaults,
    PageFileUsage,
    PageFileUsagePeak,
    Power,
    TickTime,
    TotalCpuTime,
    TotalUpTime,
    LoginFailures,
    Logins,
    LoginsPerMinute,
}
pub(crate) static COMPINST_LOGINSRV_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstLoginsrv,
> = phf_map! {
    "BytesReceivedPeer" => CompinstLoginsrv::BytesReceivedPeer, "BytesReceivedServer" =>
    CompinstLoginsrv::BytesReceivedServer, "BytesSentPeer" =>
    CompinstLoginsrv::BytesSentPeer, "BytesSentServer" =>
    CompinstLoginsrv::BytesSentServer, "cpuUsage" => CompinstLoginsrv::CpuUsage, "Freq"
    => CompinstLoginsrv::Freq, "gameDLL" => CompinstLoginsrv::GameDll, "memoryUsage" =>
    CompinstLoginsrv::MemoryUsage, "memoryUsagePeak" =>
    CompinstLoginsrv::MemoryUsagePeak, "pageFaults" => CompinstLoginsrv::PageFaults,
    "pageFileUsage" => CompinstLoginsrv::PageFileUsage, "pageFileUsagePeak" =>
    CompinstLoginsrv::PageFileUsagePeak, "Power" => CompinstLoginsrv::Power, "tickTime"
    => CompinstLoginsrv::TickTime, "totalCPUTime" => CompinstLoginsrv::TotalCpuTime,
    "totalUpTime" => CompinstLoginsrv::TotalUpTime, "loginFailures" =>
    CompinstLoginsrv::LoginFailures, "logins" => CompinstLoginsrv::Logins,
    "loginsPerMinute" => CompinstLoginsrv::LoginsPerMinute,
};
pub(crate) static COMPINST_LOGINSRV_ATTRIBUTES_ID: phf::Map<u16, CompinstLoginsrv> = phf_map! {
    156u16 => CompinstLoginsrv::BytesReceivedPeer, 157u16 =>
    CompinstLoginsrv::BytesReceivedServer, 158u16 => CompinstLoginsrv::BytesSentPeer,
    159u16 => CompinstLoginsrv::BytesSentServer, 160u16 => CompinstLoginsrv::CpuUsage,
    170u16 => CompinstLoginsrv::Freq, 161u16 => CompinstLoginsrv::GameDll, 162u16 =>
    CompinstLoginsrv::MemoryUsage, 163u16 => CompinstLoginsrv::MemoryUsagePeak, 164u16 =>
    CompinstLoginsrv::PageFaults, 165u16 => CompinstLoginsrv::PageFileUsage, 166u16 =>
    CompinstLoginsrv::PageFileUsagePeak, 169u16 => CompinstLoginsrv::Power, 152u16 =>
    CompinstLoginsrv::TickTime, 167u16 => CompinstLoginsrv::TotalCpuTime, 168u16 =>
    CompinstLoginsrv::TotalUpTime, 153u16 => CompinstLoginsrv::LoginFailures, 154u16 =>
    CompinstLoginsrv::Logins, 155u16 => CompinstLoginsrv::LoginsPerMinute,
};
impl Attribute for CompinstLoginsrv {
    fn class() -> Class {
        Class::CompinstLoginsrv
    }
    fn static_info(&self) -> &'static dyn AttributeInfo {
        match self {
            Self::BytesReceivedPeer => &Self::BytesReceivedPeer,
            Self::BytesReceivedServer => &Self::BytesReceivedServer,
            Self::BytesSentPeer => &Self::BytesSentPeer,
            Self::BytesSentServer => &Self::BytesSentServer,
            Self::CpuUsage => &Self::CpuUsage,
            Self::Freq => &Self::Freq,
            Self::GameDll => &Self::GameDll,
            Self::MemoryUsage => &Self::MemoryUsage,
            Self::MemoryUsagePeak => &Self::MemoryUsagePeak,
            Self::PageFaults => &Self::PageFaults,
            Self::PageFileUsage => &Self::PageFileUsage,
            Self::PageFileUsagePeak => &Self::PageFileUsagePeak,
            Self::Power => &Self::Power,
            Self::TickTime => &Self::TickTime,
            Self::TotalCpuTime => &Self::TotalCpuTime,
            Self::TotalUpTime => &Self::TotalUpTime,
            Self::LoginFailures => &Self::LoginFailures,
            Self::Logins => &Self::Logins,
            Self::LoginsPerMinute => &Self::LoginsPerMinute,
        }
    }
}
impl AttributeInfo for CompinstLoginsrv {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 156u16,
            Self::BytesReceivedServer => 157u16,
            Self::BytesSentPeer => 158u16,
            Self::BytesSentServer => 159u16,
            Self::CpuUsage => 160u16,
            Self::Freq => 170u16,
            Self::GameDll => 161u16,
            Self::MemoryUsage => 162u16,
            Self::MemoryUsagePeak => 163u16,
            Self::PageFaults => 164u16,
            Self::PageFileUsage => 165u16,
            Self::PageFileUsagePeak => 166u16,
            Self::Power => 169u16,
            Self::TickTime => 152u16,
            Self::TotalCpuTime => 167u16,
            Self::TotalUpTime => 168u16,
            Self::LoginFailures => 153u16,
            Self::Logins => 154u16,
            Self::LoginsPerMinute => 155u16,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::BytesReceivedPeer => "BytesReceivedPeer",
            Self::BytesReceivedServer => "BytesReceivedServer",
            Self::BytesSentPeer => "BytesSentPeer",
            Self::BytesSentServer => "BytesSentServer",
            Self::CpuUsage => "cpuUsage",
            Self::Freq => "Freq",
            Self::GameDll => "gameDLL",
            Self::MemoryUsage => "memoryUsage",
            Self::MemoryUsagePeak => "memoryUsagePeak",
            Self::PageFaults => "pageFaults",
            Self::PageFileUsage => "pageFileUsage",
            Self::PageFileUsagePeak => "pageFileUsagePeak",
            Self::Power => "Power",
            Self::TickTime => "tickTime",
            Self::TotalCpuTime => "totalCPUTime",
            Self::TotalUpTime => "totalUpTime",
            Self::LoginFailures => "loginFailures",
            Self::Logins => "logins",
            Self::LoginsPerMinute => "loginsPerMinute",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::LoginFailures => ParamType::Int,
            Self::Logins => ParamType::Int,
            Self::LoginsPerMinute => ParamType::Int,
            Self::BytesReceivedPeer => ParamType::Int,
            Self::BytesReceivedServer => ParamType::Int,
            Self::BytesSentPeer => ParamType::Int,
            Self::BytesSentServer => ParamType::Int,
            Self::CpuUsage => ParamType::Float,
            Self::Freq => ParamType::Int,
            Self::GameDll => ParamType::String,
            Self::MemoryUsage => ParamType::Int,
            Self::MemoryUsagePeak => ParamType::Int,
            Self::PageFaults => ParamType::Int,
            Self::PageFileUsage => ParamType::Int,
            Self::PageFileUsagePeak => ParamType::Int,
            Self::Power => ParamType::Int,
            Self::TickTime => ParamType::Float,
            Self::TotalCpuTime => ParamType::Float,
            Self::TotalUpTime => ParamType::Float,
        }
    }
    fn default(&self) -> &'static Value {
        static LOGIN_FAILURES: Value = Value::Int(0i32);
        static LOGINS: Value = Value::Int(0i32);
        static LOGINS_PER_MINUTE: Value = Value::Int(0i32);
        static BYTES_RECEIVED_PEER: Value = Value::Int(0i32);
        static BYTES_RECEIVED_SERVER: Value = Value::Int(0i32);
        static BYTES_SENT_PEER: Value = Value::Int(0i32);
        static BYTES_SENT_SERVER: Value = Value::Int(0i32);
        static CPU_USAGE: Value = Value::Float(0f32);
        static FREQ: Value = Value::Int(0i32);
        static GAME_DLL: Lazy<Value> = Lazy::new(|| Value::String("<none>".to_string()));
        static MEMORY_USAGE: Value = Value::Int(0i32);
        static MEMORY_USAGE_PEAK: Value = Value::Int(0i32);
        static PAGE_FAULTS: Value = Value::Int(0i32);
        static PAGE_FILE_USAGE: Value = Value::Int(0i32);
        static PAGE_FILE_USAGE_PEAK: Value = Value::Int(0i32);
        static POWER: Value = Value::Int(0i32);
        static TICK_TIME: Value = Value::Float(0f32);
        static TOTAL_CPU_TIME: Value = Value::Float(0f32);
        static TOTAL_UP_TIME: Value = Value::Float(0f32);
        match self {
            Self::LoginFailures => &LOGIN_FAILURES,
            Self::Logins => &LOGINS,
            Self::LoginsPerMinute => &LOGINS_PER_MINUTE,
            Self::BytesReceivedPeer => &BYTES_RECEIVED_PEER,
            Self::BytesReceivedServer => &BYTES_RECEIVED_SERVER,
            Self::BytesSentPeer => &BYTES_SENT_PEER,
            Self::BytesSentServer => &BYTES_SENT_SERVER,
            Self::CpuUsage => &CPU_USAGE,
            Self::Freq => &FREQ,
            Self::GameDll => &GAME_DLL,
            Self::MemoryUsage => &MEMORY_USAGE,
            Self::MemoryUsagePeak => &MEMORY_USAGE_PEAK,
            Self::PageFaults => &PAGE_FAULTS,
            Self::PageFileUsage => &PAGE_FILE_USAGE,
            Self::PageFileUsagePeak => &PAGE_FILE_USAGE_PEAK,
            Self::Power => &POWER,
            Self::TickTime => &TICK_TIME,
            Self::TotalCpuTime => &TOTAL_CPU_TIME,
            Self::TotalUpTime => &TOTAL_UP_TIME,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::LoginFailures => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Logins => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::LoginsPerMinute => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::BytesReceivedPeer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::BytesReceivedServer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::BytesSentPeer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::BytesSentServer => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::CpuUsage => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::Freq => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::GameDll => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                ]
            }
            Self::MemoryUsage => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::DupeSetOk,
                    ParamFlag::Content,
                    ParamFlag::Metric,
                ]
            }
            Self::MemoryUsagePeak => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::PageFaults => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::PageFileUsage => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::PageFileUsagePeak => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::Power => &[ParamFlag::Persistent, ParamFlag::Content],
            Self::TickTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::TotalCpuTime => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::TotalUpTime => {
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
impl FromStr for CompinstLoginsrv {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_LOGINSRV_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstLoginsrv {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            156u16 => Ok(Self::BytesReceivedPeer),
            157u16 => Ok(Self::BytesReceivedServer),
            158u16 => Ok(Self::BytesSentPeer),
            159u16 => Ok(Self::BytesSentServer),
            160u16 => Ok(Self::CpuUsage),
            170u16 => Ok(Self::Freq),
            161u16 => Ok(Self::GameDll),
            162u16 => Ok(Self::MemoryUsage),
            163u16 => Ok(Self::MemoryUsagePeak),
            164u16 => Ok(Self::PageFaults),
            165u16 => Ok(Self::PageFileUsage),
            166u16 => Ok(Self::PageFileUsagePeak),
            169u16 => Ok(Self::Power),
            152u16 => Ok(Self::TickTime),
            167u16 => Ok(Self::TotalCpuTime),
            168u16 => Ok(Self::TotalUpTime),
            153u16 => Ok(Self::LoginFailures),
            154u16 => Ok(Self::Logins),
            155u16 => Ok(Self::LoginsPerMinute),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
