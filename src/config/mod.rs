use std::net::SocketAddrV4;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfLoginServer {
    pub listen_address: SocketAddrV4,
    pub queue_listen_address: SocketAddrV4,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfRealmServer {
    pub id: u32,
    pub name: String,
    pub listen_address: SocketAddrV4,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfWorldServer {
    pub base_listen_address: SocketAddrV4,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfMain {
    pub login_server: ConfLoginServer,
    pub realm: ConfRealmServer,
    pub world: ConfWorldServer,
}

