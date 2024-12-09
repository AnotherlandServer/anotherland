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

use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{DateTime, TimeDelta, Utc};
use cluster::PeerIdentity;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};
use toolkit::{anyhow, types::Uuid};

use crate::{error::RealmResult, node_registry::NodeRegistry, proto::{InstanceKey, RealmNotification, RealmResponse, RealmServer}};

struct InstanceRequest {
    zone: Uuid,
    key: Option<Uuid>,
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
    server: Arc<RealmServer>,
    nodes: NodeRegistry,
    requests: HashMap<Uuid, InstanceRequest>,
    instances: HashMap<Uuid, Arc<Instance>>,
}

#[derive(Clone)]
pub struct InstanceRegistry(Arc<RwLock<InstanceRegistryData>>);

impl InstanceRegistry {
    pub fn new(server: Arc<RealmServer>, nodes: NodeRegistry) -> Self {
        let data = Arc::new(RwLock::new(InstanceRegistryData {
            server,
            nodes,
            requests: HashMap::new(),
            instances: HashMap::new(),
        }));

        fn start_tick(data: Arc<RwLock<InstanceRegistryData>>) {
            tokio::spawn(async move {
                loop {
                    {
                        let mut s = data.write().await;
                        let timedout: Vec<(Uuid, InstanceRequest)> = s.requests.extract_if(|key, req| {
                            Utc::now().signed_duration_since(req.valid_until)
                                .num_milliseconds() >= 0
                        }).collect();

                        for (key, req) in timedout {
                            if req.count < 4 {
                                warn!("Instance request {} timed out! Retrying...", key);

                                let request = {
                                    let entry = s.requests
                                        .entry(Uuid::new())
                                        .insert_entry(InstanceRequest {
                                            zone: req.zone,
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
                                        zone: entry.get().zone,
                                        key: entry.get().key,
                                        valid_until: entry.get().valid_until
                                    }
                                };
                        
                                // Repeat request
                                s.server.notify(request).await
                                    .expect("failed to send notification");
                            } else {
                                error!("Instance request {} timed out! No node available to handle zone {}...", key, req.zone);
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

    pub async fn request_instance(&self, zone: Uuid, key: Option<Uuid>) -> RealmResult<Arc<Instance>> {
        let mut s = self.0.write().await;
        
        // If we where supplied with a key, check if there is already an instance
        // or a request for it running.
        if let Some(key) = key {
            if let Some(instance) = s.instances.get(&key) {
                return Ok(instance.clone());
            } else if let Some(request) = s.requests
                .values()
                .find(|req| if let Some(req_key) = req.key {
                    req_key == key
                } else {
                    false
                }) {

                let mut receiver = request.wait_state.subscribe();
                drop(s); // drop state to free lock

                if let Ok(Some(instance)) = receiver.recv().await {
                    return Ok(instance);
                } else {
                    return Err(anyhow::Error::msg("failed to join instance").into());
                }
            }
        }

        // We couldn't find a running instance or request, so we are initiating a new request now
        let (sender, mut receiver) = broadcast::channel(1);
        
        let entry = s.requests.entry(Uuid::new());
        let request = {
            let entry = entry.insert_entry(InstanceRequest {
                zone,
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
                zone: entry.get().zone,
                key: entry.get().key,
                valid_until: entry.get().valid_until
            }
        };

        // Request instance in cluster
        s.server.notify(request).await?;
        drop(s);

        if let Ok(Some(instance)) = receiver.recv().await {
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
            req.key = Some(key.instance());
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
                *req_peer == peer
            {
                let instance = Arc::new(Instance {
                    id: req.key.unwrap(),
                    zone: req.zone,
                    peer: req_peer.clone(),
                });

                info!("Instance {} of zone {} got provisioned.", instance.id, instance.zone);

                s.instances.insert(instance.id, instance.clone());
                let _ = req.wait_state.send(Some(instance));
            } else {
                warn!("Instance transaction {} of zone {} committed by wrong peer!", transaction_id, req.zone);

                // Insert request back into the pile
                s.requests.insert(transaction_id, req);
            }
        }
    }

    pub async fn remove_instance(&self, id: Uuid) {
        let mut s = self.0.write().await;
        s.instances.remove(&id);
    }
}

pub struct Instance {
    pub id: Uuid,
    pub zone: Uuid,
    pub peer: PeerIdentity,
}