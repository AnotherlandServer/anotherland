use serde::{Serialize, Deserialize, de};
use serde::ser::Serializer;
use serde::de::{Deserializer, Visitor, SeqAccess};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvatarId(u64);

impl AvatarId {
    pub fn new(val: u64) -> Self { Self(val) }
    pub fn as_u64(&self) -> u64 { self.0 }
}

impl Serialize for AvatarId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        
        serializer.serialize_str(format!("{:016x}", self.0).as_str())
    }
}

struct AvatarIdVisitor;

impl<'de> Visitor<'de> for AvatarIdVisitor {
    type Value = AvatarId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an avatarid in hex format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        
        Ok(AvatarId(u64::from_str_radix(v, 16).map_err(|e| E::custom(e.to_string()))?))
    }
}

impl <'dr>Deserialize<'dr> for AvatarId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'dr> {
        deserializer.deserialize_str(AvatarIdVisitor)
    }
}

impl Into<u64> for AvatarId {
    fn into(self) -> u64 {
        self.0
    }
}

impl <'a>Into<&'a u64> for &'a AvatarId {
    fn into(self) -> &'a u64 {
        &self.0
    }
}

impl From<u64> for AvatarId {
    fn from(value: u64) -> Self {
        AvatarId(value)
    }
}