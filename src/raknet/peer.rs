use crate::{atlas::Uuid, raknet::{MAX_MTU_SIZE, PacketSequence}};

use super::{RakNetErrorKind, PeerAddress, MessageFragment, MessageNumber, Reliability, PacketSplit, Message, OnlineMessage, AckRange, RakNetError, RakNetResult};
use std::{time::{Instant, SystemTime}, time::{Duration, UNIX_EPOCH}, net::{SocketAddr, SocketAddrV4, Ipv4Addr}, collections::{VecDeque, HashMap}, rc::Rc, sync::Arc, cell::RefCell};
use bitstream_io::{BitWriter, BigEndian, BitWrite};
use log::{debug, error, trace};
use tokio::{net::UdpSocket, io};
use tokio::sync::{RwLock};
use async_trait::async_trait;

struct ResendBuffer {
    nextAction: Instant,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    Unconnected,
    HalfOpen,
    Connected,
    HalfClosed,
    Disconnected,
    Stale,
}

pub enum Priority {
    System,
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn count_priorities() -> usize { 4 }
    pub fn get_index(&self) -> usize {
        match self {
            Self::System => 0,
            Self::High => 1,
            Self::Medium => 2,
            Self::Low => 3,
        }
    }
}

#[derive(Debug)]
struct SendQueueItem {
    reliability: Reliability,
    split: PacketSplit,
    data: Vec<u8>,
}

struct OrderChannel {

}

pub struct RakNetPeer {
    guid: Uuid,
    remote_address: PeerAddress,
    local_address: PeerAddress,
    socket: Arc<UdpSocket>,
    state: State,

    pending_ack_queue: Vec<MessageNumber>,
    resend_queue: HashMap<MessageNumber, (Instant, Box<MessageFragment>)>,
    send_queue_prioritized: Arc<RwLock<Vec<VecDeque<SendQueueItem>>>>,
    remote_time: Duration,
    created: Instant,

    next_send_message_id: u32,
    next_recv_message_id: u32,
    split_packet_id: u16,
}

pub type RakNetPeerHandle = Arc<RwLock<RakNetPeer>>;

impl RakNetPeer {
    fn create_prioritized_send_queue() -> Vec<VecDeque<SendQueueItem>> {
        let mut vec = Vec::new();

        for i in 0..Priority::count_priorities() {
            vec.push(VecDeque::new());
        }

        vec
    }

    pub fn new(socket: Arc<UdpSocket>, remote_addr: SocketAddr, local_addr: SocketAddr) -> RakNetResult<Self> {
        match remote_addr {
            SocketAddr::V4(a) => {
                Ok(Self {
                    guid: Uuid::new_v4(),
                    remote_address: PeerAddress::new(a.ip(), a.port()),
                    local_address: match local_addr {
                        SocketAddr::V4(a) => PeerAddress::new(a.ip(), a.port()),
                        _ => panic!("Unsupported address type!"),
                    },
                    socket,
                    state: State::Unconnected,

                    pending_ack_queue: Vec::new(),
                    resend_queue: HashMap::new(),
                    send_queue_prioritized: Arc::new(RwLock::new(Self::create_prioritized_send_queue())),
                    remote_time: Duration::default(),
                    created: Instant::now(),

                    next_send_message_id: 0,
                    next_recv_message_id: 0,
                    split_packet_id: 0,
                })
            },
            _ => Err(RakNetError::from_kind(RakNetErrorKind::InvalidAddressFormat)),
        }
    }

    pub async fn send(&mut self, priority: Priority, reliability: Reliability, message: Message) -> RakNetResult<()> {
        if self.state == State::HalfClosed || self.state == State::Disconnected {
            return Err(RakNetError::new(RakNetErrorKind::IOError, io::Error::from(io::ErrorKind::BrokenPipe)));
        }
        
        self.send_internal(priority, reliability, message).await
    }

    async fn send_internal(&mut self, priority: Priority, reliability: Reliability, message: Message) -> RakNetResult<()> {
        let message_data = message.to_bytes();
        
        match reliability {
            Reliability::Unreliable => {
                self.send_queue_prioritized.write().await[priority.get_index()].push_back(SendQueueItem { 
                    reliability, 
                    split: PacketSplit::NotSplit,
                    data: message_data
                });

                Ok(())
            },
            Reliability::Reliable => {
                // do we have to split the message?
                if message_data.len() > MAX_MTU_SIZE {
                    let chunks = message_data.chunks(MAX_MTU_SIZE).collect::<Vec<&[u8]>>();
                    let chunk_count = chunks.len() as u32;

                    for (idx, chunk) in chunks.iter().enumerate() {
                        self.send_queue_prioritized.write().await[priority.get_index()].push_back(SendQueueItem { 
                            reliability,
                            split: PacketSplit::Split { 
                                id: self.split_packet_id, 
                                index: idx as u32, 
                                count: chunk_count,
                            },
                            data: chunk.to_vec()
                        });

                        self.split_packet_id = self.split_packet_id.wrapping_add(1);
                    }

                    Ok(())
                } else {
                    self.send_queue_prioritized.write().await[priority.get_index()].push_back(SendQueueItem { 
                        reliability, 
                        split: PacketSplit::NotSplit,
                        data: message_data
                    });

                    Ok(())
                }
            },
            _ => todo!()
        }
    }

