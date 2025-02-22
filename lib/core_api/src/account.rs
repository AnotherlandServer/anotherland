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

use std::fmt::Display;

use account_graphql::{AuthQuery, EmailQuery, FindAccount, FindAccountVariables, RegisterSteamAccount, RegisterSteamAccountVariables, SteamQuery, UsernameQuery};
use chrono::{DateTime, Utc};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use steamworks::SteamId;
use toolkit::types::Uuid;

use crate::{error::CoreApiResult, CoreApi, CoreApiError};

pub enum Identifier {
    Username(String),
    Steam(String),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Username(name) => f.write_str(name),
            Identifier::Steam(id) => f.write_str(id),
        }
    }
}

pub enum AccountQuery {
    Username(String),
    Email(String),
    SteamId(SteamId),
}

pub struct Account {
    _api_base: CoreApi,

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
    pub(crate) fn from_graphql(api_base: &CoreApi, account: account_graphql::Account) -> Self {
        Self {
            _api_base: api_base.clone(),

            id: account.id.0.parse().unwrap(),
            numeric_id: account.numeric_id,
            identifier: match account.identifier {
                account_graphql::Identifier::SteamIdentifier(steam_identifier) => 
                    Identifier::Steam(steam_identifier.steam_id),
                account_graphql::Identifier::UsernameIdentifier(username_identifier) => 
                    Identifier::Username(username_identifier.username),
                account_graphql::Identifier::Unknown => panic!(),
            },
            created: account.created.0.parse().unwrap(),
            last_login: account.last_login.map(|date| date.0.parse().unwrap()),
            banned: account.banned,
            ban_reason: account.ban_reason,
            is_gm: account.is_gm,
        }
    }

    pub fn id(&self) -> &Uuid { &self.id }
    pub fn numric_id(&self) -> i32 { self.numeric_id }
    pub fn identifier(&self) -> &Identifier { &self.identifier }
    pub fn created(&self) -> &DateTime<Utc> { &self.created }
    pub fn last_login(&self) -> Option<&DateTime<Utc>> { self.last_login.as_ref() }
    pub fn banned(&self) -> bool { self.banned }
    pub fn ban_reason(&self) -> Option<&str> { self.ban_reason.as_deref() }
    pub fn is_gm(&self) -> bool { self.is_gm }

    pub async fn ban(&mut self, _reason: String) -> CoreApiResult<()> { todo!() }
    pub async fn unban(&mut self) -> CoreApiResult<()> { todo!() }
    pub async fn promote(&mut self) -> CoreApiResult<()> { todo!() }
    pub async fn demote(&mut self) -> CoreApiResult<()> { todo!() }
}

impl CoreApi {
    pub async fn register_steam_account(&self, steam_id: SteamId) -> CoreApiResult<Account> {
        let steam_id = steam_id.steamid32();
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(RegisterSteamAccount::build(RegisterSteamAccountVariables {
                steam_id: &steam_id
            })).await?;

        if let Some(account) = response.data.map(|res| res.register_steam_account) {
            Ok(Account::from_graphql(self, account))
        } else {
            Err(CoreApiError::GraphQl(response.errors.unwrap()))
        }
    }

    pub async fn find_account(&self, query: AccountQuery) -> CoreApiResult<Option<Account>> {
        let query = match &query {
            AccountQuery::Username(username) => AuthQuery {
                username: Some(UsernameQuery { username }),
                email: None,
                steam: None,
            },
            AccountQuery::Email(email) => AuthQuery {
                username: None,
                email: Some(EmailQuery { email }),
                steam: None,
            },
            AccountQuery::SteamId(steam_id) => AuthQuery {
                username: None,
                email: None,
                steam: Some(SteamQuery { steam_id: steam_id.steamid32() }),
            },
        };

        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(FindAccount::build(FindAccountVariables { auth_query: query })).await?;

        if let Some(account) = response.data.map(|res| res.find_account) {
            Ok(account.map(|account| Account::from_graphql(self, account)))
        } else {
            Err(CoreApiError::GraphQl(response.errors.unwrap()))
        }
    }
}

// Graphql Queries
pub(crate) mod account_graphql {
    use crate::schema::*;
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct UsernameIdentifier {
        #[allow(dead_code)]
        pub email: Option<String>,
        pub username: String,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct SteamIdentifier {
        pub steam_id: String,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct RegisterSteamAccountVariables<'a> {
        pub steam_id: &'a str,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct FindAccountVariables<'a> {
        pub auth_query: AuthQuery<'a>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "core_service")]
    pub struct AuthQuery<'a> {
        #[cynic(skip_serializing_if="Option::is_none")]
        pub email: Option<EmailQuery<'a>>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub steam: Option<SteamQuery>,
        #[cynic(skip_serializing_if="Option::is_none")]
        pub username: Option<UsernameQuery<'a>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "core_service")]
    pub struct UsernameQuery<'a> {
        pub username: &'a str,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "core_service")]
    pub struct SteamQuery {
        pub steam_id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "core_service")]
    pub struct EmailQuery<'a> {
        pub email: &'a str,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "MutationRoot", variables = "RegisterSteamAccountVariables")]
    pub struct RegisterSteamAccount {
        #[arguments(steamId: $steam_id)]
        pub register_steam_account: Account,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "QueryRoot", variables = "FindAccountVariables")]
    pub struct FindAccount {
        #[arguments(authQuery: $auth_query)]
        pub find_account: Option<Account>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct Account {
        pub ban_reason: Option<String>,
        pub banned: bool,
        pub created: DateTime,
        pub id: Uuid,
        pub identifier: Identifier,
        pub is_gm: bool,
        pub last_login: Option<DateTime>,
        pub numeric_id: i32,
    }
    
    #[derive(cynic::InlineFragments, Debug)]
    #[cynic(schema = "core_service")]
    #[allow(clippy::enum_variant_names)]
    pub enum Identifier {
        SteamIdentifier(SteamIdentifier),
        UsernameIdentifier(UsernameIdentifier),
        #[cynic(fallback)]
        Unknown
    }   
}