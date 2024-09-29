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
use std::{collections::HashMap, hash::Hash};

use crate::{error::{RakNetError, Result}, fragment::FragmentQ, frame::MessageFrame};

#[derive(Debug)]
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

    pub fn to_u8(&self) -> u8 {
        match *self {
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
        self.0.drain([..]).collect()
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
            sequence_number_ackset: vec![],
            packets: HashMap::new(),
            ordered_packets: HashMap::new(),
            fragment_queue: FragmentQ::new(),
        }
    }

    pub fn insert(&mut self, frame: MessageFrame) -> Result<()> {
        if self.packets.contains_key(&frame.message_number) { return Ok(()); }

        self.sequence_number_ackset.insert(frame.message_number);
        todo!()
    }
}