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
use navmesh_tile_graphql::{BatchCreateNavmeshTiles, BatchCreateNavmeshTilesVariables, CreateNavmeshTile, CreateNavmeshTileVariables, DeleteNavmeshTile, DeleteNavmeshTileVariables, GetNavmeshTile, GetNavmeshTileVariables, GetNavmeshTiles, GetNavmeshTilesVariables, NavmeshTileFilter, NavmeshTileInput};
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct NavmeshTileQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    mesh_id: Option<Uuid>,
    
    #[builder(setter(strip_option), default)]
    tile_x: Option<i32>,
    
    #[builder(setter(strip_option), default)]
    tile_y: Option<i32>,
}

impl NavmeshTileQuery {
    fn get_filter(&self) -> Option<NavmeshTileFilter> {
        if self.mesh_id.is_none() && self.tile_x.is_none() && self.tile_y.is_none() {
            None
        } else {
            Some(NavmeshTileFilter {
                mesh_id: self.mesh_id,
                tile_x: self.tile_x,
                tile_y: self.tile_y,
            })
        }
    }
}

impl RecordQuery for NavmeshTileQuery {
    type Record = NavmeshTile;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetNavmeshTiles::build(GetNavmeshTilesVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetNavmeshTiles { navmesh_tiles }) = response.data {
            Ok(RecordPage {
                at_end: !navmesh_tiles.page_info.has_next_page,
                last_cursor: navmesh_tiles.page_info.end_cursor,
                records: navmesh_tiles.nodes.into_iter()
                    .map(|tile| NavmeshTile::from_graphql(&self.api_base, tile))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl NavmeshTileQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<NavmeshTileQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct NavmeshTile {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    pub id: Uuid,
    pub mesh_id: Uuid,
    pub tile_x: i32,
    pub tile_y: i32,
    pub data: String,
}

impl NavmeshTile {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteNavmeshTile::build(DeleteNavmeshTileVariables {
                    id: self.id
                })).await?;

            if let Some(DeleteNavmeshTile { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: navmesh_tile_graphql::NavmeshTile) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id,
            mesh_id: other.mesh_id,
            tile_x: other.tile_x,
            tile_y: other.tile_y,
            data: other.data,
        })
    }

    fn as_graphql(&self) -> NavmeshTileInput<'_> {
        NavmeshTileInput {
            id: self.id,
            mesh_id: self.mesh_id,
            tile_x: self.tile_x,
            tile_y: self.tile_y,
            data: &self.data,
        }
    }
}

impl RealmApi {
    pub async fn get_navmesh_tile(&self, id: Uuid) -> RealmApiResult<Option<NavmeshTile>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetNavmeshTile::build(GetNavmeshTileVariables {
                id
            })).await?;

        if let Some(GetNavmeshTile { navmesh_tile }) = response.data {
            if let Some(navmesh_tile) = navmesh_tile {
                Ok(Some(NavmeshTile::from_graphql(self, navmesh_tile)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_navmesh_tiles(&self) -> NavmeshTileQueryBuilder {
        NavmeshTileQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_navmesh_tile(&self, tile: NavmeshTile) -> RealmApiResult<NavmeshTile> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateNavmeshTile::build(CreateNavmeshTileVariables {
                input: tile.as_graphql()
            })).await?;

        if let Some(CreateNavmeshTile { create_navmesh_tile }) = response.data {
            Ok(NavmeshTile::from_graphql(self, create_navmesh_tile)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_navmesh_tiles(&self, tiles: Vec<NavmeshTile>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateNavmeshTiles::build(BatchCreateNavmeshTilesVariables {
                input: tiles.iter()
                    .map(|tile| tile.as_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateNavmeshTiles { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod navmesh_tile_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetNavmeshTilesVariables<'a> {
        pub filter: Option<NavmeshTileFilter>,
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateNavmeshTilesVariables<'a> {
        pub input: Vec<NavmeshTileInput<'a>>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetNavmeshTileVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteNavmeshTileVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateNavmeshTileVariables<'a> {
        pub input: NavmeshTileInput<'a>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetNavmeshTilesVariables")]
    pub struct GetNavmeshTiles {
        #[arguments(filter: $filter, after: $after, first: $first)]
        pub navmesh_tiles: NavmeshTileConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetNavmeshTileVariables")]
    pub struct GetNavmeshTile {
        #[arguments(id: $id)]
        pub navmesh_tile: Option<NavmeshTile>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshTileConnection {
        pub nodes: Vec<NavmeshTile>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteNavmeshTileVariables")]
    pub struct DeleteNavmeshTile {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_navmesh_tile: Option<NavmeshTile>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateNavmeshTileVariables")]
    pub struct CreateNavmeshTile {
        #[arguments(input: $input)]
        pub create_navmesh_tile: NavmeshTile,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshTile {
        pub id: Uuid,
        pub mesh_id: Uuid,
        pub tile_x: i32,
        pub tile_y: i32,
        pub data: String,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateNavmeshTilesVariables")]
    pub struct BatchCreateNavmeshTiles {
        #[arguments(input: $input)]
        #[allow(dead_code)]
        pub batch_create_navmesh_tiles: Vec<NavmeshTile>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshTileInput<'a> {
        pub id: Uuid,
        pub mesh_id: Uuid,
        pub tile_x: i32,
        pub tile_y: i32,
        pub data: &'a str,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct NavmeshTileFilter {
        pub mesh_id: Option<Uuid>,
        pub tile_x: Option<i32>,
        pub tile_y: Option<i32>,
    }
}