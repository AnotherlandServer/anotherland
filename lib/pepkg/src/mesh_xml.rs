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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "mesh")]
pub struct Mesh {
    #[serde(rename = "@majorRelease")]
    pub major_release: u32,

    #[serde(rename = "@minorRelease")]
    pub minor_release: u32,

    #[serde(rename = "mesh3D")]
    pub mesh_3d: Mesh3d,

    #[serde(rename = "mappingTo2D")]
    pub mapping_to_2d: MappingTo2D,

    #[serde(rename = "tiledCPFaceIndexTranslator2")]
    pub tiled_cp_face_index_translator_2: TiledCPFaceIndexTranslator2,

    #[serde(rename = "baseSurfaceTypeCosts")]
    pub base_surface_type_costs: BaseSurfaceTypeCosts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "tiledCPFaceIndexTranslator2")]
pub struct TiledCPFaceIndexTranslator2 {
    #[serde(rename = "@federationTileIndex")]
    pub federation_tile_index: u32,

    #[serde(rename = "@startX")]
    pub start_x: i32,

    #[serde(rename = "@startY")]
    pub start_y: i32,

    #[serde(rename = "@tileSize")]
    pub tile_size: i32,

    #[serde(rename = "@beginTileX")]
    pub begin_tile_x: i32,

    #[serde(rename = "@beginTileY")]
    pub begin_tile_y: i32,

    #[serde(rename = "@endTileX", default)]
    pub end_tile_x: Option<i32>,

    #[serde(rename = "@endTileY", default)]
    pub end_tile_y: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "mappingTo2D")]
pub struct MappingTo2D {
    #[serde(rename = "poly")]
    pub polys: Vec<Poly>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "mesh3d")]
pub struct Mesh3d {
    #[serde(rename = "verts")]
    pub verts: Verts,

    #[serde(rename = "tris")]
    pub tris: Tris,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verts {
    #[serde(rename = "vert")]
    pub verts: Vec<Vert>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "vert")]
pub struct Vert {
    #[serde(rename = "@x", default)]
    pub x: i32,

    #[serde(rename = "@y", default)]
    pub y: i32,

    #[serde(rename = "@z", default)]
    pub z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tris {
    #[serde(rename = "tri")]
    pub tris: Vec<Tri>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tri {
    #[serde(rename = "@surfaceType")]
    pub surface_type: u32,

    #[serde(rename = "@edge0StartVert")]
    pub edge0_start_vert: u32,

    #[serde(rename = "@edge0StartZ")]
    pub edge0_start_z: Option<i32>,

    #[serde(rename = "@edge0Connection")]
    pub edge0_connection: Option<u32>,

    #[serde(rename = "@edge1StartVert")]
    pub edge1_start_vert: u32,

    #[serde(rename = "@edge1StartZ")]
    pub edge1_start_z: Option<i32>,

    #[serde(rename = "@edge1Connection")]
    pub edge1_connection: Option<u32>,

    #[serde(rename = "@edge2StartVert")]
    pub edge2_start_vert: u32,

    #[serde(rename = "@edge2StartZ")]
    pub edge2_start_z: Option<i32>,

    #[serde(rename = "@edge2Connection")]
    pub edge2_connection: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poly {
    #[serde(rename = "edge")]
    edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    #[serde(rename = "@startVert")]
    pub start_vert: u32,

    #[serde(rename = "@connection")]
    pub connection: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "baseSurfaceTypeCosts")]
pub struct BaseSurfaceTypeCosts {
    #[serde(rename = "costs")]
    pub costs: Costs,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "costs")]
pub struct Costs {
    #[serde(rename = "entry")]
    pub costs: Vec<Cost>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cost {
    #[serde(rename = "@surfaceType")]
    pub surface_type: u32,

    #[serde(rename = "@cost")]
    pub cost: i32,
}