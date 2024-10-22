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

mod auth_session;
mod error;
mod queue_server;
mod verification_server;
mod graphql;

use std::net::SocketAddr;

use auth_session::AuthSessionContext;
use clap::{command, Parser};
use error::AppResult;
use log::info;
use once_cell::sync::Lazy;
use queue_server::start_queue_server;
use raknet::RakNetListener;
use reqwest::Url;
use toolkit::print_banner;
use verification_server::start_verification_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_AUTH_API_URL")]
    service_auth_url: Url,

    #[arg(long, env = "SERVICE_AUTH_EVENTS_ADDR")]
    service_auth_events_addr: SocketAddr,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6112")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "QUEUE_BIND_ADDR", default_value = "127.0.0.1:53292")]
    queue_bind_addr: SocketAddr,

    #[arg(long, env = "VERIFICATION_BIND_ADDR", default_value = "127.0.0.1:7998")]
    verification_bind_addr: SocketAddr,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main(cluster)]
async fn main() -> AppResult<()> {
    Lazy::force(&ARGS);

    print_banner();

    start_queue_server(ARGS.queue_bind_addr).await?;
    start_verification_server(ARGS.verification_bind_addr).await?;

    // raknet auth server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(ARGS.raknet_bind_addr).await?;
        listener.generate_random_rsa_key();
        listener.listen(100).await;
    
        info!("Server started...");
    
        loop {
            let socket = listener.accept().await.unwrap();
            AuthSessionContext::start_auth_session(socket);
        }
    }).await?
}
