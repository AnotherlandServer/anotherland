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
use item_storage_graphql::{GetOrCreateStorage, GetOrCreateStorageVariables, GetStorage, GetStorageVariables, StorageDestroyItem, StorageDestroyItemVariables, StorageEquipItem, StorageEquipItemVariables, StorageInsertItem, StorageInsertItemVariables, StorageMoveItem, StorageMoveItemVariables, StoragePurchaseItemVariables, StoragePurchaseitem, StorageUneqipItem, StorageUneqipItemVariables};
use obj_params::{GameObjectData, GenericParamSet};
use toolkit::{types::Uuid, NativeParam};

use crate::{RealmApi, RealmApiError, RealmApiResult, Skillbook, item_storage_graphql::{StorageBatchDestroyItems, StorageBatchDestroyItemsVariables, StorageBatchInsertItems, StorageBatchInsertItemsVariables}};

pub enum ItemRef<'a> {
    Name(&'a str),
    Id(i32),
    Uuid(Uuid),
}

impl <'a> TryFrom<ItemRef<'a>> for item_storage_graphql::ItemRef<'a> {
    type Error = RealmApiError;
    
    fn try_from(value: ItemRef<'a>) -> Result<Self, Self::Error> {
        match value {
            ItemRef::Name(name) => Ok(Self {
                name: Some(name),
                id: None,
                uuid: None,
            }),
            ItemRef::Id(id) => Ok(Self {
                name: None,
                id: Some(id),
                uuid: None,
            }),
            ItemRef::Uuid(uuid) => Ok(Self {
                name: None,
                id: None,
                uuid: Some(uuid),
            }),
        }
    }
}

pub enum Price {
    Bling(i32),
    GameCash(i32),
}

impl TryFrom<Price> for item_storage_graphql::Price {
    type Error = RealmApiError;
    
    fn try_from(value: Price) -> Result<Self, Self::Error> {
        match value {
            Price::Bling(bling) => Ok(Self {
                bling: Some(bling),
                game_cash: None,
            }),
            Price::GameCash(game_cash) => Ok(Self {
                bling: None,
                game_cash: Some(game_cash),
            }),
        }
    }
}

pub struct Item {
    pub id: Uuid,
    pub template_id: Uuid,
    pub instance: GameObjectData,
}

impl TryFrom<item_storage_graphql::Item> for Item {
    type Error = RealmApiError;
    
    fn try_from(value: item_storage_graphql::Item) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            template_id: value.template_id,
            instance: serde_json::from_value(value.instance.0)?,
        })
    }
}

pub enum StorageOwner {
    Account(Uuid),
    Character(Uuid),
    Guild(Uuid),
}

impl From<item_storage_graphql::StorageOwner> for StorageOwner {
    fn from(value: item_storage_graphql::StorageOwner) -> Self {
        if let Some(account) = value.account {
            Self::Account(account)
        } else if let Some(character) = value.character {
            Self::Character(character)
        } else if let Some(guild) = value.guild {
            Self::Guild(guild)
        } else {
            unreachable!()
        }
    }
}

impl From<StorageOwner> for item_storage_graphql::StorageOwnerInput {
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

pub struct ItemStorage {
    pub id: Uuid,
    pub name: String,
    pub owner: StorageOwner,
    pub capacity: i32,
    pub bling: Option<i32>,
    pub game_cash: Option<i32>,
    pub items: Vec<Item>,
}

impl TryFrom<item_storage_graphql::ItemStorage> for ItemStorage {
    type Error = RealmApiError;
    
    fn try_from(value: item_storage_graphql::ItemStorage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            owner: value.owner.into(),
            capacity: value.capacity,
            bling: value.bling,
            game_cash: value.game_cash,
            items: value.items.into_iter().map(Item::try_from).collect::<Result<Vec<_>,_>>()?,
        })
    }
}

pub struct StorageResult {
    pub storage_id: Uuid,
    pub bling: Option<i32>,
    pub game_cash: Option<i32>,
    pub changed_items: Option<Vec<Item>>,
    pub removed_items: Option<Vec<Uuid>>,
    pub error: Option<(String, Option<NativeParam>)>,
}

impl TryFrom<item_storage_graphql::StorageResult> for StorageResult {
    type Error = RealmApiError;
    
    fn try_from(value: item_storage_graphql::StorageResult) -> Result<Self, Self::Error> {
        Ok(Self {
            storage_id: value.storage_id,
            bling: value.bling,
            game_cash: value.game_cash,
            changed_items: value.changed_items
                .map(|items| 
                    items.into_iter()
                        .map(Item::try_from)
                        .collect::<Result<Vec<_>,_>>()
                )
                .transpose()?,
            removed_items: value.removed_items,
            error: value.error
                .map(|v| serde_json::from_value(v.0))
                .transpose()?,
        })
    }
}

#[derive(Default)]
pub struct EquipmentResult {
    pub error: Option<(String, Option<NativeParam>)>,
    pub storage_results: Vec<StorageResult>,
    pub character_update: Option<Box<dyn GenericParamSet>>,
    pub skillbook: Option<Skillbook>,
}

