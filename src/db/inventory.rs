// Copyright (C) 2024 AnotherlandServer
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

use async_trait::async_trait;
use atlas::{BundleItemClass, ClassId, ClassItemClass, EdnaBaseClass, EdnaFunctionClass, EdnaModuleClass, ItemBaseComponent, ItemMyLandThemeClass, MinigameItemClass, ParamBox, PortalItemClass, SomaforgeItemClass, Uuid};
use bson::doc;
use mongodb::{options::{FindOptions, IndexOptions}, Collection, Cursor, Database, IndexModel};
use serde::Serialize;
use serde_derive::Deserialize;

use crate::util::{AnotherlandError, AnotherlandResult};

use super::{DatabaseRecord, ItemContent};

#[derive(Clone, Serialize, Deserialize)]
pub enum InventoryOwner {
    Character(Uuid),
    Account(Uuid),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InventoryEntry {
    pub id: Uuid,
    pub owner: InventoryOwner,
    pub template: Uuid,
    pub params: ParamBox,
}

impl InventoryEntry {
    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = InventoryEntry::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("owner": 1))
            .options(IndexOptions::builder().unique(false).build())
            .build(), 
            None).await?;

        Ok(())
    }

    pub async fn get_player_inventory(db: Database, character_id: Uuid) -> AnotherlandResult<Cursor<InventoryEntry>> {
        let collection = InventoryEntry::collection(db);
        Ok(collection.find(doc! {
            "owner": {
                "$eq": bson::to_bson(&InventoryOwner::Character(character_id)).unwrap()
            }
        }, FindOptions::default()).await?)
    }

    pub fn from_item(owner: InventoryOwner, item: &ItemContent) -> AnotherlandResult<Self> {
        if let Some(ref params) = item.data {
            Ok(Self {
                id: Uuid::new(),
                owner,
                template: item.guid,
                params: params.clone(),
            })
        } else {
            Err(AnotherlandError::app_err("item does not contain any data"))
        }
    }
}

#[async_trait]
impl DatabaseRecord<'_> for InventoryEntry {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("inventory")
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}