    pub async fn digest_message_fragments(&mut self, fragments: Vec<MessageFragment>) -> RakNetResult<Vec<Message>> {
        let mut messages = Vec::new();
        
        for fragment in fragments {
            match fragment {
                MessageFragment::Ack(time, ranges) => {
                    for range in ranges {
                        let mut pending_ids = self.resend_queue.keys().map(|v| v.to_owned()).collect::<Vec<_>>();
                        pending_ids.sort();

                        for id in pending_ids {
                            if id >= range.0 && id <= range.1 {
                                trace!("Peer acknowledged message {}", id);

                                // Remove message from resend queue after ack
                                self.resend_queue.remove(&id);
                            }
                        }
                    }
                },
                MessageFragment::SystemTime(remote_time) => {
                    self.remote_time = remote_time;
                },
                MessageFragment::OfflineMessage(Message::OpenConnectionRequest { version }) => {
                    if version != 3 {
                        debug!("Got unexpected raknet version from peer {:#?}. Got {} expected 3!", self.remote_address(), version);
                        self.disconnect().await;
                    } else {
                        self.send_internal(Priority::System, Reliability::Unreliable, Message::OpenConnectionReply).await?;
                    }
                },
                MessageFragment::OnlineMessage(message) => {
                    if let Some(message) = self.digest_online_message(&message).await? {
                        match message {
                            Message::InternalPing { time } => {
                                self.remote_time = time;
                                self.send_internal(Priority::System, Reliability::Unreliable, Message::ConnectedPong { 
                                    remote_time: self.remote_time, 
                                    local_time: Instant::now().duration_since(self.created), 
                                }).await?;
                            },
                            Message::NewIncomingConnection { primary_address, secondary_addresses } => {
                                self.send_internal(Priority::System, Reliability::Unreliable, Message::InternalPing { 
                                    time: Instant::now().duration_since(self.created), 
                                }).await?;
                            },
                            Message::ConnectionRequest { password } => {
                                debug!("Got connection request from {:#?}", self.remote_address());

                                self.state = State::Connected;
                                self.send_internal(Priority::System, Reliability::Reliable, Message::ConnectionRequestAccepted { 
                                    index: 0, 
                                    peer_addr: self.remote_address, 
                                    own_addr: self.local_address, 
                                    guid: self.guid.clone() 
                                }).await?;
                            },
                            Message::ConnectedPong { remote_time, local_time } => {
                            },
                            Message::DisconnectionNotification => {
                                self.state = State::HalfClosed;
                                self.resend_queue.clear();
                                self.send_internal(Priority::System, Reliability::Reliable, Message::DisconnectionNotification).await?;
                            }
                            _ => {
                                if self.state == State::Connected {
                                    messages.push(message);
                                } else {
                                    debug!("Dropping message from {:#?}. Peer is not connected", self.remote_address());
                                    self.disconnect_immediate();
                                }
                            },
                        }
                    }
                },
                _ => unreachable!(),
            }
        }

        Ok(messages)
    }

