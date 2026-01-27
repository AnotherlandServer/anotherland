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

#![feature(exclusive_wrapper)]

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;
use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use chat_router::ChatRouter;
use clap::Parser;
use cluster::{ClusterEvent, Endpoint, Host, PeerIdentity};
use content::set_content_path;
use core_api::CoreApi;
use core_api::proto::{CoreRequest, CoreClient, CoreNotification};
use database::DatabaseExt;
use db::{CashShopItem, CashShopItemBundle, CashShopVendor, Character, ItemStorage, ObjectPlacement, ObjectTemplate, PremiumCurrency, PremiumCurrencyTransaction, WorldDef, Zone};
use equipment_slots::EQUIPMENT_SLOTS;
use error::RealmResult;
use instance_registry::InstanceRegistry;
use log::{debug, info, error};
use mongodb::Client;
use node_registry::{NodeRegistry, NodeSocketAddress};
use poem::{listener::TcpListener, post, Route, Server};
use proto::{NodeAddress, NodeType, RealmNotification, RealmResponse, RealmServer};
use reqwest::Url;
use schema::{MutationRoot, QueryRoot};
use session_manager::SessionManager;
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio::time;
use toolkit::print_banner;

use crate::db::{Navmesh, NavmeshTile, QuestDialogue, QuestState, QuestTemplate};
use crate::dialogue_importer::{import_dialogues, watch_dialogue_changes};
use crate::quest_importer::{import_quest_templates, watch_quest_template_changes};

mod schema;
mod db;
mod proto;
mod error;
mod node_registry;
mod instance_registry;
mod session_manager;
mod chat_router;
mod item_storage_session;
mod equipment_slots;
mod dialogue_importer;
mod quest_importer;

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

    #[arg(long, env = "IMPORT_DIALOGUES", default_value_t = false)]
    import_dialogues: bool,

    #[arg(long, env = "IMPORT_QUESTS", default_value_t = false)]
    import_quests: bool,

    #[arg(long, default_value_t = false)]
    hot_reload_dialogues: bool,

    #[arg(long, default_value_t = false)]
    hot_reload_quests: bool,
}

pub static NODE_REGISTRY: OnceLock<NodeRegistry> = OnceLock::new();
pub static INSTANCE_REGISTRY: OnceLock<InstanceRegistry> = OnceLock::new();
pub static SESSION_MANAGER: OnceLock<SessionManager> = OnceLock::new();
pub static CHAT_ROUTER: OnceLock<ChatRouter> = OnceLock::new();

