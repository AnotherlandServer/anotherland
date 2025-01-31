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

use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use session_ext_graphql::{GetSessionState, GetSessionStateVariables, JoinGame, JoinGameVariables};
use toolkit::{anyhow, types::{AvatarId, Uuid}};

use crate::{RealmApi, RealmApiError, RealmApiResult};

pub struct SessionState {
    id: Uuid,
    avatar: AvatarId,
    character: Uuid,
}

impl SessionState {
    fn from_graphql(other: session_ext_graphql::SessionState) -> RealmApiResult<Self> {
        Ok(SessionState {
            id: other.id,
            avatar: other.avatar.parse()
                .map_err(|_| anyhow::Error::msg("invalid avatar id"))?,
            character: other.character,
        })
    }

    pub fn id(&self) -> &Uuid { &self.id }
    pub fn avatar(&self) -> &AvatarId { &self.avatar }
    pub fn character(&self) -> &Uuid { &self.character }
}

impl RealmApi {
    pub async fn get_session_state(&self, id: Uuid) -> RealmApiResult<Option<SessionState>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetSessionState::build(GetSessionStateVariables {
                id
            })).await?;

        if let Some(GetSessionState { session_state }) = response.data {
            if let Some(session_state) = session_state {
                Ok(Some(SessionState::from_graphql(session_state)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn join_game(&self, id: Uuid, character_id: Uuid) -> RealmApiResult<SessionState> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(JoinGame::build(JoinGameVariables {
                id,
                character_id,
            })).await?;

        if let Some(JoinGame { join_game }) = response.data {
            Ok(SessionState::from_graphql(join_game)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod session_ext_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetSessionStateVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct JoinGameVariables {
        pub character_id: Uuid,
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetSessionStateVariables")]
    pub struct GetSessionState {
        #[arguments(id: $id)]
        pub session_state: Option<SessionState>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "JoinGameVariables")]
    pub struct JoinGame {
        #[arguments(characterId: $character_id, id: $id)]
        pub join_game: SessionState,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct SessionState {
        pub id: Uuid,
        pub avatar: String,
        pub character: Uuid,
    }
}