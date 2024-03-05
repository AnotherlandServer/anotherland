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

use crate::{raknet::MAX_MTU_SIZE, Uuid};

use super::{checksum::Checksum, Message, MessageFragment, MessageNumber, OnlineMessage, PacketSplit, PeerAddress, RakNetError, RakNetErrorKind, RakNetResult, Reliability};
use std::{collections::{HashMap, VecDeque}, mem::swap, net::SocketAddr, sync::Arc, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit}, Aes128};
use bitstream_io::{BigEndian, BitWrite, BitWriter};
use log::{debug, trace, warn};
use rand::{thread_rng, Rng};
use rsa::{rand_core::{OsRng, RngCore}, traits::PublicKeyParts, BigUint, RsaPrivateKey};
use sha1::{Sha1, Digest};
use tokio::{net::UdpSocket, io, sync::RwLock};
use async_recursion::async_recursion;
use rsa::hazmat::rsa_decrypt_and_check;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum State {
    Unconnected,
    EcryptionHandshake,
    Connected,
    HalfClosed,
    Disconnected,
}

#[allow(unused)]
#[derive(Debug)]
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

#[allow(unused)]
pub(crate) struct RakNetPeerData {
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

    split_packet_channel: HashMap<u16, Vec<Option<OnlineMessage>>>,

    next_send_message_id: u32,
    next_recv_message_id: u32,
    split_packet_id: u16,

    server_key: Option<Arc<RsaPrivateKey>>,
    encryption_key: Option<[u8; 16]>,
    new_random_number: [u8; 20],
    old_random_number: [u8; 20],
    random_number_expiration_time: Instant,
}

#[allow(unused)]
impl RakNetPeerData {
    fn create_prioritized_send_queue() -> Vec<VecDeque<SendQueueItem>> {
        let mut vec = Vec::new();

        for _ in 0..Priority::count_priorities() {
            vec.push(VecDeque::new());
        }

        vec
    }

    pub(crate) fn new(socket: Arc<UdpSocket>, remote_addr: SocketAddr, local_addr: SocketAddr, server_key: Option<Arc<RsaPrivateKey>>) -> RakNetResult<Self> {
        match remote_addr {
            SocketAddr::V4(a) => {
                let mut peer = Self {
                    guid: Uuid::new(),
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

                    split_packet_channel: HashMap::new(),

                    next_send_message_id: 0,
                    next_recv_message_id: 0,
                    split_packet_id: 0,

                    server_key,
                    encryption_key: None,

                    new_random_number: [0; 20],
                    old_random_number: [0; 20],
                    random_number_expiration_time: Instant::now(),
                };

                peer.generate_syn_cookie_random_number();

                Ok(peer)
            },
            _ => Err(RakNetError::from_kind(RakNetErrorKind::InvalidAddressFormat)),
        }
    }

    fn generate_syn_cookie_random_number(&mut self) {
        swap(&mut self.new_random_number, &mut self.old_random_number);
        OsRng.fill_bytes(&mut self.new_random_number);

        self.random_number_expiration_time = Instant::now() + Duration::from_millis(5000);
    }

