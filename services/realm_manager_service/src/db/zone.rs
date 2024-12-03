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

use database::DatabaseRecord;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{graphql_crud, types::Uuid};

#[derive(Clone, Serialize, Deserialize)]
#[graphql_crud("zone")]
pub struct Zone {
    pub id: i64,
    pub guid: Uuid,
    pub worlddef_guid: Uuid,
    pub parent_zone_guid: Uuid,
    pub zone: String,
    pub zone_type: i32,
    pub is_instance: bool,
    pub server: String,
    pub level: String,
    pub layer: String,
    pub realu_zone_type: String,
    pub game_controller: String,
}

impl DatabaseRecord for Zone {
    type PrimaryKey = i64;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "zones"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "guid": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}