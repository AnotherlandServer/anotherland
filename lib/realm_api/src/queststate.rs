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

use cynic::http::ReqwestExt;
use cynic::{MutationBuilder, QueryBuilder, Id};
use derive_builder::Builder;
use futures::TryStreamExt;
use toolkit::types::Uuid;
use toolkit::anyhow;
use toolkit::record_pagination::{RecordCursor, RecordPage, RecordQuery};
use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Debug, Clone, Copy)]
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
        }
    }

    fn as_graphql(&self) -> queststate_graphql::QuestStateInput {
        queststate_graphql::QuestStateInput {
            character_id: self.character_id,
            quest_id: self.quest_id,
            state: self.state.into(),
            conditions: self.conditions.iter().copied().map(Into::into).collect(),
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
}

pub(crate) mod queststate_graphql {
    use toolkit::types::Uuid;

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
    pub struct DeleteQueststate {
        #[arguments(id: $id)]
        pub delete_quest_state: Option<QuestState>,
    }
}
