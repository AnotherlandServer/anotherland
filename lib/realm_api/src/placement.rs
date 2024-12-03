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

use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use derive_builder::Builder;
use futures::io::Cursor;
use obj_params::{Class, GameObjectData};
use placement_graphql::{BatchCreatePlacements, BatchCreatePlacementsVariables, CreatePlacement, CreatePlacementVariables, DeletePlacement, DeletePlacementVariables, GetPlacement, GetPlacementVariables, PlacementInput};
use toolkit::types::Uuid;

use crate::{schema, RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Placement {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    id: Uuid,
    zone_guid: Uuid,
    class: Class,
    content_guid: Uuid,
    editor_name: String,
    data: GameObjectData,
    phase_tag: String,
}

impl Placement {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeletePlacement::build(DeletePlacementVariables {
                    id: self.id
                })).await?;

            if let Some(DeletePlacement { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: placement_graphql::Placement) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id,
            zone_guid: other.zone_guid,
            class: other.class,
            content_guid: other.content_guid,
            editor_name: other.editor_name,
            data: other.data.0.try_into()?,
            phase_tag: other.phase_tag,
        })
    }

    fn into_graphql<'a>(&'a self) -> PlacementInput<'a> {
        PlacementInput {
            id: self.id.into(),
            zone_guid: self.zone_guid.into(),
            class: self.class,
            content_guid: self.content_guid.into(),
            editor_name: &self.editor_name,
            data: schema::Json(serde_json::to_value(&self.data).unwrap()),
            phase_tag: &self.phase_tag,
        }
    }
}

impl RealmApi {
    pub async fn get_placement(&self, id: Uuid) -> RealmApiResult<Option<Placement>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetPlacement::build(GetPlacementVariables {
                id
            })).await?;

        if let Some(GetPlacement { placement }) = response.data {
            if let Some(placement) = placement {
                Ok(Some(Placement::from_graphql(self, placement)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_placementss(&self) -> RealmApiResult<Cursor<Placement>> {
        todo!()
    }

    pub async fn create_placement(&self, placement: Placement) -> RealmApiResult<Placement> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreatePlacement::build(CreatePlacementVariables {
                input: placement.into_graphql()
            })).await?;

        if let Some(CreatePlacement { create_placement }) = response.data {
            Ok(Placement::from_graphql(self, create_placement)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_placements(&self, placements: Vec<Placement>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreatePlacements::build(BatchCreatePlacementsVariables {
                input: placements.iter()
                    .map(|placement| placement.into_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreatePlacements { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod placement_graphql {
    use obj_params::Class;
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetPlacementsVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreatePlacementsVariables<'a> {
        pub input: Vec<PlacementInput<'a>>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetPlacementVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeletePlacementVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreatePlacementVariables<'a> {
        pub input: PlacementInput<'a>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetPlacementsVariables")]
    pub struct GetPlacements {
        #[arguments(after: $after, first: $first)]
        pub placements: PlacementConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetPlacementVariables")]
    pub struct GetPlacement {
        #[arguments(id: $id)]
        pub placement: Option<Placement>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PlacementConnection {
        pub nodes: Vec<Placement>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeletePlacementVariables")]
    pub struct DeletePlacement {
        #[arguments(id: $id)]
        pub delete_placement: Option<Placement>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreatePlacementVariables")]
    pub struct CreatePlacement {
        #[arguments(input: $input)]
        pub create_placement: Placement,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Placement {
        pub class: Class,
        pub content_guid: Uuid,
        pub data: Json,
        pub editor_name: String,
        pub id: Uuid,
        pub phase_tag: String,
        pub zone_guid: Uuid,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreatePlacementsVariables")]
    pub struct BatchCreatePlacements {
        #[arguments(input: $input)]
        pub batch_create_placements: Vec<Placement>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PlacementInput<'a> {
        pub id: Uuid,
        pub zone_guid: Uuid,
        pub class: Class,
        pub content_guid: Uuid,
        pub editor_name: &'a str,
        pub data: Json,
        pub phase_tag: &'a str,
    }    
}