#[toolkit::service_main(realm)]
async fn main() -> RealmResult<()> {
    let args = Args::parse();

    print_banner();

    let content_path = std::env::var("CONTENT_PATH")
        .ok()
        .and_then(|p| p.parse::<PathBuf>().ok())
        .or(std::env::current_dir().map(|p| p.join("content")).ok())
        .expect("content path inacessible");

    set_content_path(content_path).unwrap();

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
    db.init_collection::<CashShopItemBundle>().await;
    db.init_collection::<CashShopItem>().await;
    db.init_collection::<CashShopVendor>().await;
    db.init_collection::<ItemStorage>().await;
    db.init_collection::<Navmesh>().await;
    db.init_collection::<NavmeshTile>().await;
    db.init_collection::<QuestState>().await;
    db.init_collection::<QuestTemplate>().await;
    db.init_collection::<QuestDialogue>().await;

    // Read content
    LazyLock::force(&EQUIPMENT_SLOTS);

    if args.import_dialogues {
        import_dialogues(db.clone()).await?;
    }

    if args.hot_reload_dialogues {
        watch_dialogue_changes(db.clone())?;
    }

    if args.import_quests {
        import_quest_templates(db.clone()).await?;
    }

    if args.hot_reload_quests {
        watch_quest_template_changes(db.clone())?;
    }

    // Connect to core service
    let (core_client, core_notifications) = CoreClient::connect(&args.core_zmq_addr).await
        .expect("core service connect failed");

    let core_client = Arc::new(core_client);

    // Create realm zmq server
    let server = Arc::new(RealmServer::bind(&args.zmq_bind_url).await
        .expect("failed to start realm server"));

    let _ = NODE_REGISTRY.set(NodeRegistry::new(&server));
    let _ = INSTANCE_REGISTRY.set(InstanceRegistry::new(db.clone(), server.clone()));
    let _ = SESSION_MANAGER.set(SessionManager::new(core_api.clone(), server.clone()).await?);
    let _ = CHAT_ROUTER.set(ChatRouter::new(db.clone(), server.clone()));

    let peer_endpoints = Arc::new(Mutex::new(HashMap::new()));

    {
        let core_client = core_client.clone();
        let mut events = NODE_REGISTRY.get().unwrap().subscribe();
        let realm_id = args.realm_id;

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                match event {
                    node_registry::NodeRegistryEvent::NodeAdded(node) => {
                        if 
                            matches!(node.ty, NodeType::Frontend) &&
                            let NodeSocketAddress::Public(endpoint) = node.addr
                        {
                            // Register new frontend node with core server, so clients can connect to it
                            let _ = core_client.send(CoreRequest::ConnectRealm(realm_id, endpoint)).await;
                        }
                    },
                    node_registry::NodeRegistryEvent::NodeRemoved(node) => {
                        match node.ty {
                            proto::NodeType::Frontend => {
                                if let NodeSocketAddress::Public(endpoint) = node.addr {
                                    // Unregister frontend node from core server
                                    let _ = core_client.send(CoreRequest::DisconnectRealm(realm_id, endpoint)).await;
                                }
                            },
                            proto::NodeType::World => {
                                INSTANCE_REGISTRY.get().unwrap().purge_node(node.id).await;
                            },
                            _ => (),
                        }
                    }
                }
            }
        });
    }

    // Start graphql api
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db.clone())
        .data(core_api.clone())
        .data(server.clone())
        .finish();

    let app = Route::new()
        .at("/", post(GraphQL::new(schema.clone())));

    // Forward cluster notification to realm sub-services
    fn handle_notifications(server: Arc<RealmServer>, mut notifications: Receiver<CoreNotification>) {
        tokio::spawn(async move {
            while let Some(notification) = notifications.recv().await {
                if let CoreNotification::SessionTerminated(id) = &notification {
                    CHAT_ROUTER.get().unwrap().disconnect_session(*id).await;
                    SESSION_MANAGER.get().unwrap().terminate_session(*id).await;
                }

                // Propagate notification to realm nodes
                let _ = server.notify(RealmNotification::ClusterNotification(notification)).await;
            }
        });
    }

    fn handle_requests(
        server: Arc<RealmServer>,
        core_api: CoreApi,
        endpoints: Arc<Mutex<HashMap<PeerIdentity, Endpoint>>>
    ) {
        tokio::spawn(async move {
            while let Ok((peer, req)) = server.recv().await {
                match req {
                    proto::RealmRequest::RegisterNode(node_type, address) => {
                        match address {
                            NodeAddress::Public(addr) => {
                                NODE_REGISTRY.get().unwrap()
                                    .register_node(peer, node_type, NodeSocketAddress::Public(addr)).await
                            },
                            NodeAddress::Internal(port) => {
                                let endpoints = endpoints.clone();

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
                                                    Host::Domain(_domain) => {
                                                        unimplemented!()
                                                    }
                                                };

                                                NODE_REGISTRY.get().unwrap()
                                                    .register_node(peer, node_type, NodeSocketAddress::Internal(SocketAddr::new(ip, port))).await
                                            } else {
                                                error!("Unsupported node endpoint: {endpoint}");
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
                        INSTANCE_REGISTRY.get().unwrap()
                            .process_instance_offer(peer, transaction_id, key).await;
                    },
                    proto::RealmRequest::InstanceProvisioned { transaction_id } => {
                        INSTANCE_REGISTRY.get().unwrap()
                            .complete_instance_provisioning(peer, transaction_id).await;
                    },
                    proto::RealmRequest::InstanceShutdownNotification(key) => {
                        debug!("Instance {key:?} shutting down...");
                        INSTANCE_REGISTRY.get().unwrap()
                            .remove_instance(key.clone()).await;

                        let _ = server.send(&peer, RealmResponse::InstanceShutdownAck(key)).await;
                    },
                    proto::RealmRequest::ChatMessage { sender_id, destination, message } => {
                        CHAT_ROUTER.get().unwrap()
                            .forward_message(sender_id, destination, message).await;
                    },
                    proto::RealmRequest::ClientConnected { session_id } => {
                        if let Some(node) = NODE_REGISTRY.get().unwrap().node_for_peer(&peer).await {
                            SESSION_MANAGER.get().unwrap()
                                .update_cluster_node(session_id, node.id).await;
                        }
                    },
                    proto::RealmRequest::ClientDisconnected { session_id } => {
                        if let Ok(Some(session)) = core_api.get_session(&session_id).await {
                            let _ = session.destroy().await;
                        }
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
    handle_notifications(server.clone(), core_notifications);
    handle_requests(server.clone(), core_api.clone(), peer_endpoints);

    let _ = core_client.subscribe("core.session.").await; // subscribe to session notifications

    let core_client_handle = tokio::spawn(async move {
        loop {
            let result = core_client.recv().await;
            if let Err(e) = result {
                error!("Error receiving core server messages: {e:?}");
                break;
            }
        }
    });

    let graphql_handle = tokio::spawn(async move {
        info!("Starting realm server on http://{}", args.graphql_bind_addr);
        if let Err(e) = Server::new(TcpListener::bind(args.graphql_bind_addr))
            .run(app)
            .await
        {
            error!("GraphQL server error: {e}");
        }
    });

    // Wait for either task to complete (which would indicate a failure)
    tokio::select! {
        _ = core_client_handle => {
            error!("Core client task exited unexpectedly");
        },
        _ = graphql_handle => {
            error!("GraphQL server task exited unexpectedly");
        },
    }

    Ok(())
}
