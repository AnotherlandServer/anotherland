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
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn account(&self, ctx: &Context<'_>, id: String) -> Result<Account, Error> {
        /*let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .get_account(Uuid::parse_str(&id)?)
            .await?;
        Ok(account.into())*/
        todo!()
    }

    async fn find_account(&self, ctx: &Context<'_>, username_or_email: String) -> Result<Account, Error> {
        /*let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .find_account(username_or_email).await?;

        Ok(account.into())*/
        todo!()
    }

    async fn active_sessions(&self, ctx: &Context<'_>) -> Result<Vec<Session>, Error> {
        todo!()
    }

    async fn realms(&self, ctx: &Context<'_>) -> Result<Vec<Realm>, Error> {
        todo!()
    }
}

#[Object]
impl MutationRoot {
    async fn register_email_account(&self, ctx: &Context<'_>, name: String, email: String, password: Option<String>) -> Result<Account, Error> {
        todo!()
    }

    async fn register_steam_account(&self, ctx: &Context<'_>, name: String, steam_id: String) -> Result<Account, Error> {
        todo!()
    }

    async fn find_account(&self, ctx: &Context<'_>, query: AuthQuery) -> Result<Account, Error> {
        todo!()
    }

    async fn lock_auth(&self, ctx: &Context<'_>) -> Result<&str, Error> {
        todo!()
    }

    async fn unlock_auth(&self, ctx: &Context<'_>) -> Result<&str, Error> {
        todo!()
    }
}

#[derive(InputObject)]
pub struct UsernameQuery {
    name: String
}

#[derive(InputObject)]
pub struct EMailQuery {
    username: Option<String>,
    email: Option<String>,
}

#[derive(InputObject)]
pub struct SteamQuery {
    steam_id: String
}

#[derive(OneofObject)]
pub enum AuthQuery {
    EMail(EMailQuery),
    Steam(SteamQuery)
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    id: String,
    username: String,
    auth_method: AuthMethod,
    created: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
    banned: bool,
    ban_reason: Option<String>,
    is_gm: bool,
}

#[derive(Union, Serialize, Deserialize, Clone, Debug)]
pub enum AuthMethod {
    EMail(EMailAuth),
    Steam(SteamAuth),
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct EMailAuth {
    email: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct SteamAuth {
    steam_id: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Realm {
    pub id: u32,
    pub name: String,
    pub address: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    id: String,
    account: Account,
    is_gm: bool,
    active_realm: Option<Realm>,
    active_character_id: Option<u32>,
    created: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    
    #[graphql(skip)]
    session_id: Uuid,

    #[graphql(skip)]
    realm_id: Option<u32>,

    #[graphql(skip)]
    character_id: Option<u32>,

    #[graphql(skip)]
    zone_id: Option<Uuid>,
}
