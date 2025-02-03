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

#![feature(let_chains)]
#![feature(exclusive_wrapper)]

use instance::{InstanceLabel, ZoneSubApp};
use obj_params::Player;
use object_cache::ObjectCache;
use plugins::{ControllerEvent, NetworkExt, NetworkPlugin};
use protocol::CPkt;

use std::{collections::HashMap, hash::Hash, net::{IpAddr, Ipv6Addr, SocketAddr, SocketAddrV6}, sync::Arc, time::Duration};

use anyhow::Error;
use bevy::{app::{App, AppExit}, MinimalPlugins};
use clap::Parser;
use cluster::{ClusterEvent, Endpoint, PeerIdentity};
use core_api::CoreApi;
use error::{WorldError, WorldResult};
use futures_util::TryStreamExt;
use log::{debug, error, info, warn};
use manager::{InstanceEvent, InstanceManager};
use once_cell::sync::{Lazy, OnceCell};
use proto::{ClusterMessage, WorldMessage, WorldRequest, WorldResponse, WorldServer};
use realm_api::{proto::{InstanceKey, NodeAddress, NodeType, RealmClient, RealmNotification, RealmRequest, RealmResponse}, RealmApi, ZoneBuilder};
use reqwest::Url;
use tokio::{runtime::Handle, select, signal, sync::{mpsc::{self, error::TryRecvError, unbounded_channel, Sender, UnboundedSender}, oneshot, Mutex}, task::LocalSet, time};
use toolkit::{print_banner, types::Uuid};

mod error;
mod proto;
mod manager;
mod instance;
mod plugins;
mod object_cache;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    pub service_core_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    pub service_realm_url: Url,

    #[arg(long, env = "REALM_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15001")]
    pub realm_zmq_addr: String,

    #[arg(long, env = "ZMQ_BIND_ADDR", default_value = "tcp://127.0.0.1:15002")]
    pub zmq_bind_url: String,

    #[arg(long, env = "INSTANCE_LIMIT", default_value_t = 100)]
    pub instance_limit: usize,

    #[arg(long, env = "ZONE_GROUPS")]
    pub zone_groups: Option<String>,

    #[arg(long, default_value_t = false)]
    pub hot_reload: bool,
}

pub static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);
pub static OBJECT_CACHE: OnceCell<ObjectCache> = OnceCell::new();

fn handle_realm_events(manager: InstanceManager, mut notifications: mpsc::Receiver<RealmNotification>) {
    tokio::spawn(async move {
        while let Some(event) = notifications.recv().await {
            match event {
                RealmNotification::InstanceRequested { transaction_id, zone, key, valid_until } => {
                    debug!("Instance requested: {:?} {:?} {:?} {:?}", transaction_id, zone, key, valid_until);
                    let _ = manager.offer_instance(transaction_id, zone, key, valid_until).await;
                },
                RealmNotification::ClusterNotification(notification) => {

                },
                _ => unimplemented!(),
            }
        }
    });
}

fn handle_realm_msgs(realm_client: Arc<RealmClient>, manager: InstanceManager) {
    tokio::spawn(async move {
        while let Ok(msg) = realm_client.recv().await {
            match msg {
                RealmResponse::InstanceOfferingAccepted { transaction_id, .. } => 
                    manager.provision_instance(transaction_id).await,
                RealmResponse::InstanceShutdownAck(label) => 
                    manager.shutdown_instance(label).await,
                _ => (),
            }
        }
    });
}

fn handle_world_msgs(server: Arc<WorldServer>, realm_api: RealmApi, event_sender: UnboundedSender<InstanceEvent>, manager: InstanceManager) {
    tokio::spawn(async move {
        let mut controllers = HashMap::<Uuid, (PeerIdentity, Sender<ControllerEvent>)>::new();
        let (sender, mut receiver) = unbounded_channel();

        let mut events = server.events();

        loop {
            select! {
                Ok((router_id, msg)) = server.recv() => {
                    match msg {
                        WorldRequest::RouterChannel { id, msg} => {
                            match msg {
                                ClusterMessage::Forward { data } => {
                                    if 
                                        let Some((_, sender)) = controllers.get(&id) &&
                                        let Ok((_, pkt)) = CPkt::from_bytes(&data)
                                    {
                                        let _ = sender.send(ControllerEvent::Packet(pkt)).await;
                                    }
                                },
                                ClusterMessage::ClientArrived { session, zone, instance, mode } => {
                                    let instance = InstanceLabel::new(
                                        zone,
                                        instance
                                    );
            
                                    let (result_send, controller) = oneshot::channel();
                                    if event_sender.send(InstanceEvent::ControllerSpawnRequested {
                                        peer: id,
                                        instance, 
                                        session, 
                                        events: sender.clone(), 
                                        controller: result_send,
                                        travel_mode: mode,
                                    }).is_ok() {
                                        match controller.await {
                                            Ok(Ok(controller)) => {
                                                debug!("Player controller spawned: {}", id);
                                                controllers.insert(id, (router_id, controller));
                                            },
                                            Ok(Err(e)) => {
                                                error!("Failed to spawn player!: {:#?}", e);
                                            }
                                            Err(_) => {
                                                error!("Controler spawn cancelled!");
                                            },
                                        }
                                    }
                                },
                                ClusterMessage::ClientLeft => {
                                    debug!("Client left: {}", id);
                                    controllers.remove(&id);
                                },
                                ClusterMessage::TravelAccepted => {
                                    if let Some((_, sender)) = controllers.get(&id) {
                                        let _ = sender.send(ControllerEvent::TravelAccepted).await;
                                    }
                                },
                                ClusterMessage::TravelRejected { reason } => {
                                    if let Some((_, sender)) = controllers.get(&id) {
                                        let _ = sender.send(ControllerEvent::TravelRejected(reason)).await;
                                    }
                                },
                            }
                        },
                    }
                },
                Ok(event) = events.recv() => {
                    if let cluster::ClusterEvent::Disconnected(peer_identity) = event {
                        controllers.retain(|_, (id, _)| *id != peer_identity);
                    }
                },
                Some(event) = receiver.recv() => {
                    match event {
                        plugins::WorldEvent::Packet { controller, pkt } => {
                            if let Some((router_id,_)) = controllers.get(&controller) {
                                let _ = server.send(router_id, WorldResponse::RouterChannel {
                                    id: controller,
                                    msg: WorldMessage::ServerMessage{ data: pkt.to_bytes() }
                                }).await;
                            }
                        },
                        plugins::WorldEvent::TravelRequest { controller, zone, instance, mode } => {
                            if let Some((router_id,_)) = controllers.get(&controller) {
                                let _ = server.send(router_id, WorldResponse::RouterChannel {
                                    id: controller,
                                    msg: WorldMessage::TravelRequest { zone, instance, mode }
                                }).await;
                            }
                        },
                        plugins::WorldEvent::TravelCommited { controller } => {
                            if let Some((router_id,_)) = controllers.get(&controller) {
                                let _ = server.send(router_id, WorldResponse::RouterChannel {
                                    id: controller,
                                    msg: WorldMessage::TravelCommited
                                }).await;
                            }
                        },
                        plugins::WorldEvent::Close { controller } => {
                            if let Some((router_id,_)) = controllers.get(&controller) {
                                let _ = server.send(router_id, WorldResponse::RouterChannel {
                                    id: controller,
                                    msg: WorldMessage::Close
                                }).await;
                            }
                        }
                    }
                },
            }
        }
    });
}

