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

use std::sync::Arc;

use async_graphql::{Context, Error, Object, SimpleObject};
use core_api::CoreApi;
use database::DatabaseRecord;
use mongodb::{bson::doc, options::ReturnDocument, Database};
use toolkit::types::Uuid;

use crate::{db, session_manager::{self, SessionManager}, SESSION_MANAGER};

#[derive(Default)]
pub struct SessionStateRoot;

#[derive(Default)]
pub struct SessionStateMutationRoot;

#[Object]
impl SessionStateRoot {
    async fn session_state(&self, _ctx: &Context<'_>, id: Uuid) -> Result<Option<SessionState>, Error> {
        Ok(SESSION_MANAGER.get().unwrap().get_state(id).await
            .map(|state| state.into()))
    }
}

#[Object]
impl SessionStateMutationRoot {
    async fn join_game(&self, _ctx: &Context<'_>, id: Uuid, character_id: Uuid) -> Result<SessionState, Error> {
        Ok(SESSION_MANAGER.get().unwrap().join_game(id, character_id).await?.into())
    }
}

#[derive(SimpleObject)]
pub struct SessionState {
    id: Uuid,
    avatar: String,
    character: Uuid,
    zone: Option<Uuid>,
    instance: Option<Uuid>,
}

impl From<Arc<session_manager::SessionState>> for SessionState {
    fn from(value: Arc<session_manager::SessionState>) -> Self {
        Self {
            id: value.id,
            avatar: value.avatar_id.to_string(),
            character: value.character_id,
            zone: value.zone,
            instance: value.instance,
        }
    }
}