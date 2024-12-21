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

use chrono::NaiveDate;
use database::DatabaseRecord;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "cash_shop_item")]
pub struct CashShopItem {
    pub id: Uuid,
    pub display_name: String,
    pub description: String,
    pub reference_item_name: String,
    pub reference_item_guid: Uuid,
    pub cash_price: i32,
    pub sku_code: String,
    pub rental_duration: i32,
    pub is_in_stock: bool,
    pub is_hot: bool,
    pub is_new: bool,
    pub version: i32,
    pub is_visible: bool,
    pub is_tradable: bool,
    pub is_featured: bool,
    pub quantity: i32,
    pub discount: i32,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
}

impl DatabaseRecord for CashShopItem {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "cash_shop_item"
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
                .keys(doc! { "id": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build()).await?;

        Ok(())
    }
}