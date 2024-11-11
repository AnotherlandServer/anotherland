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

use std::net::SocketAddr;

use auth_session::AuthSessionContext;
use clap::Parser;
use core_api::{proto::{CoreClient, CoreNotification}, CoreApi};
use error::AppResult;
use log::info;
use queue_server::start_queue_server;
use raknet::RakNetListener;
use reqwest::Url;
use tokio::sync::{broadcast, mpsc::Receiver};
use toolkit::print_banner;
use verification_server::start_verification_server;

mod error;
mod auth_session;
mod queue_server;
mod verification_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct LoginServerOptions {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "CORE_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15000")]
    core_zmq_addr: String,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6112")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "QUEUE_BIND_ADDR", default_value = "0.0.0.0:53292")]
    queue_bind_addr: SocketAddr,

    #[arg(long, env = "VERIFICATION_BIND_ADDR", default_value = "0.0.0.0:7998")]
    verification_bind_addr: SocketAddr,
}

#[toolkit::service_main(cluster)]
async fn main() -> AppResult<()> {
    let opts = LoginServerOptions::parse();

    print_banner();

    let core_api = CoreApi::new(opts.service_core_url.clone());
    let (core_client, notifications) = CoreClient::connect(&opts.core_zmq_addr).await
        .expect("failed to connect to core zmq server");

    core_client.subscribe("core.realms.").await?;

    start_queue_server(opts.queue_bind_addr).await?;
    start_verification_server(core_api.clone(), opts.verification_bind_addr).await?;

    let (realm_update_sender, _) = broadcast::channel(100);

    // listen for realm updates
    fn forward_realm_updates(realm_update_sender: broadcast::Sender<()>, mut notifications: Receiver<CoreNotification>) {
        tokio::spawn(async move {
            while let Some(notification) = notifications.recv().await {
                if let CoreNotification::RealmListUpdated = notification {
                    let _ = realm_update_sender.send(());
                }
            }
        });
    }

    forward_realm_updates(realm_update_sender.clone(), notifications);

    // raknet auth server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(opts.raknet_bind_addr).await?;
        //listener.generate_random_rsa_key();
        listener.listen(100).await;
    
        info!("Server started...");
    
        loop {
            let socket = listener.accept().await.unwrap();
            AuthSessionContext::start_auth_session(core_api.clone(), socket, realm_update_sender.subscribe());
        }
    }).await?
}
