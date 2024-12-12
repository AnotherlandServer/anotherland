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

use bevy::prelude::*;

use std::{collections::HashMap, hash::Hash, net::{IpAddr, Ipv6Addr, SocketAddr, SocketAddrV6}, sync::Arc};

use anyhow::Error;
use bevy::{app::{App, AppExit}, MinimalPlugins};
use clap::Parser;
use cluster::Endpoint;
use core_api::CoreApi;
use error::{WorldError, WorldResult};
use futures_util::TryStreamExt;
use log::info;
use manager::{InstanceEvent, InstanceManager};
use once_cell::sync::Lazy;
use proto::WorldServer;
use realm_api::{proto::{InstanceKey, NodeAddress, NodeType, RealmClient, RealmNotification, RealmRequest, RealmResponse}, RealmApi, ZoneBuilder};
use reqwest::Url;
use tokio::{runtime::Handle, sync::mpsc::{self, error::TryRecvError}};
use toolkit::{print_banner, types::Uuid};
use zone::{ZoneInstanceBuilder, ZoneSubApp};

mod error;
mod proto;
mod manager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    #[arg(long, env = "REALM_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15001")]
    realm_zmq_addr: String,

    #[arg(long, env = "ZONE_GROUP")]
    zone_group: String,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

async fn start_world_server() -> WorldResult<WorldServer> {
    for port in 49152u16 .. 65535u16 {
        match WorldServer::bind(&format!("tcp://{}", SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), port))).await {
            Ok(server) => return Ok(server),
            Err(_) => continue,
        }
    }

    Err(WorldError::Other(Error::msg("can't bin world server")))
}

fn world_runner(mut app: App) -> AppExit {
    let mut events = app.world_mut().remove_non_send_resource::<mpsc::Receiver<InstanceEvent>>()
        .expect("instance events not added to app");

    loop {
        match events.try_recv() {
            Ok(event) => match event {
                InstanceEvent::InstanceAdded(sub_app) => 
                    { app.insert_sub_app(sub_app.label(), sub_app); },
                InstanceEvent::InstanceRemoved(zone_label) => 
                    { app.remove_sub_app(zone_label); },
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break AppExit::Success,
        }
        
        app.update();
        if let Some(exit) = app.should_exit() {
            break exit;
        }
    }
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

    let server = start_world_server().await?;
    let (manager, instance_events) = InstanceManager::new(
        realm_api.clone(),
        realm_client.clone(),
        &ARGS.zone_group
    ).await?;

    // register node
    if let Endpoint::Tcp(_, port) = server.endpoint() {
        realm_client.send(RealmRequest::RegisterNode(NodeType::World, NodeAddress::Internal(*port))).await?;
    } else {
        unreachable!()
    }

    fn handle_realm_events(manager: InstanceManager, mut notifications: mpsc::Receiver<RealmNotification>) {
        tokio::spawn(async move {
            while let Some(event) = notifications.recv().await {
                match event {
                    RealmNotification::InstanceRequested { transaction_id, zone, key, valid_until } => {
                        manager.offer_instance(transaction_id, zone, key, valid_until).await;
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
                }
            }
        });
    }

    handle_realm_events(manager.clone(), notifications);
    handle_realm_msgs(realm_client.clone(), manager);

    info!("Starting world server!");

    App::new()
        .add_plugins(MinimalPlugins)
        .insert_non_send_resource(Handle::current())
        .insert_non_send_resource(instance_events)
        .set_runner(world_runner)
        .run();

    Ok(())
}
