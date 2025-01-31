// Copyright (C) 2025 AnotherlandServer
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

use std::{net::SocketAddr, sync::Arc, time::{Duration, Instant, SystemTime}};

use log::{debug, trace, warn};
use rsa::{hazmat::rsa_decrypt_and_check, rand_core::{OsRng, RngCore}, BigUint, RsaPrivateKey};
use sha1::{Sha1, Digest};
use tokio::{net::UdpSocket, sync::{mpsc::{channel, Receiver, Sender}, oneshot, Mutex, Notify}, time::sleep};
use uuid::Uuid;

use crate::{buffer::{RakNetReader, RakNetWriter}, encryption::{aes_decrypt, aes_encrypt, EncryptionHanshakeContext}, error::Result, frame::{Message, MessageFrame}, packet::{write_connection_request_accepted, write_secured_connection_response}, reliability::{RecvQ, Reliability, SendQ}, util::cur_timestamp, PacketID, RakNetError};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum ConnectionState {
    DisconnectAsap,
    DisconnectAsapSilently,
    Disconnecting,
    Disconnected,
    RequestedConntection,
    HandlingConnectionRequest,
    UnverifiedSender,
    SetEncryptionOnMultiple16BytePacket(u128),
    Connected,
}

const RECEIVE_TIMEOUT: u64 = 60000;

pub struct RakNetSocket {
    id: Uuid,
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    close_notifier: Arc<Notify>,
    drop_notifier: Arc<Notify>,
    incoming_notify: Arc<Notify>,
    incoming_receiver: Mutex<Receiver<Vec<u8>>>,
    outgoing_sender: Sender<(Vec<u8>, Reliability, oneshot::Sender<Result<()>>)>,
}

