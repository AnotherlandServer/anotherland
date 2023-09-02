mod raknet;
mod login_server;
mod realm_server;
mod queue_server;
mod world_server;
mod atlas;

// Import modules
use std::fs;

use realm_server::RealmServer;

// Use
use tokio::{io, signal};
use login_server::LoginServer;
use queue_server::QueueServer;
use world_server::WorldServer;

use crate::atlas::CParamClass_faction;

#[tokio::main]
async fn main() -> io::Result<()> {
    let bytes = fs::read("Faction_8sqzaredred.bin").expect("Read not ok");
    let (_, faction) = CParamClass_faction::from_bytes(bytes.as_ref()).expect("parse not ok");
    let faction_bytes = faction.to_bytes();

    let login_server = LoginServer::bind_server("0.0.0.0:6112").await?;
    let realm_server = RealmServer::bind_server("0.0.0.0:6113").await?;
    let world_server = WorldServer::bind_server("0.0.0.0:6114").await?;
    //let queue_server = QueueServer::bind_server("0.0.0.0:53292").await?;


    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    Ok(())
}