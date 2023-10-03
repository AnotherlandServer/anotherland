use std::path::PathBuf;

use log::info;
use mongodb::{IndexModel, options::IndexOptions, bson::doc};
use tokio::runtime::Handle;

use crate::{util::AnotherlandResult, db::{Content, Instance, WorldDef, Zone, realm_database}};
use atlas::{CParamClass, Uuid};

async fn import_content_table(game_client_path: &PathBuf, src_table: &str, target_table: &str) -> AnotherlandResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas\\data\\otherlandgame\\content\\dbbba21e-2342-4357-a777-302ed11b978b\\content.db")
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
                Some(CParamClass::parse_class(row.read::<i64,_>("ixClass") as u16, 
                    bin_data).unwrap().1)
            } else {
                None
            };
    
            documents.push(Content {
                id: row.read::<i64,_>("id"),
                guid: Uuid::from_str(row.read::<&str,_>("guid")).unwrap(),
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
            .join("Atlas\\data\\otherlandgame\\content\\dbbba21e-2342-4357-a777-302ed11b978b\\instance.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<Instance>("instances")
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
                Some(CParamClass::parse_class(row.read::<i64,_>("ixClass") as u16, 
                    bin_data).unwrap().1)
            } else {
                None
            };
    
            documents.push(Instance {
                id: row.read::<i64,_>("ixInstanceID"),
                guid: Uuid::from_str(row.read::<&str,_>("uxInstanceGuid")).unwrap(),
                zone_guid: Uuid::from_str(row.read::<&str,_>("uxZoneGuid")).unwrap(),
                class: row.read::<i64,_>("ixClass"),
                content_guid: Uuid::from_str(row.read::<&str,_>("uxContentGuid")).unwrap(),
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
                .keys(doc!("instance_id": 1))
                .options(IndexOptions::builder().unique(true).build())
                .build(), None).await?;
            collection.create_index(
                IndexModel::builder()
                .keys(doc!("instance_guid": 1))
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
            .join("Atlas\\data\\otherlandgame\\content\\dbbba21e-2342-4357-a777-302ed11b978b\\instance.db")
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
                guid: Uuid::from_str(row.read::<&str,_>("uxWorldDefGuid")).unwrap(),
                name: row.read::<&str,_>("sWorldDef").to_owned(),
                umap_guid: Uuid::from_str(row.read::<&str,_>("uxUMapGuid")).unwrap(),
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
            .join("Atlas\\data\\otherlandgame\\content\\dbbba21e-2342-4357-a777-302ed11b978b\\instance.db")
        ).unwrap();

        let collection = Handle::current().block_on(async move {
            realm_database().await.collection::<Zone>("zones")
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
            documents.push(Zone {
                id: row.read::<i64,_>("ixZoneID"),
                guid: Uuid::from_str(row.read::<&str,_>("uxZoneGuid")).unwrap(),
                worlddef_guid: Uuid::from_str(row.read::<&str,_>("uxWorldDefGuid")).unwrap(),
                parent_zone_guid: Uuid::from_str(row.read::<&str,_>("uxParentZoneGuid")).unwrap(),
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

        Ok(())
    }).await?
}