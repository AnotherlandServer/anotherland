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

use std::sync::Arc;

use async_graphql::{Context, Error, Json, Object, OneofObject, SimpleObject};
use database::DatabaseRecord;
use mongodb::{bson::doc, Database};
use obj_params::GenericParamSet;
use toolkit::{types::Uuid, NativeParam};

use crate::{db::{self, Character, FlatennedStorageOwner, Item, ItemStorageOutput, ObjectTemplate, SkillbookOutput, StorageOwner}, item_storage_session::{ItemStorageSession, ItemStorageSessionError, ItemStorageSessionResult}, proto::{RealmNotification, RealmServer}};

#[derive(Default)]
pub struct ItemStorageExtMutationRoot;

#[derive(OneofObject)]
pub enum ItemRef {
    Name(String),
    Id(i32),
    Uuid(Uuid),
}

#[derive(SimpleObject, Default)]
pub struct StorageResult {
    storage_id: Uuid,
    error: Option<async_graphql::Json<(String, Option<NativeParam>)>>,
    changed_items: Option<Vec<Item>>,
    removed_items: Option<Vec<Uuid>>,
    bling: Option<i32>,
    game_cash: Option<i32>,
}

impl From<ItemStorageSessionResult> for StorageResult {
    fn from(result: ItemStorageSessionResult) -> Self {
        Self {
            storage_id: result.id,
            error: None,
            changed_items: if result.changed_items.is_empty() { None } else { Some(result.changed_items.iter()
                .map(|v| {
                    Item {
                        id: v.id,
                        template_id: v.template_id,
                        instance: v.instance.clone(),
                    }
                }).collect()
            ) },
            removed_items: if result.removed_items.is_empty() { None } else { Some(result.removed_items.clone()) },
            bling: result.bling,
            game_cash: result.game_cash,
        }
    }
}

#[derive(SimpleObject)]
pub struct EquipmentResult {
    pub error: Option<async_graphql::Json<(String, Option<NativeParam>)>>,
    pub storage_result: Vec<StorageResult>,
    pub character_update: Option<async_graphql::Json<Box<dyn GenericParamSet>>>,
    pub skillbook: Option<SkillbookOutput>,
}

#[derive(OneofObject)]
pub enum Price {
    Bling(i32),
    GameCash(i32),
}

pub async fn find_item(db: &Database, item_ref: ItemRef) -> Result<Option<ObjectTemplate>, Error> {
    match item_ref {
        ItemRef::Name(name) => {
            Ok(ObjectTemplate::collection(db)
                .find_one(doc! { "name": name })
                .await?)
        },
        ItemRef::Id(id) => {
            Ok(ObjectTemplate::collection(db)
                .find_one(doc! { "numeric_id": id })
                .await?)
        },
        ItemRef::Uuid(uuid) => {
            Ok(ObjectTemplate::collection(db)
                .find_one(doc! { "id": uuid })
                .await?)
        }
    }
}

pub async fn send_inventory_update_notifications(ctx: &Context<'_>, tag: Option<String>, results: &[ItemStorageSessionResult]) -> Result<(), Error> {
    let server = ctx.data::<Arc<RealmServer>>()?;

    for res in results {
        server.notify(RealmNotification::ItemStorageUpdated { id: res.id, tag: tag.clone() }).await?;
    }

    Ok(())
}

#[Object]
impl ItemStorageExtMutationRoot {
    pub async fn get_or_create_storage(&self, ctx: &Context<'_>, name: String, owner: FlatennedStorageOwner) -> Result<ItemStorageOutput, Error> {
        let db = ctx.data::<Database>()?.clone();
        Ok(
            db::ItemStorage::get_or_create_for_owner(&db, &name, owner.into()).await?.try_into()?
        )
    }

