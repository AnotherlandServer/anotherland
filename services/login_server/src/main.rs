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
use core_api::CoreApi;
use error::AppResult;
use log::info;
use queue_server::start_queue_server;
use raknet::RakNetListener;
use reqwest::Url;
use toolkit::print_banner;
use verification_server::start_verification_server;

mod error;
mod auth_session;
mod queue_server;
mod verification_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct LoginServerOptions {
    #[arg(long, env = "SERVICE_AUTH_API_URL", default_value = "http://127.0.0.1:8000")]
    service_auth_url: Url,

    #[arg(long, env = "SERVICE_AUTH_EVENTS_ADDR", default_value = "tcp://127.0.0.1:15000")]
    service_auth_events_addr: String,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6112")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "QUEUE_BIND_ADDR", default_value = "127.0.0.1:53292")]
    queue_bind_addr: SocketAddr,

    #[arg(long, env = "VERIFICATION_BIND_ADDR", default_value = "127.0.0.1:7998")]
    verification_bind_addr: SocketAddr,
}

#[toolkit::service_main(cluster)]
async fn main() -> AppResult<()> {
    let opts = LoginServerOptions::parse();

    print_banner();

    let auth_api = CoreApi::new(opts.service_auth_url.clone());

    start_queue_server(opts.queue_bind_addr).await?;
    start_verification_server(auth_api.clone(), opts.verification_bind_addr).await?;

    // raknet auth server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(opts.raknet_bind_addr).await?;
        listener.generate_random_rsa_key();
        listener.listen(100).await;
    
        info!("Server started...");
    
        loop {
            let socket = listener.accept().await.unwrap();
            AuthSessionContext::start_auth_session(auth_api.clone(), socket);
        }
    }).await?
}
