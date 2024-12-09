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

use std::net::SocketAddr;

use clap::Parser;
use core_api::CoreApi;
use error::FrontendResult;
use frontend_session_context::FrontendSessionContext;
use log::info;
use once_cell::sync::Lazy;
use raknet::RakNetListener;
use realm_api::{proto::{NodeAddress, NodeType, RealmClient, RealmRequest}, RealmApi};
use reqwest::Url;
use toolkit::print_banner;

mod error;
mod frontend_session_context;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    #[arg(long, env = "REALM_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15001")]
    realm_zmq_addr: String,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6113")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "PUBLIC_ADDR")]
    public_addr: SocketAddr,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main]
async fn main() -> FrontendResult<()> {
    Lazy::force(&ARGS);

    print_banner();

    let realm_api = RealmApi::new(ARGS.service_realm_url.clone());
    let core_api = CoreApi::new(ARGS.service_core_url.clone());

    let (realm_client, notifications) = RealmClient::connect(&ARGS.realm_zmq_addr).await
        .expect("failed to connect to realm zmq server");

    // subscribe to events
    realm_client.subscribe("core.session.").await?;

    // raknet server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(ARGS.raknet_bind_addr).await?;
        listener.generate_random_rsa_key();
        listener.listen(100).await;
    
        info!("Server started...");

        // notify realm server we're online
        realm_client.send(RealmRequest::RegisterNode(NodeType::Frontend, NodeAddress::Public(ARGS.public_addr))).await?;
    
        loop {
            let socket = listener.accept().await.unwrap();
            FrontendSessionContext::start_frontend_session(
                core_api.clone(), 
                realm_api.clone(), 
                socket
            );
        }
    }).await?
}
