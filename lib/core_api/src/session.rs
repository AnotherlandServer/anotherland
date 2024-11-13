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

use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use session_graphql::{DestroySession, DestroySessionVariables, GetSession, GetSessionVariables, LoginSteam, LoginSteamVariables, LoginUsername, LoginUsernameVariables};
use steamworks::SteamId;
use toolkit::types::Uuid;

use crate::{error::{CoreApiError, CoreApiResult}, schema, Account, CoreApi};

pub struct Session {
    api_base: CoreApi,

    id: Uuid,
    account: Account,
    created: DateTime<Utc>,
    last_seen: DateTime<Utc>,
}

impl Session {
    pub(crate) fn from_graphql(api_base: &CoreApi, session: session_graphql::Session) -> Self {
        Self {
            api_base: api_base.clone(),

            id: Uuid::parse_str(&session.id.0).unwrap(),
            account: Account::from_graphql(api_base, session.account),
            created: session.last_seen.0.parse().unwrap(),
            last_seen: session.last_seen.0.parse().unwrap(),
        }
    }

    pub fn id(&self) -> &Uuid { &self.id }
    pub fn account(&self) -> &Account { &self.account }
    pub fn created(&self) -> &DateTime<Utc> { &self.created }
    pub fn last_seen(&self) -> &DateTime<Utc> { &self.last_seen }

    pub async fn destroy(self) -> CoreApiResult<()> { 
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(DestroySession::build(DestroySessionVariables {
                id: schema::Uuid(self.id.to_string())
            })).await?;

        if let Some(errors) = response.errors {
            Err(CoreApiError::GraphQl(errors))
        } else {
            Ok(())
        }
    }
}

pub enum LoginError {
    WrongCredentials,
    Banned,
    ServerLocked,
}

impl CoreApi {
    pub async fn login_username<'a>(&self, username_or_mail: &'a str, password: &'a str) -> CoreApiResult<Result<Session, LoginError>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(LoginUsername::build(LoginUsernameVariables {
                username_or_mail,
                password
            })).await?;

        if let Some(result) = response.data.map(|res| res.create_session) {
            if let Some(session) = result.session {
                Ok(Ok(Session::from_graphql(self, session)))
            } else {
                Ok(Err(
                    match result.error.unwrap() {
                        session_graphql::AuthError::Banned => LoginError::Banned,
                        session_graphql::AuthError::ServerLocked => LoginError::ServerLocked,
                        session_graphql::AuthError::WrongCredentials => LoginError::WrongCredentials,
                    }
                ))
            }
        } else {
            Err(CoreApiError::GraphQl(response.errors.unwrap()))
        }
    }

    pub async fn login_steam(&self, steam_id: SteamId, auth_session_token: &[u8]) -> CoreApiResult<Result<Session, LoginError>> {
        let steam_id = steam_id.steamid32();
        let auth_session_token = general_purpose::STANDARD.encode(auth_session_token);

        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(LoginSteam::build(LoginSteamVariables {
                steam_id: &steam_id,
                web_auth_token: &auth_session_token,
            })).await?;

        if let Some(result) = response.data.map(|res| res.create_session) {
            if let Some(session) = result.session {
                Ok(Ok(Session::from_graphql(self, session)))
            } else {
                Ok(Err(
                    match result.error.unwrap() {
                        session_graphql::AuthError::Banned => LoginError::Banned,
                        session_graphql::AuthError::ServerLocked => LoginError::ServerLocked,
                        session_graphql::AuthError::WrongCredentials => LoginError::WrongCredentials,
                    }
                ))
            }
        } else {
            Err(CoreApiError::GraphQl(response.errors.unwrap()))
        }
    }

    pub async fn get_session(&self, id: &Uuid) -> CoreApiResult<Option<Session>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetSession::build(GetSessionVariables {
                id: schema::Uuid(id.to_string())
            })).await?;

        if let Some(session) = response.data.map(|res| res.session) {
            Ok(session.map(|session| Session::from_graphql(self, session)))
        } else if let Some(errors) = response.errors {
            Err(CoreApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

// Graphql Queries
pub(crate) mod session_graphql {
    use crate::{account_graphql::Account, schema::*};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetSessionVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct LoginUsernameVariables<'a> {
        pub username_or_mail: &'a str,
        pub password: &'a str,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct LoginSteamVariables<'a> {
        pub steam_id: &'a str,
        pub web_auth_token: &'a str,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DestroySessionVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "QueryRoot", variables = "GetSessionVariables")]
    pub struct GetSession {
        #[arguments(id: $id)]
        pub session: Option<Session>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "MutationRoot", variables = "LoginSteamVariables")]
    pub struct LoginSteam {
        #[arguments(auth: { steamAuth: { steamId: $steam_id, webAuthToken: $web_auth_token } })]
        pub create_session: AuthResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "MutationRoot", variables = "LoginUsernameVariables")]
    pub struct LoginUsername {
        #[arguments(auth: { emailAuth: { password: $password, usernameOrMail: $username_or_mail } })]
        pub create_session: AuthResult,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "MutationRoot", variables = "DestroySessionVariables")]
    pub struct DestroySession {
        #[arguments(id: $id)]
        pub destroy_session: Session,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct Session {
        pub account: Account,
        pub created: DateTime,
        pub id: Uuid,
        pub last_seen: DateTime,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "core_service")]
    pub enum AuthError {
        Banned,
        ServerLocked,
        WrongCredentials,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct AuthResult {
        pub error: Option<AuthError>,
        pub session: Option<Session>,
    }
}