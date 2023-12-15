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
use mongodb::Database;

use crate::{cluster::actor::Actor, util::AnotherlandResult, db::{Account, Session, cluster_database}, NODE};

use super::SessionManager;

pub struct Authenticator {
    cluster_db: Database,
    session_manager: Option<ActorRef<SessionManager>>,
}

impl Authenticator {
    pub async fn new() -> Self {
        Self {
            cluster_db: cluster_database().await,
            session_manager: None,
        }
    }
}

#[async_trait]
impl Actor for Authenticator {
    fn name(&self) -> &str { "authenticator" }

    async fn pre_start(&mut self) -> AnotherlandResult<()> { 
        self.session_manager = Some(NODE.get_actor("session_manager").unwrap());

        Ok(()) 
    }
}

pub enum LoginResult {
    Session(Session),
    InvalidCredentials,
    Banned,
}

#[actor_actions]
impl Authenticator {
    #[rpc]
    pub async fn register(&self, username: String, email: Option<String>, password: String) -> AnotherlandResult<Account> {
        let account = Account::create(self.cluster_db.clone(), username, email, password).await?;
        Ok(account)
    }

    #[rpc]
    pub async fn login(&self, username_or_email: String, password: String) -> AnotherlandResult<LoginResult> {
        if let Some(account) = Account::get_by_username_or_mail(self.cluster_db.clone(), &username_or_email).await? {
            if account.banned {
                Ok(LoginResult::Banned)
            } else {
                if bcrypt::verify(&password, &account.password)? {
                    
                }
                todo!()
            }
        } else {
            Ok(LoginResult::InvalidCredentials)
        }
    }
}