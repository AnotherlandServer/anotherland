// Copyright (C) 2023 AnotherlandServer
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

mod util;
mod db;
mod config;
mod cluster;
mod data_import;
mod components;
mod frontends;

// Import modules
use std::net::Ipv4Addr;
use clap::{Parser, Subcommand};
use cluster::actor::ClusterNode;
use ::config::File;
use log::{LevelFilter, info};
use log4rs::{self, append::console::ConsoleAppender, Config, config::{Appender, Root}};
use glob::glob;
use once_cell::sync::Lazy;
use mongodb::bson::doc;
use tokio::{signal, sync::RwLock};

use util::AnotherlandResult;
use crate::{config::ConfMain, cluster::ServerRunner, data_import::import_client_data, db::{database, initalize_db, realm_database, ZoneDef}, components::SessionManager, frontends::LoginFrontend};
use crate::components::{Authenticator, SessionHandler};
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

async fn init_database() -> AnotherlandResult<()> {
    

    Ok(())
}

#[tokio::main]  //(flavor = "current_thread")
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
    let mut servers: Vec<()> = Vec::new();
    //let mut api_server = None;

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
            NODE.add_actor(Authenticator::new().await);
            NODE.add_actor(SessionManager::new().await?);
            NODE.add_frontend("login_frontend", LoginFrontend::new());
            //servers.push(ServerRunner::new::<LoginServer>(()));
        },
        StartCommand::RealmServer { .. } => {
            //servers.push(ServerRunner::new::<RealmServer>(()));
        },
        StartCommand::FrontendServer { .. } => {
            //servers.push(ServerRunner::new::<FrontendServer>(()));
        }
        StartCommand::NodeServer { .. } => {
            //WORLD_SERVER_IDS.write().await.push_front(*world_id);

            //servers.push(ServerRunner::new::<WorldServer>());
        },
        StartCommand::InstancePoolServer { .. } => {

        },
        StartCommand::ApiServer => {
            /*let server = ApiServer::new().await?;
            server.start().await?;

            api_server = Some(server);*/
        },
        StartCommand::StandaloneServer { .. } => {
            init_database().await?;

            NODE.add_actor(Authenticator::new().await);
            NODE.add_actor(SessionManager::new().await?);
            NODE.add_frontend("login_frontend", LoginFrontend::new());

            // start api server
            /*let server = ApiServer::new().await?;
            server.start().await?;
            
            api_server = Some(server);

            // load all zones
            let zones = ZoneDef::list(realm_database().await).await?.into_iter().map(|z| z.guid);

            servers.push(ServerRunner::new::<LoginServer>(()));
            servers.push(ServerRunner::new::<RealmServer>(()));
            servers.push(ServerRunner::new::<FrontendServer>(()));

            for zone_guid in zones {
                servers.push(ServerRunner::new::<NodeServer>(NodeServerOptions {
                    realm_id: CONF.realm.id,
                    zone_guid
                }));
            }*/
        }
    }

    if !servers.is_empty() {
        match signal::ctrl_c().await {
            Ok(()) => {
                /*if let Some(api_server) = api_server {
                    api_server.stop().await;
                }

                for server in servers.drain(..) {
                    server.stop().await;
                }*/
            },
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
                // we also shut down in case of error
            },
        }
    }

    Ok(())
}