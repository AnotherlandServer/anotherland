// Copyright (C) 2025 AnotherlandServer
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

use std::{collections::{HashMap, HashSet}, ops::Deref, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use core_api::CoreApi;
use rand::random;
use tokio::sync::Mutex;
use toolkit::types::{AvatarId, AvatarType, Uuid};

use crate::{error::RealmResult, proto::RealmServer, CHAT_ROUTER};

struct SessionManagerData {
    core_api: CoreApi,
    realm_server: Arc<RealmServer>,
    states: HashMap<Uuid, Arc<SessionState>>,
    avatars: HashMap<AvatarId, Arc<SessionState>>,
    avatar_ids: HashSet<AvatarId>,
}

#[derive(Clone)]
pub struct SessionState {
    pub id: Uuid,
    pub avatar_id: AvatarId,
    pub character_id: Uuid,
    pub zone: Option<Uuid>,
    pub instance: Option<Uuid>,
    pub cluster_node: Option<Uuid>,
}

#[derive(Clone)]
pub struct SessionManager(Arc<Mutex<SessionManagerData>>);

impl SessionManager {
    pub async fn new(core_api: CoreApi, realm_server: Arc<RealmServer>) -> RealmResult<Self> {
        Ok(Self(Arc::new(Mutex::new(
            SessionManagerData { 
                core_api, 
                realm_server, 
                states: HashMap::new(),
                avatars: HashMap::new(),
                avatar_ids: HashSet::new(),
            }
        ))))
    }

    pub async fn get_state(&self, session: Uuid) -> Option<Arc<SessionState>> {
        let s = self.0.lock().await;
        s.states.get(&session).cloned()
    }

    pub async fn get_state_for_avatar(&self, avatar_id: AvatarId) -> Option<Arc<SessionState>> {
        let s = self.0.lock().await;
        s.avatars.get(&avatar_id).cloned()
    }

    pub async fn join_game(&self, session: Uuid, character_id: Uuid) -> RealmResult<Arc<SessionState>> {
        let mut s = self.0.lock().await;
        
        let avatar_id = loop {
            let random_component = random::<u32>();
            let time_component = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;

            let avatar_id = AvatarId::new((time_component as u64) << 40 | (random_component as u64) << 8, AvatarType::Player);
            if s.avatar_ids.contains(&avatar_id) {
                continue;
            } else {
                s.avatar_ids.insert(avatar_id);
                break avatar_id;
            }
        };

        let state = Arc::new(SessionState {
            id: session,
            avatar_id,
            character_id,
            zone: None,
            instance: None,
            cluster_node: None,
        });

        s.states.insert(session, state.clone());
        s.avatars.insert(avatar_id, state.clone());

        CHAT_ROUTER.get().unwrap().connect_session(session).await;

        Ok(state)
    }

    pub async fn update_instance(&self, session: Uuid, zone: Uuid, instance: Option<Uuid>) {
        let mut s = self.0.lock().await;
        
        if let Some(state) = s.states.get(&session).cloned() {
            let mut state = state.deref().clone();
            state.zone = Some(zone);
            state.instance = instance;

            s.states.insert(session, Arc::new(state));
        }
    }

    pub async fn update_cluster_node(&self, session: Uuid, node: Uuid) {
        let mut s = self.0.lock().await;
        
        if let Some(state) = s.states.get(&session).cloned() {
            let mut state = state.deref().clone();
            state.cluster_node = Some(node);

            s.states.insert(session, Arc::new(state));
        }
    }

    pub async fn terminate_session(&self, session: Uuid) {
        let mut s = self.0.lock().await;
        if let Some(state) = s.states.remove(&session) {
            s.avatars.remove(&state.avatar_id);

            // Free avatar id
            s.avatar_ids.remove(&state.avatar_id);
        }
    }
}