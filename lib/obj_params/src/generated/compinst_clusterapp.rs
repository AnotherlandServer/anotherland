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
pub enum CompinstClusterapp {
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
    ClusterName,
}
pub(crate) static COMPINST_CLUSTERAPP_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstClusterapp,
> = phf_map! {
    "BytesReceivedPeer" => CompinstClusterapp::BytesReceivedPeer, "BytesReceivedServer"
    => CompinstClusterapp::BytesReceivedServer, "BytesSentPeer" =>
    CompinstClusterapp::BytesSentPeer, "BytesSentServer" =>
    CompinstClusterapp::BytesSentServer, "cpuUsage" => CompinstClusterapp::CpuUsage,
    "Freq" => CompinstClusterapp::Freq, "gameDLL" => CompinstClusterapp::GameDll,
    "memoryUsage" => CompinstClusterapp::MemoryUsage, "memoryUsagePeak" =>
    CompinstClusterapp::MemoryUsagePeak, "pageFaults" => CompinstClusterapp::PageFaults,
    "pageFileUsage" => CompinstClusterapp::PageFileUsage, "pageFileUsagePeak" =>
    CompinstClusterapp::PageFileUsagePeak, "Power" => CompinstClusterapp::Power,
    "tickTime" => CompinstClusterapp::TickTime, "totalCPUTime" =>
    CompinstClusterapp::TotalCpuTime, "totalUpTime" => CompinstClusterapp::TotalUpTime,
    "clusterName" => CompinstClusterapp::ClusterName,
};
pub(crate) static COMPINST_CLUSTERAPP_ATTRIBUTES_ID: phf::Map<u16, CompinstClusterapp> = phf_map! {
    173u16 => CompinstClusterapp::BytesReceivedPeer, 174u16 =>
    CompinstClusterapp::BytesReceivedServer, 175u16 => CompinstClusterapp::BytesSentPeer,
    176u16 => CompinstClusterapp::BytesSentServer, 177u16 =>
    CompinstClusterapp::CpuUsage, 187u16 => CompinstClusterapp::Freq, 178u16 =>
    CompinstClusterapp::GameDll, 179u16 => CompinstClusterapp::MemoryUsage, 180u16 =>
    CompinstClusterapp::MemoryUsagePeak, 181u16 => CompinstClusterapp::PageFaults, 182u16
    => CompinstClusterapp::PageFileUsage, 183u16 =>
    CompinstClusterapp::PageFileUsagePeak, 186u16 => CompinstClusterapp::Power, 171u16 =>
    CompinstClusterapp::TickTime, 184u16 => CompinstClusterapp::TotalCpuTime, 185u16 =>
    CompinstClusterapp::TotalUpTime, 172u16 => CompinstClusterapp::ClusterName,
};
impl Attribute for CompinstClusterapp {
    fn class() -> Class {
        Class::CompinstClusterapp
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
            Self::ClusterName => &Self::ClusterName,
        }
    }
}
impl AttributeInfo for CompinstClusterapp {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 173u16,
            Self::BytesReceivedServer => 174u16,
            Self::BytesSentPeer => 175u16,
            Self::BytesSentServer => 176u16,
            Self::CpuUsage => 177u16,
            Self::Freq => 187u16,
            Self::GameDll => 178u16,
            Self::MemoryUsage => 179u16,
            Self::MemoryUsagePeak => 180u16,
            Self::PageFaults => 181u16,
            Self::PageFileUsage => 182u16,
            Self::PageFileUsagePeak => 183u16,
            Self::Power => 186u16,
            Self::TickTime => 171u16,
            Self::TotalCpuTime => 184u16,
            Self::TotalUpTime => 185u16,
            Self::ClusterName => 172u16,
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
            Self::ClusterName => "clusterName",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::ClusterName => ParamType::String,
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
        static CLUSTER_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
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
            Self::ClusterName => &CLUSTER_NAME,
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
            Self::ClusterName => {
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
impl FromStr for CompinstClusterapp {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_CLUSTERAPP_ATTRIBUTES
            .get(s)
            .copied()
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstClusterapp {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            173u16 => Ok(Self::BytesReceivedPeer),
            174u16 => Ok(Self::BytesReceivedServer),
            175u16 => Ok(Self::BytesSentPeer),
            176u16 => Ok(Self::BytesSentServer),
            177u16 => Ok(Self::CpuUsage),
            187u16 => Ok(Self::Freq),
            178u16 => Ok(Self::GameDll),
            179u16 => Ok(Self::MemoryUsage),
            180u16 => Ok(Self::MemoryUsagePeak),
            181u16 => Ok(Self::PageFaults),
            182u16 => Ok(Self::PageFileUsage),
            183u16 => Ok(Self::PageFileUsagePeak),
            186u16 => Ok(Self::Power),
            171u16 => Ok(Self::TickTime),
            184u16 => Ok(Self::TotalCpuTime),
            185u16 => Ok(Self::TotalUpTime),
            172u16 => Ok(Self::ClusterName),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
