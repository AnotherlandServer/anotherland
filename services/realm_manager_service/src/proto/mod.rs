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
use std::net::SocketAddr;

pub use notification::*;

use cluster::{ClusterClient, ClusterServer, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RealmRequest {
    RegisterNode(NodeType, SocketAddr),
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

pub type RealmServer = ClusterServer<RealmRequest, RealmResponse, RealmNotification>;
pub type RealmClient = ClusterClient<RealmRequest, RealmResponse, RealmNotification>;