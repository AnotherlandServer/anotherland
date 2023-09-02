use std::io;

use super::generated::Uuid;
use uuid::Uuid as external_uuid;

impl Uuid {
    pub fn from_str(val: &str) -> Result<Self, io::Error> {
        let uuid = external_uuid::parse_str(val).unwrap();
        let (time_low, time_mid, time_hi_and_version, tail) = uuid.to_fields_le();
        Ok(Self {
            time_low: time_low.swap_bytes(),
            time_mid: time_mid.swap_bytes(),
            time_hi_and_version: time_hi_and_version.swap_bytes(),
            clock_seq_high_and_reserved: tail[0],
            clock_seq_low: tail[1],
            node: tail[2..].to_vec(),
        })
    }

    pub fn new_v4() -> Self {
        let uuid = external_uuid::new_v4();
        let (time_low, time_mid, time_hi_and_version, tail) = uuid.to_fields_le();
        Self {
            time_low: time_low.swap_bytes(),
            time_mid: time_mid.swap_bytes(),
            time_hi_and_version: time_hi_and_version.swap_bytes(),
            clock_seq_high_and_reserved: tail[0],
            clock_seq_low: tail[1],
            node: tail[2..].to_vec(),
        }
    }
}