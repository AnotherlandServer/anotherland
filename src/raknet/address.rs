use std::net::Ipv4Addr;

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};

#[derive(Debug)]
pub struct PeerAddress {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl PeerAddress {
    pub fn new(ip: &Ipv4Addr, port: u16) -> PeerAddress {
        PeerAddress { ip: ip.clone(), port }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        writer.write::<u32>(self.ip.into()).expect("Failed to write ip");
        writer.write(self.port).expect("Failed to write port");

        buf
    }
}
