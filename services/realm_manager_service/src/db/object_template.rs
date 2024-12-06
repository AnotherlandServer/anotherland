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
use mongodb::{bson::{self, doc}, options::IndexOptions, IndexModel};
use obj_params::{Class, GameObjectData};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

use crate::schema::ClassWrapper;

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    NoBinding,
    Buffs,
    Drops,
    Enemies,
    Factions,
    Items,
    Metagame,
    Misc,
    Npcs,
    Projectiles,
    Quests,
    Recipes,
    Skills,
    Spawners,
    Structures,
}

impl From<Category> for mongodb::bson::Bson {
    fn from(value: Category) -> Self {
        bson::to_bson(&value).unwrap()
    }
}

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "object_template")]
pub struct ObjectTemplate {
    id: Uuid,
    #[graphql_crud(filter)]
    category: Category,
    name: String,
    #[graphql_crud(serialize_as = ClassWrapper, filter)]
    class: Class,
    #[graphql_crud(serialize_as = serde_json::Value)]
    data: GameObjectData,
}

impl DatabaseRecord for ObjectTemplate {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "object_templates"
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
            .keys(doc! { "category": 1 })
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