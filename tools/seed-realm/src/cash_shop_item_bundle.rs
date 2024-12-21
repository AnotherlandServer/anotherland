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

use std::{mem::take, path::Path};

use indicatif::ProgressBar;
use log::info;
use obj_params::{Class, GameObjectData, GenericParamSet, ParamReader};
use realm_api::{CashShopItemBundleBuilder, ObjectPlacementBuilder, RealmApi};
use tokio::runtime::Handle;

use crate::{error::SeedRealmResult, MP};

pub async fn import_cash_shop_item_bundles(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        info!("Importing cash shop item bundles...");

        let count = db
            .prepare("SELECT COUNT(*) FROM BundleItems")
            .unwrap()
            .into_iter()
            .next().unwrap().unwrap()
            .read::<i64,_>(0);
    
        let result = db
            .prepare("SELECT * FROM BundleItems")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let pg = MP.add(ProgressBar::new(count as u64));

        let mut records = Vec::new();
    
        // dump data
        for row in result {              
            records.push(
                CashShopItemBundleBuilder::default()
                    .id(row.read::<&str,_>("BundleID").parse().unwrap())
                    .display_name(row.read::<&str,_>("DisplayName").to_owned())
                    .description(row.read::<&str,_>("Description").to_owned())
                    .cash_price(row.read::<i64,_>("CashPrice") as i32)
                    .icon(row.read::<&str,_>("Icon").to_owned())
                    .item_list_and_count(row.read::<&str,_>("ItemListAndCount").to_owned())
                    .is_in_stock(row.read::<i64,_>("IsInStock") != 0)
                    .is_hot(row.read::<i64,_>("IsHot") != 0)
                    .is_new(row.read::<i64,_>("IsNew") != 0)
                    .version(row.read::<i64,_>("Version") as i32)
                    .is_visible(row.read::<i64,_>("IsVisible") != 0)
                    .is_tradable(row.read::<i64,_>("IsTradable") != 0)
                    .is_featured(row.read::<i64,_>("IsFeatured") != 0)
                    .quantity(row.read::<i64,_>("Quantity") as i32)
                    .discount(row.read::<i64,_>("Discount") as i32)
                    .date_start(row.read::<&str,_>("DateStart").parse().ok())
                    .date_end(row.read::<&str,_>("DateEnd").parse().ok())
                    .build().unwrap()
            );

            if records.len() == 100 {
                Handle::current().block_on(async {
                    api.batch_create_cash_shop_item_bundles(take(&mut records))
                        .await.unwrap()
                });

                pg.inc(100);
            }
        }

        if !records.is_empty(){
            Handle::current().block_on(async {
                api.batch_create_cash_shop_item_bundles(take(&mut records))
                    .await.unwrap()
            });

            pg.finish();
        }
    });

    Ok(())
}