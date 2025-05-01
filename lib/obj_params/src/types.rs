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

use std::{fmt::{Display, Formatter}, ops::Deref, str::FromStr};
use anyhow::anyhow;
use toolkit::types::Uuid;

use crate::{Class, ParamError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    String,
    StringPair,
    StringFloatPair,
    StringSet,
    Guid,
    GuidPair,
    Bool,
    Int,
    BitField128,
    BitSetFilter,
    Float,
    FloatRange,
    Vector3,
    Vector3Uts,
    Vector4,
    LocalizedString,
    AvatarId,
    UniqueId,
    JsonValue,
    Int64,
    Quarternion,
    Positionable,
    ContentRef,
    ContentRefAndInt,
    ContentRefAndFloat,
    ContentRefList,
    ClassRefPowerRangeList,
    VectorInt,
    VectorInt64,
    VectorFloat,
    VectorString,
    AvatarIdSet,
    VectorAvatarId,
    GuidSet,
    VectorGuid,
    HashmapStringInt,
    HashmapStringString,
    Any,
    VectorLocalizedString,
    InstanceGroup,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentRef {
    pub class: Class,
    pub id: Uuid,
}

impl FromStr for ContentRef {
    type Err = ParamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ':');
        let class = parts.next().ok_or(ParamError::Other(anyhow!("Class id missing")))?;
        let id = parts.next().ok_or(ParamError::Other(anyhow!("Content id missing")))?;

        Ok(ContentRef {
            class: Class::from_id(
                class.parse::<u16>()
                .map_err(|_| ParamError::Other(anyhow!("Class id invalid")))?
            ).ok_or(ParamError::Other(anyhow!("Unknown class")))?,
            id: id.parse().map_err(|_| ParamError::Other(anyhow!("Content id invalid")))?,
        })
    }
}

impl Display for ContentRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.class.id(), self.id)
    }
}

impl From<ContentRef> for Uuid {
    fn from(content_ref: ContentRef) -> Self {
        content_ref.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ContentRefList(Vec<ContentRef>);

impl From<Vec<ContentRef>> for ContentRefList {
    fn from(content_refs: Vec<ContentRef>) -> Self {
        ContentRefList(content_refs)
    }
}

impl Deref for ContentRefList {
    type Target = Vec<ContentRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ContentRefList {
    type Err = ParamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content_refs = Vec::new();
        let parts = s.split(']').filter(|part| !part.is_empty());

        for part in parts {
            let trimmed_part = part.trim_start_matches('[');
            let content_ref = ContentRef::from_str(trimmed_part)?;
            content_refs.push(content_ref);
        }

        Ok(ContentRefList(content_refs))
    }
}

impl Display for ContentRefList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut content_refs = String::new();

        for content_ref in &self.0 {
            content_refs.push('[');
            content_refs.push_str(&content_ref.to_string());
            content_refs.push(']');
        }

        write!(f, "{content_refs}")
    }
}