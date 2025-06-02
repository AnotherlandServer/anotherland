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
use glam::{Vec2, Vec3};
use log::debug;
use nom::{bytes::complete::take, combinator::fail, error::{VerboseError}, multi::count, number::complete::{le_i16, le_i32, le_u16, le_u32, le_u8}, sequence::tuple, IResult};

use crate::{types::parse_object, Container, DeserializeUnrealObject, ObjectRef, PackageFile, UPKResult};

use super::ObjectProperty;

#[derive(Debug, Clone)]
pub struct Bounds {
    pub origin: Vec3,
    pub box_extent: Vec3,
    pub sphere_radius: f32,
}

pub fn parse_vec3(i: &[u8]) -> nom::IResult<&[u8], Vec3, VerboseError<&[u8]>> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::number::complete::le_f32,
            nom::number::complete::le_f32,
            nom::number::complete::le_f32,
        )),
        |(x, y, z)| Vec3::new(x, y, z),
    )(i)
}

#[derive(Debug, Clone)]
pub struct KDOPUncompressedType {
    pub min: Vec3,
    pub max: Vec3,
}

fn parse_kdop_uncompressed_type(i: &[u8]) -> nom::IResult<&[u8], KDOPUncompressedType, VerboseError<&[u8]>> {
    nom::combinator::map(
        nom::sequence::tuple((
            parse_vec3,
            parse_vec3,
        )),
        |(min, max)| KDOPUncompressedType {
            min, 
            max
        },
    )(i)
}

#[derive(Debug, Clone)]
pub struct KDOPCompact {
    pub min: [u8; 3],
    pub max: [u8; 3],
}

fn parse_fkdop_compact(i: &[u8]) -> nom::IResult<&[u8], KDOPCompact, VerboseError<&[u8]>> {
    nom::combinator::map(
        nom::sequence::tuple((
            take::<_, _, VerboseError<&[u8]>>(3usize),
            take::<_, _, VerboseError<&[u8]>>(3usize),
        )),
        |(mins, maxs)| KDOPCompact {
            min: mins.try_into().unwrap(),
            max: maxs.try_into().unwrap(),
        },
    )(i)
}

#[derive(Debug, Clone)]
pub struct KDOPCollisionTriangle {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub material_index: u16,
}

fn parse_fk_dop_triangle3(i: &[u8]) -> nom::IResult<&[u8], KDOPCollisionTriangle, VerboseError<&[u8]>> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::number::complete::le_u16,
            nom::number::complete::le_u16,
            nom::number::complete::le_u16,
            nom::number::complete::le_u16,
        )),
        |(f0, f2, f4, f6)| KDOPCollisionTriangle {
            v1: f0,
            v2: f2,
            v3: f4,
            material_index: f6,
        },
    )(i)
}

#[allow(clippy::type_complexity)]
pub fn parse_bulk_array<T, F>(f: F)
    -> impl FnMut(&'_ [u8])
    -> IResult<&'_ [u8], Vec<T>, VerboseError<&'_ [u8]>>
where
    F: for<'a> FnMut(&'a [u8]) -> IResult<&'a [u8], T, VerboseError<&'a [u8]>> + Copy,
{
    move |i: &[u8]| {
        let (i, _) =  nom::number::complete::le_u32::<_, VerboseError<&[u8]>>(i)?;
        let (i, count) = nom::number::complete::le_u32::<_, VerboseError<&[u8]>>(i)?;
        debug!("Array count: {count}");

        let (i, items) = nom::multi::count(f, count as usize)(i)?;
        Ok((i, items))
    }
}

pub fn skip_bulk_array(i: &[u8]) -> IResult<&[u8], (), VerboseError<&[u8]>> {
    let (i, size) = nom::number::complete::le_u32::<_, VerboseError<_>>(i)?; // Skip the first u32
    let (i, count) = nom::number::complete::le_u32::<_, VerboseError<_>>(i)?;
    debug!("Skipping array of count: {count}");

    let (i, _) = take::<_, _, VerboseError<&[u8]>>(count as usize * size as usize)(i)?; // Skip the items
    Ok((i, ()))
}

