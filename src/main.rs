mod util;
mod db;
mod config;
mod raknet;
mod login_server;
/*mod realm_server;
mod world_server;*/
mod queue_server;
mod atlas;

// Import modules
use std::{fs, path::Path, env};
use ::config::File;
use log::{LevelFilter, info, debug};
use log4rs::{self, append::console::ConsoleAppender, Config, config::{Appender, Root, Logger}};
use glob::glob;

use nom::error::ErrorKind;
use once_cell::sync::Lazy;
use surrealdb::{Surreal, engine::{local::{Db, Mem, SpeeDb}, remote::ws::Ws}};
// Use
use tokio::{io, signal, sync::RwLock};
use queue_server::QueueServer;
use util::AnotherlandResult;
/*use login_server::LoginServer;
use realm_server::RealmServer;
use world_server::WorldServer;*/

use crate::{atlas::CParamClass_faction, raknet::RakNetListener, config::ConfMain, login_server::LoginServer, db::AccountRecord};

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);
static CONF: Lazy<ConfMain> = Lazy::new(|| {
    type Config = ::config::Config;
    
    Config::builder()
        .add_source(
            glob("conf/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .build()
        .unwrap()
        .try_deserialize::<ConfMain>()
        .expect("Failed to parse config")
});

#[tokio::main]
async fn main() -> AnotherlandResult<()> {
    // Setup logging
    if let Err(_) = log4rs::init_file("log4rs.yaml", Default::default()) {
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    // Database
    info!("Opening database...");
    DB.connect::<SpeeDb>(env::current_dir().unwrap().join("data/database")).await?;
    DB.use_ns("anotherland").use_db("anotherland").await?;

    info!("Applying schema...");
    DB.query(include_str!(concat!(env!("OUT_DIR"), "/schema.surql"))).await?;

    // Create Admin account if it doesn't exist yet
    debug!("Admin account: {:#?}", AccountRecord::get_by_username_or_mail("admin").await?);
    if AccountRecord::get_by_username_or_mail("admin").await?.is_none() {
        AccountRecord::create("admin".to_owned(), "admin@localhost".to_owned(), "1234".to_owned()).await?;

        info!("=========== ADMIN ACCOUNT PASSWORD ===========");
        info!("1234");
        info!("==============================================");
    }

    let login_server = LoginServer::init().await?;
    //let queue_server = QueueServer::bind_server("0.0.0.0:53292").await?;

    //let login_server = LoginServer::bind_server("0.0.0.0:6112").await?;
    //let realm_server = RealmServer::bind_server("0.0.0.0:6113").await?;
    //let world_server = WorldServer::bind_server("0.0.0.0:6114").await?;

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    Ok(())
}