impl RakNetSocket {
    pub(crate) async fn open(
        addr: &SocketAddr,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<Vec<u8>>,
        mtu: u16,
        reaper: Sender<SocketAddr>,
        rsa_key: Option<RsaPrivateKey>,
    ) -> Result<Self> {
        let (incoming_sender, incoming_receiver) = channel(10);
        let (outgoing_sender, mut outgoing_receiver) = channel(10);

        let mut random_number = [0u8; 20];
        OsRng.fill_bytes(&mut random_number);

        let mut hasher = Sha1::new();
        hasher.update(addr.ip().to_string());
        hasher.update(addr.port().to_le_bytes());
        hasher.update(random_number);

        let reference_time = SystemTime::now();
        let drop_notifier = Arc::new(Notify::new());
        let incoming_notify = Arc::new(Notify::new());
        let close_notifier = Arc::new(Notify::new());

        let socket = Self {
            id: Uuid::new_v4(),
            local_addr: s.local_addr().unwrap(),
            peer_addr: *addr,
            close_notifier: close_notifier.clone(),
            drop_notifier: drop_notifier.clone(),
            incoming_notify: incoming_notify.clone(),
            incoming_receiver: Mutex::new(incoming_receiver),
            outgoing_sender,
        };

        let peer_addr = *addr;
        let local_addr = s.local_addr().unwrap();
        let mut encryption_context = rsa_key
            .map(|rsa_key| EncryptionHanshakeContext::new(peer_addr, rsa_key));
        let s = s.clone();
        
        let guid = socket.id;

        tokio::spawn(async move {
            let mut state = ConnectionState::UnverifiedSender;
            let mut recvq = RecvQ::new();
            let mut sendq = SendQ::new(mtu);

            let mut aes_key: u128 = 0;
            let mut encryption_active = false;
            let mut last_heartbeat_time = Instant::now();
            let mut remote_time = Duration::default();

            'net_loop: while !matches!(state, ConnectionState::Disconnected) {
                tokio::select! {
                    _ = sleep(Duration::from_millis(SendQ::DEFAULT_TIMEOUT_MILLIS)) => (),
                    _ = drop_notifier.notified() => {
                        trace!("got notified about socket drop");
                        close_notifier.notify_one();
                    },
                    _ = close_notifier.notified() => {
                        trace!("beginning disconnect");

                        if matches!(state, ConnectionState::Disconnecting) {
                            break;
                        } else {
                            state = ConnectionState::Disconnecting;
                            sendq.insert(Reliability::ReliableOrdered, [
                                PacketID::DisconnectionNotification.to_u8()
                            ].to_vec()).unwrap();
                        }
                    },
                    Some((buf, r, cb)) = outgoing_receiver.recv(), if !outgoing_receiver.is_closed() => {
                        let _ = cb.send(
                            sendq.insert(r, buf)
                        );
                    },
                    buf = receiver.recv() => {
                        if let Some(mut buf) = buf {
                            if buf.len() > 2 {
                                if 
                                    buf.len() % 16 == 0 && 
                                    let ConnectionState::SetEncryptionOnMultiple16BytePacket(_) = state
                                {
                                    let mut test_buf = buf.clone();
            
                                    // Test key before enabling encryption
                                    if aes_decrypt(aes_key, &mut test_buf).is_ok() {
                                        trace!("Turning on encryption");
                                        encryption_active = true;
                                    }
                                }
            
                                if encryption_active {
                                    if let Err(e) = aes_decrypt(aes_key, &mut buf) {
                                        trace!("Decryption failed: {:?}", e);
                                        continue;
                                    }
                                }
            
                                if let Ok(frame) = MessageFrame::from(&buf) {
                                    // Process acks
                                    if let Some(acks) = frame.acks() {
                                        for acks in acks {
                                            for message_number in acks.clone() {
                                                sendq.ack(message_number, cur_timestamp(reference_time));
                                            }
                                        }
                                    }

                                    last_heartbeat_time = Instant::now();
            
                                    if let Some(time) = frame.remote_system_time() {
                                        remote_time = time;
                                    }
            
                                    // Insert frame
                                    for message in frame.into_message_vector() {
                                        recvq.insert(message);
            
                                        // acknowledge received frames
                                        let acks = recvq.get_ack();
            
                                        if !acks.is_empty() {
                                            // send ack frame
                                            let mut frame = MessageFrame::new();
                                            frame.set_acks(remote_time, acks);
            
                                            let data = frame.serialize()
                                                    .expect("Failed to serialize message frame!");
            
                                            if encryption_active {
                                                let data = aes_encrypt(aes_key, &data);
                                                let _ = s.send_to(&data, peer_addr).await;
                                            } else {
                                                let _ = s.send_to(&data, peer_addr).await;
                                            }
                                        }
                                    }
                                } else {
                                    debug!("Received malformed frame: {}", peer_addr);
                                }

                                // Flush receive queue
                                for f in recvq.flush() {
                                    match Self::handle(
                                        state,
                                        guid,
                                        &f, 
                                        reference_time,
                                        &peer_addr, 
                                        &local_addr, 
                                        &mut sendq, 
                                        &incoming_sender,
                                        encryption_context.as_mut()
                                    ).await {
                                        Ok(Some(next_state)) => {
                                            match next_state {
                                                ConnectionState::DisconnectAsap => {
                                                    for _ in 0..10 {
                                                        let _ = sendq.insert(Reliability::Unreliable, [
                                                            PacketID::DisconnectionNotification.to_u8()
                                                        ].to_vec());
                                                    }
            
                                                    close_notifier.notify_one();
                                                },
                                                ConnectionState::DisconnectAsapSilently => {
                                                    break 'net_loop;
                                                },
                                                ConnectionState::Disconnecting => {
                                                    let _ = sendq.insert(Reliability::ReliableOrdered, [
                                                        PacketID::DisconnectionNotification.to_u8()
                                                    ].to_vec());
                                                },
                                                ConnectionState::Connected => {
                                                    incoming_notify.notify_one();
                                                },
                                                ConnectionState::SetEncryptionOnMultiple16BytePacket(key) => {
                                                    aes_key = key;
                                                },
                                                _ => (),
                                            }

                                            state = next_state;
                                        },
                                        Ok(None) => (),
                                        Err(e) => {
                                            debug!("Message handler failed: {:?}", e);
                                            close_notifier.notify_one();
                                        },
                                    }
                                }
                            } else {
                                // React to certain internal packages
                                if let PacketID::DisconnectionNotification = PacketID::from(buf[0]) {
                                    if matches!(state, ConnectionState::Disconnecting) {
                                        state = ConnectionState::Disconnected;
                                    } else {
                                        close_notifier.notify_one();
                                    }
                                }
                            }
                        } else {
                            // The server socket went down.
                            // Immediately leave the peer loop.
                            break;
                        }
                    }
                }

                // flush sendq
                let frames = sendq.flush(cur_timestamp(reference_time), &peer_addr);

                // send frames
                for f in frames {
                    trace!("TX: {:?}", f);

                    let mut frame = MessageFrame::new();
                    frame.add_message(f);

                    let data = frame.serialize()
                        .expect("Failed to serialize message frame!");
                    if encryption_active {
                        let data = aes_encrypt(aes_key, &data);
                        let _ = s.send_to(&data, peer_addr).await;
                    } else {
                        let _ = s.send_to(&data, peer_addr).await;
                    }
                }

                // if we haven't received a message in 60s, immediately close the connection.
                if (Instant::now() - last_heartbeat_time).as_millis() as u64 > RECEIVE_TIMEOUT {
                    break;
                }

                if matches!(state, ConnectionState::DisconnectAsap) ||
                    (matches!(state, ConnectionState::Disconnecting) && sendq.acks_pending() == 0)
                {
                    break;
                }
            }

            let _ = reaper.send(peer_addr).await;
        });

        // wait for incomming notify or close
        let incoming_notify = socket.incoming_notify.clone();
        let close_notifier = socket.close_notifier.clone();
    
