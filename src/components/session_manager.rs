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
use mongodb::Database;
use tokio_stream::StreamExt;

use crate::{cluster::{actor::{Actor, ActorResult}, connect_queue, MessageChannel, MessageQueueProducer, MessageQueueConsumer, ClusterMessage}, util::{AnotherlandResult, AnotherlandError}, db::{Account, Session, cluster_database, DatabaseRecord}, NODE};

pub struct SessionManager {
    cluster_db: Database,
    cluster_channel_producer: MessageQueueProducer,
}

impl SessionManager {   
    pub async fn new() -> AnotherlandResult<Self> {
        let (producer, mut consumer) = connect_queue(MessageChannel::ClusterChannel).await?;

        Ok(Self {
            cluster_db: cluster_database().await,
            cluster_channel_producer: producer
        })
    }
}

#[async_trait]
impl Actor for SessionManager {
    fn name(&self) -> &str { "session_manager" }
}

#[actor_actions]
impl SessionManager {
    pub async fn create_session(&mut self, account_id: Uuid) -> AnotherlandResult<Session> {
        if let Some(account) = Account::get(self.cluster_db.clone(), &account_id).await? {
            // Check if we have a session already running and destroy those
            let collection = self.cluster_db.collection::<Session>("sessions");
            let mut result = collection.find(doc! { "account": { "$eq": account_id.to_string() } }, None).await?;

            // Destroy all exsiting sessions for the requested account
            while let Some(session) = result.try_next().await? {
                self.destroy_session(session.id).await?;
            }
            
            Ok(Session::create(self.cluster_db.clone(), &account).await?)
        } else {
            Err(AnotherlandError::app_err("account not found"))
        }
    }

    #[rpc]
    pub async fn get_session(&self, session_id: Uuid) -> AnotherlandResult<Session> {
        if let Some(session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            Ok(session)
        } else {
            Err(AnotherlandError::app_err("session not found"))
        }
    }

    #[rpc]
    pub async fn destroy_session(&mut self, session_id: Uuid) -> AnotherlandResult<()> {
        // first we tell all session handlers, that this session became invalid
        self.cluster_channel_producer.send(ClusterMessage::SessionDestroyed { session_id: session_id.clone() }).await?;

        // and only then we update the database
        if let Some(session) = Session::get(self.cluster_db.clone(), &session_id).await? {
            session.delete(self.cluster_db.clone()).await?;
        }

        Ok(())
    }


}