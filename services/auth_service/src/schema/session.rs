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

use async_graphql::{Context, Error, InputObject, Object, OneofObject, SimpleObject, futures_util::TryStreamExt};
use bson::{doc, Uuid};
use chrono::{DateTime, Utc};
use database::DatabaseRecord;
use messages::{auth::AuthSrvEvent, message::StructuredMessage};
use mongodb::Database;
use zeromq::SocketSend;

use crate::{db, EventSocket};

use super::account::Account;

#[derive(Default)]
pub struct SessionRoot;

#[derive(Default)]
pub struct SessionMutationRoot;

#[Object]
impl SessionRoot {
    async fn active_sessions(&self, ctx: &Context<'_>) -> Result<Vec<Session>, Error> {
        todo!()
    }
}

#[Object]
impl SessionMutationRoot {
    async fn create_session(&self, ctx: &Context<'_>, auth: AuthInfo) -> Result<Session, Error> {
        let db = ctx.data::<Database>()?.clone();

        let status = db::Status::get(&db, &()).await?.unwrap_or_default();

        match auth {
            AuthInfo::EMailAuth(auth) => {
                if let Some(mut account) = db::Account::get_by_username_or_mail(&db, &auth.username_or_mail).await? {
                    if let db::Credentials::Username{password, ..} = &account.credentials {
                        if !password.check_password(auth.password) {
                            return Err(Error::new("WRONG_CREDENTIALS"));
                        }

                        if account.banned {
                            return Err(Error::new("ACCOUNT_BANNED"));
                        }

                        if status.cluster_locked.unwrap_or_default() && !account.is_gm {
                            return Err(Error::new("SERVER_LOCKED"));
                        }

                        // Force logout any existing sessions for this account, as there
                        // can only be one active session per account.
                        let _ = self.force_logout_account(ctx, account.id).await?;

                        account.record_login();
                        account.save(&db).await?;

                        let session = db::Session::create(&db, &account).await?;
                        Ok(Session::from_db(session, account))
                    } else{
                        Err(Error::new("WRONG_CREDENTIALS"))
                    }
                } else {
                    Err(Error::new("WRONG_CREDENTIALS"))
                }
            },
            AuthInfo::SteamAuth(auth) => {
                if let Some(mut account) = db::Account::get_by_steam_id(&db, &auth.steam_id).await? {
                    if account.banned {
                        return Err(Error::new("ACCOUNT_BANNED"));
                    }

                    if status.cluster_locked.unwrap_or_default() && !account.is_gm {
                        return Err(Error::new("SERVER_LOCKED"));
                    }

                    // Force logout any existing sessions for this account, as there
                    // can only be one active session per account.
                    let _ = self.force_logout_account(ctx, account.id).await?;

                    account.record_login();
                    account.save(&db).await?;

                    let session = db::Session::create(&db, &account).await?;
                    Ok(Session::from_db(session, account))
                } else {
                    Err(Error::new("WRONG_CREDENTIALS"))
                }
            },
        }
    }

    async fn destroy_session(&self, ctx: &Context<'_>, id: Uuid) -> Result<Session, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut socket = ctx.data::<EventSocket>()?.lock().await;
        
        if let Some(session) = db::Session::get(&db, &id).await? {
            session.delete(&db).await?;
            socket.send(AuthSrvEvent::SessionTerminated(id).into_message()?).await?;

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
        let mut socket = ctx.data::<EventSocket>()?.lock().await;
        let mut cursor = db::Session::list(&db).await?;
        let mut removed_sessions = vec![];

        while let Some(session) = cursor.try_next().await? {
            session.delete(&db).await?;
            socket.send(AuthSrvEvent::SessionTerminated(session.id).into_message()?).await?;

            if let Some(account) = db::Account::get(&db, &session.account).await? {
                removed_sessions.push(Session::from_db(session, account));
            }
        }

        Ok(removed_sessions)
    }

    async fn force_logout_account(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<Option<Session>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut socket = ctx.data::<EventSocket>()?.lock().await;
        
        if let Some(session) = db::Session::collection(&db).find_one_and_delete(doc!{"account": account_id}).await? {
            socket.send(AuthSrvEvent::SessionTerminated(session.id).into_message()?).await?;

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

#[derive(SimpleObject, Clone, Debug)]
pub struct Session {
    id: uuid::Uuid,
    account: Account,
    is_gm: bool,
    //active_realm: Option<Realm>,
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

impl Session {
    fn from_db(session: db::Session, account: db::Account) -> Self {
        Self {
            id: session.id.to_uuid_1(),
            account: Account::from_db(account),
            is_gm: session.is_gm,
            //active_realm: None,
            active_character_id: None,
            created: session.created,
            last_seen: session.last_seen,
            session_id: session.id,
            realm_id: session.realm_id,
            character_id: session.character_id,
            zone_id: session.zone_guid,
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