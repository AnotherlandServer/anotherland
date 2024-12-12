// Copyright (C) 2024 AnotherlandServer
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

mod notification;
use std::{fmt::{Debug, Display}, net::SocketAddr};

use async_graphql::Enum;
use chrono::{DateTime, Utc};
pub use notification::*;

use cluster::{ClusterClient, ClusterServer, Request, Response};
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct InstanceKey(Uuid, Option<Uuid>);

impl InstanceKey {
    pub fn new(zone: Uuid, instance: Option<Uuid>) -> Self {
        Self(zone, instance)
    }

    pub fn zone(&self) -> Uuid { self.0 }
    pub fn instance(&self) -> Option<Uuid> { self.1 }
}

impl Display for InstanceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(key) = self.1 {
            f.write_fmt(format_args!("({}, {})", self.0, key))
        } else {
            f.write_fmt(format_args!("({})", self.0))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum NodeAddress {
    Public(SocketAddr),
    Internal(u16),
}

impl Display for NodeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize)]
pub enum RealmRequest {
    RegisterNode(NodeType, NodeAddress),
    InstanceOffering {
        transaction_id: Uuid,
        key: InstanceKey,
    },
    InstanceProvisioned {
        transaction_id: Uuid
    },
    RemoveInstance(InstanceKey),
}

impl Request for RealmRequest {}


#[derive(Serialize, Deserialize)]
pub enum RealmResponse {
    InstanceOfferingAccepted { 
        transaction_id: Uuid,
        key: InstanceKey 
    },
}

impl Response for RealmResponse {}

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Frontend,
    Cluster,
    World,
    Dungeon,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Frontend => f.write_str("Frontend"),
            NodeType::Cluster => f.write_str("Cluster"),
            NodeType::World => f.write_str("World"),
            NodeType::Dungeon => f.write_str("Dungeon"),
        }
    }
}

pub type RealmServer = ClusterServer<RealmRequest, RealmResponse, RealmNotification>;
pub type RealmClient = ClusterClient<RealmRequest, RealmResponse, RealmNotification>;