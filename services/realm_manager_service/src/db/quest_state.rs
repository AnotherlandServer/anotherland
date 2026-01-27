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

use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use database::DatabaseRecord;
use mongodb::{bson::{doc, Bson}, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud, ObjectId};

#[derive(Debug, Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum QuestProgressionState {
    Active,
    Completed,
    Finished,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, InputObject, SimpleObject)]
#[graphql(input_name = "QuestConditionInput", name = "QuestCondition")]
pub struct QuestCondition {
    pub id: i32,
    pub current_count: i32,
    pub required_count: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "QuestState", primary_key_type = "async_graphql::types::ID")]
pub struct QuestState {
    #[serde(
        rename = "_id",
        default,
    )]
    #[graphql_crud(serialize_as = "async_graphql::types::ID", readonly)]
    pub id: ObjectId,

    #[graphql_crud(filter)]
    pub character_id: Uuid,

    #[graphql_crud(filter)]
    pub quest_id: i32,

    pub accepted_time: DateTime<Utc>,
    pub last_condition_update: DateTime<Utc>,

    pub state: QuestProgressionState,
    pub conditions: Vec<QuestCondition>,

    pub lua_state: Option<Bson>,
}

impl DatabaseRecord for QuestState {
    type PrimaryKey = ObjectId;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "_id"
    }

    fn collection_name() -> &'static str {
        "quest_states"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "character_id": 1, "quest_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "character_id": 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build()).await?;

        Ok(())
    }
}