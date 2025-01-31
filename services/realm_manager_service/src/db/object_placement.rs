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

use database::DatabaseRecord;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use obj_params::{Class, GameObjectData};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

use crate::schema::ClassWrapper;

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "object_placement")]
pub struct ObjectPlacement {
    id: Uuid,
    #[graphql_crud(filter)]
    zone_guid: Uuid,
    #[graphql_crud(serialize_as = ClassWrapper, filter)]
    class: Class,
    content_guid: Uuid,
    editor_name: String,
    #[graphql_crud(serialize_as = serde_json::Value)]
    data: GameObjectData,
    #[graphql_crud(filter)]
    phase_tag: String,
}

impl DatabaseRecord for ObjectPlacement {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "object_placements"
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
            .keys(doc! { "zone_guid": 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "class": 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build()).await?;

        Ok(())
    }
}