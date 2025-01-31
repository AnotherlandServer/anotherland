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

use derive_builder::Builder;
use worlddef_graphql::{BatchCreateWorlddef, BatchCreateWorlddefVariables, CreateWorlddef, CreateWorlddefVariables, DeleteWorlddef, DeleteWorlddefVariables, GetWorlddef, GetWorlddefVariables, GetWorlddefs, GetWorlddefsVariables, WorldDefFilter, WorldDefInput};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

pub struct WorldDef {
    api_base: Option<RealmApi>,

    id: u16,
    guid: Uuid,
    name: String,
    umap_guid: Uuid,
}

impl WorldDef {
    pub fn id(&self) -> &u16 { &self.id }
    pub fn guid(&self) -> &Uuid { &self.guid }
    pub fn name(&self) -> &str { &self.name }
    pub fn umap_guid(&self) -> &Uuid { &self.umap_guid }

    pub fn new(id: u16, guid: Uuid, name: String, umap_guid: Uuid) -> Self {
        Self {
            api_base: None, 
            id,
            guid,
            name,
            umap_guid
        }
    }

    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteWorlddef::build(DeleteWorlddefVariables {
                    id: self.id as i32
                })).await?;

            if let Some(DeleteWorlddef { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: worlddef_graphql::WorldDef) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id as u16,
            guid: other.guid,
            name: other.name,
            umap_guid: other.umap_guid
        })
    }

    fn into_graphql<'a>(&'a self) -> WorldDefInput<'a> {
        WorldDefInput {
            id: self.id as i32,
            guid: self.guid,
            name: &self.name,
            umap_guid: self.umap_guid,
        }
    }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct WorldDefQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    guid: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    name: Option<String>,
}

impl WorldDefQuery {
    fn get_filter(&self) -> Option<WorldDefFilter<'_>> {
        if self.guid.is_none() && self.name.is_none() {
            None
        } else {
            Some(WorldDefFilter {
                guid: self.guid,
                name: self.name.as_deref()
            })
        }
    }
}

impl RecordQuery for WorldDefQuery {
    type Error = RealmApiError;
    type Record = WorldDef;
    
    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetWorlddefs::build(GetWorlddefsVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetWorlddefs { worlddefs }) = response.data {
            Ok(RecordPage {
                at_end: !worlddefs.page_info.has_next_page,
                last_cursor: worlddefs.page_info.end_cursor,
                records: worlddefs.nodes.into_iter()
                    .map(|worlddef| WorldDef::from_graphql(&self.api_base, worlddef))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl WorldDefQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<WorldDefQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

impl RealmApi {
    pub async fn get_worlddef(&self, id: u16) -> RealmApiResult<Option<WorldDef>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetWorlddef::build(GetWorlddefVariables {
                id: id as i32
            })).await?;

        if let Some(GetWorlddef { worlddef }) = response.data {
            if let Some(worlddef) = worlddef {
                Ok(Some(WorldDef::from_graphql(self, worlddef)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_worlddefs(&self) -> WorldDefQueryBuilder {
        WorldDefQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_worlddefs(&self, world: WorldDef) -> RealmApiResult<WorldDef> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateWorlddef::build(CreateWorlddefVariables {
                id: world.id as i32,
                guid: world.guid,
                name: &world.name,
                umap_guid: world.umap_guid,
            })).await?;

        if let Some(CreateWorlddef { create_worlddef }) = response.data {
            Ok(WorldDef::from_graphql(self, create_worlddef)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_worlddef(&self, worlds: Vec<WorldDef>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateWorlddef::build(BatchCreateWorlddefVariables {
                input: worlds.iter()
                    .map(|worlddef| worlddef.into_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateWorlddef { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod worlddef_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetWorlddefVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateWorlddefVariables<'a> {
        pub input: Vec<WorldDefInput<'a>>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetWorlddefsVariables<'a> {
        pub filter: Option<WorldDefFilter<'a>>,
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateWorlddefVariables<'a> {
        pub guid: Uuid,
        pub id: i32,
        pub name: &'a str,
        pub umap_guid: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteWorlddefVariables {
        pub id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetWorlddefsVariables")]
    pub struct GetWorlddefs {
        #[arguments(filter: $filter, after: $after, first: $first)]
        pub worlddefs: WorldDefConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WorldDefConnection {
        pub nodes: Vec<WorldDef>,
        pub page_info: PageInfo,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetWorlddefVariables")]
    pub struct GetWorlddef {
        #[arguments(id: $id)]
        pub worlddef: Option<WorldDef>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub has_next_page: bool,
        pub end_cursor: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WorldDefFilter<'a> {
        pub guid: Option<Uuid>,
        pub name: Option<&'a str>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateWorlddefVariables")]
    pub struct CreateWorlddef {
        #[arguments(input: { guid: $guid, id: $id, name: $name, umapGuid: $umap_guid })]
        pub create_worlddef: WorldDef,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateWorlddefVariables")]
    pub struct BatchCreateWorlddef {
        #[arguments(input: $input)]
        pub batch_create_worlddefs: Vec<WorldDef>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteWorlddefVariables")]
    pub struct DeleteWorlddef {
        #[arguments(id: $id)]
        pub delete_worlddef: Option<WorldDef>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WorldDef {
        pub guid: Uuid,
        pub id: i32,
        pub name: String,
        pub umap_guid: Uuid,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct WorldDefInput<'a> {
        pub id: i32,
        pub guid: Uuid,
        pub name: &'a str,
        pub umap_guid: Uuid,
    }
}