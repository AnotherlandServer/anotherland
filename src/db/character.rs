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
    player.set_alive(true);
    player.set_attribute_constitution(32.0);
    player.set_attribute_crafting(32.0);
    player.set_attribute_dexterity(32.0);
    player.set_attribute_disguise(32.0);
    player.set_attribute_energy(32.0);
    player.set_attribute_focus(32.0);
    player.set_attribute_intuition(32.0);
    player.set_attribute_movement(32.0);
    player.set_attribute_strength(32.0);
    player.set_attribute_wisdom(32.0);
    player.set_auto_loot_radius(60.0);
    player.set_aware_dist(3900.0);
    player.set_aware_range(3900.0);
    player.set_bling(-1);
    player.set_collision_extent(Vec3::new(21.0, 21.0, 21.0));
    player.set_combat_style(0);
    player.set_game_cash(-1);
    player.set_hp_cur(1000);
    player.set_hp_max(1000);
    player.set_hp_min(0);
    player.set_jump_velocity(310.0);
    player.set_move_speed(192.0);
    player.set_lvl(1);
    player.set_size(1.0);
    player.set_spawn_mode(0);
    player.set_zone("ClassSelection_P");
    player.set_zone_guid(Uuid::from_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap());
    player.set_first_time_spawn(true);
    player.set_default_items_content_guid(vec![40711, 40712, 40713, 41206]);
    player.set_last_skusync_time(UNIX_EPOCH.elapsed().unwrap().as_secs() as i64);
    player.set_last_vendor_sync_time(UNIX_EPOCH.elapsed().unwrap().as_secs() as i64);
    player.set_tutorial_mode(false);

    // visible stuff
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