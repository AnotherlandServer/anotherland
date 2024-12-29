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

use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use cluster_context::ClusterContext;
use core_api::CoreApi;
use error::ClusterFrontendResult;
use once_cell::sync::Lazy;
use protocol::CPkt;
use raknet::RakNetListener;
use realm_api::{proto::{NodeAddress, NodeType, RealmClient, RealmRequest}, RealmApi};
use reqwest::Url;
use log::{error, info};
use router::Router;
use toolkit::print_banner;

mod error;
mod cluster_context;
mod router;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    #[arg(long, env = "REALM_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15001")]
    realm_zmq_addr: String,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6114")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "PUBLIC_ADDR")]
    public_addr: SocketAddr,

    #[arg(long, default_value_t = false)]
    insecure: bool,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main]
async fn main() -> ClusterFrontendResult<()> {
    print_banner();

    let realm_api = RealmApi::new(ARGS.service_realm_url.clone());
    let core_api = CoreApi::new(ARGS.service_core_url.clone());

    let (realm_client, notifications) = RealmClient::connect(&ARGS.realm_zmq_addr).await
        .expect("failed to connect to realm zmq server");

    realm_client.subscribe("core.session.terminated").await?;

    let router = Router::new(realm_api.clone());

    // raknet server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(ARGS.raknet_bind_addr).await?;

        if !ARGS.insecure {
            listener.generate_random_rsa_key();
        }
        
        listener.listen(100).await;

        info!("Server started...");

        // notify realm server we're online
        realm_client.send(RealmRequest::RegisterNode(NodeType::Cluster, NodeAddress::Public(ARGS.public_addr))).await?;

        loop {
            let socket = listener.accept().await?;
            let realm_api = realm_api.clone();
            let core_api = core_api.clone();
            let router = router.clone();

            tokio::spawn(async move {
                // Silently drop all connections which do not send oaPktRequestEnterGame as
                // their first message or whose session is invalid.
                if 
                    let Ok(pkt) = socket.recv().await &&
                    let Ok((_, CPkt::oaPktRequestEnterGame(pkt))) = CPkt::from_bytes(&pkt) &&
                    let Ok(Some(session)) = core_api.get_session(&pkt.session_id).await
                {
                    if let Err(e) = ClusterContext::create_and_start(
                        core_api, 
                        realm_api, 
                        router, 
                        socket, 
                        session
                    ).await {
                        error!("Failed to start cluster session: {:#?}", e);
                    }
                }
            });
        }
    }).await?
}
