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
pub struct CashShopVendor {
    pub id: Uuid,
    pub name: String,
    pub sku_list: Vec<Uuid>,
    pub bundle_list: Vec<Uuid>,
    pub version: u32,
}

#[async_trait]
impl DatabaseRecord<'_> for CashShopVendor {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("cash_shop_vendors")
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}