use super::{Guid, Error, PeerAddress, Packet, MessageNumber, Reliability, PacketSplit};
use std::{time::Instant, sync::Arc, net::{UdpSocket, SocketAddr}, collections::VecDeque};
use async_trait::async_trait;

struct ResendBuffer {
    nextAction: Instant,
}

pub enum Priority {
    System,
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn count_priorities() -> usize { 4 }
}

pub struct RakNetPeer {
    guid: Guid,
    address: PeerAddress,

    awaiting_ack_queue: VecDeque<Packet>,
    resend_queue: VecDeque<Packet>,
    send_queue_prioritized: Vec<VecDeque<Packet>>,
}

impl RakNetPeer {
    fn create_prioritized_send_queue() -> Vec<VecDeque<Packet>> {
        let mut vec = Vec::new();

        for i in 0..Priority::count_priorities() {
            vec.push(VecDeque::new());
        }

        vec
    }

    pub fn new<'a>(addr: SocketAddr) -> Result<Self, Error<'a>> {
        match addr {
            SocketAddr::V4(a) => {
                Ok(Self {
                    guid: Guid::create_random(),
                    address: PeerAddress::new(a.ip(), a.port()),

                    awaiting_ack_queue: VecDeque::new(),
                    resend_queue: VecDeque::new(),
                    send_queue_prioritized: Self::create_prioritized_send_queue(),
                })
            },
            _ => Err(Error::InvalidAddressFormat),
        }
    }

    pub async fn handle_raw_message(&mut self, number: MessageNumber, reliability: Reliability, split: PacketSplit, data: Vec<u8>) {
        
    }

    pub async fn run_update(&mut self) {

    }
}
