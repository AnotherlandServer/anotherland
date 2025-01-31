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
use realm_api::{CashShopItemBuilder, CashShopItemBundleBuilder, CashShopVendorBuilder, ObjectPlacementBuilder, RealmApi};
use tokio::runtime::Handle;
use toolkit::types::Uuid;

use crate::{error::SeedRealmResult, MP};

pub async fn import_cash_shop_vendors(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        info!("Importing cash shop items...");

        let count = db
            .prepare("SELECT COUNT(*) FROM VendorData")
            .unwrap()
            .into_iter()
            .next().unwrap().unwrap()
            .read::<i64,_>(0);
    
        let result = db
            .prepare("SELECT * FROM VendorData")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let pg = MP.add(ProgressBar::new(count as u64));

        let mut records = Vec::new();
    
        // dump data
        for row in result {              
            records.push(
                CashShopVendorBuilder::default()
                    .id(row.read::<&str,_>("VendorID").parse().unwrap())
                    .vendor_name(row.read::<&str,_>("VendorName").to_owned())
                    .sku_list(
                        row.try_read::<&str,_>("SKUList")
                            .unwrap_or("")
                            .split(",")
                            .filter(|s| !s.is_empty())
                            .map(|item| {
                                item.trim().parse()
                            }).collect::<Result<Vec<Uuid>, _>>().unwrap()
                    )
                    .bundle_list(
                        row.try_read::<&str,_>("BundleList")
                            .unwrap_or("")
                            .split(",")
                            .filter(|s| !s.is_empty())
                            .map(|item| {
                                item.trim().parse()
                            }).collect::<Result<Vec<Uuid>, _>>().unwrap()
                    )
                    .version(row.read::<i64,_>("Version") as i32)
                    .build().unwrap()
            );

            if records.len() == 100 {
                Handle::current().block_on(async {
                    api.batch_create_cash_shop_vendors(take(&mut records))
                        .await.unwrap()
                });

                pg.inc(100);
            }
        }

        if !records.is_empty(){
            Handle::current().block_on(async {
                api.batch_create_cash_shop_vendors(take(&mut records))
                    .await.unwrap()
            });

            pg.finish();
        }
    });

    Ok(())
}