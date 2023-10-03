use async_trait::async_trait;

use crate::util::AnotherlandResult;
use atlas::raknet::RakNetRequest;

use super::ClusterMessage;

#[async_trait]
pub trait ServerInstance {
    async fn init() -> AnotherlandResult<Box<Self>>;
    async fn close(&mut self);
    async fn next_request(&mut self) -> AnotherlandResult<Option<RakNetRequest>>;
    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()>;
    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()>;
    async fn tick(&mut self) -> AnotherlandResult<()>;
}