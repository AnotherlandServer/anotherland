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

use bevy::{app::{App, AppExit, Plugin, SubApp}, MinimalPlugins};
use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use log::{debug, info};
use obj_params::OaZoneConfig;
use protocol::CPkt;
use realm_api::{proto::{InstanceKey, RealmClient, RealmRequest}, ObjectTemplate, RealmApi, Zone};
use tokio::{runtime::Handle, sync::{mpsc::{self, Sender, UnboundedSender}, oneshot, Mutex}};
use toolkit::types::{Uuid, UUID_NIL};

use crate::{error::WorldResult, instance::{InstanceLabel, ZoneInstanceBuilder, ZoneSubApp}, plugins::{ControllerEvent, WorldEvent}, ARGS};

struct PendingInstance {
    zone: Arc<Zone>,
    key: Option<Uuid>,
    valid_until: DateTime<Utc>,
}

struct InstanceManagerData {
    realm_api: RealmApi,
    realm_client: Arc<RealmClient>,
    zones: HashMap<Uuid, Arc<Zone>>,
    requests: HashMap<Uuid, PendingInstance>,
    instances: HashMap<Uuid, InstanceLabel>,
    event_sender: mpsc::Sender<InstanceEvent>,
    limit: usize,
}

pub enum InstanceEvent {
    InstanceAdded(Box<SubApp>),
    InstanceRemoved(InstanceLabel),
    ControllerSpawnRequested {
        peer: Uuid,
        instance: InstanceLabel,
        session: Uuid,
        events: UnboundedSender<WorldEvent>,
        controller: oneshot::Sender<WorldResult<Sender<ControllerEvent>>>
    }
}

#[derive(Clone)]
pub struct InstanceManager(Arc<Mutex<InstanceManagerData>>);

impl InstanceManager {
    pub async fn new(realm_api: RealmApi, realm_client: Arc<RealmClient>, event_sender: mpsc::Sender<InstanceEvent>, limit: usize, groups: &[&str]) -> WorldResult<Self> {
        let mut zones = vec![];
        
        if groups.is_empty() {
            let mut cursor = realm_api.query_zones()
                .query().await?;
            
            while let Ok(Some(zone)) = cursor.try_next().await {
                debug!("Serving zone: {:?}", zone.guid());
                zones.push(zone);
            }
            
            info!("Serving all zones...");
        } else {
            while let Some(server) = groups.iter().next() {
                let mut cursor = realm_api.query_zones()
                    .server(server.to_string())
                    .query().await?;
                
                while let Ok(Some(zone)) = cursor.try_next().await {
                    zones.push(zone);
                }
            }

            info!("Serving zones for groups: {:?}", groups);
        }
        
        Ok(Self(Arc::new(Mutex::new(InstanceManagerData { 
            realm_api,
            realm_client,
            zones: zones.into_iter()
                .map(|zone| (*zone.guid(), Arc::new(zone)))
                .collect(),
            requests: HashMap::new(),
            instances: HashMap::new(),
            event_sender,
            limit,
        }))))
    }

    pub async fn offer_instance(
        &self,
        transaction_id: Uuid,
        zone: Uuid,
        mut key: Option<Uuid>,
        valid_until: DateTime<Utc>,
    ) -> WorldResult<()> {
        let mut s = self.0.lock().await;
        if s.instances.len() >= s.limit { return Ok(()); } // Exit early if instance limit is hit

        debug!("Got instance request for zone {} with key {:?} until {}", zone, key, valid_until);

        if let Some(zone) = s.zones.get(&zone).cloned() {
            if key.is_none() && !zone.realu_zone_type().is_empty() {
                let conf = s.realm_api.query_object_templates()
                    .name(zone.realu_zone_type().to_string())
                    .query().await?
                    .try_next().await?;

                if let Some(conf) = conf {
                    if *conf.data.get(OaZoneConfig::UseGuidAsKey)? {
                        key = Some(*zone.guid());
                    } else if 
                        *conf.data.get(OaZoneConfig::ForceGenerateGuidKey)? ||
                        *conf.data.get(OaZoneConfig::IsInstance)?
                    {
                        key = Some(Uuid::new());
                    }
                }
            }

            // Insert request to pending requests
            s.requests.insert(transaction_id, PendingInstance { 
                zone: zone.clone(),
                key,
                valid_until
            });

            debug!("Offering instance for zone {} with key {:?} until {}", zone.guid(), key, valid_until);

            // Offer instance to realm
            let _ = s.realm_client.send(RealmRequest::InstanceOffering { 
                transaction_id,
                key: InstanceKey::new(*zone.guid(), key),
            }).await;
        } else {
            debug!("Zone {} not served by this server.", zone);
        }

        // Cleanup expired requests
        s.requests.retain(|_, req| {
            Utc::now().signed_duration_since(req.valid_until)
                .num_milliseconds() < 0
        });

        Ok(())
    }

    pub async fn provision_instance(&self, transaction_id: Uuid) {
        let mut s = self.0.lock().await;
        if let Some(req) = s.requests.remove(&transaction_id) {
            if let Ok(instance) = ZoneInstanceBuilder::default()
                .zone(req.zone.clone())
                .realm_api(s.realm_api.clone())
                .handle(Handle::current())
                .instance_id(req.key)
                .instantiate().await
            {
                s.instances.insert(*req.zone.guid(), instance.label());
                
                let _ = s.realm_client.send(RealmRequest::InstanceProvisioned { 
                    transaction_id 
                }).await;

                let _ = s.event_sender.send(InstanceEvent::InstanceAdded(Box::new(instance))).await;
            }
        }
    }
}