        tokio::select! {
            _ = incoming_notify.notified() => {
                Ok(socket)
            },
            _ = close_notifier.notified() => {
                Err(RakNetError::HandshakeFailed)
            }
        }
    }

    pub async fn close(&self) {
        self.close_notifier.notify_one();
    }

    pub async fn send(&self, buf: &[u8], r: Reliability) -> Result<()> {
        let (res_sender, res_receiver) = oneshot::channel();

        self.outgoing_sender.send((buf.to_vec(), r, res_sender)).await
            .map_err(|_| RakNetError::ConnectionClosed)?;

        res_receiver.await
            .map_err(|_| RakNetError::ConnectionClosed)?
    }

    pub async fn recv(&self) -> Result<Vec<u8>> {
        match self.incoming_receiver.lock().await.recv().await {
            Some(p) => Ok(p),
            None => {
                Err(RakNetError::ConnectionClosed)
            }
        }
    }

    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }

    pub fn peer_addr(&self) -> &SocketAddr {
        &self.peer_addr
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle(
        state: ConnectionState,
        guid: Uuid,
        frame: &Message,
        reference_time: SystemTime,
        peer_addr: &SocketAddr,
        local_addr: &SocketAddr,
        sendq: &mut SendQ,
        user_data_sender: &Sender<Vec<u8>>,
        encryption_context: Option<&mut EncryptionHanshakeContext>,
    ) -> Result<Option<ConnectionState>> {
        trace!("{:?} - RX: {:?}", state, frame);

        if frame.data().is_empty() { return Ok(None); }

        if matches!(state, ConnectionState::UnverifiedSender) {
            if matches!(PacketID::from(frame.data()[0]), PacketID::ConnectionRequest) {
                handle_connection_request(
                    guid,
                    peer_addr,
                    local_addr,
                    encryption_context.as_deref(),
                    sendq
                ).await
            } else {
                Ok(Some(ConnectionState::DisconnectAsapSilently))
            }
        } else {
            match PacketID::from(frame.data()[0]) {
                PacketID::ConnectionRequest => {
                    if matches!(state, ConnectionState::RequestedConntection) {
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
                
                        sendq.insert(Reliability::Reliable, writer.take_buffer())?;

                        Ok(None)
                    }
                },
                PacketID::NewIncomingConnection => {
                    if 
                        matches!(state, ConnectionState::HandlingConnectionRequest) ||
                        matches!(state, ConnectionState::RequestedConntection) ||
                        matches!(state, ConnectionState::SetEncryptionOnMultiple16BytePacket(_)) 
                    {
                        // Immediately send ping
                        let mut writer = RakNetWriter::new();
                        writer.write_u8(PacketID::InternalPing.to_u8());
                        writer.write_u32(cur_timestamp(reference_time).as_millis() as u32);

                        sendq.insert(Reliability::Unreliable, writer.take_buffer())?;

                        Ok(Some(ConnectionState::Connected))
                    } else {
                        Ok(None)
                    }
                },
                PacketID::ConnectedPong => Ok(None),
                PacketID::DisconnectionNotification => {
                    Ok(Some(ConnectionState::Disconnecting))
                },
                PacketID::InternalPing => {
                    let mut buf = RakNetReader::new(frame.data());
                    let send_ping_time = buf.read_u32()?;

                    let mut buf = RakNetWriter::new();
                    buf.write_u8(PacketID::ConnectedPong.to_u8());
                    buf.write_u32(send_ping_time);
                    buf.write_u32(cur_timestamp(reference_time).as_millis() as u32);

                    sendq.insert(Reliability::Unreliable, buf.take_buffer())?;

                    Ok(None)
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

                                let mut buf = RakNetWriter::new();
                                write_connection_request_accepted(
                                    &mut buf, 
                                    *peer_addr, 
                                    *local_addr, 
                                    guid
                                )?;

                                sendq.insert(Reliability::Reliable, buf.take_buffer())?;

                                return Ok(Some(ConnectionState::SetEncryptionOnMultiple16BytePacket(u128::from_le_bytes(aes_key))));
                            } else {
                                warn!("RSA handshake failed");
                            }
                        } else {
                            trace!("Syn cookie mismatch. Generating a new one.");

                            // generate a new cookie
                            context.create_syn_cookie();
                        }
                    } else {
                        trace!("No encryption context to handle SecuredConnectionConfirmation");
                    }

                    Ok(None)
                },
                PacketID::GamePerformanceReport => {
                    // Packet contains a 32-bit integer, containing the games tick count.
                    Ok(None)
                },
                PacketID::User(_) => {
                    match user_data_sender.send(frame.data().to_vec()).await {
                        Ok(_) => Ok(None),
                        Err(_) => Ok(Some(ConnectionState::DisconnectAsap))
                    }
                },
                _ => {
                    warn!("Packet id {:?} not implemented!", PacketID::from(frame.data()[0]));
                    Ok(Some(ConnectionState::DisconnectAsap))
                }
            }
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
    sendq: &mut SendQ
) -> Result<Option<ConnectionState>> {
    if let Some(context) = encryption_context {
        let mut writer = RakNetWriter::new();
        let public = context.rsa_key().to_public_key();
        
        write_secured_connection_response(&mut writer, context.syn_cookie(), public)?;

        sendq.insert(Reliability::Unreliable, writer.take_buffer())?;

    } else {
        let mut writer = RakNetWriter::new();
        write_connection_request_accepted(&mut writer, *peer_addr, *local_addr, guid)?;

        sendq.insert(Reliability::Reliable, writer.take_buffer())?;
    }

    Ok(Some(ConnectionState::HandlingConnectionRequest))
}