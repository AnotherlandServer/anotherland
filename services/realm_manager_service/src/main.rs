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

use std::{net::SocketAddrV4, sync::Arc};

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use clap::Parser;
use core_api::CoreApi;
use core_api::proto::{CoreRequest, CoreClient, CoreNotification};
use database::DatabaseExt;
use db::{Character, PremiumCurrency, PremiumCurrencyTransaction};
use error::RealmResult;
use log::info;
use mongodb::Client;
use poem::{listener::TcpListener, post, Route, Server};
use proto::{RealmNotification, RealmServer};
use reqwest::Url;
use schema::{MutationRoot, QueryRoot};
use tokio::sync::{mpsc::Receiver, Mutex};
use toolkit::print_banner;

mod schema;
mod db;
mod proto;
mod error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env = "GRAPHQL_BIND_ADDR", default_value = "127.0.0.1:8001")]
    graphql_bind_addr: String,

    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "CORE_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15000")]
    core_zmq_addr: String,

    #[arg(long, env = "ZMQ_BIND_ADDR", default_value = "tcp://127.0.0.1:15001")]
    zmq_bind_url: String,

    #[arg(long, env = "MONGO_URI")]
    mongo_uri: String,

    #[arg(long, env = "MONGO_DB", default_value = "realm")]
    mongo_db: String,

    #[arg(long, env = "REALM_ID")]
    realm_id: i32,

    #[arg(long, env = "REALM_FRONTEND_IP")]
    frontend_ip: SocketAddrV4,
}

#[toolkit::service_main(realm)]
async fn main() -> RealmResult<()> {
    let args = Args::parse();

    print_banner();

    // Init core api
    let core_api = CoreApi::new(args.service_core_url);

    // Get realm info
    let realm = core_api.get_realm(args.realm_id).await?
        .expect("Realm not found");

    info!("Starting up realm {}...", realm.name());

    // Init database
    let client = Client::with_uri_str(&args.mongo_uri).await
        .expect("Database connection failed");
    let db = client.database(&args.mongo_db);

    db.init_collection::<Character>().await;
    db.init_collection::<PremiumCurrencyTransaction>().await;
    db.init_collection::<PremiumCurrency>().await;

    // Start graphql api
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    let app = Route::new()
        .at("/", post(GraphQL::new(schema.clone())));

    // Connect to core service
    let (core_client, core_notifications) = CoreClient::connect(&args.core_zmq_addr).await
        .expect("core service connect failed");

    // Create realm zmq server
    let server = Arc::new(RealmServer::bind(&args.zmq_bind_url).await
        .expect("failed to start realm server"));

    // Forward cluster notification to realm sub-services
    async fn forward_notifications(server: Arc<RealmServer>, mut notifications: Receiver<CoreNotification>) {
        tokio::spawn(async move {
            while let Some(notification) = notifications.recv().await {
                let _ = server.notify(RealmNotification::ClusterNotification(notification)).await;
            }
        });
    }

    forward_notifications(server.clone(), core_notifications).await;

    let _ = core_client.subscribe("core.session.").await; // subscribe to session notifications
    let _ = core_client.send(CoreRequest::ConnectRealm(args.realm_id, args.frontend_ip.into())).await;

    tokio::spawn(async move {
        info!("Starting realm server on http://{}", args.graphql_bind_addr);
        Server::new(TcpListener::bind(args.graphql_bind_addr))
            .run(app)
            .await
            .unwrap();
    })
    .await
    .unwrap();

    Ok(())
}
