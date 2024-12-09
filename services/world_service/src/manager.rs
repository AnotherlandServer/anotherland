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

use std::{collections::{HashMap, HashSet}, sync::Arc};

use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use realm_api::{proto::{InstanceKey, RealmClient, RealmRequest}, RealmApi, Zone};
use tokio::sync::Mutex;
use toolkit::types::{Uuid, UUID_NIL};
use zone::ZoneLabel;

use crate::{error::WorldResult, ARGS};

struct PendingInstance {
    zone: Arc<Zone>,
    valid_until: DateTime<Utc>,
}

struct InstanceManagerData {
    realm_api: RealmApi,
    realm_client: Arc<RealmClient>,
    zones: HashMap<Uuid, Arc<Zone>>,
    requests: HashMap<Uuid, PendingInstance>,
    instances: HashMap<Uuid, ZoneLabel>,
}

#[derive(Clone)]
pub struct InstanceManager(Arc<Mutex<InstanceManagerData>>);

impl InstanceManager {
    pub async fn new(realm_api: RealmApi, realm_client: Arc<RealmClient>, server: &str) -> WorldResult<Self> {
        let mut cursor = realm_api.query_zones()
            .server(server.to_string())
            .query().await?;

        let mut zones = vec![];
        while let Ok(Some(zone)) = cursor.try_next().await {
            zones.push(zone);
        }
        
        Ok(Self(Arc::new(Mutex::new(InstanceManagerData { 
            realm_api,
            realm_client,
            zones: zones.into_iter()
                .map(|zone| (*zone.guid(), Arc::new(zone)))
                .collect(),
            requests: HashMap::new(),
            instances: HashMap::new(),
        }))))
    }

    pub async fn offer_instance(
        &self,
        transaction_id: Uuid,
        zone: Uuid,
        valid_until: DateTime<Utc>,
    ) {
        let mut s = self.0.lock().await;
        if let Some(zone) = s.zones.get(&zone).cloned() {
            // Insert request to pending requests
            s.requests.insert(transaction_id, PendingInstance { 
                zone: zone.clone(), 
                valid_until
            });

            // Offer instance to realm
            let _ = s.realm_client.send(RealmRequest::InstanceOffering { 
                transaction_id,
                key: InstanceKey::new(*zone.guid(), UUID_NIL),
            }).await;
        }

        // Cleanup expired requests
        s.requests.retain(|_, req| {
            Utc::now().signed_duration_since(req.valid_until)
                .num_milliseconds() < 0
        });
    }

    pub async fn provision_instance(&self, transaction_id: Uuid) {
        let mut s = self.0.lock().await;
        
    }
}