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

use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{DateTime, TimeDelta, Utc};
use cluster::PeerIdentity;
use log::{error, info, warn};
use mongodb::Database;
use tokio::sync::{broadcast, RwLock};
use toolkit::{anyhow, types::Uuid};

use crate::{error::RealmResult, proto::{InstanceKey, RealmNotification, RealmResponse, RealmServer}, NODE_REGISTRY, SESSION_MANAGER};

struct InstanceRequest {
    key: InstanceKey,
    state: InstanceRequestState,
    valid_until: DateTime<Utc>,
    wait_state: broadcast::Sender<Option<Arc<Instance>>>,
    count: i32,
}

enum InstanceRequestState {
    Inquiried,
    Offered { peer: PeerIdentity },
}

struct InstanceRegistryData {
    _db: Database,
    server: Arc<RealmServer>,
    requests: HashMap<Uuid, InstanceRequest>,
    instances: HashMap<InstanceKey, Arc<Instance>>,
}

#[derive(Clone)]
pub struct InstanceRegistry(Arc<RwLock<InstanceRegistryData>>);

impl InstanceRegistry {
    pub fn new(db: Database, server: Arc<RealmServer>) -> Self {
        let data = Arc::new(RwLock::new(InstanceRegistryData {
            _db: db,
            server,
            requests: HashMap::new(),
            instances: HashMap::new(),
        }));

        fn start_tick(data: Arc<RwLock<InstanceRegistryData>>) {
            tokio::spawn(async move {
                loop {
                    {
                        let mut s = data.write().await;
                        let timedout: Vec<(Uuid, InstanceRequest)> = s.requests.extract_if(|_, req| {
                            Utc::now().signed_duration_since(req.valid_until)
                                .num_milliseconds() >= 0
                        }).collect();

                        for (key, req) in timedout {
                            if req.count < 4 {
                                warn!("Instance request {key} timed out! Retrying...");

                                let request = {
                                    let entry = s.requests
                                        .entry(Uuid::new())
                                        .insert_entry(InstanceRequest {
                                            key: req.key,
                                            state: InstanceRequestState::Inquiried,
                                            valid_until: Utc::now()
                                                .checked_add_signed(TimeDelta::seconds(2))
                                                .expect("valid time"),
                                            wait_state: req.wait_state,
                                            count: req.count + 1,
                                        });
                        
                                    RealmNotification::InstanceRequested {
                                        transaction_id: *entry.key(),
                                        zone: entry.get().key.zone(),
                                        key: entry.get().key.instance(),
                                        valid_until: entry.get().valid_until
                                    }
                                };
                        
                                // Repeat request
                                s.server.notify(request).await
                                    .expect("failed to send notification");
                            } else {
                                error!("Instance request {} timed out! No node available to handle zone {}...", key, req.key.zone());
                            }
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });
        }

        start_tick(data.clone());

        Self(data)
    }

    pub async fn request_instance(&self, session_id: Uuid, key: InstanceKey) -> RealmResult<Arc<Instance>> {
        let mut s = self.0.write().await;
        
        // Check if there is already a running instance we could connect to
        if let Some(instance) = s.instances.get(&key) {
            return Ok(instance.clone());
        } else if let Some(request) = s.requests
            .values()
            .find(|req| req.key == key) {

            let mut receiver = request.wait_state.subscribe();
            drop(s); // drop state to free lock

            if let Ok(Some(instance)) = receiver.recv().await {
                return Ok(instance);
            } else {
                return Err(anyhow::Error::msg("failed to join instance").into());
            }
        }

        // We couldn't find a running instance or request, so we are initiating a new request now
        let (sender, mut receiver) = broadcast::channel(1);
        
        let entry = s.requests.entry(Uuid::new());
        let request = {
            let entry = entry.insert_entry(InstanceRequest {
                key,
                state: InstanceRequestState::Inquiried,
                valid_until: Utc::now()
                    .checked_add_signed(TimeDelta::seconds(2))
                    .expect("valid time"),
                wait_state: sender,
                count: 0,
            });

            RealmNotification::InstanceRequested {
                transaction_id: *entry.key(),
                zone: entry.get().key.zone(),
                key: entry.get().key.instance(),
                valid_until: entry.get().valid_until
            }
        };

        // Request instance in cluster
        s.server.notify(request).await?;
        drop(s);

        if let Ok(Some(instance)) = receiver.recv().await {
            SESSION_MANAGER.get().unwrap()
                .update_instance(session_id, instance.key.zone(), instance.key.instance()).await;

            Ok(instance)
        } else {
            Err(anyhow::Error::msg("failed to join instance").into())
        }
    }

    pub async fn process_instance_offer(&self, peer: PeerIdentity, transaction_id: Uuid, key: InstanceKey) {
        let mut s = self.0.write().await;
        if 
            let Some(req) = s.requests.get_mut(&transaction_id) &&
            matches!(req.state, InstanceRequestState::Inquiried)
        {
            req.key = key.clone();
            req.state = InstanceRequestState::Offered { peer: peer.clone() };
            
            // We accept the first offer we receive
            let _ = s.server.send(&peer, RealmResponse::InstanceOfferingAccepted { 
                transaction_id, 
                key,
            }).await;
        }
    }

    pub async fn complete_instance_provisioning(&self, peer: PeerIdentity, transaction_id: Uuid) {
        let mut s = self.0.write().await;
        if let Some(req) = s.requests.remove(&transaction_id) {
            if 
                let InstanceRequestState::Offered { peer: req_peer } = &req.state &&
                *req_peer == peer &&
                let Some(node) = NODE_REGISTRY.get().unwrap().node_for_peer(&peer).await
            {
                let instance = Arc::new(Instance {
                    key: req.key,
                    node: node.id
                });

                info!("Instance {} of zone {} got provisioned.", instance.key, instance.key.zone());

                s.instances.insert(instance.key.clone(), instance.clone());
                let _ = req.wait_state.send(Some(instance));
            } else {
                warn!("Instance transaction {} of zone {} committed by wrong peer!", transaction_id, req.key.zone());

                // Insert request back into the pile
                s.requests.insert(transaction_id, req);
            }
        }
    }

    pub async fn remove_instance(&self, key: InstanceKey) {
        let mut s = self.0.write().await;
        s.instances.remove(&key);
    }

    pub async fn get_instance(&self, key: InstanceKey) -> Option<Arc<Instance>> {
        let s = self.0.read().await;
        s.instances.get(&key).cloned()
    }

    pub async fn purge_node(&self, node: Uuid) {
        let mut s = self.0.write().await;
        s.instances.retain(|_, instance| instance.node != node);
    }
}

pub struct Instance {
    pub key: InstanceKey,
    pub node: Uuid,
}