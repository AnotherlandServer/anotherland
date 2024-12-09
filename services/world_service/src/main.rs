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

use std::{collections::HashMap, hash::Hash, net::{IpAddr, Ipv6Addr, SocketAddr, SocketAddrV6}, sync::Arc};

use anyhow::Error;
use bevy::{app::App, MinimalPlugins};
use clap::Parser;
use cluster::Endpoint;
use core_api::CoreApi;
use error::{WorldError, WorldResult};
use futures_util::TryStreamExt;
use log::info;
use manager::InstanceManager;
use once_cell::sync::Lazy;
use proto::WorldServer;
use realm_api::{proto::{InstanceKey, NodeAddress, NodeType, RealmClient, RealmNotification, RealmRequest}, RealmApi, ZoneBuilder};
use reqwest::Url;
use tokio::sync::mpsc;
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

    let mut server = start_world_server().await?;
    let manager = InstanceManager::new(
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
                    RealmNotification::InstanceRequested { transaction_id, zone, valid_until, .. } => {
                        manager.offer_instance(transaction_id, zone, valid_until).await;
                    },
                    RealmNotification::ClusterNotification(notification) => {
    
                    },
                    _ => unimplemented!(),
                }
            }
        });
    }

    handle_realm_events(manager, notifications);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    info!("Starting world server!");
    app.run();

    info!("Server stopped");

    Ok(())
}
