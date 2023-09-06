use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

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

    pub fn peer(&self) -> RakNetPeerHandle {
        self.peer.clone()
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}