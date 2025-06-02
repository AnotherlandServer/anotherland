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
use log::{debug, info};
use nom::{error::VerboseError, number::complete::le_f32, IResult};

use crate::{types::{parse_bulk_array, skip_bulk_array}, Container, DeserializeUnrealObject, ObjectRef, UPKResult};

use super::parse_vec3;

#[derive(Debug, Clone)]
pub struct Model {
    pub bounds: BoxSphereBounds,
}

#[async_trait]
impl DeserializeUnrealObject for Model {
    async fn deserialize<'a>(_object: &ObjectRef, _container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (i, bounds) = parse_bounds(i)?;
        debug!("Model bounds: {:?}", bounds);

        let (i, vectors) = parse_bulk_array(parse_vec3)(i)?;
        info!("Model vectors: {:?}", vectors);

        let (i, points) = parse_bulk_array(parse_vec3)(i)?;
        info!("Model points: {:?}", points);

        let (i, _) = skip_bulk_array(i)?; // Nodes

        Ok((i, Model {
            bounds,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct BoxSphereBounds {
    pub origin: Vec3,
    pub box_extent: Vec3,
    pub sphere_radius: f32,
}

fn parse_bounds(i: &[u8]) -> IResult<&[u8], BoxSphereBounds, VerboseError<&[u8]>> {
    let (i, origin) = parse_vec3(i)?;
    let (i, box_extent) = parse_vec3(i)?;
    let (i, sphere_radius) = le_f32(i)?;

    Ok((i, BoxSphereBounds {
        origin,
        box_extent,
        sphere_radius,
    }))
}