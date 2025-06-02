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
use glam::{Mat4, Vec4};
use nom::{error::VerboseError, multi::count};

use crate::{Container, DeserializeUnrealObject, ObjectRef, UPKResult};

#[derive(Debug, Clone)]
pub struct StaticMeshCollectionActor {
    pub parent_to_world: Vec<Mat4>,
}

#[async_trait]
impl DeserializeUnrealObject for StaticMeshCollectionActor {
    async fn deserialize<'a>(_object: &ObjectRef, _container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        // Unusually, the matrices for this object are not serialized as an array. 
        // Instead, the amount of entries to be read is determined by another property, we don't 
        // have access to here. 
        // So we will read until we reach the end of the object data.
        let mut parent_to_world = Vec::new();
        let mut remaining = i;

        while !remaining.is_empty() {
            let (next, matrix) = count(parse_vec4, 4)(remaining)?;

            parent_to_world.push(Mat4 { 
                x_axis: matrix[0], 
                y_axis: matrix[1], 
                z_axis: matrix[2], 
                w_axis: matrix[3], 
            });
            
            remaining = next;
        }

        Ok((i, Self {
            parent_to_world
        }))
    }
}

pub fn parse_vec4(i: &[u8]) -> nom::IResult<&[u8], Vec4, VerboseError<&[u8]>> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::number::complete::le_f32,
            nom::number::complete::le_f32,
            nom::number::complete::le_f32,
            nom::number::complete::le_f32,
        )),
        |(x, y, z, w)| Vec4::new(x, y, z, w),
    )(i)
}
