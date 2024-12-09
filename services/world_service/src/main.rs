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

use std::{collections::HashMap, hash::Hash, net::{IpAddr, Ipv6Addr, SocketAddr, SocketAddrV6}};

use anyhow::Error;
use bevy::{app::App, MinimalPlugins};
use clap::Parser;
use cluster::Endpoint;
use core_api::CoreApi;
use error::{WorldError, WorldResult};
use futures_util::TryStreamExt;
use log::info;
use once_cell::sync::Lazy;
use proto::WorldServer;
use realm_api::{proto::{NodeType, RealmClient, RealmRequest}, RealmApi, ZoneBuilder};
use reqwest::Url;
use toolkit::{print_banner, types::Uuid};
use zone::{ZoneInstanceBuilder, ZoneSubApp};

mod error;
mod proto;

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

    // subscribe to events
    realm_client.subscribe("core.session.").await?;

    let mut server = start_world_server().await?;

    // register node
    if let Endpoint::Tcp(_, port) = server.endpoint() {
        realm_client.send(RealmRequest::RegisterNode(NodeType::WorldNode, SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), *port))).await?;
    } else {
        unreachable!()
    }

    let mut cursor = realm_api.query_zones()
        .server(ARGS.zone_group.clone())
        .query().await?;

    info!("Creating zone instances");

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    
    while let Some(zone) = cursor.try_next().await? {
        info!("Spawning zone: {}...", zone.zone());

        let zone = ZoneInstanceBuilder::default()
            .realm_api(realm_api.clone())
            .zone(zone)
            .instantiate()
            .await?;

        realm_client.send(RealmRequest::RegisterInstance { 
            zone: zone.zone_id(), 
            instance: zone.instance_id(), 
        }).await?;

        app.insert_sub_app(zone.label(), zone);
    }

    info!("Starting world server!");
    app.run();

    info!("Server stopped");

    Ok(())
}
