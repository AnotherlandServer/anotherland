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

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::Uuid;
use bson::doc;
use mongodb::{Database, options::{UpdateModifications, UpdateOptions}};
use serde_derive::{Serialize, Deserialize};

use crate::{cluster::actor::Actor, util::{AnotherlandResult, AnotherlandError}, db::{Account, Session, cluster_database}, NODE};

use super::SessionManager;

#[derive(Default, Serialize, Deserialize)]
struct Status {
    servers_locked: Option<bool>,
}

pub struct Authenticator {
    cluster_db: Database,
    session_manager: Option<ActorRef<SessionManager>>,
    servers_locked: bool,
}

impl Authenticator {
    pub async fn initialize() -> AnotherlandResult<Self> {
        let cluster_db = cluster_database().await;

        let status_record = cluster_db.collection::<Status>("authenticator_status").find_one(None, None).await?.unwrap_or_default();

        Ok(Self {
            cluster_db: cluster_db.clone(),
            session_manager: None,
            servers_locked: status_record.servers_locked.unwrap_or(true),
        })
    }

    fn session_manager_mut(&mut self) -> &mut ActorRef<SessionManager> {
        self.session_manager.as_mut().unwrap()
    }

    async fn update_status_record(&self) -> AnotherlandResult<()> {
        let record = Status {
            servers_locked: Some(self.servers_locked)
        };

        self.cluster_db.collection::<Status>("authenticator_status").update_one(
            doc!{}, 
            doc!{"$set": bson::to_bson(&record).unwrap().as_document().unwrap() }, 
            UpdateOptions::builder().upsert(true).build()).await?;

        Ok(())
    }
}

#[async_trait]
impl Actor for Authenticator {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { Some("authenticator") }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        self.session_manager = Some(NODE.get_actor("session_manager").unwrap());

        // Update status record, in case we changed or added any defaults
        self.update_status_record().await?;

        Ok(()) 
    }
}

pub enum LoginResult {
    Session(Session),
    InvalidCredentials,
    Banned,
    ServersLocked,
}

#[actor_actions]
impl Authenticator {
    #[rpc]
    pub async fn register(&self, username: String, email: Option<String>, password: String) -> AnotherlandResult<Account> {
        let account = Account::create(self.cluster_db.clone(), username, email, password).await?;
        Ok(account)
    }

    #[rpc]
    pub async fn get_account(&self, account_id: Uuid) -> AnotherlandResult<Account> {
        let account = Account::get_by_id(self.cluster_db.clone(), &account_id.into()).await?;
        account.ok_or(AnotherlandError::app_err("account not found"))
    }

    #[rpc]
    pub async fn login(&mut self, username_or_email: String, password: String) -> AnotherlandResult<LoginResult> {
        if let Some(account) = Account::get_by_username_or_mail(self.cluster_db.clone(), &username_or_email).await? {
            if bcrypt::verify(&password, &account.password)? {
                if account.banned {
                    Ok(LoginResult::Banned)
                } else if self.servers_locked && !account.is_gm {
                    // only allow gm logins when servers are locked
                    Ok(LoginResult::ServersLocked)
                } else {
                    let session = self.session_manager_mut().create_session(account.id.clone().into()).await?;
                    Ok(LoginResult::Session(session))
                }
            } else {
                Ok(LoginResult::InvalidCredentials)
            }
        } else {
            Ok(LoginResult::InvalidCredentials)
        }
    }

    #[rpc]
    pub async fn lock_servers(&mut self) -> AnotherlandResult<()> {
        self.servers_locked = true;
        self.update_status_record().await?;

        Ok(())
    }

    #[rpc]
    pub async fn unlock_servers(&mut self) -> AnotherlandResult<()> {
        self.servers_locked = false;
        self.update_status_record().await?;

        Ok(())
    }
}