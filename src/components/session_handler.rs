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

use std::collections::HashMap;
use std::marker::PhantomData;
use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::Uuid;
use futures::select;
use log::{debug, trace};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, mpsc};

use crate::cluster::{connect_queue, MessageChannel, ClusterMessage};
use crate::util::{AnotherlandResult, AnotherlandError};
use crate::{cluster::actor::Actor, NODE};
use crate::cluster::RemoteActorRef;
use crate::db::{Session, Account};

use super::{SessionManager, Authenticator};

/// Manages a cache of local sessions per connection, enriched with custom state data
pub struct SessionHandler<T: 'static + Default + Send + Sync> {
    name: String,
    authenticator: Option<RemoteActorRef<Authenticator>>,
    session_manager: Option<RemoteActorRef<SessionManager>>,
    session_data: HashMap<Uuid, SessionRef<T>>,
    session_tokens: HashMap<Uuid, CancellationToken>,
    session_id_to_peer: HashMap<Uuid, Uuid>,
    cancellation_token: CancellationToken,
    subtasks: TaskTracker,
    _marker: PhantomData<T>,
}

impl <T: 'static +  Send + Sync + Default> SessionHandler<T> {
    pub async fn initialize(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            authenticator: None,
            session_manager: None,
            session_data: HashMap::new(),
            session_tokens: HashMap::new(),
            session_id_to_peer: HashMap::new(),
            cancellation_token: CancellationToken::new(),
            subtasks: TaskTracker::new(),
            _marker: PhantomData,
        }
    }

    pub fn authenticator(&self) -> &RemoteActorRef<Authenticator> {
        self.authenticator.as_ref().unwrap()
    }

    pub fn session_manager(&self) -> &RemoteActorRef<SessionManager> {
        self.session_manager.as_ref().unwrap()
    }

    pub fn session_manager_mut(&mut self) -> &mut RemoteActorRef<SessionManager> {
        self.session_manager.as_mut().unwrap()
    }
}

#[async_trait]
impl<T: 'static + Send + Sync + Default> Actor for SessionHandler<T> {
    fn name(&self) -> &str { &self.name }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        self.authenticator = Some(NODE.get_remote_actor("authenticator").unwrap());
        self.session_manager = Some(NODE.get_remote_actor("session_manager").unwrap());

        Ok(()) 
    }


    async fn started(&mut self) -> AnotherlandResult<()> {
        let actor_name = self.name().to_owned();
        let cancellation_token = self.cancellation_token.clone();

        self.subtasks.spawn(async move {
            let mut local_actor = NODE.get_actor::<Self>(&actor_name).unwrap();
            let (_, mut cluster_channel) = connect_queue(MessageChannel::ClusterChannel).await.unwrap();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => { break; },
                    Ok(msg) = cluster_channel.recv() => {
                        match msg {
                            ClusterMessage::SessionDestroyed { session_id } => {
                                local_actor.invalidate_session(session_id).await;
                            },
                            _ => (),
                        }
                    },
                }
            }
        });

        Ok(()) 
    }

    async fn stopping(&mut self) -> AnotherlandResult<()> {
        self.cancellation_token.cancel();
        self.subtasks.close();
        self.subtasks.wait().await;

        Ok(()) 
    }
}

