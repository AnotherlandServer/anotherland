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

use async_graphql::{InputObject, SimpleObject};
use database::DatabaseRecord;
use mongodb::{bson::{self, doc}, options::{IndexOptions, ReturnDocument}, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};
use anyhow::anyhow;

#[derive(SimpleObject, InputObject, Serialize, Deserialize, Clone)]
#[graphql(name = "AbilitySlot", input_name = "AbilitySlotInput")]
pub struct  AbilitySlot {
    pub id: i32,
    pub ability: String,
}

impl Default for AbilitySlot {
    fn default() -> Self {
        Self {
            id: -1,
            ability: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, GraphqlCrud, Clone)]
#[graphql_crud(name = "ability_bar")]
pub struct AbilityBar {
    pub character_id: Uuid,
    pub single_slot: AbilitySlot,
    pub slots: Vec<AbilitySlot>,
}

impl DatabaseRecord for AbilityBar {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.character_id
    }

    fn key_name() -> &'static str {
        "character_id"
    }
    
    fn collection_name() -> &'static str {
        "ability_bar"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "character_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}

impl AbilityBar {
    pub async fn get_or_create(db: &mongodb::Database, character_id: Uuid) -> database::DBResult<AbilityBar> {
        let collection = Self::collection(db);

        let empty_ability_bar = AbilityBar {
            character_id,
            single_slot: AbilitySlot::default(),
            slots: vec![],
        };

        let storage = collection.find_one_and_update(doc! { "character_id": character_id }, 
        doc!{
                "$setOnInsert": bson::to_bson(&empty_ability_bar).unwrap()
            })
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or_else(|| anyhow!("upsert failed"))?;

        Ok(storage)
    }
}