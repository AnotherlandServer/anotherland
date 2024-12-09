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
use std::{fmt::Display, net::SocketAddr};

pub use notification::*;

use cluster::{ClusterClient, ClusterServer, Request, Response};
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

#[derive(Serialize, Deserialize)]
pub enum RealmRequest {
    RegisterNode(NodeType, SocketAddr),
    RegisterInstance {
        zone: Uuid,
        instance: Uuid,
    }
}

impl Request for RealmRequest {}


#[derive(Serialize, Deserialize)]
pub enum RealmResponse {

}

impl Response for RealmResponse {}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum NodeType {
    FrontendNode,
    ClusterNode,
    WorldNode,
    DungeonNode,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::FrontendNode => f.write_str("FrontendNode"),
            NodeType::ClusterNode => f.write_str("ClusterNode"),
            NodeType::WorldNode => f.write_str("WorldNode"),
            NodeType::DungeonNode => f.write_str("DungeonNode"),
        }
    }
}

pub type RealmServer = ClusterServer<RealmRequest, RealmResponse, RealmNotification>;
pub type RealmClient = ClusterClient<RealmRequest, RealmResponse, RealmNotification>;