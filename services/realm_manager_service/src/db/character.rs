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

use cynic::serde::{Deserialize, Serialize};
use database::DatabaseRecord;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use obj_params::GameObjectData;
use toolkit::{types::Uuid, GraphqlCrud};

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "character")]
pub struct Character {
    pub id: Uuid,
    #[graphql_crud(filter)]
    pub account: Uuid,
    #[graphql_crud(filter)]
    pub index: i32,
    #[graphql_crud(filter)]
    pub name: String,
    #[graphql_crud(serialize_as = serde_json::Value)]
    pub data: GameObjectData,
}

impl DatabaseRecord for Character {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "characters"
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
            .keys(doc! { "name": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "account": 1, "index": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}