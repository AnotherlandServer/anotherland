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

use async_graphql::{ComplexObject, InputObject, Interface, OneofObject, SimpleObject};
use database::DatabaseRecord;
use mongodb::{Database, IndexModel, bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use toolkit::{GraphqlCrud, types::Uuid};

use crate::db::CombatStyle;

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "QuestTemplate")]
pub struct QuestTemplate {
    pub id: i32,
    pub chain_id: Option<i32>,
    pub level: i32,

    #[graphql_crud(filter)]
    pub world_id: u16,
    
    pub exp_reward: Option<i32>,
    pub bit_reward: Option<i32>,
    pub available_dialogue_id: Option<i32>,
    pub progress_dialogue_id: Option<i32>,
    pub completion_dialogue_id: Option<i32>,
    pub prerequisites: Option<Prerequisites>,
    pub conditions: Vec<Condition>,
}

impl DatabaseRecord for QuestTemplate {
    type PrimaryKey = i32;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "quest_templates"
    }
    
    async fn build_index(db: &Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()
        ).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "world_id": 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build()
        ).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "PrerequisitesInput")]
pub struct Prerequisites {
    pub level: Option<i32>,
    pub combat_style: Option<CombatStyle>,
    pub quests_finished: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct AvatarSelectorOutput {
    pub content_id: Option<Uuid>,
    pub instance_id: Option<Uuid>,
    pub quest_tag: Option<i32>,
    pub loot_item: Option<Uuid>,
    pub dialog_id: Option<i32>,
}

impl From<AvatarSelector> for AvatarSelectorOutput {
    fn from(other: AvatarSelector) -> Self {
        match other {
            AvatarSelector::ContentId(id) => AvatarSelectorOutput {
                content_id: Some(id),
                instance_id: None,
                quest_tag: None,
                loot_item: None,
                dialog_id: None,
            },
            AvatarSelector::InstanceId(id) => AvatarSelectorOutput {
                content_id: None,
                instance_id: Some(id),
                quest_tag: None,
                loot_item: None,
                dialog_id: None,
            },
            AvatarSelector::QuestTag(tag) => AvatarSelectorOutput {
                content_id: None,
                instance_id: None,
                quest_tag: Some(tag),
                loot_item: None,
                dialog_id: None,
            },
            AvatarSelector::LootItem(item) => AvatarSelectorOutput {
                content_id: None,
                instance_id: None,
                quest_tag: None,
                loot_item: Some(item),
                dialog_id: None,
            },
            AvatarSelector::DialogId(id) => AvatarSelectorOutput {
                content_id: None,
                instance_id: None,
                quest_tag: None,
                loot_item: None,
                dialog_id: Some(id),
            },
        }
    }
}

#[derive(Serialize, Deserialize, OneofObject, Clone)]
pub enum AvatarSelector {
    ContentId(Uuid),
    InstanceId(Uuid),
    QuestTag(i32),
    LootItem(Uuid),
    DialogId(i32),
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "InteractConditionInput")]
pub struct InteractCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub beacon: Option<Uuid>,
    pub required_count: i32,
    #[graphql(skip_output)]
    pub avatar_selector: AvatarSelector,
}

#[ComplexObject]
impl InteractCondition {
    async fn avatar_selector(&self) -> AvatarSelectorOutput {
        self.avatar_selector.clone().into()
    }
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "DialogueConditionInput")]
pub struct DialogueCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub beacon: Option<Uuid>,
    pub required_count: i32,
    pub dialogue_id: i32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "RemovedConditionInput")]
pub struct RemovedCondition {
    pub id: i32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "WaitConditionInput")]
pub struct WaitCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub wait_time_seconds: f32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "KillConditionInput")]
pub struct KillCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub beacon: Option<Uuid>,
    pub required_count: i32,
    #[graphql(skip_output)]
    pub avatar_selector: AvatarSelector,
}

#[ComplexObject]
impl KillCondition {
    async fn avatar_selector(&self) -> AvatarSelectorOutput {
        self.avatar_selector.clone().into()
    }
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "LootConditionInput")]
pub struct LootCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub beacon: Option<Uuid>,
    pub required_count: i32,
    pub item_name: String,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "ProximityConditionInput")]
pub struct ProximityCondition {
    pub id: i32,
    pub stage: i32,
    pub hidden: bool,
    pub beacon: Option<Uuid>,
    pub required_count: i32,
    #[graphql(skip_output)]
    pub avatar_selector: AvatarSelector,
    pub radius: f32,
}

#[ComplexObject]
impl ProximityCondition {
    async fn avatar_selector(&self) -> AvatarSelectorOutput {
        self.avatar_selector.clone().into()
    }
}

#[derive(Serialize, Deserialize, Interface, OneofObject)]
#[serde(tag = "type")]
#[graphql(
    field(name = "id", ty = "&i32"),
    input_name = "ConditionInput"
)]
pub enum Condition {
    Interact(InteractCondition),
    Dialogue(DialogueCondition),
    Wait(WaitCondition),
    Kill(KillCondition),
    Loot(LootCondition),
    Proximity(ProximityCondition),
}