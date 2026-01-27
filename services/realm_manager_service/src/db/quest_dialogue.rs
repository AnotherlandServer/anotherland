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
use database::DatabaseRecord;
use mongodb::{IndexModel, bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use toolkit::GraphqlCrud;

use crate::db::CombatStyle;

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "QuestDialogue")]
pub struct QuestDialogue {
    pub id: i32,
    pub branches: Vec<DialogueBranch>,
}

impl DatabaseRecord for QuestDialogue {
    type PrimaryKey = i32;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "quest_dialogues"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()
        ).await?;

        Ok(())
    }
}


#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "DialogueBranchInput")]
pub struct DialogueBranch {
    pub selector: Option<DialogueBranchSelector>,
    pub lines: Vec<DialogueLine>,
}

#[derive(Serialize, Deserialize, Enum, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Close,
    Approve,
    Reject,
    Next,
    TellMore,
    Offer,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "DialogueLineInput")]
pub struct DialogueLine {
    pub line_id: i32,
    pub animation_name: Option<String>,
    pub choice: Option<Choice>,
    pub quest_id: Option<i32>,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "DialogueBranchSelectorInput")]
pub struct DialogueBranchSelector {
    pub quests_available: Vec<i32>,
    pub quests_in_progress: Vec<i32>,
    pub quests_complete: Vec<i32>,
    pub quests_finished: Vec<i32>,
    pub level: i32,
    pub combat_style: Option<CombatStyle>,
}