    async fn digest_online_message(&mut self, message: &OnlineMessage) -> RakNetResult<Option<Message>> {
        match message.reliability {
            Reliability::Unreliable => Ok(Some(
                    Message::from_bytes(&message.data).map_err(|e| 
                        io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?.1
                )),
            Reliability::UnreliableSequenced(_) => Ok(Some(
                Message::from_bytes(&message.data).map_err(|e| 
                    io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?.1
                )),
            Reliability::Reliable => {
                self.pending_ack_queue.push(message.number);

                Ok(Some(
                    Message::from_bytes(&message.data).map_err(|e| 
                        io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?.1
                ))
            },
            Reliability::ReliableOrdered(_) => {
                self.pending_ack_queue.push(message.number);

                Ok(Some(
                    Message::from_bytes(&message.data).map_err(|e| 
                        io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?.1
                ))
            },
            Reliability::ReliableSequenced(_) => {
                self.pending_ack_queue.push(message.number);

                Ok(Some(
                    Message::from_bytes(&message.data).map_err(|e| 
                        io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?.1
                ))
            },
            _ => todo!(),
        }
    }

    pub fn generate_next_message_id(&mut self) -> u32 {
        let msg = self.next_send_message_id;
        self.next_send_message_id = self.next_send_message_id.wrapping_add(1);

        msg
    }

    pub fn remote_address(&self) -> &PeerAddress {
        &self.remote_address
    }

    pub fn local_address(&self) -> &PeerAddress {
        &self.local_address
    }

    pub fn guid(&self) -> &Uuid {
        &self.guid
    }

    pub fn remote_time(&self) -> Duration {
        self.remote_time
    }

    pub fn state(&self) -> State { 
        self.state
    }

    pub async fn disconnect(&mut self) {
        match self.state {
            State::Unconnected => self.state = State::Disconnected,
            _ => {
                self.state = State::HalfClosed;
                self.send_internal(Priority::System, Reliability::Unreliable, Message::DisconnectionNotification).await;
            }
        }
    }

    pub fn disconnect_immediate(&mut self) {
        self.state = State::Disconnected
    }

    pub fn serialize_acks_to_bitwriter<E, W>(&mut self, writer: &mut BitWriter<E, W>) -> RakNetResult<()> 
    where
    E: std::io::Write,
    W: bitstream_io::Endianness
    {
        self.pending_ack_queue.sort();

        if self.pending_ack_queue.is_empty() {
            let _ = writer.write_bit(false);
        } else {
            let mut ack_ranges = Vec::<(u32, u32)>::new();
            let _ = writer.write_bit(true);
            
            let mut id_min = *self.pending_ack_queue.first().unwrap();
            let mut id_max = id_min;
            for &id in &self.pending_ack_queue[1..] {
                if id - id_max > 1 {
                    ack_ranges.push((id_min, id_max));
                    id_min = id;
                    id_max = id;
                } else {
                    id_max = id;
                }
            }
            ack_ranges.push((id_min, id_max));

            trace!("Acknowledge message range {}-{}", id_min, id_max);

            let _ = MessageFragment::Ack(self.remote_time, ack_ranges).serialize_to_bitwriter(writer);
        }

        self.pending_ack_queue.clear();

        Ok(())
    }
    
    pub async fn run_update(&mut self) -> RakNetResult<()> {
        let mut time_sent = false;
        let mut acks_sent = false;

        let mut buf = Vec::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        // resend pending messages not yet acknowledged
        let mut pending_ids = self.resend_queue.keys().map(|v| v.to_owned()).collect::<Vec<_>>();
        pending_ids.sort();

        for id in pending_ids {
            let (sent) = self.resend_queue.get(&id).map(|(sent, _)| sent.to_owned()).unwrap();
            if Instant::now().duration_since(sent).as_millis() > 1000 {
                let (id, (_, message)) = self.resend_queue.remove_entry(&id).unwrap();

                trace!("Resending message id {}:{}", self.guid.to_string(), id);

                self.serialize_acks_to_bitwriter(&mut writer)?;
                acks_sent = true;

                message.serialize_to_bitwriter(&mut writer)?;
                self.resend_queue.insert(id, (Instant::now(), message));
            }
        }

        // dequeue pending messages,  from high priority to low
        for queue in self.send_queue_prioritized.clone().write().await.iter_mut() {              
            while let Some(message) = queue.pop_front() {
                // Offline? Then just send the raw message
                if self.state == State::Unconnected {
                    trace!("Sending offline message to client {:#?}: {:#?}", self.guid.to_string(), message);

                    let data = message.data;
                    self.send_raw(&data).await?;
                    continue;
                }

                self.serialize_acks_to_bitwriter(&mut writer)?;
                acks_sent = true;

                if !time_sent {
                    let _ = writer.write_bit(true);
                    let _ = MessageFragment::SystemTime(SystemTime::now().duration_since(UNIX_EPOCH).unwrap()).serialize_to_bitwriter(&mut writer);

                    time_sent = true;
                } else {
                    let _ = writer.write_bit(false);
                }

                let online_message = OnlineMessage { 
                    number: self.generate_next_message_id(), 
                    reliability: message.reliability, 
                    split: message.split, 
                    data: message.data
                };

                trace!("Sending online message to client {:#?}: {:#?}", self.guid.to_string(), online_message);

                match message.reliability {
                    Reliability::Reliable | Reliability::ReliableOrdered(_) => {
                        self.resend_queue.insert(online_message.number, (Instant::now(), Box::new(MessageFragment::OnlineMessage(online_message.clone()))));
                    },
                    _ => (),
                }

                let _ = MessageFragment::OnlineMessage(online_message).serialize_to_bitwriter(&mut writer)?;
                
                let _ = writer.byte_align();
                let _ = writer.flush();

                self.send_raw(writer.into_writer().as_slice()).await?;

                buf.clear();
                writer = BitWriter::endian(&mut buf, BigEndian);
            }
        }


        // Write ack only message if no acks where sent yet
        if !acks_sent && !self.pending_ack_queue.is_empty() {
            self.serialize_acks_to_bitwriter(&mut writer)?;

            let _ = writer.byte_align();
            let _ = writer.flush();

            let remainder = writer.into_writer();
            self.send_raw(&remainder.as_slice()).await?;
        }

        if self.state == State::HalfClosed {
            if self.resend_queue.is_empty() {
                self.state = State::Disconnected;
            }
        }

        Ok(())
    }

    async fn send_raw(&self, data: &[u8]) -> RakNetResult<()> {
        self.socket.send_to(data, self.remote_address.as_socket_addr()).await?;
        Ok(())
    }
}
