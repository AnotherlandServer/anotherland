use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use rand::prelude::*;

#[derive(Debug)]
pub struct Guid {
    pub g: [u32; 4],
}

impl Guid {
    pub fn create_random() -> Self {
        let mut rng = rand::thread_rng();

        Guid { g: [
            rng.next_u32(), 
            rng.next_u32(), 
            rng.next_u32(), 
            rng.next_u32()
        ] }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

        for g in self.g {
            writer.write(g);
        }

        buf
    }
}