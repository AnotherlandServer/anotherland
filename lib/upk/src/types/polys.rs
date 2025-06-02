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
use glam::Vec3;
use nom::{error::VerboseError, number::complete::{le_f32, le_i32, le_u32}, IResult};

use crate::{Container, DeserializeUnrealObject, FName, ObjectRef, PackageFile, UPKResult};

use super::{parse_array, parse_name, parse_object, parse_vec3, ObjectProperty};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Polys {
    element: Vec<Poly>,
}

#[derive(Debug, Clone)]
pub struct Poly {
    pub base: Vec3,
    pub normal: Vec3,
    pub texture_u: Vec3,
    pub texture_v: Vec3,
    pub vertices: Vec<Vec3>,
    pub poly_flags: u32,
    pub actor: Option<ObjectRef>,
    pub item_name: Option<FName>,
    pub material: Option<ObjectRef>,
    pub i_link: i32,
    pub i_brush_poly: i32,
    pub shadow_map_scale: f32,
    pub lighing_channels: u32,
}

#[async_trait]
impl DeserializeUnrealObject for Polys {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let file = object.package().unwrap();

        let (i, element) = parse_array(parse_poly(&file, container))(i)?;

        Ok((i, Self {
            element,
        }))
    }
}

fn parse_poly(file: &Arc<PackageFile>, container: &Container)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], Poly, VerboseError<&[u8]>> + Copy
{
    move |i: &'_ [u8]| {
        let (i, base) = parse_vec3(i)?;
        let (i, normal) = parse_vec3(i)?;
        let (i, texture_u) = parse_vec3(i)?;
        let (i, texture_v) = parse_vec3(i)?;
        let (i, vertices) = parse_array(parse_vec3)(i)?;
        let (i, poly_flags) = le_u32::<_, VerboseError<_>>(i)?;
        let (i, actor) = parse_object(file.clone(), container)(i)?;
        
        let actor = if let ObjectProperty::Object(obj) = actor {
            Some(obj)
        } else {
            None
        };

        let (i, item_name) = parse_name(file)(i)?;
        let item_name = if let ObjectProperty::Name(name) = item_name {
            Some(name)
        } else {
            None
        };

        let (i, material) = parse_object(file.clone(), container)(i)?;
        
        let material = if let ObjectProperty::Object(material) = material {
            Some(material)
        } else {
            None
        };

        let (i, i_link) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, i_brush_poly) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, shadow_map_scale) = le_f32::<_, VerboseError<_>>(i)?;
        let (i, lighing_channels) = le_u32::<_, VerboseError<_>>(i)?;

        Ok((
            i,
            Poly {
                base,
                normal,
                texture_u,
                texture_v,
                vertices,
                poly_flags,
                actor,
                item_name,
                material,
                i_link,
                i_brush_poly,
                shadow_map_scale,
                lighing_channels,
            }
        ))
    }
}
