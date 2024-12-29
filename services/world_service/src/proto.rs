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

use bevy::math::Vec3;
use cluster::{ClusterClient, ClusterServer, Notification, Request, Response};
use protocol::CPkt;
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

#[derive(Serialize, Deserialize)]
pub enum WorldRequest {
    RouterChannel { id: Uuid, msg: ClusterMessage }
}

#[derive(Serialize, Deserialize)]
pub enum ClusterMessage {
    Forward { data: Vec<u8> },
    ClientArrived { session: Uuid, zone: Uuid, instance: Option<Uuid>, mode: TravelMode },
    ClientLeft,
    TravelAccepted,
    TravelRejected { reason: TravelRejectReason },
}

impl Request for WorldRequest {}

#[derive(Serialize, Deserialize)]
pub enum WorldResponse {
    RouterChannel { id: Uuid, msg: WorldMessage },
}

#[derive(Serialize, Deserialize)]
pub enum WorldMessage {
    ServerMessage{ data: Vec<u8> },
    TravelRequest { zone: Uuid, instance: Option<Uuid>, mode: TravelMode },
    TravelCommited,
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TravelRejectReason {
    ZoneOffline,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TravelMode {
    Login,
    Portal { uuid: Uuid },
    Position { pos: Vec3, rot: Vec3 },
    EntryPoint,
}