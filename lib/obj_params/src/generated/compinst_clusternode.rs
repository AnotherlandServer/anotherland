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
pub enum CompinstClusternode {
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
    ClusterNodeName,
    SparseSeed,
    SparseUpdate,
    TickNoSleep,
}
pub(crate) static COMPINST_CLUSTERNODE_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstClusternode,
> = phf_map! {
    "BytesReceivedPeer" => CompinstClusternode::BytesReceivedPeer, "BytesReceivedServer"
    => CompinstClusternode::BytesReceivedServer, "BytesSentPeer" =>
    CompinstClusternode::BytesSentPeer, "BytesSentServer" =>
    CompinstClusternode::BytesSentServer, "cpuUsage" => CompinstClusternode::CpuUsage,
    "Freq" => CompinstClusternode::Freq, "gameDLL" => CompinstClusternode::GameDll,
    "memoryUsage" => CompinstClusternode::MemoryUsage, "memoryUsagePeak" =>
    CompinstClusternode::MemoryUsagePeak, "pageFaults" =>
    CompinstClusternode::PageFaults, "pageFileUsage" =>
    CompinstClusternode::PageFileUsage, "pageFileUsagePeak" =>
    CompinstClusternode::PageFileUsagePeak, "Power" => CompinstClusternode::Power,
    "tickTime" => CompinstClusternode::TickTime, "totalCPUTime" =>
    CompinstClusternode::TotalCpuTime, "totalUpTime" => CompinstClusternode::TotalUpTime,
    "clusterName" => CompinstClusternode::ClusterName, "clusterNodeName" =>
    CompinstClusternode::ClusterNodeName, "sparseSeed" =>
    CompinstClusternode::SparseSeed, "sparseUpdate" => CompinstClusternode::SparseUpdate,
    "tickNoSleep" => CompinstClusternode::TickNoSleep,
};
pub(crate) static COMPINST_CLUSTERNODE_ATTRIBUTES_ID: phf::Map<
    u16,
    CompinstClusternode,
> = phf_map! {
    194u16 => CompinstClusternode::BytesReceivedPeer, 195u16 =>
    CompinstClusternode::BytesReceivedServer, 196u16 =>
    CompinstClusternode::BytesSentPeer, 197u16 => CompinstClusternode::BytesSentServer,
    198u16 => CompinstClusternode::CpuUsage, 208u16 => CompinstClusternode::Freq, 199u16
    => CompinstClusternode::GameDll, 200u16 => CompinstClusternode::MemoryUsage, 201u16
    => CompinstClusternode::MemoryUsagePeak, 202u16 => CompinstClusternode::PageFaults,
    203u16 => CompinstClusternode::PageFileUsage, 204u16 =>
    CompinstClusternode::PageFileUsagePeak, 207u16 => CompinstClusternode::Power, 188u16
    => CompinstClusternode::TickTime, 205u16 => CompinstClusternode::TotalCpuTime, 206u16
    => CompinstClusternode::TotalUpTime, 193u16 => CompinstClusternode::ClusterName,
    192u16 => CompinstClusternode::ClusterNodeName, 189u16 =>
    CompinstClusternode::SparseSeed, 190u16 => CompinstClusternode::SparseUpdate, 191u16
    => CompinstClusternode::TickNoSleep,
};
impl Attribute for CompinstClusternode {
    fn class() -> Class {
        Class::CompinstClusternode
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
            Self::ClusterNodeName => &Self::ClusterNodeName,
            Self::SparseSeed => &Self::SparseSeed,
            Self::SparseUpdate => &Self::SparseUpdate,
            Self::TickNoSleep => &Self::TickNoSleep,
        }
    }
}
impl AttributeInfo for CompinstClusternode {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 194u16,
            Self::BytesReceivedServer => 195u16,
            Self::BytesSentPeer => 196u16,
            Self::BytesSentServer => 197u16,
            Self::CpuUsage => 198u16,
            Self::Freq => 208u16,
            Self::GameDll => 199u16,
            Self::MemoryUsage => 200u16,
            Self::MemoryUsagePeak => 201u16,
            Self::PageFaults => 202u16,
            Self::PageFileUsage => 203u16,
            Self::PageFileUsagePeak => 204u16,
            Self::Power => 207u16,
            Self::TickTime => 188u16,
            Self::TotalCpuTime => 205u16,
            Self::TotalUpTime => 206u16,
            Self::ClusterName => 193u16,
            Self::ClusterNodeName => 192u16,
            Self::SparseSeed => 189u16,
            Self::SparseUpdate => 190u16,
            Self::TickNoSleep => 191u16,
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
            Self::ClusterNodeName => "clusterNodeName",
            Self::SparseSeed => "sparseSeed",
            Self::SparseUpdate => "sparseUpdate",
            Self::TickNoSleep => "tickNoSleep",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::ClusterNodeName => ParamType::String,
            Self::SparseSeed => ParamType::Int,
            Self::SparseUpdate => ParamType::Bool,
            Self::TickNoSleep => ParamType::Float,
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
            Self::ClusterName => ParamType::String,
        }
    }
    fn default(&self) -> &'static Value {
        static CLUSTER_NODE_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        static SPARSE_SEED: Value = Value::Int(0i32);
        static SPARSE_UPDATE: Value = Value::Bool(false);
        static TICK_NO_SLEEP: Value = Value::Float(0f32);
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
        static CLUSTER_NAME: Lazy<Value> = Lazy::new(|| Value::String(
            String::default(),
        ));
        match self {
            Self::ClusterNodeName => &CLUSTER_NODE_NAME,
            Self::SparseSeed => &SPARSE_SEED,
            Self::SparseUpdate => &SPARSE_UPDATE,
            Self::TickNoSleep => &TICK_NO_SLEEP,
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
            Self::ClusterName => &CLUSTER_NAME,
        }
    }
    fn flags(&self) -> &[ParamFlag] {
        match self {
            Self::ClusterNodeName => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::SparseSeed => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::SparseUpdate => {
                &[
                    ParamFlag::NodeOwn,
                    ParamFlag::ServerOwn,
                    ParamFlag::Persistent,
                    ParamFlag::Content,
                ]
            }
            Self::TickNoSleep => {
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
            Self::ClusterName => {
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
impl FromStr for CompinstClusternode {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_CLUSTERNODE_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstClusternode {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            194u16 => Ok(Self::BytesReceivedPeer),
            195u16 => Ok(Self::BytesReceivedServer),
            196u16 => Ok(Self::BytesSentPeer),
            197u16 => Ok(Self::BytesSentServer),
            198u16 => Ok(Self::CpuUsage),
            208u16 => Ok(Self::Freq),
            199u16 => Ok(Self::GameDll),
            200u16 => Ok(Self::MemoryUsage),
            201u16 => Ok(Self::MemoryUsagePeak),
            202u16 => Ok(Self::PageFaults),
            203u16 => Ok(Self::PageFileUsage),
            204u16 => Ok(Self::PageFileUsagePeak),
            207u16 => Ok(Self::Power),
            188u16 => Ok(Self::TickTime),
            205u16 => Ok(Self::TotalCpuTime),
            206u16 => Ok(Self::TotalUpTime),
            193u16 => Ok(Self::ClusterName),
            192u16 => Ok(Self::ClusterNodeName),
            189u16 => Ok(Self::SparseSeed),
            190u16 => Ok(Self::SparseUpdate),
            191u16 => Ok(Self::TickNoSleep),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
