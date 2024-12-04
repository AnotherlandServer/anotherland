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
use object_template_graphql::{BatchCreateObjectTemplates, BatchCreateObjectTemplatesVariables, CreateObjectTemplate, CreateObjectTemplateVariables, DeleteObjectTemplate, DeleteObjectTemplateVariables, GetObjectTemplate, GetObjectTemplateVariables, ObjectTemplateInput};
use toolkit::types::Uuid;

use crate::{schema, RealmApi, RealmApiError, RealmApiResult};

pub use object_template_graphql::Category;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ObjectTemplate {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    id: Uuid,
    category: Category,
    name: String,
    class: Class,
    data: GameObjectData,
}

impl ObjectTemplate {
    pub async fn delete(&self) -> RealmApiResult<()> {
        if let Some(api_base) = &self.api_base {
            let response = api_base.0.client
                .post(api_base.0.base_url.clone())
                .run_graphql(DeleteObjectTemplate::build(DeleteObjectTemplateVariables {
                    id: self.id
                })).await?;

            if let Some(DeleteObjectTemplate { .. }) = response.data {
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

    fn from_graphql(api: &RealmApi, other: object_template_graphql::ObjectTemplate) -> RealmApiResult<Self> {
        Ok(Self {
            api_base: Some(api.clone()),
            id: other.id,
            category: other.category,
            name: other.name,
            class: other.class,
            data: other.data.0.try_into()?,
        })
    }

    fn into_graphql<'a>(&'a self) -> ObjectTemplateInput<'a> {
        ObjectTemplateInput {
            id: self.id,
            category: self.category,
            class: self.class,
            name: &self.name,
            data: schema::Json(serde_json::to_value(&self.data).unwrap()),
        }
    }
}

impl RealmApi {
    pub async fn get_object_template(&self, id: Uuid) -> RealmApiResult<Option<ObjectTemplate>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetObjectTemplate::build(GetObjectTemplateVariables {
                id
            })).await?;

        if let Some(GetObjectTemplate { object_template }) = response.data {
            if let Some(object_template) = object_template {
                Ok(Some(ObjectTemplate::from_graphql(self, object_template)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_object_templates(&self) -> RealmApiResult<Cursor<ObjectTemplate>> {
        todo!()
    }

    pub async fn create_object_template(&self, template: ObjectTemplate) -> RealmApiResult<ObjectTemplate> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateObjectTemplate::build(CreateObjectTemplateVariables {
                input: template.into_graphql()
            })).await?;

        if let Some(CreateObjectTemplate { create_object_template }) = response.data {
            Ok(ObjectTemplate::from_graphql(self, create_object_template)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_object_templates(&self, templates: Vec<ObjectTemplate>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateObjectTemplates::build(BatchCreateObjectTemplatesVariables {
                input: templates.iter()
                    .map(|template| template.into_graphql())
                    .collect()
            })).await?;

        if let Some(BatchCreateObjectTemplates { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod object_template_graphql {
    use obj_params::Class;
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetObjectTemplatesVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateObjectTemplatesVariables<'a> {
        pub input: Vec<ObjectTemplateInput<'a>>,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetObjectTemplateVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteObjectTemplateVariables {
        pub id: Uuid,
    }
    
    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateObjectTemplateVariables<'a> {
        pub input: ObjectTemplateInput<'a>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetObjectTemplatesVariables")]
    pub struct GetObjectTemplates {
        #[arguments(after: $after, first: $first)]
        pub object_templates: ObjectTemplateConnection,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetObjectTemplateVariables")]
    pub struct GetObjectTemplate {
        #[arguments(id: $id)]
        pub object_template: Option<ObjectTemplate>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectTemplateConnection {
        pub nodes: Vec<ObjectTemplate>,
        pub page_info: PageInfo,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
        pub has_next_page: bool,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteObjectTemplateVariables")]
    pub struct DeleteObjectTemplate {
        #[arguments(id: $id)]
        pub delete_object_template: Option<ObjectTemplate>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateObjectTemplateVariables")]
    pub struct CreateObjectTemplate {
        #[arguments(input: $input)]
        pub create_object_template: ObjectTemplate,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectTemplate {
        pub id: Uuid,
        pub category: Category,
        pub name: String,
        pub class: Class,
        pub data: Json,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateObjectTemplatesVariables")]
    pub struct BatchCreateObjectTemplates {
        #[arguments(input: $input)]
        pub batch_create_object_templates: Vec<ObjectTemplate>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectTemplateInput<'a> {
        pub id: Uuid,
        pub category: Category,
        pub name: &'a str,
        pub class: Class,
        pub data: Json,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum Category {
        NoBinding,
        Buffs,
        Drops,
        Enemies,
        Factions,
        Items,
        Metagame,
        Misc,
        Npcs,
        Projectiles,
        Quests,
        Recipes,
        Skills,
        Spawners,
        Structures,
    }
}