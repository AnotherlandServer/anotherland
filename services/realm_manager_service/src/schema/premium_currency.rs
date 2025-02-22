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

use async_graphql::{Context, Error, Object, SimpleObject};
use mongodb::Database;
use toolkit::types::Uuid;

use crate::db;

#[derive(Default)]
pub struct PremiumCurrencyRoot;

#[derive(Default)]
pub struct PremiumCurrencyMutationRoot;

#[Object]
impl PremiumCurrencyRoot {
    async fn premium_currency_balance(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<PremiumCurrencyBalance, Error> {
        let db = ctx.data::<Database>()?.clone();
        let balance = db::PremiumCurrency::get_balance(&db, &account_id).await?;

        Ok(PremiumCurrencyBalance { 
            account_id,
            balance,
        })
    }
}

#[Object]
impl PremiumCurrencyMutationRoot {
    async fn premium_currency_transfer(&self, ctx: &Context<'_>, account_id: Uuid, amount: i32, comment: Option<String>) -> Result<PremiumCurrencyBalance, Error> {
        let db = ctx.data::<Database>()?.clone();
        let transaction = db::PremiumCurrency::transfer_currency(&db, &account_id, amount, comment).await?;

        Ok(PremiumCurrencyBalance { 
            account_id,
            balance: transaction.balance,
        })
    }
}

#[derive(SimpleObject)]
pub struct PremiumCurrencyBalance {
    account_id: Uuid,
    balance: i32,
}