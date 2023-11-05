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
    pub async fn load_for_world(db: Database, world_guid: &Uuid) -> AnotherlandResult<Vec<Self>> {
        let mut rows = Vec::new();

        let mut result = Self::collection(db).find(doc!{"worlddef_guid": {"$eq": world_guid.to_string()}}, None).await?;
        while let Some(row) = result.try_next().await? {
            rows.push(row);
        }

        Ok(rows)
    }

    pub async fn list(db: Database) -> AnotherlandResult<Vec<Self>> {
        let collection = Self::collection(db);
        let mut zonedefs = Vec::new();

        let mut result = collection.find(None, None).await?;
        while let Some(zonedef) = result.try_next().await? {
            zonedefs.push(zonedef);
        }

        Ok(zonedefs)
    }

    pub async fn get_by_name(db: Database, name: &str) -> AnotherlandResult<Option<ZoneDef>> {
        let collection = Self::collection(db);

        Ok(collection.find_one(
            doc! {"zone": name}, 
            None
        ).await?)  
    }
}