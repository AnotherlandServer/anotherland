use std::{sync::{Arc, mpsc::Receiver}, net::SocketAddr, collections::{VecDeque, LinkedList, HashMap}, future::Future};

use nom::{bits, sequence::tuple, error::{context, VerboseError}, combinator::{flat_map, cond, map}, IResult, multi::many0, Map};
use tokio::{net::{ToSocketAddrs, UdpSocket}, io, sync::{mpsc::{self, Sender}, broadcast}, sync::oneshot};

use super::{peer::RakNetPeer, Packet, RakNetRequest, Message};

enum RakNetListenerCommand {
    ReceivedDatagram(SocketAddr, Vec<u8>),
}

pub struct RakNetListener {
    sender: Sender<RakNetListenerCommand>,
    messages: broadcast::Receiver<Arc<RakNetRequest>>,
}

const MAX_MTU_SIZE: usize = 1492;
const MAX_ACCEPT_BACKLOG: usize = 10;

impl RakNetListener {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<RakNetListener> {
        let socket = Arc::new(UdpSocket::bind(addr).await?);
        let (thread_tx,  mut thread_rx) = mpsc::channel::<RakNetListenerCommand>(100);
        let (request_tx, request_rx) = broadcast::channel::<Arc<RakNetRequest>>(100);

        // Data pump
        let data_pump_thread_tx = thread_tx.clone();
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

        tokio::task::spawn(async move {
            let mut listener = RakNetListenerImpl::new();

            'update_loop: loop {
                // Check for commands
                match thread_rx.recv().await {
                    Some(cmd) => match cmd {
                        RakNetListenerCommand::ReceivedDatagram(addr, data) => listener.ingest_datagram(addr, data).await,
                    },
                    None => { break 'update_loop; }
                }
            }
        });

        Ok(Self {
            sender: thread_tx,
            messages: request_rx,
        })
    }

    pub fn get_request_receiver(&self) -> broadcast::Receiver<Arc<RakNetRequest>> {
        self.messages.resubscribe()
    }
}

struct RakNetListenerImpl {
    peers: HashMap<SocketAddr, RakNetPeer>,
}

impl RakNetListenerImpl {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub async fn ingest_datagram(&mut self, addr: SocketAddr, data: Vec<u8>) {
        if let Ok((_, packets)) = Self::parse_datagram(data.as_slice()) {
            for p in packets {
                println!("{:#?}", p);

                match p {
                    // Handle initial handshake
                    Packet::OfflineMessage(Message::OpenConnectionRequest { version }) => {
                        // TODO: Send failure message on version mismatch or error during creation
                        // of RakNetPeer
                        if version == 3 {
                            if let Ok(peer) = RakNetPeer::new(addr) {
                                // Create new unconnected peer and add it to the list
                                self.peers.insert(addr, peer);
                            }
                        }
                    },

                    Packet::OfflineMessage(Message::OpenConnectionReply) => {},

                    // All messages are raw messages directly after parsing. 
                    // We need to combine and order them here for the higher layers.
                    Packet::RawMessage { number, reliability, split, data } => {
                        if let Some(peer) = self.peers.get_mut(&addr) {
                            peer.handle_raw_message(number, reliability, split, data).await;
                        }
                    },

                    _ => unreachable!("Unexpected message parsed"),
                }
            }
        }
    }

    fn parse_datagram<'a>(data: &'a[u8]) -> IResult<&'a[u8], Vec<Packet>, VerboseError<&'a[u8]>> {
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
                res.append(&mut packets);

                res
            }))(data)
        }
    }
} 