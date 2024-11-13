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

use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use toolkit::types::Uuid;
use tokio::sync::RwLock;

use crate::proto::{CoreNotification, CoreServer};

pub struct RealmStatusRegistry {
    registry: RwLock<HashMap<i32, (SocketAddr, f32)>>,
    server: Arc<CoreServer>,
}

impl RealmStatusRegistry {
    pub fn new(server: Arc<CoreServer>) -> Self {
        Self {
            registry: RwLock::new(HashMap::new()),
            server,
        }
    }

    pub async fn register_endpoint(&self, id: i32, endpoint: SocketAddr) {
        let mut registry = self.registry.write().await;
        registry.insert(id, (endpoint, 0.0));

        let _ = self.server.notify(CoreNotification::RealmListUpdated).await;
    }

    pub async fn unregister_endpoint(&self, id: i32) {
        let mut registry = self.registry.write().await;
        registry.remove(&id);

        let _ = self.server.notify(CoreNotification::RealmListUpdated).await;
    }

    pub async fn update_population(&self, id: &i32, population: f32) {
        let mut registry = self.registry.write().await;
        if let Some((_, current_pop)) = registry.get_mut(id) {
            *current_pop = population;
        }

        let _ = self.server.notify(CoreNotification::RealmListUpdated).await;
    }

    pub async fn status(&self, id: &i32) -> Option<(SocketAddr, f32)> {
        let registry = self.registry.read().await;
        registry.get(id).cloned()
    }
}