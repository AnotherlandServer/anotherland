// Copyright (C) 2023 AnotherlandServer
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

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use log::info;
use mongodb::{IndexModel, options::IndexOptions, bson::doc};
use tokio::runtime::Handle;
use uuid::Uuid;

use crate::{util::AnotherlandResult, db::{Content, Instance, WorldDef, ZoneDef, realm_database, cluster_database, CashShopVendor, CashShopItem, CashShopBundle, RawInstance}};
use atlas::ParamClassContainer;

async fn import_content_table(game_client_path: &PathBuf, src_table: &str, target_table: &str) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/content.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<Content>(target_table)
        });

        info!("Importing {} -> {}...", src_table, target_table);
    
        let result = db
            .prepare(format!("SELECT * FROM {}", src_table))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut documents = Vec::new();
    
        // dump data
        for row in result {
            let bin_data = row.read::<&[u8], _>("data");
            let data = if !bin_data.is_empty() {
                let mut class = ParamClassContainer::read(row.read::<i64,_>("ixClass") as u16, bin_data).unwrap().1;
                class.strip_original_data();

                Some(class)
            } else {
                None
            };
            
            
            documents.push(Content {
                id: row.read::<i64,_>("id"),
                guid: Uuid::parse_str(row.read::<&str,_>("guid")).unwrap(),
                name: row.read::<&str,_>("name").to_owned(),
                class: row.read::<i64,_>("ixClass") as u16,
                data,
            });

        }

            
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(IndexModel::builder().keys(doc!("id": 1)).build(), None).await?;
            collection.create_index(IndexModel::builder().keys(doc!("guid": "text")).build(), None).await?;
            Ok(())
        })
    })
}

