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
pub enum CompinstMulticlusterAvatarLookup {
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
pub(crate) static COMPINST_MULTICLUSTER_AVATAR_LOOKUP_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstMulticlusterAvatarLookup,
> = phf_map! {
    "BytesReceivedPeer" => CompinstMulticlusterAvatarLookup::BytesReceivedPeer,
    "BytesReceivedServer" => CompinstMulticlusterAvatarLookup::BytesReceivedServer,
    "BytesSentPeer" => CompinstMulticlusterAvatarLookup::BytesSentPeer, "BytesSentServer"
    => CompinstMulticlusterAvatarLookup::BytesSentServer, "cpuUsage" =>
    CompinstMulticlusterAvatarLookup::CpuUsage, "Freq" =>
    CompinstMulticlusterAvatarLookup::Freq, "gameDLL" =>
    CompinstMulticlusterAvatarLookup::GameDll, "memoryUsage" =>
    CompinstMulticlusterAvatarLookup::MemoryUsage, "memoryUsagePeak" =>
    CompinstMulticlusterAvatarLookup::MemoryUsagePeak, "pageFaults" =>
    CompinstMulticlusterAvatarLookup::PageFaults, "pageFileUsage" =>
    CompinstMulticlusterAvatarLookup::PageFileUsage, "pageFileUsagePeak" =>
    CompinstMulticlusterAvatarLookup::PageFileUsagePeak, "Power" =>
    CompinstMulticlusterAvatarLookup::Power, "tickTime" =>
    CompinstMulticlusterAvatarLookup::TickTime, "totalCPUTime" =>
    CompinstMulticlusterAvatarLookup::TotalCpuTime, "totalUpTime" =>
    CompinstMulticlusterAvatarLookup::TotalUpTime,
};
pub(crate) static COMPINST_MULTICLUSTER_AVATAR_LOOKUP_ATTRIBUTES_ID: phf::Map<
    u16,
    CompinstMulticlusterAvatarLookup,
> = phf_map! {
    85u16 => CompinstMulticlusterAvatarLookup::BytesReceivedPeer, 84u16 =>
    CompinstMulticlusterAvatarLookup::BytesReceivedServer, 83u16 =>
    CompinstMulticlusterAvatarLookup::BytesSentPeer, 82u16 =>
    CompinstMulticlusterAvatarLookup::BytesSentServer, 81u16 =>
    CompinstMulticlusterAvatarLookup::CpuUsage, 71u16 =>
    CompinstMulticlusterAvatarLookup::Freq, 80u16 =>
    CompinstMulticlusterAvatarLookup::GameDll, 79u16 =>
    CompinstMulticlusterAvatarLookup::MemoryUsage, 78u16 =>
    CompinstMulticlusterAvatarLookup::MemoryUsagePeak, 77u16 =>
    CompinstMulticlusterAvatarLookup::PageFaults, 76u16 =>
    CompinstMulticlusterAvatarLookup::PageFileUsage, 75u16 =>
    CompinstMulticlusterAvatarLookup::PageFileUsagePeak, 72u16 =>
    CompinstMulticlusterAvatarLookup::Power, 70u16 =>
    CompinstMulticlusterAvatarLookup::TickTime, 74u16 =>
    CompinstMulticlusterAvatarLookup::TotalCpuTime, 73u16 =>
    CompinstMulticlusterAvatarLookup::TotalUpTime,
};
impl Attribute for CompinstMulticlusterAvatarLookup {
    fn class() -> Class {
        Class::CompinstMulticlusterAvatarLookup
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
impl AttributeInfo for CompinstMulticlusterAvatarLookup {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 85u16,
            Self::BytesReceivedServer => 84u16,
            Self::BytesSentPeer => 83u16,
            Self::BytesSentServer => 82u16,
            Self::CpuUsage => 81u16,
            Self::Freq => 71u16,
            Self::GameDll => 80u16,
            Self::MemoryUsage => 79u16,
            Self::MemoryUsagePeak => 78u16,
            Self::PageFaults => 77u16,
            Self::PageFileUsage => 76u16,
            Self::PageFileUsagePeak => 75u16,
            Self::Power => 72u16,
            Self::TickTime => 70u16,
            Self::TotalCpuTime => 74u16,
            Self::TotalUpTime => 73u16,
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
impl FromStr for CompinstMulticlusterAvatarLookup {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_MULTICLUSTER_AVATAR_LOOKUP_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstMulticlusterAvatarLookup {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            85u16 => Ok(Self::BytesReceivedPeer),
            84u16 => Ok(Self::BytesReceivedServer),
            83u16 => Ok(Self::BytesSentPeer),
            82u16 => Ok(Self::BytesSentServer),
            81u16 => Ok(Self::CpuUsage),
            71u16 => Ok(Self::Freq),
            80u16 => Ok(Self::GameDll),
            79u16 => Ok(Self::MemoryUsage),
            78u16 => Ok(Self::MemoryUsagePeak),
            77u16 => Ok(Self::PageFaults),
            76u16 => Ok(Self::PageFileUsage),
            75u16 => Ok(Self::PageFileUsagePeak),
            72u16 => Ok(Self::Power),
            70u16 => Ok(Self::TickTime),
            74u16 => Ok(Self::TotalCpuTime),
            73u16 => Ok(Self::TotalUpTime),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
