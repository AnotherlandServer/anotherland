use std::{sync::Arc, collections::HashMap, net::SocketAddr, time, io::{Error, ErrorKind}, hash};
use chrono::Duration;
use sha1::{Sha1, Digest};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle, sync::mpsc::{self, Receiver}, time::timeout};
use rsa::traits::PublicKeyParts;
use num_traits::cast::ToPrimitive;

use super::{RakNetPeer, Message, ReliableMessage};

enum ClientThreadMessage {

}

pub struct RakNetListener {
    sock: Arc<UdpSocket>,
}

impl RakNetListener {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<RakNetListener> {
        let sock = Arc::new(UdpSocket::bind(addr).await?);

        println!("RakNet listening on {}", sock.local_addr().unwrap().to_string());

        Ok(RakNetListener { 
            sock,
        })
    }

    pub async fn run(&mut self) -> io::Result<()> {
        let mut buf = Box::new([0; 65527]);
        let mut peers = HashMap::<SocketAddr, RakNetPeer>::new();
        let mut last_update_run = time::Instant::now();

        loop {
            match timeout(time::Duration::from_millis(50), self.sock.recv_from(&mut *buf)).await {
                // Process message
                Ok(Ok((received, peer_addr))) => {
                    let handler_result = async {
                        let data = &buf[0..received];

                        // Do we already have a peer for this address? Handle online connection
                        if let Some(peer) = peers.get_mut(&peer_addr) {
                            self.handle_online_message(peer, data).await?;
                        } else {
                            // Handle offline request. If it's a connection request, we return a new peer here
                            if let Some(peer) = self.handle_offline_request( peer_addr, data).await? {
                                peers.insert(peer_addr, peer);
                            }
                        }

                        Ok::<_, io::Error>(())
                    }.await;

                    match handler_result {
                        Ok(_) => (),
                        Err(e) => println!("Packet handler error {:#?}", e),
                    }
                },
                Ok(Err(_)) |
                Err(_) => (),
            }


            // If 10ms have passed, update all clients
            if time::Instant::now().duration_since(last_update_run).as_millis() >= 500 {
                last_update_run = time::Instant::now();

                for p in peers.values_mut() {
                    p.update().await;
                }
            }
        }

        Ok(())
    }

    async fn handle_online_message(&mut self, peer: &mut RakNetPeer, packet: &[u8]) -> io::Result<()> {
        let (_, (system_time, frame)) = ReliableMessage::from_bytes(packet)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Message parsing failed"))?;

        if let Some(system_time) = system_time {
            peer.update_time(system_time);
        }
    
        println!("Got online message: {:#?}", frame);
        match &frame {
            ReliableMessage::Ack { time, id_ranges } => {
                peer.handle_acknowledgements(time, id_ranges);
                Ok(())
            },
            ReliableMessage::MessageFrame { id, reliability, split, message } => {
                peer.acknowledge(*id).await;

                match message {
                    Message::ConnectionRequest => {
                        /*let mut hasher = Sha1::new();

                        let binary_address: u32 = match peer.peer_addr().ip() {
                            std::net::IpAddr::V4(a) => a.into(),
                            _ => 0,
                        };

                        hasher.update(binary_address.to_le_bytes());
                        hasher.update(peer.peer_addr().port().to_le_bytes());

                        peer.send_unreliable_message(Message::SecuredConnectionResponse {
                            hash: hasher.finalize().into(),
                            e: peer.pub_key().e().to_u32().unwrap(),
                            modulus: peer.pub_key().n().to_bytes_le().as_slice().try_into().unwrap(),
                        }).await*/

                        peer.send_unreliable_message(Message::ConnectionRequestAccepted { 
                            index: 0, 
                            peer_addr: peer.peer_addr(), 
                            own_addr: peer.own_addr(), 
                            guid: [0x01, 0x02, 0x03, 0x04] 
                        }).await
                    }
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Unexpected message id")),
                }
            }
        }
    }

    async fn handle_offline_request(&mut self, peer_addr: SocketAddr, packet: &[u8]) -> io::Result<Option<RakNetPeer>> {
        let message = Message::from_bytes(packet)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Message parsing failed"))?;

        println!("Got offline message: {:#?}", message);

        match message {
            Message::OpenConnectionRequest { version } => {
                // TODO: Validate version

                let peer = RakNetPeer::connecting(self.sock.clone(), peer_addr);

                peer.send_connectionless_message(Message::OpenConnectionReply).await?;

                Ok(Some(peer))
            },
            _ => Err(Error::new(ErrorKind::InvalidInput, "Unexpected message id")),
        }
    }
}


