use std::{sync::Arc, collections::{HashMap, VecDeque}, net::SocketAddr};

use log::{error, trace, debug};
use nom::{IResult, error::{VerboseError, context}, bits, combinator::{map, flat_map, cond}, sequence::tuple, multi::many0};
use tokio::{net::{ToSocketAddrs, UdpSocket}, task::{JoinHandle, self}, time, sync::{RwLock, mpsc}};

use crate::{Uuid, raknet::{RakNetPeer, State}};

use super::{RakNetPeerHandle, MessageFragment, Message, PeerAddress, RakNetResult, RakNetRequest};

//pub const MAX_MTU_SIZE: usize = 1492;
pub const MAX_MTU_SIZE: usize = 1024;
pub const RECV_BUFFER_SIZE: usize = 2048;

pub struct RakNetInternal {
    pub socket: Option<Arc<UdpSocket>>,
    pub peer_guid_map: RwLock<HashMap<Uuid, RakNetPeerHandle>>,
    pub peer_address_map: RwLock<HashMap<PeerAddress, RakNetPeerHandle>>,
    
    //pub request_connection_queue: RwLock<LinkedList<RakNetPeerHandle>>,
    pub request_queue_tx: mpsc::Sender<RakNetRequest>,
    pub request_queue_rx: Arc<RwLock<mpsc::Receiver<RakNetRequest>>>,
}

impl RakNetInternal {
    pub fn new() -> Self {
        let (request_queue_tx, request_queue_rx) = mpsc::channel(10);

        Self {
            socket: None,
            peer_guid_map: RwLock::new(HashMap::new()),
            peer_address_map: RwLock::new(HashMap::new()),
            //request_connection_queue: RwLock::new(LinkedList::new()),
            request_queue_tx,
            request_queue_rx: Arc::new(RwLock::new(request_queue_rx)),
        }
    }

    async fn enqueue_connecting_peer(&self, peer: RakNetPeer) -> RakNetPeerHandle {
        let peer_handle = Arc::new(RwLock::new(peer));
        let peer = peer_handle.read().await;

        trace!("Try adding new peer to maps");

        self.peer_address_map.write().await.insert(peer.remote_address().clone(), peer_handle.clone());
        self.peer_guid_map.write().await.insert(peer.guid().clone(), peer_handle.clone());
        //self.request_connection_queue.write().await.push_back(peer_handle.clone());

        trace!("Added peer instance for {:#?} with guid {}", peer.remote_address(), peer.guid().to_string());

        peer_handle.clone()
    }
}

pub struct RakNetListener {
    internal: Arc<RwLock<RakNetInternal>>,
    update_task: Option<Arc<JoinHandle<()>>>,
}

#[allow(unused)]
impl RakNetListener {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(RwLock::new(RakNetInternal::new())),
            update_task: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
            update_task: self.update_task.as_ref().map(|v| v.clone()),
        }
    }

    pub async fn listen<'a, A: ToSocketAddrs>(&mut self, addr: A) -> RakNetResult<()> {
        let mut internal: tokio::sync::RwLockWriteGuard<'_, RakNetInternal> = self.internal.write().await;
        let mut disconnected_peers = VecDeque::<RakNetPeerHandle>::new();

        internal.socket = Some(Arc::new(
            UdpSocket::bind(addr).await?
        ));

        debug!("Listening on {}", internal.socket.as_ref().unwrap().local_addr().unwrap().to_string());
        
        {
            let listener = self.internal.clone();
            self.update_task = Some(Arc::new(task::spawn(async move {
                let mut buf = vec![0u8; RECV_BUFFER_SIZE];
                let mut interval = time::interval(time::Duration::from_millis(1));
                
                let socket = {
                    let listener = listener.read().await;
                    listener.socket.as_ref().expect("Running update task without open socket").to_owned()
                };

                loop {
                    interval.tick().await;
                    
                    // Parse all received messages
                    while let Ok((size, addr)) = socket.try_recv_from(buf.as_mut()) {
                        match Self::parse_datagram(&buf[..size]) {
                            Ok((_, fragments)) => {
                                trace!("Received message fragments from {} - {:#?}", addr, fragments);

                                let listener = listener.read().await;
                                if let Ok(peer_address) = PeerAddress::try_from(addr) {

                                    let mut peer = listener.peer_address_map.read().await.get(&peer_address).map(|v| v.to_owned());
                                    
                                    if peer.is_none() {
                                        debug!("Got new connection from {}", addr.to_string());

                                        // peer is not connected yet, create new connection and add to connection queue
                                        match RakNetPeer::new(socket.to_owned(), addr, socket.local_addr().unwrap()) {
                                            Ok(new_peer) => {
                                                peer = Some(listener.enqueue_connecting_peer(new_peer).await);
                                            },
                                            Err(e) => {
                                                error!("Failed to create peer: {:#?}", e);
                                            },
                                        }
                                    }

                                    // Have we got a valid peer instance?
                                    // Then digest message fragments
                                    if let Some(peer) = peer {
                                        match peer.write().await.digest_message_fragments(fragments).await {
                                            Ok(messages) => {
                                                for message in messages {
                                                    listener.request_queue_tx.send(RakNetRequest::new(peer.to_owned(), message)).await;
                                                }
                                            },
                                            Err(e) => {
                                                error!("Error while digesting message fragments: {}", e.to_string())
                                            }
                                        }
                                    }
                                } else {
                                    error!("Peer ({}) has invalid address format. Otherland can only handle ipv4!", addr.to_string());
                                }
                            },
                            Err(e) => {
                                error!("Received malformed packet: {}", e.to_string());
                            }
                        }
                    }

                    let listener = listener.read().await;

                    // Update raknet peers
                    for peer in listener.peer_guid_map.read().await.values() {
                        if let Err(e) = peer.write().await.run_update().await {
                            error!("Peer update failed. Closing. Error: {}", e.to_string());

                            peer.write().await.disconnect_immediate();
                        }

                        if peer.read().await.state() == State::Disconnected {
                            disconnected_peers.push_back(peer.to_owned());
                        }
                    }

                    // Remove disconnected peers
                    while let Some(disconnected_peer) = disconnected_peers.pop_back() {
                        let disconnected_peer = disconnected_peer.read().await;

                        debug!("Peer {} disconnected.", disconnected_peer.guid().to_string());

                        listener.peer_guid_map.write().await.remove(disconnected_peer.guid());
                        listener.peer_address_map.write().await.remove(disconnected_peer.remote_address());
                    }
                }
            })));
        }

        Ok(())
    }

    pub async fn next_request<'a>(&self) -> Option<RakNetRequest> {
        let rx = self.internal.read().await.request_queue_rx.clone();
        let request = rx.write().await.recv().await;
        request
    }

    pub async fn try_next_request<'a>(&self) -> Option<RakNetRequest> {
        let rx = self.internal.read().await.request_queue_rx.clone();
        let request = rx.write().await.try_recv().ok();
        request
    }

    pub fn peer(&self, guid: &Uuid) -> Option<RakNetPeerHandle> {
        let listener = self.internal.blocking_read();
        let peer = listener.peer_guid_map.blocking_read().get(guid).map(|v| v.to_owned());
        peer
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

    pub async fn local_addr(&self) -> Option<SocketAddr> {
        self.internal.read().await.socket.as_ref().map_or(None, |v| Some(v.local_addr().unwrap()))
    }
}