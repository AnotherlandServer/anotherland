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

use std::collections::HashMap;
use std::sync::Arc;

use cluster::PeerIdentity;

use crate::proto::{CoreServer, CoreRequest};
use crate::realm_status_registry::RealmStatusRegistry;

pub async fn run_core_server(server: Arc<CoreServer>, status_registry: Arc<RealmStatusRegistry>) {
    tokio::spawn(async move {
        let mut events = server.events();
        let mut registered_realm_endpoints = HashMap::new();

        loop {
            tokio::select! {
                Ok(event) = events.recv() => {
                    match event {
                        cluster::ClusterEvent::Accepted(_, _) => (),
                        cluster::ClusterEvent::Disconnected(identity) => {
                            if let Some((id, endpoints)) = registered_realm_endpoints.remove(&identity) {
                                for endpoint in endpoints {
                                    status_registry.unregister_endpoint(id, endpoint).await;
                                }
                            }
                        },
                    }
                },
                Ok((identity, msg)) = server.recv() => {
                    match msg {
                        CoreRequest::ConnectRealm(id, endpoint) => {
                            let entry = registered_realm_endpoints.entry(identity)
                                .or_insert((id, vec![]));

                            entry.1.push(endpoint);
                            status_registry.register_endpoint(id, endpoint).await;
                        },
                        CoreRequest::DisconnectRealm(id, endpoint) => {
                            if let Some(entry) = registered_realm_endpoints.get_mut(&identity) {
                                entry.1.retain_mut(|compare_endpoint| compare_endpoint != &endpoint);
                            }

                            if !status_registry.unregister_endpoint(id, endpoint).await {
                                registered_realm_endpoints.remove(&identity);
                            }
                        },
                        CoreRequest::UpdateRealmPopulation(population) => {
                            if let Some((id, _)) = registered_realm_endpoints.get(&identity) {
                                status_registry.update_population(id, population).await;
                            }
                        },
                    }
                }
            }
        }
    });
}