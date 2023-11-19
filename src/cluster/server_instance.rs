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

use async_trait::async_trait;

use crate::util::AnotherlandResult;
use atlas::raknet::{RakNetRequest, RakNetListener};

use super::{ClusterMessage, MessageChannel};

#[async_trait]
pub trait ServerInstance {
    type ServerProperties: Send + Sync;

    async fn init(properties: &Self::ServerProperties) -> AnotherlandResult<Box<Self>>;
    async fn close(&mut self);
    fn raknet_listener(&self) -> Option<&RakNetListener> { None }
    async fn handle_request(&mut self, _request: RakNetRequest) -> AnotherlandResult<()> { Ok(()) }
    async fn handle_cluster_message(&mut self, _message: ClusterMessage) -> AnotherlandResult<()> { Ok(()) }
    async fn tick(&mut self) -> AnotherlandResult<()> { Ok(()) }
    fn get_subscribed_channels(&self) -> Vec<MessageChannel>;
    fn tickrate() -> Duration { Duration::from_secs(1) }
}