impl EquipmentResult {
    pub fn from_graphql(api_base: &RealmApi, result: item_storage_graphql::EquipmentResult) -> RealmApiResult<Self> {
        Ok(Self {
            error: result.error
                .and_then(|v| serde_json::from_value(v.0).ok()),
            storage_results: result.storage_result.into_iter()
                .map(StorageResult::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            character_update: result.character_update
                .and_then(|v| serde_json::from_value(v.0).ok()),
            skillbook: result.skillbook.map(|v| Skillbook::from_graphql(api_base, v)),
        })
    }
}

pub struct ItemStorageId {
    id: Uuid,
    api_base: RealmApi,
}

impl ItemStorageId {
    pub async fn insert_item(&self, item_ref: ItemRef<'_>, tag: Option<String>) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageInsertItem::build(StorageInsertItemVariables {
                id: self.id,
                base_item: item_ref.try_into()?,
                insert_at: None,
                tag,
            })).await?;

        if let Some(StorageInsertItem { storage_insert_item }) = response.data {
            Ok(storage_insert_item.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

     pub async fn batch_insert_items(&self, item_refs: Vec<ItemRef<'_>>, tag: Option<String>) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageBatchInsertItems::build(StorageBatchInsertItemsVariables {
                id: self.id,
                base_items: item_refs
                    .into_iter()
                    .map(|r| r.try_into())
                    .collect::<Result<Vec<_>, _>>()?,
                tag,
            })).await?;

        if let Some(StorageBatchInsertItems { storage_batch_insert_items }) = response.data {
            Ok(storage_batch_insert_items.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn destroy_item(&self, item_id: Uuid, tag: Option<String>) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageDestroyItem::build(StorageDestroyItemVariables {
                id: self.id,
                item_id,
                tag,
            })).await?;

        if let Some(StorageDestroyItem { storage_destroy_item }) = response.data {
            Ok(storage_destroy_item.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_destroy_items(&self, item_ids: Vec<Uuid>, tag: Option<String>) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageBatchDestroyItems::build(StorageBatchDestroyItemsVariables {
                id: self.id,
                item_ids,
                tag,
            })).await?;

        if let Some(StorageBatchDestroyItems { storage_batch_destroy_items }) = response.data {
            Ok(storage_batch_destroy_items.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn move_item(&self, item_id: Uuid, slot: i32, tag: Option<String>) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageMoveItem::build(StorageMoveItemVariables {
                id: self.id,
                item_id,
                new_slot: slot,
                tag,
            })).await?;

        if let Some(StorageMoveItem { storage_move_item }) = response.data {
            Ok(storage_move_item.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn equip_item(&self, item_id: Uuid, slot: Option<i32>, tag: Option<String>) -> RealmApiResult<EquipmentResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageEquipItem::build(StorageEquipItemVariables {
                id: self.id,
                item_id,
                tag,
                idx: slot,
            })).await?;

        if let Some(StorageEquipItem { storage_equip_item }) = response.data {
            EquipmentResult::from_graphql(&self.api_base, storage_equip_item)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn unequip_item(&self, item_id: Uuid, tag: Option<String>) -> RealmApiResult<EquipmentResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StorageUneqipItem::build(StorageUneqipItemVariables {
                id: self.id,
                item_id,
                tag,
            })).await?;

        if let Some(StorageUneqipItem { storage_uneqip_item }) = response.data {
            EquipmentResult::from_graphql(&self.api_base, storage_uneqip_item)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn purchase_item(&self, item_ref: ItemRef<'_>, tag: Option<String>, price: Price) -> RealmApiResult<StorageResult> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(StoragePurchaseitem::build(StoragePurchaseItemVariables {
                base_item: item_ref.try_into()?,
                id: self.id,
                price: price.try_into()?,
                tag,
            })).await?;

        if let Some(StoragePurchaseitem { storage_purchase_item }) = response.data {
            Ok(storage_purchase_item.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl RealmApi {
    pub async fn get_item_storage(&self, id: &Uuid) -> RealmApiResult<Option<ItemStorage>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetStorage::build(GetStorageVariables {
                id: *id
            })).await?;

        if let Some(GetStorage { item_storage }) = response.data {
            if let Some(item_storage) = item_storage {
                Ok(Some(item_storage.try_into()?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_or_create_item_storage(&self, owner: StorageOwner, name: &str) -> RealmApiResult<ItemStorage> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetOrCreateStorage::build(GetOrCreateStorageVariables {
                owner: owner.into(),
                name,
            })).await?;

        if let Some(GetOrCreateStorage { get_or_create_storage }) = response.data {
            Ok(get_or_create_storage.try_into()?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn item_storage_access(&self, id: &Uuid) -> ItemStorageId {
        ItemStorageId {
            id: *id,
            api_base: self.clone(),
        }
    }
}

pub(crate) mod item_storage_graphql {
    use toolkit::types::Uuid;

    use crate::{schema::*, skillbook_graphql::Skillbook};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetOrCreateStorageVariables<'a> {
        pub owner: StorageOwnerInput,
        pub name: &'a str,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageDestroyItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageBatchDestroyItemsVariables {
        pub id: Uuid,
        pub item_ids: Vec<Uuid>,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageEquipItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub idx: Option<i32>,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StoragePurchaseItemVariables<'a> {
        pub base_item: ItemRef<'a>,
        pub id: Uuid,
        pub price: Price,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    #[allow(dead_code)]
    pub struct StorageSellItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageInsertItemVariables<'a> {
        pub base_item: ItemRef<'a>,
        pub id: Uuid,
        pub insert_at: Option<i32>,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageBatchInsertItemsVariables<'a> {
        pub base_items: Vec<ItemRef<'a>>,
        pub id: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageMoveItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub new_slot: i32,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    #[allow(dead_code)]
    pub struct StorageTransferItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub new_slot: i32,
        pub new_storage: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct StorageUneqipItemVariables {
        pub id: Uuid,
        pub item_id: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    #[allow(dead_code)]
    pub struct StorageDepositBlingVariables {
        pub amount: i32,
        pub id: Uuid,
        pub tag: Option<String>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetStorageVariables {
        pub id: Uuid
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageUneqipItemVariables")]
    pub struct StorageUneqipItem {
        #[arguments(id: $id, itemId: $item_id, tag: $tag)]
        pub storage_uneqip_item: EquipmentResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageTransferItemVariables")]
    pub struct StorageTransferItem {
        #[arguments(id: $id, itemId: $item_id, newSlot: $new_slot, newStorage: $new_storage, tag: $tag)]
        #[allow(dead_code)]
        pub storage_transfer_item: Vec<StorageResult>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageSellItemVariables")]
    pub struct StorageSellItem {
        #[arguments(id: $id, itemId: $item_id, tag: $tag)]
        #[allow(dead_code)]
        pub storage_sell_item: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StoragePurchaseItemVariables")]
    pub struct StoragePurchaseitem {
        #[arguments(baseItem: $base_item, id: $id, price: $price, tag: $tag)]
        #[allow(dead_code)]
        pub storage_purchase_item: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageMoveItemVariables")]
    pub struct StorageMoveItem {
        #[arguments(id: $id, itemId: $item_id, newSlot: $new_slot, tag: $tag)]
        pub storage_move_item: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageInsertItemVariables")]
    pub struct StorageInsertItem {
        #[arguments(baseItem: $base_item, id: $id, insertAt: $insert_at, tag: $tag)]
        pub storage_insert_item: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageBatchInsertItemsVariables")]
    pub struct StorageBatchInsertItems {
        #[arguments(baseItems: $base_items, id: $id, tag: $tag)]
        pub storage_batch_insert_items: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageEquipItemVariables")]
    pub struct StorageEquipItem {
        #[arguments(id: $id, itemId: $item_id, tag: $tag, idx: $idx)]
        pub storage_equip_item: EquipmentResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageDestroyItemVariables")]
    pub struct StorageDestroyItem {
        #[arguments(id: $id, itemId: $item_id, tag: $tag)]
        pub storage_destroy_item: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageBatchDestroyItemsVariables")]
    pub struct StorageBatchDestroyItems {
        #[arguments(id: $id, itemIds: $item_ids, tag: $tag)]
        pub storage_batch_destroy_items: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "StorageDepositBlingVariables")]
    pub struct StorageDepositBling {
        #[arguments(amount: $amount, id: $id, tag: $tag)]
        #[allow(dead_code)]
        pub storage_deposit_bling: StorageResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "GetOrCreateStorageVariables")]
    pub struct GetOrCreateStorage {
        #[arguments(name: $name, owner: $owner)]
        pub get_or_create_storage: ItemStorage,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetStorageVariables")]
    pub struct GetStorage {
        #[arguments(id: $id)]
        pub item_storage: Option<ItemStorage>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ItemStorage {
        pub bling: Option<i32>,
        pub game_cash: Option<i32>,
        pub capacity: i32,
        pub id: Uuid,
        pub name: String,
        pub owner: StorageOwner,
        pub items: Vec<Item>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct StorageOwner {
        pub account: Option<Uuid>,
        pub character: Option<Uuid>,
        pub guild: Option<Uuid>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Item {
        pub id: Uuid,
        pub instance: Json,
        pub template_id: Uuid,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct EquipmentResult {
        pub character_update: Option<Json>,
        pub error: Option<Json>,
        pub storage_result: Vec<StorageResult>,
        pub skillbook: Option<Skillbook>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct StorageResult {
        pub bling: Option<i32>,
        pub game_cash: Option<i32>,
        pub changed_items: Option<Vec<Item>>,
        pub error: Option<Json>,
        pub removed_items: Option<Vec<Uuid>>,
        pub storage_id: Uuid,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct StorageOwnerInput {
        pub account: Option<Uuid>,
        pub character: Option<Uuid>,
        pub guild: Option<Uuid>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Price {
        #[cynic(skip_serializing_if="Option::is_none")]
        pub bling: Option<i32>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub game_cash: Option<i32>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ItemRef<'a> {
        #[cynic(skip_serializing_if="Option::is_none")]
        pub name: Option<&'a str>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub id: Option<i32>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub uuid: Option<Uuid>,
    }
}