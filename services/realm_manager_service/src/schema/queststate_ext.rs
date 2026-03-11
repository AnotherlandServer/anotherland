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

use async_graphql::{Context, Enum, Error, ID, InputObject, Json, Object, SimpleObject};
use database::DatabaseRecord;
use log::debug;
use mongodb::{Database, bson::{self, doc, oid::ObjectId}, options::ReturnDocument};
use obj_params::{GenericParamSet, ParamSet, Player};
use toolkit::{transaction_with_retry, types::Uuid};

use crate::{db::{Character, QuestProgressionState, QuestState, QuestStateOutput}, item_storage_session::ItemStorageSession, schema::item_storage_ext::{EquipmentResult, ItemRef, StorageResult, find_item}};

#[derive(Default)]
pub struct QuestStateExtMutationRoot;

#[derive(Clone, Copy, Enum, PartialEq, Eq)]
pub enum ConditionUpdate {
    Increment,
    Set,
}

#[derive(InputObject)]
pub struct QuestRewards {
    pub storage_id: Uuid,
    pub tag: Option<String>,
    pub experience: u32,
    pub bits: u32,
    pub cash: u32,
    pub items: Vec<ItemRef>,
}

#[derive(SimpleObject)]
pub struct QuestStateChangeResult {
    pub state: QuestStateOutput,
    pub equipment_result: Option<EquipmentResult>,
}

#[Object]
impl QuestStateExtMutationRoot {
    async fn update_condition(&self, ctx: &Context<'_>, state_id: ID, condition_idx: u32, update: ConditionUpdate, value: i32) -> Result<Option<QuestStateChangeResult>, Error> {
        let db = ctx.data::<Database>()?.clone();

        let quest_state = transaction_with_retry(db.clone(), async |mut session| -> Result<_, Error> {
            let quest_state_id: ObjectId = state_id.parse()?;
            let mut quest_state = QuestState::collection(&db)
                .find_one_and_update(doc! { "_id": quest_state_id }, match update {
                    ConditionUpdate::Increment => doc! { 
                        "$inc": { format!("conditions.{}.current_count", condition_idx): value },
                        "$set": { "last_condition_update": chrono::Utc::now().to_rfc3339() }
                    },
                    ConditionUpdate::Set => doc! { 
                        "$set": { 
                            format!("conditions.{}.current_count", condition_idx): value, 
                            "last_condition_update": chrono::Utc::now().to_rfc3339() 
                        }
                    },
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

            Ok((session, quest_state))
        }).await?;

        match quest_state {
            Some(state) => Ok(Some(QuestStateChangeResult {
                state: state.try_into()?,
                equipment_result: None,
            })),
            None => Ok(None),
        }
    }

    async fn update_state(&self, ctx: &Context<'_>, state_id: ID, new_state: QuestProgressionState, rewards: Option<QuestRewards>) -> Result<Option<QuestStateChangeResult>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let rewards = rewards.as_ref();

        let (quest_state, equipment_result) = transaction_with_retry(db.clone(), async |mut session| -> Result<_, Error> {
            let quest_state_id: ObjectId = state_id.parse()?;
            let Some(prev_quest_state) = QuestState::collection(&db)
                .find_one(doc! { "_id": quest_state_id })
                .session(&mut session)
                .await? else {
                    return Ok((session, (None, None)));
                };

            let Some(quest_state) = QuestState::collection(&db)
                .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": { "state": bson::to_bson(&new_state)? } })
                .return_document(ReturnDocument::After)
                .session(&mut session)
                .await? else {
                    return Ok((session, (None, None)));
                };

            match (prev_quest_state.state, quest_state.state) {
                (QuestProgressionState::Active, QuestProgressionState::Completed) |
                (QuestProgressionState::Active, QuestProgressionState::Finished) |
                (QuestProgressionState::Completed, QuestProgressionState::Finished) => {
                    let mut conditions = HashMap::new();
                    for (idx, condition) in quest_state.conditions.iter().enumerate() {
                        conditions.insert(format!("conditions.{}.current_count", idx), condition.required_count);
                    }

                    let quest_state_res = QuestState::collection(&db)
                        .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": bson::to_bson(&conditions)? })
                        .return_document(ReturnDocument::After)
                        .session(&mut session)
                        .await?;

                    // Give out rewards when quest is finished
                    if 
                        prev_quest_state.state != QuestProgressionState::Finished &&
                        quest_state.state == QuestProgressionState::Finished &&
                        let Some(rewards) = rewards 
                    {
                        let mut inventory_session = ItemStorageSession::with_session(&db, session, rewards.storage_id).await?;

                        if !rewards.items.is_empty() {
                            debug!("Giving quest rewards items: {:?}", rewards.items);

                            for item_ref in &rewards.items {
                                if let Some(item) = find_item(&db, item_ref.clone()).await? {
                                    inventory_session.insert_item(item, None, None).await?;
                                }
                            }
                        }

                        if rewards.bits > 0 {
                            debug!("Giving quest rewards bits: {}", rewards.bits);
                            inventory_session.add_bits(rewards.bits as i32).await?;
                        }

                        if rewards.cash > 0 {
                            debug!("Giving quest rewards cash: {}", rewards.cash);
                            inventory_session.add_cash(rewards.cash as i32).await?;
                        }

                        let (mut session, item_storage_result) = inventory_session.write_uncommitted().await?;

                        let character_update: Option<Json<Box<dyn GenericParamSet + 'static>>> = if rewards.experience > 0 {
                            debug!("Giving quest rewards experience: {}", rewards.experience);

                            let mut character = Character::collection(&db)
                                .find_one(doc! { "id": quest_state.character_id })
                                .session(&mut session)
                                .await?
                                .ok_or_else(|| Error::new("Character not found"))?;

                            character.add_exp(rewards.experience);
                            
                            let mut changes = ParamSet::<Player>::new();
                            character.data.changes()
                                .for_each(|(key, value)| {
                                    changes.set_param(key.name(), value);
                                });

                            character.save_uncommited(&Character::collection(&db)).session(&mut session).await?;

                            Some(Json(Box::new(changes)))
                        } else {
                            None
                        };

                        Ok((session, (
                                quest_state_res, Some(EquipmentResult {
                                error: None,
                                storage_result: item_storage_result
                                    .into_iter()
                                    .map(StorageResult::from)
                                    .collect(),
                                skillbook: None,
                                character_update,
                            }))
                        ))
                    } else {
                        Ok((session, (quest_state_res, None)))
                    }
                },
                (QuestProgressionState::Active, QuestProgressionState::Failed) => {
                    let quest_state_res = QuestState::collection(&db)
                        .find_one_and_update(doc! { "_id": quest_state_id }, doc! { "$set": { "conditions.$[].current_count": 0 } })
                        .return_document(ReturnDocument::After)
                        .session(&mut session)
                        .await?;

                    Ok((
                        session,
                        (
                            quest_state_res,
                            None,
                        )
                    ))
                },
                _ => {
                    session.abort_transaction().await?;
                    Err(Error::new("Invalid state transition"))
                }
            }
        }).await?;

        match quest_state {
            Some(state) => Ok(Some(QuestStateChangeResult {
                state: state.try_into()?,
                equipment_result,
            })),
            None => Ok(None),
        }
    }
}
