// Copyright (C) 2023 AnotherlandServer
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

use std::{sync::Arc, net::SocketAddr, time::Duration, collections::HashMap};

use log::{error, trace, debug};
use nom::{IResult, error::{VerboseError, context}, bits, combinator::{map, flat_map, cond}, sequence::tuple, multi::many0};
use tokio::{net::{ToSocketAddrs, UdpSocket}, time, sync::{mpsc, oneshot}, select, task::JoinHandle};
use futures::future::join_all;
use uuid::Uuid;

use crate::raknet::{State, RakNetPeerData};

use super::{MessageFragment, Message, RakNetResult, RakNetError, RakNetErrorKind, Priority, Reliability};

//pub const MAX_MTU_SIZE: usize = 1492;
pub const MAX_MTU_SIZE: usize = 1024;
pub const RECV_BUFFER_SIZE: usize = 2048;

/*pub struct RakNetInternal {
    pub(self) socket: Option<Arc<UdpSocket>>,
    pub(self) peer_guid_map: RwLock<HashMap<Uuid, RakNetPeerHandle>>,
    pub(self) peer_address_map: RwLock<HashMap<PeerAddress, RakNetPeerHandle>>,
    pub(self) command_channel: mpsc::Receiver<RakNetCommand>,
}*/

#[derive(Debug)]
enum RakNetCommand {
    Stop(oneshot::Sender<()>),
    PeerConnected(RakNetPeer),
    PeerDisconnected(SocketAddr),
}

pub struct RakNetListener {
    local_addr: SocketAddr,
    command_channel: mpsc::Sender<RakNetCommand>,
    connection_channel: mpsc::Receiver<RakNetPeer>,
    listen_task: Option<JoinHandle<()>>,
}

impl Drop for RakNetListener {
    fn drop(&mut self) {
        let command_channel = self.command_channel.clone();

        // tell the underlying task to stop on drop
        tokio::spawn(async move {
            let (result_sender, result_receiver) = oneshot::channel();

            // only wait for the result command when we sucessfully sent the command
            // to avoid a deadlock here.
            if let Ok(_) = command_channel.send(RakNetCommand::Stop(result_sender)).await {
                let _ = result_receiver.await;
            }
        });
    }
}

#[allow(unused)]
impl RakNetListener {
    pub async fn bind<'a, A: ToSocketAddrs>(address: A) -> RakNetResult<Self> {
        let (command_sender, mut command_receiver) = mpsc::channel(10);
        let (connection_sender, connection_receiver) = mpsc::channel(10);

        let mut socket = Arc::new(UdpSocket::bind(address).await?);
        
        let task_command_sender = command_sender.clone();
        let task_socket = socket.clone();

        let mut stopping = false;
        let mut peers: HashMap<SocketAddr, mpsc::Sender<RakNetPeerCommand>> = HashMap::new();

        debug!("Listening on {}", socket.local_addr().unwrap().to_string());

