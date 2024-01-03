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

use std::{error::Error, str::FromStr};

use serde::{Serializer, de::{Visitor, self, SeqAccess}, Deserializer, Serialize, Deserialize, ser::SerializeSeq};
use serde_json::Value;
use uuid::Uuid;

pub fn serialize_string<S>(
    string: &String, 
    _option: &Option<u8>, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(string)
}
struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = (String, Option<u8>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok((v.to_owned(), None))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error, {

        Ok((v, None))
    }
}

pub fn deserialize_string<'de, D>(deserializer: D) -> Result<(String, Option<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_string(StringVisitor)
}

pub fn serialize_json<S>(
    value: &Value, 
    _option: &Option<String>, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

pub fn deserialize_json<'de, D>(deserializer: D) -> Result<(Value, Option<String>), D::Error>
where
    D: Deserializer<'de>,
{
    Ok((Value::deserialize(deserializer)?, None))
}

pub fn serialize_vec_uuid<S>(
    values: &Vec<Uuid>, 
    _option: &Option<u8>, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut serializer = serializer.serialize_seq(Some(values.len()))?;
    for v in values {
        serializer.serialize_element(&v.to_string())?;
    }

    serializer.end()
}

struct UuidVecVisitor;

impl<'de> Visitor<'de> for UuidVecVisitor {
    type Value = (Vec<Uuid>, Option<u8>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>, {
        
        let mut values = Vec::new();

        while let Some(value) = seq.next_element::<String>()? {
            values.push(Uuid::from_str(value.as_str()).unwrap());
        }

        Ok((values, None))
    }
}

pub fn deserialize_vec_uuid<'de, D>(deserializer: D) -> Result<(Vec<Uuid>, Option<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(UuidVecVisitor)
}

pub fn serialize_i32<S>(
    value: &i32, 
    _option: &Option<u8>, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(*value)
}

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = (i32, Option<u8>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a number")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error, {
        
        Ok((v, None))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok((v as i32, None))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok((v as i32, None))
    }
}

pub fn deserialize_i32<'de, D>(deserializer: D) -> Result<(i32, Option<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i32(I32Visitor)
}