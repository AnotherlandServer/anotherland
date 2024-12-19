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

#![feature(let_chains)]
#![feature(hash_extract_if)]

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use std::{net::SocketAddrV4, sync::Arc};

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use clap::Parser;
use cluster::{ClusterEvent, Endpoint, Host, PeerIdentity};
use core_api::CoreApi;
use core_api::proto::{CoreRequest, CoreClient, CoreNotification};
use database::{DatabaseExt, DatabaseRecord};
use db::{Character, ObjectPlacement, ObjectTemplate, PremiumCurrency, PremiumCurrencyTransaction, WorldDef, Zone};
use error::RealmResult;
use instance_registry::InstanceRegistry;
use log::{error, info, warn};
use mongodb::bson::doc;
use mongodb::{Client, Database};
use node_registry::{NodeRegistry, NodeSocketAddress};
use poem::{listener::TcpListener, post, Route, Server};
use proto::{NodeAddress, RealmNotification, RealmServer};
use reqwest::Url;
use schema::{MutationRoot, QueryRoot};
use session_manager::SessionManager;
use tokio::net::lookup_host;
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio::time;
use toolkit::print_banner;

mod schema;
mod db;
mod proto;
mod error;
mod node_registry;
mod instance_registry;
mod session_manager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env = "GRAPHQL_BIND_ADDR", default_value = "127.0.0.1:8001")]
    graphql_bind_addr: String,

    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "CORE_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15000")]
    core_zmq_addr: String,

    #[arg(long, env = "ZMQ_BIND_ADDR", default_value = "tcp://127.0.0.1:15001")]
    zmq_bind_url: String,

    #[arg(long, env = "MONGO_URI")]
    mongo_uri: String,

    #[arg(long, env = "MONGO_DB", default_value = "realm")]
    mongo_db: String,

    #[arg(long, env = "REALM_ID")]
    realm_id: i32,
}

