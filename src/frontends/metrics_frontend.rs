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

use std::time::Duration;

use async_trait::async_trait;
use bevy::log::error;
use poem::{get, handler, http::StatusCode, listener::TcpListener, IntoResponse, Response, Route, Server};
use prometheus::{Encoder, TextEncoder};
use tokio_util::sync::CancellationToken;

use crate::{cluster::frontend::Frontend, util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind}, CONF};

pub struct MetricsFrontend;

impl MetricsFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl Frontend for MetricsFrontend {
    fn name(&self) -> &str { "metrics" }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let app = Route::new().at("/metrics", get(metrics));

        tokio::spawn(async move {
            Server::new(TcpListener::bind(CONF.metrics.listen_address))
                .run_with_graceful_shutdown(app, token.cancelled(), Some(Duration::from_secs(1)))
                .await
        }).await?.map_err(|e| AnotherlandError::new(AnotherlandErrorKind::Application, e))
    }
}

#[handler]
async fn metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => {
            Response::builder()
                .status(StatusCode::OK)
                .content_type(encoder.format_type())
                .body(buffer)
        },
        Err(e) => {
            error!("metric error: {:?}", e);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(())
        }
    }
}