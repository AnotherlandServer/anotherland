// Copyright (C) 2024 AnotherlandServer
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

use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}, str::FromStr};

use async_graphql::{connection::CursorType, InputType, InputValueResult, OutputType, ScalarType, Value};
use serde::{Deserialize, Serialize};
use uuid::Bytes;

pub use bson::uuid::Error as Error;

// The ultimate Uuid type. Acts like bson::Uuid for mongodb serialzation but also
// integrates with async_graphql like uuid::Uuid.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Uuid(bson::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(bson::uuid::Uuid::new())
    }

    pub const fn from_bytes(bytes: Bytes) -> Self {
        Self(bson::Uuid::from_bytes(bytes))
    }

    pub fn from_bytes_le(b: Bytes) -> Self {
        Self(uuid::Uuid::from_bytes_le(b).into())
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl OutputType for Uuid {
    fn type_name() -> std::borrow::Cow<'static, str> {
        <uuid::Uuid as OutputType>::type_name()
    }

    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        <uuid::Uuid as OutputType>::create_type_info(registry)
    }

    fn resolve(
        &self,
        ctx: &async_graphql::context::ContextSelectionSet<'_>,
        field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> impl std::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>> + Send {
        let uuid_1 = self.0.to_uuid_1();

        async move {
            <uuid::Uuid as OutputType>::resolve(&uuid_1, ctx, field).await
        }
    }
}

impl InputType for Uuid {
    type RawValueType = Self;

    fn type_name() -> std::borrow::Cow<'static, str> {
        <uuid::Uuid as InputType>::type_name()
    }

    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        <uuid::Uuid as InputType>::create_type_info(registry)
    }

    fn parse(value: Option<async_graphql::Value>) -> async_graphql::InputValueResult<Self> {
        <uuid::Uuid as InputType>::parse(value)
            .map(|val| Self(bson::Uuid::from_uuid_1(val)))
            .map_err(|e| e.propagate() )
    }

    fn to_value(&self) -> async_graphql::Value {
        InputType::to_value(&self.0.to_uuid_1())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
}

impl ScalarType for Uuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        <uuid::Uuid as ScalarType>::parse(value)
            .map(|val| val.into())
            .map_err(|e| e.propagate())
    }

    fn to_value(&self) -> Value {
        ScalarType::to_value(&self.0.to_uuid_1())
    }
}

impl From<Uuid> for bson::Uuid {
    fn from(value: Uuid) -> Self {
        value.0
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        value.0.into()
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.into())
    }
}

impl From<bson::Uuid> for Uuid {
    fn from(value: bson::Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid> for bson::Bson {
    fn from(value: Uuid) -> Self {
        value.0.into()
    }
}

impl Deref for Uuid {
    type Target = bson::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Uuid {
    type Err = bson::uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(bson::uuid::Uuid::parse_str(s)?))
    }
}

impl CursorType for Uuid {
    type Error = bson::uuid::Error;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }

    fn encode_cursor(&self) -> String {
        self.0.to_string()
    }
}

pub static UUID_NIL: Uuid = Uuid(bson::uuid::Uuid::from_bytes([0; 16]));
pub static UUID_MAX: Uuid = Uuid(bson::uuid::Uuid::from_bytes([0xFF; 16]));
