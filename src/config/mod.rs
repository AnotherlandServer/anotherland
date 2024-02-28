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

use std::net::{SocketAddrV4, SocketAddr};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfLoginServer {
    pub listen_address: SocketAddrV4,
    pub queue_listen_address: SocketAddrV4,
    pub one_time_password_duration: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfRealmServer {
    pub id: u32,
    pub name: String,
    pub listen_address: SocketAddrV4,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfFrontendServer {
    pub listen_address: SocketAddrV4,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfApiServer {
    pub listen_address: SocketAddr,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfMain {
    pub login_server: ConfLoginServer,
    pub realm: ConfRealmServer,
    pub frontend: ConfFrontendServer,
    pub api: ConfApiServer,
}

