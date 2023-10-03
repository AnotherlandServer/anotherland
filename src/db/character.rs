use async_trait::async_trait;
use bson::{doc, Document};
use glam::Vec3;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use sha1::{Sha1, Digest};
use tokio_stream::StreamExt;

use crate::{util::AnotherlandResult};
use atlas::{Uuid, CParamClass_player, CParam};

use super::{Account, DatabaseRecord};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub guid: Uuid,
    pub account: Uuid,
    pub name: String,
    pub world_id: u32,
    
    pub data: CParamClass_player,
}

static NEW_CHARACTER_TEMPLATE: Lazy<CParamClass_player> = Lazy::new(|| CParamClass_player {
    alive: Some(CParam::Bool(true)),
    attribute_constitution: Some(CParam::Float(32.0)),
    attribute_crafting: Some(CParam::Float(32.0)),
    attribute_dexterity: Some(CParam::Float(32.0)),
    attribute_disguise: Some(CParam::Float(32.0)),
    attribute_energy: Some(CParam::Float(32.0)),
    attribute_focus: Some(CParam::Float(32.0)),
    attribute_energy_max: Some(CParam::Float(1.0)),
    attribute_intuition: Some(CParam::Float(32.0)),
    attribute_movement: Some(CParam::Float(32.0)),
    attribute_strength: Some(CParam::Float(32.0)),
    attribute_wisdom: Some(CParam::Float(32.0)),
    auto_loot_radius: Some(CParam::Float(60.0)),
    aware_dist: Some(CParam::Float(3900.0)),
    aware_range: Some(CParam::Float(3900.0)),
    bling: Some(CParam::Int32(-1)),
    collision_extent: Some(CParam::Vector3(Vec3 { x: 21.0, y: 21.0, z: 21.0 })),
    combat_style: Some(CParam::Int32(6)),
    game_cash: Some(CParam::Int32(-1)),
    hp_cur: Some(CParam::Int32(1000)),
    hp_max: Some(CParam::Int32(1000)),
    hp_min: Some(CParam::Int32(0)),
    jump_velocity: Some(CParam::Float(310.0)),
    move_speed: Some(CParam::Float(192.0)),
    lvl: Some(CParam::Int32(1)), 
    size: Some(CParam::Float(1.0)),
    spawn_mode: Some(CParam::Int32(0)),
    ue3class_id: Some(CParam::String("Engine.AtlasAvatar".to_owned())),
    zone: Some(CParam::String("ClassSelection_P".to_owned())),
    zone_guid: Some(CParam::CGuid(Uuid::from_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap())),

    ..CParamClass_player::default()
});

impl Character {
    pub async fn create(db: Database, account_id: &Uuid, name: &str) -> AnotherlandResult<Character> {
        let guid = Uuid::new_v4();

        // Compute numeric character id, similar to how we build account ids..
        let mut hasher = Sha1::new();
        hasher.update(guid.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());

        let character = Character {
            id: numeric_id,
            guid,
            account: account_id.clone(),
            name: name.to_owned(),
            world_id: 130,
            data: NEW_CHARACTER_TEMPLATE.clone(),
        };

        let collection = db.collection::<Character>("characters");
        collection.insert_one(&character, None).await?;

        Ok(character)
    }

    pub async fn list(db: Database, account_id: &Uuid) -> AnotherlandResult<Vec<Character>> {
        let collection = db.collection::<Character>("characters");
        let mut chracters = Vec::new();

        let mut result = collection.find(doc!{"account": {"$eq":account_id.to_string()}}, None).await?;
        while let Some(character) = result.try_next().await? {
            chracters.push(character);
        }

        Ok(chracters)
    }

    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = db.collection::<Character>("characters");
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