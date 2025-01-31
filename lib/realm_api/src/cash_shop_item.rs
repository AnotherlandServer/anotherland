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

use cash_shop_item_graphql::{BatchCreateCashShopItems, BatchCreateCashShopItemsVariables, CashShopItemInput, CreateCashShopItem, CreateCashShopItemVariables, GetCashShopItem, GetCashShopItemVariables, GetCashShopItems, GetCashShopItemsVariables};
use chrono::NaiveDate;
use cynic::{http::ReqwestExt, QueryBuilder, MutationBuilder};
use derive_builder::Builder;
use toolkit::{record_pagination::{RecordCursor, RecordPage, RecordQuery}, types::Uuid};

use crate::{RealmApi, RealmApiError, RealmApiResult};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private))]
pub struct CashShopItemQuery {
    #[builder(private)]
    api_base: RealmApi,
}

impl RecordQuery for CashShopItemQuery {
    type Record = CashShopItem;
    type Error = RealmApiError;

    async fn query_next(&mut self, after: Option<String>, limit: usize) -> Result<RecordPage<Self::Record>, Self::Error> {
        let response = self.api_base.0.client
            .post(self.api_base.0.base_url.clone())
            .run_graphql(GetCashShopItems::build(GetCashShopItemsVariables {
                after: after.as_deref(),
                first: Some(limit as i32)
            })).await?;

        if let Some(GetCashShopItems { cash_shop_items }) = response.data {
            Ok(RecordPage {
                at_end: !cash_shop_items.page_info.has_next_page,
                last_cursor: cash_shop_items.page_info.end_cursor,
                records: cash_shop_items.nodes.into_iter()
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
pub struct CashShopItem {
    pub id: Uuid,
    pub display_name: String,
    pub description: String,
    pub reference_item_name: String,
    pub reference_item_guid: Uuid,
    pub cash_price: i32,
    pub sku_code: String,
    pub rental_duration: i32,
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

impl TryFrom<cash_shop_item_graphql::CashShopItem> for CashShopItem {
    type Error = RealmApiError;

    fn try_from(value: cash_shop_item_graphql::CashShopItem) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: value.display_name,
            description: value.description,
            reference_item_name: value.reference_item_name,
            reference_item_guid: value.reference_item_guid,
            cash_price: value.cash_price,
            sku_code: value.sku_code,
            rental_duration: value.rental_duration,
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

impl <'a> TryFrom<&'a CashShopItem> for cash_shop_item_graphql::CashShopItemInput<'a> {
    type Error = RealmApiError;

    fn try_from(value: &'a CashShopItem) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: &value.display_name,
            description: &value.description,
            reference_item_name: &value.reference_item_name,
            reference_item_guid: value.reference_item_guid,
            cash_price: value.cash_price,
            sku_code: &value.sku_code,
            rental_duration: value.rental_duration,
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

impl CashShopItemQueryBuilder {
    pub async fn query(self) -> RealmApiResult<RecordCursor<CashShopItemQuery>> {
        Ok(RecordCursor::new(self.build().unwrap()))
    }
}

impl RealmApi {
    pub async fn get_cash_shop_item(&self, id: Uuid) -> RealmApiResult<Option<CashShopItem>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(GetCashShopItem::build(GetCashShopItemVariables {
                id
            })).await?;

        if let Some(GetCashShopItem { cash_shop_item }) = response.data {
            if let Some(cash_shop_item) = cash_shop_item {
                Ok(Some(cash_shop_item.try_into()?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub fn query_cash_shop_items(&self) -> CashShopItemQueryBuilder {
        CashShopItemQueryBuilder::create_empty()
            .api_base(self.clone())
    }

    pub async fn create_cash_shop_item(&self, item: CashShopItem) -> RealmApiResult<CashShopItem> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(CreateCashShopItem::build(CreateCashShopItemVariables {
                input: (&item).try_into()?
            })).await?;

        if let Some(CreateCashShopItem { create_cash_shop_item }) = response.data {
            create_cash_shop_item.try_into()
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn batch_create_cash_shop_items(&self, items: Vec<CashShopItem>) -> RealmApiResult<()> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(BatchCreateCashShopItems::build(BatchCreateCashShopItemsVariables {
                input: items.iter()
                    .map(|item| <CashShopItemInput<'_>>::try_from(item))
                    .collect::<RealmApiResult<Vec<_>>>()?
            })).await?;

        if let Some(BatchCreateCashShopItems { .. }) = response.data {
            Ok(())
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod cash_shop_item_graphql {
    use chrono::NaiveDate;
    use toolkit::types::Uuid;

    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct BatchCreateCashShopItemsVariables<'a> {
        pub input: Vec<CashShopItemInput<'a>>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopItemVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct GetCashShopItemsVariables<'a> {
        pub after: Option<&'a str>,
        pub first: Option<i32>,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteCashShopItemVariables {
        pub id: Uuid,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateCashShopItemVariables<'a> {
        pub input: CashShopItemInput<'a>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopItemsVariables")]
    pub struct GetCashShopItems {
        #[arguments(after: $after, first: $first)]
        pub cash_shop_items: CashShopItemConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "GetCashShopItemVariables")]
    pub struct GetCashShopItem {
        #[arguments(id: $id)]
        pub cash_shop_item: Option<CashShopItem>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "DeleteCashShopItemVariables")]
    pub struct DeleteCashShopItem {
        #[arguments(id: $id)]
        pub delete_cash_shop_item: Option<CashShopItem>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "CreateCashShopItemVariables")]
    pub struct CreateCashShopItem {
        #[arguments(input: $input)]
        pub create_cash_shop_item: CashShopItem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "BatchCreateCashShopItemsVariables")]
    pub struct BatchCreateCashShopItems {
        #[arguments(input: $input)]
        pub batch_create_cash_shop_items: Vec<CashShopItem>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopItemConnection {
        pub nodes: Vec<CashShopItem>,
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
    pub struct CashShopItem {
        pub id: Uuid,
        pub display_name: String,
        pub description: String,
        pub reference_item_name: String,
        pub reference_item_guid: Uuid,
        pub cash_price: i32,
        pub sku_code: String,
        pub rental_duration: i32,
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

    #[derive(cynic::InputObject, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct CashShopItemInput<'a> {
        pub id: Uuid,
        pub display_name: &'a str,
        pub description: &'a str,
        pub reference_item_name: &'a str,
        pub reference_item_guid: Uuid,
        pub cash_price: i32,
        pub sku_code: &'a str,
        pub rental_duration: i32,
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