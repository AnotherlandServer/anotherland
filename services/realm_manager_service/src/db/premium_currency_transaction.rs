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

use chrono::{DateTime, Utc};
use database::{DBResult, DatabaseError, DatabaseRecord};
use mongodb::{bson::{doc, Bson, Document, Uuid}, options::IndexOptions, ClientSession, Database, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PremiumCurrencyTransaction {
    _id: Option<Bson>,
    account_id: Uuid,
    date: DateTime<Utc>,
    amount: i32,
    balance: i32,
    comment: Option<String>,
    completed: bool,
}

impl PremiumCurrencyTransaction {
    pub async fn write(db: &Database, session: &mut ClientSession, account_id: Uuid, amount: i32, balance: i32, comment: Option<String>, completed: bool) -> DBResult<Self> {
        let id = Self::collection(db)
            .insert_one(PremiumCurrencyTransaction {
                _id: None,
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
            .ok_or(DatabaseError::Custom("transaction not found"))

    }

    pub fn date(&self) -> DateTime<Utc> { self.date }
    pub fn balance(&self) -> i32 { self.balance }
    pub fn amount(&self) -> i32 { self.amount }
    pub fn comment(&self) -> Option<&str> { self.comment.as_ref().map(String::as_str) }
    pub fn completed(&self) -> bool { self.completed }

    pub async fn complete(&mut self, db: &Database) -> DBResult<()> {
        self.completed = true;
        self.save(db).await?;
        Ok(())
    }
}

impl DatabaseRecord<'_> for PremiumCurrencyTransaction {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.account_id
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