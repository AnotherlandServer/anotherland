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

use character_graphql::{CreateCharacterInAccount, CreateCharacterInAccountVariables, DeleteCharacter, DeleteCharacterVariables, GetAccountCharacter, GetAccountCharacterVariables, GetCharacter, GetCharacterVariables, GetCharactersForAccount, GetCharactersForAccountVariables, UpdateCharacterDataDiff, UpdateCharacterDataDiffVariables};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use log::debug;
use obj_params::{GameObjectData, GenericParamSet};
use toolkit::{anyhow::{self, anyhow}, types::Uuid};

use crate::{schema, EquipmentResult, RealmApi, RealmApiError, RealmApiResult};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CombatStyle {
    Rage,
    Tech,
    Assassin,
    Energizer,
    Hacker,
    Cyber,
    None,
}

impl From<character_graphql::CombatStyle> for CombatStyle {
    fn from(value: character_graphql::CombatStyle) -> Self {
        match value {
            character_graphql::CombatStyle::Rage => CombatStyle::Rage,
            character_graphql::CombatStyle::Tech => CombatStyle::Tech,
            character_graphql::CombatStyle::Assassin => CombatStyle::Assassin,
            character_graphql::CombatStyle::Energizer => CombatStyle::Energizer,
            character_graphql::CombatStyle::Hacker => CombatStyle::Hacker,
            character_graphql::CombatStyle::Cyber => CombatStyle::Cyber,
            character_graphql::CombatStyle::None => CombatStyle::None,
        }
    }
}

impl From<CombatStyle> for character_graphql::CombatStyle {
    fn from(value: CombatStyle) -> Self {
        match value {
            CombatStyle::Rage => character_graphql::CombatStyle::Rage,
            CombatStyle::Tech => character_graphql::CombatStyle::Tech,
            CombatStyle::Assassin => character_graphql::CombatStyle::Assassin,
            CombatStyle::Energizer => character_graphql::CombatStyle::Energizer,
            CombatStyle::Hacker => character_graphql::CombatStyle::Hacker,
            CombatStyle::Cyber => character_graphql::CombatStyle::Cyber,
            CombatStyle::None => character_graphql::CombatStyle::None,
        }
    }
}

impl From<CombatStyle> for i32 {
    fn from(style: CombatStyle) -> Self {
        match style {
            CombatStyle::Rage => 0,
            CombatStyle::Tech => 1,
            CombatStyle::Assassin => 2,
            CombatStyle::Energizer => 3,
            CombatStyle::Hacker => 4,
            CombatStyle::Cyber => 5,
            CombatStyle::None => 6,
        }
    }
}

impl TryFrom<i32> for CombatStyle {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CombatStyle::Rage),
            1 => Ok(CombatStyle::Tech),
            2 => Ok(CombatStyle::Assassin),
            3 => Ok(CombatStyle::Energizer),
            4 => Ok(CombatStyle::Hacker),
            5 => Ok(CombatStyle::Cyber),
            6 => Ok(CombatStyle::None),
            _ => Err(anyhow!("Invalid combat style value: {}", value)),
        }
    }
}

pub struct Character {
    api_base: RealmApi,

    id: Uuid,
    account: Uuid,
    index: i32,
    name: String,
    data: GameObjectData,
}

impl Character {
    pub fn id(&self) -> &Uuid { &self.id }
    pub fn account(&self) -> &Uuid { &self.account }
    pub fn index(&self) -> i32 { self.index }
    pub fn name(&self) -> &str { &self.name }
    pub fn data(&self) -> &GameObjectData { &self.data }
    pub fn data_mut(&mut self) -> &mut GameObjectData { &mut self.data }
    pub fn take_data(self) -> GameObjectData { self.data }

    pub async fn delete(&self) -> RealmApiResult<()> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(DeleteCharacter::build(DeleteCharacterVariables {
                id: self.id
            })).await?;

        if let Some(DeleteCharacter { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl Character {
    fn from_graphql(api: &RealmApi, other: character_graphql::Character) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: api.clone(),
            id: other.id,
            account: other.account,
            index: other.index,
            name: other.name,
            data: serde_json::from_value(other.data.0)?,
        })
    }
}

