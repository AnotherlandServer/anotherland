// Copyright (C) 2025 AnotherlandServer
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
use std::time::Duration;

use log::debug;

use crate::{buffer::{RakNetReader, RakNetWriter}, error::{RakNetError, Result}, reliability::Reliability, MAX_MTU_SIZE};

#[derive(Debug, Clone)]
pub struct MessageFrame {
    acks: Option<Vec<RangeInclusive<u32>>>,
    rtt_reference: Option<Duration>,
    reference_system_time: Option<Duration>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    message_number: u32,
    reliability: Reliability,
    order: Option<Order>,
    split: Option<Split>,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub channel: u8,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct Split {
    pub id: u16,
    pub index: u32,
    pub count: u32,
}

#[allow(dead_code)]
impl MessageFrame {
    pub fn new() -> Self {
        Self {
            acks: None,
            rtt_reference: None,
            reference_system_time: None,
            messages: vec![],
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[Message] { &self.messages }

    pub fn into_message_vector(self) -> Vec<Message> { self.messages }

    pub fn from(buf: &[u8]) -> Result<Self> {
        let local_system_time;
        let acks;

        let mut data = RakNetReader::new(buf);

        // Read acks
        if data.read_bit()? {
            local_system_time = Some(Duration::from_millis(data.read_u32()? as u64));
            let entries = data.read_u16_compressed()?;
            let mut acks_read = Vec::new();

            for _ in 0..entries {
                let max_equals_min = data.read_bit()?;
                if !max_equals_min {
                    acks_read.push(RangeInclusive::new(
                        data.read_u32()?, 
                        data.read_u32()?
                    ));
                } else {
                    let msg_id = data.read_u32()?;

                    acks_read.push(RangeInclusive::new(
                        msg_id, 
                        msg_id
                    ));
                };
            }

            acks = Some(acks_read);
        } else {
            local_system_time = None;
            acks = None;
        }

        // Read remote system time
        let remote_system_time = if 
            let Ok(has_system_time) = data.read_bit() &&
            has_system_time
        {
            Some(Duration::from_millis(data.read_u32()? as u64))
        } else {
            None
        };

        let mut frame = Self {
            acks,
            rtt_reference: local_system_time,
            reference_system_time: remote_system_time,
            messages: vec![],
        };

        while let Ok(message_number) = data.read_u32() {
            // Read reliability
            let mut reliability = [0u8; 1];
            data.read_bits(&mut reliability, 3)?;

            let reliability = Reliability::from(reliability[0])?;

            // Read order info
            let order = if 
                matches!(reliability, Reliability::UnreliableSequenced) ||
                matches!(reliability, Reliability::ReliableSequenced) ||
                matches!(reliability, Reliability::ReliableOrdered)
            {
                let mut channel = [0u8; 1];
                data.read_bits(&mut channel, 5)?;

                Some(Order {
                    channel: channel[0],
                    index: data.read_u32()?,
                })
            } else {
                None
            };

            // Read split info
            let split = if data.read_bit()? {
                Some(Split {
                    id: data.read_u16()?,
                    index: data.read_u32_compressed()?,
                    count: data.read_u32_compressed()?
                })
            } else {
                None
            };

            // Read length
            let data_bit_length = data.read_u16_compressed()? as usize;
            if data_bit_length / 8 > MAX_MTU_SIZE {
                debug!("Got length larger than MTU size: {}", data_bit_length / 8);
                return Err(RakNetError::FrameError);
            }

            data.byte_align();

            if data_bit_length > data.bits_remaining() {
                debug!("Length mismatch! Should read {} bits but got only {}.", data_bit_length, data.bits_remaining());
                return Err(RakNetError::FrameError);
            }

            // Read packet data
            let mut pkg_data = vec![0u8; data_bit_length / 8];
            data.read(&mut pkg_data)?;

            /*if data.bits_remaining() != 0 { 
                let mut remainder = vec![0u8; data.remaining()];
                data.read(&mut remainder).unwrap();
                
                debug!("{:?}", Self {
                    acks,
                    rtt_reference: local_system_time,
                    reference_system_time: remote_system_time,
                    message_number,
                    reliability,
                    order,
                    split,
                    data: pkg_data
                });

                debug!("{:?}", remainder);

                panic!("Data remains in frame"); 
            }*/

            frame.messages.push(Message {
                message_number,
                reliability,
                order,
                split,
                data: pkg_data
            });
        }

        Ok(frame)
    }

    pub fn acks(&self) -> Option<&[RangeInclusive<u32>]> {
        self.acks.as_deref()
    }

    pub fn local_system_time(&self) -> Option<Duration> {
        self.rtt_reference
    }

    pub fn remote_system_time(&self) -> Option<Duration> {
        self.reference_system_time
    }

    pub fn set_acks(&mut self, system_time: Duration, acks: Vec<RangeInclusive<u32>>) {
        self.acks = Some(acks);
        self.rtt_reference = Some(system_time);
    }

    pub fn set_remote_system_time(&mut self, system_time: Duration) {
        self.reference_system_time = Some(system_time);
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut writer = RakNetWriter::new();

        if let Some(acks) = self.acks.as_ref() {
            writer.write_bit(true);
            writer.write_u32(
                self.rtt_reference
                        .ok_or(RakNetError::FrameError)?
                        .as_millis() as u32
            );
            writer.write_u16_compressed(acks.len() as u16);

            for ack_range in acks {
                if *ack_range.start() == *ack_range.end() {
                    writer.write_bit(true);
                    writer.write_u32(*ack_range.start());
                } else {
                    writer.write_bit(false);
                    writer.write_u32(*ack_range.start());
                    writer.write_u32(*ack_range.end());
                }
            }
        } else {
            writer.write_bit(false);
        }

        if let Some(system_time) = self.reference_system_time {
            writer.write_bit(true);
            writer.write_u32(system_time.as_millis() as u32);
        } else {
            writer.write_bit(false);
        }

        for message in &self.messages {
            writer.write_u32(message.message_number);
            writer.write_bits(3, &[message.reliability.to_u8()]);

            if 
                matches!(message.reliability, Reliability::UnreliableSequenced) ||
                matches!(message.reliability, Reliability::ReliableSequenced) ||
                matches!(message.reliability, Reliability::ReliableOrdered)
            {
                if let Some(order) = message.order.as_ref() {
                    writer.write_bits(5, &[order.channel]);
                    writer.write_u32(order.index);
                } else {
                    return Err(RakNetError::FrameError);
                }
            }

            if let Some(split) = message.split.as_ref() {
                writer.write_bit(true);

                writer.write_u16(split.id);
                writer.write_u32_compressed(split.index);
                writer.write_u32_compressed(split.count);
            } else {
                writer.write_bit(false);
            }

            writer.write_u16_compressed((message.data.len() * 8) as u16);
            writer.write_aligned(&message.data);
        }

        Ok(writer.take_buffer())
    }
}

impl Message {
    pub fn new(message_number: u32, reliability: Reliability, data: Vec<u8>) -> Self {
        Self {
            message_number,
            reliability,
            order: None,
            split: None,
            data,
        }
    }

    pub fn message_number(&self) -> u32 {
        self.message_number
    }

    pub fn reliability(&self) -> Reliability {
        self.reliability
    }

    pub fn order(&self) -> Option<&Order> {
        self.order.as_ref()
    }

    pub fn split(&self) -> Option<&Split> {
        self.split.as_ref()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn set_message_number(&mut self, number: u32) {
        self.message_number = number;
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }

    pub fn set_split(&mut self, split: Split) {
        self.split = Some(split);
    }
}
