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
use error::FrontendResult;
use log::info;
use once_cell::sync::Lazy;
use raknet::RakNetListener;
use reqwest::Url;
use toolkit::print_banner;

mod error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_AUTH_API_URL")]
    service_auth_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL")]
    service_realm_url: Url,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6113")]
    raknet_bind_addr: SocketAddr,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main]
async fn main() -> FrontendResult<()> {
    Lazy::force(&ARGS);

    print_banner();

    // raknet server
    tokio::spawn(async move {
        let mut listener = RakNetListener::bind(ARGS.raknet_bind_addr).await?;
        listener.generate_random_rsa_key();
        listener.listen(100).await;
    
        info!("Server started...");
    
        loop {
            let socket = listener.accept().await.unwrap();
        }
    }).await?
}
