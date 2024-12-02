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

use async_graphql::{Context, Error, InputObject, Object, OneofObject, SimpleObject, Union};
use chrono::{DateTime, Utc};
use database::DatabaseRecord;
use mongodb::Database;
use toolkit::types::Uuid;

use crate::db;

#[derive(Default)]
pub struct AccountRoot;

#[derive(Default)]
pub struct AccountMutationRoot;

#[Object]
impl AccountRoot {
    async fn account(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = db::Account::get(&db, &id).await?;

        Ok(res.map(Account::from_db))
    }

    async fn find_account(&self, ctx: &Context<'_>, auth_query: AuthQuery) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = match auth_query {
            AuthQuery::Username(username_query) => db::Account::get_by_username_or_mail(&db, &username_query.username).await?,
            AuthQuery::Email(email_query) => db::Account::get_by_username_or_mail(&db, &email_query.email).await?,
            AuthQuery::Steam(steam_query) => db::Account::get_by_steam_id(&db, &steam_query.steam_id).await?,
        };

        Ok(res.map(Account::from_db))
    }
}

#[Object]
impl AccountMutationRoot {
    async fn register_email_account(&self, ctx: &Context<'_>, username: String, email: Option<String>, password: Option<String>) -> Result<Account, Error> {
        let db = ctx.data::<Database>()?.clone();
        Ok(Account::from_db(
            db::Account::create(&db, db::Credentials::Username { 
                name: username, 
                email, 
                password: match password {
                    Some(password) => db::PasswordHash::hash_password(password),
                    None => db::PasswordHash::Unset,
                }
            }).await?
        ))
    }

    async fn register_steam_account(&self, ctx: &Context<'_>, steam_id: String) -> Result<Account, Error> {
        let db = ctx.data::<Database>()?.clone();
        Ok(Account::from_db(
            db::Account::create(&db, db::Credentials::Steam { steam_id }).await?
        ))
    }

    async fn set_password(&self, ctx: &Context<'_>, id: Uuid, password: String) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            if let db::Credentials::Username { .. } = account.credentials {
                account.set_password(password)?;
                account.save(&db).await?;
    
                Ok(Some(Account::from_db(account)))
            } else {
                Err(Error::new("can't change password of steam account"))
            }
        } else {
            Ok(None)
        }
    }

    async fn set_one_time_password(&self, ctx: &Context<'_>, id: Uuid, password: String) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            if let db::Credentials::Username { .. } = account.credentials {
                account.set_one_time_password(password)?;
                account.save(&db).await?;
    
                Ok(Some(Account::from_db(account)))
            } else {
                Err(Error::new("can't change password of steam account"))
            }
        } else {
            Ok(None)
        }
    }

    async fn ban_account(&self, ctx: &Context<'_>, id: Uuid, reason: String) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            account.banned = true;
            account.ban_reason = Some(reason);
            account.save(&db).await?;

            Ok(Some(Account::from_db(account)))
        } else {
            Ok(None)
        }
    }

    async fn unban_account(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            account.banned = false;
            account.ban_reason = None;
            account.save(&db).await?;

            Ok(Some(Account::from_db(account)))
        } else {
            Ok(None)
        }
    }

    async fn promote_account(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            account.is_gm = true;
            account.save(&db).await?;

            Ok(Some(Account::from_db(account)))
        } else {
            Ok(None)
        }
    }

    async fn demote_account(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Account>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut account) = db::Account::get(&db, &id).await? {
            account.is_gm = false;
            account.save(&db).await?;

            Ok(Some(Account::from_db(account)))
        } else {
            Ok(None)
        }
    }
}

#[derive(InputObject)]
pub struct UsernameQuery {
    username: String
}

#[derive(InputObject)]
pub struct EmailQuery {
    email: String
}

#[derive(InputObject)]
pub struct SteamQuery {
    steam_id: String
}

#[derive(OneofObject)]
pub enum AuthQuery {
    Username(UsernameQuery),
    Email(EmailQuery),
    Steam(SteamQuery)
}

#[derive(SimpleObject)]
pub struct Account {
    id: Uuid,
    numeric_id: i32,
    identifier: Identifier,
    created: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
    banned: bool,
    ban_reason: Option<String>,
    is_gm: bool,
}

impl Account {
    pub fn from_db(account: db::Account) -> Self {
        Self {
            id: account.id,
            numeric_id: account.numeric_id,
            identifier: match account.credentials {
                db::Credentials::Username { name, email, .. } => {
                    Identifier::Username(UsernameIdentifier { username: name, email })
                },
                db::Credentials::Steam { steam_id } => {
                    Identifier::Steam(SteamIdentifier { steam_id })
                }
            },
            created: account.created,
            last_login: account.last_login,
            banned: account.banned,
            ban_reason: account.ban_reason,
            is_gm: account.is_gm,
        }
    }
}

#[derive(SimpleObject)]
pub struct SteamIdentifier {
    pub steam_id: String,
}

#[derive(SimpleObject)]
pub struct UsernameIdentifier {
    pub username: String,
    pub email: Option<String>,
}

#[derive(Union)]
pub enum Identifier {
    Steam(SteamIdentifier),
    Username(UsernameIdentifier),
}
