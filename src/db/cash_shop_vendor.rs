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
use mongodb::{Database, Collection};
use serde::Serialize;
use serde_derive::Deserialize;
use uuid::Uuid;

use super::DatabaseRecord;

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