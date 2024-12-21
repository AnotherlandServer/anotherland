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

use cluster::{ClusterClient, ClusterServer, Notification, Request, Response};
use protocol::CPkt;
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

#[derive(Serialize, Deserialize)]
pub enum WorldRequest {
    ClientMessage { peer: Uuid, data: Vec<u8> },
    ClientConnected { peer: Uuid, session: Uuid, zone: Uuid, instance: Option<Uuid> },
    ClientDisconnected { peer: Uuid },
}

impl Request for WorldRequest {}

#[derive(Serialize, Deserialize)]
pub enum WorldResponse {
    ServerMessage{ peer: Uuid, data: Vec<u8> },
    Travel { peer: Uuid, data: () }
}

impl Response for WorldResponse {}

#[derive(Serialize, Deserialize)]
pub enum WorldNotification {

}

impl Notification for WorldNotification {
    fn topic_name(&self) -> &'static str {
        unreachable!()
    }
}

pub type WorldServer = ClusterServer<WorldRequest, WorldResponse, WorldNotification>;
pub type WorldClient = ClusterClient<WorldRequest, WorldResponse, WorldNotification>;