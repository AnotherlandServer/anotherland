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

use cash_shop_vendor_graphql::{BatchCreateCashShopVendors, BatchCreateCashShopVendorsVariables, CashShopVendorInput, CreateCashShopVendor, CreateCashShopVendorVariables, GetCashShopVendor, GetCashShopVendorVariables, GetCashShopVendors, GetCashShopVendorsVariables};
use cynic::{http::ReqwestExt, QueryBuilder, MutationBuilder};
use derive_builder::Builder;
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct CashShopVendorQuery {
    #[builder(private)]
    api_base: RealmApi,
}

impl RecordQuery for CashShopVendorQuery {
    type Record = CashShopVendor;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetCashShopVendors::build(GetCashShopVendorsVariables {
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetCashShopVendors { cash_shop_vendors }) = response.data {
            Ok(RecordPage {
                at_end: !cash_shop_vendors.page_info.has_next_page,
                last_cursor: cash_shop_vendors.page_info.end_cursor,
                records: cash_shop_vendors.nodes.into_iter()
                    .map(|zone| zone.try_into())
                    .collect::<Result<Vec<_>, Self::Error>>()?,
            })
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CashShopVendor {
    pub id: Uuid,
    pub vendor_name: String,
    pub sku_list: Vec<Uuid>,
    pub bundle_list: Vec<Uuid>,
    pub version: i32,
}

impl TryFrom<cash_shop_vendor_graphql::CashShopVendor> for CashShopVendor {
    type Error = RealmApiError;

    fn try_from(value: cash_shop_vendor_graphql::CashShopVendor) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            vendor_name: value.vendor_name,
            sku_list: value.sku_list,
            bundle_list: value.bundle_list,
            version: value.version,
        })
    }
}

impl <'a> TryFrom<&'a CashShopVendor> for cash_shop_vendor_graphql::CashShopVendorInput<'a> {
    type Error = RealmApiError;

    fn try_from(value: &'a CashShopVendor) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            vendor_name: &value.vendor_name,
            sku_list: value.sku_list.clone(),
            bundle_list: value.bundle_list.clone(),
            version: value.version,
        })
    }
}

impl CashShopVendorQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<CashShopVendorQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

impl RealmApi {
    pub async fn get_cash_shop_vendor(&self, id: Uuid) -> RealmApiResult<Option<CashShopVendor>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetCashShopVendor::build(GetCashShopVendorVariables {
                id
            })).await?;

        if let Some(GetCashShopVendor { cash_shop_vendor }) = response.data {
            if let Some(cash_shop_vendor) = cash_shop_vendor {
                Ok(Some(cash_shop_vendor.try_into()?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_cash_shop_vendors(&self) -> CashShopVendorQueryBuilder {
        CashShopVendorQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_cash_shop_vendor(&self, item: CashShopVendor) -> RealmApiResult<CashShopVendor> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateCashShopVendor::build(CreateCashShopVendorVariables {
                input: (&item).try_into()?
            })).await?;

        if let Some(CreateCashShopVendor { create_cash_shop_vendor }) = response.data {
            create_cash_shop_vendor.try_into()
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_cash_shop_vendors(&self, items: Vec<CashShopVendor>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateCashShopVendors::build(BatchCreateCashShopVendorsVariables {
                input: items.iter()
                    .map(|item| <CashShopVendorInput<'_>>::try_from(item))
                    .collect::<RealmApiResult<Vec<_>>>()?
            })).await?;

        if let Some(BatchCreateCashShopVendors { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod cash_shop_vendor_graphql {
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateCashShopVendorsVariables<'a> {
        pub input: Vec<CashShopVendorInput<'a>>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopVendorVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopVendorsVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteCashShopVendorVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateCashShopVendorVariables<'a> {
        pub input: CashShopVendorInput<'a>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopVendorsVariables")]
    pub struct GetCashShopVendors {
        #[arguments(after: $after, first: $first)]
        pub cash_shop_vendors: CashShopVendorConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopVendorVariables")]
    pub struct GetCashShopVendor {
        #[arguments(id: $id)]
        pub cash_shop_vendor: Option<CashShopVendor>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteCashShopVendorVariables")]
    pub struct DeleteCashShopVendor {
        #[arguments(id: $id)]
        pub delete_cash_shop_vendor: Option<CashShopVendor>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateCashShopVendorVariables")]
    pub struct CreateCashShopVendor {
        #[arguments(input: $input)]
        pub create_cash_shop_vendor: CashShopVendor,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateCashShopVendorsVariables")]
    pub struct BatchCreateCashShopVendors {
        #[arguments(input: $input)]
        pub batch_create_cash_shop_vendors: Vec<CashShopVendor>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopVendorConnection {
        pub nodes: Vec<CashShopVendor>,
        pub page_info: PageInfo,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PageInfo {
        pub has_next_page: bool,
        pub end_cursor: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopVendor {
        pub id: Uuid,
        pub vendor_name: String,
        pub sku_list: Vec<Uuid>,
        pub bundle_list: Vec<Uuid>,
        pub version: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopVendorInput<'a> {
        pub id: Uuid,
        pub vendor_name: &'a str,
        pub sku_list: Vec<Uuid>,
        pub bundle_list: Vec<Uuid>,
        pub version: i32,
    }
}