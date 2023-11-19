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

use async_graphql::{Object, Result, Context, SimpleObject};
use atlas::Uuid;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::cluster::{ApiRequest, ApiResponse};

use super::ApiServer;

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn account(&self, ctx: &Context<'_>, id: String) -> Result<Account> {
        match ctx.data_unchecked::<ApiServer>().query_cluster(ApiRequest::QueryAccount { id }).await {
            Ok(ApiResponse::Account(account)) => Ok(account),
            Err(e) => Err(e),
            _ => Err(async_graphql::Error::new_with_source("Unexpected response")),
        }
    }
}

#[Object]
impl MutationRoot {
    async fn create_account(&self, ctx: &Context<'_>, name: String, email: String, password: String) -> Result<Account> {
        match ctx.data_unchecked::<ApiServer>().query_cluster(ApiRequest::CreateAccout { name, email, password }).await {
            Ok(ApiResponse::Account(account)) => Ok(account),
            Err(e) => Err(e),
            _ => Err(async_graphql::Error::new_with_source("Unexpected response")),
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    id: String,
    username: String,
    email: String,
    created: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
    banned: bool,
    ban_reason: Option<String>,
    is_gm: bool,
}

impl From<crate::db::Account> for Account {
    fn from(value: crate::db::Account) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username,
            email: value.email,
            created: value.created,
            last_login: value.last_login,
            banned: value.banned,
            ban_reason: value.ban_reason,
            is_gm: value.is_gm,
        }
    }
}