impl RealmApi {
    pub async fn get_character(&self, id: &Uuid) -> RealmApiResult<Option<Character>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetCharacter::build(GetCharacterVariables {
                id: *id
            })).await?;

        if let Some(GetCharacter { character }) = response.data {
            if let Some(character) = character {
                Ok(Some(Character::from_graphql(self, character)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_character_for_account(&self, account_id: &Uuid, index: i32) -> RealmApiResult<Option<Character>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetAccountCharacter::build(GetAccountCharacterVariables {
                account_id: *account_id,
                index,
            })).await?;

        if let Some(GetAccountCharacter { account_character }) = response.data {
            if let Some(character) = account_character {
                Ok(Some(Character::from_graphql(self, character)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn create_character(&self, account_id: &Uuid, name: String) -> RealmApiResult<Character> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateCharacterInAccount::build(CreateCharacterInAccountVariables {
                account: *account_id,
                name: &name,
            })).await?;

        if let Some(CreateCharacterInAccount { create_character_in_account }) = response.data {
            Ok(Character::from_graphql(self, create_character_in_account)?)
        } else if let Some(errors) = response.errors {
            debug!("Errors: {errors:#?}");
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_characters_for_account(&self, account_id: &Uuid) -> RealmApiResult<Vec<Character>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetCharactersForAccount::build(GetCharactersForAccountVariables {
                account_id: *account_id
            })).await?;

        if let Some(GetCharactersForAccount { characters_for_account }) = response.data {
            Ok(
                characters_for_account.into_iter()
                    .map(|c| Character::from_graphql(self, c).unwrap())
                    .collect()
            )
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn update_character_data_diff(&self, id: &Uuid, diff: Box<dyn GenericParamSet>) -> RealmApiResult<Option<Character>> {
        let params = schema::Json(serde_json::to_value(&diff)?);

        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(UpdateCharacterDataDiff::build(UpdateCharacterDataDiffVariables {
                id: *id,
                params,
            })).await?;

        if let Some(UpdateCharacterDataDiff { update_character_data_diff }) = response.data {
            if let Some(character) = update_character_data_diff {
                Ok(Some(Character::from_graphql(self, character)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            debug!("Errors: {errors:#?}");
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn character_apply_class_item(&self, id: &Uuid, class_item: &str, clear_inventory: bool) -> RealmApiResult<EquipmentResult> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(character_graphql::CharacterApplyClassItem::build(character_graphql::CharacterApplyClassItemVariables {
                id: *id,
                class_item: class_item.to_string(),
                clear_inventory,
            })).await?;

        if let Some(character_graphql::CharacterApplyClassItem { character_apply_class_item }) = response.data {
            EquipmentResult::from_graphql(self, character_apply_class_item)
        } else if let Some(errors) = response.errors {
            debug!("Errors: {errors:#?}");
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod character_graphql {
    use toolkit::types::Uuid;

    use crate::{item_storage_graphql::EquipmentResult, schema::*};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCharactersForAccountVariables {
        pub account_id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateCharacterInAccountVariables<'a> {
        pub account: Uuid,
        pub name: &'a str,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCharacterVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteCharacterVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCharactersForAccountVariables")]
    pub struct GetCharactersForAccount {
        #[arguments(accountId: $account_id)]
        pub characters_for_account: Vec<Character>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetAccountCharacterVariables {
        pub account_id: Uuid,
        pub index: i32,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetAccountCharacterVariables")]
    pub struct GetAccountCharacter {
        #[arguments(accountId: $account_id, index: $index)]
        pub account_character: Option<Character>,
    }

    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCharacterVariables")]
    pub struct GetCharacter {
        #[arguments(id: $id)]
        pub character: Option<Character>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateCharacterDataDiffVariables {
        pub id: Uuid,
        pub params: Json,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CharacterApplyClassItemVariables {
        pub id: Uuid,
        pub class_item: String,
        pub clear_inventory: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "UpdateCharacterDataDiffVariables")]
    pub struct UpdateCharacterDataDiff {
        #[arguments(id: $id, params: $params)]
        pub update_character_data_diff: Option<Character>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PremiumCurrencyBalance {
        #[allow(dead_code)]
        pub account_id: Uuid,
        #[allow(dead_code)]
        pub balance: i32,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteCharacterVariables")]
    pub struct DeleteCharacter {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_character: Option<Character>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateCharacterInAccountVariables")]
    pub struct CreateCharacterInAccount {
        #[arguments(input: { account: $account, name: $name })]
        pub create_character_in_account: Character,
    }
    
    //CharacterApplyClassItemVariables
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CharacterApplyClassItemVariables")]
    pub struct CharacterApplyClassItem {
        #[arguments(id: $id, classItem: $class_item, clearInventory: $clear_inventory)]
        pub character_apply_class_item: EquipmentResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Character {
        pub account: Uuid,
        pub data: Json,
        pub id: Uuid,
        pub index: i32,
        pub name: String,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum CombatStyle {
        Rage,
        Tech,
        Assassin,
        Energizer,
        Hacker,
        Cyber,
        None,
    }
}