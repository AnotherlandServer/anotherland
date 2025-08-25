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
pub enum CompinstClustersrv {
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
    SparseSeed,
    SparseUpdate,
    SrvResendQueue,
}
pub(crate) static COMPINST_CLUSTERSRV_ATTRIBUTES: phf::Map<
    &'static str,
    CompinstClustersrv,
> = phf_map! {
    "BytesReceivedPeer" => CompinstClustersrv::BytesReceivedPeer, "BytesReceivedServer"
    => CompinstClustersrv::BytesReceivedServer, "BytesSentPeer" =>
    CompinstClustersrv::BytesSentPeer, "BytesSentServer" =>
    CompinstClustersrv::BytesSentServer, "cpuUsage" => CompinstClustersrv::CpuUsage,
    "Freq" => CompinstClustersrv::Freq, "gameDLL" => CompinstClustersrv::GameDll,
    "memoryUsage" => CompinstClustersrv::MemoryUsage, "memoryUsagePeak" =>
    CompinstClustersrv::MemoryUsagePeak, "pageFaults" => CompinstClustersrv::PageFaults,
    "pageFileUsage" => CompinstClustersrv::PageFileUsage, "pageFileUsagePeak" =>
    CompinstClustersrv::PageFileUsagePeak, "Power" => CompinstClustersrv::Power,
    "tickTime" => CompinstClustersrv::TickTime, "totalCPUTime" =>
    CompinstClustersrv::TotalCpuTime, "totalUpTime" => CompinstClustersrv::TotalUpTime,
    "clusterName" => CompinstClustersrv::ClusterName, "sparseSeed" =>
    CompinstClustersrv::SparseSeed, "sparseUpdate" => CompinstClustersrv::SparseUpdate,
    "srvResendQueue" => CompinstClustersrv::SrvResendQueue,
};
pub(crate) static COMPINST_CLUSTERSRV_ATTRIBUTES_ID: phf::Map<u16, CompinstClustersrv> = phf_map! {
    214u16 => CompinstClustersrv::BytesReceivedPeer, 215u16 =>
    CompinstClustersrv::BytesReceivedServer, 216u16 => CompinstClustersrv::BytesSentPeer,
    217u16 => CompinstClustersrv::BytesSentServer, 218u16 =>
    CompinstClustersrv::CpuUsage, 228u16 => CompinstClustersrv::Freq, 219u16 =>
    CompinstClustersrv::GameDll, 220u16 => CompinstClustersrv::MemoryUsage, 221u16 =>
    CompinstClustersrv::MemoryUsagePeak, 222u16 => CompinstClustersrv::PageFaults, 223u16
    => CompinstClustersrv::PageFileUsage, 224u16 =>
    CompinstClustersrv::PageFileUsagePeak, 227u16 => CompinstClustersrv::Power, 209u16 =>
    CompinstClustersrv::TickTime, 225u16 => CompinstClustersrv::TotalCpuTime, 226u16 =>
    CompinstClustersrv::TotalUpTime, 213u16 => CompinstClustersrv::ClusterName, 210u16 =>
    CompinstClustersrv::SparseSeed, 211u16 => CompinstClustersrv::SparseUpdate, 212u16 =>
    CompinstClustersrv::SrvResendQueue,
};
impl Attribute for CompinstClustersrv {
    fn class() -> Class {
        Class::CompinstClustersrv
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
            Self::SparseSeed => &Self::SparseSeed,
            Self::SparseUpdate => &Self::SparseUpdate,
            Self::SrvResendQueue => &Self::SrvResendQueue,
        }
    }
}
impl AttributeInfo for CompinstClustersrv {
    fn class(&self) -> Class {
        <Self as Attribute>::class()
    }
    fn id(&self) -> u16 {
        match self {
            Self::BytesReceivedPeer => 214u16,
            Self::BytesReceivedServer => 215u16,
            Self::BytesSentPeer => 216u16,
            Self::BytesSentServer => 217u16,
            Self::CpuUsage => 218u16,
            Self::Freq => 228u16,
            Self::GameDll => 219u16,
            Self::MemoryUsage => 220u16,
            Self::MemoryUsagePeak => 221u16,
            Self::PageFaults => 222u16,
            Self::PageFileUsage => 223u16,
            Self::PageFileUsagePeak => 224u16,
            Self::Power => 227u16,
            Self::TickTime => 209u16,
            Self::TotalCpuTime => 225u16,
            Self::TotalUpTime => 226u16,
            Self::ClusterName => 213u16,
            Self::SparseSeed => 210u16,
            Self::SparseUpdate => 211u16,
            Self::SrvResendQueue => 212u16,
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
            Self::SparseSeed => "sparseSeed",
            Self::SparseUpdate => "sparseUpdate",
            Self::SrvResendQueue => "srvResendQueue",
        }
    }
    fn datatype(&self) -> ParamType {
        match self {
            Self::SparseSeed => ParamType::Int,
            Self::SparseUpdate => ParamType::Bool,
            Self::SrvResendQueue => ParamType::Int,
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
        static SPARSE_SEED: Value = Value::Int(0i32);
        static SPARSE_UPDATE: Value = Value::Bool(false);
        static SRV_RESEND_QUEUE: Value = Value::Int(0i32);
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
            Self::SparseSeed => &SPARSE_SEED,
            Self::SparseUpdate => &SPARSE_UPDATE,
            Self::SrvResendQueue => &SRV_RESEND_QUEUE,
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
            Self::SrvResendQueue => {
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
impl FromStr for CompinstClustersrv {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COMPINST_CLUSTERSRV_ATTRIBUTES
            .get(s)
            .map(|v| *v)
            .ok_or(ParamError::UnknownAttributeName)
    }
}
impl TryFrom<u16> for CompinstClustersrv {
    type Error = ParamError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            214u16 => Ok(Self::BytesReceivedPeer),
            215u16 => Ok(Self::BytesReceivedServer),
            216u16 => Ok(Self::BytesSentPeer),
            217u16 => Ok(Self::BytesSentServer),
            218u16 => Ok(Self::CpuUsage),
            228u16 => Ok(Self::Freq),
            219u16 => Ok(Self::GameDll),
            220u16 => Ok(Self::MemoryUsage),
            221u16 => Ok(Self::MemoryUsagePeak),
            222u16 => Ok(Self::PageFaults),
            223u16 => Ok(Self::PageFileUsage),
            224u16 => Ok(Self::PageFileUsagePeak),
            227u16 => Ok(Self::Power),
            209u16 => Ok(Self::TickTime),
            225u16 => Ok(Self::TotalCpuTime),
            226u16 => Ok(Self::TotalUpTime),
            213u16 => Ok(Self::ClusterName),
            210u16 => Ok(Self::SparseSeed),
            211u16 => Ok(Self::SparseUpdate),
            212u16 => Ok(Self::SrvResendQueue),
            _ => Err(ParamError::UnknownAttributeId),
        }
    }
}
