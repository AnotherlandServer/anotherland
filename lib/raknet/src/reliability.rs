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

use core::ops::RangeInclusive;
use std::{collections::{hash_map::Entry, HashMap}, net::SocketAddr, sync::Arc, time::Duration};

use log::{debug, trace};
use tokio::sync::Notify;

use crate::{error::{RakNetError, Result}, fragment::FragmentQ, frame::{MessageFrame, Order, Split}};

#[derive(Debug, Clone, Copy)]
pub enum Reliability {
    // Same as regular UDP, except that it will also discard duplicate datagrams.
    // RakNet adds (6 to 17) + 21 bits of overhead, 16 of which is used to detect 
    // duplicate packets and 6 to 17 of which is used for message length.
    Unreliable,

    // Regular UDP with a sequence counter. Out of order messages will be discarded. 
    // This adds an additional 13 bits on top what is used for Unreliable.
    UnreliableSequenced,

    // The message is sent reliably, but not necessarily in any order. Same overhead as Unreliable.
    Reliable,

    // This message is reliable and will arrive in the order you sent it.
    // Messages will be delayed while waiting for out of order messages.  
    // Same overhead as UnreliableSequenced.
    ReliableOrdered,

    // This message is reliable and will arrive in the sequence you sent it. 
    // Out or order messages will be dropped. Same overhead as UnreliableSequenced.
    ReliableSequenced,
}

impl Reliability {
    pub fn from(val: u8) -> Result<Self> {
        match val {
            0 => Ok(Reliability::Unreliable),
            1 => Ok(Reliability::UnreliableSequenced),
            2 => Ok(Reliability::Reliable),
            3 => Ok(Reliability::ReliableOrdered),
            4 => Ok(Reliability::ReliableSequenced),
            _ => Err(RakNetError::FrameError)
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Unreliable => 0,
            Self::UnreliableSequenced => 1,
            Self::Reliable => 2,
            Self::ReliableOrdered => 3,
            Self::ReliableSequenced => 4,
        }
    }
}

struct AckSet(Vec<(u32, u32)>);

impl AckSet {
    pub fn new() -> Self {
        AckSet(vec![])
    }

    pub fn insert(&mut self, num: u32) {
        trace!("Queue ACK: {}", num);

        for ack in self.0.iter_mut() {
            if num >= ack.0 && num <= ack.1 { return; }

            if ack.0 != 0 && num == ack.0 - 1 {
                ack.0 = num;
                return;
            }

            if num == ack.1 + 1 {
                ack.1 = num;
                return;
            }
        }

        self.0.push((num, num));
    }

    pub fn get_ack(&mut self) -> Vec<(u32, u32)> {
        self.0.drain(..).collect()
    }
}

pub struct RecvQ {
    sequenced_frame_index: u32,
    last_ordered_index: u32,
    sequence_number_ackset: AckSet,
    packets: HashMap<u32, MessageFrame>,
    ordered_packets: HashMap<u32, MessageFrame>,
    fragment_queue: FragmentQ,
}

impl RecvQ {
    pub fn new() -> Self {
        Self {
            sequenced_frame_index: 0,
            last_ordered_index: 0,
            sequence_number_ackset: AckSet::new(),
            packets: HashMap::new(),
            ordered_packets: HashMap::new(),
            fragment_queue: FragmentQ::new(),
        }
    }

