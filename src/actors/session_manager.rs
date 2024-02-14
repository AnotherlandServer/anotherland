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

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::Uuid;
use bson::doc;
use log::debug;
use mongodb::Database;
use tokio_stream::StreamExt;

use crate::{cluster::{actor::{Actor}, connect_queue, MessageChannel, MessageQueueProducer, ClusterMessage}, util::{AnotherlandResult, AnotherlandError}, db::{Account, Session, cluster_database, DatabaseRecord}};

pub struct SessionManager {
    cluster_db: Database,
    cluster_channel_producer: MessageQueueProducer,
}

impl SessionManager {   
    pub async fn initialize() -> AnotherlandResult<Self> {
        let (producer, _) = connect_queue(MessageChannel::ClusterChannel).await?;

        Ok(Self {
            cluster_db: cluster_database().await,
            cluster_channel_producer: producer
        })
    }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }
}

#[async_trait]
impl Actor for SessionManager {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { Some("session_manager") }
}

#[actor_actions]
impl SessionManager {
    pub async fn create_session(&mut self, account_id: Uuid) -> AnotherlandResult<Session> {
        if let Some(account) = Account::get(self.cluster_db.clone(), &account_id).await? {
            self.force_logout_account(account_id).await?;
            Ok(Session::create(self.cluster_db.clone(), &account).await?)
        } else {
            Err(AnotherlandError::app_err("account not found"))
        }
    }

    #[rpc]
    pub async fn get_session(&self, session_id: Uuid) -> AnotherlandResult<Session> {
        debug!("Session id: {}", session_id);

        if let Some(session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn session_select_realm(&self, session_id: Uuid, realm_id: u32) -> AnotherlandResult<Session> {
        if let Some(mut session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.select_realm(self.cluster_db.clone(), realm_id).await?;
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn session_select_character(&self, session_id: Uuid, character_id: u32) -> AnotherlandResult<Session> {
        if let Some(mut session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.select_character(self.cluster_db.clone(), character_id).await?;
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn session_select_world(&self, session_id: Uuid, world_id: u16) -> AnotherlandResult<Session> {
        if let Some(mut session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.select_world(self.cluster_db.clone(), world_id).await?;
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn session_select_zone(&self, session_id: Uuid, zone_id: Uuid) -> AnotherlandResult<Session> {
        if let Some(mut session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.select_zone(self.cluster_db.clone(), zone_id).await?;
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn force_logout_account(&mut self, account_id: Uuid) -> AnotherlandResult<()> {
        // Find all session associated with the given account
        let collection = self.cluster_db.collection::<Session>("sessions");
        let mut result = collection.find(doc! { "account": { "$eq": account_id } }, None).await?;

        // Destroy all found sessions
        while let Some(session) = result.try_next().await? {
            self.destroy_session(session.id).await?;
        }
    
        Ok(())
    }

    #[rpc]
    pub async fn destroy_session(&mut self, session_id: Uuid) -> AnotherlandResult<()> {
        // first we tell all session handlers, that this session became invalid
        self.cluster_channel_producer.send(ClusterMessage::SessionDestroyed { session_id }).await?;

        // and only then we update the database
        if let Some(session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.delete(self.cluster_db.clone()).await?;
        }

        Ok(())
    }

    #[rpc]
    pub async fn destroy_all_unprivileged_sessions(&mut self) -> AnotherlandResult<()> {
        // Collect all unprivileged sessions
        let collection = self.cluster_db.collection::<Session>("sessions");
        let mut result = collection.find(doc! { "$not" : { "is_gm": { "$eq": true } } }, None).await?;

        // Destroy all found sessions
        while let Some(session) = result.try_next().await? {
            self.destroy_session(session.id).await?;
        }

        Ok(())
    }
}