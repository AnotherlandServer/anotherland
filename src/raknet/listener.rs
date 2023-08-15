use std::{sync::{Arc, mpsc::Receiver}, net::{SocketAddr, SocketAddrV4}, collections::{VecDeque, LinkedList, HashMap}, future::Future};

use nom::{bits, sequence::tuple, error::{context, VerboseError}, combinator::{flat_map, cond, map}, IResult, multi::many0, Map};
use tokio::{net::{ToSocketAddrs, UdpSocket}, io, sync::{mpsc::{self, Sender}, broadcast}, sync::oneshot, time};

use super::{peer::RakNetPeer, Packet, RakNetRequest, Message, RequestHandler, RakNetResponse, State, PeerAddress};

enum RakNetListenerCommand {
    ReceivedDatagram(SocketAddr, Vec<u8>),
    UpdateClients,
}

pub struct RakNetListener {
    sender: Sender<RakNetListenerCommand>,
    messages: broadcast::Receiver<Arc<RakNetRequest>>,
}

const MAX_MTU_SIZE: usize = 1492;
const MAX_ACCEPT_BACKLOG: usize = 10;

impl RakNetListener {
    pub async fn bind<A: ToSocketAddrs>(handler: Box<dyn RequestHandler>, addr: A) -> io::Result<RakNetListener> {
        let bound_socket = Arc::new(UdpSocket::bind(addr).await?);
        let (thread_tx,  mut thread_rx) = mpsc::channel::<RakNetListenerCommand>(100);
        let (request_tx, request_rx) = broadcast::channel::<Arc<RakNetRequest>>(100);

        // Data pump
        {
            let data_pump_thread_tx = thread_tx.clone();
            let socket = bound_socket.clone();
            tokio::task::spawn(async move {
                let mut buf: [u8; MAX_MTU_SIZE] = [0; MAX_MTU_SIZE];

                loop {
                    // Pump new packets to update thread
                    if let Ok((size, addr)) = socket.recv_from(&mut buf).await {
                        println!("Got message from {:#?} len {}", addr, size);
                        let _ = data_pump_thread_tx.send(RakNetListenerCommand::ReceivedDatagram(addr, buf[0..size].to_vec())).await;
                    } else {
                        break;
                    }
                }
            });
        }

        // Update timer
        {
            let timer_thread_tx = thread_tx.clone();
            tokio::task::spawn(async move {
                let mut interval = time::interval(time::Duration::from_millis(1000));
                loop {
                    interval.tick().await;
                    let _ = timer_thread_tx.send(RakNetListenerCommand::UpdateClients).await;
                }
            });
        }

        {
            let socket = bound_socket.clone();
            tokio::task::spawn(async move {
                let mut listener = RakNetListenerImpl::new(socket, handler);

                'update_loop: loop {
                    // Check for commands
                    match thread_rx.recv().await {
                        Some(cmd) => match cmd {
                            RakNetListenerCommand::ReceivedDatagram(addr, data) => listener.ingest_datagram(addr, data).await,
                            RakNetListenerCommand::UpdateClients => listener.update_clients().await,
                        },
                        None => { break 'update_loop; }
                    }
                }
            });
        }

        Ok(Self {
            sender: thread_tx,
            messages: request_rx,
        })
    }

    pub fn get_request_receiver(&self) -> broadcast::Receiver<Arc<RakNetRequest>> {
        self.messages.resubscribe()
    }
}

struct RakNetListenerImpl<'a> {
    socket: Arc<UdpSocket>,
    peers: HashMap<SocketAddr, RakNetPeer<'a>>,
    handler: Box<dyn RequestHandler>,
}

impl <'a>RakNetListenerImpl<'a> {
    pub fn new(socket: Arc<UdpSocket>, handler: Box<dyn RequestHandler>) -> Self {
        Self {
            socket,
            peers: HashMap::new(),
            handler,
        }
    }

    pub async fn ingest_datagram(&mut self, addr: SocketAddr, data: Vec<u8>) {
        if let Ok((_, packets)) = Self::parse_datagram(data.as_slice()) {
            for p in packets {
                match p {
                    // Handle initial handshake
                    Packet::OfflineMessage(Message::OpenConnectionRequest { version }) => {
                        // TODO: Send failure message on version mismatch or error during creation
                        // of RakNetPeer
                        if version == 3 {
                            if let Ok(peer) = RakNetPeer::new(self.socket.clone(), addr, self.socket.local_addr().unwrap()) {
                                if let Ok(_) = self.socket.send_to(Message::OpenConnectionReply.to_bytes().as_slice(), addr).await {
                                    // Create new unconnected peer and add it to the list
                                    self.peers.insert(addr, peer);
                                }
                            }
                        }
                    },

                    Packet::OfflineMessage(Message::OpenConnectionReply) => {},

                    Packet::Ack(_, _) => {},

                    Packet::SystemTime(_) => {},

                    // All messages are raw messages directly after parsing. 
                    // We need to combine and order them here for the higher layers.
                    Packet::RawRequest { number, reliability, split, data } => {
                        if let Some(peer) = self.peers.get_mut(&addr) {
                            let (request, mut response) = peer.handle_raw_message(number, reliability, split, data).await;
                            
                            // Run request handlers
                            if let Some(request) =  request{
                                let _ = peer.handle_request(self.handler.as_mut(), request, &mut response).await;
                            }

                            let response_data = response.pack_response(peer);
                            let _ = self.socket.send_to(&response_data, SocketAddr::V4(SocketAddrV4::new(peer.remote_address().ip, peer.remote_address().port))).await;
                        }
                    },

                    _ => unreachable!("Unexpected message parsed"),
                }
            }
        }
    }

    pub async fn update_clients(&mut self) {
        let mut remove_peers = Vec::new();

        for (addr, peer) in &mut self.peers {
            let mut update = RakNetResponse::new(peer.remote_time());

            if peer.state() == State::HalfClosed || peer.state() == State::Disconnected {
                remove_peers.push(addr.clone());
                continue;
            }

            self.handler.update_client(&peer, &mut update).await;

            let update_data = update.pack_response(peer);
            if !update_data.is_empty() {
                let _ = self.socket.send_to(&update_data, SocketAddr::V4(SocketAddrV4::new(peer.remote_address().ip, peer.remote_address().port))).await;
            }
        }

        // Cleanup clients
        for addr in remove_peers {
            self.peers.remove(&addr);
        }
    }

    fn parse_datagram<'b>(data: &'b[u8]) -> IResult<&'b[u8], Vec<Packet>, VerboseError<&'b[u8]>> {
        if Message::test_offline_message(data) {
            Message::from_bytes(data).map(|(i, m)| (i, vec![Packet::OfflineMessage(m)]))
        } else {
            bits(map(tuple((
                context("acks", flat_map(
                    nom::bits::complete::bool, 
                    |has_acks| cond(has_acks, Packet::parse_ack))),
                context("system_time", flat_map(
                    nom::bits::complete::bool, 
                    |has_time| cond(has_time, Packet::parse_system_time))),
                many0(Packet::parse_packet)
            )), |(acks, system_time, mut packets)| {
                let mut res  = Vec::new();
                if let Some(acks) = acks { res.push(acks); }
                if let Some(system_time) = system_time { res.push(system_time); }

                println!("{:#?}", packets);
                res.append(&mut packets);

                res
            }))(data)
        }
    }
} 