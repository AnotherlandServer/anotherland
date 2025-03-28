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


use async_graphql::{CustomValidator, InputObject, SimpleObject};
use anyhow::anyhow;
use database::DatabaseRecord;
use mongodb::{bson::{self, doc, Bson}, options::{IndexOptions, ReturnDocument}, IndexModel};
use obj_params::GameObjectData;
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum StorageOwner {
    Account(Uuid),
    Character(Uuid),
    Guild(Uuid),
}

impl From<StorageOwner> for Bson {
    fn from(val: StorageOwner) -> Self {
        bson::to_bson(&val).unwrap()
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(name = "StorageOwner", input_name = "StorageOwnerInput")]
pub struct FlatennedStorageOwner {
    pub account: Option<Uuid>,
    pub character: Option<Uuid>,
    pub guild: Option<Uuid>,
}

impl From<StorageOwner> for FlatennedStorageOwner {
    fn from(value: StorageOwner) -> Self {
        match value {
            StorageOwner::Account(account) => Self {
                account: Some(account),
                character: None,
                guild: None,
            },
            StorageOwner::Character(character) => Self {
                account: None,
                character: Some(character),
                guild: None,
            },
            StorageOwner::Guild(guild) => Self {
                account: None,
                character: None,
                guild: Some(guild),
            },
        }
    }
}

impl From<FlatennedStorageOwner> for StorageOwner {
    fn from(value: FlatennedStorageOwner) -> Self {
        if let Some(account) = value.account {
            return StorageOwner::Account(account);
        }
        if let Some(character) = value.character {
            return StorageOwner::Character(character);
        }
        if let Some(guild) = value.guild {
            return StorageOwner::Guild(guild);
        }
        panic!("Invalid FlatennedStorageOwner");
    }
}

struct StorageOwnerValidator;

impl CustomValidator<FlatennedStorageOwner> for StorageOwnerValidator {
    fn check(&self, _value: &FlatennedStorageOwner) -> Result<(), async_graphql::InputValueError<FlatennedStorageOwner>> {
        Ok(()) // todo: implement
    }
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject, Clone, Debug)]
#[graphql(input_name = "ItemInput")]
pub struct Item {
    pub id: Uuid,
    pub template_id: Uuid,
    pub instance: async_graphql::Json<GameObjectData>,
}

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "item_storage")]
pub struct ItemStorage {
    pub id: Uuid,
    #[graphql_crud(filter)]
    pub name: String,
    #[graphql_crud(filter, validator = "StorageOwnerValidator", serialize_as = FlatennedStorageOwner)]
    pub owner: StorageOwner,
    pub capacity: i32,
    pub bling: Option<i32>,
    pub game_cash: Option<i32>,
    pub items: Vec<Item>,
}

impl DatabaseRecord for ItemStorage {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "item_storages"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { 
                "name": 1,
                "owner": 1,
            })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}

impl ItemStorage {
    pub async fn get_or_create_for_owner(db: &mongodb::Database, name: &str, owner: StorageOwner) -> database::DBResult<ItemStorage> {
        let collection = Self::collection(db);

        let empty_storage = bson::to_bson(&match owner {
            StorageOwner::Account(_) => ItemStorage {
                id: Uuid::new(),
                name: name.to_string(),
                owner,
                capacity: 30,
                bling: Some(0),
                game_cash: Some(0),
                items: vec![],
            },
            StorageOwner::Character(_) => ItemStorage {
                id: Uuid::new(),
                name: name.to_string(),
                owner,
                capacity: 30,
                bling: Some(0),
                game_cash: Some(0),
                items: vec![],
            },
            StorageOwner::Guild(_) => ItemStorage {
                id: Uuid::new(),
                name: name.to_string(),
                owner,
                capacity: 30,
                bling: None,
                game_cash: Some(0),
                items: vec![],
            },
        }).unwrap();

        let storage = collection.find_one_and_update(doc! {
                "owner": owner,
                "name": name,
            }, doc!{
                "$setOnInsert": empty_storage
            })
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or_else(|| anyhow!("upsert failed"))?;

        Ok(storage)
    }
}