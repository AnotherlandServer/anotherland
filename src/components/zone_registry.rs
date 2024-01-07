// Copyright (C) 2023 AnotherlandServer
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

use std::{net::SocketAddr, collections::HashMap};

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::Uuid;

use crate::{cluster::actor::Actor, db::{ZoneDef}, util::AnotherlandResult};

pub struct ZoneRegistry {
    zones: HashMap<Uuid, SocketAddr>,
}

impl ZoneRegistry {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self {
            zones: HashMap::new(),
        })
    }
}

#[async_trait]
impl Actor for ZoneRegistry {
    fn name(&self) -> &str { "zone_registry" }
}

#[actor_actions]
impl ZoneRegistry {
    #[rpc]
    pub fn register_zone_frontend(&mut self, zone_id: Uuid, address: SocketAddr) {
        self.zones.insert(zone_id, address);
    }

    #[rpc]
    pub fn resolve_zone_address(&self, zone_id: Uuid) -> Option<SocketAddr> {
        self.zones.get(&zone_id).map(|v| v.to_owned())
    }
}