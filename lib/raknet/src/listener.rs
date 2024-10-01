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

use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::{Duration, SystemTime}};

use log::{debug, info};
use tokio::{net::{ToSocketAddrs, UdpSocket}, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Notify, Semaphore}};

use crate::{buffer::RakNetWriter, error::{RakNetError, Result}, frame::MessageFrame, packet::read_open_connection_request, reliability::Reliability, PacketID, RakNetSocket, MAX_MTU_SIZE, RECV_BUFFER_SIZE};

type SessionSender = (Duration, Sender<Vec<u8>>);

pub struct RakNetListener {
    socket: Option<Arc<UdpSocket>>,
    close_notifier: Arc<Semaphore>,
    drop_notifier: Arc<Notify>,
    listening: bool,
    sessions: Arc<Mutex<HashMap<SocketAddr, SessionSender>>>,
    connection_receiver: Receiver<RakNetSocket>,
    connection_sender: Sender<RakNetSocket>,
}

impl RakNetListener {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await
            .map_err(|_| RakNetError::BindAddressError)?;

        let (connection_sender, connection_receiver) = channel::<RakNetSocket>(10);

        let ret = Self {
            socket: Some(Arc::new(socket)),
            close_notifier: Arc::new(Semaphore::new(0)),
            drop_notifier: Arc::new(Notify::new()),
            listening: false,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            connection_receiver,
            connection_sender,
        };

        ret.drop_watcher().await;
        Ok(ret)
    }

    async fn drop_watcher(&self) {
        let close_notifier = self.close_notifier.clone();
        let drop_notifier = self.drop_notifier.clone();

        tokio::spawn(async move {
            drop_notifier.notify_one();

            // Wait for RakNetListener beeing droped
            drop_notifier.notified().await;

            if close_notifier.is_closed() {
                log::debug!("listener already closed");
                return;
            }

            close_notifier.close();
            log::debug!("listener closed by drop watcher");
        });

        self.drop_notifier.notified().await;
    }

    pub async fn listen(&mut self, max_connections: usize) {
        if self.close_notifier.is_closed() || self.listening {
            return;
        }

        if let Some(socket) = self.socket.as_ref().cloned() {
            self.listening = true;

            let start_time = SystemTime::now();

            let local_addr = socket.local_addr().unwrap();
            let close_notify = self.close_notifier.clone();
            let sessions = self.sessions.clone();
            let connection_sender = self.connection_sender.clone();

            let (reaper_sender, reaper_receiver) = channel::<SocketAddr>(10);

            tokio::spawn(async move {
                let mut buf = [0u8; RECV_BUFFER_SIZE];

                info!("Listening on {}", local_addr);

                'net_loop: loop {
                    let size;
                    let addr;
                    
                    // Wait for message
                    tokio::select! {
                        r = socket.recv_from(&mut buf) => {
                            match r {
                                Ok(p) => {
                                    size = p.0;
                                    addr = p.1;
                                },
                                Err(e) => {
                                    debug!("Listener recv error: {}", e);
                                    break 'net_loop;
                                }
                            }
                        },
                        _ = close_notify.acquire() => {
                            debug!("Listener closed");
                            break 'net_loop;
                        }
                    }

                    let mut sessions = sessions.lock().await;

                    // Check if we have a new connection request
                    if 
                        size <= size_of::<u16>() &&
                        let PacketID::OpenConnectionRequest = PacketID::from(buf[0])
                    {
                        if 
                            let Ok(req) = read_open_connection_request(&buf[1..]) &&
                            req.version == 3
                        {
                            let mut response = RakNetWriter::new();

                            if sessions.contains_key(&addr) {
                                response.write_u8(PacketID::AlreadyConnected.to_u8());
                            } else if sessions.len() >= max_connections {
                                response.write_u8(PacketID::NoFreeIncomingConnections.to_u8());
                            } else {
                                response.write_u8(PacketID::OpenConnectionReply.to_u8());

                                // Open new socket
                                let (sender, receiver) = channel::<Vec<u8>>(10);
        
                                sessions.insert(addr, (
                                    SystemTime::now().duration_since(start_time).unwrap(), 
                                    sender
                                ));

                                let reaper_sender = reaper_sender.clone();
                                let connection_sender = connection_sender.clone();
                                let socket = socket.clone();

                                tokio::spawn(async move {
                                    if let Ok(socket) = RakNetSocket::open(
                                        &addr, 
                                        &socket, 
                                        receiver, 
                                        MAX_MTU_SIZE as u16, 
                                        &reaper_sender
                                    ).await {
                                        let _ = connection_sender.send(socket).await;
                                    }
                                });
                            }

                            response.write_u8(0); // Pad, some routers block 1 byte packets

                            // Send connection request result
                            let _ = socket.send_to(response.buffer(), addr).await;
                        }
                    } else if let Some(sender) = sessions.get_mut(&addr) {
                        sender.0 = SystemTime::now().duration_since(start_time).unwrap();
                        if sender.1.send(buf[0..size].to_vec()).await.is_err() {
                            sessions.remove(&addr);
                        }
                    }
                    
                }
            });
        }
    }

    pub async fn accept(&mut self) -> Result<RakNetSocket> {
        if !self.listening {
            Err(RakNetError::NotListening)
        } else {
            tokio::select! {
                r = self.connection_receiver.recv() => {
                    match r {
                        Some(socket) => Ok(socket),
                        None => {
                            Err(RakNetError::NotListening)
                        }
                    }
                },
                _ = self.close_notifier.acquire() => {
                    debug!("accept close notified");
                    Err(RakNetError::NotListening)
                }
            }
        }
    }
}

impl Drop for RakNetListener {
    fn drop(&mut self) {
        self.drop_notifier.notify_one();
    }
}