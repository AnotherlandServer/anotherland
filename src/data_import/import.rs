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

use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use bevy::log::debug;
use chrono::{DateTime, Utc};
use futures::{future::AndThen, StreamExt, TryStreamExt};
use glam::Vec3;
use log::info;
use mongodb::{IndexModel, options::IndexOptions, bson::doc};
use regex::Regex;
use tokio::runtime::Handle;

use crate::{db::{cluster_database, realm_database, CashShopBundle, CashShopItem, CashShopVendor, Content, ControlPoint, DatabaseRecord, DisplayName, FlightTube, RawInstance, WorldDef, ZoneDef}, util::AnotherlandResult};
use atlas::{ParamBox, ParamSetBox, Uuid};
use upk::{types::{ObjectProperty, ScriptObject}, Container};

async fn import_content_table(game_client_path: &Path, src_table: &str, target_table: &str) -> AnotherlandResult<()> {
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
                let mut class = ParamBox::read(row.read::<i64,_>("ixClass") as u16, bin_data).unwrap().1;
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

async fn import_instance(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM Instance")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut documents = Vec::new();
    
        // dump data
        for row in result {
            let bin_data = row.read::<&[u8], _>("data");
            let data = if !bin_data.is_empty() {
                let mut set = ParamSetBox::read(row.read::<i64,_>("ixClass") as u16, bin_data).unwrap().1;
                set.strip_original_data();
                Some(set)
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


async fn import_worlddef(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM WorldDef")
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

async fn import_zone(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM Zone")
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

async fn import_vendor_data(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM VendorData")
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
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| s.len() == 36)
                    .map(|s| Uuid::parse_str(s)
                    .unwrap())
                    .collect(),
                bundle_list: row.try_read::<&str,_>("BundleList")
                    .unwrap_or("")
                    .split(',')
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

async fn import_shop_items(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM SKUItems")
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


async fn import_shop_bundles(game_client_path: &Path) -> AnotherlandResult<()> {
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
            .prepare("SELECT * FROM BundleItems")
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
                item_list_andcount: row.read::<&str,_>("ItemListAndCount").split(',').filter(|s| !s.is_empty()).map(|s| {
                    let item_count: Vec<_> = s.split('=').collect();
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

pub async fn import_flighttubes(game_client_path: &Path) -> AnotherlandResult<()> {
    let tube_collection = FlightTube::collection(realm_database().await);
    let mut worlds = WorldDef::collection(realm_database().await).find(None, None).await?;

    let mut upk_collection = Container::new(game_client_path.join("UnrealEngine3/AmunGame/CookedPCConsole"));

    let mut documents = HashMap::new();

    while let Some(world) = worlds.try_next().await? {
        let _ = upk_collection.mount_package(&world.name).await;

        info!("Importing FlightTubes from {}...", world.name);

        // scan for flight tube actors
        for obj in upk_collection.objects() {
            if obj.class().name() == "OLFlightTubeActor" && !obj.name().contains("Default__") {
                let so = upk_collection.deserialize::<ScriptObject>(obj).await?;
                
                let id = so.attrib("UniqueGUID")
                    .and_then(|attrib| {
                        if let ObjectProperty::Guid(guid) = attrib {
                            Some(Uuid::from_uuid_1(*guid))
                        } else {
                            None
                        }
                    })
                    .unwrap();

                let travel_target = so.attrib("TravelTargetGUID")
                    .and_then(|attrib| {
                        if let ObjectProperty::Guid(guid) = attrib {
                            Some(Uuid::from_uuid_1(*guid))
                        } else {
                            None
                        }
                    })
                    .unwrap();

                let surfing_speed = so.attrib("SurfingSpeed")
                    .and_then(|attrib| {
                        if let ObjectProperty::Float(val) = attrib {
                            Some(*val)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(1500.0);

                let location = so.attrib("Location")
                    .and_then(|attrib| {
                        if let ObjectProperty::Vector(val) = attrib {
                            Some(Vec3::from_array(*val))
                        } else {
                            None
                        }
                    })
                    .unwrap();

                let control_points = so.attrib("ControlPoints")
                    .and_then(|attrib| {
                        (if let ObjectProperty::Array(control_points) = attrib {
                            Some(control_points)
                        } else {
                            None
                        })
                        .map(|control_points| {
                            control_points
                                .iter()
                                .filter_map(|property| {
                                    if let ObjectProperty::Struct(_, obj) = property {
                                        Some(obj)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .map(|control_points| {
                            control_points.iter().map(|control_point| {
                                let point = control_point.attrib("Point")
                                    .and_then(|attrib| {
                                        if let ObjectProperty::Vector(val) = attrib {
                                            Some(Vec3::from_array(*val))
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap();

                                let tangent = control_point.attrib("Tangent")
                                    .and_then(|attrib| {
                                        if let ObjectProperty::Vector(val) = attrib {
                                            Some(Vec3::from_array(*val))
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap();

                                ControlPoint {
                                    point,
                                    tangent
                                }
                            })
                            .collect()
                        })
                    })
                    .unwrap();

                documents.insert(id, FlightTube {
                    id,
                    name: obj.name().to_owned(),
                    travel_target,
                    surfing_speed,
                    location,
                    control_points,
                });
            }
        }

        upk_collection.umount_package(&world.name);
    }

    if !documents.is_empty() {
        tube_collection.insert_many(documents.values(), None).await?;
    }

    Ok(())
}

async fn import_display_names(game_client_path: &Path) -> AnotherlandResult<()> {
    import_display_names_from_file(game_client_path.join("Atlas/data/otherlandgame/localization/International/common.txt")).await?;
    import_display_names_from_file(game_client_path.join("Atlas/data/otherlandgame/localization/International/AutoGenerated_common.txt")).await?;

    Ok(())
}

async fn import_display_names_from_file(path: PathBuf) -> AnotherlandResult<()> {
    info!("Importing locales from {}...", path.file_name().unwrap().to_string_lossy());

    let file = {
        let content = fs::read(path)?;
        String::from_utf16le(&content[2..]).unwrap()
    };
        
    let lines = file.lines();
    let mut category = String::default();
    let mut locales = Vec::new();

    let category_expression = Regex::new(r"^\[([^)]+)\]").unwrap();
    let name_expression = Regex::new(r"([^,]+)\s*,\s*(\S+)").unwrap();

    for line in lines {
        if let Some(captures) = category_expression.captures(line) {
            captures.get(1).unwrap().as_str().clone_into(&mut category);
        } else if let Some(captures) = name_expression.captures(line) {
            let id = Uuid::parse_str(captures.get(1).unwrap().as_str()).unwrap();
            
            locales.push(DisplayName {
                id,
                name: format!("#{}.{}#", category, captures.get(2).unwrap().as_str()),
            });
        } else if !line.is_empty() {
            panic!("'{}'", line);
        }
    }

    DisplayName::collection(realm_database().await).insert_many(locales, None).await?;

    Ok(())
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

        import_flighttubes(&game_client_path).await?;

        import_display_names(&game_client_path).await?;

        Ok(())
    }).await?
}