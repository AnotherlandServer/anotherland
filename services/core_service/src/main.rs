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

use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use clap::Parser;
use database::DatabaseExt;
use db::{Account, Realm, Session, Status};
use log::info;
use mongodb::Client;
use poem::{listener::TcpListener, post, Route, Server};
use schema::{MutationRoot, QueryRoot};
use tokio::sync::Mutex;
use toolkit::print_banner;
use zeromq::{PubSocket, Socket};

mod db;
mod schema;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env = "GRAPHQL_BIND_ADDR", default_value = "127.0.0.1:8000")]
    graphql_bind_addr: String,

    #[arg(long, env = "AUTH_EVENTS_BIND_URL", default_value = "tcp://127.0.0.1:15000")]
    auth_events_bind_url: String,

    #[arg(long, env = "REALM_EVENTS_BIND_URL", default_value = "tcp://127.0.0.1:15001")]
    realm_events_bind_url: String,

    #[arg(long, env = "MONGO_URI")]
    mongo_uri: String,

    #[arg(long, env = "MONGO_DB", default_value = "auth")]
    mongo_db: String,
}

type EventSocket = Arc<Mutex<PubSocket>>;

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

    // Open auth event socket
    let mut socket = PubSocket::new();
    let endpoint = socket.bind(&args.auth_events_bind_url).await
        .expect("failed to bind auth event socket");

    info!("Publishing auth events on {}", endpoint);

    // Start graphql api
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .data(Arc::new(Mutex::new(socket)))
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
