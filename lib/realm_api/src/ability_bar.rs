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

use ability_bar_graphql::{AbilityBarInput, AbilitySlotInput, GetOrCreateAbilityBar, GetOrCreateAbilityBarVariables, UpdateAbilityBar, UpdateAbilityBarVariables};
use cynic::{http::ReqwestExt, MutationBuilder};
use toolkit::types::Uuid;

use crate::{RealmApi, RealmApiError, RealmApiResult};

pub struct AbilitySlot {
    pub id: i32,
    pub ability: String,
}

pub struct AbilityBar {
    api_base: RealmApi,

    pub character_id: Uuid,
    pub single_slot: AbilitySlot,
    pub slots: Vec<AbilitySlot>,
}

impl AbilityBar {
    fn from_graphql(api: &RealmApi, other: ability_bar_graphql::AbilityBar) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: api.clone(),
            character_id: other.character_id,
            single_slot: AbilitySlot { 
                id: other.single_slot.id, 
                ability: other.single_slot.ability, 
            },
            slots: other.slots.into_iter()
                .map(|s| AbilitySlot {
                    id: s.id,
                    ability: s.ability,
                })
                .collect()
        })
    }

    fn as_graphql(&self) -> AbilityBarInput {
        AbilityBarInput {
            character_id: self.character_id,
            single_slot: AbilitySlotInput {
                id: self.single_slot.id,
                ability: &self.single_slot.ability,
            },
            slots: self.slots.iter()
                .map(|s| AbilitySlotInput {
                    id: s.id,
                    ability: &s.ability,
                })
                .collect()
        }
    }

    pub async fn save(&self) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(UpdateAbilityBar::build(UpdateAbilityBarVariables {
                id: self.character_id,
                input: self.as_graphql(),
            })).await?;

        if let Some(UpdateAbilityBar { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl RealmApi {
    pub fn create_empty_ability_bar(&self, character_id: Uuid) -> AbilityBar {
        AbilityBar {
            api_base: self.clone(),
            character_id,
            single_slot: AbilitySlot { id: -1, ability: String::default() },
            slots: vec![]
        }
    }

    pub async fn get_or_create_ability_bar(&self, character_id: Uuid) -> RealmApiResult<AbilityBar> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetOrCreateAbilityBar::build(GetOrCreateAbilityBarVariables {
                character_id
            })).await?;

        if let Some(GetOrCreateAbilityBar { get_or_create_ability_bar }) = response.data {
            Ok(AbilityBar::from_graphql(self, get_or_create_ability_bar)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod ability_bar_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetOrCreateAbilityBarVariables {
        pub character_id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateAbilityBarVariables<'a> {
        pub id: Uuid,
        pub input: AbilityBarInput<'a>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[allow(dead_code)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "UpdateAbilityBarVariables")]
    pub struct UpdateAbilityBar {
        #[arguments(id: $id, input: $input)]
        pub update_ability_bar: Option<AbilityBar>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "GetOrCreateAbilityBarVariables")]
    pub struct GetOrCreateAbilityBar {
        #[arguments(characterId: $character_id)]
        pub get_or_create_ability_bar: AbilityBar,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AbilityBar {
        pub character_id: Uuid,
        pub single_slot: AbilitySlot,
        pub slots: Vec<AbilitySlot>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AbilitySlot {
        pub ability: String,
        pub id: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AbilityBarInput<'a> {
        pub character_id: Uuid,
        pub single_slot: AbilitySlotInput<'a>,
        pub slots: Vec<AbilitySlotInput<'a>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct AbilitySlotInput<'a> {
        pub id: i32,
        pub ability: &'a str,
    }
}