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

use std::net::SocketAddrV4;

use cynic::{http::ReqwestExt, QueryBuilder};
use realm_graphql::{GetRealm, GetRealmVariables, GetRealms};

use crate::{CoreApi, CoreApiError, CoreApiResult};

pub struct Realm {
    id: i32,
    name: String,
    population: f64,
    endoint: SocketAddrV4,
}

impl Realm {
    pub(crate) fn from_graphql(realm: realm_graphql::Realm) -> Self {
        Self {
            id: realm.id,
            name: realm.name,
            population: realm.population,
            endoint: realm.endpoint.parse().unwrap(),
        }
    }

    pub fn id(&self) -> i32 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn population(&self) -> f64 { self.population }
    pub fn endpoint(&self) -> &SocketAddrV4 { &self.endoint }
}

impl CoreApi {
    pub async fn get_realm(&self, id: i32) -> CoreApiResult<Option<Realm>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetRealm::build(GetRealmVariables {
                id,
            })).await?;

            if let Some(realm) = response.data.map(|res| res.realm) {
                Ok(realm.map(Realm::from_graphql))
            } else {
                Err(CoreApiError::GraphQl(response.errors.unwrap()))
            }
    }

    pub async fn get_realms(&self) -> CoreApiResult<Vec<Realm>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetRealms::build(())).await?;

            if let Some(result) = response.data.map(|res| res.realms) {
                Ok(result.into_iter().map(Realm::from_graphql).collect())
            } else {
                Err(CoreApiError::GraphQl(response.errors.unwrap()))
            }
    }
}

pub(crate) mod realm_graphql {
    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetRealmVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "QueryRoot")]
    pub struct GetRealms {
        pub realms: Vec<Realm>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service", graphql_type = "QueryRoot", variables = "GetRealmVariables")]
    pub struct GetRealm {
        #[arguments(id: $id)]
        pub realm: Option<Realm>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "core_service")]
    pub struct Realm {
        pub id: i32,
        pub name: String,
        pub population: f64,
        pub endpoint: String,
    }
}
