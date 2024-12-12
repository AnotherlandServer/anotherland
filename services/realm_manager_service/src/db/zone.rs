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

use async_graphql::Enum;
use database::DatabaseRecord;
use mongodb::{bson::{self, doc}, options::IndexOptions, Database, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    World,
    Ghost,
}

impl From<ZoneType> for mongodb::bson::Bson {
    fn from(value: ZoneType) -> Self {
        bson::to_bson(&value).unwrap()
    }
}

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "zone")]
pub struct Zone {
    pub id: i64,
    pub guid: Uuid,
    #[graphql_crud(filter)]
    pub worlddef_guid: Uuid,
    #[graphql_crud(filter)]
    pub parent_zone_guid: Uuid,
    pub zone: String,
    #[graphql_crud(filter)]
    pub zone_type: ZoneType,
    pub is_instance: bool,
    #[graphql_crud(filter)]
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

impl Zone {
    pub async fn get_by_guid(db: &Database, guid: Uuid) -> database::DBResult<Option<Self>> {
        Ok(Self::collection(db)
            .find_one(doc!{ "guid": guid })
            .await?)
    }
}