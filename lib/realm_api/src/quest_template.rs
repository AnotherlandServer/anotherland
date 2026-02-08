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
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{CombatStyle, RealmApi, RealmApiError, RealmApiResult};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AvatarSelector {
    ContentId(Uuid),
    InstanceId(Uuid),
    QuestTag(i32),
    LootItem(Uuid),
    DialogId(i32),
}

impl AvatarSelector {
    fn from_graphql(other: quest_template_graphql::AvatarSelectorOutput) -> RealmApiResult<Self> {
        if let Some(content_id) = other.content_id {
            Ok(Self::ContentId(content_id))
        } else if let Some(instance_id) = other.instance_id {
            Ok(Self::InstanceId(instance_id))
        } else if let Some(quest_tag) = other.quest_tag {
            Ok(Self::QuestTag(quest_tag))
        } else if let Some(loot_item) = other.loot_item {
            Ok(Self::LootItem(loot_item))
        } else if let Some(dialog_id) = other.dialog_id {
            Ok(Self::DialogId(dialog_id))
        } else {
            Err(RealmApiError::Other(toolkit::anyhow::anyhow!("Invalid AvatarSelector: no field set")))
        }
    }

    pub(crate) fn as_graphql(&self) -> quest_template_graphql::AvatarSelector {
        match self {
            Self::ContentId(id) => quest_template_graphql::AvatarSelector {
                content_id: Some(*id),
                instance_id: None,
                quest_tag: None,
                loot_item: None,
                dialog_id: None,
            },
            Self::InstanceId(id) => quest_template_graphql::AvatarSelector {
                content_id: None,
                instance_id: Some(*id),
                quest_tag: None,
                loot_item: None,
                dialog_id: None,
            },
            Self::QuestTag(tag) => quest_template_graphql::AvatarSelector {
                content_id: None,
                instance_id: None,
                quest_tag: Some(*tag),
                loot_item: None,
                dialog_id: None,
            },
            Self::LootItem(id) => quest_template_graphql::AvatarSelector {
                content_id: None,
                instance_id: None,
                quest_tag: None,
                loot_item: Some(*id),
                dialog_id: None,
            },
            Self::DialogId(id) => quest_template_graphql::AvatarSelector {
                content_id: None,
                instance_id: None,
                quest_tag: None,
                loot_item: None,
                dialog_id: Some(*id),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Interact {
        id: i32,
        stage: i32,
        hidden: bool,
        beacon: Option<Uuid>,
        required_count: i32,
        avatar_selector: AvatarSelector,
    },
    Dialogue {
        id: i32,
        stage: i32,
        hidden: bool,
        beacon: Option<Uuid>,
        required_count: i32,
        dialogue_id: i32,
    },
    Wait {
        id: i32,
        stage: i32,
        hidden: bool,
        wait_time_seconds: f64,
    },
    Kill {
        id: i32,
        stage: i32,
        hidden: bool,
        beacon: Option<Uuid>,
        required_count: i32,
        avatar_selector: AvatarSelector,
    },
    Loot {
        id: i32,
        stage: i32,
        hidden: bool,
        beacon: Option<Uuid>,
        required_count: i32,
        item_id: Uuid,
    },
    Proximity {
        id: i32,
        stage: i32,
        hidden: bool,
        beacon: Option<Uuid>,
        required_count: i32,
        avatar_selector: AvatarSelector,
        radius: f64,
    },
}

impl Condition {
    pub fn id(&self) -> i32 {
        match self {
            Self::Interact { id, .. } => *id,
            Self::Dialogue { id, .. } => *id,
            Self::Wait { id, .. } => *id,
            Self::Kill { id, .. } => *id,
            Self::Loot { id, .. } => *id,
            Self::Proximity { id, .. } => *id,
        }
    }

    fn from_graphql(other: quest_template_graphql::ConditionInterface) -> RealmApiResult<Self> {
        match other {
            quest_template_graphql::ConditionInterface::InteractCondition(c) => {
                Ok(Self::Interact {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    beacon: c.beacon,
                    required_count: c.required_count,
                    avatar_selector: AvatarSelector::from_graphql(c.avatar_selector)?,
                })
            },
            quest_template_graphql::ConditionInterface::DialogueCondition(c) => {
                Ok(Self::Dialogue {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    beacon: c.beacon,
                    required_count: c.required_count,
                    dialogue_id: c.dialogue_id,
                })
            },
            quest_template_graphql::ConditionInterface::WaitCondition(c) => {
                Ok(Self::Wait {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    wait_time_seconds: c.wait_time_seconds,
                })
            },
            quest_template_graphql::ConditionInterface::KillCondition(c) => {
                Ok(Self::Kill {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    beacon: c.beacon,
                    required_count: c.required_count,
                    avatar_selector: AvatarSelector::from_graphql(c.avatar_selector)?,
                })
            },
            quest_template_graphql::ConditionInterface::LootCondition(c) => {
                Ok(Self::Loot {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    beacon: c.beacon,
                    required_count: c.required_count,
                    item_id: c.item_id,
                })
            },
            quest_template_graphql::ConditionInterface::ProximityCondition(c) => {
                Ok(Self::Proximity {
                    id: c.id,
                    stage: c.stage,
                    hidden: c.hidden,
                    beacon: c.beacon,
                    required_count: c.required_count,
                    avatar_selector: AvatarSelector::from_graphql(c.avatar_selector)?,
                    radius: c.radius,
                })
            },
            quest_template_graphql::ConditionInterface::Unknown => {
                Err(RealmApiError::Other(toolkit::anyhow::anyhow!("Unknown Condition type")))
            }
        }
    }

    fn as_graphql(&self) -> quest_template_graphql::ConditionInput {
        match *self {
            Self::Interact { id, stage, hidden, beacon, required_count, avatar_selector } => quest_template_graphql::ConditionInput {
                interact: Some(quest_template_graphql::InteractConditionInput {
                    id,
                    stage,
                    hidden, 
                    beacon,
                    required_count,
                    avatar_selector: avatar_selector.as_graphql(),
                }),
                dialogue: None,
                wait: None,
                kill: None,
                loot: None,
                proximity: None,
            },
            Self::Dialogue { id, stage, hidden, beacon, required_count, dialogue_id } => quest_template_graphql::ConditionInput {
                interact: None,
                dialogue: Some(quest_template_graphql::DialogueConditionInput {
                    id,
                    stage,
                    hidden,
                    beacon,
                    required_count,
                    dialogue_id,
                }),
                wait: None,
                kill: None,
                loot: None,
                proximity: None,
            },
            Self::Wait { id, stage, hidden, wait_time_seconds } => quest_template_graphql::ConditionInput {
                interact: None,
                dialogue: None,
                wait: Some(quest_template_graphql::WaitConditionInput {
                    id,
                    stage,
                    hidden, 
                    wait_time_seconds,
                }),
                kill: None,
                loot: None,
                proximity: None,
            },
            Self::Kill { id, stage, hidden, beacon, required_count, avatar_selector } => quest_template_graphql::ConditionInput {
                interact: None,
                dialogue: None,
                wait: None,
                kill: Some(quest_template_graphql::KillConditionInput {
                    id,
                    stage,
                    hidden, 
                    beacon,
                    required_count,
                    avatar_selector: avatar_selector.as_graphql(),
                }),
                loot: None,
                proximity: None,
            },
            Self::Loot { id, stage, hidden, beacon, required_count, item_id } => quest_template_graphql::ConditionInput {
                interact: None,
                dialogue: None,
                wait: None,
                kill: None,
                loot: Some(quest_template_graphql::LootConditionInput {
                    id,
                    stage,
                    hidden,
                    beacon,
                    required_count,
                    item_id,
                }),
                proximity: None,
            },
            Self::Proximity { id, stage, hidden, beacon, required_count, avatar_selector, radius } => quest_template_graphql::ConditionInput {
                interact: None,
                dialogue: None,
                wait: None,
                kill: None,
                loot: None,
                proximity: Some(quest_template_graphql::ProximityConditionInput {
                    id,
                    stage,
                    hidden, 
                    beacon,
                    required_count,
                    avatar_selector: avatar_selector.as_graphql(),
                    radius,
                }),
            }
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct Prerequisites {
    pub level: Option<i32>,
    pub combat_style: Option<CombatStyle>,
    pub quests_finished: Option<Vec<i32>>,
}

impl Prerequisites {
    fn from_graphql(other: quest_template_graphql::Prerequisites) -> Self {
        Self {
            level: other.level,
            combat_style: other.combat_style.map(|cs| cs.into()),
            quests_finished: other.quests_finished,
        }
    }

    fn as_graphql(&self) -> quest_template_graphql::PrerequisitesInput {
        quest_template_graphql::PrerequisitesInput {
            level: self.level,
            combat_style: self.combat_style.map(|cs| cs.into()),
            quests_finished: self.quests_finished.clone(),
        }
    }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct QuestTemplateQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    world_id: Option<u16>,
}

impl QuestTemplateQuery {
    fn get_filter(&self) -> Option<quest_template_graphql::QuestTemplateFilter> {
        if self.world_id.is_none() {
            None
        } else {
            Some(quest_template_graphql::QuestTemplateFilter { 
                world_id: self.world_id.map(|id| id as i32),
            })
        }
    }
}

impl RecordQuery for QuestTemplateQuery {
    type Record = QuestTemplate;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(quest_template_graphql::GetQuestTemplates::build(quest_template_graphql::GetQuestTemplatesVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(quest_template_graphql::GetQuestTemplates { quest_templates }) = response.data {
            Ok(RecordPage {
                at_end: !quest_templates.page_info.has_next_page,
                last_cursor: quest_templates.page_info.end_cursor,
                records: quest_templates.nodes.into_iter()
                    .map(|t| QuestTemplate::from_graphql(&self.api_base, t))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl QuestTemplateQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<QuestTemplateQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct QuestTemplate {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    pub id: i32,
    #[builder(setter(strip_option), default)]
    pub chain_id: Option<i32>,
    pub level: i32,
    pub world_id: i32,
    #[builder(setter(strip_option), default)]
    pub exp_reward: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub bit_reward: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub available_dialogue_id: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub progress_dialogue_id: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub completion_dialogue_id: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub prerequisites: Option<Prerequisites>,
    #[builder(default)]
    pub conditions: Vec<Condition>,
}

impl QuestTemplate {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(quest_template_graphql::DeleteQuestTemplate::build(quest_template_graphql::DeleteQuestTemplateVariables {
                    id: self.id
                })).await?;

            if let Some(quest_template_graphql::DeleteQuestTemplate { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: quest_template_graphql::QuestTemplate) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id,
            chain_id: other.chain_id,
            level: other.level,
            world_id: other.world_id,
            exp_reward: other.exp_reward,
            bit_reward: other.bit_reward,
            available_dialogue_id: other.available_dialogue_id,
            progress_dialogue_id: other.progress_dialogue_id,
            completion_dialogue_id: other.completion_dialogue_id,
            prerequisites: other.prerequisites.map(Prerequisites::from_graphql),
            conditions: other.conditions.into_iter()
                .map(Condition::from_graphql)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn as_graphql(&self) -> quest_template_graphql::QuestTemplateInput {
        quest_template_graphql::QuestTemplateInput {
            id: self.id,
            chain_id: self.chain_id,
            level: self.level,
            world_id: self.world_id,
            exp_reward: self.exp_reward,
            bit_reward: self.bit_reward,
            available_dialogue_id: self.available_dialogue_id,
            progress_dialogue_id: self.progress_dialogue_id,
            completion_dialogue_id: self.completion_dialogue_id,
            prerequisites: self.prerequisites.as_ref().map(Prerequisites::as_graphql),
            conditions: self.conditions.iter().map(Condition::as_graphql).collect(),
        }
    }
}

// ============================================================================
// RealmApi Extension Methods
// ============================================================================

impl RealmApi {
    pub async fn get_quest_template(&self, id: i32) -> RealmApiResult<Option<QuestTemplate>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_template_graphql::GetQuestTemplate::build(quest_template_graphql::GetQuestTemplateVariables {
                id
            })).await?;

        if let Some(quest_template_graphql::GetQuestTemplate { quest_template }) = response.data {
            if let Some(template) = quest_template {
                Ok(Some(QuestTemplate::from_graphql(self, template)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_quest_templates(&self) -> QuestTemplateQueryBuilder {
        QuestTemplateQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_quest_template(&self, template: QuestTemplate) -> RealmApiResult<QuestTemplate> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_template_graphql::CreateQuestTemplate::build(quest_template_graphql::CreateQuestTemplateVariables {
                input: template.as_graphql()
            })).await?;

        if let Some(quest_template_graphql::CreateQuestTemplate { create_quest_template }) = response.data {
            Ok(QuestTemplate::from_graphql(self, create_quest_template)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_quest_templates(&self, templates: Vec<QuestTemplate>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(quest_template_graphql::BatchCreateQuestTemplates::build(quest_template_graphql::BatchCreateQuestTemplatesVariables {
                input: templates.iter()
                    .map(|t| t.as_graphql())
                    .collect()
            })).await?;

        if let Some(quest_template_graphql::BatchCreateQuestTemplates { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod quest_template_graphql {
    use toolkit::types::Uuid;
    use crate::schema::*;
    use crate::character::character_graphql::CombatStyle;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetQuestTemplatesVariables<'a> {
        pub filter: Option<QuestTemplateFilter>,
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetQuestTemplateVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateQuestTemplateVariables {
        pub input: QuestTemplateInput,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateQuestTemplatesVariables {
        pub input: Vec<QuestTemplateInput>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteQuestTemplateVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetQuestTemplatesVariables")]
    pub struct GetQuestTemplates {
        #[arguments(filter: $filter, after: $after, first: $first)]
        pub quest_templates: QuestTemplateConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetQuestTemplateVariables")]
    pub struct GetQuestTemplate {
        #[arguments(id: $id)]
        pub quest_template: Option<QuestTemplate>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestTemplateConnection {
        pub nodes: Vec<QuestTemplate>,
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
    pub struct QuestTemplate {
        pub id: i32,
        pub chain_id: Option<i32>,
        pub level: i32,
        pub world_id: i32,
        pub exp_reward: Option<i32>,
        pub bit_reward: Option<i32>,
        pub available_dialogue_id: Option<i32>,
        pub progress_dialogue_id: Option<i32>,
        pub completion_dialogue_id: Option<i32>,
        pub prerequisites: Option<Prerequisites>,
        pub conditions: Vec<ConditionInterface>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Prerequisites {
        pub level: Option<i32>,
        pub combat_style: Option<CombatStyle>,
        pub quests_finished: Option<Vec<i32>>,
    }

    #[derive(cynic::InlineFragments, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "Condition")]
    pub enum ConditionInterface {
        InteractCondition(InteractCondition),
        DialogueCondition(DialogueCondition),
        WaitCondition(WaitCondition),
        KillCondition(KillCondition),
        LootCondition(LootCondition),
        ProximityCondition(ProximityCondition),
        #[cynic(fallback)]
        Unknown,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct InteractCondition {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub avatar_selector: AvatarSelectorOutput,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueCondition {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub dialogue_id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WaitCondition {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub wait_time_seconds: f64,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct KillCondition {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub avatar_selector: AvatarSelectorOutput,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct LootCondition {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub item_id: Uuid,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ProximityCondition {
         pub id: i32,
         pub stage: i32,
         pub hidden: bool,
         pub beacon: Option<Uuid>,
         pub required_count: i32,
         pub avatar_selector: AvatarSelectorOutput,
         pub radius: f64,
     }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AvatarSelectorOutput {
        pub content_id: Option<Uuid>,
        pub instance_id: Option<Uuid>,
        pub quest_tag: Option<i32>,
        pub loot_item: Option<Uuid>,
        pub dialog_id: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteQuestTemplateVariables")]
    pub struct DeleteQuestTemplate {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_quest_template: Option<QuestTemplate>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateQuestTemplateVariables")]
    pub struct CreateQuestTemplate {
        #[arguments(input: $input)]
        pub create_quest_template: QuestTemplate,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateQuestTemplatesVariables")]
    pub struct BatchCreateQuestTemplates {
        #[arguments(input: $input)]
        #[allow(dead_code)]
        pub batch_create_quest_templates: Vec<QuestTemplate>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestTemplateInput {
        pub id: i32,
        pub chain_id: Option<i32>,
        pub level: i32,
        pub world_id: i32,
        pub exp_reward: Option<i32>,
        pub bit_reward: Option<i32>,
        pub available_dialogue_id: Option<i32>,
        pub progress_dialogue_id: Option<i32>,
        pub completion_dialogue_id: Option<i32>,
        pub prerequisites: Option<PrerequisitesInput>,
        pub conditions: Vec<ConditionInput>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct QuestTemplateFilter {
        pub world_id: Option<i32>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PrerequisitesInput {
        pub level: Option<i32>,
        pub combat_style: Option<CombatStyle>,
        pub quests_finished: Option<Vec<i32>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ConditionInput {
        pub interact: Option<InteractConditionInput>,
        pub dialogue: Option<DialogueConditionInput>,
        pub wait: Option<WaitConditionInput>,
        pub kill: Option<KillConditionInput>,
        pub loot: Option<LootConditionInput>,
        pub proximity: Option<ProximityConditionInput>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct InteractConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub avatar_selector: AvatarSelector,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct DialogueConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub dialogue_id: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WaitConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub wait_time_seconds: f64,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct KillConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub avatar_selector: AvatarSelector,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct LootConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub item_id: Uuid,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ProximityConditionInput {
        pub id: i32,
        pub stage: i32,
        pub hidden: bool,
        pub beacon: Option<Uuid>,
        pub required_count: i32,
        pub avatar_selector: AvatarSelector,
        pub radius: f64,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AvatarSelector {
        #[cynic(skip_serializing_if="Option::is_none")]
        pub content_id: Option<Uuid>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub instance_id: Option<Uuid>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub quest_tag: Option<i32>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub loot_item: Option<Uuid>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub dialog_id: Option<i32>,
    }
}
