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
use toolkit::types::Uuid;
use zone_graphql::{BatchCreateZones, BatchCreateZonesVariables, CreateZone, CreateZoneVariables, DeleteZone, DeleteZoneVariables, GetZone, GetZoneVariables, ZoneInput};

use crate::{schema, RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
pub struct Zone {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    id: i64,
    guid: Uuid,
    worlddef_guid: Uuid,
    parent_zone_guid: Uuid,
    zone: String,
    zone_type: i32,
    is_instance: bool,
    server: String,
    level: String,
    layer: String,
    realu_zone_type: String,
    game_controller: String,
}

impl Zone {
    pub fn id(&self) -> &i64 { &self.id }
    pub fn guid(&self) -> &Uuid { &self.guid }
    pub fn worlddef_guid(&self) -> &Uuid { &self.worlddef_guid }
    pub fn parent_zone_guid(&self) -> &Uuid { &self.parent_zone_guid }
    pub fn zone(&self) -> &str { &self.zone }
    pub fn zone_type(&self) -> &i32 { &self.zone_type }
    pub fn is_instance(&self) -> &bool { &self.is_instance }
    pub fn server(&self) -> &str { &self.server }
    pub fn level(&self) -> &str { &self.level }
    pub fn layer(&self) -> &str { &self.layer }
    pub fn realu_zone_type(&self) -> &str { &self.realu_zone_type }
    pub fn game_controller(&self) -> &str { &self.game_controller }

    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteZone::build(DeleteZoneVariables {
                    id: self.id as i32
                })).await?;

            if let Some(DeleteZone { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: zone_graphql::Zone) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id as i64,
            guid: other.guid.0.parse()?,
            worlddef_guid: other.worlddef_guid.0.parse()?,
            parent_zone_guid: other.parent_zone_guid.0.parse()?,
            zone: other.zone,
            zone_type: other.zone_type,
            is_instance: other.is_instance,
            server: other.server,
            level: other.level,
            layer: other.layer,
            realu_zone_type: other.realu_zone_type,
            game_controller: other.game_controller,
        })
    }

    fn into_graphql<'a>(&'a self) -> ZoneInput<'a> {
        ZoneInput {
            id: self.id as i32,
            guid: schema::Uuid(self.guid.to_string()),
            worlddef_guid: schema::Uuid(self.worlddef_guid.to_string()),
            parent_zone_guid: schema::Uuid(self.parent_zone_guid.to_string()),
            zone: &self.zone,
            zone_type: self.zone_type,
            is_instance: self.is_instance,
            server: &self.server,
            level: &self.level,
            layer: &self.layer,
            realu_zone_type: &self.realu_zone_type,
            game_controller: &self.game_controller,
        }
    }
}

impl RealmApi {
    pub async fn get_zone(&self, id: i64) -> RealmApiResult<Option<Zone>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetZone::build(GetZoneVariables {
                id: id as i32
            })).await?;

        if let Some(GetZone { zone }) = response.data {
            if let Some(zone) = zone {
                Ok(Some(Zone::from_graphql(self, zone)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_zones(&self) -> RealmApiResult<Cursor<Zone>> {
        todo!()
    }

    pub async fn create_zone(&self, zone: Zone) -> RealmApiResult<Zone> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateZone::build(CreateZoneVariables {
                input: zone.into_graphql()
            })).await?;

        if let Some(CreateZone { create_zone }) = response.data {
            Ok(Zone::from_graphql(self, create_zone)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_zones(&self, zones: Vec<Zone>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateZones::build(BatchCreateZonesVariables {
                input: zones.iter()
                    .map(|zone| zone.into_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateZones { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod zone_graphql {
    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateZoneVariables<'a> {
        pub input: ZoneInput<'a>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetZonesVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteZoneVariables {
        pub id: i32,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateZonesVariables<'a> {
        pub input: Vec<ZoneInput<'a>>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetZoneVariables {
        pub id: i32,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetZonesVariables")]
    pub struct GetZones {
        #[arguments(after: $after, first: $first)]
        pub zones: ZoneConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ZoneConnection {
        pub nodes: Vec<Zone>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetZoneVariables")]
    pub struct GetZone {
        #[arguments(id: $id)]
        pub zone: Option<Zone>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteZoneVariables")]
    pub struct DeleteZone {
        #[arguments(id: $id)]
        pub delete_zone: Option<Zone>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateZoneVariables")]
    pub struct CreateZone {
        #[arguments(input: $input)]
        pub create_zone: Zone,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateZonesVariables")]
    pub struct BatchCreateZones {
        #[arguments(input: $input)]
        pub batch_create_zones: Vec<Zone>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Zone {
        pub game_controller: String,
        pub guid: Uuid,
        pub id: i32,
        pub is_instance: bool,
        pub layer: String,
        pub level: String,
        pub parent_zone_guid: Uuid,
        pub realu_zone_type: String,
        pub server: String,
        pub worlddef_guid: Uuid,
        pub zone: String,
        pub zone_type: i32,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ZoneInput<'a> {
        pub id: i32,
        pub guid: Uuid,
        pub worlddef_guid: Uuid,
        pub parent_zone_guid: Uuid,
        pub zone: &'a str,
        pub zone_type: i32,
        pub is_instance: bool,
        pub server: &'a str,
        pub level: &'a str,
        pub layer: &'a str,
        pub realu_zone_type: &'a str,
        pub game_controller: &'a str,
    }
}