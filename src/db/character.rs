use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use bson::{doc, Document};
use chrono::{DateTime, Utc};
use glam::Vec3;
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use sha1::{Sha1, Digest};
use tokio_stream::StreamExt;

use crate::{util::AnotherlandResult};
use atlas::{Uuid, PlayerParam, Player};

use super::{Account, DatabaseRecord, ItemContent};

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub guid: Uuid,
    pub account: Uuid,
    pub name: String,
    pub world_id: u32,
    
    pub data: PlayerParam,
}

static NEW_CHARACTER_TEMPLATE: Lazy<PlayerParam> = Lazy::new(|| {
    let mut player = PlayerParam::default();

    player.set_world_map_guid(Uuid::from_str("f6b8f8b7-a726-4d36-9634-f6d403943fff").unwrap());
    player.set_zone_guid(Uuid::from_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap());
    player.set_zone("ClassSelection_P");

    // default customization
    player.set_visible_item_info(vec![3840]);
    player.set_customization_gender(1.0);
    player.set_customization_height(0.5);
    player.set_customization_bust_size(0.5);
    player.set_customization_fat(0.0);
    player.set_customization_skinny(0.7);
    player.set_customization_muscular(0.3);

    player
}); 

impl Character {
    pub async fn create(db: Database, account_id: &Uuid, name: &str) -> AnotherlandResult<Character> {
        let guid = Uuid::new_v4();

        // Compute numeric character id, similar to how we build account ids..
        let mut hasher = Sha1::new();
        hasher.update(guid.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());

        let mut avatar_data = NEW_CHARACTER_TEMPLATE.clone();
        let default_items = ItemContent::list_by_categories(db.clone(), vec![
            Uuid::from_str("6B74CF2D-79A3-48B8-B752-995179A064BD").unwrap()
        ].as_slice()).await?;

        debug!("Default item count: {}", default_items.len());

        avatar_data.set_default_items_content_guid(default_items.into_iter().map(|v| v.id as i32).collect());

        let character = Character {
            id: numeric_id,
            guid,
            account: account_id.clone(),
            name: name.to_owned(),
            world_id: 130,
            data: avatar_data,
        };

        let collection = Character::collection(db.clone());
        collection.insert_one(&character, None).await?;

        Ok(character)
    }

    pub async fn list(db: Database, account_id: &Uuid) -> AnotherlandResult<Vec<Character>> {
        let collection = Character::collection(db.clone());
        let mut chracters = Vec::new();

        let mut result = collection.find(doc!{"account": {"$eq":account_id.to_string()}}, None).await?;
        while let Some(character) = result.try_next().await? {
            chracters.push(character);
        }

        Ok(chracters)
    }

    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = Character::collection(db.clone());
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("guid": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("name": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("account": 1))
            .options(IndexOptions::builder().build())
            .build(), 
            None).await?;

        Ok(())
    }

    pub async fn save(&mut self, db: Database) -> AnotherlandResult<()> {
        let collection = db.collection::<Character>("characters");
        collection.update_one(doc!{"id": {"$eq": self.id}}, doc!{"$set": &bson::to_bson(self).unwrap().as_document()}, None).await?;

        Ok(())
    }
}

#[async_trait]
impl DatabaseRecord<'_> for Character {
    type Key = u32;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("characters")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}