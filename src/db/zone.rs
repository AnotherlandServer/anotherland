use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};

use atlas::Uuid;

use super::DatabaseRecord;

#[derive(Serialize, Deserialize)]
pub struct Zone {
    pub id: i64,
    pub guid: Uuid,
    pub worlddef_guid: Uuid,
    pub parent_zone_guid: Uuid,
    pub zone: String,
    pub zone_type: i32,
    pub is_instance: bool,
    pub server: String,
    pub level: String,
    pub layer: String,
    pub realu_zone_type: String,
    pub game_controller: String,
}

#[async_trait]
impl DatabaseRecord<'_> for Zone {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("zones")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.guid
    }
}