#[toolkit::service_main]
async fn main() -> WorldResult<()> {
    Lazy::force(&ARGS);

    print_banner();

    let realm_api = RealmApi::new(ARGS.service_realm_url.clone());
    let core_api = CoreApi::new(ARGS.service_core_url.clone());

    let (realm_client, notifications) = RealmClient::connect(&ARGS.realm_zmq_addr).await
        .expect("failed to connect to realm zmq server");
    let realm_client = Arc::new(realm_client);

    // subscribe to events
    realm_client.subscribe("core.session.").await?;
    realm_client.subscribe("realm.instance.").await?;

    // initialize caches
    let _ = OBJECT_CACHE.set(ObjectCache::new(realm_api.clone()));

    let (instance_event_sender, mut instance_events) = mpsc::unbounded_channel();

    let server = Arc::new(WorldServer::bind(&ARGS.zmq_bind_url).await?);
    let manager = InstanceManager::new(
        realm_api.clone(),
        core_api.clone(),
        realm_client.clone(),
        instance_event_sender.clone(),
        ARGS.instance_limit,
        &ARGS.zone_groups
            .as_ref()
            .map(|groups| groups.split(",").collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<&str>>()
    ).await?;

    // register node
    if let Endpoint::Tcp(_, port) = server.endpoint() {
        realm_client.send(RealmRequest::RegisterNode(NodeType::World, NodeAddress::Internal(*port))).await?;
    } else {
        unreachable!()
    }

    handle_realm_events(manager.clone(), notifications);
    handle_realm_msgs(realm_client.clone(), manager.clone());
    handle_world_msgs(server, realm_api.clone(), instance_event_sender.clone(), manager.clone());

    info!("Starting world server!");

    // Create bevy app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Aim for 50 cycles/sec
    let mut update_interval = time::interval(Duration::from_millis(20));   

    {
        let manager = manager.clone();
        let ctrl_c = signal::ctrl_c();
        tokio::spawn(async move {
            if ctrl_c.await.is_err() {
                warn!("Error while listening for ctrl_c signal!");
            }
        
            warn!("Shutting down world server...");
            manager.shutdown_world().await;
        });
    }

    loop {
        select! {
            _ = update_interval.tick() => {
                tokio::task::block_in_place(|| {
                    app.update();
                });
            },
            event = instance_events.recv() => {
                match event {
                    Some(event) => match event {
                        InstanceEvent::InstanceAdded(sub_app) => 
                            { app.insert_sub_app(sub_app.label(), *sub_app); },
                        InstanceEvent::InstanceRemoved(zone_label) => { 
                            app.remove_sub_app(zone_label);
                        },
                        InstanceEvent::InstanceStopping(label) => {
                            if let Some(subapp) = app.get_sub_app_mut(label.clone()) {
                                subapp.shutdown();
                                let tasks = subapp.zone_instance().task_tracker();
                                let manager = manager.clone();

                                tokio::spawn(async move {
                                    tasks.close();
                                    tasks.wait().await;
                                    manager.unregister_instance(label).await;
                                });
                            }
                        },
                        InstanceEvent::ControllerSpawnRequested { peer, instance, session, events, controller, travel_mode } => {
                            if let Some(subapp) = app.get_sub_app_mut(instance) {
                                let _ = controller.send(subapp.create_player_controller(peer, session, travel_mode, events).await);
                            } else {
                                let _ = controller.send(Err(anyhow::Error::msg("instance not found").into()));
                            }
                        },
                        InstanceEvent::WorldShutdown => {
                            info!("World server shutdown completed!");
                            break;
                        }
                    },
                    None => break,
                }
            },
        }
    }

    Ok(())
}
