use std::{net::{SocketAddr}, sync::Arc, collections::{VecDeque, LinkedList}, time::{Instant, Duration}, iter::Filter, borrow::Borrow};

use chrono::{Utc};
use rsa::{RsaPublicKey, RsaPrivateKey};
use tokio::{net::UdpSocket, io};

use super::{Message, ReliableMessage, MessageNumber, AckRange, Reliability};

pub enum PeerState {
    Connecting,
    Connected,
}

pub struct RakNetPeer {
    sock: Arc<UdpSocket>,
    state: PeerState,
    address: SocketAddr,
    peer_time: Duration,

    opened: Instant,
    output_queue: VecDeque<ReliableMessage>,
    resend_queue: VecDeque<ReliableMessage>,
    next_message_number: MessageNumber,

    priv_key: RsaPrivateKey,
    pub_key: RsaPublicKey,
    is_encrypted: bool,
}

impl RakNetPeer {
    pub fn connecting(sock: Arc<UdpSocket>, address: SocketAddr) -> Self {
        let mut rng = rand::thread_rng();
        let bits = 512;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        Self {
            sock,
            state: PeerState::Connecting,
            address,
            peer_time: Duration::default(),

            opened: Instant::now(),
            output_queue: VecDeque::new(),
            resend_queue: VecDeque::new(),
            next_message_number: 1,

            priv_key,
            pub_key,
            is_encrypted: false,
        }
    }

    pub fn peer_addr(&self) -> SocketAddr { self.address }
    pub fn own_addr(&self) -> SocketAddr { self.sock.local_addr().unwrap() }

    pub fn priv_key(&self) -> &RsaPrivateKey { &self.priv_key }
    pub fn pub_key(&self) -> &RsaPublicKey { &self.pub_key }

    pub fn update_time(&mut self, time: Duration) { 
        println!("New peer time: {:#?}", time);
        self.peer_time = time 
    }

    pub async fn send_connectionless_message(&self, message: Message) -> io::Result<usize> {
        self.sock.send_to(message.to_bytes().as_slice(), self.address).await
    }

    pub async fn send_reliable_message(&mut self, message: Message) -> io::Result<()> {
        self.output_queue.push_back(ReliableMessage::MessageFrame { 
            id: self.next_message_number, 
            reliability: Reliability::Reliable, 
            split: super::PacketSplit::NotSplit, 
            message 
        });

        self.next_message_number += 1;
        self.update().await
    }

    pub async fn send_unreliable_message(&mut self, message: Message) -> io::Result<()> {
        self.output_queue.push_back(ReliableMessage::MessageFrame { 
            id: self.next_message_number, 
            reliability: Reliability::Unreliable, 
            split: super::PacketSplit::NotSplit, 
            message 
        });

        self.next_message_number += 1;
        self.update().await
    }

    pub async fn acknowledge(&mut self, id: MessageNumber) -> io::Result<()> {
        self.output_queue.push_back(ReliableMessage::Ack { 
            time: self.peer_time, 
            id_ranges: vec![AckRange { start: id, end: id }]
        });

        self.update().await
    }

    pub fn handle_acknowledgements(&mut self, time: &Duration, id_range: &Vec<AckRange>) {
        self.output_queue = self.output_queue.drain(..).filter(move |p| {
            match p {
                ReliableMessage::MessageFrame { id, reliability, split, message } => {
                    for a in id_range {
                        if a.contains(id) {
                            return false;
                        }
                    }

                    return true;
                },
                _ => return true,
            }
        }).collect();
    }

    pub async fn update(&mut self) -> io::Result<()> {
        for m in self.output_queue.iter_mut() {
            /*let acks = if self.acks.is_empty() {
                None
            } else {
                Some(Acks::new(self.peer_time, self.acks.drain(..).collect()))
            };*/

            match m {
                ReliableMessage::Ack { time, id_ranges } => {
                    self.sock.send_to(m.to_bytes(None).unwrap().as_slice(), self.address).await?;
                },
                ReliableMessage::MessageFrame { id, reliability, split, message } => {
                    self.sock.send_to(m.to_bytes(Some(Instant::now().duration_since(self.opened))).unwrap().as_slice(), self.address).await?;
                }
            }
        }

        // Filter out acks & unreliable messages
        self.output_queue = self.output_queue.drain(..).filter(move |p| {
            match p {
                ReliableMessage::Ack { time, id_ranges } => return false,
                ReliableMessage::MessageFrame { id, reliability, split, message } => match reliability {
                    Reliability::Unreliable => return false,
                    Reliability::UnreliableSequenced(_) => return false,
                    _ => return true,
                }
                _ => return true,
            }
        }).collect();

        Ok(())
    }
}