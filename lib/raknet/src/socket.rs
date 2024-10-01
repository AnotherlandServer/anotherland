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

use std::{net::SocketAddr, sync::{atomic::AtomicU8, Arc}};

use log::{debug, error, warn};
use rsa::rand_core::{OsRng, RngCore};
use sha1::{Sha1, Digest};
use tokio::{net::UdpSocket, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Notify, RwLock, Semaphore}};

use crate::{buffer::RakNetWriter, error::Result, frame::MessageFrame, packet::read_connection_request, reliability::{RecvQ, Reliability, SendQ}, PacketID, RakNetError};

pub struct RakNetSocket {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    close_notifier: Arc<Semaphore>,
    loss_rate: Arc<AtomicU8>,
    sender: Sender<(Vec<u8>, SocketAddr, bool, u8)>,
    drop_notifier: Arc<Notify>,
    incomming_notifier: Arc<Notify>,
    recvq: Arc<Mutex<RecvQ>>,
    sendq: Arc<RwLock<SendQ>>,
    syn_cookie: Arc<Mutex<Vec<u8>>>,
}

impl RakNetSocket {
    pub(crate) async fn open(
        addr: &SocketAddr,
        s: &Arc<UdpSocket>,
        receiver: Receiver<Vec<u8>>,
        mtu: u16,
        reaper: &Sender<SocketAddr>,
    ) -> Result<Self> {
        let (user_data_sender, user_data_receiver) = channel::<Vec<u8>>(100);
        let (sender_sender, sender_reciever) = channel::<(Vec<u8>, SocketAddr, bool, u8)>(10);

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
            loss_rate: Arc::new(AtomicU8::new(0)),
            sender: sender_sender,
            drop_notifier: Arc::new(Notify::new()),
            incomming_notifier: Arc::new(Notify::new()),
            recvq: Arc::new(Mutex::new(RecvQ::new())),
            sendq: Arc::new(RwLock::new(SendQ::new(mtu))),
            syn_cookie: Arc::new(Mutex::new(hasher.finalize().to_vec())),
        };

        socket.start_receiver(s, receiver, user_data_sender);
        socket.start_tick(s, reaper.clone());
        socket.start_sender(s, sender_reciever);
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

    fn start_receiver(
        &self,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<Vec<u8>>,
        user_data_sender: Sender<Vec<u8>>,
    ) {
        let connected = self.close_notifier.clone();
        let peer_addr = self.peer_addr;
        let local_addr = self.local_addr;
        let s = s.clone();
        let loss_rate = self.loss_rate.clone();
        let recvq = self.recvq.clone();
        let sendq = self.sendq.clone();
        let close_notifier = self.close_notifier.clone();
        let incoming_notify = self.incomming_notifier.clone();
        let syn_cookie = self.syn_cookie.clone();

        tokio::spawn(async move {
            'receive_loop: loop {
                if connected.is_closed() {
                    break 'receive_loop;
                }

                let mut recvq = recvq.lock().await;
                for f in recvq.flush() {
                    if let Err(e) = 
                        Self::handle(
                            &f, 
                            &peer_addr, 
                            &local_addr, 
                            &sendq, 
                            &user_data_sender, 
                            &incoming_notify,
                            syn_cookie.clone()
                        ).await
                    {
                        debug!("Message handler failed: {:?}", e);
                        close_notifier.close();
                    }
                }

                let buf = match receiver.recv().await {
                    Some(buf) => buf,
                    None => {
                        debug!("channel receiver finished");
                        connected.close();
                        break 'receive_loop;
                    }
                };


            }

            debug!("receiver finished: {}", peer_addr);
        });
    }

    fn start_sender(
        &self,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<(Vec<u8>, SocketAddr, bool, u8)>
    ) {

    }

    fn start_tick(&self, s: &Arc<UdpSocket>, reapter: Sender<SocketAddr>) {

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
        frame: &MessageFrame,
        peer_addr: &SocketAddr,
        local_addr: &SocketAddr,
        sendq: &RwLock<SendQ>,
        user_data_sender: &Sender<Vec<u8>>,
        incoming_notify: &Notify,
        syn_cookie: Arc<Mutex<Vec<u8>>>,
    ) -> Result<bool> {
        if !frame.data().is_empty() {
            debug!("Received: {:?}", PacketID::from(frame.data()[0]));

            match PacketID::from(frame.data()[0]) {
                PacketID::ConnectionRequest => {
                    let syn_cookie = syn_cookie.lock().await;
                    let buf = RakNetWriter::new();
                    
                    

                    sendq
                        .write().await
                        .insert(Reliability::ReliableOrdered, buf.take_buffer())?;

                    Ok(true)
                },
                PacketID::ConnectedPong => Ok(true),
                PacketID::DisconnectionNotification => {
                    Ok(false)
                },
                PacketID::User(_) => {
                    match user_data_sender.send(frame.data().to_vec()).await {
                        Ok(_) => Ok(true),
                        Err(_) => Ok(false)
                    }
                },
                _ => {
                    warn!("Oacket id {:?} not implemented!", PacketID::from(frame.data()[0]));
                    Ok(true)
                }
            }
        } else {
            Ok(true)
        }
    }
}

impl Drop for RakNetSocket {
    fn drop(&mut self) {
        self.drop_notifier.notify_one();
    }
}