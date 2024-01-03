// Copyright (C) 2023 AnotherlandServer
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
use bson::doc;
use chrono::{DateTime, Utc};
use mongodb::{Database, Collection};
use serde::Serialize;
use serde_derive::Deserialize;
use uuid::Uuid;

use super::DatabaseRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashShopItem {
    pub id: Uuid,
    pub display_name: String,
    pub description: String,
    pub reference_item_name: String,
    pub reference_item_guid: Uuid,
    pub cash_price: u32,
    pub sku_code: String,
    pub rental_duration: u32,
    pub is_in_stock: bool,
    pub is_hot: bool,
    pub is_new: bool,
    pub version: u32,
    pub is_visible: bool,
    pub is_tradable: bool,
    pub is_featured: bool,
    pub quantity: u32,
    pub discount: u32,
    pub date_start: Option<DateTime<Utc>>,
    pub date_end: Option<DateTime<Utc>>,   
}

#[async_trait]
impl DatabaseRecord<'_> for CashShopItem {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("cash_shop_items")
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}