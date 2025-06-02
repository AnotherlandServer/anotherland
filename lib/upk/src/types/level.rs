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

use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use nom::{bytes::complete::take, combinator::{map, map_res}, error::VerboseError, multi::count, number::complete::{le_i32, le_u16}, IResult};

use crate::{types::parse_array, Container, DeserializeUnrealObject, ObjectRef, PackageFile, UPKError, UPKResult};

use super::{parse_object, ObjectProperty};

#[derive(Debug, Clone)]
pub struct Level {
    pub actors: Vec<ObjectRef>,
    pub url: String,
    pub model: ObjectRef,
    pub model_components: Vec<ObjectRef>,
    pub gamesequences: Vec<ObjectRef>,
}

#[async_trait]
impl DeserializeUnrealObject for Level {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let file = object.package().unwrap();

        let (i, (_owner, actors)) = parse_trans_array(file.clone(), container, |i| map(
            parse_object(file.clone(), container),
            |prop| {
                if let ObjectProperty::Object(obj) = prop {
                    Some(obj)
                } else {
                    None
                }
            }
        )(i))(i)?;

        debug!("Actors: {actors:?}");

        let (i, url) = parse_url(i)?;

        debug!("URL: {url}");

        let (i, model) = parse_object(file.clone(), container)(i)?;
        let model = if let ObjectProperty::Object(model) = model {
            model
        } else {
            return Err(UPKError::Custom("Expected Model".into()));
        };

        let (i, model_components) = parse_array(|i| map_res(
            parse_object(file.clone(), container),
            |prop| {
                if let ObjectProperty::Object(obj) = prop {
                    Ok(obj)
                } else {
                    Err(UPKError::Custom("Expected ModelComponent".into()))
                }
            }
        )(i))(i)?;

        let (i, gamesequences) = parse_array(|i| map(
            parse_object(file.clone(), container),
            |prop| {
                if let ObjectProperty::Object(obj) = prop {
                    Some(obj)
                } else {
                    None
                }
            }
        )(i))(i)?;

        Ok((i, Level {
            actors: actors.into_iter().flatten().collect(),
            url,
            model,
            model_components,
            gamesequences: gamesequences.into_iter().flatten().collect(),
        }))
    }
}

#[allow(clippy::type_complexity)]
pub fn parse_trans_array<T, F>(file: Arc<PackageFile>, container: &Container, f: F)
    -> impl FnMut(&'_ [u8])
    -> IResult<&'_ [u8], (ObjectRef, Vec<T>), VerboseError<&'_ [u8]>>
where
    F: FnMut(&[u8]) -> IResult<&[u8], T, VerboseError<&[u8]>> + Copy,
{
    move |i: &[u8]| {
        let (i, owner) = map_res(
            parse_object(file.clone(), container),
            |prop| {
                if let ObjectProperty::Object(obj) = prop {
                    Ok(obj)
                } else {
                    Err(UPKError::Custom("Expected Owner".to_string()))
                }
            }
        )(i)?;

        debug!("Owner: {owner:?}");

        let (i, array) = parse_array(f)(i)?;

        Ok((i, (owner, array)))
    }
}

fn parse_string(i: &[u8]) -> IResult<&[u8], String, VerboseError<&[u8]>> {
    let (i, len) = le_i32(i)?;

    let (len, is_unicode) = if len >= 0 {
        (len as usize, false)
    } else {
        (-len as usize, true)
    };

    let (i, string) = if len > 0 {
        if is_unicode {
            let (i, str_bytes) = count(le_u16, len )(i)?;
            (i, String::from_utf16_lossy(&str_bytes[..len-1]))
        } else {
            let (i, str_bytes) = take(len)(i)?;
            (i, String::from_utf8_lossy(&str_bytes[..len-1]).to_string())
        }
    } else {
        (i, String::new())
    };

    Ok((i, string))
}

pub fn parse_url(i: &[u8]) -> IResult<&[u8], String, VerboseError<&[u8]>> {
    let (i, protocol) = parse_string(i)?;
    let (i, host) = parse_string(i)?;
    let (i, map) = parse_string(i)?;
    let (i, portal) = parse_string(i)?;
    let (i, op) = parse_array(parse_string)(i)?;
    let (i, port) = le_i32(i)?;
    let (i, _valid) = le_i32(i)?;

    Ok((i, format!(
        "{protocol}://{host}:{port}/{map}{}{}",
        op.iter().map(|s| format!("#{s}")).collect::<String>(),
        if portal.is_empty() {
            String::new()
        } else {
            format!("#{portal}")
        }
    )))
}