#[actor_actions]
impl<T: 'static + Send + Sync + Default> SessionHandler<T> {
    pub async fn initiate(&mut self, peer_id: Uuid, session_id: Uuid, validation_cookie: Vec<u8>) -> AnotherlandResult<SessionRef<T>> {
        self.initiate_trusted(peer_id, session_id).await
    }

    pub async fn initiate_trusted(&mut self, peer_id: Uuid, session_id: Uuid) -> AnotherlandResult<SessionRef<T>> {
        let session = self.session_manager().get_session(session_id.clone()).await?;
        let account = self.authenticator().get_account(session.account.clone()).await?;

        let token = CancellationToken::new();

        let data = SessionRef{
            data: Arc::new(Mutex::new(SessionData {
                handler: NODE.get_actor::<Self>(self.name()).unwrap(),
                account,
                session,
                token: token.clone(),
                data: T::default(),
            })),
            token: token.clone()
        };

        self.session_id_to_peer.insert(session_id.clone(), peer_id.clone());
        self.session_data.insert(peer_id, data.clone());
        self.session_tokens.insert(session_id, token);

        Ok(data)
    }

    pub async fn get(&self, peer_id: Uuid) -> AnotherlandResult<SessionRef<T>> {
        if let Some(session_s) = self.session_data.get(&peer_id) {
            Ok(session_s.to_owned())
        } else {
            Err(AnotherlandError::app_err("no session for peer"))
        }
    }

    pub fn invalidate_session(&mut self, session_id: Uuid) {
        // perform a quick reverse-lookup of session id to peer id
        if let Some(peer_id) = self.session_id_to_peer.remove(&session_id) {
            self.session_data.remove(&peer_id);
        }

        // cancel session
        if let Some(token) = self.session_tokens.remove(&session_id) {
            token.cancel();
        }
    }

    pub async fn forget_peer(&mut self, peer_id: Uuid) {
        if let Some(session) = self.session_data.remove(&peer_id) {
            self.session_id_to_peer.remove(&session.lock().await.session().id.into());
        }
    }

    pub async fn destroy_session(&mut self, session_id: Uuid) -> AnotherlandResult<()> {
        self.session_manager_mut().destroy_session(session_id).await
    }

    pub async fn session_select_realm(&self, session_id: Uuid, realm_id: u32) -> AnotherlandResult<Session> {
        self.session_manager().session_select_realm(session_id, realm_id).await
    }

    pub async fn session_select_character(&self, session_id: Uuid, character_id: u32) -> AnotherlandResult<Session> {
        self.session_manager().session_select_character(session_id, character_id).await
    }

    pub async fn session_select_world(&self, session_id: Uuid, world_id: u16) -> AnotherlandResult<Session> {
        self.session_manager().session_select_world(session_id, world_id).await
    }

    pub async fn session_select_zone(&self, session_id: Uuid, zone_id: Uuid) -> AnotherlandResult<Session> {
        self.session_manager().session_select_zone(session_id, zone_id).await
    }

    pub fn active_sessions(&self) -> usize {
        self.session_data.len()
    }

    async fn get_session(&self, session_id: Uuid) -> AnotherlandResult<Session> {
        self.session_manager().get_session(session_id.clone()).await
    }
}

pub struct SessionRef<T: 'static + Default + Send + Sync> {
    data: Arc<Mutex<SessionData<T>>>,
    token: CancellationToken,
}

impl<T: 'static + Default + Send + Sync> SessionRef<T> {
    pub async fn invalidated(&self) {
        self.token.cancelled().await
    }
}

impl<T: 'static + Default + Send + Sync> Clone for SessionRef<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            token: self.token.clone(),
        }
    }
}

impl <T: 'static + Default + Send + Sync>SessionRef<T> {
    pub async fn lock(&self) -> MutexGuard<'_, SessionData<T>> {
        self.data.lock().await
    }
}

pub struct SessionData<T: 'static + Default + Send + Sync> {
    handler: ActorRef<SessionHandler<T>>,
    account: Account,
    session: Session,
    token: CancellationToken,
    data: T,
}

impl<T: 'static + Default + Send + Sync> Drop for SessionData<T> {
    fn drop(&mut self) {
        debug!("Drop sessiondata");
    }
}

impl<T: 'static + Default + Send + Sync> SessionData<T> {
    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub async fn reload(&mut self) -> AnotherlandResult<()> {
        self.session = self.handler.get_session(self.session.id.clone().into()).await?;
        Ok(())
    }

    pub async fn select_realm(&mut self, realm_id: u32) -> AnotherlandResult<()> {
        self.session = self.handler.session_select_realm(self.session.id.clone().into(), realm_id).await?;
        Ok(())
    }

    pub async fn select_world(&mut self, world_id: u16) -> AnotherlandResult<()> {
        self.session = self.handler.session_select_world(self.session.id.clone().into(), world_id).await?;
        Ok(())
    }

    pub async fn select_character(&mut self, character_id: u32) -> AnotherlandResult<()> {
        self.session = self.handler.session_select_character(self.session.id.clone().into(), character_id).await?;
        Ok(())
    }

    pub async fn select_zone(&mut self, zone_guid: Uuid) -> AnotherlandResult<()> {
        self.session = self.handler.session_select_zone(self.session.id.clone().into(), zone_guid).await?;
        Ok(())
    }

    pub async fn invalidated(&self) {
        self.token.cancelled().await
    }
}
