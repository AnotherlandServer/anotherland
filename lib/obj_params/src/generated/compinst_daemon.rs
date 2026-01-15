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
pub enum CompinstDaemon {
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
    ProcKills,
    ProcStarts,
}
pub(crate) static COMPINST_DAEMON_ATTRIBUTES: phf::Map<&'static str, CompinstDaemon> = phf_map! {
    "BytesReceivedPeer" => CompinstDaemon::BytesReceivedPeer, "BytesReceivedServer" =>
    CompinstDaemon::BytesReceivedServer, "BytesSentPeer" =>
    CompinstDaemon::BytesSentPeer, "BytesSentServer" => CompinstDaemon::BytesSentServer,
    "cpuUsage" => CompinstDaemon::CpuUsage, "Freq" => CompinstDaemon::Freq, "gameDLL" =>
    CompinstDaemon::GameDll, "memoryUsage" => CompinstDaemon::MemoryUsage,
    "memoryUsagePeak" => CompinstDaemon::MemoryUsagePeak, "pageFaults" =>
    CompinstDaemon::PageFaults, "pageFileUsage" => CompinstDaemon::PageFileUsage,
    "pageFileUsagePeak" => CompinstDaemon::PageFileUsagePeak, "Power" =>
    CompinstDaemon::Power, "tickTime" => CompinstDaemon::TickTime, "totalCPUTime" =>
    CompinstDaemon::TotalCpuTime, "totalUpTime" => CompinstDaemon::TotalUpTime,
    "procKills" => CompinstDaemon::ProcKills, "procStarts" => CompinstDaemon::ProcStarts,
};
pub(crate) static COMPINST_DAEMON_ATTRIBUTES_ID: phf::Map<u16, CompinstDaemon> = phf_map! {
    105u16 => CompinstDaemon::BytesReceivedPeer, 106u16 =>
    CompinstDaemon::BytesReceivedServer, 107u16 => CompinstDaemon::BytesSentPeer, 108u16
    => CompinstDaemon::BytesSentServer, 109u16 => CompinstDaemon::CpuUsage, 119u16 =>
    CompinstDaemon::Freq, 110u16 => CompinstDaemon::GameDll, 111u16 =>
    CompinstDaemon::MemoryUsage, 112u16 => CompinstDaemon::MemoryUsagePeak, 113u16 =>
    CompinstDaemon::PageFaults, 114u16 => CompinstDaemon::PageFileUsage, 115u16 =>
    CompinstDaemon::PageFileUsagePeak, 118u16 => CompinstDaemon::Power, 102u16 =>
    CompinstDaemon::TickTime, 116u16 => CompinstDaemon::TotalCpuTime, 117u16 =>
    CompinstDaemon::TotalUpTime, 103u16 => CompinstDaemon::ProcKills, 104u16 =>
    CompinstDaemon::ProcStarts,
};
impl Attribute for CompinstDaemon {
    fn class() -> Class {
        Class::CompinstDaemon
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
            Self::ProcKills => &Self::ProcKills,
            Self::ProcStarts => &Self::ProcStarts,
        }
    }
}
impl AttributeInfo for CompinstDaemon {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 105u16,
            Self::BytesReceivedServer => 106u16,
            Self::BytesSentPeer => 107u16,
            Self::BytesSentServer => 108u16,
            Self::CpuUsage => 109u16,
            Self::Freq => 119u16,
            Self::GameDll => 110u16,
            Self::MemoryUsage => 111u16,
            Self::MemoryUsagePeak => 112u16,
            Self::PageFaults => 113u16,
            Self::PageFileUsage => 114u16,
            Self::PageFileUsagePeak => 115u16,
            Self::Power => 118u16,
            Self::TickTime => 102u16,
            Self::TotalCpuTime => 116u16,
            Self::TotalUpTime => 117u16,
            Self::ProcKills => 103u16,
            Self::ProcStarts => 104u16,
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
            Self::ProcKills => "procKills",
            Self::ProcStarts => "procStarts",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::ProcKills => ParamType::Int,
            Self::ProcStarts => ParamType::Int,
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
        static PROC_KILLS: Value = Value::Int(0i32);
        static PROC_STARTS: Value = Value::Int(0i32);
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
            Self::ProcKills => &PROC_KILLS,
            Self::ProcStarts => &PROC_STARTS,
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
            Self::ProcKills => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::ProcStarts => {
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
impl FromStr for CompinstDaemon {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_DAEMON_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstDaemon {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            105u16 => Ok(Self::BytesReceivedPeer),
            106u16 => Ok(Self::BytesReceivedServer),
            107u16 => Ok(Self::BytesSentPeer),
            108u16 => Ok(Self::BytesSentServer),
            109u16 => Ok(Self::CpuUsage),
            119u16 => Ok(Self::Freq),
            110u16 => Ok(Self::GameDll),
            111u16 => Ok(Self::MemoryUsage),
            112u16 => Ok(Self::MemoryUsagePeak),
            113u16 => Ok(Self::PageFaults),
            114u16 => Ok(Self::PageFileUsage),
            115u16 => Ok(Self::PageFileUsagePeak),
            118u16 => Ok(Self::Power),
            102u16 => Ok(Self::TickTime),
            116u16 => Ok(Self::TotalCpuTime),
            117u16 => Ok(Self::TotalUpTime),
            103u16 => Ok(Self::ProcKills),
            104u16 => Ok(Self::ProcStarts),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
