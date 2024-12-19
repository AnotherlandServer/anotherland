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

use std::{collections::{HashMap, HashSet}, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use core_api::CoreApi;
use rand::random;
use tokio::sync::Mutex;
use toolkit::types::{AvatarId, AvatarType, Uuid};

use crate::{error::RealmResult, proto::RealmServer};

struct SessionManagerData {
    core_api: CoreApi,
    realm_server: Arc<RealmServer>,
    states: HashMap<Uuid, Arc<SessionState>>,
    avatar_ids: HashSet<AvatarId>,
}

pub struct SessionState {
    pub id: Uuid,
    pub avatar_id: AvatarId,
    pub character_id: Uuid,
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
                avatar_ids: HashSet::new(),
            }
        ))))
    }

    pub async fn get_state(&self, session: Uuid) -> RealmResult<Option<Arc<SessionState>>> {
        let s = self.0.lock().await;
        Ok(s.states.get(&session).cloned())
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
        });

        s.states.insert(session, state.clone());

        Ok(state)
    }

    pub async fn terminate_session(&self, session: Uuid) {
        let mut s = self.0.lock().await;
        if let Some(state) = s.states.remove(&session) {
            // Free avatar id
            s.avatar_ids.remove(&state.avatar_id);
        }
    }
}