use std::time::Duration;

use async_trait::async_trait;
use bson::doc;
use chrono::{DateTime, Utc};
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use serde::Serialize;
use serde_derive::Deserialize;

use crate::{util::AnotherlandResult};
use atlas::Uuid;

use super::{Account, DatabaseRecord};

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