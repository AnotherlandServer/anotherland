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

use cynic::{http::ReqwestExt, MutationBuilder};
use skillbook_graphql::{GetOrCreateSkillbook, GetOrCreateSkillbookVariables};
use toolkit::types::Uuid;

use crate::{CombatStyle, RealmApi, RealmApiError, RealmApiResult};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Unqualified,
    Locked,
    Unlocked,
}

impl From<skillbook_graphql::State> for State {
    fn from(state: skillbook_graphql::State) -> Self {
        match state {
            skillbook_graphql::State::Unqualified => State::Unqualified,
            skillbook_graphql::State::Locked => State::Locked,
            skillbook_graphql::State::Unlocked => State::Unlocked,
        }
    }
}

impl From<State> for skillbook_graphql::State {
    fn from(state: State) -> Self {
        match state {
            State::Unqualified => skillbook_graphql::State::Unqualified,
            State::Locked => skillbook_graphql::State::Locked,
            State::Unlocked => skillbook_graphql::State::Unlocked,
        }
    }
}

pub struct Entry {
    pub id: Uuid,
    pub ability_id: Uuid,
    pub group: String,
    pub required_level: i32,
    pub state: State,
    pub unlock_cost: Option<i32>,
    pub stance: i32,
}

pub struct Skillbook {
    api_base: RealmApi,

    pub character_id: Uuid,
    pub combat_style: CombatStyle,
    pub character_level: i32,
    pub skills: Vec<Entry>,
}

impl Skillbook {
    fn from_graphql(api_base: &RealmApi, skillbook: skillbook_graphql::Skillbook) -> Self {
        Skillbook {
            api_base: api_base.clone(),
            character_id: skillbook.character_id,
            combat_style: skillbook.combat_style.into(),
            character_level: skillbook.character_level,
            skills: skillbook.skills.into_iter()
                .map(Entry::from_graphql)
                .collect(),
        }
    }

    pub async fn change_class(&mut self, combat_style: CombatStyle, level: Option<i32>) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(skillbook_graphql::SkillbookChangeClass::build(
                skillbook_graphql::SkillbookChangeClassVariables {
                    character_id: self.character_id,
                    combat_style: combat_style.into(),
                    level,
                }
            )).await?;

        if let Some(skillbook_graphql::SkillbookChangeClass { skillbook_change_class }) = response.data {
            if let Some(skillbook) = skillbook_change_class {
                *self = Skillbook::from_graphql(&self.api_base, skillbook);
            }

            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn level_up(&mut self, level: i32) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(skillbook_graphql::SkillbookLevelUp::build(
                skillbook_graphql::SkillbookLevelUpVariables {
                    character_id: self.character_id,
                    level,
                }
            )).await?;

        if let Some(skillbook_graphql::SkillbookLevelUp { skillbook_level_up }) = response.data {
            if let Some(skillbook) = skillbook_level_up {
                *self = Skillbook::from_graphql(&self.api_base, skillbook);
            }

            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn unlock_ability(&mut self, ability_id: Uuid) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(skillbook_graphql::SkillbookUnlockAbility::build(
                skillbook_graphql::SkillbookUnlockAbilityVariables {
                    character_id: self.character_id,
                    ability_id,
                }
            )).await?;

        if let Some(skillbook_graphql::SkillbookUnlockAbility { skillbook_unlock_ability }) = response.data {
            if let Some(skillbook) = skillbook_unlock_ability {
                *self = Skillbook::from_graphql(&self.api_base, skillbook);
            }

            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn unlock_all(&mut self) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(skillbook_graphql::SkillbookUnlockAll::build(
                skillbook_graphql::SkillbookUnlockAllVariables {
                    character_id: self.character_id,
                }
            )).await?;

        if let Some(skillbook_graphql::SkillbookUnlockAll { skillbook_unlock_all }) = response.data {
            if let Some(skillbook) = skillbook_unlock_all {
                *self = Skillbook::from_graphql(&self.api_base, skillbook);
            }

            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl Entry {
    fn from_graphql(entry: skillbook_graphql::SkillbookEntry) -> Self {
        Entry {
            id: entry.id,
            ability_id: entry.ability_id,
            group: entry.group,
            required_level: entry.required_level,
            state: entry.state.into(),
            unlock_cost: entry.unlock_cost,
            stance: entry.stance,
        }
    }
}

impl RealmApi {
    pub async fn get_or_create_skillbook(&self, character_id: Uuid) -> RealmApiResult<Skillbook> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetOrCreateSkillbook::build(GetOrCreateSkillbookVariables {
                character_id
            })).await?;

        if let Some(GetOrCreateSkillbook { get_or_create_skillbook }) = response.data {
            Ok(Skillbook::from_graphql(self, get_or_create_skillbook))
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod skillbook_graphql {
    use toolkit::types::Uuid;

    use crate::{character_graphql::CombatStyle, schema::*};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SkillbookUnlockAbilityVariables {
        pub ability_id: Uuid,
        pub character_id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetOrCreateSkillbookVariables {
        pub character_id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SkillbookLevelUpVariables {
        pub character_id: Uuid,
        pub level: i32,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SkillbookUnlockAllVariables {
        pub character_id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SkillbookChangeClassVariables {
        pub character_id: Uuid,
        pub combat_style: CombatStyle,
        pub level: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "SkillbookUnlockAllVariables")]
    pub struct SkillbookUnlockAll {
        #[arguments(characterId: $character_id)]
        pub skillbook_unlock_all: Option<Skillbook>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "SkillbookUnlockAbilityVariables")]
    pub struct SkillbookUnlockAbility {
        #[arguments(abilityId: $ability_id, characterId: $character_id)]
        pub skillbook_unlock_ability: Option<Skillbook>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "SkillbookLevelUpVariables")]
    pub struct SkillbookLevelUp {
        #[arguments(characterId: $character_id, level: $level)]
        pub skillbook_level_up: Option<Skillbook>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "SkillbookChangeClassVariables")]
    pub struct SkillbookChangeClass {
        #[arguments(characterId: $character_id, combatStyle: $combat_style, level: $level)]
        pub skillbook_change_class: Option<Skillbook>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "GetOrCreateSkillbookVariables")]
    pub struct GetOrCreateSkillbook {
        #[arguments(characterId: $character_id)]
        pub get_or_create_skillbook: Skillbook,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Skillbook {
        pub character_id: Uuid,
        pub character_level: i32,
        pub combat_style: CombatStyle,
        pub skills: Vec<SkillbookEntry>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct SkillbookEntry {
        pub ability_id: Uuid,
        pub group: String,
        pub id: Uuid,
        pub required_level: i32,
        pub state: State,
        pub unlock_cost: Option<i32>,
        pub stance: i32,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum State {
        Unqualified,
        Locked,
        Unlocked,
    }
}