#[toolkit::service_main(realm)]
async fn main() -> RealmResult<()> {
    let args = Args::parse();

    print_banner();

    // Init core api
    let core_api = CoreApi::new(args.service_core_url);

    // Get realm info
    let realm = core_api.get_realm(args.realm_id).await?
        .expect("Realm not found");

    info!("Starting up realm {}...", realm.name());

    // Init database
    let client = Client::with_uri_str(&args.mongo_uri).await
        .expect("Database connection failed");
    let db = client.database(&args.mongo_db);

    db.init_collection::<Character>().await;
    db.init_collection::<PremiumCurrencyTransaction>().await;
    db.init_collection::<PremiumCurrency>().await;
    db.init_collection::<WorldDef>().await;
    db.init_collection::<Zone>().await;
    db.init_collection::<ObjectPlacement>().await;
    db.init_collection::<ObjectTemplate>().await;

    // Connect to core service
    let (core_client, core_notifications) = CoreClient::connect(&args.core_zmq_addr).await
        .expect("core service connect failed");

    let core_client = Arc::new(core_client);

    // Create realm zmq server
    let server = Arc::new(RealmServer::bind(&args.zmq_bind_url).await
        .expect("failed to start realm server"));

    let node_registry = NodeRegistry::new(args.realm_id, server.clone(), core_client.clone());
    let instance_registry = InstanceRegistry::new(db.clone(), server.clone(), node_registry.clone());
    let session_manager = SessionManager::new(core_api.clone(), server.clone()).await?;
    let peer_endpoints = Arc::new(Mutex::new(HashMap::new()));

    // Start graphql api
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db.clone())
        .data(core_api.clone())
        .data(node_registry.clone())
        .data(instance_registry.clone())
        .data(session_manager.clone())
        .finish();

    let app = Route::new()
        .at("/", post(GraphQL::new(schema.clone())));

    // Forward cluster notification to realm sub-services
    fn handle_notifications(server: Arc<RealmServer>, session_manager: SessionManager, mut notifications: Receiver<CoreNotification>) {
        tokio::spawn(async move {
            while let Some(notification) = notifications.recv().await {
                if let CoreNotification::SessionTerminated(id) = &notification {
                    session_manager.terminate_session(*id).await;
                }

                // Propagate notification to realm nodes
                let _ = server.notify(RealmNotification::ClusterNotification(notification)).await;
            }
        });
    }

    fn handle_requests(
        server: Arc<RealmServer>, 
        node_registry: NodeRegistry,
        instance_registry: InstanceRegistry,
        endpoints: Arc<Mutex<HashMap<PeerIdentity, Endpoint>>>
    ) {
        tokio::spawn(async move {
            while let Ok((peer, req)) = server.recv().await {
                match req {
                    proto::RealmRequest::RegisterNode(node_type, address) => {
                        match address {
                            NodeAddress::Public(addr) => {
                                node_registry.register_node(peer, node_type, NodeSocketAddress::Public(addr)).await
                            },
                            NodeAddress::Internal(port) => {
                                let endpoints = endpoints.clone();
                                let node_registry = node_registry.clone();

                                // Spawn a new task to deal with the fact, that this request might be
                                // processed before endpoints where updated.
                                tokio::spawn(async move {
                                    let mut tries = 0;

                                    while tries < 3 {
                                        let endpoints = endpoints.lock().await;
                                        if let Some(endpoint) = endpoints.get(&peer) {
                                            if let Endpoint::Tcp(host, _) = endpoint {
                                                let ip: IpAddr = match host {
                                                    &Host::Ipv4(addr) => addr.into(),
                                                    &Host::Ipv6(addr) => addr.into(),
                                                    Host::Domain(domain) => {
                                                        unimplemented!()
                                                    }
                                                };

                                                node_registry.register_node(peer, node_type, NodeSocketAddress::Internal(SocketAddr::new(ip, port))).await
                                            } else {
                                                error!("Unsupported node endpoint: {}", endpoint);
                                            }

                                            break;
                                        } else {
                                            tries += 1;
                                            time::sleep(Duration::from_millis(100)).await;
                                        }
                                    }
                                });
                            }
                        };
                    },
                    proto::RealmRequest::InstanceOffering { transaction_id, key } => {
                        instance_registry.process_instance_offer(peer, transaction_id, key).await;
                    },
                    proto::RealmRequest::InstanceProvisioned { transaction_id } => {
                        instance_registry.complete_instance_provisioning(peer, transaction_id).await;
                    },
                    proto::RealmRequest::RemoveInstance(key) => {
                        instance_registry.remove_instance(key).await;
                    }
                }
            }
        });
    }

    fn monitor_realm_server(server: Arc<RealmServer>, endpoints: Arc<Mutex<HashMap<PeerIdentity, Endpoint>>>) {
        tokio::spawn(async move {
            let mut events = server.events();
            while let Ok(event) = events.recv().await {
                match event {
                    ClusterEvent::Accepted(id, endpoint) => {
                        let mut endpoints = endpoints.lock().await;
                        endpoints.insert(id, endpoint);
                    },
                    ClusterEvent::Disconnected(id) => {
                        let mut endpoints = endpoints.lock().await;
                        endpoints.remove(&id);
                    },
                }
            }
        });
    }

    monitor_realm_server(server.clone(), peer_endpoints.clone());
    handle_notifications(server.clone(), session_manager.clone(), core_notifications);
    handle_requests(server.clone(), node_registry, instance_registry.clone(), peer_endpoints);

    let _ = core_client.subscribe("core.session.").await; // subscribe to session notifications
    //let _ = core_client.send(CoreRequest::ConnectRealm(args.realm_id, args.frontend_ip.into())).await;

    tokio::spawn(async move {
        info!("Starting realm server on http://{}", args.graphql_bind_addr);
        Server::new(TcpListener::bind(args.graphql_bind_addr))
            .run(app)
            .await
            .unwrap();
    })
    .await
    .unwrap();

    Ok(())
}
