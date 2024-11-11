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

use std::sync::Arc;

use async_graphql::{futures_util::TryStreamExt, Context, Enum, Error, InputObject, Object, OneofObject, SimpleObject};
use bson::{doc, Uuid};
use chrono::{DateTime, Utc};
use database::DatabaseRecord;
use mongodb::Database;

use crate::{db, proto::{CoreNotification, CoreServer}};

use super::account::Account;

#[derive(Default)]
pub struct SessionRoot;

#[derive(Default)]
pub struct SessionMutationRoot;

#[Object]
impl SessionRoot {
    async fn session(&self, ctx: &Context<'_>, id: Uuid) -> Result<Session, Error> {
        let db = ctx.data::<Database>()?.clone();
        if 
            let Some(session) = db::Session::get(&db, &id).await? &&
            let Some(account) = db::Account::get(&db, &session.account).await?
        {
            Ok(Session::from_db(session, account))
        } else {
            Err(Error::new("not found"))
        }
    }
}

#[Object]
impl SessionMutationRoot {
    async fn create_session(&self, ctx: &Context<'_>, auth: AuthInfo) -> Result<AuthResult, Error> {
        let db = ctx.data::<Database>()?.clone();

        let status = db::Status::get(&db, &()).await?.unwrap_or_default();

        match auth {
            AuthInfo::EMailAuth(auth) => {
                if let Some(mut account) = db::Account::get_by_username_or_mail(&db, &auth.username_or_mail).await? {
                    if let db::Credentials::Username{password, ..} = &account.credentials {
                        if !password.check_password(auth.password) {
                            return Ok(AuthResult {
                                session: None,
                                error: Some(AuthError::WrongCredentials)
                            });
                        }

                        if account.banned {
                            return Ok(AuthResult {
                                session: None,
                                error: Some(AuthError::Banned)
                            });
                        }

                        if status.cluster_locked.unwrap_or_default() && !account.is_gm {
                            return Ok(AuthResult {
                                session: None,
                                error: Some(AuthError::ServerLocked)
                            });
                        }

                        // Force logout any existing sessions for this account, as there
                        // can only be one active session per account.
                        let _ = self.force_logout_account(ctx, account.id).await?;

                        account.record_login();
                        account.save(&db).await?;

                        let session = db::Session::create(&db, &account).await?;
                        Ok(AuthResult {
                            session: Some(Session::from_db(session, account)),
                            error: None,
                        })
                    } else{
                        Ok(AuthResult {
                            session: None,
                            error: Some(AuthError::WrongCredentials)
                        })
                    }
                } else {
                    Ok(AuthResult {
                        session: None,
                        error: Some(AuthError::WrongCredentials)
                    })
                }
            },
            AuthInfo::SteamAuth(auth) => {
                if let Some(mut account) = db::Account::get_by_steam_id(&db, &auth.steam_id).await? {
                    if account.banned {
                        return Ok(AuthResult {
                            session: None,
                            error: Some(AuthError::Banned)
                        });
                    }

                    if status.cluster_locked.unwrap_or_default() && !account.is_gm {
                        return Ok(AuthResult {
                            session: None,
                            error: Some(AuthError::ServerLocked)
                        });
                    }

                    // Force logout any existing sessions for this account, as there
                    // can only be one active session per account.
                    let _ = self.force_logout_account(ctx, account.id).await?;

                    account.record_login();
                    account.save(&db).await?;

                    let session = db::Session::create(&db, &account).await?;
                    Ok(AuthResult {
                        session: Some(Session::from_db(session, account)),
                        error: None,
                    })
                } else {
                    Ok(AuthResult {
                        session: None,
                        error: Some(AuthError::WrongCredentials)
                    })
                }
            },
        }
    }

    async fn destroy_session(&self, ctx: &Context<'_>, id: Uuid) -> Result<Session, Error> {
        let db = ctx.data::<Database>()?.clone();
        let socket = ctx.data::<Arc<CoreServer>>()?.clone();
        
        if let Some(session) = db::Session::get(&db, &id).await? {
            session.delete(&db).await?;
            socket.notify(CoreNotification::SessionTerminated(id)).await?;

            if let Some(account) = db::Account::get(&db, &session.account).await? {
                Ok(Session::from_db(session, account))
            } else {
                Err(Error::new("ACCOUNT_NOT_FOUND"))
            }
        } else {
            Err(Error::new("NOT_FOUND"))
        }
    }

    async fn destroy_all_unprivileged_sessions(&self, ctx: &Context<'_>) -> Result<Vec<Session>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let socket = ctx.data::<Arc<CoreServer>>()?.clone();
        let mut cursor = db::Session::list(&db).await?;
        let mut removed_sessions = vec![];

        while let Some(session) = cursor.try_next().await? {
            session.delete(&db).await?;
            socket.notify(CoreNotification::SessionTerminated(session.id)).await?;

            if let Some(account) = db::Account::get(&db, &session.account).await? {
                removed_sessions.push(Session::from_db(session, account));
            }
        }

        Ok(removed_sessions)
    }

    async fn force_logout_account(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<Option<Session>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let socket = ctx.data::<Arc<CoreServer>>()?.clone();
        
        if let Some(session) = db::Session::collection(&db).find_one_and_delete(doc!{"account": account_id}).await? {
            socket.notify(CoreNotification::SessionTerminated(session.id)).await?;

            if let Some(account) = db::Account::get(&db, &session.account).await? {
                Ok(Some(Session::from_db(session, account)))
            } else {
                Err(Error::new("ACCOUNT_NOT_FOUND"))
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(SimpleObject)]
pub struct Session {
    id: uuid::Uuid,
    account: Account,
    created: DateTime<Utc>,
    last_seen: DateTime<Utc>,
}

impl Session {
    fn from_db(session: db::Session, account: db::Account) -> Self {
        Self {
            id: session.id.to_uuid_1(),
            account: Account::from_db(account),
            created: session.created,
            last_seen: session.last_seen,
        }
    }
}

#[derive(InputObject)]
pub struct EMailAuthInfo {
    pub username_or_mail: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct SteamAuthInfo {
    pub steam_id: String,
    pub web_auth_token: String,
}

#[derive(OneofObject)]
pub enum AuthInfo {
    EMailAuth(EMailAuthInfo),
    SteamAuth(SteamAuthInfo)
}

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
pub enum AuthError {
    Banned,
    ServerLocked,
    WrongCredentials,
}

#[derive(SimpleObject)]
pub struct AuthResult {
    pub session: Option<Session>,
    pub error: Option<AuthError>,
}