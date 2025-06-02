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
use nom::number::complete::le_u16;

use crate::{types::parse_array, Container, DeserializeUnrealObject, ObjectRef, UPKResult};

#[derive(Debug, Clone)]
pub struct Terrain {
    pub heights: Vec<u16>,
    pub info_data: Vec<u16>,
}

#[async_trait]
impl DeserializeUnrealObject for Terrain {
    async fn deserialize<'a>(_object: &ObjectRef, _container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (i, heights) = parse_array(|i| le_u16(i))(i)?;
        let (i, info_data) = parse_array(|i| le_u16(i))(i)?;

        Ok((i, Self {
            heights,
            info_data,
        }))
    }
}