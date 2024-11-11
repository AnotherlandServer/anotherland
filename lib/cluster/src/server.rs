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

use std::{collections::{HashMap, HashSet}, io::{self, Read}, marker::PhantomData, net::ToSocketAddrs, ops::Deref, sync::Arc, time::Duration};

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use log::{debug, info, warn};
use tokio::{sync::{broadcast::{self, Receiver}, mpsc, Mutex, RwLock}, time};
use zeromq::{util::PeerIdentity, Endpoint, RouterSocket, Socket, SocketEvent, SocketRecv, SocketSend, ZmqMessage};

use crate::{identifier::Identifier, message::{Request, Response}, notification::Notification, state::StateMessage, ClusterResult, Error};

#[derive(Default)]
struct ClientState {
    subscriptions: HashSet<String>,
}

impl ClientState {
    fn is_subscribed(&self, topic: &str) -> bool {
        for subscription in &self.subscriptions {
            if topic.starts_with(subscription) {
                return true;
            }
        }

        false
    }
}

#[derive(Clone)]
pub enum ClusterEvent {
    Accepted(PeerIdentity),
    Disconnected(PeerIdentity),
}

pub struct ClusterServer<T: Request, TR: Response, N: Notification> {
    tx_sender: mpsc::Sender<ZmqMessage>,
    rx_receiver: Arc<Mutex<mpsc::Receiver<(PeerIdentity, T)>>>,
    clients: Arc<RwLock<HashMap<PeerIdentity, ClientState>>>,
    event_sender: broadcast::Sender<ClusterEvent>,

    _phantom: PhantomData<(N, TR)>,

    endpoint: Endpoint,
}

async fn handle_state_message(
    identity: &PeerIdentity,
    message: StateMessage,
    clients: Arc<RwLock<HashMap<PeerIdentity, ClientState>>>
) {
    let mut clients = clients.write().await;
    let client = clients.entry(identity.clone()).or_insert_with(ClientState::default);

    match message {
        StateMessage::Subscribe(topic) => {
            client.subscriptions.insert(topic);
        },
        StateMessage::Unsubscribe(topic) => {
            client.subscriptions.remove(&topic);
        },
    }
}

impl <T: Request + 'static, TR: Response, N: Notification>ClusterServer<T, TR, N> {
    pub async fn bind(uri: &str) -> ClusterResult<Self> {
        let mut socket = RouterSocket::new();
        let endpoint = socket.bind(uri).await?;

        info!("server listening on {}", endpoint);

        let clients = Arc::new(RwLock::new(HashMap::new()));
        let (tx_sender, tx_receiver) = mpsc::channel(100);
        let (rx_sender, rx_receiver) = mpsc::channel(100);
        let (event_sender, _) = broadcast::channel(100);

        async fn start_monitor(mut monitor: futures_channel::mpsc::Receiver<SocketEvent>, clients: Arc<RwLock<HashMap<PeerIdentity, ClientState>>>, events: broadcast::Sender<ClusterEvent>) {
            tokio::spawn(async move {
                loop {
                    match monitor.next().await {
                        Some(SocketEvent::Accepted(endpoint, identity)) => {
                            debug!("connected cluster client {:?} @ {}", identity, endpoint);

                            let mut clients = clients.write().await;
                            clients.entry(identity.clone()).or_insert_with(ClientState::default);
                            let _ = events.send(ClusterEvent::Accepted(identity));
                        },
                        Some(SocketEvent::Disconnected(identity)) => {
                            debug!("disconnected cluster client {:?}", identity);

                            let mut clients = clients.write().await;
                            clients.remove(&identity);
                            let _ = events.send(ClusterEvent::Disconnected(identity));
                        },
                        Some(_) => (),
                        None => break,
                    }
                }
            });
        }

        async fn start_socket<T: Request + 'static>(
            mut socket: RouterSocket, 
            tx_sender: mpsc::Sender<ZmqMessage>,
            mut tx_receiver: mpsc::Receiver<ZmqMessage>,
            rx_sender: mpsc::Sender<(PeerIdentity, T)>,
            clients: Arc<RwLock<HashMap<PeerIdentity, ClientState>>>
        ) {
            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(1));

                loop {
                    tokio::select! {
                        Some(message) = tx_receiver.recv() => {
                            debug!("Send: {:?}", message);
                            let _ = socket.send(message).await;
                        },
                        Ok(message) = socket.recv() => {
                            let identity: PeerIdentity = message.get(0).unwrap().to_owned().try_into().unwrap();
                            let identifier = flexbuffers::from_slice::<Identifier>(message.get(1).unwrap()).unwrap();
        
                            match identifier {
                                Identifier::Request => {
                                    let request = flexbuffers::from_slice(message.get(2).unwrap()).unwrap();
                                    let _ = rx_sender.send((identity, request)).await;
                                },
                                Identifier::State => {
                                    let state = flexbuffers::from_slice::<StateMessage>(message.get(2).unwrap()).unwrap();
                                    handle_state_message(&identity, state, clients.clone()).await;
                                },
                                Identifier::Pong => (),
                                _ => unreachable!(),
                            }
                        },
                        _ = interval.tick() => {
                            for identifier in clients.read().await.keys() {
                                let mut msg = ZmqMessage::from(Bytes::from(identifier.clone()));
                                msg.push_back(flexbuffers::to_vec(Identifier::Ping).unwrap().into());
                                let _ = tx_sender.send(msg).await;
                            }
                        }
                    }
                }
            });
        }
        
        start_monitor(socket.monitor(), clients.clone(), event_sender.clone()).await;
        start_socket::<T>(socket, tx_sender.clone(), tx_receiver, rx_sender, clients.clone()).await;

        Ok(Self {
            tx_sender,
            rx_receiver: Arc::new(Mutex::new(rx_receiver)),
            clients,
            event_sender,
            _phantom: PhantomData,
            endpoint,
        })
    }

    pub fn endpoint(&self) -> &Endpoint { &self.endpoint }
    pub fn events(&self) -> Receiver<ClusterEvent> { self.event_sender.subscribe() }

    pub async fn notify(&self, notification: N) -> ClusterResult<()> {
        let topic = notification.topic_name();

        let clients = self.clients.read().await;
        let mut base_message = ZmqMessage::from(flexbuffers::to_vec(Identifier::Notification)?);
        base_message.push_back(Bytes::from_static(topic.as_bytes()));
        base_message.push_back(flexbuffers::to_vec(notification)?.into());

        for (identifier, state) in clients.iter() {
            if !state.is_subscribed(topic) { continue; }

            let mut message = base_message.clone();
            message.push_front(Bytes::from(identifier.clone()));

            let _ = self.tx_sender.send(message).await;
        }

        Ok(())
    }

    pub async fn send(&self, peer: &PeerIdentity, msg: TR) -> ClusterResult<()> {
        let mut frame = ZmqMessage::from(Bytes::from(peer.clone()));
        frame.push_back(flexbuffers::to_vec(Identifier::Response)?.into());
        frame.push_back(flexbuffers::to_vec(msg)?.into());

        self.tx_sender.send(frame).await
            .map_err(|_| Error::IoError(io::Error::from(io::ErrorKind::BrokenPipe)))
    }

    pub async fn recv(&self) -> ClusterResult<(PeerIdentity, T)> {
        let mut receiver = self.rx_receiver.lock().await;
        receiver.recv().await
            .ok_or(Error::IoError(io::Error::from(io::ErrorKind::BrokenPipe)))
    }
}