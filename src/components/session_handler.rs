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
use futures::select;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, mpsc};

use atlas::Uuid;

use crate::cluster::{connect_queue, MessageChannel, ClusterMessage};
use crate::util::{AnotherlandResult, AnotherlandError};
use crate::{cluster::actor::Actor, NODE};
use crate::cluster::actor::RemoteActorRef;
use crate::db::Session;

use super::SessionManager;

/// Manages a cache of local sessions per connection, enriched with custom state data
pub struct SessionHandler<T: 'static + Default> {
    name: String,
    session_manager: Option<RemoteActorRef<SessionManager>>,
    session_data: HashMap<Uuid, SessionRef<T>>,
    session_id_to_peer: HashMap<Uuid, Uuid>,
    stop_signal: Option<mpsc::Sender<()>>,
    _marker: PhantomData<T>,
}

impl <T: 'static +  Send + Sync + Default> SessionHandler<T> {
    pub async fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            session_manager: None,
            session_data: HashMap::new(),
            session_id_to_peer: HashMap::new(),
            stop_signal: None,
            _marker: PhantomData,
        }
    }

    pub fn session_manager(&self) -> &RemoteActorRef<SessionManager> {
        self.session_manager.as_ref().unwrap()
    }

    async fn started(&mut self) -> AnotherlandResult<()> {
        let (producer, mut consumer) = mpsc::channel(1);

        tokio::spawn(async move {
            let mut local_actor = NODE.get_actor::<Self>("session_handler").unwrap();
            let (_, mut cluster_channel) = connect_queue(MessageChannel::ClusterChannel).await.unwrap();

            loop {
                tokio::select! {
                    _ = consumer.recv() => { break; },
                    Ok(msg) = cluster_channel.recv() => {
                        match msg {
                            ClusterMessage::SessionDestroyed { session_id } => {
                                local_actor.forget_session(session_id).await;
                            },
                            _ => (),
                        }
                    },
                }
            }
        });

        self.stop_signal = Some(producer);

        Ok(()) 
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }
}

#[async_trait]
impl<T: 'static + Send + Sync + Default> Actor for SessionHandler<T> {
    fn name(&self) -> &str { &self.name }

    async fn pre_start(&mut self) -> AnotherlandResult<()> { 
        self.session_manager = Some(NODE.get_remote_actor("session_manager").unwrap());

        Ok(()) 
    }
}

#[actor_actions]
impl<T: 'static + Send + Sync + Default> SessionHandler<T> {
    pub async fn initiate(&mut self, peer_id: Uuid, session_id: Uuid, validation_cookie: Vec<u8>) -> AnotherlandResult<SessionRef<T>> {
        self.initiate_trusted(peer_id, session_id).await
    }

    pub async fn initiate_trusted(&mut self, peer_id: Uuid, session_id: Uuid) -> AnotherlandResult<SessionRef<T>> {
        let session = self.session_manager().get_session(session_id).await?;

        let data = SessionRef(Arc::new(Mutex::new(SessionData {
            session,
            data: T::default(),
        })));

        self.session_data.insert(peer_id, data.clone());

        Ok(data)
    }

    pub async fn get(&self, peer_id: Uuid) -> AnotherlandResult<SessionRef<T>> {
        if let Some(session_s) = self.session_data.get(&peer_id) {
            Ok(session_s.to_owned())
        } else {
            Err(AnotherlandError::app_err("no session for peer"))
        }
    }

    pub fn forget_session(&mut self, session_id: Uuid) {
        // perform a quick reverse-lookup of session id to peer id
        if let Some(peer_id) = self.session_id_to_peer.remove(&session_id) {
            self.session_data.remove(&peer_id);
        }
    }
}

pub struct SessionRef<T: 'static + Default>(Arc<Mutex<SessionData<T>>>);

impl<T: 'static + Default> Clone for SessionRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct SessionData<T: 'static + Default> {
    session: Session,
    data: T,
}

impl <T: 'static + Default>SessionRef<T> {
    async fn lock(&self) -> MutexGuard<'_, SessionData<T>> {
        self.0.lock().await
    }
}