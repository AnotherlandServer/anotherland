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

use std::{collections::HashMap, fmt::Display, net::SocketAddr, sync::Arc};

use cluster::{ClusterEvent, PeerIdentity};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast::{self, Receiver}, RwLock};
use toolkit::types::Uuid;

use crate::proto::{NodeType, RealmServer};

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

#[derive(Clone)]
pub enum NodeRegistryEvent {
    NodeAdded(Node),
    NodeRemoved(Node),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub ty: NodeType,
    pub addr: NodeSocketAddress,
}

struct NodeRegistryData {
    nodes: HashMap<PeerIdentity, Node>,
}

#[derive(Clone)]
pub struct NodeRegistry {
    data: Arc<RwLock<NodeRegistryData>>,
    events: broadcast::Sender<NodeRegistryEvent>,
}

impl NodeRegistry {
    pub fn new(realm_server: &RealmServer) -> NodeRegistry {
        let server_events = realm_server.events();
        let (events, _) = broadcast::channel(10);
        let data = Arc::new(RwLock::new(NodeRegistryData {  
            nodes: HashMap::new(),
        }));

        NodeRegistry::start_monitor(data.clone(), events.clone(), server_events);

        Self {
            data,
            events
        }
    }

    pub async fn register_node(&self, peer_identity: PeerIdentity, node_type: NodeType, address: NodeSocketAddress) {
        let mut state = self.data.write().await;

        let entry = state.nodes.entry(peer_identity)
            .or_insert(Node {
                id: Uuid::new(),
                ty: node_type, 
                addr: address
            })
            .clone();

        info!("Registered {node_type} at {address}");

        let _ = self.events.send(NodeRegistryEvent::NodeAdded(entry));
    }

    fn start_monitor(state: Arc<RwLock<NodeRegistryData>>, events: broadcast::Sender<NodeRegistryEvent>, mut receiver: Receiver<ClusterEvent>) {
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                if let ClusterEvent::Disconnected(peer_identity) = event {
                    let mut state = state.write().await;

                    if let Some(node) = state.nodes.remove(&peer_identity) {
                        info!("Unregistered {} at {}", node.ty, node.addr);
                        
                        let _ = events.send(NodeRegistryEvent::NodeRemoved(node));
                    }
                }
            }
        });
    }

    pub async fn node_for_peer(&self, peer: &PeerIdentity) -> Option<Node> {
        let s = self.data.read().await;
        s.nodes.get(peer)
            .cloned()
    }

    pub async fn node(&self, id: Uuid) -> Option<(PeerIdentity, Node)> {
        let s = self.data.read().await;
        s.nodes.iter()
            .find(|(_, node)| node.id == id)
            .map(|(k,v)| (k.clone(), v.clone()))
    }

    pub async fn nodes(&self) -> Vec<(PeerIdentity, Node)> {
        let s = self.data.read().await;
        s.nodes.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<NodeRegistryEvent> {
        self.events.subscribe()
    }
}