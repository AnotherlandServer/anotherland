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
use std::time::Duration;

use log::debug;

use crate::{buffer::RakNetReader, error::{RakNetError, Result}, reliability::Reliability, MAX_MTU_SIZE};

#[derive(Debug)]
pub struct MessageFrame {
    pub acks: Option<Vec<RangeInclusive<u32>>>,
    pub local_system_time: Option<Duration>,
    pub remote_system_time: Option<Duration>,
    pub message_number: u32,
    pub reliability: Reliability,
    pub order: Option<Order>,
    pub split: Option<Split>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Order {
    pub channel: u8,
    pub index: u32,
}

#[derive(Debug)]
pub struct Split {
    pub id: u16,
    pub index: u32,
    pub count: u32,
}

impl MessageFrame {
    pub fn new(message_number: u32, data: Vec<u8>) -> Self {
        Self {
            acks: None,
            local_system_time: None,
            remote_system_time: None,
            message_number,
            reliability: Reliability::Unreliable,
            order: None,
            split: None,
            data
        }
    }

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
        let remote_system_time = if data.read_bit()? {
            Some(Duration::from_millis(data.read_u32()? as u64))
        } else {
            None
        };

        // Read message number
        let message_number = data.read_u32()?;

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

        Ok(Self {
            acks,
            local_system_time,
            remote_system_time,
            message_number,
            reliability,
            order,
            split,
            data: pkg_data
        })
    }
}