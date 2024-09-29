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

use log::debug;
use tokio::{net::UdpSocket, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Notify, Semaphore}};

pub struct RakNetSocket {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    close_notifier: Arc<Semaphore>,
    loss_rate: Arc<AtomicU8>,
    sender: Sender<(Vec<u8>, SocketAddr, bool, u8)>,
    drop_notifier: Arc<Notify>,
    connected_notifier: Arc<Notify>,
}

impl RakNetSocket {
    pub(crate) async fn open(
        addr: &SocketAddr,
        s: &Arc<UdpSocket>,
        receiver: Receiver<Vec<u8>>,
        mtu: u16,
        reaper: &Sender<SocketAddr>,
        connection_sender: &Sender<RakNetSocket>,
    ) {
        let (user_data_sender, user_data_receiver) = channel::<Vec<u8>>(100);
        let (sender_sender, sender_reciever) = channel::<(Vec<u8>, SocketAddr, bool, u8)>(10);

        let socket = Self {
            local_addr: s.local_addr().unwrap(),
            peer_addr: *addr,
            close_notifier: Arc::new(Semaphore::new(0)),
            loss_rate: Arc::new(AtomicU8::new(0)),
            sender: sender_sender,
            drop_notifier: Arc::new(Notify::new()),
            connected_notifier: Arc::new(Notify::new()),
        };

        socket.start_receiver(s, receiver, user_data_sender);
        socket.start_tick(s, reaper.clone());
        socket.start_sender(s, sender_reciever);
        socket.drop_watcher().await;

        socket.start_connected_watcher(connection_sender.clone());
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

        tokio::spawn(async move {
            'receive_loop: loop {
                if connected.is_closed() {
                    break 'receive_loop;
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

    fn start_connected_watcher(self, sender: Sender<RakNetSocket>) {
        let close_notifier = self.close_notifier.clone();
        let connected_notifier = self.connected_notifier.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = close_notifier.acquire() => {
                    debug!("socket closed before connected");
                },
                _ = connected_notifier.notified() => {
                    let _ = sender.send(self).await;
                }
            }
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
}

impl Drop for RakNetSocket {
    fn drop(&mut self) {
        self.drop_notifier.notify_one();
    }
}