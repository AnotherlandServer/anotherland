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

use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use once_cell::sync::Lazy;
use raknet::RakNetListener;
use reqwest::Url;
use tokio::time::sleep;
use log::debug;

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
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main]
async fn main() {
    let mut listener = RakNetListener::bind("0.0.0.0:6112").await.unwrap();
    listener.listen(100).await;

    loop {
        let socket = listener.accept().await;
        debug!("Got new connection!");
        sleep(Duration::from_millis(10)).await;
    }
}
