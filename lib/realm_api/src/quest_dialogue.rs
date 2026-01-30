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

use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use derive_builder::Builder;
use toolkit::record_pagination::{RecordCursor, RecordPage, RecordQuery};

use crate::{CombatStyle, RealmApi, RealmApiError, RealmApiResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Close,
    Approve,
    Reject,
    Next,
    TellMore,
    Offer,
}

impl From<quest_dialogue_graphql::Choice> for Choice {
    fn from(value: quest_dialogue_graphql::Choice) -> Self {
        match value {
            quest_dialogue_graphql::Choice::Close => Self::Close,
            quest_dialogue_graphql::Choice::Approve => Self::Approve,
            quest_dialogue_graphql::Choice::Reject => Self::Reject,
            quest_dialogue_graphql::Choice::Next => Self::Next,
            quest_dialogue_graphql::Choice::TellMore => Self::TellMore,
            quest_dialogue_graphql::Choice::Offer => Self::Offer,
        }
    }
}

impl From<Choice> for quest_dialogue_graphql::Choice {
    fn from(value: Choice) -> Self {
        match value {
            Choice::Close => Self::Close,
            Choice::Approve => Self::Approve,
            Choice::Reject => Self::Reject,
            Choice::Next => Self::Next,
            Choice::TellMore => Self::TellMore,
            Choice::Offer => Self::Offer,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogueLine {
    pub line_id: i32,
    pub animation_name: Option<String>,
    pub choice: Option<Choice>,
    pub quest_id: Option<i32>,
}

impl DialogueLine {
    fn from_graphql(other: quest_dialogue_graphql::DialogueLine) -> Self {
        Self {
            line_id: other.line_id,
            animation_name: other.animation_name,
            choice: other.choice.map(Choice::from),
            quest_id: other.quest_id,
        }
    }

    fn as_graphql(&self) -> quest_dialogue_graphql::DialogueLineInput<'_> {
        quest_dialogue_graphql::DialogueLineInput {
            line_id: self.line_id,
            animation_name: self.animation_name.as_deref(),
            choice: self.choice.map(|c| c.into()),
            quest_id: self.quest_id,
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct DialogueBranchSelector {
    pub quests_available: Vec<i32>,
    pub quests_in_progress: Vec<i32>,
    pub quests_complete: Vec<i32>,
    pub quests_finished: Vec<i32>,
    pub level: i32,
    pub combat_style: Option<CombatStyle>,
}

impl DialogueBranchSelector {
    fn from_graphql(other: quest_dialogue_graphql::DialogueBranchSelector) -> Self {
        Self {
            quests_available: other.quests_available,
            quests_in_progress: other.quests_in_progress,
            quests_complete: other.quests_complete,
            quests_finished: other.quests_finished,
            level: other.level,
            combat_style: other.combat_style.map(|cs| cs.into()),
        }
    }

    fn as_graphql(&self) -> quest_dialogue_graphql::DialogueBranchSelectorInput {
        quest_dialogue_graphql::DialogueBranchSelectorInput {
            quests_available: self.quests_available.clone(),
            quests_in_progress: self.quests_in_progress.clone(),
            quests_complete: self.quests_complete.clone(),
            quests_finished: self.quests_finished.clone(),
            level: self.level,
            combat_style: self.combat_style.map(|cs| cs.into()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DialogueBranch {
    pub selector: Option<DialogueBranchSelector>,
    pub lines: Vec<DialogueLine>,
}

impl DialogueBranch {
    fn from_graphql(other: quest_dialogue_graphql::DialogueBranch) -> Self {
        Self {
            selector: other.selector.map(DialogueBranchSelector::from_graphql),
            lines: other.lines.into_iter().map(DialogueLine::from_graphql).collect(),
        }
    }

    fn as_graphql(&self) -> quest_dialogue_graphql::DialogueBranchInput<'_> {
        quest_dialogue_graphql::DialogueBranchInput {
            selector: self.selector.as_ref().map(DialogueBranchSelector::as_graphql),
            lines: self.lines.iter().map(DialogueLine::as_graphql).collect(),
        }
    }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct QuestDialogueQuery {
    #[builder(private)]
    api_base: RealmApi,
}

impl RecordQuery for QuestDialogueQuery {
    type Record = QuestDialogue;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(quest_dialogue_graphql::GetQuestDialogues::build(quest_dialogue_graphql::GetQuestDialoguesVariables {
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(quest_dialogue_graphql::GetQuestDialogues { quest_dialogues }) = response.data {
            Ok(RecordPage {
                at_end: !quest_dialogues.page_info.has_next_page,
                last_cursor: quest_dialogues.page_info.end_cursor,
                records: quest_dialogues.nodes.into_iter()
                    .map(|d| QuestDialogue::from_graphql(&self.api_base, d))
                    .collect(),
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl QuestDialogueQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<QuestDialogueQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct QuestDialogue {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    pub id: i32,
    #[builder(default)]
    pub branches: Vec<DialogueBranch>,
}

impl QuestDialogue {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(quest_dialogue_graphql::DeleteQuestDialogue::build(quest_dialogue_graphql::DeleteQuestDialogueVariables {
                    id: self.id
                })).await?;

            if let Some(quest_dialogue_graphql::DeleteQuestDialogue { .. }) = response.data {
                Ok(())
            } else if let Some(errors) = response.errors {
                Err(RealmApiError::GraphQl(errors))
            } else {
                unreachable!()
            }
        } else {
            Ok(())
        }
    }

    fn from_graphql(api: &RealmApi, other: quest_dialogue_graphql::QuestDialogue) -> Self {
        Self {
            api_base: Some(api.clone()),
            id: other.id,
            branches: other.branches.into_iter().map(DialogueBranch::from_graphql).collect(),
        }
    }

    fn as_graphql(&self) -> quest_dialogue_graphql::QuestDialogueInput<'_> {
        quest_dialogue_graphql::QuestDialogueInput {
            id: self.id,
            branches: self.branches.iter().map(DialogueBranch::as_graphql).collect(),
        }
    }
}

impl RealmApi {
    pub async fn get_quest_dialogue(&self, id: i32) -> RealmApiResult<Option<QuestDialogue>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_dialogue_graphql::GetQuestDialogue::build(quest_dialogue_graphql::GetQuestDialogueVariables {
                id
            })).await?;

        if let Some(quest_dialogue_graphql::GetQuestDialogue { quest_dialogue }) = response.data {
            if let Some(dialogue) = quest_dialogue {
                Ok(Some(QuestDialogue::from_graphql(self, dialogue)))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_quest_dialogues(&self) -> QuestDialogueQueryBuilder {
        QuestDialogueQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_quest_dialogue(&self, dialogue: QuestDialogue) -> RealmApiResult<QuestDialogue> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_dialogue_graphql::CreateQuestDialogue::build(quest_dialogue_graphql::CreateQuestDialogueVariables {
                input: dialogue.as_graphql()
            })).await?;

        if let Some(quest_dialogue_graphql::CreateQuestDialogue { create_quest_dialogue }) = response.data {
            Ok(QuestDialogue::from_graphql(self, create_quest_dialogue))
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_quest_dialogues(&self, dialogues: Vec<QuestDialogue>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_dialogue_graphql::BatchCreateQuestDialogues::build(quest_dialogue_graphql::BatchCreateQuestDialoguesVariables {
                input: dialogues.iter()
                    .map(|d| d.as_graphql())
                    .collect()
            })).await?;

        if let Some(quest_dialogue_graphql::BatchCreateQuestDialogues { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod quest_dialogue_graphql {
    use crate::schema::*;
    use crate::character::character_graphql::CombatStyle;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetQuestDialoguesVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetQuestDialogueVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateQuestDialogueVariables<'a> {
        pub input: QuestDialogueInput<'a>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateQuestDialoguesVariables<'a> {
        pub input: Vec<QuestDialogueInput<'a>>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteQuestDialogueVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetQuestDialoguesVariables")]
    pub struct GetQuestDialogues {
        #[arguments(after: $after, first: $first)]
        pub quest_dialogues: QuestDialogueConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetQuestDialogueVariables")]
    pub struct GetQuestDialogue {
        #[arguments(id: $id)]
        pub quest_dialogue: Option<QuestDialogue>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestDialogueConnection {
        pub nodes: Vec<QuestDialogue>,
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
    pub struct QuestDialogue {
        pub id: i32,
        pub branches: Vec<DialogueBranch>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueBranch {
        pub selector: Option<DialogueBranchSelector>,
        pub lines: Vec<DialogueLine>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueBranchSelector {
        pub quests_available: Vec<i32>,
        pub quests_in_progress: Vec<i32>,
        pub quests_complete: Vec<i32>,
        pub quests_finished: Vec<i32>,
        pub level: i32,
        pub combat_style: Option<CombatStyle>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueLine {
        pub line_id: i32,
        pub animation_name: Option<String>,
        pub choice: Option<Choice>,
        pub quest_id: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteQuestDialogueVariables")]
    pub struct DeleteQuestDialogue {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_quest_dialogue: Option<QuestDialogue>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateQuestDialogueVariables")]
    pub struct CreateQuestDialogue {
        #[arguments(input: $input)]
        pub create_quest_dialogue: QuestDialogue,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateQuestDialoguesVariables")]
    pub struct BatchCreateQuestDialogues {
        #[arguments(input: $input)]
        #[allow(dead_code)]
        pub batch_create_quest_dialogues: Vec<QuestDialogue>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestDialogueInput<'a> {
        pub id: i32,
        pub branches: Vec<DialogueBranchInput<'a>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueBranchInput<'a> {
        pub selector: Option<DialogueBranchSelectorInput>,
        pub lines: Vec<DialogueLineInput<'a>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueBranchSelectorInput {
        pub quests_available: Vec<i32>,
        pub quests_in_progress: Vec<i32>,
        pub quests_complete: Vec<i32>,
        pub quests_finished: Vec<i32>,
        pub level: i32,
        pub combat_style: Option<CombatStyle>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueLineInput<'a> {
        pub line_id: i32,
        pub animation_name: Option<&'a str>,
        pub choice: Option<Choice>,
        pub quest_id: Option<i32>,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum Choice {
        Close,
        Approve,
        Reject,
        Next,
        TellMore,
        Offer,
    }
}