    pub async fn storage_insert_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, base_item: ItemRef, insert_at: Option<i32>) -> Result<StorageResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        if let Some(item) = find_item(&db, base_item).await? {
            match session.insert_item(item, insert_at, None).await {
                Ok(_) => {
                    let results = session.commit().await?;
                    send_inventory_update_notifications(ctx, tag, &results).await?;

                    let res = results.into_iter().next().unwrap();

                    Ok(res.into())
                },
                Err(ItemStorageSessionError::ClientError(str, e)) => {
                    Ok(StorageResult { 
                        storage_id: id, 
                        error: Some(async_graphql::Json((str.to_string(), e))),
                        changed_items: None, 
                        removed_items: None, 
                        bling: None,
                        game_cash: None,
                    })
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::new("Item not found"))
        }
    }

    pub async fn storage_destroy_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, item_id: Uuid) -> Result<StorageResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        match session.destroy_item(item_id).await {
            Ok(_) => {
                let results = session.commit().await?;
                send_inventory_update_notifications(ctx, tag, &results).await?;

                let res = results.into_iter().next().unwrap();
        
                Ok(res.into())
            },
            Err(ItemStorageSessionError::ClientError(str, e)) => {
                Ok(StorageResult { 
                    storage_id: id, 
                    error: Some(async_graphql::Json((str.to_string(), e))),
                    changed_items: None, 
                    removed_items: None, 
                    bling: None,
                    game_cash: None,
                })
            },
            Err(e) => Err(e.into())
        }
    }

    pub async fn storage_move_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, item_id: Uuid, new_slot: i32) -> Result<StorageResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        match session.move_item(item_id, new_slot).await {
            Ok(_) => {
                let results = session.commit().await?;
                send_inventory_update_notifications(ctx, tag, &results).await?;

                let res = results.into_iter().next().unwrap();
        
                Ok(res.into())
            },
            Err(ItemStorageSessionError::ClientError(str, e)) => {
                Ok(StorageResult { 
                    storage_id: id, 
                    error: Some(async_graphql::Json((str.to_string(), e))),
                    changed_items: None, 
                    removed_items: None, 
                    bling: None,
                    game_cash: None,
                })
            },
            Err(e) => Err(e.into())
        }
    }

    pub async fn storage_transfer_item(&self, _ctx: &Context<'_>, _tag: Option<String>, _id: Uuid, _item_id: Uuid, _new_storage: Uuid, _new_slot: i32) -> Result<Vec<StorageResult>, Error> {
        unimplemented!()
    }

    pub async fn storage_equip_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, item_id: Uuid, idx: Option<i32>) -> Result<EquipmentResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        if let &StorageOwner::Character(char_id) = session.owner() {
            match session.equip_item(item_id, idx).await {
                Ok(_) => {
                    let (mut session, results) = session.write_uncommitted().await?;
                    let changes = Character::update_equipment(&db, &mut session, char_id, id).await?;

                    session.commit_transaction().await?;

                    send_inventory_update_notifications(ctx, tag, &results).await?;

                    let res = results.into_iter().next().unwrap();
            
                    Ok(EquipmentResult {
                        error: None,
                        character_update: Some(Json(changes)),
                        storage_result: vec![res.into()],
                        skillbook: None,
                    })
                },
                Err(ItemStorageSessionError::ClientError(str, e)) => {
                    Ok(EquipmentResult {
                        error: Some(async_graphql::Json((str.to_string(), e))),
                        character_update: None,
                        storage_result: vec![],
                        skillbook: None,
                    })
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::new("Storage is not a character storage"))
        }
    }

    pub async fn storage_uneqip_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, item_id: Uuid) -> Result<EquipmentResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        if let &StorageOwner::Character(char_id) = session.owner() {
            match session.unequip_item(item_id).await {
                Ok(_) => {
                    let (mut session, results) = session.write_uncommitted().await?;
                    let changes = Character::update_equipment(&db, &mut session, char_id, id).await?;

                    session.commit_transaction().await?;

                    send_inventory_update_notifications(ctx, tag, &results).await?;

                    let res = results.into_iter().next().unwrap();
            
                    Ok(EquipmentResult {
                        error: None,
                        character_update: Some(Json(changes)),
                        storage_result: vec![res.into()],
                        skillbook: None,
                    })
                },
                Err(ItemStorageSessionError::ClientError(str, e)) => {
                    Ok(EquipmentResult {
                        error: Some(async_graphql::Json((str.to_string(), e))),
                        character_update: None,
                        storage_result: vec![],
                        skillbook: None,
                    })
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::new("Storage is not a character storage"))
        }

    }

    pub async fn storage_deposit_bling(&self, _ctx: &Context<'_>, _tag: Option<String>, _id: Uuid, _amount: i32) -> Result<StorageResult, Error> {
        unimplemented!()
    }

    pub async fn storage_purchase_item(&self, ctx: &Context<'_>, tag: Option<String>, id: Uuid, base_item: ItemRef, _price: Price) -> Result<StorageResult, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut session = ItemStorageSession::start(&db, id).await?;

        if let Some(item) = find_item(&db, base_item).await? {
            match session.insert_item(item, None, None).await {
                Ok(_) => {
                    let results = session.commit().await?;
                    send_inventory_update_notifications(ctx, tag, &results).await?;

                    let res = results.into_iter().next().unwrap();

                    Ok(res.into())
                },
                Err(ItemStorageSessionError::ClientError(str, e)) => {
                    Ok(StorageResult { 
                        storage_id: id, 
                        error: Some(async_graphql::Json((str.to_string(), e))),
                        changed_items: None, 
                        removed_items: None, 
                        bling: None,
                        game_cash: None,
                    })
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::new("Item not found"))
        }
    }

    pub async fn storage_sell_item(&self, _ctx: &Context<'_>, _tag: Option<String>, _id: Uuid, _item_id: Uuid) -> Result<StorageResult, Error> {
        unimplemented!()
    }
}