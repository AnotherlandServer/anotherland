use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::Uuid;

use crate::util::AnotherlandResult;

use super::DatabaseRecord;

#[derive(Clone, Serialize, Deserialize)]
pub struct ZoneDef {
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
impl DatabaseRecord<'_> for ZoneDef {
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

impl ZoneDef {
    pub async fn load_for_world(db: Database, world_guid: &Uuid) -> AnotherlandResult<Vec<ZoneDef>> {
        let mut rows = Vec::new();

        let mut result = Self::collection(db).find(doc!{"worlddef_guid": {"$eq": world_guid.to_string()}}, None).await?;
        while let Some(row) = result.try_next().await? {
            rows.push(row);
        }

        Ok(rows)
    }
}