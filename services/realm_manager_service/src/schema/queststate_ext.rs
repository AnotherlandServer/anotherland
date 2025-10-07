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

use std::collections::HashMap;

use async_graphql::{Context, Enum, Error, Object, ID};
use database::DatabaseRecord;
use log::debug;
use mongodb::{bson::{self, doc, oid::ObjectId}, options::{ReadConcern, ReadPreference, ReturnDocument, SelectionCriteria, TransactionOptions, WriteConcern}, Database};

use crate::db::{QuestProgressionState, QuestState, QuestStateOutput};

#[derive(Default)]
pub struct QuestStateExtMutationRoot;

#[derive(Clone, Copy, Enum, PartialEq, Eq)]
pub enum ConditionUpdate {
    Increment,
    Set,
}

#[Object]
impl QuestStateExtMutationRoot {
    async fn update_condition(&self, ctx: &Context<'_>, state_id: ID, condition_id: u32, update: ConditionUpdate, value: i32) -> Result<Option<QuestStateOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = db.client().start_session()
            .default_transaction_options(TransactionOptions::builder()
                .read_concern(ReadConcern::majority())
                .write_concern(WriteConcern::majority())
                .selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::Primary))
                .build()
            )
            .causal_consistency(true)
            .await?;
        session.start_transaction().await?;

        let quest_state_id: ObjectId = state_id.parse()?;
        let mut quest_state = QuestState::collection(&db)
            .find_one_and_update(doc! { "_id": quest_state_id }, match update {
                ConditionUpdate::Increment => doc! { "$inc": { format!("conditions.{}.current_count", condition_id): value } },
                ConditionUpdate::Set => doc! { "$set": { format!("conditions.{}.current_count", condition_id): value } },
            })
            .return_document(ReturnDocument::After)
            .session(&mut session)
            .await?;

        // Check if quest is completed
        if 
            let Some(state) = &mut quest_state  &&
            state.conditions.iter().all(|c| c.current_count >= c.required_count)
        {
            state.state = QuestProgressionState::Completed;
            state
                .save_uncommited(&QuestState::collection(&db))
                .session(&mut session)
                .await?;
        }

        session.commit_transaction().await?; 

        match quest_state {
            Some(state) => Ok(Some(state.try_into()?)),
            None => Ok(None),
        }
    }

    async fn update_state(&self, ctx: &Context<'_>, state_id: ID, new_state: QuestProgressionState) -> Result<Option<QuestStateOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = db.client().start_session()
            .default_transaction_options(TransactionOptions::builder()
                .read_concern(ReadConcern::majority())
                .write_concern(WriteConcern::majority())
                .selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::Primary))
                .build()
            )
            .causal_consistency(true)
            .await?;

        session.start_transaction().await?;

        let quest_state_id: ObjectId = state_id.parse()?;
        let Some(prev_quest_state) = QuestState::collection(&db)
            .find_one(doc! { "_id": quest_state_id })
            .session(&mut session)
            .await? else {
                return Ok(None);
            };

        let Some(quest_state) = QuestState::collection(&db)
            .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": { "state": bson::to_bson(&new_state)? } })
            .return_document(ReturnDocument::After)
            .session(&mut session)
            .await? else {
                return Ok(None);
            };

        let quest_state = match (prev_quest_state.state, quest_state.state) {
            (QuestProgressionState::Active, QuestProgressionState::Completed) |
            (QuestProgressionState::Active, QuestProgressionState::Finished) |
            (QuestProgressionState::Completed, QuestProgressionState::Finished) => {
                let mut conditions = HashMap::new();
                for (idx, condition) in quest_state.conditions.iter().enumerate() {
                    conditions.insert(format!("conditions.{}.current_count", idx), condition.required_count);
                }

                QuestState::collection(&db)
                    .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": bson::to_bson(&conditions)? })
                    .return_document(ReturnDocument::After)
                    .session(&mut session)
                    .await?
            },
            (QuestProgressionState::Active, QuestProgressionState::Failed) => {
                QuestState::collection(&db)
                    .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": { "conditions.$[].current_count": 0 } })
                    .return_document(ReturnDocument::After)
                    .session(&mut session)
                    .await?
            },
            _ => {
                session.abort_transaction().await?;
                return Err(Error::new("Invalid state transition"));
            }
        };

        session.commit_transaction().await?;

        match quest_state {
            Some(state) => Ok(Some(state.try_into()?)),
            None => Ok(None),
        }
    }
}
