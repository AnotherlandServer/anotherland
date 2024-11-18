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

use cluster::{ClusterEvent, PeerIdentity};
use core_api::proto::{CoreClient, CoreRequest};
use mongodb::bson::Uuid;
use tokio::sync::{broadcast::Receiver, RwLock};

use crate::proto::{NodeType, RealmNotification, RealmServer};

struct NodeRegistryData {
    realm_id: i32,
    realm_server: Arc<RealmServer>,
    core: Arc<CoreClient>,
    nodes: HashMap<PeerIdentity, (Uuid, NodeType, SocketAddr)>,
}

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

    pub async fn register_node(&self, peer_identity: PeerIdentity, node_type: NodeType, endpoint: SocketAddr) {
        let mut state = self.0.write().await;
        let realm_server = state.realm_server.clone();
        let core = state.core.clone();

        let entry = state.nodes.entry(peer_identity)
            .or_insert((Uuid::new(), node_type, endpoint));
        let _ = realm_server.notify(RealmNotification::NodeAdded((entry.0, entry.1, entry.2))).await;

        if matches!(entry.1, NodeType::FrontendNode) {
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

                    if let Some((id, node_type, endpoint)) = state.nodes.remove(&peer_identity) {
                        let _ = realm_server.notify(RealmNotification::NodeRemoved(id)).await;

                        if matches!(node_type, NodeType::FrontendNode) {
                            // Unregister frontend node from core server
                            let _ = core.send(CoreRequest::DisconnectRealm(state.realm_id, endpoint)).await;
                        }
                    }
                }
            }
        });
    }
}