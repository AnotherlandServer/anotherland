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

use database::{DBResult, DatabaseRecord, transaction_with_retry};
use mongodb::{bson::doc, options::IndexOptions, ClientSession, Database, IndexModel};
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

use super::PremiumCurrencyTransaction;

#[derive(Serialize, Deserialize)]
pub struct PremiumCurrency {
    account_id: Uuid,
    balance: i32,
}

impl PremiumCurrency {
    pub async fn transfer_currency(db: &Database, account_id: &Uuid, amount: i32, comment: Option<String>) -> DBResult<PremiumCurrencyTransaction> {
        let comment = comment.as_ref();

        transaction_with_retry(db.clone(), async |mut session| -> DBResult<(ClientSession, PremiumCurrencyTransaction)> {
            let record = PremiumCurrency::collection(db).find_one_and_update(doc! {
                "account_id": account_id,
            }, doc! {
                "$inc": { "balance": amount }
            })
            .upsert(true)
            .session(&mut session)
            .await?
            .unwrap();

            let transaction = PremiumCurrencyTransaction::write(db, 
                &mut session, 
                *account_id, 
                amount, 
                record.balance, 
                comment.cloned(), 
                true
            ).await?;

            Ok((session, transaction))
        }).await
    }

    pub async fn get_balance(db: &Database, account_id: &Uuid) -> DBResult<i32> {
        if let Some(record) = Self::get(db, account_id).await? {
            Ok(record.balance)
        } else {
            Ok(0)
        }
    }
}

impl DatabaseRecord for PremiumCurrency {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.account_id
    }

    fn key_name() -> &'static str {
        "account_id"
    }

    fn collection_name() -> &'static str {
        "premium_currency"
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