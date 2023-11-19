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
    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()> { Ok(()) }
    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> { Ok(()) }
    async fn tick(&mut self) -> AnotherlandResult<()> { Ok(()) }
    fn get_subscribed_channels(&self) -> Vec<MessageChannel>;
    fn tickrate() -> Duration { Duration::from_secs(1) }
}