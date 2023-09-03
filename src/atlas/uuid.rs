use std::{io, fmt};
use std::hash::{Hash, Hasher};

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

impl Hash for Uuid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time_hi_and_version.hash(state);
        self.time_mid.hash(state);
        self.time_low.hash(state);
        self.clock_seq_high_and_reserved.hash(state);
        self.clock_seq_low.hash(state);
        self.node.hash(state);
    }
}

impl PartialEq for Uuid {
    fn eq(&self, other: &Self) -> bool {
        self.time_hi_and_version == other.time_hi_and_version &&
        self.time_mid == other.time_mid &&
        self.time_low == other.time_low &&
        self.clock_seq_high_and_reserved == other.clock_seq_high_and_reserved &&
        self.clock_seq_low == other.clock_seq_low &&
        self.node == other.node
    }
}

impl Eq for Uuid {}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_bytes();
        let euuid = external_uuid::from_bytes_le(bytes.as_slice().try_into().unwrap());

        write!(f, "{}", euuid.to_string())
    }
}