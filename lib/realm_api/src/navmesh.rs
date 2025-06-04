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
use derive_builder::Builder;
use navmesh_graphql::{BatchCreateNavmeshs, BatchCreateNavmeshsVariables, CreateNavmesh, CreateNavmeshVariables, DeleteNavmesh, DeleteNavmeshVariables, GetNavmesh, GetNavmeshVariables, GetNavmeshs, GetNavmeshsVariables, NavmeshFilter, NavmeshInput};
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct NavmeshQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    world_id: Option<i32>,

    #[builder(setter(strip_option), default)]
    world_guid: Option<Uuid>,
}

impl NavmeshQuery {
    fn get_filter(&self) -> Option<NavmeshFilter> {
        if self.world_id.is_none() && self.world_guid.is_none() {
            None
        } else {
            Some(NavmeshFilter {
                world_id: self.world_id,
                world_guid: self.world_guid,
            })
        }
    }
}

impl RecordQuery for NavmeshQuery {
    type Record = Navmesh;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetNavmeshs::build(GetNavmeshsVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetNavmeshs { navmeshs }) = response.data {
            Ok(RecordPage {
                at_end: !navmeshs.page_info.has_next_page,
                last_cursor: navmeshs.page_info.end_cursor,
                records: navmeshs.nodes.into_iter()
                    .map(|navmesh| Navmesh::from_graphql(&self.api_base, navmesh))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl NavmeshQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<NavmeshQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Navmesh {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    pub id: Uuid,
    pub world_id: i32,
    pub world_guid: Uuid,
    pub tile_width: i32,
    pub tile_height: i32,
}

impl Navmesh {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteNavmesh::build(DeleteNavmeshVariables {
                    id: self.id
                })).await?;

            if let Some(DeleteNavmesh { .. }) = response.data {
                Ok(())
            } else if let Some(errors) = response.errors {
                Err(RealmApiError::GraphQl(errors))
            } else {
                unreachable!()
            }
        } else {
            Ok(())
        }
    }

    fn from_graphql(api: &RealmApi, other: navmesh_graphql::Navmesh) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id,
            world_id: other.world_id,
            world_guid: other.world_guid,
            tile_width: other.tile_width,
            tile_height: other.tile_height,
        })
    }

    fn as_graphql(&self) -> NavmeshInput {
        NavmeshInput {
            id: self.id,
            world_id: self.world_id,
            world_guid: self.world_guid,
            tile_width: self.tile_width,
            tile_height: self.tile_height,
        }
    }
}

impl RealmApi {
    pub async fn get_navmesh(&self, id: Uuid) -> RealmApiResult<Option<Navmesh>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetNavmesh::build(GetNavmeshVariables {
                id
            })).await?;

        if let Some(GetNavmesh { navmesh }) = response.data {
            if let Some(navmesh) = navmesh {
                Ok(Some(Navmesh::from_graphql(self, navmesh)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_navmeshs(&self) -> NavmeshQueryBuilder {
        NavmeshQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_navmesh(&self, navmesh: Navmesh) -> RealmApiResult<Navmesh> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateNavmesh::build(CreateNavmeshVariables {
                input: navmesh.as_graphql()
            })).await?;

        if let Some(CreateNavmesh { create_navmesh }) = response.data {
            Ok(Navmesh::from_graphql(self, create_navmesh)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_navmeshs(&self, navmeshs: Vec<Navmesh>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateNavmeshs::build(BatchCreateNavmeshsVariables {
                input: navmeshs.iter()
                    .map(|navmesh| navmesh.as_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateNavmeshs { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod navmesh_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetNavmeshsVariables<'a> {
        pub filter: Option<NavmeshFilter>,
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateNavmeshsVariables {
        pub input: Vec<NavmeshInput>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetNavmeshVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteNavmeshVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateNavmeshVariables {
        pub input: NavmeshInput,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetNavmeshsVariables")]
    pub struct GetNavmeshs {
        #[arguments(filter: $filter, after: $after, first: $first)]
        pub navmeshs: NavmeshConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetNavmeshVariables")]
    pub struct GetNavmesh {
        #[arguments(id: $id)]
        pub navmesh: Option<Navmesh>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshConnection {
        pub nodes: Vec<Navmesh>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteNavmeshVariables")]
    pub struct DeleteNavmesh {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_navmesh: Option<Navmesh>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateNavmeshVariables")]
    pub struct CreateNavmesh {
        #[arguments(input: $input)]
        pub create_navmesh: Navmesh,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Navmesh {
        pub id: Uuid,
        pub world_id: i32,
        pub world_guid: Uuid,
        pub tile_width: i32,
        pub tile_height: i32,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateNavmeshsVariables")]
    pub struct BatchCreateNavmeshs {
        #[arguments(input: $input)]
        #[allow(dead_code)]
        pub batch_create_navmeshs: Vec<Navmesh>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshInput {
        pub id: Uuid,
        pub world_id: i32,
        pub world_guid: Uuid,
        pub tile_width: i32,
        pub tile_height: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshFilter {
        pub world_id: Option<i32>,
        pub world_guid: Option<Uuid>,
    }
}

