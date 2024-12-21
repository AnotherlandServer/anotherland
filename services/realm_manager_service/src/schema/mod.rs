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

use async_graphql::{scalar, MergedObject};
use character_ext::{CharacterExtMutationRoot, CharacterExtRoot};
use instances::{InstancesMutationRoot, InstancesRoot};
use nodes::NodesRoot;
use premium_currency::{PremiumCurrencyMutationRoot, PremiumCurrencyRoot};
use session_state::{SessionStateMutationRoot, SessionStateRoot};

use crate::db;

mod character_ext;
mod premium_currency;
mod nodes;
mod types;
mod instances;
mod session_state;

pub use types::*;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    pub CharacterExtRoot,
    pub PremiumCurrencyRoot,
    pub NodesRoot,
    pub InstancesRoot,
    pub SessionStateRoot,
    pub db::WorldDefQueryRoot,
    pub db::ZoneQueryRoot,
    pub db::ObjectPlacementQueryRoot,
    pub db::ObjectTemplateQueryRoot,
    pub db::CharacterQueryRoot,
    pub db::CashShopItemBundleQueryRoot,
    pub db::CashShopItemQueryRoot,
    pub db::CashShopVendorQueryRoot,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    pub CharacterExtMutationRoot,
    pub PremiumCurrencyMutationRoot,
    pub InstancesMutationRoot,
    pub SessionStateMutationRoot,
    pub db::WorldDefMutationRoot,
    pub db::ZoneMutationRoot,
    pub db::ObjectPlacementMutationRoot,
    pub db::ObjectTemplateMutationRoot,
    pub db::CharacterMutationRoot,
    pub db::CashShopItemBundleMutationRoot,
    pub db::CashShopItemMutationRoot,
    pub db::CashShopVendorMutationRoot,
);