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
use object_placement_graphql::{BatchCreateObjectPlacements, BatchCreateObjectPlacementsVariables, CreateObjectPlacement, CreateObjectPlacementVariables, DeleteObjectPlacement, DeleteObjectPlacementVariables, GetObjectPlacement, GetObjectPlacementVariables, ObjectPlacementInput};
use toolkit::types::Uuid;

use crate::{schema, RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ObjectPlacement {
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

impl ObjectPlacement {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteObjectPlacement::build(DeleteObjectPlacementVariables {
                    id: self.id
                })).await?;

            if let Some(DeleteObjectPlacement { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: object_placement_graphql::ObjectPlacement) -> RealmApiResult<Self> {
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

    fn into_graphql<'a>(&'a self) -> ObjectPlacementInput<'a> {
        ObjectPlacementInput {
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
    pub async fn get_object_placement(&self, id: Uuid) -> RealmApiResult<Option<ObjectPlacement>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetObjectPlacement::build(GetObjectPlacementVariables {
                id
            })).await?;

        if let Some(GetObjectPlacement { object_placement }) = response.data {
            if let Some(object_placement) = object_placement {
                Ok(Some(ObjectPlacement::from_graphql(self, object_placement)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_object_placements(&self) -> RealmApiResult<Cursor<ObjectPlacement>> {
        todo!()
    }

    pub async fn create_object_placement(&self, placement: ObjectPlacement) -> RealmApiResult<ObjectPlacement> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateObjectPlacement::build(CreateObjectPlacementVariables {
                input: placement.into_graphql()
            })).await?;

        if let Some(CreateObjectPlacement { create_object_placement }) = response.data {
            Ok(ObjectPlacement::from_graphql(self, create_object_placement)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_object_placements(&self, placements: Vec<ObjectPlacement>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateObjectPlacements::build(BatchCreateObjectPlacementsVariables {
                input: placements.iter()
                    .map(|placement| placement.into_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateObjectPlacements { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod object_placement_graphql {
    use obj_params::Class;
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetObjectPlacementsVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateObjectPlacementsVariables<'a> {
        pub input: Vec<ObjectPlacementInput<'a>>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetObjectPlacementVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteObjectPlacementVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateObjectPlacementVariables<'a> {
        pub input: ObjectPlacementInput<'a>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetObjectPlacementsVariables")]
    pub struct GetObjectPlacements {
        #[arguments(after: $after, first: $first)]
        pub object_placements: ObjectPlacementConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetObjectPlacementVariables")]
    pub struct GetObjectPlacement {
        #[arguments(id: $id)]
        pub object_placement: Option<ObjectPlacement>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectPlacementConnection {
        pub nodes: Vec<ObjectPlacement>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteObjectPlacementVariables")]
    pub struct DeleteObjectPlacement {
        #[arguments(id: $id)]
        pub delete_object_placement: Option<ObjectPlacement>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateObjectPlacementVariables")]
    pub struct CreateObjectPlacement {
        #[arguments(input: $input)]
        pub create_object_placement: ObjectPlacement,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectPlacement {
        pub class: Class,
        pub content_guid: Uuid,
        pub data: Json,
        pub editor_name: String,
        pub id: Uuid,
        pub phase_tag: String,
        pub zone_guid: Uuid,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateObjectPlacementsVariables")]
    pub struct BatchCreateObjectPlacements {
        #[arguments(input: $input)]
        pub batch_create_object_placements: Vec<ObjectPlacement>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectPlacementInput<'a> {
        pub id: Uuid,
        pub zone_guid: Uuid,
        pub class: Class,
        pub content_guid: Uuid,
        pub editor_name: &'a str,
        pub data: Json,
        pub phase_tag: &'a str,
    }    
}