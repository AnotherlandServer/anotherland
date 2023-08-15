mod raknet;
mod login_server;
mod realm_server;
mod queue_server;
mod atlas;

// Import modules
use std::{time::Duration, net::{SocketAddrV4, Ipv4Addr}};

use chrono::Utc;
use realm_server::RealmServer;
use nom::{error::convert_error, Finish};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::traits::PublicKeyParts;
use sha1::{Sha1, Digest};
use num_traits::cast::ToPrimitive;

// Use
use tokio::{io, signal};
use login_server::LoginServer;
use queue_server::QueueServer;

#[tokio::main]
async fn main() -> io::Result<()> {
    let login_server = LoginServer::bind_server("0.0.0.0:6112").await?;
    let realm_server = RealmServer::bind_server("0.0.0.0:6113").await?;
    let queue_server = QueueServer::bind_server("0.0.0.0:53292").await?;

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    Ok(())
}