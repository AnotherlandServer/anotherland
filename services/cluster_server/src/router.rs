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

use std::{collections::HashMap, sync::Arc};

use log::{debug, warn};
use realm_api::{ClusterAddress, ClusterNode, Instance, RealmApi};
use tokio::{select, sync::{mpsc::{self, Receiver, Sender}, Mutex}};
use toolkit::types::Uuid;
use world_service::{WorldClient, WorldRequest, WorldResponse};

use crate::error::ClusterFrontendResult;

struct RouterData {
    worlds: HashMap<Uuid, Sender<WorldRequest>>,
    peers: HashMap<Uuid, (Uuid, Sender<WorldResponse>)>,
}

#[derive(Clone)]
pub struct Router {
    realm_api: RealmApi,
    data: Arc<Mutex<RouterData>>
}

impl Router {
    pub fn new(realm_api: RealmApi) -> Self {
        Self {
            realm_api,
            data: Arc::new(Mutex::new(RouterData {
                worlds: HashMap::new(),
                peers: HashMap::new(),
            }))
        }
    }

    async fn get_or_connect_world(&self, node: &ClusterNode) -> ClusterFrontendResult<(Uuid, Sender<WorldRequest>)> {
        let mut s = self.data.lock().await;
        if let Some(sender) = s.worlds.get(&node.id) {
            Ok((node.id, sender.clone()))
        } else {
            let node_addr = match node.addr {
                ClusterAddress::Public(_) => unreachable!(),
                ClusterAddress::Internal(addr) => addr,
            };

            let (sender, mut receiver) = mpsc::channel(10);
            let (client, _) = WorldClient::connect(
                &format!("tcp://{}:{}", 
                        node_addr.ip().to_string(), 
                        node_addr.port()
                    )
                ).await?;

            s.worlds.insert(node.id, sender.clone());

            let data = self.data.clone();
            let world_id = node.id;

            tokio::spawn(async move {
                loop {
                    select! {
                        req = receiver.recv() => {
                            match req {
                                Some(req) => {
                                    let _ = client.send(req).await;
                                },
                                None => {
                                    debug!("Closing world connection. No active sessissions.");
                                    break
                                },
                            }
                        },
                        res = client.recv() => {
                            match res {
                                Ok(res) => {
                                    let mut s = data.lock().await;
                                    let peer_id = match res {
                                        WorldResponse::ServerMessage { peer, .. } => Some(*peer),
                                        WorldResponse::Travel { peer, .. } => Some(*peer),
                                    };

                                    if 
                                        let Some(peer_id) = peer_id &&
                                        let Some((_, peer)) = s.peers.get(&peer_id.into()) &&
                                        peer.send(res).await.is_err()
                                    {
                                        s.peers.remove(&peer_id.into());
                                    }
                                },
                                Err(_) => {
                                    warn!("World node '{}' connection closed.", world_id);
                                    break
                                },
                            }
                        }
                    }
                }

                let mut s = data.lock().await;
                s.worlds.remove(&world_id);

                // Remove all peers connected to this world
                s.peers.retain(|_, (id, _)| id != &world_id);
            });

            Ok((node.id, sender))
        }
    }

    pub async fn join_instance(&self, peer: Uuid, session: Uuid, zone: Uuid, key: Option<Uuid>) 
        -> ClusterFrontendResult<(Instance, Receiver<WorldResponse>, Sender<WorldRequest>)> 
    {
        let instance = self.realm_api.join_instance(session, zone, key).await?;
        let (node_id, node) = self.get_or_connect_world(&instance.node).await?;
        let (sender, receiver) = mpsc::channel(10);

        let mut s = self.data.lock().await;
        s.peers.insert(peer, (node_id, sender));

        Ok((instance, receiver, node))
    }

    pub async fn remove_peer(&self, peer: Uuid) {
        let mut s = self.data.lock().await;
        s.peers.remove(&peer);
    }
}