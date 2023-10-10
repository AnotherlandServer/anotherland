use serde::{Serializer, de::{Visitor, self, SeqAccess}, Deserializer, Serialize, Deserialize, ser::SerializeSeq};
use serde_json::Value;

use crate::Uuid;


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
}

pub fn deserialize_string<'de, D>(deserializer: D) -> Result<(String, Option<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(StringVisitor)
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

        while let Some(value) = seq.next_element::<&str>()? {
            values.push(Uuid::from_str(value).unwrap());
        }

        Ok((values, None))
    }
}

pub fn deserialize_vec_uuid<'de, D>(deserializer: D) -> Result<(Vec<Uuid>, Option<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(UuidVecVisitor)
}