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

use cash_shop_item_bundle_graphql::{BatchCreateCashShopItemBundles, BatchCreateCashShopItemBundlesVariables, CashShopItemBundleInput, CreateCashShopItemBundle, CreateCashShopItemBundleVariables, GetCashShopItemBundle, GetCashShopItemBundleVariables, GetCashShopItemBundles, GetCashShopItemBundlesVariables};
use chrono::NaiveDate;
use cynic::{http::ReqwestExt, QueryBuilder, MutationBuilder};
use derive_builder::Builder;
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct CashShopItemBundleQuery {
    #[builder(private)]
    api_base: RealmApi,
}

impl RecordQuery for CashShopItemBundleQuery {
    type Record = CashShopItemBundle;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetCashShopItemBundles::build(GetCashShopItemBundlesVariables {
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetCashShopItemBundles { cash_shop_item_bundles }) = response.data {
            Ok(RecordPage {
                at_end: !cash_shop_item_bundles.page_info.has_next_page,
                last_cursor: cash_shop_item_bundles.page_info.end_cursor,
                records: cash_shop_item_bundles.nodes.into_iter()
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

impl CashShopItemBundleQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<CashShopItemBundleQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CashShopItemBundle {
    pub id: Uuid,
    pub display_name: String,
    pub description: String,
    pub cash_price: i32,
    pub icon: String,
    pub item_list_and_count: String,
    pub is_in_stock: bool,
    pub is_hot: bool,
    pub is_new: bool,
    pub version: i32,
    pub is_visible: bool,
    pub is_tradable: bool,
    pub is_featured: bool,
    pub quantity: i32,
    pub discount: i32,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>
}

impl TryFrom<cash_shop_item_bundle_graphql::CashShopItemBundle> for CashShopItemBundle {
    type Error = RealmApiError;

    fn try_from(value: cash_shop_item_bundle_graphql::CashShopItemBundle) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: value.display_name,
            description: value.description,
            cash_price: value.cash_price,
            icon: value.icon,
            item_list_and_count: value.item_list_and_count,
            is_in_stock: value.is_in_stock,
            is_hot: value.is_hot,
            is_new: value.is_new,
            version: value.version,
            is_visible: value.is_visible,
            is_tradable: value.is_tradable,
            is_featured: value.is_featured,
            quantity: value.quantity,
            discount: value.discount,
            date_start: value.date_start,
            date_end: value.date_end,
        })
    }
}

impl <'a> TryFrom<&'a CashShopItemBundle> for cash_shop_item_bundle_graphql::CashShopItemBundleInput<'a> {
    type Error = RealmApiError;

    fn try_from(value: &'a CashShopItemBundle) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: &value.display_name,
            description: &value.description,
            cash_price: value.cash_price,
            icon: &value.icon,
            item_list_and_count: &value.item_list_and_count,
            is_in_stock: value.is_in_stock,
            is_hot: value.is_hot,
            is_new: value.is_new,
            version: value.version,
            is_visible: value.is_visible,
            is_tradable: value.is_tradable,
            is_featured: value.is_featured,
            quantity: value.quantity,
            discount: value.discount,
            date_start: value.date_start,
            date_end: value.date_end,
        })
    }
}

impl RealmApi {
    pub async fn get_cash_shop_item_bundle(&self, id: Uuid) -> RealmApiResult<Option<CashShopItemBundle>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetCashShopItemBundle::build(GetCashShopItemBundleVariables {
                id
            })).await?;

        if let Some(GetCashShopItemBundle { cash_shop_item_bundle }) = response.data {
            if let Some(cash_shop_item_bundle) = cash_shop_item_bundle {
                Ok(Some(cash_shop_item_bundle.try_into()?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_cash_shop_item_bundles(&self) -> CashShopItemBundleQueryBuilder {
        CashShopItemBundleQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_cash_shop_item_bundle(&self, item_bundle: CashShopItemBundle) -> RealmApiResult<CashShopItemBundle> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateCashShopItemBundle::build(CreateCashShopItemBundleVariables {
                input: (&item_bundle).try_into()?
            })).await?;

        if let Some(CreateCashShopItemBundle { create_cash_shop_item_bundle }) = response.data {
            create_cash_shop_item_bundle.try_into()
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_cash_shop_item_bundles(&self, item_bundles: Vec<CashShopItemBundle>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateCashShopItemBundles::build(BatchCreateCashShopItemBundlesVariables {
                input: item_bundles.iter()
                    .map(|item_bundle| <CashShopItemBundleInput<'_>>::try_from(item_bundle))
                    .collect::<RealmApiResult<Vec<_>>>()?
            })).await?;

        if let Some(BatchCreateCashShopItemBundles { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod cash_shop_item_bundle_graphql {
    use chrono::NaiveDate;
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateCashShopItemBundlesVariables<'a> {
        pub input: Vec<CashShopItemBundleInput<'a>>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopItemBundleVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopItemBundlesVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteCashShopItemBundleVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateCashShopItemBundleVariables<'a> {
        pub input: CashShopItemBundleInput<'a>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopItemBundlesVariables")]
    pub struct GetCashShopItemBundles {
        #[arguments(after: $after, first: $first)]
        pub cash_shop_item_bundles: CashShopItemBundleConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopItemBundleVariables")]
    pub struct GetCashShopItemBundle {
        #[arguments(id: $id)]
        pub cash_shop_item_bundle: Option<CashShopItemBundle>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteCashShopItemBundleVariables")]
    pub struct DeleteCashShopItemBundle {
        #[arguments(id: $id)]
        pub delete_cash_shop_item_bundle: Option<CashShopItemBundle>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateCashShopItemBundleVariables")]
    pub struct CreateCashShopItemBundle {
        #[arguments(input: $input)]
        pub create_cash_shop_item_bundle: CashShopItemBundle,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateCashShopItemBundlesVariables")]
    pub struct BatchCreateCashShopItemBundles {
        #[arguments(input: $input)]
        pub batch_create_cash_shop_item_bundles: Vec<CashShopItemBundle>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopItemBundleConnection {
        pub nodes: Vec<CashShopItemBundle>,
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
    pub struct CashShopItemBundle {
        pub cash_price: i32,
        pub date_end: Option<NaiveDate>,
        pub date_start: Option<NaiveDate>,
        pub description: String,
        pub discount: i32,
        pub display_name: String,
        pub icon: String,
        pub is_featured: bool,
        pub id: Uuid,
        pub is_hot: bool,
        pub is_in_stock: bool,
        pub is_new: bool,
        pub is_tradable: bool,
        pub is_visible: bool,
        pub item_list_and_count: String,
        pub quantity: i32,
        pub version: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopItemBundleInput<'a> {
        pub id: Uuid,
        pub display_name: &'a str,
        pub description: &'a str,
        pub cash_price: i32,
        pub icon: &'a str,
        pub item_list_and_count: &'a str,
        pub is_in_stock: bool,
        pub is_hot: bool,
        pub is_new: bool,
        pub version: i32,
        pub is_visible: bool,
        pub is_tradable: bool,
        pub is_featured: bool,
        pub quantity: i32,
        pub discount: i32,
        pub date_start: Option<NaiveDate>,
        pub date_end: Option<NaiveDate>,
    }
}