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

use async_trait::async_trait;
use nom::{bytes::complete::take, error::Error, number::complete::le_i32};

use crate::{Container, DeserializeUnrealObject, LocalObjectIndexRef, ObjectRef, UPKResult};

#[derive(Debug)]
pub struct StructProperty {
    class: ObjectRef
}

impl StructProperty {
    pub fn class(&self) -> ObjectRef {
        self.class.clone()
    }
}

#[async_trait]
impl DeserializeUnrealObject for StructProperty {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (i, _) = take::<usize, _, Error<_>>(i.len() - 4)(i)?;
        let (i, struct_ref) = le_i32::<_, Error<_>>(i)?;

        let class = container.resolve_object(object.package().unwrap(), LocalObjectIndexRef::from_idx(struct_ref)).unwrap();

        Ok((
            i,
                Self {
                class
            }
        ))
    }
}