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
use obj_params::{AsGameObjectDataRef, Class, GameObjectData};
use object_template_graphql::{BatchCreateObjectTemplates, BatchCreateObjectTemplatesVariables, CreateObjectTemplate, CreateObjectTemplateVariables, DeleteObjectTemplate, DeleteObjectTemplateVariables, GetObjectTemplate, GetObjectTemplateVariables, GetObjectTemplates, GetObjectTemplatesVariables, ObjectTemplateFilter, ObjectTemplateInput};
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{schema, RealmApi, RealmApiError, RealmApiResult};

pub use object_template_graphql::Category;

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct ObjectTemplateQuery {
    #[builder(private)]
    api_base: RealmApi,

    #[builder(setter(strip_option), default)]
    numeric_id: Option<i32>,

    #[builder(setter(strip_option), default)]
    name: Option<String>,

    #[builder(setter(strip_option), default)]
    class: Option<Class>,

    #[builder(setter(strip_option), default)]
    category: Option<Category>,
}

impl ObjectTemplateQuery {
    fn get_filter(&self) -> Option<ObjectTemplateFilter<'_>> {
        if self.name.is_none() && self.class.is_none() && self.category.is_none() {
            None
        } else {
            Some(ObjectTemplateFilter { 
                numeric_id: self.numeric_id,
                category: self.category, 
                name: self.name.as_deref(), 
                class: self.class, 
            })
        }
    }
}

impl RecordQuery for ObjectTemplateQuery {
    type Record = ObjectTemplate;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetObjectTemplates::build(GetObjectTemplatesVariables {
                filter: self.get_filter(),
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetObjectTemplates { object_templates }) = response.data {
            Ok(RecordPage {
                at_end: !object_templates.page_info.has_next_page,
                last_cursor: object_templates.page_info.end_cursor,
                records: object_templates.nodes.into_iter()
                    .map(|zone| ObjectTemplate::from_graphql(&self.api_base, zone))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

impl ObjectTemplateQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<ObjectTemplateQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ObjectTemplate {
    #[builder(setter(skip))]
    api_base: Option<RealmApi>,

    pub id: Uuid,
    pub numeric_id: i32,
    pub category: Category,
    pub name: String,
    pub class: Class,
    pub data: GameObjectData,
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
            numeric_id: other.numeric_id,
            category: other.category,
            name: other.name,
            class: other.class,
            data: other.data.0.try_into()?,
        })
    }

    fn as_graphql<'a>(&'a self) -> ObjectTemplateInput<'a> {
        ObjectTemplateInput {
            id: self.id,
            numeric_id: self.numeric_id,
            category: self.category,
            class: self.class,
            name: &self.name,
            data: schema::Json(serde_json::to_value(&self.data).unwrap()),
        }
    }
}

impl AsGameObjectDataRef for ObjectTemplate {
    fn as_data_ref(&self) -> &GameObjectData {
        &self.data
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

    pub fn query_object_templates(&self) -> ObjectTemplateQueryBuilder {
        ObjectTemplateQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_object_template(&self, template: ObjectTemplate) -> RealmApiResult<ObjectTemplate> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateObjectTemplate::build(CreateObjectTemplateVariables {
                input: template.as_graphql()
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
                    .map(|template| template.as_graphql())
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
        pub filter: Option<ObjectTemplateFilter<'a>>,
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
        #[arguments(filter: $filter, after: $after, first: $first)]
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
        #[allow(dead_code)]
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
        pub numeric_id: i32,
        pub category: Category,
        pub name: String,
        pub class: Class,
        pub data: Json,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateObjectTemplatesVariables")]
    pub struct BatchCreateObjectTemplates {
        #[arguments(input: $input)]
        #[allow(dead_code)]
        pub batch_create_object_templates: Vec<ObjectTemplate>,
    }
    
    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectTemplateInput<'a> {
        pub id: Uuid,
        pub numeric_id: i32,
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

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct ObjectTemplateFilter<'a> {
        pub numeric_id: Option<i32>,
        pub category: Option<Category>,
        pub name: Option<&'a str>,
        pub class: Option<Class>,
    }
}