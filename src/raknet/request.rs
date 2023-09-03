use tokio::sync::RwLockReadGuard;

use super::{Message, RakNetPeerHandle, RakNetPeer};

pub struct RakNetRequest {
    peer: RakNetPeerHandle,
    message: Message,
}

impl RakNetRequest {
    pub fn new(peer: RakNetPeerHandle, message: Message) -> Self {
        Self {
            peer,
            message
        }
    }

    pub fn peer(&self) -> RwLockReadGuard<'_, RakNetPeer> {
        self.peer.blocking_read()
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}