        let listen_task = Some(tokio::spawn(async move {
            'event_loop: loop {
                let mut buf = [0; RECV_BUFFER_SIZE];

                // stop task when there's nothing left to be done
                if stopping & peers.is_empty() {
                    break 'event_loop;
                }

                select! {
                    Some(cmd) = command_receiver.recv() => {
                        trace!("Listener command: {:#?}", cmd);

                        match cmd {
                            RakNetCommand::Stop(ret) => {
                                stopping = true;

                                let peers = peers.clone();

                                tokio::spawn(async move {
                                    let mut wait_handles = Vec::new();

                                    for (_, cmd) in peers {
                                        let (result_sender, result_receiver) = oneshot::channel();

                                        if let Ok(_) = cmd.send(RakNetPeerCommand::Disconnect(result_sender)).await {
                                            wait_handles.push(result_receiver);
                                        }
                                    }

                                    // wait for all connections to close
                                    join_all(wait_handles);

                                    ret.send(());
                                });
                            },
                            RakNetCommand::PeerConnected(peer) => {
                                if !stopping {
                                    let _ = connection_sender.send(peer).await;
                                } else {
                                    // immediately disconnect the peer if we're stopping
                                    peer.disconnect().await;
                                }
                            },
                            RakNetCommand::PeerDisconnected(addr) => {
                                peers.remove(&addr);
                            }
                        }
                    },
                    result = task_socket.recv_from(&mut buf) => {
                        match result {
                            Ok((len, addr)) => {
                                match Self::parse_datagram(&buf[..len]) {
                                    Ok((_, fragments)) => {
                                        // is there a task running for this peer?
                                        if let Some(cmd) = peers.get(&addr) {
                                            // let the peer task handle to fragments
                                            if let Err(_) = cmd.send(RakNetPeerCommand::Digest(fragments)).await {
                                                peers.remove(&addr);
                                            }
                                        } else {
                                            let (peer_cmd_sender, mut peer_cmd_receiver) = mpsc::channel(10);
                                            let (peer_event_sender, peer_event_receiver) = mpsc::channel(10);

                                            let task_command_sender = task_command_sender.clone();

                                            let mut update_interval = time::interval(Duration::from_millis(10));

                                            let mut prev_state = State::Unconnected;
 
                                            if let Ok(mut peer) = RakNetPeerData::new(
                                                    task_socket.clone(), 
                                                    addr.clone(), 
                                                    task_socket.local_addr().unwrap()
                                                ) {

                                                debug!("Got new connection from {}", addr.to_string());
                                                peers.insert(addr, peer_cmd_sender.clone());

                                                let mut peet_handle = Some(RakNetPeer::new(peer.guid().clone(), peer_event_receiver, peer_cmd_sender.clone()));

                                                tokio::spawn(async move {
                                                    loop {
                                                        select! {
                                                            Some(cmd) = peer_cmd_receiver.recv() => {
                                                                trace!("Peer command: {:#?}", cmd);

                                                                match cmd {
                                                                    RakNetPeerCommand::Disconnect(ret) => { 
                                                                        peer.disconnect().await;

                                                                        let _ = ret.send(());
                                                                    },
                                                                    RakNetPeerCommand::Send(priority, reliability, message, ret) => { 
                                                                        if let Ok(_) = peer.send(priority, reliability, message).await {
                                                                            // run update once to immediately send messages
                                                                            let _ = peer.run_update().await;

                                                                            ret.send(Ok(()));
                                                                        } else {
                                                                            ret.send(Err(RakNetError::from_kind(RakNetErrorKind::IOError)));
                                                                        }
                                                                    },
                                                                    RakNetPeerCommand::Digest(fragments) => { 
                                                                        match peer.digest_message_fragments(fragments).await {
                                                                            Ok(messages) => {
                                                                                for message in messages {
                                                                                    let _ = peer_event_sender.send(RakNetPeerEvent::Message(message)).await;
                                                                                }

                                                                                // run update once immediately after digesting the messages
                                                                                let _ = peer.run_update().await;

                                                                                // check connection state
                                                                                if *peer.state() == State::Connected && prev_state == State::Unconnected {
                                                                                    let _ = task_command_sender.send(RakNetCommand::PeerConnected(peet_handle.take().unwrap())).await;
                                                                                    prev_state = *peer.state();
                                                                                } else if *peer.state() == State::Disconnected {
                                                                                    let _ = task_command_sender.send(RakNetCommand::PeerDisconnected(addr.clone())).await;
                                                                                    break;
                                                                                }
                                                                            },
                                                                            Err(e) => {
                                                                                error!("Error while digesting message fragments: {:#?}", e);
                                                                            }
                                                                        }
                                                                    },
                                                                }
                                                            },
                                                            _ = update_interval.tick() => {
                                                                if let Err(e) = peer.run_update().await {
                                                                    error!("Peer update failed. Closing. Error: {}", e.to_string());
                                        
                                                                    peer.disconnect_immediate();
                                                                }

                                                                // stop loop on disconnect
                                                                if *peer.state() == State::Disconnected {
                                                                    let _ = task_command_sender.send(RakNetCommand::PeerDisconnected(addr.clone())).await;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("Received malformed packet: {}", e.to_string());
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Socket recv error: {:?}", e);
                                break 'event_loop;
                            }
                        }
                    }
                }
            }

            debug!("Stopped on {}", task_socket.local_addr().unwrap().to_string());
        }));

        Ok(Self {
            local_addr: socket.local_addr()?,
            command_channel: command_sender,
            connection_channel: connection_receiver,
            listen_task,
        })
    }  

    pub async fn accept(&mut self) -> RakNetResult<RakNetPeer> {
        self.connection_channel.recv().await.ok_or(RakNetError::from_kind(RakNetErrorKind::IOError))
    }

    pub async fn shutdown(&mut self) -> RakNetResult<()> {
        let (result_sender, result_receiver) = oneshot::channel();

        // if the task is already stopped, sending might fail and the
        // command is never executed.
        // therefore avoid waiting on the return channel if the command send failed.
        if let Ok(_) = self.command_channel.send(RakNetCommand::Stop(result_sender)).await {
            let res = result_receiver.await.map_err(|e| RakNetError::from_kind(RakNetErrorKind::IOError));
            let _ = self.listen_task.take().unwrap().await;

            res
        } else {
            Ok(())
        }
    }

    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }

    fn parse_datagram<'b>(data: &'b[u8]) -> IResult<&'b[u8], Vec<MessageFragment>, VerboseError<&'b[u8]>> {
        if Message::test_offline_message(data) {
            Message::from_bytes(data).map(|(i, m)| (i, vec![MessageFragment::OfflineMessage(m)]))
        } else {
            bits(map(tuple((
                context("acks", flat_map(
                    nom::bits::complete::bool, 
                    |has_acks| cond(has_acks, MessageFragment::parse_ack))),
                context("system_time", flat_map(
                    nom::bits::complete::bool, 
                    |has_time| cond(has_time, MessageFragment::parse_system_time))),
                many0(MessageFragment::parse_packet)
            )), |(acks, system_time, mut packets)| {
                let mut res  = Vec::new();
                if let Some(acks) = acks { res.push(acks); }
                if let Some(system_time) = system_time { res.push(system_time); }

                //println!("{:#?}", packets);
                res.append(&mut packets);

                res
            }))(data)
        }
    }
}

#[derive(Debug)]
enum RakNetPeerCommand {
    Disconnect(oneshot::Sender<()>),
    Send(Priority, Reliability, Message, oneshot::Sender<RakNetResult<()>>),
    Digest(Vec<MessageFragment>),
}

enum RakNetPeerEvent {
    Message(Message),
}

#[derive(Debug)]
pub struct RakNetPeer {
    id: Uuid,
    event_receiver: mpsc::Receiver<RakNetPeerEvent>,
    command: mpsc::Sender<RakNetPeerCommand>,
}

impl RakNetPeer {
    fn new(id: Uuid, event_receiver: mpsc::Receiver<RakNetPeerEvent>, command_sender: mpsc::Sender<RakNetPeerCommand>) -> Self {
        Self {
            id,
            event_receiver,
            command: command_sender,
        }
    }

    pub fn id(&self) -> &Uuid { &self.id }

    pub async fn recv(&mut self) -> RakNetResult<Message> {
        loop {
            match self.event_receiver.recv().await {
                Some(RakNetPeerEvent::Message(msg)) => {
                    break Ok(msg)
                },
                None => {
                    break Err(RakNetError::from_kind(RakNetErrorKind::IOError))
                }
            }
        }
    }

    pub async fn send(&self, priority: Priority, reliability: Reliability, message: Message) -> RakNetResult<()> {
        let (result_sender, result_receiver) = oneshot::channel();

        if let Ok(_) = self.command.send(RakNetPeerCommand::Send(priority, reliability, message, result_sender)).await {
            match result_receiver.await {
                Ok(r) => r,
                Err(_) => Err(RakNetError::from_kind(RakNetErrorKind::IOError)),
            }
        } else {
            Err(RakNetError::from_kind(RakNetErrorKind::IOError))
        }
    }

    pub async fn disconnect(&self) {
        let (result_sender, result_receiver) = oneshot::channel();

        if let Ok(_) = self.command.send(RakNetPeerCommand::Disconnect(result_sender)).await {
            let _ = result_receiver.await;
        }
    }
}

impl Drop for RakNetPeer {
    fn drop(&mut self) {
        let command = self.command.clone();

        // disconnect peer on drop
        tokio::spawn(async move {
            let (result_sender, result_receiver) = oneshot::channel();

            if let Ok(_) = command.send(RakNetPeerCommand::Disconnect(result_sender)).await {
                let _ = result_receiver.await;
            }
        });
    }
}

