use std::{io, fmt};
use std::hash::{Hash, Hasher};

use super::generated::Uuid;
use serde::de::{Visitor, Expected};
use serde::{Serialize, Deserialize};
use surrealdb::sql::Id;
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

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct UuidVisitor;

impl<'de> Visitor<'de> for UuidVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a uuid in a format like 00000000-0000-0000-0000-000000000000")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if v.len() == 36 {
            Ok(v.to_owned())
        } else {
            Err(E::custom(format!("Expected string with len 36: {}", v)))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {

        if v.len() == 36 {
            Ok(v)
        } else {
            Err(E::custom(format!("Expected string with len 36: {}", v)))
        } 
    }
}

impl <'de>Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let string_representation = deserializer.deserialize_str(UuidVisitor)?;
        Uuid::from_str(&string_representation).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Into<Id> for Uuid {
    fn into(self) -> Id {
        Id::String(self.to_string())
    }
}