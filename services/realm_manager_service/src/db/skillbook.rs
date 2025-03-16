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
use mongodb::{bson::{self, doc}, options::{IndexOptions, ReturnDocument}, Database, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};
use anyhow::anyhow;

use crate::error::RealmResult;

use super::{CombatStyle, ObjectTemplate};

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unqualified,
    Locked,
    Unlocked,
}

#[derive(SimpleObject, InputObject, Serialize, Deserialize, Clone)]
#[graphql(input_name = "SkillbookEntryInput", name = "SkillbookEntry")]
pub struct Entry {
    pub id: Uuid,
    pub ability_id: Uuid,
    pub group: String,
    pub required_level: i32,
    pub state: State,
    pub unlock_cost: Option<i32>,
    pub stance: i32,
}

#[derive(Serialize, Deserialize, GraphqlCrud, Clone)]
#[graphql_crud(name = "skillbook")]
pub struct Skillbook {
    pub character_id: Uuid,
    pub combat_style: CombatStyle,
    pub character_level: i32,
    pub skills: Vec<Entry>,
}

impl DatabaseRecord for Skillbook {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.character_id
    }

    fn key_name() -> &'static str {
        "character_id"
    }

    fn collection_name() -> &'static str {
        "skillbook"
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

impl Skillbook {   
    pub fn new(character_id: Uuid) -> Skillbook {
        Skillbook {
            character_id,
            character_level: 0,
            combat_style: CombatStyle::None,
            skills: Vec::new(),
        }
    }

    pub async fn get_or_create(db: &mongodb::Database, character_id: Uuid) -> database::DBResult<Skillbook> {
        let collection = Self::collection(db);

        let storage = collection.find_one_and_update(doc! { "character_id": character_id }, 
        doc!{
                "$setOnInsert": bson::to_bson(&Skillbook::new(character_id)).unwrap()
            })
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or_else(|| anyhow!("upsert failed"))?;

        Ok(storage)
    }

    pub async fn change_class(&mut self, db: &Database, style: CombatStyle, level: i32) -> RealmResult<()> {
        self.combat_style = style;
        self.character_level = level;

        self.skills.clear();

        let definition = style.load_definition().await?;

        for skill in definition.skills {

            if let Some(ability) = ObjectTemplate::collection(db)
                .find_one(doc! { "name": skill.ability }).await? 
            {
                self.skills.push(Entry {
                    id: Uuid::new(),
                    ability_id: ability.id,
                    group: skill.group,
                    required_level: skill.level,
                    state: if self.character_level < skill.level {
                        State::Unqualified
                    } else if skill.upgrade_cost.is_some() {
                        State::Locked
                    } else {
                        State::Unlocked
                    },
                    unlock_cost: skill.upgrade_cost,
                    stance: skill.stance,
                });
            }
        }

        Ok(())
    }

    pub fn level_up(&mut self, level: i32) {
        for skill in self.skills.iter_mut() {
            if skill.state == State::Unqualified && level >= skill.required_level {
                skill.state = State::Locked;
            }
        }
    }

    pub fn unlock_ability(&mut self, id: Uuid) -> Option<i32> {
        for skill in self.skills.iter_mut() {
            if skill.id == id && skill.unlock_cost.is_some() {
                skill.state = State::Unlocked;
                return skill.unlock_cost;
            }
        }

        None
    }
}