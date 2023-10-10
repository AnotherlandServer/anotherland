mod util;
mod db;
mod config;
mod login_server;
mod realm_server;
mod world_server;
mod cluster;
mod world;
mod data_import;

// Import modules
use std::{net::Ipv4Addr, collections::VecDeque};
use clap::{Parser, Subcommand};
use ::config::File;
use log::{LevelFilter, info};
use log4rs::{self, append::console::ConsoleAppender, Config, config::{Appender, Root}};
use glob::glob;
use once_cell::sync::Lazy;
use mongodb::bson::doc;
use tokio::{signal, sync::RwLock};

use util::AnotherlandResult;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env = "EXTERNAL_IP")]
    external_ip: Ipv4Addr,

    #[arg(long, env = "MONGO_URI")]
    mongo_uri: String,

    #[arg(long, env = "MONGO_CLUSTER_DB", default_value = "anotherland")]
    mongo_cluster_db: String,

    #[command(subcommand)]
    start_command: StartCommand,
}

#[derive(Subcommand)]
enum StartCommand {
    InitDb,
    DataImport {
        path_to_client: String,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    LoginServer {
        #[arg(long, env = "MAX_ACTIVE_SESSIONS")]
        max_active_sessions: usize
    },
    RealmServer {
        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    WorldServer {
        #[arg(long)]
        world_id: u16,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    InstancePoolServer {
        #[arg(long, env = "MAX_INSTANCES")]
        max_instances: usize,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    StandaloneServer {
        #[arg(long, env = "MAX_INSTANCES")]
        max_instances: usize,

        #[arg(long, env = "MAX_ACTIVE_SESSIONS")]
        max_active_sessions: usize,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    }
}

impl Args {
    pub fn max_instances(&self) -> Option<usize> {
        match self.start_command {
            StartCommand::InstancePoolServer { max_instances, .. } => Some(max_instances),
            StartCommand::StandaloneServer { max_instances, .. } => Some(max_instances),
            _ => None,
        }
    }

    pub fn max_active_sessions(&self) -> Option<usize> {
        match self.start_command {
            StartCommand::LoginServer { max_active_sessions } => Some(max_active_sessions),
            StartCommand::StandaloneServer { max_active_sessions, .. } => Some(max_active_sessions),
            _ => None,
        }
    }

    pub fn mongo_realm_db(&self) -> Option<String> {
        match &self.start_command {
            StartCommand::WorldServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::InstancePoolServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::StandaloneServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::DataImport { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::RealmServer { mongo_realm_db } => Some(mongo_realm_db.clone()),
            _ => None
        }
    }
}

use crate::{config::ConfMain, login_server::LoginServer, realm_server::RealmServer, cluster::ServerRunner, data_import::import_client_data, db::{database, initalize_db, realm_database, WorldDef}, world_server::WorldServer};

static ARGS: Lazy<Args> = Lazy::new(Args::parse);

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

// Todo: This is a hack, but I don't want to refactor the server runner / server instance impl
// to accept parameters on creation.
static WORLD_SERVER_IDS: Lazy<RwLock<VecDeque<u16>>> = Lazy::new(||RwLock::new(VecDeque::new()));

async fn init_database() -> AnotherlandResult<()> {
    

    Ok(())
}

#[tokio::main] // (flavor = "current_thread")
async fn main() -> AnotherlandResult<()> {
    let _ = dotenvy::dotenv();

    // Setup logging
    if let Err(_) = log4rs::init_file("log4rs.yaml", Default::default()) {
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    // Load config
    Lazy::force(&CONF);

    info!("Testing database connection...");
    database("admin")
        .await
        .run_command(doc! {"ping": 1}, None)
        .await?;

    info!("Setting up database...");
    initalize_db().await?;

    // Start tokio runtime
    let mut servers = Vec::new();

    match &ARGS.start_command {
        StartCommand::InitDb => {
            init_database().await?;
        },
        StartCommand::DataImport { path_to_client, .. } => {
            // Init database anyway
            init_database().await?;
            import_client_data(path_to_client.into()).await?;
        }
        StartCommand::LoginServer { .. } => {
            servers.push(ServerRunner::new::<LoginServer>());
        },
        StartCommand::RealmServer { .. } => {
            servers.push(ServerRunner::new::<RealmServer>());
        },
        StartCommand::WorldServer { world_id, .. } => {
            WORLD_SERVER_IDS.write().await.push_front(*world_id);

            servers.push(ServerRunner::new::<WorldServer>());
        },
        StartCommand::InstancePoolServer { .. } => {

        },
        StartCommand::StandaloneServer { .. } => {
            init_database().await?;

            // load all worlds
            let worlds = WorldDef::list(realm_database().await).await?;
            let world_count = worlds.len();

            WORLD_SERVER_IDS.write().await.append(&mut worlds.into_iter().map(|m| m.id).collect::<VecDeque<u16>>());

            servers.push(ServerRunner::new::<LoginServer>());
            servers.push(ServerRunner::new::<RealmServer>());

            for _ in 1..=world_count {
                servers.push(ServerRunner::new::<WorldServer>());
            }
        }
    }

    if !servers.is_empty() {
        match signal::ctrl_c().await {
            Ok(()) => {
                for server in servers.drain(..) {
                    server.stop().await;
                }
            },
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
                // we also shut down in case of error
            },
        }
    }

    Ok(())
}