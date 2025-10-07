// Copyright (C) 2025 AnotherlandServer
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

use std::{fmt::{Display, Formatter}, ops::{Deref, DerefMut}};

use async_graphql::{connection::CursorType, InputType, ID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(mongodb::bson::oid::ObjectId);

impl Display for ObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let oid = mongodb::bson::oid::ObjectId::deserialize(deserializer)?;
        Ok(ObjectId(oid))
    }
}

impl Deref for ObjectId {
    type Target = mongodb::bson::oid::ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CursorType for ObjectId {
    type Error = bson::oid::Error;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        bson::oid::ObjectId::parse_str(s).map(ObjectId)
    }

    fn encode_cursor(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<ID> for ObjectId {
    type Error = bson::oid::Error;

    fn try_from(value: ID) -> Result<Self, Self::Error> {
        bson::oid::ObjectId::parse_str(&value.0).map(ObjectId)
    }
}
