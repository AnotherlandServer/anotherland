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

use std::{sync::Arc, ops::Deref, convert::Infallible, collections::HashMap};

use async_graphql::{Schema, EmptySubscription, http::GraphiQLSource, FieldResult};
use async_graphql_poem::GraphQL;
use async_trait::async_trait;
use log::{info, error};
use poem::{Route, IntoResponse, web::Html, handler, get, Server, listener::TcpListener};
use tokio::{sync::{RwLock, oneshot::{Sender, self}}};
use atlas::Uuid;
use crate::{cluster::{ServerInstance, ClusterMessage, MessageChannel, ApiResponse, ApiRequest, connect_queue, MessageQueueConsumer, MessageQueueProducer, ApiError}, util::AnotherlandResult, CONF};

use super::schema::{QueryRoot, MutationRoot};

pub struct ApiServerData {
    api_cluster: MessageQueueProducer,
    api_realm: HashMap<u32, MessageQueueProducer>,

    active_requests: HashMap<Uuid, Sender<ApiResponse>>,
}

#[derive(Clone)]
pub struct ApiServer(Arc<RwLock<ApiServerData>>);

impl Deref for ApiServer {
    type Target = Arc<RwLock<ApiServerData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

impl ApiServer {
    pub async fn new() -> AnotherlandResult<ApiServer> {
        let (api_cluster, _) = connect_queue(MessageChannel::ClusterApiChannel).await?;

        Ok(ApiServer(Arc::new(RwLock::new(ApiServerData { 
            api_cluster,
            api_realm: HashMap::new(),
            active_requests: HashMap::new(),
        }))))
    }

    pub async fn start(&self) -> AnotherlandResult<()> {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(self.clone())
            .finish();

        let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

        tokio::spawn(async move {
            Server::new(TcpListener::bind(CONF.api.listen_address))
                .run(app)
                .await
                .unwrap();
        });

        let api = self.clone();

        tokio::spawn(async move {
            let (_, mut api_frontend) = connect_queue(MessageChannel::ApiFrontend).await.unwrap();
            
            while let Ok(m) = api_frontend.recv().await {
                match m {
                    ClusterMessage::ApiResponse { request_id, response } => {
                        let mut api = api.write().await;

                        if let Some(req) = api.active_requests.remove(&request_id) {
                            req.send(response).unwrap();
                        }
                    },
                    _ => panic!("Received unexpected message on api channel: {:#?}", m),
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&self) {

    }

    // query the cluster
    async fn query(&self, channel: &MessageQueueProducer, request: ApiRequest) -> FieldResult<ApiResponse> {
        let (tx, rx) = oneshot::channel();
        let id = Uuid::new_v4();

        self.write().await.active_requests.insert(id.clone(), tx);

        if let Err(e) = channel.send(ClusterMessage::ApiRequest { 
            request_id: id, 
            request 
        }).await {
            return Err(async_graphql::Error::new_with_source(e));
        }

        match rx.await {
            Ok(v) => {
                match v {
                    ApiResponse::Error(e) => Err(async_graphql::Error::new_with_source(e)),
                    _ => Ok(v),
                }
            },
            Err(e) => {
                Err(async_graphql::Error::new_with_source(e))
            }
        }
    }

    pub(super) async fn query_cluster(&self, request: ApiRequest) -> FieldResult<ApiResponse> {
        let cluster = self.read().await.api_cluster.clone();

        self.query(&cluster, request).await
    }

    pub(super) async fn query_realm(&self, realm_id: u32, request: ApiRequest) -> FieldResult<ApiResponse> {
        let cluster = self.read().await.api_cluster.clone();

        self.query(&cluster, request).await
    }
}