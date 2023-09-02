use super::{Guid, Error, PeerAddress, Packet, MessageNumber, Reliability, PacketSplit, RakNetRequest, RakNetResponse, Message, RequestHandler};
use std::{time::Instant, time::Duration, sync::Arc, net::{SocketAddr}, collections::VecDeque};
use tokio::net::UdpSocket;
use async_trait::async_trait;

struct ResendBuffer {
    nextAction: Instant,
}

pub enum Priority {
    System,
    High,
    Medium,
    Low,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    HalfOpen,
    Connected,
    HalfClosed,
    Disconnected,
}

impl Priority {
    pub fn count_priorities() -> usize { 4 }
}

pub struct RakNetPeer<'a> {
    guid: Guid,
    remote_address: PeerAddress,
    local_address: PeerAddress,
    socket: Arc<UdpSocket>,
    state: State,

    awaiting_ack_queue: VecDeque<Packet<'a>>,
    resend_queue: VecDeque<Packet<'a>>,
    send_queue_prioritized: Vec<VecDeque<Packet<'a>>>,
    remote_time: Duration,
    created: Instant,

    next_send_message_id: u32,
    next_recv_message_id: u32,
}

impl <'a>RakNetPeer<'a> {
    fn create_prioritized_send_queue() -> Vec<VecDeque<Packet<'a>>> {
        let mut vec = Vec::new();

        for i in 0..Priority::count_priorities() {
            vec.push(VecDeque::new());
        }

        vec
    }

    pub fn new<'b>(socket: Arc<UdpSocket>, remote_addr: SocketAddr, local_addr: SocketAddr) -> Result<Self, Error<'b>> {
        match remote_addr {
            SocketAddr::V4(a) => {
                Ok(Self {
                    guid: Guid::create_random(),
                    remote_address: PeerAddress::new(a.ip(), a.port()),
                    local_address: match local_addr {
                        SocketAddr::V4(a) => PeerAddress::new(a.ip(), a.port()),
                        _ => panic!("Unsupported address type!"),
                    },
                    socket,
                    state: State::HalfOpen,

                    awaiting_ack_queue: VecDeque::new(),
                    resend_queue: VecDeque::new(),
                    send_queue_prioritized: Self::create_prioritized_send_queue(),
                    remote_time: Duration::default(),
                    created: Instant::now(),

                    next_send_message_id: 0,
                    next_recv_message_id: 0,
                })
            },
            _ => Err(Error::InvalidAddressFormat),
        }
    }

    pub async fn handle_raw_message(&mut self, number: MessageNumber, reliability: Reliability, split: PacketSplit, data: Vec<u8>) -> (Option<RakNetRequest>, RakNetResponse) {
        let mut response = RakNetResponse::new(self.remote_time.clone());

        if number >= self.next_recv_message_id {
            match reliability {
                Reliability::Reliable | Reliability::ReliableOrdered(_) |Reliability::ReliableSequenced(_) => 
                    response.add_ack(number),
                _ => (),
            }

            self.next_recv_message_id = number.wrapping_add(1);

            match Message::from_bytes(data.as_slice()) {
                Ok((_, message)) => (Some(RakNetRequest::new(message)), response),
                Err(e) => {
                    println!("Message parse error:\n{:#?}", e);
                    panic!();
                    (None, response)
                }
            }
        } else {
            println!("Unexpected message number");
            (None, RakNetResponse::new(self.remote_time.clone()))
        }
    }

    pub async fn handle_request(&mut self, handler: &mut dyn RequestHandler, request: RakNetRequest, response: &mut RakNetResponse) {
        //println!("Message: {:#?}", *request.message());

        match request.message() {
            Message::InternalPing { time } => {
                self.remote_time = time.clone();

                response.add_message(Reliability::Unreliable, Message::ConnectedPong { 
                    remote_time: self.remote_time, 
                    local_time: Instant::now().duration_since(self.created) 
                });
            },

            Message::ConnectionRequest { password } => {
                println!("Got connection reqeuest!");

                self.state = State::Connected;

                response.add_message(Reliability::Reliable, Message::ConnectionRequestAccepted { 
                        index: 0, 
                        peer_addr: self.remote_address, 
                        own_addr: self.local_address,
                        guid: self.guid, 
                    });
            },

            Message::NewIncomingConnection { primary_address, secondary_addresses } => {
                println!("Primary address: {:#?}", primary_address);
                println!("Secondary addresses: {:#?}", secondary_addresses);

                response.add_message(Reliability::Unreliable, Message::InternalPing { time: Instant::now().duration_since(self.created) });
            },

            Message::DisconnectionNotification => {
                self.state = State::HalfClosed;

                response.add_message(Reliability::Reliable, Message::DisconnectionNotification);
            }

            _ => { 
                println!("RX {}: {:#?}", self.socket.local_addr().unwrap().port(), request.message());
                let _ = handler.handle_request(self, &request, response).await; 
            },
        }
    }

    pub fn generate_next_message_id(&mut self) -> u32 {
        let msg = self.next_send_message_id;
        self.next_send_message_id = self.next_send_message_id.wrapping_add(1);

        msg
    }

    pub fn remote_address(&self) -> PeerAddress {
        self.remote_address
    }

    pub fn local_address(&self) -> PeerAddress {
        self.local_address
    }


    pub fn remote_time(&self) -> Duration {
        self.remote_time
    }

    pub fn state(&self) -> State { 
        self.state
    }

    pub async fn run_update(&mut self) {

    }
}
