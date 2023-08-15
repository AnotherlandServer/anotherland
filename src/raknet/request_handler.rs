use async_trait::async_trait;

use super::{RakNetRequest, RakNetResponse, Error, peer::RakNetPeer};

#[async_trait]
pub trait RequestHandler: Send {
    async fn handle_request<'a>(&'a mut self, peer: &RakNetPeer, request: &'a RakNetRequest, response: &'a mut RakNetResponse) -> Result<(), Error>;
    async fn update_client<'a>(&'a mut self, peer: &RakNetPeer, update: &'a mut RakNetResponse) -> Result<(), crate::raknet::Error<'a>>;
}