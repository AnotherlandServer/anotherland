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
pub enum CompinstMasterRedirectSrv {
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
pub(crate) static COMPINST_MASTER_REDIRECT_SRV_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstMasterRedirectSrv,
> = phf_map! {
    "BytesReceivedPeer" => CompinstMasterRedirectSrv::BytesReceivedPeer,
    "BytesReceivedServer" => CompinstMasterRedirectSrv::BytesReceivedServer,
    "BytesSentPeer" => CompinstMasterRedirectSrv::BytesSentPeer, "BytesSentServer" =>
    CompinstMasterRedirectSrv::BytesSentServer, "cpuUsage" =>
    CompinstMasterRedirectSrv::CpuUsage, "Freq" => CompinstMasterRedirectSrv::Freq,
    "gameDLL" => CompinstMasterRedirectSrv::GameDll, "memoryUsage" =>
    CompinstMasterRedirectSrv::MemoryUsage, "memoryUsagePeak" =>
    CompinstMasterRedirectSrv::MemoryUsagePeak, "pageFaults" =>
    CompinstMasterRedirectSrv::PageFaults, "pageFileUsage" =>
    CompinstMasterRedirectSrv::PageFileUsage, "pageFileUsagePeak" =>
    CompinstMasterRedirectSrv::PageFileUsagePeak, "Power" =>
    CompinstMasterRedirectSrv::Power, "tickTime" => CompinstMasterRedirectSrv::TickTime,
    "totalCPUTime" => CompinstMasterRedirectSrv::TotalCpuTime, "totalUpTime" =>
    CompinstMasterRedirectSrv::TotalUpTime,
};
pub(crate) static COMPINST_MASTER_REDIRECT_SRV_ATTRIBUTES_ID: phf::Map<
    u16,
    CompinstMasterRedirectSrv,
> = phf_map! {
    5528u16 => CompinstMasterRedirectSrv::BytesReceivedPeer, 5527u16 =>
    CompinstMasterRedirectSrv::BytesReceivedServer, 5526u16 =>
    CompinstMasterRedirectSrv::BytesSentPeer, 5525u16 =>
    CompinstMasterRedirectSrv::BytesSentServer, 5524u16 =>
    CompinstMasterRedirectSrv::CpuUsage, 5514u16 => CompinstMasterRedirectSrv::Freq,
    5523u16 => CompinstMasterRedirectSrv::GameDll, 5522u16 =>
    CompinstMasterRedirectSrv::MemoryUsage, 5521u16 =>
    CompinstMasterRedirectSrv::MemoryUsagePeak, 5520u16 =>
    CompinstMasterRedirectSrv::PageFaults, 5519u16 =>
    CompinstMasterRedirectSrv::PageFileUsage, 5518u16 =>
    CompinstMasterRedirectSrv::PageFileUsagePeak, 5515u16 =>
    CompinstMasterRedirectSrv::Power, 5529u16 => CompinstMasterRedirectSrv::TickTime,
    5517u16 => CompinstMasterRedirectSrv::TotalCpuTime, 5516u16 =>
    CompinstMasterRedirectSrv::TotalUpTime,
};
impl Attribute for CompinstMasterRedirectSrv {
    fn class() -> Class {
        Class::CompinstMasterRedirectSrv
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
impl AttributeInfo for CompinstMasterRedirectSrv {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 5528u16,
            Self::BytesReceivedServer => 5527u16,
            Self::BytesSentPeer => 5526u16,
            Self::BytesSentServer => 5525u16,
            Self::CpuUsage => 5524u16,
            Self::Freq => 5514u16,
            Self::GameDll => 5523u16,
            Self::MemoryUsage => 5522u16,
            Self::MemoryUsagePeak => 5521u16,
            Self::PageFaults => 5520u16,
            Self::PageFileUsage => 5519u16,
            Self::PageFileUsagePeak => 5518u16,
            Self::Power => 5515u16,
            Self::TickTime => 5529u16,
            Self::TotalCpuTime => 5517u16,
            Self::TotalUpTime => 5516u16,
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
impl FromStr for CompinstMasterRedirectSrv {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_MASTER_REDIRECT_SRV_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstMasterRedirectSrv {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            5528u16 => Ok(Self::BytesReceivedPeer),
            5527u16 => Ok(Self::BytesReceivedServer),
            5526u16 => Ok(Self::BytesSentPeer),
            5525u16 => Ok(Self::BytesSentServer),
            5524u16 => Ok(Self::CpuUsage),
            5514u16 => Ok(Self::Freq),
            5523u16 => Ok(Self::GameDll),
            5522u16 => Ok(Self::MemoryUsage),
            5521u16 => Ok(Self::MemoryUsagePeak),
            5520u16 => Ok(Self::PageFaults),
            5519u16 => Ok(Self::PageFileUsage),
            5518u16 => Ok(Self::PageFileUsagePeak),
            5515u16 => Ok(Self::Power),
            5529u16 => Ok(Self::TickTime),
            5517u16 => Ok(Self::TotalCpuTime),
            5516u16 => Ok(Self::TotalUpTime),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
