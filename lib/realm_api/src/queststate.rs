// Copyright (C) 2026 AnotherlandServer
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

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use cynic::http::ReqwestExt;
use cynic::{MutationBuilder, QueryBuilder, Id};
use derive_builder::Builder;
use futures::TryStreamExt;
use toolkit::types::Uuid;
use toolkit::anyhow;
use toolkit::record_pagination::{RecordCursor, RecordPage, RecordQuery};
use crate::queststate_graphql::{QuestStateChangeResult, UpdateState};
use crate::{EquipmentResult, ItemRef, RealmApi, RealmApiError, RealmApiResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestProgressionState {
    Active,
    Completed,
    Finished,
    Failed,
}

impl From<queststate_graphql::QuestProgressionState> for QuestProgressionState {
    fn from(value: queststate_graphql::QuestProgressionState) -> Self {
        match value {
            queststate_graphql::QuestProgressionState::Active => Self::Active,
            queststate_graphql::QuestProgressionState::Completed => Self::Completed,
            queststate_graphql::QuestProgressionState::Finished => Self::Finished,
            queststate_graphql::QuestProgressionState::Failed => Self::Failed,
        }
    }
}

impl From<QuestProgressionState> for queststate_graphql::QuestProgressionState {
    fn from(value: QuestProgressionState) -> Self {
        match value {
            QuestProgressionState::Active => Self::Active,
            QuestProgressionState::Completed => Self::Completed,
            QuestProgressionState::Finished => Self::Finished,
            QuestProgressionState::Failed => Self::Failed,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConditionUpdate {
    Increment,
    Set,
}

impl From<queststate_graphql::ConditionUpdate> for ConditionUpdate {
    fn from(value: queststate_graphql::ConditionUpdate) -> Self {
        match value {
            queststate_graphql::ConditionUpdate::Increment => Self::Increment,
            queststate_graphql::ConditionUpdate::Set => Self::Set,
        }
    }
}

impl From<ConditionUpdate> for queststate_graphql::ConditionUpdate {
    fn from(value: ConditionUpdate) -> Self {
        match value {
            ConditionUpdate::Increment => Self::Increment,
            ConditionUpdate::Set => Self::Set,
        }
    }
}


#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct QuestStateQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    character_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    quest_id: Option<i32>,
}

impl QuestStateQuery {
    fn get_filter(&self) -> Option<queststate_graphql::QuestStateFilter> {
        if self.character_id.is_none() && self.quest_id.is_none() {
            None
        } else {
            Some(queststate_graphql::QuestStateFilter { 
                character_id: self.character_id,
                quest_id: self.quest_id,
            })
        }
    }
}

impl RecordQuery for QuestStateQuery {
    type Record = QuestState;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(queststate_graphql::GetQuestStates::build(queststate_graphql::GetQuestStatesVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(queststate_graphql::GetQuestStates { quest_states }) = response.data {
            Ok(RecordPage {
                at_end: !quest_states.page_info.has_next_page,
                last_cursor: quest_states.page_info.end_cursor,
                records: quest_states.nodes.into_iter()
                    .map(|quest_state| QuestState::from_graphql(&self.api_base, quest_state))
                    .collect::<Vec<_>>(),
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl QuestStateQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<QuestStateQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuestCondition {
    pub id: i32,
    pub current_count: i32,
    pub required_count: i32,
}

impl From<queststate_graphql::QuestCondition> for QuestCondition {
    fn from(value: queststate_graphql::QuestCondition) -> Self {
        QuestCondition {
            id: value.id,
            current_count: value.current_count,
            required_count: value.required_count,
        }
    }
}

impl From<QuestCondition> for queststate_graphql::QuestConditionInput {
    fn from(value: QuestCondition) -> Self {
        queststate_graphql::QuestConditionInput {
            id: value.id,
            current_count: value.current_count,
            required_count: value.required_count,
        }
    }
}

#[derive(Default)]
pub struct QuestRewards<'a> {
    pub storage_id: Uuid,
    pub tag: Option<String>,
    pub experience: u32,
    pub bits: u32,
    pub cash: u32,
    pub items: Vec<ItemRef<'a>>,
}

impl<'a> TryFrom<QuestRewards<'a>> for queststate_graphql::QuestRewards<'a> {
    type Error = RealmApiError;

    fn try_from(value: QuestRewards<'a>) -> Result<Self, Self::Error> {
        Ok(queststate_graphql::QuestRewards {
            experience: value.experience.try_into().unwrap_or_default(),
            bits: value.bits.try_into().unwrap_or_default(),
            cash: value.cash.try_into().unwrap_or_default(),
            items: value.items.into_iter().map(TryInto::try_into).collect::<Result<_, RealmApiError>>()?,
            storage_id: value.storage_id, 
            tag: value.tag.clone(), 
        })
    }
}

#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct QuestState {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,
    #[builder(setter(skip), default)]
    pub id: Option<Id>,

    pub character_id: Uuid,
    pub quest_id: i32,
    pub state: QuestProgressionState,
    pub conditions: Vec<QuestCondition>,
    pub accepted_time: DateTime<Utc>,
    pub last_condition_update: DateTime<Utc>,
}

impl QuestState {
    fn from_graphql(api: &RealmApi, other: queststate_graphql::QuestState) -> Self {
        Self {
            api_base: Some(api.clone()),
            id: Some(other.id),
            character_id: other.character_id, 
			quest_id: other.quest_id,
            state: other.state.into(),
            conditions: other
                .conditions
                .into_iter()
                .map(QuestCondition::from)
                .collect(),
            accepted_time: other.accepted_time,
            last_condition_update: other.last_condition_update,
        }
    }

    fn as_graphql(&self) -> queststate_graphql::QuestStateInput {
        queststate_graphql::QuestStateInput {
            character_id: self.character_id,
            quest_id: self.quest_id,
            state: self.state.into(),
            conditions: self.conditions.iter().copied().map(Into::into).collect(),
            accepted_time: self.accepted_time,
            last_condition_update: self.last_condition_update,
        }
    }

    pub async fn save(&mut self) -> RealmApiResult<()> {
		if 
			let Some(api_base) = &self.api_base && 
			let Some(graphql_id) = &self.id
		{
			let response = api_base
				.0
				.client
				.post(api_base.0.base_url.clone())
				.run_graphql(queststate_graphql::UpdateQueststate::build(
					queststate_graphql::UpdateQueststateVariables {
						id: graphql_id.clone(),
						input: self.as_graphql(),
					},
				))
				.await?;

			if let Some(errors) = response.errors {
				return Err(RealmApiError::GraphQl(errors));
			}

			if 
				let Some(data) = response.data &&
				let Some(quest_state) = data.update_quest_state 
			{
				*self = Self::from_graphql(api_base, quest_state);
			}
		}

        Ok(())
    }

    pub async fn update_condition(&mut self, condition_idx: i32, update: ConditionUpdate, value: i32) -> RealmApiResult<()> {
        if 
            let Some(api_base) = &self.api_base &&
            let Some(graphql_id) = &self.id &&
            let Some(updated) = api_base
                .update_condition(graphql_id, condition_idx, update, value)
                .await?
        {
            *self = updated;
        }
        Ok(())
    }

    /// Update the quest progression state and refresh this instance.
    pub async fn update_state(&mut self, new_state: QuestProgressionState) -> RealmApiResult<()> {
        if 
            let (Some(api_base), Some(graphql_id)) = (&self.api_base, &self.id) &&
            let (Some(updated), _) = api_base.update_state(graphql_id, new_state, None).await?
        {
            *self = updated;
        }
        Ok(())
    }

    pub async fn return_quest(&mut self, rewards: QuestRewards<'_>) -> RealmApiResult<EquipmentResult> {
        if 
            let (Some(api_base), Some(graphql_id)) = (&self.api_base, &self.id) &&
            let (Some(updated), Some(equipment_result)) = api_base
                .update_state(graphql_id, QuestProgressionState::Finished, Some(rewards)).await?
        {
            *self = updated;

            Ok(equipment_result)
        } else {
            Err(RealmApiError::Other(anyhow!("Invalid quest state")))
        }
    }

    pub async fn delete(self) -> RealmApiResult<()> {
		if 
			let Some(api_base) = &self.api_base &&
			let Some(graphql_id) = &self.id
		{
			let response = api_base
				.0
				.client
				.post(api_base.0.base_url.clone())
				.run_graphql(queststate_graphql::DeleteQueststate::build(
					queststate_graphql::DeleteQueststateVariables { id: graphql_id.clone() },
				))
				.await?;

			if let Some(errors) = response.errors {
				return Err(RealmApiError::GraphQl(errors));
			}
		}

        Ok(())
    }
}

impl RealmApi {
    pub fn create_empty_queststate(
        &self,
        character_id: Uuid,
        quest_id: i32,
        state: QuestProgressionState,
    ) -> QuestState {
        QuestState {
            api_base: Some(self.clone()),
            id: None,
			character_id,
			quest_id,
            state,
            conditions: vec![],
            accepted_time: Utc::now(),
            last_condition_update: Utc::now(),
        }
    }

    pub fn query_quest_states(&self) -> QuestStateQueryBuilder {
        QuestStateQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn get_queststate(&self, character_id: Uuid, quest_id: i32) -> RealmApiResult<Option<QuestState>> {
        // Query using character_id and quest_id filters
        let mut query = self.query_quest_states()
            .character_id(character_id)
            .quest_id(quest_id)
            .query().await?;

        // Get the first result if any
        if let Some(quest_state) = query.try_next().await? {
            return Ok(Some(quest_state));
        }
        
        Ok(None)
    }

    pub async fn create_queststate(&self, quest_state: &QuestState) -> RealmApiResult<QuestState> {
        let response = self
            .0
            .client
            .post(self.0.base_url.clone())
            .run_graphql(queststate_graphql::CreateQueststate::build(
                queststate_graphql::CreateQueststateVariables {
                    input: quest_state.as_graphql(),
                },
            ))
            .await?;

        if let Some(errors) = response.errors {
            return Err(RealmApiError::GraphQl(errors));
        }

        if let Some(data) = response.data {
            return Ok(QuestState::from_graphql(self, data.create_quest_state));
        }

        Err(RealmApiError::Other(anyhow::anyhow!("No data returned")))
    }

    /// Call the updateCondition mutation directly.
    pub async fn update_condition(
        &self,
        state_id: &Id,
        condition_idx: i32,
        update: ConditionUpdate,
        value: i32,
    ) -> RealmApiResult<Option<QuestState>> {
        let response = self
            .0
            .client
            .post(self.0.base_url.clone())
            .run_graphql(queststate_graphql::UpdateCondition::build(
                queststate_graphql::UpdateConditionVariables {
                    state_id: state_id.clone(),
                    condition_idx,
                    update: update.into(),
                    value,
                },
            ))
            .await?;

        if let Some(errors) = response.errors {
            return Err(RealmApiError::GraphQl(errors));
        }

        if let Some(data) = response.data {
            Ok(data.update_condition.map(|qs| QuestState::from_graphql(self, qs.state)))
        } else {
            Err(RealmApiError::Other(anyhow::anyhow!("No data returned")))
        }
    }

    /// Call the updateState mutation directly.
    pub async fn update_state(
        &self,
        state_id: &Id,
        new_state: QuestProgressionState,
        rewards: Option<QuestRewards<'_>>,
    ) -> RealmApiResult<(Option<QuestState>, Option<EquipmentResult>)> {
        let response = self
            .0
            .client
            .post(self.0.base_url.clone())
            .run_graphql(queststate_graphql::UpdateState::build(
                queststate_graphql::UpdateStateVariables {
                    state_id: state_id.clone(),
                    new_state: new_state.into(),
                    rewards: rewards.map(|r| r.try_into()).transpose()?,
                },
            ))
            .await?;

        if let Some(errors) = response.errors {
            return Err(RealmApiError::GraphQl(errors));
        }

        if let Some(UpdateState { update_state }) = response.data {
            if let Some(QuestStateChangeResult { state, equipment_result }) = update_state {
                Ok((
                    Some(QuestState::from_graphql(self, state)),
                    equipment_result.map(|r| EquipmentResult::from_graphql(self, r))
                        .transpose()?,
                ))
            } else {
                Ok((None, None))
            }
        } else {
            Err(RealmApiError::Other(anyhow::anyhow!("No data returned")))
        }
    }
}

pub(crate) mod queststate_graphql {
    use chrono::{DateTime, Utc};
    use toolkit::types::Uuid;

    use crate::item_storage_graphql::{EquipmentResult, ItemRef};
    use crate::schema::*;
    use crate::schema::schema::ID;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetQuestStatesVariables<'a> {
        pub filter: Option<QuestStateFilter>,
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateQueststateVariables {
        pub input: QuestStateInput,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateQueststateVariables {
        pub id: ID,
        pub input: QuestStateInput,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteQueststateVariables {
        pub id: ID,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetQuestStatesVariables")]
    pub struct GetQuestStates {
        #[arguments(filter: $filter, after: $after, first: $first)]
        pub quest_states: QuestStateConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestStateConnection {
        pub nodes: Vec<QuestState>,
        pub page_info: PageInfo,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestCondition {
        pub id: i32,
        pub current_count: i32,
        pub required_count: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestState {
        pub id: ID,
        pub character_id: Uuid,
        pub quest_id: i32,
        pub state: QuestProgressionState,
        pub conditions: Vec<QuestCondition>,
        pub accepted_time: DateTime<Utc>,
        pub last_condition_update: DateTime<Utc>,
    }

    #[derive(cynic::QueryFragment)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestStateChangeResult {
        pub state: QuestState,
        pub equipment_result: Option<EquipmentResult>,
    }

    #[derive(cynic::Enum, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum QuestProgressionState {
        Active,
        Completed,
        Finished,
        Failed,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestStateFilter {
        #[cynic(rename = "characterId")]
        pub character_id: Option<Uuid>,
        #[cynic(rename = "questId")]
        pub quest_id: Option<i32>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestConditionInput {
        pub id: i32,
        pub current_count: i32,
        pub required_count: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestStateInput {
        pub character_id: Uuid,
        pub quest_id: i32,
        pub state: QuestProgressionState,
        pub conditions: Vec<QuestConditionInput>,
        pub accepted_time: DateTime<Utc>,
        pub last_condition_update: DateTime<Utc>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateQueststateVariables")]
    pub struct CreateQueststate {
        #[arguments(input: $input)]
        pub create_quest_state: QuestState,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "UpdateQueststateVariables")]
    pub struct UpdateQueststate {
        #[arguments(id: $id, input: $input)]
        pub update_quest_state: Option<QuestState>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteQueststateVariables")]
    #[allow(dead_code)]
    pub struct DeleteQueststate {
        #[arguments(id: $id)]
        pub delete_quest_state: Option<QuestState>,
    }

    // New enum for condition update
    #[derive(cynic::Enum, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum ConditionUpdate {
        Increment,
        Set,
    }

    // Variables for updateCondition
    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateConditionVariables {
        pub state_id: ID,
        pub condition_idx: i32,
        pub update: ConditionUpdate,
        pub value: i32,
    }

    // Mutation fragment for updateCondition
    #[derive(cynic::QueryFragment)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "UpdateConditionVariables")]
    pub struct UpdateCondition {
        #[arguments(stateId: $state_id, conditionIdx: $condition_idx, update: $update, value: $value)]
        pub update_condition: Option<QuestStateChangeResult>,
    }

    // Variables for updateState
    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateStateVariables<'a> {
        pub state_id: ID,
        pub new_state: QuestProgressionState,
        pub rewards: Option<QuestRewards<'a>>,
    }

    // Mutation fragment for updateState
    #[derive(cynic::QueryFragment)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "UpdateStateVariables")]
    pub struct UpdateState {
        #[arguments(stateId: $state_id, newState: $new_state, rewards: $rewards)]
        pub update_state: Option<QuestStateChangeResult>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestRewards<'a> {
        pub storage_id: Uuid,
        pub tag: Option<String>,
        pub experience: i32,
        pub bits: i32,
        pub cash: i32,
        pub items: Vec<ItemRef<'a>>,
    }
}
