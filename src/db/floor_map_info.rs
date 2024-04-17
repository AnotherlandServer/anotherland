// Copyright (C) 2024 AnotherlandServer
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

use atlas::Uuid;
use bson::{doc, Document};
use glam::Vec3;
use log::info;
use mongodb::{Collection, Database};
use poem::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tokio_stream::StreamExt;

use crate::{db::realm_database, util::{AnotherlandError, AnotherlandResult}};

use super::DatabaseRecord;

#[derive(Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn contains(&self, pos: &Vec3) -> bool {
        pos.x >= self.min.x &&
        pos.x <= self.max.x &&
        pos.y >= self.min.y &&
        pos.y <= self.max.y &&
        pos.z >= self.min.z &&
        pos.z <= self.max.z
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FloorMapInfo {
    pub id: Uuid,
    pub bounded_texture_size_x: u32,
    pub bounded_texture_size_y: u32,
    pub bounding_box: BoundingBox,
    pub tile_size: u32,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub units_per_pixel: f32,
    pub world_name: String,
    pub world_id: Option<u16>,
    pub zone_id: u16,
}

#[async_trait]
impl DatabaseRecord<'_> for FloorMapInfo {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("floor_map_infos")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

static FLOOR_MAP_CACHE: OnceCell<Vec<FloorMapInfo>> = OnceCell::const_new();

pub async fn initialize_floor_map_cache() -> AnotherlandResult<()> {
    FLOOR_MAP_CACHE.get_or_try_init(|| async move {
        let collection = FloorMapInfo::collection(realm_database().await);
        let mut cursor = collection.find(None, None).await?;
    
        let mut cache = Vec::new();
    
        while let Some(item) = cursor.try_next().await? {
            cache.push(item);
        }

        info!("Cached {} floor maps...", cache.len());

        Ok::<_, AnotherlandError>(cache)
    }).await?;
    
    Ok(())
}

pub fn get_cached_floor_maps(world_id: u16) -> Vec<&'static FloorMapInfo> {
    FLOOR_MAP_CACHE.get()
        .expect("floor map cache not initialized")
        .iter()
        .filter(|v| {
            v.world_id == Some(world_id)
        })
        .collect()
}
