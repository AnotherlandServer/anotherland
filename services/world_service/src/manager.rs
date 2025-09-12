// Copyright (C) 2025 AnotherlandServer
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

use std::{collections::HashMap, sync::Arc};

use bevy::app::SubApp;
use chrono::{DateTime, Utc};
use core_api::CoreApi;
use futures_util::TryStreamExt;
use log::{debug, error, info, trace};
use obj_params::OaZoneConfig;
use realm_api::{proto::{InstanceKey, RealmClient, RealmRequest}, RealmApi, WorldDef, Zone};
use tokio::{runtime::Handle, sync::{mpsc::{self, Sender, UnboundedSender}, oneshot, Mutex}};
use tokio_util::task::TaskTracker;
use toolkit::types::Uuid;

use crate::{error::WorldResult, instance::{InstanceLabel, ZoneInstanceBuilder, ZoneSubApp}, object_cache::ObjectCache, plugins::{ControllerEvent, WorldEvent}, proto::TravelMode, OBJECT_CACHE};

struct PendingInstance {
    world_def: Arc<WorldDef>,
    zone: Arc<Zone>,
    key: Option<Uuid>,
    valid_until: DateTime<Utc>,
}

struct InstanceManagerData {
    realm_api: RealmApi,
    core_api: CoreApi,
    realm_client: Arc<RealmClient>,
    zones: HashMap<Uuid, Arc<Zone>>,
    worlds: HashMap<Uuid, Arc<WorldDef>>,
    requests: HashMap<Uuid, PendingInstance>,
    instances: Vec<InstanceLabel>,
    event_sender: mpsc::UnboundedSender<InstanceEvent>,
    limit: usize,
    object_cache: ObjectCache,
}

pub enum InstanceEvent {
    InstanceAdded(Box<SubApp>),
    InstanceStopping(InstanceLabel),
    InstanceRemoved(InstanceLabel),
    ControllerSpawnRequested {
        peer: Uuid,
        instance: InstanceLabel,
        session: Uuid,
        events: UnboundedSender<WorldEvent>,
        controller: oneshot::Sender<WorldResult<Sender<ControllerEvent>>>,
        travel_mode: TravelMode,
    },
    WorldShutdown,
}

#[derive(Clone)]
pub struct InstanceManager(Arc<Mutex<InstanceManagerData>>);

impl InstanceManager {
    pub async fn new(
        realm_api: RealmApi, 
        core_api: CoreApi,
        realm_client: Arc<RealmClient>, 
        event_sender: mpsc::UnboundedSender<InstanceEvent>, 
        limit: usize, 
        groups: &[&str]
    ) -> WorldResult<Self> {
        // Query worlds
        let mut worlds = HashMap::new();
        let mut cursor = realm_api.query_worlddefs().query().await?;
        while let Ok(Some(world)) = cursor.try_next().await {
            worlds.insert(*world.guid(), Arc::new(world));
        }

        // Query zones
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

            info!("Serving zones for groups: {groups:?}");
        }
        
        Ok(Self(Arc::new(Mutex::new(InstanceManagerData { 
            realm_api: realm_api.clone(),
            core_api,
            realm_client,
            zones: zones.into_iter()
                .map(|zone| (*zone.guid(), Arc::new(zone)))
                .collect(),
            worlds,
            requests: HashMap::new(),
            instances: Vec::new(),
            event_sender,
            limit,
            object_cache: OBJECT_CACHE.wait().clone(),
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

        debug!("Got instance request for zone {zone} with key {key:?} until {valid_until}");

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

            if let Some(world_def) = s.worlds.get(zone.worlddef_guid()).cloned() {
                // Insert request to pending requests
                s.requests.insert(transaction_id, PendingInstance {
                    world_def,
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
                error!("Can't offer zone {}, world not found!", zone.guid());
            }
        } else {
            debug!("Zone {zone} not served by this server.");
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

        let realm_api = s.realm_api.clone();
        let core_api = s.core_api.clone();
        let realm_client = s.realm_client.clone();
        let object_cache = s.object_cache.clone();

        if let Some(req) = s.requests.remove(&transaction_id) {
            drop(s); // Release lock before instantiation

            match ZoneInstanceBuilder::default()
                .world_def(req.world_def.clone())
                .zone(req.zone.clone())
                .realm_api(realm_api)
                .core_api(core_api)
                .realm_client(realm_client)
                .handle(Handle::current())
                .task_tracker(TaskTracker::new())
                .object_cache(object_cache)
                .instance_id(req.key)
                .manager(self.clone())
                .instantiate().await
            {
                Ok(instance) => {
                    // Relock to update state
                    let mut s = self.0.lock().await;
                    s.instances.push(instance.label());
                    
                    let _ = s.realm_client.send(RealmRequest::InstanceProvisioned { 
                        transaction_id 
                    }).await;
    
                    let _ = s.event_sender.send(InstanceEvent::InstanceAdded(Box::new(instance)));
                },
                Err(e) => {
                    error!("Failed to instantiate instance: {e:?}");
                }
            }
        }
    }

    pub async fn request_unregister_instance(&self, label: InstanceLabel) {
        let s = self.0.lock().await;
        let _ = s.realm_client.send(RealmRequest::InstanceShutdownNotification(
            InstanceKey::new(label.id(), label.instance())
        )).await;
    }

    pub async fn unregister_instance(&self, label: InstanceLabel) {
        let mut s = self.0.lock().await;
        s.instances.retain(|l| l != &label);

        debug!("Instance stopped {label:?}");

        let _ = s.event_sender.send(InstanceEvent::InstanceRemoved(label));
        if s.instances.is_empty() && s.limit == 0 {
            let _ = s.event_sender.send(InstanceEvent::WorldShutdown);
        }
    }

    pub async fn shutdown_instance(&self, key: InstanceKey) {
        let s = self.0.lock().await;
        let _ = s.event_sender.send(InstanceEvent::InstanceStopping(InstanceLabel::new(key.zone(), key.instance())));
    }

    pub async fn shutdown_world(&self) {
        trace!("Begin world shutdown");
        let mut s = self.0.lock().await;
        s.limit = 0; // Reduce instance limit to zero, so no new instances will be offered.

        trace!("Check instances");
        if s.instances.is_empty() {
            let _ = s.event_sender.send(InstanceEvent::WorldShutdown);
        } else {
            for label in s.instances.iter() {
                trace!("Announcing instace shutdown {label:?}");
                let _ = s.realm_client.send(RealmRequest::InstanceShutdownNotification(
                    InstanceKey::new(label.id(), label.instance())
                )).await;
            }
        }
    }

    pub fn get_world_def(&self, id: &Uuid) -> Option<Arc<WorldDef>> {
        let s = self.0.blocking_lock();
        s.worlds.get(id).cloned()
    }

    pub fn get_world_def_by_name(&self, name: &str) -> Option<Arc<WorldDef>> {
        let s = self.0.blocking_lock();
        s.worlds.values()
            .find(|w| w.name() == name)
            .cloned()
    }
}
