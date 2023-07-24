use std::time::Duration;

use bitstream_io::{BitWriter, BigEndian, BitWrite};

use super::{AckRange, Reliability, PacketSplit, MessageNumber, Packet, peer::RakNetPeer, Message};

pub struct RakNetResponse {
    acks: (Duration, Vec<MessageNumber>),
    time: Option<Duration>,
    reliability: Option<Reliability>, 
    split: Option<PacketSplit>,
    packets: Vec<Packet>,
}

impl RakNetResponse {
    pub fn new(remote_time: Duration) -> Self {
        Self {
            acks: (remote_time, Vec::new()),
            time: None,
            reliability: None,
            split: None,
            packets: Vec::new(),
        }
    }

    pub fn add_ack(&mut self, message_number: MessageNumber) {
        self.acks.1.push(message_number);
    }

    pub fn add_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }

    pub fn pack_response(&self, peer: &mut RakNetPeer) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = BitWriter::endian(&mut buf, BigEndian);

        if self.acks.1.is_empty() && self.time.is_none() && self.packets.is_empty() {
            return Vec::new();
        }

        if self.acks.1.is_empty() {
            let _ = writer.write_bit(false);
        } else {
            let mut ack_ranges = Vec::<(u32, u32)>::new();
            let _ = writer.write_bit(true);
            
            let mut id_min = *self.acks.1.first().unwrap();
            let mut id_max = id_min;
            for &id in &self.acks.1[1..] {
                if id - id_max > 1 {
                    ack_ranges.push((id_min, id_max));
                    id_min = id;
                    id_max = id;
                } else {
                    id_max = id;
                }
            }
            ack_ranges.push((id_min, id_max));

            let _ = Packet::Ack(self.acks.0, ack_ranges).serialize_to_bitwriter(&mut writer);
        }

        if let Some(time) = self.time {
            let _ = writer.write_bit(true);
            let _ = Packet::SystemTime(time).serialize_to_bitwriter(&mut writer);
        } else {
            let _ = writer.write_bit(false);
        }

        for packet in &self.packets {
            let _ = packet.serialize_to_bitwriter(&mut writer);
        }

        /*if let Some(packet) = &self.packets {
            
            let _ = Packet::RawMessage { 
                number: peer.generate_next_message_id(), 
                reliability: self.reliability.unwrap_or(Reliability::Unreliable), 
                split: self.split.unwrap_or(PacketSplit::NotSplit), 
                data: data.clone(),
            }.serialize_to_bitwriter(&mut writer);
        }*/

        buf
    }
}