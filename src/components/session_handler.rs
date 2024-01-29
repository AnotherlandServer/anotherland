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
use atlas::Uuid;
use tokio_util::sync::CancellationToken;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, RwLock, OnceCell};

use crate::cluster::{connect_queue, MessageChannel, ClusterMessage};
use crate::util::{AnotherlandResult, AnotherlandError};
use crate::NODE;
use crate::cluster::RemoteActorRef;
use crate::db::{Session, Account};

use crate::actors::{SessionManager, Authenticator};

/// Manages a cache of local sessions per connection, enriched with custom state data
pub struct SessionHandler {
    authenticator: OnceCell<RemoteActorRef<Authenticator>>,
    session_manager: OnceCell<RemoteActorRef<SessionManager>>,
    session_data: HashMap<Uuid, SessionRef>,
    session_tokens: HashMap<Uuid, CancellationToken>,
    session_id_to_peer: HashMap<Uuid, Uuid>,
    cancellation_token: CancellationToken,
}

impl SessionHandler {
    pub fn new() -> Arc<RwLock<Self>> {
        let token = CancellationToken::new();

        let handler = Arc::new(RwLock::new(Self {
            authenticator: OnceCell::new(),
            session_manager: OnceCell::new(),
            session_data: HashMap::new(),
            session_tokens: HashMap::new(),
            session_id_to_peer: HashMap::new(),
            cancellation_token: token.clone(),
        }));

        tokio::spawn({
            let handler = handler.clone();

            async move {
            
                let (_, mut cluster_channel) = connect_queue(MessageChannel::ClusterChannel).await.unwrap();

                loop {
                    tokio::select! {
                        _ = token.cancelled() => { break; },
                        Ok(msg) = cluster_channel.recv() => {
                            if let ClusterMessage::SessionDestroyed { session_id } = msg {
                                handler.write().await.invalidate_session(session_id);
                            }
                        },
                    }
                }
            }
        });

        handler
    }

    pub async fn initiate(&mut self, peer_id: Uuid, session_id: Uuid, _validation_cookie: Vec<u8>) -> AnotherlandResult<SessionRef> {
        self.initiate_trusted(peer_id, session_id).await
    }

    async fn get_authenticator() -> RemoteActorRef<Authenticator> {
        NODE.get_remote_actor("authenticator").unwrap()
    }

    async fn get_session_manager() -> RemoteActorRef<SessionManager> {
        NODE.get_remote_actor("session_manager").unwrap()
    }

    pub async fn initiate_trusted(&mut self, peer_id: Uuid, session_id: Uuid) -> AnotherlandResult<SessionRef> {
        let authenticator = self.authenticator.get_or_init(Self::get_authenticator).await.clone();
        let session_manager = self.session_manager.get_or_init(Self::get_session_manager).await.clone();

        let session = session_manager.get_session(session_id).await?;
        let account = authenticator.get_account(session.account).await?;

        let token = CancellationToken::new();

        let data = SessionRef{
            data: Arc::new(Mutex::new(SessionData {
                manager: session_manager.clone(),
                account,
                session,
                token: token.clone(),
            })),
            token: token.clone()
        };

        self.session_id_to_peer.insert(session_id, peer_id);
        self.session_data.insert(peer_id, data.clone());
        self.session_tokens.insert(session_id, token);

        Ok(data)
    }

    pub fn get(&self, peer_id: Uuid) -> AnotherlandResult<SessionRef> {
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
            self.session_id_to_peer.remove(&session.lock().await.session().id);
        }
    }

    pub async fn destroy_session(&mut self, session_id: Uuid) {
        if let Some(peer_id) = self.session_id_to_peer.remove(&session_id) {
            self.session_data.remove(&peer_id);
            let _ = self.session_manager.get_or_init(Self::get_session_manager).await.clone().destroy_session(session_id).await;
        }
    }

    pub fn active_sessions(&self) -> usize {
        self.session_data.len()
    }

}

pub struct SessionRef {
    data: Arc<Mutex<SessionData>>,
    token: CancellationToken,
}

impl SessionRef {
    pub async fn invalidated(&self) {
        self.token.cancelled().await
    }
}

impl Clone for SessionRef {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            token: self.token.clone(),
        }
    }
}

impl SessionRef {
    pub async fn lock(&self) -> MutexGuard<'_, SessionData> {
        self.data.lock().await
    }
}

pub struct SessionData {
    manager: RemoteActorRef<SessionManager>,
    account: Account,
    session: Session,
    token: CancellationToken,
}

impl SessionData {
    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub async fn reload(&mut self) -> AnotherlandResult<()> {
        self.session = self.manager.get_session(self.session.id).await?;
        Ok(())
    }

    pub async fn select_realm(&mut self, realm_id: u32) -> AnotherlandResult<()> {
        self.session = self.manager.session_select_realm(self.session.id, realm_id).await?;
        Ok(())
    }

    pub async fn select_world(&mut self, world_id: u16) -> AnotherlandResult<()> {
        self.session = self.manager.session_select_world(self.session.id, world_id).await?;
        Ok(())
    }

    pub async fn select_character(&mut self, character_id: u32) -> AnotherlandResult<()> {
        self.session = self.manager.session_select_character(self.session.id, character_id).await?;
        Ok(())
    }

    pub async fn select_zone(&mut self, zone_guid: Uuid) -> AnotherlandResult<()> {
        self.session = self.manager.session_select_zone(self.session.id, zone_guid).await?;
        Ok(())
    }
    pub async fn invalidated(&self) {
        self.token.cancelled().await
    }
}
