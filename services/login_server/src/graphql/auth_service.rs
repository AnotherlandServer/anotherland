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

#[cynic::schema("auth_service")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct FindAccountQueryVariables {
    pub steam_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service", graphql_type = "QueryRoot", variables = "FindAccountQueryVariables")]
pub struct FindAccountQuery {
    #[arguments(authQuery: { steam: { steamId: $steam_id } })]
    pub find_account: Option<Account>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct RegisterSteamAccountVariables {
    pub steam_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service", graphql_type = "MutationRoot", variables = "RegisterSteamAccountVariables")]
pub struct RegisterSteamAccount {
    #[arguments(steamId: $steam_id)]
    pub register_steam_account: Account,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct LoginAccountVariables {
    pub auth: AuthInfo,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service", graphql_type = "MutationRoot", variables = "LoginAccountVariables")]
pub struct LoginAccount {
    #[arguments(auth: $auth)]
    pub create_session: Session,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service")]
pub struct Session {
    pub id: Uuid,
    pub account: SessionAccount,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service", graphql_type = "Account")]
pub struct SessionAccount {
    pub id: Uuid,
    pub numeric_id: i32,
    pub credentials: Credentials,
}

#[derive(cynic::InlineFragments, Debug)]
#[cynic(schema = "auth_service")]
pub enum Credentials {
    SteamCredentials(SteamCredentials),
    UsernameCredentials(UsernameCredentials),
    #[cynic(fallback)]
    Unknown
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service")]
pub struct UsernameCredentials {
    pub email: Option<String>,
    pub username: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service")]
pub struct SteamCredentials {
    pub steam_id: String,
}


#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "auth_service")]
pub struct AuthInfo {
    #[cynic(skip_serializing_if="Option::is_none")]
    pub email_auth: Option<EmailAuthInfo>,
    #[cynic(skip_serializing_if="Option::is_none")]
    pub steam_auth: Option<SteamAuthInfo>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "auth_service")]
pub struct SteamAuthInfo {
    pub steam_id: String,
    pub web_auth_token: String,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "auth_service")]
pub struct EmailAuthInfo {
    pub username_or_mail: String,
    pub password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service", graphql_type = "QueryRoot")]
pub struct RealmList {
    pub realms: Vec<Realm>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service")]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub population: f64,
    pub endpoint: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "auth_service")]
pub struct Account {
    pub id: Uuid,
}

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "UUID")]
pub struct Uuid(pub String);