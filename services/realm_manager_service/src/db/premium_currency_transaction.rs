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

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use database::{DBResult, DatabaseRecord};
use mongodb::{bson::{doc, Bson}, options::IndexOptions, ClientSession, Database, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PremiumCurrencyTransaction {
    #[serde(rename = "_id")]
    pub id: Option<Bson>,
    pub account_id: Uuid,
    pub date: DateTime<Utc>,
    pub amount: i32,
    pub balance: i32,
    pub comment: Option<String>,
    pub completed: bool,
}

impl PremiumCurrencyTransaction {
    pub async fn write(db: &Database, session: &mut ClientSession, account_id: Uuid, amount: i32, balance: i32, comment: Option<String>, completed: bool) -> DBResult<Self> {
        let id = Self::collection(db)
            .insert_one(PremiumCurrencyTransaction {
                id: None,
                account_id,
                date: Utc::now(),
                amount,
                balance,
                comment,
                completed,
            })
            .session(&mut *session)
            .await?
            .inserted_id;

        Self::collection(db).find_one(doc!{"_id": id}).session(&mut *session).await?
            .ok_or(anyhow!("transaction not found").into())

    }
}

impl DatabaseRecord for PremiumCurrencyTransaction {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.account_id
    }

    fn key_name() -> &'static str {
        "account_id"
    }

    fn collection_name() -> &'static str {
        "premium_currency_transactions"
    }

    async fn build_index(db: &Database) -> DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "account_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}