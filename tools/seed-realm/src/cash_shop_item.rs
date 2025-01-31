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

use std::{mem::take, path::Path};

use indicatif::ProgressBar;
use log::info;
use obj_params::{Class, GameObjectData, GenericParamSet, ParamReader};
use realm_api::{CashShopItemBuilder, CashShopItemBundleBuilder, ObjectPlacementBuilder, RealmApi};
use tokio::runtime::Handle;

use crate::{error::SeedRealmResult, MP};

pub async fn import_cash_shop_item(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        info!("Importing cash shop items...");

        let count = db
            .prepare("SELECT COUNT(*) FROM SKUItems")
            .unwrap()
            .into_iter()
            .next().unwrap().unwrap()
            .read::<i64,_>(0);
    
        let result = db
            .prepare("SELECT * FROM SKUItems")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let pg = MP.add(ProgressBar::new(count as u64));

        let mut records = Vec::new();
    
        // dump data
        for row in result {              
            records.push(
                CashShopItemBuilder::default()
                    .id(row.read::<&str,_>("SKUID").parse().unwrap())
                    .display_name(row.read::<&str,_>("DisplayName").to_owned())
                    .description(row.read::<&str,_>("Description").to_owned())
                    .reference_item_name(row.read::<&str,_>("ReferenceItemName").to_owned())
                    .reference_item_guid(row.read::<&str,_>("ReferenceItemGUID").parse().unwrap())
                    .cash_price(row.read::<&str,_>("CashPrice").parse().unwrap())
                    .sku_code(row.read::<&str,_>("SKUCode").to_owned())
                    .rental_duration(row.read::<&str,_>("RentalDuration").parse().unwrap())
                    .is_in_stock(row.read::<&str,_>("IsInStock").parse::<i32>().unwrap() != 0)
                    .is_hot(row.read::<&str,_>("IsHot").parse::<i32>().unwrap() != 0)
                    .is_new(row.read::<&str,_>("IsNew").parse::<i32>().unwrap() != 0)
                    .version(row.read::<&str,_>("Version").parse().unwrap())
                    .is_visible(row.read::<&str,_>("IsVisible").parse::<i32>().unwrap() != 0)
                    .is_tradable(row.read::<&str,_>("IsTradable").parse::<i32>().unwrap() != 0)
                    .is_featured(row.read::<&str,_>("IsFeatured").parse::<i32>().unwrap() != 0)
                    .quantity(row.read::<&str,_>("Quantity").parse().unwrap())
                    .discount(row.read::<&str,_>("Discount").parse().unwrap())
                    .date_start(row.read::<&str,_>("DateStart").parse().ok())
                    .date_end(row.read::<&str,_>("DateEnd").parse().ok())
                    .build().unwrap()
            );

            if records.len() == 100 {
                Handle::current().block_on(async {
                    api.batch_create_cash_shop_items(take(&mut records))
                        .await.unwrap()
                });

                pg.inc(100);
            }
        }

        if !records.is_empty(){
            Handle::current().block_on(async {
                api.batch_create_cash_shop_items(take(&mut records))
                    .await.unwrap()
            });

            pg.finish();
        }
    });

    Ok(())
}