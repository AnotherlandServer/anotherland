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

use std::{net::SocketAddr, sync::{atomic::{AtomicBool, AtomicI64, AtomicU128, AtomicU64, AtomicU8, Ordering}, Arc}, time::{Duration, SystemTime}};

use log::{debug, error, warn};
use rand::Rng;
use rsa::{hazmat::rsa_decrypt_and_check, rand_core::{OsRng, RngCore}, traits::PublicKeyParts, BigUint, RsaPrivateKey};
use sha1::{Sha1, Digest};
use tokio::{net::UdpSocket, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Notify, RwLock, Semaphore}, time::{sleep, Sleep}};
use uuid::Uuid;

use crate::{buffer::{RakNetReader, RakNetWriter}, encryption::{aes_decrypt, EncryptionHanshakeContext}, error::Result, frame::MessageFrame, packet::{read_connection_request, write_connection_request_accepted, write_secured_connection_response}, reliability::{self, RecvQ, Reliability, SendQ}, util::cur_timestamp, PacketID, RakNetError};

#[derive(Debug, Clone, Copy)]
enum ConnectMode {
    NoAction,
    DisconnectAsap,
    DisconnectAsapSilently,
    DisconnectOnNoAck,
    RequestedConntection,
    HandlingConnectionRequest,
    UnverifiedSender,
    SetEncryptionOnMultiple16BytePacket(u128),
    Connected,
}

const RECEIVE_TIMEOUT: u64 = 60000;

pub struct RakNetSocket {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    close_notifier: Arc<Semaphore>,
    reference_time: Arc<SystemTime>,
    last_heartbeat_time: Arc<AtomicU64>,
    //sender: Sender<(Vec<u8>, SocketAddr, bool, u8)>,
    drop_notifier: Arc<Notify>,
    incomming_notifier: Arc<Notify>,
    recvq: Arc<Mutex<RecvQ>>,
    sendq: Arc<RwLock<SendQ>>,
    encryption: Arc<AtomicBool>,
    aes_key: Arc<AtomicU128>,
    user_data_receiver: Mutex<Receiver<Vec<u8>>>,
}

impl RakNetSocket {
    pub(crate) async fn open(
        addr: &SocketAddr,
        s: &Arc<UdpSocket>,
        receiver: Receiver<Vec<u8>>,
        mtu: u16,
        reaper: &Sender<SocketAddr>,
        rsa_key: Option<RsaPrivateKey>,
    ) -> Result<Self> {
        let (user_data_sender, user_data_receiver) = channel::<Vec<u8>>(100);
        //let (sender_sender, sender_reciever) = channel::<(Vec<u8>, SocketAddr, bool, u8)>(10);

        let send_notify = Arc::new(Notify::new());

        let mut random_number = [0u8; 20];
        OsRng.fill_bytes(&mut random_number);

        let mut hasher = Sha1::new();
        hasher.update(addr.ip().to_string());
        hasher.update(addr.port().to_le_bytes());
        hasher.update(random_number);

        let socket = Self {
            local_addr: s.local_addr().unwrap(),
            peer_addr: *addr,
            close_notifier: Arc::new(Semaphore::new(0)),

            // We are never comparing times between connections,
            // se we get await with a reference_time per connection,
            // to get the most out of the 32-bit timestamps
            // raknet uses.
            reference_time: Arc::new(SystemTime::now()), 

            last_heartbeat_time: Arc::new(AtomicU64::new(0)),
            //sender: sender_sender,
            drop_notifier: Arc::new(Notify::new()),
            incomming_notifier: Arc::new(Notify::new()),
            recvq: Arc::new(Mutex::new(RecvQ::new())),
            sendq: Arc::new(RwLock::new(SendQ::new(mtu, send_notify.clone()))),

            encryption: Arc::new(AtomicBool::new(false)),
            aes_key: Arc::new(AtomicU128::new(0)),
            user_data_receiver: Mutex::new(user_data_receiver),
        };

        socket.start_receiver(receiver, user_data_sender, rsa_key);
        socket.start_tick(s, reaper.clone(), send_notify.clone());
        //socket.start_sender(s, sender_reciever);
        socket.drop_watcher().await;

        // wait for incomming notify or close
        let incomming_notifier = socket.incomming_notifier.clone();
        let close_notifier = socket.close_notifier.clone();
    
        tokio::select! {
            _ = incomming_notifier.notified() => {
                Ok(socket)
            },
            _ = close_notifier.acquire() => {
                Err(RakNetError::HandshakeFailed)
            }
        }
    }

    pub async fn close(&self) {
        if !self.close_notifier.is_closed() {
            self.close_notifier.close();
        }
    }

