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

use std::time::Duration;

use async_graphql::{Schema, EmptySubscription, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use async_trait::async_trait;
use poem::{Server, Route, get, handler, IntoResponse, web::Html, listener::TcpListener};
use tokio_util::sync::CancellationToken;

use crate::{cluster::frontend::Frontend, util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind}, CONF, NODE, actors::Authenticator};

use super::schema::{QueryRoot, MutationRoot};

pub struct ApiFrontend;

impl ApiFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl Frontend for ApiFrontend {
    fn name(&self) -> &str { "api" }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {

        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(NODE.get_remote_actor::<Authenticator>("authenticator").unwrap())
            .finish();

        let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

        tokio::spawn(async move {
            Server::new(TcpListener::bind(CONF.api.listen_address))
                .run_with_graceful_shutdown(app, token.cancelled(), Some(Duration::from_secs(1)))
                .await
        }).await?.map_err(|e| AnotherlandError::new(AnotherlandErrorKind::ApplicationError, e))
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}