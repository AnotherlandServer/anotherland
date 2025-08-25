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
pub enum CompinstCommunicationsrv {
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
}
pub(crate) static COMPINST_COMMUNICATIONSRV_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstCommunicationsrv,
> = phf_map! {
    "BytesReceivedPeer" => CompinstCommunicationsrv::BytesReceivedPeer,
    "BytesReceivedServer" => CompinstCommunicationsrv::BytesReceivedServer,
    "BytesSentPeer" => CompinstCommunicationsrv::BytesSentPeer, "BytesSentServer" =>
    CompinstCommunicationsrv::BytesSentServer, "cpuUsage" =>
    CompinstCommunicationsrv::CpuUsage, "Freq" => CompinstCommunicationsrv::Freq,
    "gameDLL" => CompinstCommunicationsrv::GameDll, "memoryUsage" =>
    CompinstCommunicationsrv::MemoryUsage, "memoryUsagePeak" =>
    CompinstCommunicationsrv::MemoryUsagePeak, "pageFaults" =>
    CompinstCommunicationsrv::PageFaults, "pageFileUsage" =>
    CompinstCommunicationsrv::PageFileUsage, "pageFileUsagePeak" =>
    CompinstCommunicationsrv::PageFileUsagePeak, "Power" =>
    CompinstCommunicationsrv::Power, "tickTime" => CompinstCommunicationsrv::TickTime,
    "totalCPUTime" => CompinstCommunicationsrv::TotalCpuTime, "totalUpTime" =>
    CompinstCommunicationsrv::TotalUpTime,
};
pub(crate) static COMPINST_COMMUNICATIONSRV_ATTRIBUTES_ID: phf::Map<
    u16,
    CompinstCommunicationsrv,
> = phf_map! {
    137u16 => CompinstCommunicationsrv::BytesReceivedPeer, 138u16 =>
    CompinstCommunicationsrv::BytesReceivedServer, 139u16 =>
    CompinstCommunicationsrv::BytesSentPeer, 140u16 =>
    CompinstCommunicationsrv::BytesSentServer, 141u16 =>
    CompinstCommunicationsrv::CpuUsage, 151u16 => CompinstCommunicationsrv::Freq, 142u16
    => CompinstCommunicationsrv::GameDll, 143u16 =>
    CompinstCommunicationsrv::MemoryUsage, 144u16 =>
    CompinstCommunicationsrv::MemoryUsagePeak, 145u16 =>
    CompinstCommunicationsrv::PageFaults, 146u16 =>
    CompinstCommunicationsrv::PageFileUsage, 147u16 =>
    CompinstCommunicationsrv::PageFileUsagePeak, 150u16 =>
    CompinstCommunicationsrv::Power, 136u16 => CompinstCommunicationsrv::TickTime, 148u16
    => CompinstCommunicationsrv::TotalCpuTime, 149u16 =>
    CompinstCommunicationsrv::TotalUpTime,
};
impl Attribute for CompinstCommunicationsrv {
    fn class() -> Class {
        Class::CompinstCommunicationsrv
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
        }
    }
}
impl AttributeInfo for CompinstCommunicationsrv {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 137u16,
            Self::BytesReceivedServer => 138u16,
            Self::BytesSentPeer => 139u16,
            Self::BytesSentServer => 140u16,
            Self::CpuUsage => 141u16,
            Self::Freq => 151u16,
            Self::GameDll => 142u16,
            Self::MemoryUsage => 143u16,
            Self::MemoryUsagePeak => 144u16,
            Self::PageFaults => 145u16,
            Self::PageFileUsage => 146u16,
            Self::PageFileUsagePeak => 147u16,
            Self::Power => 150u16,
            Self::TickTime => 136u16,
            Self::TotalCpuTime => 148u16,
            Self::TotalUpTime => 149u16,
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
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
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
impl FromStr for CompinstCommunicationsrv {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_COMMUNICATIONSRV_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstCommunicationsrv {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            137u16 => Ok(Self::BytesReceivedPeer),
            138u16 => Ok(Self::BytesReceivedServer),
            139u16 => Ok(Self::BytesSentPeer),
            140u16 => Ok(Self::BytesSentServer),
            141u16 => Ok(Self::CpuUsage),
            151u16 => Ok(Self::Freq),
            142u16 => Ok(Self::GameDll),
            143u16 => Ok(Self::MemoryUsage),
            144u16 => Ok(Self::MemoryUsagePeak),
            145u16 => Ok(Self::PageFaults),
            146u16 => Ok(Self::PageFileUsage),
            147u16 => Ok(Self::PageFileUsagePeak),
            150u16 => Ok(Self::Power),
            136u16 => Ok(Self::TickTime),
            148u16 => Ok(Self::TotalCpuTime),
            149u16 => Ok(Self::TotalUpTime),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
