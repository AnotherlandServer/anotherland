// Copyright (C) 2023 AnotherlandServer
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

use std::fmt::{Debug, Display};

use log::kv::{ToValue, Value};
use serde::{Serialize, Deserialize, de};
use serde::ser::Serializer;
use serde::de::{Deserializer, Visitor};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AvatarId((u64, AvatarType));

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AvatarType {
    None,
    Player,
    Npc,
    Other,
}

impl AvatarId {
    pub fn new(val: u64, avatar_type: AvatarType) -> Self { Self((val & !0xFF, avatar_type)) }

    pub fn as_u64(&self) -> u64 { (self.0.0 & !0xFF) | match self.0.1 {
        // Those prefixes need to be verified. 
        // I'm only sure that player is 1 and NPCs 2.
        AvatarType::None => 0x00,
        AvatarType::Player => 0x01,
        AvatarType::Npc => 0x02,
        AvatarType::Other => 0x03,
    } }

    pub fn avatar_type(&self) -> &AvatarType { &self.0.1 }

    pub fn is_none(&self) -> bool { self.0.1 == AvatarType::None }
    pub fn is_player(&self) -> bool { self.0.1 == AvatarType::Player }
    pub fn is_npc(&self) -> bool { self.0.1 == AvatarType::Npc }
    pub fn is_other(&self) -> bool { self.0.1 == AvatarType::Other }
}

impl Default for AvatarId {
    fn default() -> Self {
        Self((0, AvatarType::None))
    }
}

impl From<u64> for AvatarId {
    fn from(mut value: u64) -> Self {
        let avatar_type = match value & 0xFF {
            0x01 => AvatarType::Player,
            0x02 => AvatarType::Npc,
            0x03 => AvatarType::Other,
            _ => {
                value = 0;
                AvatarType::None
            },
        };

        AvatarId((value & !0xFF, avatar_type))
    }
}

impl Display for AvatarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:016x}", self.as_u64())
    }
}

impl Serialize for AvatarId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        
        serializer.serialize_str(format!("{:016x}", self.as_u64()).as_str())
    }
}

impl ToValue for AvatarId {
    fn to_value(&self) -> Value {
        Value::from_display(self)
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
        
        Ok(u64::from_str_radix(v, 16).map_err(|e| E::custom(e.to_string()))?.into())
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
        self.as_u64()
    }
}