    pub(crate) async fn send(&mut self, priority: Priority, reliability: Reliability, message: Message) -> RakNetResult<()> {
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
                let max_mtu_size = if self.encryption_key.is_some() && self.state == State::Connected {
                    // account for encryption overhead
                    MAX_MTU_SIZE - 16
                } else {
                    MAX_MTU_SIZE
                };

                // do we have to split the message?
                if message_data.len() > max_mtu_size {
                    let chunks = message_data.chunks(max_mtu_size).collect::<Vec<&[u8]>>();
                    let chunk_count = chunks.len() as u32;

                    self.split_packet_id = self.split_packet_id.wrapping_add(1);

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

    pub fn activate_encryption(&mut self) {
        if self.state == State::EcryptionHandshake && self.encryption_key.is_some() {
            self.state = State::Connected;
        }
    }

    #[async_recursion]
    pub(crate) async fn digest_message_fragments(&mut self, fragments: Vec<MessageFragment>) -> RakNetResult<Vec<Message>> {
        let mut messages = Vec::new();
        
        for fragment in fragments {
            match fragment {
                MessageFragment::Ack(_, ranges) => {
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
                    match message.split {
                        PacketSplit::Split { id, index, count } => {
                            let reliability = message.reliability;

                            if let Some(split_messages) = self.split_packet_channel.get_mut(&id) {
                                // Insert new message
                                split_messages[index as usize] = Some(message);
                            } else {
                                let mut split_messages = Vec::new();
                                split_messages.resize(count as usize, None);
                                split_messages[index as usize] = Some(message);

                                self.split_packet_channel.insert(id, split_messages);
                            }

                            // Check if were complete
                            if self.split_packet_channel.get(&id).unwrap().iter().all(|m| m.is_some()) {
                                let mut combined_message = OnlineMessage {
                                    number: 0,
                                    split: PacketSplit::NotSplit,
                                    reliability,
                                    data: Vec::new(),
                                };

                                let mut split_messages = self.split_packet_channel.remove(&id).unwrap();
                                for m in split_messages {
                                    combined_message.data.append(&mut m.unwrap().data);
                                }

                                messages.append(&mut self.digest_message_fragments(vec![MessageFragment::OnlineMessage(combined_message)]).await?);
                            }
                        },
                        PacketSplit::NotSplit => {
                            if let Some(message) = self.digest_online_message(&message).await? {
                                match message {
                                    Message::InternalPing { time } => {
                                        self.remote_time = time;
                                        self.send_internal(Priority::System, Reliability::Unreliable, Message::ConnectedPong { 
                                            remote_time: self.remote_time, 
                                            local_time: Instant::now().duration_since(self.created), 
                                        }).await?;
                                    },
                                    Message::NewIncomingConnection { .. } => {
                                        self.send_internal(Priority::System, Reliability::Unreliable, Message::InternalPing { 
                                            time: Instant::now().duration_since(self.created), 
                                        }).await?;
                                    },
                                    Message::ConnectionRequest { .. } => {
                                        debug!("Got connection request from {:?}", self.remote_address());

                                        if let Some(server_key) = self.server_key.as_ref() {
                                            // generate syn-cookie
                                            let mut hasher = Sha1::new();
                                            hasher.update(self.remote_address.ip.octets());
                                            hasher.update(self.remote_address.port.to_be_bytes());
                                            hasher.update(self.new_random_number);
                                            let hash = hasher.finalize();
                                            
                                            let public_key = server_key.to_public_key();

                                            self.state = State::EcryptionHandshake;

                                            // initiate key exchange
                                            self.send_internal(Priority::System, Reliability::Unreliable, Message::SecuredConnectionResponse { 
                                                syn_cookie: hash[..].try_into().unwrap(), 
                                                e: public_key.e().get_limb(0) as u32,
                                                modulus: public_key.n().to_bytes_le().try_into().unwrap(), 
                                            }).await?;
                                        } else {
                                            self.state = State::Connected;

                                            self.send_internal(Priority::System, Reliability::Reliable, Message::ConnectionRequestAccepted { 
                                                index: 0, 
                                                peer_addr: self.remote_address, 
                                                own_addr: self.local_address, 
                                                guid: self.guid 
                                            }).await?;
                                        }
                                    },
                                    Message::ConnectedPong { .. } => {
                                    },
                                    Message::DisconnectionNotification => {
                                        self.state = State::HalfClosed;
                                        self.resend_queue.clear();
                                        self.send_internal(Priority::System, Reliability::Reliable, Message::DisconnectionNotification).await?;
                                    },
                                    Message::SecuredConnectionConfirmation { syn_cookie, mut encrypted_rsa_key } => {
                                        if let Some(server_key) = self.server_key.as_ref() {
                                            let mut confirmed_hash = false;
                                            let mut new_rand_number = false;

                                            // generate syn-cookie
                                            let mut hasher = Sha1::new();
                                            hasher.update(self.remote_address.ip.octets());
                                            hasher.update(self.remote_address.port.to_be_bytes());
                                            hasher.update(self.new_random_number);
                                            let hash = hasher.finalize();

                                            if hash[..20] == syn_cookie {
                                                confirmed_hash = true;
                                                new_rand_number = true;
                                            } else if self.random_number_expiration_time <  Instant::now() {
                                                let mut hasher = Sha1::new();
                                                hasher.update(self.remote_address.ip.octets());
                                                hasher.update(self.remote_address.port.to_be_bytes());
                                                hasher.update(self.old_random_number);
                                                let hash = hasher.finalize();

                                                if hash[..20] == syn_cookie {
                                                    confirmed_hash = true;
                                                }
                                            }

                                            if confirmed_hash {
                                                // decrypt and save key
                                                match rsa_decrypt_and_check(
                                                    server_key.as_ref(), 
                                                    Option::<&mut OsRng>::None, 
                                                    &BigUint::from_bytes_le(&encrypted_rsa_key)
                                                ) {
                                                    Ok(random_number) => {
                                                        let random_number = random_number.to_bytes_le();
                                                        let mut aes_key = [0; 16];

                                                        // compute aes key
                                                        for i in 0usize..16 {
                                                            aes_key[i] = syn_cookie[i] ^ random_number[i];
                                                        }

                                                        self.encryption_key = Some(aes_key);

                                                        // accept connection
                                                        self.send_internal(Priority::System, Reliability::Reliable, Message::ConnectionRequestAccepted { 
                                                            index: 0, 
                                                            peer_addr: self.remote_address, 
                                                            own_addr: self.local_address, 
                                                            guid: self.guid 
                                                        }).await?;
                                                    },
                                                    Err(e) => {
                                                        warn!("Failed to decrypt client key: {:?}", e);
                                                    }
                                                }

                                                if new_rand_number {
                                                    self.generate_syn_cookie_random_number();
                                                }
                                            }
                                        }
                                    },
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
            }
        }
    }

    fn generate_next_message_id(&mut self) -> u32 {
        let msg = self.next_send_message_id;
        self.next_send_message_id = self.next_send_message_id.wrapping_add(1);

        msg
    }

    pub(crate) fn remote_address(&self) -> &PeerAddress {
        &self.remote_address
    }

    pub(crate) fn local_address(&self) -> &PeerAddress {
        &self.local_address
    }

    pub(crate) fn guid(&self) -> &Uuid {
        &self.guid
    }

    pub(crate) fn remote_time(&self) -> &Duration {
        &self.remote_time
    }

    pub(crate) fn state(&self) -> &State { 
        &self.state
    }

    pub(crate) async fn disconnect(&mut self) {
        match self.state {
            State::Unconnected => self.state = State::Disconnected,
            _ => {
                self.state = State::HalfClosed;
                self.send_internal(Priority::System, Reliability::Unreliable, Message::DisconnectionNotification).await;
            }
        }
    }

    pub(crate) fn disconnect_immediate(&mut self) {
        self.state = State::Disconnected
    }

    fn serialize_acks_to_bitwriter<E, W>(&mut self, writer: &mut BitWriter<E, W>) -> RakNetResult<()> 
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
    
    pub(crate) async fn run_update(&mut self) -> RakNetResult<()> {
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

                MessageFragment::OnlineMessage(online_message).serialize_to_bitwriter(&mut writer)?;
                
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
            self.send_raw(remainder.as_slice()).await?;
        }

        if self.state == State::HalfClosed && self.resend_queue.is_empty() {
            self.state = State::Disconnected;
        }

        Ok(())
    }

    async fn send_raw(&self, data: &[u8]) -> RakNetResult<()> {
        if let Some(key) = self.encryption_key.as_ref() && self.state >= State::Connected {
            self.socket.send_to(&Self::encrypt_message(key, data), self.remote_address.as_socket_addr()).await?;
            Ok(())
        } else {
            self.socket.send_to(data, self.remote_address.as_socket_addr()).await?;
            Ok(())
        }
    }

    pub(crate) fn encrypt_message(key: &[u8; 16], data: &[u8]) -> Vec<u8> {
        let padding_bytes = 16 - (((data.len() + 6 - 1) % 16) + 1);
        let mut checksum = Checksum::new();
        let mut message_buffer = vec![0; data.len() + 6 + padding_bytes];

        if message_buffer.len() % 16 != 0 { panic!(); }

        // generate random based message part
        {
            let mut rng = thread_rng();

            // pad size
            let mut encoded_pad = rng.gen::<u8>();
            encoded_pad <<= 4;
            encoded_pad |= padding_bytes as u8;

            // write random char
            message_buffer[4] = rng.gen::<u8>();

            // write padding size
            message_buffer[5] = encoded_pad;

            // write padding
            let buffer_len = message_buffer.len();
            rng.fill_bytes(&mut message_buffer[6..6 + padding_bytes]);
        }

        // copy data
        message_buffer[6 + padding_bytes..].copy_from_slice(data);

        // generate checksum
        checksum.write(&message_buffer[4..]);
        message_buffer[..4].copy_from_slice(&checksum.finish().to_le_bytes());

        // initialize encryption
        let mut blocks: Vec<&mut [u8]> = message_buffer.chunks_mut(16).collect();
        let cipher = Aes128::new(GenericArray::from_slice(key));
        let mut prev_block = 0;

        // encrypt first block
        cipher.encrypt_block(GenericArray::from_mut_slice(blocks[0]));

        // encrypt remaining blocks, starting from the end
        for index in (1..blocks.len()).rev() {
            for byte_index in 0..16 {
                blocks[index][byte_index] ^= blocks[prev_block][byte_index];
            }

            cipher.encrypt_block(GenericArray::from_mut_slice(blocks[index]));
            prev_block = index;
        }

        message_buffer
    }

    pub(crate) fn optional_message_decrypt(&self, message: &mut Vec<u8>) -> RakNetResult<()> {
        if let Some(key) = self.encryption_key && (self.state >= State::Connected) {
            Self::decrypt_message(&key, message)
        } else {
            Ok(())
        }
    }

    pub(crate) fn decrypt_message(key: &[u8; 16], message: &mut Vec<u8>) -> RakNetResult<()> {
        if message.len() % 16 != 0 || message.is_empty() {
            return Err(RakNetError::new(RakNetErrorKind::IOError, "invalid message len"));
        }

        // initialize decryption
        let mut blocks: Vec<&mut [u8]> = message.chunks_mut(16).collect();
        let cipher = Aes128::new(GenericArray::from_slice(key));

        // decrypt blocks following the first one
        for index in 1..blocks.len() {
            cipher.decrypt_block(GenericArray::from_mut_slice(blocks[index]));

            for byte_index in 0..16 {
                if index == blocks.len() - 1 {
                    blocks[index][byte_index] ^= blocks[0][byte_index];
                } else {
                    blocks[index][byte_index] ^= blocks[index + 1][byte_index];
                }
            }
        }

        // decrypt first block
        cipher.decrypt_block(GenericArray::from_mut_slice(blocks[0]));

        // read size of padding
        let paddingbytes = (message[5] & 0x0F) as usize;

        // compute original message length
        let message_len = message.len() - 6 - paddingbytes;

        // validate checksum
        let mut checksum = Checksum::new();
        checksum.write(&message[4..]);

        if u32::from_le_bytes(message[..4].to_owned().try_into().unwrap()) != checksum.finish() {
            debug!("Expected: {}", u32::from_le_bytes(message[..4].to_owned().try_into().unwrap()));
            debug!("Computed: {}", checksum.finish());
            return Err(RakNetError::new(RakNetErrorKind::IOError, "checksum error"));
        }

        // move decrypted message to the front of the buffer
        message.copy_within(6 + paddingbytes..6 + paddingbytes + message_len, 0);

        // truncate message buffer
        message.resize(message_len, 0);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    use super::RakNetPeerData;

    #[test]
    pub fn test_aes() {
        let mut key: [u8; 16] = [0; 16];
        let mut rng = thread_rng();
        rng.fill(&mut key);

        let message = b"Test message";
        let mut encrypted = RakNetPeerData::encrypt_message(&key, message);
        RakNetPeerData::decrypt_message(&key, &mut encrypted).expect("decryption failed");

        assert_eq!(encrypted, message);
    }

    #[test]
    pub fn test_aes1() {
        let mut key: [u8; 16] = [0; 16];
        let mut rng = thread_rng();
        rng.fill(&mut key);

        let message = b"Test";
        let mut encrypted = RakNetPeerData::encrypt_message(&key, message);
        RakNetPeerData::decrypt_message(&key, &mut encrypted).expect("decryption failed");

        assert_eq!(encrypted, message);
    }

    #[test]
    pub fn test_aes_2() {
        let mut key: [u8; 16] = [0; 16];
        let mut rng = thread_rng();
        rng.fill(&mut key);

        let message = b"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let mut encrypted = RakNetPeerData::encrypt_message(&key, message);
        RakNetPeerData::decrypt_message(&key, &mut encrypted).expect("decryption failed");

        assert_eq!(encrypted, message);
    }
}