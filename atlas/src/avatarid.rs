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

use std::fmt::Debug;

use serde::{Serialize, Deserialize, de};
use serde::ser::Serializer;
use serde::de::{Deserializer, Visitor};

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AvatarId(u64);

impl AvatarId {
    pub fn new(val: u64) -> Self { Self(val) }
    pub fn as_u64(&self) -> u64 { self.0 }
}

impl Debug for AvatarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:016x}", self.0)
        //f.debug_tuple("AvatarId").field(&format!("#{:016x}", self.0)).finish()
    }
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