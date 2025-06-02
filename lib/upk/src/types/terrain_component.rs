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
use glam::Vec3;

use crate::{Container, DeserializeUnrealObject, ObjectRef, UPKResult};

use super::{parse_array, parse_vec3};

#[derive(Debug, Clone)]
pub struct TerrainComponent {
    pub collision_vertices: Vec<Vec3>,
}

#[async_trait]
impl DeserializeUnrealObject for TerrainComponent {
    async fn deserialize<'a>(_object: &ObjectRef, _container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (i, collision_vertices) = parse_array(parse_vec3)(i)?;

        Ok((i, Self {
            collision_vertices
        }))
    }
}