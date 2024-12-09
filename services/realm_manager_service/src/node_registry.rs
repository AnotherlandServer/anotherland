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

use std::{collections::HashMap, fmt::Display, net::SocketAddr, sync::Arc};

use cluster::{ClusterEvent, PeerIdentity};
use core_api::proto::{CoreClient, CoreRequest};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast::Receiver, RwLock};
use toolkit::types::Uuid;

use crate::proto::{NodeAddress, NodeType, RealmNotification, RealmServer};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum NodeSocketAddress {
    Public(SocketAddr),
    Internal(SocketAddr),
}

impl Display for NodeSocketAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public(adr) => adr.fmt(f),
            Self::Internal(adr) => adr.fmt(f),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: Uuid,
    pub ty: NodeType,
    pub addr: NodeSocketAddress,
}

struct NodeRegistryData {
    realm_id: i32,
    realm_server: Arc<RealmServer>,
    core: Arc<CoreClient>,
    nodes: HashMap<PeerIdentity, Node>,
}

#[derive(Clone)]
pub struct NodeRegistry(Arc<RwLock<NodeRegistryData>>);

impl NodeRegistry {
    pub fn new(realm_id: i32, realm_server: Arc<RealmServer>, core: Arc<CoreClient>) -> NodeRegistry {
        let server_events = realm_server.events();
        let data = Arc::new(RwLock::new(NodeRegistryData {  
            realm_id,
            realm_server,
            core,
            nodes: HashMap::new(),
        }));

        NodeRegistry::start_monitor(data.clone(), server_events);

        Self(data)
    }

    pub async fn register_node(&self, peer_identity: PeerIdentity, node_type: NodeType, address: NodeSocketAddress) {
        let mut state = self.0.write().await;
        let realm_server = state.realm_server.clone();
        let core = state.core.clone();

        let entry = state.nodes.entry(peer_identity)
            .or_insert(Node {
                id: Uuid::new(), 
                ty: node_type, 
                addr: address
            });

        info!("Registered {} at {}", node_type, address);

        let _ = realm_server.notify(RealmNotification::NodeAdded(entry.clone())).await;

        if 
            matches!(entry.ty, NodeType::Frontend) &&
            let NodeSocketAddress::Public(endpoint) = entry.addr
        {
            // Register new frontend node with core server, so clients can connect to it
            let _ = core.send(CoreRequest::ConnectRealm(state.realm_id, endpoint)).await;
        }
    }

    fn start_monitor(state: Arc<RwLock<NodeRegistryData>>, mut receiver: Receiver<ClusterEvent>) {
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                if let ClusterEvent::Disconnected(peer_identity) = event {
                    let mut state = state.write().await;
                    let realm_server = state.realm_server.clone();
                    let core = state.core.clone();            

                    if let Some(node) = state.nodes.remove(&peer_identity) {
                        info!("Unregisted {} at {}", node.ty, node.addr);
                        
                        let _ = realm_server.notify(RealmNotification::NodeRemoved(node.id)).await;

                        if 
                            matches!(node.ty, NodeType::Frontend) &&
                            let NodeSocketAddress::Public(endpoint) = node.addr
                        {
                            // Unregister frontend node from core server
                            let _ = core.send(CoreRequest::DisconnectRealm(state.realm_id, endpoint)).await;
                        }
                    }
                }
            }
        });
    }

    pub async fn node_for_peer(&self, peer: &PeerIdentity) -> Option<Node> {
        let s = self.0.read().await;
        s.nodes.get(peer)
            .cloned()
    }

    pub async fn node(&self, id: Uuid) -> Option<Node> {
        let s = self.0.read().await;
        s.nodes.values()
            .find(|node| node.id == id)
            .cloned()
    }

    pub async fn nodes(&self) -> Vec<Node> {
        let s = self.0.read().await;
        s.nodes.values()
            .cloned()
            .collect()
    }
}