    pub fn insert(&mut self, frame: MessageFrame) -> Result<()> {
        if self.packets.contains_key(&frame.message_number()) { return Ok(()); }

        trace!("Got new message: {:?}", frame);

        self.sequence_number_ackset.insert(frame.message_number());
        
        match frame.reliability() {
            Reliability::Unreliable => {
                self.packets.entry(frame.message_number()).or_insert(frame);
            },
            Reliability::UnreliableSequenced => {
                let message_number = frame.message_number();
                if message_number >= self.sequenced_frame_index {
                    if let Entry::Vacant(e) = self.packets.entry(frame.message_number()) {
                        e.insert(frame);
                        self.sequenced_frame_index = message_number + 1;
                    }
                }
            },
            Reliability::Reliable => {
                self.packets.insert(frame.message_number(), frame);
            },
            Reliability::ReliableOrdered => {
                if frame.message_number() < self.last_ordered_index {
                    return Ok(());
                }

                if frame.split().is_some() {
                    self.fragment_queue.insert(frame);

                    for i in self.fragment_queue.flush()? {
                        if let Some(order) = i.order() {
                            self.ordered_packets
                                .entry(order.index)
                                .or_insert(i);
                        }
                    }
                } else if let Some(order) = frame.order() {
                    self.ordered_packets
                        .entry(order.index)
                        .or_insert(frame);
                }
            },
            Reliability::ReliableSequenced => {
                let message_number = frame.message_number();
                if message_number >= self.sequenced_frame_index {
                    if let Entry::Vacant(e) = self.packets.entry(frame.message_number()) {
                        e.insert(frame);
                        self.sequenced_frame_index = message_number + 1;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn get_ack(&mut self) -> Vec<RangeInclusive<u32>> {
        self.sequence_number_ackset.get_ack()
            .iter()
            .map(|(from,to)| RangeInclusive::new(*from, *to))
            .collect()
    }

    pub fn flush(&mut self) -> Vec<MessageFrame> {
        let mut ret = vec![];
        let mut ordered_keys: Vec<u32> = self.ordered_packets.keys().cloned().collect();

        ordered_keys.sort_unstable();

        for i in ordered_keys {
            if i == self.last_ordered_index {
                let frame = self.ordered_packets.remove(&i).unwrap();
                ret.push(frame);

                self.last_ordered_index = i + 1;
            }
        }

        let mut packets_keys: Vec<u32> = self.packets.keys().cloned().collect();
        packets_keys.sort_unstable();

        for i in packets_keys {
            let v = self.packets.remove(&i).unwrap();
            ret.push(v);
        }
        
        ret
    }
}

pub struct SendQ {
    mtu: u16,
    ack_sequence_number: u32,
    message_number: u32,
    sequenced_frame_index: u32,
    ordered_frame_index: u32,
    compund_id: u16,
    packets: Vec<MessageFrame>,
    rto: Duration,
    srtt: Duration,
    sent_packet: Vec<(MessageFrame, bool, Duration, u32, Vec<u32>)>,
    send_notify: Arc<Notify>,
}

impl SendQ {
    pub const DEFAULT_TIMEOUT_MILLIS: u64 = 50;

    const RTO_UBOUND: u64 = 12000;
    const RTO_LBOUND: u64 = 50;

    pub fn new(mtu: u16, send_notify: Arc<Notify>) -> Self {
        Self {
            mtu,
            ack_sequence_number: 0,
            message_number: 0,
            packets: vec![],
            sent_packet: vec![],
            sequenced_frame_index: 0,
            ordered_frame_index: 0,
            compund_id: 0,
            send_notify,

            rto: Duration::from_millis(Self::DEFAULT_TIMEOUT_MILLIS),
            srtt: Duration::from_millis(Self::DEFAULT_TIMEOUT_MILLIS),
        }
    }

    pub fn insert(&mut self, reliability: Reliability, buf: Vec<u8>) -> Result<()> {
        match reliability {
            Reliability::Unreliable => {
                if buf.len() > (self.mtu - 5) as usize {
                    return Err(RakNetError::PacketSizeExceedsMTU);
                }

                let mut frame = MessageFrame::new(reliability, buf);
                frame.set_message_number(self.message_number);

                self.message_number += 1;
                self.packets.push(frame);
            },
            Reliability::UnreliableSequenced => {
                if buf.len() > (self.mtu - 7) as usize {
                    return Err(RakNetError::PacketSizeExceedsMTU);
                }

                let mut frame = MessageFrame::new(reliability, buf);
                frame.set_message_number(self.message_number);

                frame.set_order(Order {
                    channel: 0,
                    index: self.ordered_frame_index,
                });

                self.packets.push(frame);
                self.message_number += 1;                
            },
            Reliability::Reliable => {
                if buf.len() > (self.mtu - 5) as usize {
                    return Err(RakNetError::PacketSizeExceedsMTU);
                }

                let mut frame = MessageFrame::new(reliability, buf);
                frame.set_message_number(self.message_number);

                self.packets.push(frame);
                self.message_number += 1;
            },
            Reliability::ReliableOrdered => {
                let max = (self.mtu - 7) as usize;

                if buf.len() < max {
                   let mut frame = MessageFrame::new(reliability, buf);
                   frame.set_message_number(self.message_number);
                   frame.set_order(Order {
                    channel: 0,
                    index: self.ordered_frame_index
                   });

                   self.packets.push(frame);
                   self.ordered_frame_index += 1;
                   self.message_number += 1;
                } else {
                    let mut split_packets = buf.len() / max;
                    if buf.len() % max != 0 {
                        split_packets += 1;
                    }

                    for i in 0..split_packets {
                        let begin = max * i;
                        let end = if i == split_packets - 1 {
                            buf.len()
                        } else {
                            max * (i + 1)
                        };

                        let mut frame = MessageFrame::new(reliability, buf[begin..end].to_vec());
                        frame.set_message_number(self.message_number);

                        frame.set_split(Split {
                            id: self.compund_id,
                            count: split_packets as u32,
                            index: i as u32,
                        });
                        frame.set_order(Order {
                            channel: 0,
                            index: self.ordered_frame_index,
                        });

                        self.packets.push(frame);
                        self.message_number += 1;
                    }

                    self.compund_id += 1;
                    self.ordered_frame_index += 1;
                }
            },
            Reliability::ReliableSequenced => {
                if buf.len() > (self.mtu - 7) as usize {
                    return Err(RakNetError::PacketSizeExceedsMTU);
                }

                let mut frame = MessageFrame::new(reliability, buf);
                frame.set_order(Order {
                    channel: 0,
                    index: self.ordered_frame_index
                });

                self.packets.push(frame);
                self.sequenced_frame_index += 1;
            }
        }

        self.send_notify.notify_one();

        Ok(())
    }

    fn update_rto(&mut self, rtt: Duration) {
        // SRTT = ( ALPHA * SRTT ) + ((1-ALPHA) * RTT)
        // ALPHA = 0.8
        self.srtt = Duration::from_millis(((self.srtt.as_millis_f64() * 0.8) + (rtt.as_millis_f64() * 0.2)) as u64);
        // RTO = min[UBOUND,max[LBOUND,(BETA*SRTT)]]
        // BETA = 1.5
        let rto_right = (1.5 * self.srtt.as_millis_f64()) as u64;
        let rto_right = if rto_right > Self::RTO_LBOUND {
            rto_right
        } else {
            Self::RTO_LBOUND
        };
        self.rto = if rto_right < Self::RTO_UBOUND {
            Duration::from_millis(rto_right)
        } else {
            Duration::from_millis(Self::RTO_UBOUND)
        };
    }

    pub fn ack(&mut self, sequence: u32, tick: Duration) {
        self.ack_sequence_number = sequence;

        let mut rtts = vec![];

        for i in 0..self.sent_packet.len() {
            let item = &mut self.sent_packet[i];
            if item.0.message_number() == sequence || item.4.contains(&sequence) {
                rtts.push(tick - item.2);
                self.sent_packet.remove(i);
                break;
            }
        }

        for i in rtts {
            self.update_rto(i);
        }
    }

    pub fn tick(&mut self, tick: Duration) {
        for i in 0..self.sent_packet.len() {
            let p = &mut self.sent_packet[i];

            let mut cur_rto = self.rto;

            // TCP timeout calculation is RTOx2, so three consecutive packet losses will make it RTOx8, which is very terrible,
            // while this implementation it is not x2, but x1.5 (Experimental results show that the value of 1.5 is relatively good), which has improved the transmission speed.
            for _ in 0..p.3 {
                cur_rto = Duration::from_millis((cur_rto.as_millis_f64() * 1.5) as u64);
            }

            if p.1 && tick - p.2 >= cur_rto {
                p.1 = false;
            }
        }
    }

    pub fn flush(&mut self, tick: Duration, peer_addr: &SocketAddr) -> Vec<MessageFrame> {
        self.tick(tick);

        let mut ret = vec![];

        if !self.sent_packet.is_empty() {
            self.sent_packet
                .sort_by(|x, y| x.0.message_number().cmp(&y.0.message_number()));

            for i in 0..self.sent_packet.len() {
                let p = &mut self.sent_packet[i];
                if !p.1 {
                    debug!(
                        "{}, packet {} resend {} times",
                        peer_addr,
                        p.0.message_number(),
                        p.3 + 1
                    );

                    ret.push(p.0.clone());
                    p.1 = true;
                    p.2 = tick;
                    p.3 += 1;
                }
            }

            return ret;
        }

        if !self.packets.is_empty() {
            for frame in self.packets.drain(..) {
                if 
                    matches!(frame.reliability(), Reliability::Reliable) ||
                    matches!(frame.reliability(), Reliability::ReliableOrdered) ||
                    matches!(frame.reliability(), Reliability::ReliableSequenced) {
                    
                    self.sent_packet.push((
                        frame.clone(),
                        true,
                        tick,
                        0,
                        vec![]
                    ));
                }
                
                ret.push(frame);
            }
        }

        ret
    }
}