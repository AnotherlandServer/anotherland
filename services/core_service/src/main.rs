// Copyright (C) 2025 AnotherlandServer
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

use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use clap::Parser;
use core_server_runner::run_core_server;
use database::DatabaseExt;
use db::{Account, Realm, Session, Status};
use log::info;
use mongodb::Client;
use poem::{listener::TcpListener, post, Route, Server};
use proto::CoreServer;
use realm_status_registry::RealmStatusRegistry;
use schema::{MutationRoot, QueryRoot};
use toolkit::print_banner;

mod db;
mod schema;
mod proto;
mod realm_status_registry;
mod core_server_runner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env = "GRAPHQL_BIND_ADDR", default_value = "127.0.0.1:8000")]
    graphql_bind_addr: String,

    #[arg(long, env = "ZMQ_BIND_ADDR", default_value = "tcp://127.0.0.1:15000")]
    zmq_bind_url: String,

    #[arg(long, env = "MONGO_URI")]
    mongo_uri: String,

    #[arg(long, env = "MONGO_DB", default_value = "core")]
    mongo_db: String,
}

#[toolkit::service_main(cluster)]
async fn main() {
    let args = Args::parse();

    print_banner();

    // Init database
    let client = Client::with_uri_str(&args.mongo_uri).await
        .expect("Database connection failed");
    let db = client.database(&args.mongo_db);

    // Init collections
    db.init_collection::<Account>().await;
    db.init_collection::<Session>().await;
    db.init_collection::<Status>().await;
    db.init_collection::<Realm>().await;

    // Cluster server
    let server = Arc::new(
        CoreServer::bind(&args.zmq_bind_url).await
            .expect("failed to start cluster server")
    );

    // Status registry
    let status_registry = Arc::new(RealmStatusRegistry::new(server.clone()));

    // Run server
    run_core_server(server.clone(), status_registry.clone()).await;

    // Start graphql api
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .data(server)
        .data(status_registry)
        .finish();

    let app = Route::new()
        .at("/", post(GraphQL::new(schema.clone())));

    tokio::spawn(async move {
        info!("Starting core server on http://{}", args.graphql_bind_addr);
        Server::new(TcpListener::bind(args.graphql_bind_addr))
            .run(app)
            .await
            .unwrap();
    })
    .await
    .unwrap()
}
