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

mod util;
mod db;
mod config;
mod cluster;
mod data_import;
mod actors;
mod components;
mod frontends;

// Import modules
use std::net::Ipv4Addr;
use atlas::{CommonConfigClass, OaCommonConfigParams};
use clap::{Parser, Subcommand};
use cluster::ClusterNode;
use actors::{Realm, RealmList, Social, ZoneRegistry};
use ::config::File;
use db::WorldDef;
use frontends::{ApiFrontend, ClusterFrontend, LoginQueueFrontend, RealmFrontend, ZoneFrontend};
use log::{LevelFilter, info, warn, error};
use log4rs::{self, append::console::ConsoleAppender, Config, config::{Appender, Root}};
use glob::glob;
use once_cell::sync::Lazy;
use mongodb::bson::doc;
use rcgen::Certificate;
use tokio::signal;

use tokio_stream::StreamExt;
use util::AnotherlandResult;
use crate::{config::ConfMain, data_import::import_client_data, db::{database, initalize_db, realm_database, ZoneDef, DatabaseRecord, MiscContent}, actors::SessionManager, frontends::LoginFrontend};
use crate::actors::Authenticator;
//use crate::{login_server::LoginServer, realm_server::RealmServer, frontend_server::FrontendServer, node_server::{NodeServer, NodeServerOptions}, api_server::ApiServer};

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
        #[arg(long, env = "REALM_ID")]
        realm_id: i32,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    FrontendServer {
        #[arg(long, env = "REALM_ID")]
        realm_id: i32,

        #[arg(long, env = "MONGO_REALM_DB")]
        mongo_realm_db: String,
    },
    NodeServer {
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
    ApiServer,
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
            StartCommand::NodeServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::InstancePoolServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::StandaloneServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::DataImport { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            StartCommand::RealmServer { mongo_realm_db, .. } => Some(mongo_realm_db.clone()),
            _ => None
        }
    }
}

static ARGS: Lazy<Args> = Lazy::new(Args::parse);

static CONF: Lazy<ConfMain> = Lazy::new(|| {
    type Config = ::config::Config;
    
    let mut builder = Config::builder()
        .add_source(
            glob("conf/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );

    if cfg!(unix) {
        builder = builder.add_source(
            glob("/etc/anotherland/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );
    }
        
    builder
        .build()
        .unwrap()
        .try_deserialize::<ConfMain>()
        .expect("Failed to parse config")
});

static NODE: Lazy<ClusterNode> = Lazy::new(ClusterNode::new);

static CLUSTER_CERT: Lazy<Certificate> = Lazy::new(||rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap());

async fn init_database() -> AnotherlandResult<()> {
    

    Ok(())
}

async fn initialize_login_server() -> AnotherlandResult<()> {
    NODE.add_actor(Authenticator::initialize().await?);
    NODE.add_actor(SessionManager::initialize().await?);
    NODE.add_actor(RealmList::initialize().await);
    NODE.add_frontend(LoginFrontend::initialize().await?);
    NODE.add_frontend(LoginQueueFrontend::initialize().await?);

    Ok(())
}

async fn initialize_realm_server() -> AnotherlandResult<()> {
    NODE.add_actor(Realm::initialize().await?);
    NODE.add_frontend(RealmFrontend::initialize().await?);

    Ok(())
}

async fn initialize_cluster_frontend_server() -> AnotherlandResult<()> {
    NODE.add_actor(ZoneRegistry::initialize().await?);
    NODE.add_actor(Social::initialize().await?);
    NODE.add_frontend(ClusterFrontend::initialize().await?);

    Ok(())
}

async fn initialize_zone_server(world_def: WorldDef, zone_def: ZoneDef) -> AnotherlandResult<()> {
    NODE.add_frontend(ZoneFrontend::initialize(world_def, zone_def).await?);

    Ok(())
}

async fn initialize_api_server() -> AnotherlandResult<()> {
    NODE.add_frontend(ApiFrontend::initialize().await?);

    Ok(())
}

#[tokio::main]
async fn main() -> AnotherlandResult<()> {
    let _ = dotenvy::dotenv();

    // Setup logging
    if log4rs::init_file("log4rs.yaml", Default::default()).is_err() {
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

    match &ARGS.start_command {
        StartCommand::InitDb => {
            init_database().await?;
        },
        StartCommand::DataImport { path_to_client, .. } => {
            init_database().await?;
            import_client_data(path_to_client.into()).await?;

            return Ok(());
        }
        StartCommand::LoginServer { .. } => {
            todo!()
        },
        StartCommand::RealmServer { .. } => {
            todo!()
        },
        StartCommand::FrontendServer { .. } => {
            todo!()
        }
        StartCommand::NodeServer { .. } => {
            todo!()
        },
        StartCommand::InstancePoolServer { .. } => {
            todo!()
        },
        StartCommand::ApiServer => {
            todo!()
        },
        StartCommand::StandaloneServer { .. } => {
            Lazy::force(&CLUSTER_CERT);

            init_database().await?;

            initialize_login_server().await?;
            initialize_realm_server().await?;
            initialize_cluster_frontend_server().await?;
            initialize_api_server().await?;

            // load all active maps
            {
                let db = realm_database().await;

                if let Some(config) = MiscContent::get_by_name(db.clone(), "ActiveMaps").await?
                    .as_ref()
                    .and_then(|v| v.data.as_ref())
                    .and_then(|v| v.get::<CommonConfigClass>().ok())
                    .and_then(|v| v.value())
                {
                    if let Some(active_maps) = config.get("activeMaps") {
                        for map in active_maps.as_array().unwrap() {
                            // load world by name
                            if let Some(world_def) = WorldDef::get_by_name(db.clone(), map["map"].as_str().unwrap()).await? {
                                // load and spawn world zones
                                for zone in ZoneDef::load_for_world(db.clone(), &world_def.guid).await? {
                                    initialize_zone_server(world_def.clone(), zone).await?;
                                }
                            } else {
                                error!("World {} not found!", map["map"].as_str().unwrap());
                            }
                        }
                    }
                } else {
                    warn!("No active maps found!");
                }
            }

            // load all persistent zones
            // the game does differentiate between primary, secondary and tertriary servers
            // according to the instance.db database. 
            // as we currently don't know how the original server architecture defined those,
            // we currently treat all those zones as "primary", giving each one a dedicated zone actor
            // with a single shared udp server (cluster frontend).
            {
                let db = realm_database().await;

                let zone_collection = ZoneDef::collection(db.clone());
                let mut zones = zone_collection.find(doc!{
                    "$or": [
                        // todo: normalize this field during import
                        {"server": {"$eq": "Primary"}},
                        {"server": {"$eq": "primary"}},
                        {"server": {"$eq": "Secondary"}},
                        {"server": {"$eq": "secondary"}},
                        {"server": {"$eq": "Tertriary"}},
                        {"server": {"$eq": "tertriary"}}
                    ]
                }, None).await?;

                while let Some(zone) = zones.try_next().await? {
                    if let Some(world_def) = WorldDef::get_by_guid(db.clone(), &zone.worlddef_guid).await? {
                        initialize_zone_server(world_def, zone).await?;
                    } else {
                        warn!("Skipping zone {} - {}, world not found", zone.zone, zone.guid);
                    }
                }
            }
        }
    }

    NODE.start().await;

    match signal::ctrl_c().await {
        Ok(()) => {
            NODE.stop().await;
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    Ok(())
}