#[allow(clippy::type_complexity)]
pub fn parse_array<T, F>(f: F)
    -> impl FnMut(&'_ [u8])
    -> IResult<&'_ [u8], Vec<T>, VerboseError<&'_ [u8]>>
where
    F: FnMut(&[u8]) -> IResult<&[u8], T, VerboseError<&[u8]>> + Copy,
{
    move |i: &[u8]| {
        let (i, count) = nom::number::complete::le_u32::<_, VerboseError<&[u8]>>(i)?;
        debug!("Array count: {count}");

        let (i, items) = nom::multi::count(f, count as usize)(i)?;
        Ok((i, items))
    }
}

pub fn skip_bulk_data(i: &[u8]) -> IResult<&[u8], (), VerboseError<&[u8]>> {
    let (i, _flags) = nom::number::complete::le_u32::<_, VerboseError<_>>(i)?; 
    let (i, _element_count) = le_i32::<_, VerboseError<_>>(i)?;
    let (i, size_on_disk) = le_i32::<_, VerboseError<_>>(i)?;
    let (i, _) = le_i32::<_, VerboseError<_>>(i)?;

    let (i, _) = take::<_, _, VerboseError<&[u8]>>(size_on_disk as usize)(i)?;

    Ok((i, ()))
}

#[derive(Debug, Clone)]
pub struct StaticMeshRenderData {
    pub elements: Vec<StaticMeshElement>,
    pub position_vertex_buffer: PositionVertexBuffer,
    pub index_buffer: Vec<u16>,
}

fn parse_static_mesh_render_data(file: &Arc<PackageFile>, container: &Container)
    -> impl FnMut(&'_ [u8])
    -> IResult<&'_ [u8], StaticMeshRenderData, VerboseError<&'_ [u8]>> + Copy
{
    move |i: &[u8]| {
        let (i, _raw_triangles) = skip_bulk_data(i)?;
        let (i, elements) = parse_array(parse_static_mesh_element(file, container))(i)?;
        debug!("Parsed {} elements", elements.len());
        debug!("Elements: {elements:#?}");

        let (i, vertex_buffer) = parse_position_vertex_buffer(i)?;
        let (i, _) = parse_static_mesh_vertex_buffer(i)?;
        let (i, _) = parse_color_vertex_buffer(i)?;

        let (i, _num_vertices) = le_u32::<_, VerboseError<_>>(i)?;
        let (i, index_buffer) = parse_bulk_array(|i| {
            let (i, index) = le_u16(i)?;
            Ok((i, index))
        })(i)?; // Index buffer 
        let (i, _) = skip_bulk_array(i)?; // Wireframe index buffer
        let (i, _) = skip_bulk_array(i)?; // ?? Additional Index buffer

        Ok((
            i,
            StaticMeshRenderData {
                elements,
                position_vertex_buffer: vertex_buffer,
                index_buffer,
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PositionVertexBuffer {
    pub stride: u32,
    pub num_vertices: u32,
    pub data: Vec<Vec3>,
}

fn parse_position_vertex_buffer(i: &[u8]) -> IResult<&[u8], PositionVertexBuffer, VerboseError<&[u8]>> {
    let (i, stride) = le_u32::<_, VerboseError<_>>(i)?;
    let (i, num_vertices) = le_u32::<_, VerboseError<_>>(i)?;

    let (i, data) = parse_bulk_array(parse_vec3)(i)?; // Vertex data

    Ok((i, PositionVertexBuffer {
        stride,
        num_vertices,
        data,
    }))
}

#[derive(Debug, Clone)]
pub struct StaticMeshVertexBuffer {

}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct PackedNormal {
    x: u8,
    y: u8,
    z: u8,
    w: u8,
}

pub type StaticMeshFullVertex = (PackedNormal, PackedNormal);

#[allow(unused)]
pub struct Vec2Half {
    x: f16,
    y: f16,
}

fn parse_vec2d_half(i: &[u8]) -> IResult<&[u8], Vec2Half, VerboseError<&[u8]>> {
    let (i, x) = le_f16(i)?;
    let (i, y) = le_f16(i)?;
    Ok((i, Vec2Half { x, y }))
}

pub enum StaticMeshFullVertexUVs {
    Float16{vertex: StaticMeshFullVertex, uv: Vec<Vec2Half>},
    Float32{vertex: StaticMeshFullVertex, uv: Vec<Vec2>},
}

fn le_f16(i: &[u8]) -> IResult<&[u8], f16, VerboseError<&[u8]>> {
    let (i, bytes) = take(2usize)(i)?;
    let value = f16::from_bits(u16::from_le_bytes(bytes.try_into().unwrap()));
    Ok((i, value))
}

fn parse_static_mesh_full_vertex(i: &[u8]) -> IResult<&[u8], StaticMeshFullVertex, VerboseError<&[u8]>> {
    let (i, packed_normal) = tuple((le_u8, le_u8, le_u8, le_u8))(i)?;
    let (i, packed_tangent) = tuple((le_u8, le_u8, le_u8, le_u8))(i)?;

    Ok((
        i,
        (
            PackedNormal {
                x: packed_normal.0,
                y: packed_normal.1,
                z: packed_normal.2,
                w: packed_normal.3,
            },
            PackedNormal {
                x: packed_tangent.0,
                y: packed_tangent.1,
                z: packed_tangent.2,
                w: packed_tangent.3,
            },
        ),
    ))
}

fn parse_static_mesh_vertex_buffer(i: &[u8]) -> IResult<&[u8], StaticMeshVertexBuffer, VerboseError<&[u8]>> {
    let (i, num_tex_coords) = le_u32::<_, VerboseError<_>>(i)?;
    let (i, _stride) = le_u32::<_, VerboseError<_>>(i)?;
    let (i, _num_vertices) = le_u32::<_, VerboseError<_>>(i)?;
    let (i, _b_use_full_precision_uvs) = le_u32::<_, VerboseError<_>>(i)?;

    let (i, _data) = parse_bulk_array(|i: &[u8]| {
        let (i, packed_normal) = parse_static_mesh_full_vertex(i)?;
        let (i, uv) = count(parse_vec2d_half, num_tex_coords as usize)(i)?;
        Ok((i, StaticMeshFullVertexUVs::Float16 {
            vertex: packed_normal,
            uv,
        }))
    })(i)?;

    Ok((i, StaticMeshVertexBuffer {}))
}

fn parse_color_vertex_buffer(i: &[u8]) -> IResult<&[u8], StaticMeshVertexBuffer, VerboseError<&[u8]>> {
    let (i, _stride) = le_u32::<_, VerboseError<_>>(i)?;
    let (i, num_vertices) = le_u32::<_, VerboseError<_>>(i)?;

    let (i, _data) = if num_vertices > 0 {
        parse_bulk_array(
            |i: &[u8]| {
                let (i, (a, r, g, b)) = tuple((le_u8, le_u8, le_u8, le_u8))(i)?;
                Ok((i, (a, r, g, b)))
            })(i)?
    } else {
        (i, Vec::new())
    };

    Ok((i, StaticMeshVertexBuffer {}))
}

#[derive(Debug, Clone)]
pub struct StaticMeshElement {
    pub mat: Option<ObjectRef>,
    pub enable_collision: bool,
    pub old_enable_collision: bool,
    pub enable_shadow_casting: bool,
    pub first_index: i32,
    pub num_faces: i32,
    pub min_vertex_index: i32,
    pub max_vertex_index: i32,
    pub material_index: i32,
    pub fragments: Vec<(i32, i32)>,
}

fn parse_static_mesh_element(file: &Arc<PackageFile>, container: &Container)
    -> impl FnMut(&'_ [u8])
    -> IResult<&'_ [u8], StaticMeshElement, VerboseError<&'_ [u8]>> + Copy
{
    move |i: &[u8]| {
        let (i, mat) = parse_object(file.clone(), container)(i)
            .map(|(i, obj)| {
                if let ObjectProperty::Object(obj) = obj {
                    (i, Some(obj))
                } else {
                    (i, None)
                }
            })?;
        let (i, enable_collision) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, old_enable_collision) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, enable_shadow_casting) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, first_index) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, num_faces) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, min_vertex_index) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, max_vertex_index) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, material_index) = le_i32::<_, VerboseError<_>>(i)?;

        let (i, fragments) = parse_array(|i: &[u8]| {
            let (i, f30_0) = le_i32::<_, VerboseError<_>>(i)?;
            let (i, f30_1) = le_i32::<_, VerboseError<_>>(i)?;
            Ok((i, (f30_0, f30_1)))
        })(i)?;

        //let (i, _) = le_u8(i)?;
        //let (i, _) = le_u32(i)?;

        let (i, platform_data) = le_u8(i)?;
        let (i, _) = if platform_data == 0 {
            (i, ())
        } else {
            debug!("Parsing platform data for StaticMeshSection");

            let (i, _) = parse_array(|i| le_u32(i))(i)?;
            let (i, _) = parse_array(|i| le_u32(i))(i)?;
            let (i, _) = parse_array(|i| le_u16(i))(i)?;
            let (i, _) = parse_array(|i| le_i16(i))(i)?;
            let (i, _) = parse_array(|i| le_u16(i))(i)?;
            let (i, _) = parse_array(|i| le_u16(i))(i)?;
            let (i, _) = parse_array(|i| le_u16(i))(i)?;
            let (i, _) = parse_array(|i| le_u16(i))(i)?;

            (i, ())
        };

        debug!("enable_collision: {enable_collision}");
        debug!("old_enable_collision: {old_enable_collision}");
        debug!("enable_shadow_casting: {enable_shadow_casting}");

        Ok((
            i,
            StaticMeshElement {
                mat,
                enable_collision: enable_collision != 0,
                old_enable_collision: old_enable_collision != 0,
                enable_shadow_casting: enable_shadow_casting != 0,
                first_index,
                num_faces,
                min_vertex_index,
                max_vertex_index,
                material_index,
                fragments,
            },
        ))
    }
}

pub struct StaticMesh {
    pub bounds: Bounds,
    pub body_setup: Option<ObjectRef>,
    pub lod_meshes: Vec<StaticMeshRenderData>,
}

#[async_trait]
impl DeserializeUnrealObject for StaticMesh {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, i: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        debug!("Deserializing StaticMeshData for object: {}", object.fully_qualified_name());

        /*fs::write(format!("{}.{}", object.fully_qualified_name().replace("/", "_"), object.class().name()), i)
            .await
            .expect("Failed to write StaticMeshData to file");*/

        let file = object.package().unwrap();
        let object_start = i;

        let (i, origin) = parse_vec3(i)?;
        let (i, box_extent) = parse_vec3(i)?;
        let (i, sphere_radius) = nom::number::complete::le_f32::<_, VerboseError<_>>(i)?;

        let bounds = Bounds {
            origin,
            box_extent,
            sphere_radius,
        };

        debug!("Bounds: {bounds:#?}");

        let (i, body_setup) = parse_object(file.clone(), container)(i)
            .map(|(i, property)| if let ObjectProperty::Object(obj) = property {
                (i, Some(obj))
            } else {
                (i, None)
            })?;
        debug!("BodySetup: {body_setup:#?}");

        let (i, fk_dop_bounds) = parse_kdop_uncompressed_type(i)?;
        debug!("FkDOPBounds: {fk_dop_bounds:#?}");
        
        let (i, fk_dop_nodes) = parse_bulk_array(parse_fkdop_compact)(i)?;
        debug!("FkDOPNodes: {fk_dop_nodes:#?}");

        let (i, fk_drop_tris) = parse_bulk_array(parse_fk_dop_triangle3)(i)?;
        debug!("FkDOPTriangles: {fk_drop_tris:#?}");

        let (i, internal_version) = nom::number::complete::le_u32::<_, VerboseError<_>>(i)?;
        debug!("Internal Version: {internal_version}");

        let (i, has_unused_lod) = le_i32::<_, VerboseError<_>>(i)?;
        let (i, _) = if has_unused_lod == 0 {
            (i, ())
        } else {
            let (i, unused_lod) = parse_static_mesh_render_data(&file, container)(i)?;

            debug!("Unused LOD: {unused_lod:#?}");
            (i, ())
        };

        let (i, _): (_, Vec<u8>) = parse_array(|i: &[u8]| fail(i))(i)?;
        let (i, _) = le_i32::<_, VerboseError<_>>(i)?;

        debug!("File offset: {}", object_start.subslice_range(i).unwrap().start);

        let (i, lods) = parse_array(parse_static_mesh_render_data(&file, container))(i)?;

        debug!("Parsed {} LODs", lods.len());
        debug!("LODS: {lods:#?}");

        // Return the remaining bytes and an instance of StaticMesh
        Ok((i, StaticMesh {
            bounds,
            body_setup,
            lod_meshes: lods,
        }))
    }
}