async fn import_instance(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<RawInstance>("instances")
        });

        info!("Importing Instance -> instances...");
    
        let result = db
            .prepare(format!("SELECT * FROM Instance"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut documents = Vec::new();
    
        // dump data
        for row in result {
            let bin_data = row.read::<&[u8], _>("data");
            let data = if !bin_data.is_empty() {
                let mut class = ParamClassContainer::read(row.read::<i64,_>("ixClass") as u16, bin_data).unwrap().1;
                class.strip_original_data();
                Some(class)
            } else {
                None
            };
    
            documents.push(RawInstance {
                id: row.read::<i64,_>("ixInstanceID"),
                guid: Uuid::parse_str(row.read::<&str,_>("uxInstanceGuid")).unwrap(),
                zone_guid: Uuid::parse_str(row.read::<&str,_>("uxZoneGuid")).unwrap(),
                class: row.read::<i64,_>("ixClass"),
                content_guid: Uuid::parse_str(row.read::<&str,_>("uxContentGuid")).unwrap(),
                editor_name: row.read::<&str,_>("sEditorName").to_owned(),
                data,
                phase_tag: row.read::<&str,_>("phaseTag").to_owned(),
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            collection.create_index(
                IndexModel::builder()
                .keys(doc!("guid": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            Ok(())
        })
    })
}


async fn import_worlddef(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<WorldDef>("worlddefs")
        });

        info!("Importing WorldDef -> worlddefs...");
    
        let result = db
            .prepare(format!("SELECT * FROM WorldDef"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut documents = Vec::new();
    
        // dump data
        for row in result {   
            documents.push(WorldDef {
                id: row.read::<i64,_>("ixWorldID") as u16,
                guid: Uuid::parse_str(row.read::<&str,_>("uxWorldDefGuid")).unwrap(),
                name: row.read::<&str,_>("sWorldDef").to_owned(),
                umap_guid: Uuid::parse_str(row.read::<&str,_>("uxUMapGuid")).unwrap(),
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            collection.create_index(
                IndexModel::builder()
                .keys(doc!("guid": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            Ok(())
        })
    })
}

async fn import_zone(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<ZoneDef>("zones")
        });

        info!("Importing Zone -> zones...");
    
        let result = db
            .prepare(format!("SELECT * FROM Zone"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

            let mut documents = Vec::new();
    
        // dump data
        for row in result {   
            documents.push(ZoneDef {
                id: row.read::<i64,_>("ixZoneID"),
                guid: Uuid::parse_str(row.read::<&str,_>("uxZoneGuid")).unwrap(),
                worlddef_guid: Uuid::parse_str(row.read::<&str,_>("uxWorldDefGuid")).unwrap(),
                parent_zone_guid: Uuid::parse_str(row.read::<&str,_>("uxParentZoneGuid")).unwrap(),
                zone: row.read::<&str,_>("sZone").to_owned(),
                zone_type: row.read::<i64,_>("iType") as i32,
                is_instance: row.read::<i64,_>("bInstance") != 0,
                server: row.read::<&str,_>("sServer").to_owned(),
                level: row.read::<&str,_>("sLevel").to_owned(),
                layer: row.read::<&str,_>("sLayer").to_owned(),
                realu_zone_type: row.read::<&str,_>("sRealUZoneType").to_owned(),
                game_controller: row.read::<&str,_>("sGameController").to_owned(),
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            collection.create_index(
                IndexModel::builder()
                .keys(doc!("guid": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            Ok(())
        })
    })
}

async fn import_vendor_data(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            cluster_database().await.collection::<CashShopVendor>("cash_shop_vendors")
        });

        info!("Importing VendorData -> cash_shop_vendors...");
    
        let result = db
            .prepare(format!("SELECT * FROM VendorData"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

            let mut documents = Vec::new();
    
        // dump data
        for row in result {   
            documents.push(CashShopVendor {
                id: Uuid::parse_str(row.read::<&str,_>("VendorID")).unwrap(),
                name: row.read::<&str,_>("VendorName").to_owned(),
                sku_list: row.read::<&str,_>("SKUList")
                    .split(",")
                    .map(|s| s.trim())
                    .filter(|s| s.len() == 36)
                    .map(|s| Uuid::parse_str(s)
                    .unwrap())
                    .collect(),
                bundle_list: row.try_read::<&str,_>("BundleList")
                    .unwrap_or("")
                    .split(",")
                    .map(|s| s.trim())
                    .filter(|s| s.len() == 36)
                    .map(|s| Uuid::parse_str(s)
                    .unwrap()).collect(),
                version: row.read::<i64,_>("Version") as u32
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;

            Ok(())
        })
    })
}

async fn import_shop_items(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            cluster_database().await.collection::<CashShopItem>("cash_shop_items")
        });

        info!("Importing SKUItems -> cash_shop_items...");
    
        let result = db
            .prepare(format!("SELECT * FROM SKUItems"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

            let mut documents = Vec::new();
    
        // dump data
        for row in result {   
            documents.push(CashShopItem {
                id: Uuid::parse_str(row.read::<&str,_>("SKUID")).unwrap(),
                display_name: row.read::<&str,_>("DisplayName").to_owned(),
                description: row.read::<&str,_>("Description").to_owned(),
                reference_item_name: row.read::<&str,_>("ReferenceItemName").to_owned(),
                reference_item_guid: Uuid::parse_str(row.read::<&str,_>("ReferenceItemGUID")).unwrap(),
                cash_price: row.read::<&str,_>("CashPrice").parse::<u32>().unwrap(),
                sku_code: row.read::<&str,_>("SKUCode").to_owned(),
                rental_duration: row.read::<&str,_>("RentalDuration").parse::<u32>().unwrap(),
                is_in_stock: row.read::<&str,_>("IsInStock").parse::<u32>().unwrap() != 0,
                is_hot: row.read::<&str,_>("IsHot").parse::<u32>().unwrap() != 0,
                is_new: row.read::<&str,_>("IsNew").parse::<u32>().unwrap() != 0,
                version: row.read::<&str,_>("Version").parse::<u32>().unwrap(),
                is_visible: row.read::<&str,_>("IsVisible").parse::<u32>().unwrap() != 0,
                is_tradable: row.read::<&str,_>("IsTradable").parse::<u32>().unwrap() != 0,
                is_featured: row.read::<&str,_>("IsFeatured").parse::<u32>().unwrap() != 0,
                quantity: row.read::<&str,_>("Quantity").parse::<u32>().unwrap(),
                discount: row.read::<&str,_>("Discount").parse::<u32>().unwrap(),
                date_start: DateTime::parse_from_rfc3339(row.read::<&str,_>("DateStart"))
                    .map(|e| Some(e.with_timezone(&Utc))).unwrap_or(None),
                date_end: DateTime::parse_from_rfc3339(row.read::<&str,_>("DateEnd"))
                    .map(|e| Some(e.with_timezone(&Utc))).unwrap_or(None)
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("sku_code": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            Ok(())
        })
    })
}


async fn import_shop_bundles(game_client_path: &PathBuf) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/system/SKUExport/SKUItems.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            cluster_database().await.collection::<CashShopBundle>("cash_shop_bundles")
        });

        info!("Importing BundleItems -> cash_shop_bundles...");
    
        let result = db
            .prepare(format!("SELECT * FROM BundleItems"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

            let mut documents = Vec::new();
    
        // dump data
        for row in result {   
            documents.push(CashShopBundle {
                id: Uuid::parse_str(row.read::<&str,_>("BundleID")).unwrap(),
                display_name: row.read::<&str,_>("DisplayName").to_owned(),
                description: row.read::<&str,_>("Description").to_owned(),
                cash_price: row.read::<i64,_>("CashPrice") as u32,
                icon: row.read::<&str,_>("Icon").to_owned(),
                item_list_andcount: row.read::<&str,_>("ItemListAndCount").to_owned().split(",").filter(|s| s.len() != 0).map(|s| {
                    let item_count: Vec<_> = s.split("=").collect();
                    (item_count[0].to_owned(), item_count[1].parse().unwrap())
                }).collect(),
                is_in_stock: row.read::<i64,_>("IsInStock") != 0,
                is_hot: row.read::<i64,_>("IsHot") != 0,
                is_new: row.read::<i64,_>("IsNew") != 0,
                version: row.read::<i64,_>("Version") as u32,
                is_visible: row.read::<i64,_>("IsVisible") != 0,
                is_tradable: row.read::<i64,_>("IsTradable") != 0,
                is_featured: row.read::<i64,_>("IsFeatured") != 0,
                quantity: row.read::<i64,_>("Quantity") as u32,
                discount: row.read::<i64,_>("Discount") as u32,
                date_start: DateTime::parse_from_rfc3339(row.read::<&str,_>("DateStart"))
                    .map(|e| Some(e.with_timezone(&Utc))).unwrap_or(None),
                date_end: DateTime::parse_from_rfc3339(row.read::<&str,_>("DateEnd"))
                    .map(|e| Some(e.with_timezone(&Utc))).unwrap_or(None)
            });
        }
    
        Handle::current().block_on(async {
            if !documents.is_empty() {
                collection.insert_many(documents, None).await?;
            }

            collection.create_index(
                IndexModel::builder()
                .keys(doc!("id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;

            Ok(())
        })
    })
}



pub async fn import_client_data(game_client_path: PathBuf) -> AnotherlandResult<()> {
    tokio::task::spawn(async move {
        import_content_table(&game_client_path, "NoBinding", "no_binding").await?;
        import_content_table(&game_client_path, "buffs", "buffs").await?;
        import_content_table(&game_client_path, "drops", "drops").await?;
        import_content_table(&game_client_path, "enemies", "enemies").await?;
        import_content_table(&game_client_path, "enemies", "enemies").await?;
        import_content_table(&game_client_path, "factions", "factions").await?;
        import_content_table(&game_client_path, "items", "items").await?;
        import_content_table(&game_client_path, "metagame", "metagame").await?;
        import_content_table(&game_client_path, "misc", "misc").await?;
        import_content_table(&game_client_path, "npcs", "npcs").await?;
        import_content_table(&game_client_path, "projectiles", "projectiles").await?;
        import_content_table(&game_client_path, "quests", "quests").await?;
        import_content_table(&game_client_path, "recipes", "recipes").await?;
        import_content_table(&game_client_path, "skills", "skills").await?;
        import_content_table(&game_client_path, "spawners", "spawners").await?;
        import_content_table(&game_client_path, "structures", "structures").await?;

        import_instance(&game_client_path).await?;
        import_worlddef(&game_client_path).await?;
        import_zone(&game_client_path).await?;

        import_vendor_data(&game_client_path).await?;
        import_shop_items(&game_client_path).await?;
        import_shop_bundles(&game_client_path).await?;

        Ok(())
    }).await?
}