    pub async fn send(&self, buf: &[u8], r: Reliability) -> Result<()> {
        if buf.is_empty() { return Ok(()); }

        if self.close_notifier.is_closed() {
            return Err(RakNetError::ConnectionClosed);
        }

        self.sendq.write().await
            .insert(r, buf.to_vec())
    }

    pub async fn recv(&self) -> Result<Vec<u8>> {
        match self.user_data_receiver.lock().await.recv().await {
            Some(p) => Ok(p),
            None => {
                if self.close_notifier.is_closed() {
                    Err(RakNetError::ConnectionClosed)
                } else {
                    Err(RakNetError::SocketError)
                }
            }
        }
    }

    fn start_receiver(
        &self,
        mut receiver: Receiver<Vec<u8>>,
        user_data_sender: Sender<Vec<u8>>,
        rsa_encryption: Option<RsaPrivateKey>
    ) {
        let connected = self.close_notifier.clone();
        let peer_addr = self.peer_addr;
        let local_addr = self.local_addr;
        let recvq = self.recvq.clone();
        let sendq = self.sendq.clone();
        let close_notifier = self.close_notifier.clone();
        let incoming_notify = self.incomming_notifier.clone();
        let last_heartbeat_time = self.last_heartbeat_time.clone();
        let rsa_encryption = rsa_encryption.clone();
        let reference_time = self.reference_time.clone();
        let mut encrytion_context = rsa_encryption
            .map(|rsa_key| EncryptionHanshakeContext::new(peer_addr, rsa_key));

        let encryption = self.encryption.clone();
        let aes_key = self.aes_key.clone();

        let guid = Uuid::new_v4();

        tokio::spawn(async move {
            let mut connect_mode = ConnectMode::UnverifiedSender;

            'receive_loop: loop {
                if connected.is_closed() {
                    break 'receive_loop;
                }

                let mut recvq = recvq.lock().await;
                for f in recvq.flush() {
                    match Self::handle(
                        connect_mode,
                        guid,
                        &f, 
                        *reference_time,
                        &peer_addr, 
                        &local_addr, 
                        &sendq, 
                        &user_data_sender,
                        encrytion_context.as_mut()
                    ).await {
                        Ok(new_mode) => {
                            match new_mode {
                                ConnectMode::NoAction => (),
                                ConnectMode::DisconnectAsapSilently => {
                                    close_notifier.close();
                                    connect_mode = new_mode;
                                },
                                ConnectMode::Connected => {
                                    incoming_notify.notify_one();
                                    connect_mode = new_mode;
                                },
                                ConnectMode::SetEncryptionOnMultiple16BytePacket(key) => {
                                    aes_key.store(key, Ordering::Relaxed);
                                    connect_mode = new_mode;
                                },
                                _ => connect_mode = new_mode,
                            }
                        },
                        Err(e) => {
                            debug!("Message handler failed: {:?}", e);
                            close_notifier.close();
                        }
                    }
                }

                let mut buf = match receiver.recv().await {
                    Some(buf) => buf,
                    None => {
                        debug!("channel receiver finished");
                        connected.close();
                        break 'receive_loop;
                    }
                };

                if buf.len() > 2 {
                    if 
                        buf.len() % 16 == 0 && 
                        let ConnectMode::SetEncryptionOnMultiple16BytePacket(_) = connect_mode
                    {
                        debug!("Turning on encryption");
                        encryption.store(true, Ordering::Relaxed);
                    }

                    if encryption.load(Ordering::Relaxed) {
                        if let Err(e) = aes_decrypt(aes_key.load(Ordering::Relaxed), &mut buf) {
                            debug!("Decryption failed: {:?}", e);
                            continue;
                        }
                    }

                    if let Ok(frame) = MessageFrame::from(&buf) {
                        last_heartbeat_time.store(
                            cur_timestamp(*reference_time)
                                .as_millis()
                                .try_into()
                                .unwrap(), 
                            Ordering::Relaxed
                        );

                        if let Err(e) = recvq.insert(frame) {
                            debug!("Failed to insert frame into receive queue: {:?}", e);
                            connected.close();
                        }
                    } else {
                        debug!("Received malformed frame: {}", peer_addr);
                        connected.close();
                    }
                } else {
                    // React to certain internal packages
                    if let PacketID::DisconnectionNotification = PacketID::from(buf[0]) {
                        debug!("Received close command");
                        close_notifier.close();
                    }
                }
            }

            debug!("receiver finished: {}", peer_addr);
        });
    }

    fn start_tick(&self, s: &Arc<UdpSocket>, reapter: Sender<SocketAddr>, sent_notify: Arc<Notify>) {
        let connected = self.close_notifier.clone();
        let s = s.clone();
        let peer_addr = self.peer_addr;
        let sendq = self.sendq.clone();
        let recvq = self.recvq.clone();
        let reference_time = self.reference_time.clone();
        let mut last_monitor_tick = cur_timestamp(*reference_time);
        let last_heartbeat_time = self.last_heartbeat_time.clone();
        tokio::spawn(async move {
            loop {
                 tokio::select! {
                    _ = sleep(Duration::from_millis(SendQ::DEFAULT_TIMEOUT_MILLIS)) => (),
                    _ = sent_notify.notified() => (),
                };

                // get acks
                let acks = {
                    let mut recvq = recvq.lock().await;
                    recvq.get_ack()
                };

                // flush sendq
                let mut sendq = sendq.write().await;
                let frames = sendq.flush(cur_timestamp(*reference_time), &peer_addr);

                if !acks.is_empty() {
                    // send ack frame
                    let mut frame = MessageFrame::new(Reliability::Unreliable, vec![]);
                    frame.set_acks(cur_timestamp(*reference_time), acks);

                    debug!("TX: {:?}", frame);

                    let data = frame.serialize()
                            .expect("Failed to serialize message frame!");
                    let _ = s.send_to(&data, peer_addr).await;
                } 
            
                // send frames
                for f in frames {
                    debug!("TX: {:?}", f);

                    let data = f.serialize()
                        .expect("Failed to serialize message frame!");
                    let _ = s.send_to(&data, peer_addr).await;
                }

                if connected.is_closed() {
                    break;
                }

                // if we haven't received a message in 60s, close the connection.
                if (
                        cur_timestamp(*reference_time) - 
                        Duration::from_millis(last_heartbeat_time.load(Ordering::Relaxed))
                    ).as_millis() as u64 > RECEIVE_TIMEOUT
                {
                    debug!("recv timeout");
                    connected.close();
                }

                // Send close notification
                if connected.is_closed() {
                    for _ in 0..10 {
                        sendq.insert(Reliability::Unreliable, [
                            PacketID::DisconnectionNotification.to_u8()
                        ].to_vec()).unwrap();
                    }
                }
            }

            let _ = reapter.send(peer_addr).await;
            debug!("tick worker closed");
        });
    }

    async fn drop_watcher(&self) {
        let close_notifier = self.close_notifier.clone();
        let drop_notifier = self.drop_notifier.clone();

        tokio::spawn(async move {
            debug!("socket drop watcher start");
            drop_notifier.notify_one();

            drop_notifier.notified().await;

            if close_notifier.is_closed() {
                debug!("socket close notifier closed");
                return;
            }

            close_notifier.close();
            debug!("socket drop watcher closed");
        });

        self.drop_notifier.notified().await;
    }

    async fn handle(
        connect_mode: ConnectMode,
        guid: Uuid,
        frame: &MessageFrame,
        reference_time: SystemTime,
        peer_addr: &SocketAddr,
        local_addr: &SocketAddr,
        sendq: &RwLock<SendQ>,
        user_data_sender: &Sender<Vec<u8>>,
        encryption_context: Option<&mut EncryptionHanshakeContext>,
    ) -> Result<ConnectMode> {
        debug!("RX: {:?}", frame);

        // Process acks
        if let Some(acks) = frame.acks() {
            let mut sendq = sendq.write().await;

            for acks in acks {
                for message_number in acks.clone() {
                    sendq.ack(message_number, cur_timestamp(reference_time));
                }
            }
        }

        if !frame.data().is_empty() {
            if matches!(connect_mode, ConnectMode::UnverifiedSender) {
                if matches!(PacketID::from(frame.data()[0]), PacketID::ConnectionRequest) {
                    handle_connection_request(
                        guid,
                        peer_addr,
                        local_addr,
                        encryption_context.as_deref(),
                        sendq
                    ).await
                } else {
                    Ok(ConnectMode::DisconnectAsapSilently)
                }
            } else {
                match PacketID::from(frame.data()[0]) {
                    PacketID::ConnectionRequest => {
                        if matches!(connect_mode, ConnectMode::RequestedConntection) {
                            handle_connection_request(
                                guid,
                                peer_addr,
                                local_addr,
                                encryption_context.as_deref(),
                                sendq
                            ).await
                        } else {
                            let mut writer = RakNetWriter::new();
                            write_connection_request_accepted(&mut writer, *peer_addr, *local_addr, guid)?;
                    
                            sendq.write().await
                                .insert(Reliability::Reliable, writer.take_buffer())?;

                            Ok(ConnectMode::NoAction)
                        }
                    },
                    PacketID::NewIncomingConnection => {
                        if 
                            matches!(connect_mode, ConnectMode::HandlingConnectionRequest) ||
                            matches!(connect_mode, ConnectMode::RequestedConntection) ||
                            matches!(connect_mode, ConnectMode::SetEncryptionOnMultiple16BytePacket(_)) 
                        {
                            // Immediately send ping
                            let mut writer = RakNetWriter::new();
                            writer.write_u8(PacketID::InternalPing.to_u8());
                            writer.write_u32(cur_timestamp(reference_time).as_millis() as u32);

                            sendq.write().await
                                .insert(Reliability::Unreliable, writer.take_buffer())?;

                            Ok(ConnectMode::Connected)
                        } else {
                            Ok(ConnectMode::NoAction)
                        }
                    },
                    PacketID::ConnectedPong => Ok(ConnectMode::NoAction),
                    PacketID::DisconnectionNotification => {
                        Ok(ConnectMode::DisconnectAsap)
                    },
                    PacketID::InternalPing => {
                        let mut buf = RakNetReader::new(frame.data());
                        let send_ping_time = buf.read_u32()?;

                        let mut buf = RakNetWriter::new();
                        buf.write_u8(PacketID::ConnectedPong.to_u8());
                        buf.write_u32(send_ping_time);
                        buf.write_u32(cur_timestamp(reference_time).as_millis() as u32);

                        sendq.write().await
                            .insert(Reliability::Unreliable, buf.take_buffer())?;

                        Ok(ConnectMode::NoAction)
                    },
                    PacketID::SecuredConnectionConfirmation => {
                        if let Some(context) = encryption_context {
                            let mut buf = RakNetReader::new(frame.data());
                            let _ = buf.read_u8()?;
                            let mut syn_cookie = [0u8; 20];
                            buf.read(&mut syn_cookie)?;

                            if syn_cookie == context.syn_cookie() {
                                let mut aes_key = [0u8; 16];
                                let mut message = [0u8; 64];
                                buf.read(&mut message)?;

                                if let Ok(message) = rsa_decrypt_and_check(
                                    context.rsa_key(), 
                                    Option::<&mut OsRng>::None, 
                                    &BigUint::from_bytes_le(&message)) {

                                    let message = message.to_bytes_le();

                                    for i in 0..aes_key.len() {
                                        aes_key[i] = syn_cookie[i] ^ message[i];
                                    }

                                    /*let mut buf = RakNetWriter::new();
                                    write_connection_request_accepted(
                                        &mut buf, 
                                        *peer_addr, 
                                        *local_addr, 
                                        guid
                                    )?;

                                    sendq.write().await
                                        .insert(Reliability::Reliable, buf.take_buffer())?;*/

                                    return Ok(ConnectMode::SetEncryptionOnMultiple16BytePacket(u128::from_le_bytes(aes_key)));
                                }
                            } else {
                                // generate a new cookie
                                context.create_syn_cookie();
                            }
                        }

                        Ok(ConnectMode::NoAction)
                    }
                    PacketID::User(_) => {
                        match user_data_sender.send(frame.data().to_vec()).await {
                            Ok(_) => Ok(ConnectMode::NoAction),
                            Err(_) => Ok(ConnectMode::DisconnectAsap)
                        }
                    },
                    _ => {
                        warn!("Packet id {:?} not implemented!", PacketID::from(frame.data()[0]));
                        Ok(ConnectMode::DisconnectAsap)
                    }
                }
            }
        } else {
            Ok(ConnectMode::NoAction)
        }
    }
}

impl Drop for RakNetSocket {
    fn drop(&mut self) {
        self.drop_notifier.notify_one();
    }
}

async fn handle_connection_request(
    guid: Uuid,
    peer_addr: &SocketAddr,
    local_addr: &SocketAddr,
    encryption_context: Option<&EncryptionHanshakeContext>, 
    sendq: &RwLock<SendQ>
) -> Result<ConnectMode> {
    if let Some(context) = encryption_context {
        let mut writer = RakNetWriter::new();
        let public = context.rsa_key().to_public_key();
        
        write_secured_connection_response(&mut writer, context.syn_cookie(), public)?;

        sendq
            .write().await
            .insert(Reliability::Unreliable, writer.take_buffer())?;

    } else {
        let mut writer = RakNetWriter::new();
        write_connection_request_accepted(&mut writer, *peer_addr, *local_addr, guid)?;

        sendq.write().await
            .insert(Reliability::Reliable, writer.take_buffer())?;
    }

    Ok(ConnectMode::HandlingConnectionRequest)
}