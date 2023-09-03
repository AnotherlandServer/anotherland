use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};

use super::RakNetErrorKind;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PeerAddress {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl PeerAddress {
    pub fn new(ip: &Ipv4Addr, port: u16) -> PeerAddress {
        PeerAddress { ip: ip.clone(), port }
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(self.ip, self.port))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        writer.write::<u32>(self.ip.into()).expect("Failed to write ip");
        writer.write(self.port).expect("Failed to write port");

        buf
    }
}

impl TryFrom<SocketAddr> for PeerAddress {
    type Error = super::RakNetError;

    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        match value {
            SocketAddr::V4(addr) => {
                Ok(PeerAddress { ip: addr.ip().to_owned(), port: addr.port() })
            },
            _ => Err(Self::Error::from_kind(RakNetErrorKind::InvalidAddressFormat)),
        }
    }
}