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

use std::{io, marker::PhantomData, net::ToSocketAddrs};

use log::debug;
use tokio::sync::{mpsc::{self, Receiver, Sender}, Mutex};
use zeromq::{util::PeerIdentity, DealerSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

use crate::{identifier::Identifier, state::StateMessage, ClusterResult, Error, Notification, Request, Response};

enum ClientMessage<T: Request> {
    State(StateMessage),
    Request(T),
}

pub struct ClusterClient<T: Request, TR: Response, N: Notification> {
    tx_sender: Sender<ClientMessage<T>>,
    rx_receiver: Mutex<Receiver<TR>>,
    _phantom: PhantomData<(N, T, TR)>,
}

impl <T: Request + 'static, TR: Response + 'static, N: Notification + 'static>ClusterClient<T, TR, N> {
    pub async fn connect(uri: &str) -> ClusterResult<(Self, Receiver<N>)> {
        let (notification_sender, notification_receiver) = mpsc::channel(100);
        let (tx_sender, tx_receiver) = mpsc::channel(100);
        let (rx_sender, rx_receiver) = mpsc::channel(100);

        let mut socket = DealerSocket::new();

        socket.connect(uri).await?;

        async fn start_receiver_task<T: Request + 'static, TR: Response + 'static, N: Notification + 'static>(
            mut socket: DealerSocket, 
            mut tx_receiver: Receiver<ClientMessage<T>>, 
            rx_sender: Sender<TR>, 
            notification_sender: Sender<N>
        ) {
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        res = socket.recv() => {
                            match res {
                                Ok(message) => {
                                    let identifier = flexbuffers::from_slice::<Identifier>(message.get(0).unwrap()).unwrap();
                            
                                    match identifier {
                                        Identifier::Response => {
                                            let response = flexbuffers::from_slice(message.get(1).unwrap()).unwrap();
                                            let _ = rx_sender.send(response).await;
                                        },
                                        Identifier::Notification => {
                                            let notification = flexbuffers::from_slice(message.get(2).unwrap()).unwrap();
                                            let _ = notification_sender.send(notification).await;
                                        },
                                        Identifier::Ping => {
                                            let msg = ZmqMessage::from(flexbuffers::to_vec(Identifier::Pong).unwrap());
                                            let _ = socket.send(msg).await;
                                        },
                                        _ => unreachable!(),
                                    }
                                },
                                Err(e) => {
                                    debug!("Error receiving message: {:?}", e);
                                    break;
                                }
                            }
                        },
                        Some(message) = tx_receiver.recv() => {
                            match message {
                                ClientMessage::State(state_message) => {
                                    let mut msg = ZmqMessage::from(flexbuffers::to_vec(Identifier::State).unwrap());
                                    msg.push_back(flexbuffers::to_vec(state_message).unwrap().into());

                                    let _ = socket.send(msg).await;
                                },
                                ClientMessage::Request(request) => {
                                    let mut msg = ZmqMessage::from(flexbuffers::to_vec(Identifier::Request).unwrap());
                                    msg.push_back(flexbuffers::to_vec(request).unwrap().into());

                                    let _ = socket.send(msg).await;
                                },
                            }
                        }
                    }
                }
            });
        }

        start_receiver_task::<T, TR, N>(socket, tx_receiver, rx_sender, notification_sender).await;

        Ok((
            Self {
                tx_sender,
                rx_receiver: Mutex::new(rx_receiver),
                _phantom: PhantomData
            },
            notification_receiver
        ))
    }

    pub async fn subscribe(&self, topic: &str) -> ClusterResult<()> {
        self.tx_sender.send(ClientMessage::State(StateMessage::Subscribe(topic.to_string()))).await
            .map_err(|_| Error::IoError(io::ErrorKind::BrokenPipe.into()))
    }

    pub async fn unsubscribe(&self, topic: &str) -> ClusterResult<()> {
        self.tx_sender.send(ClientMessage::State(StateMessage::Unsubscribe(topic.to_string()))).await
            .map_err(|_| Error::IoError(io::ErrorKind::BrokenPipe.into()))
    }

    pub async fn send(&self, req: T) -> ClusterResult<()> {
        self.tx_sender.send(ClientMessage::Request(req)).await
            .map_err(|_| Error::IoError(io::ErrorKind::BrokenPipe.into()))
    }

    pub async fn recv(&self) -> ClusterResult<TR> {
        let mut rx_receiver = self.rx_receiver.lock().await;
        rx_receiver.recv().await
            .ok_or(Error::IoError(io::ErrorKind::BrokenPipe.into()))
    }
}