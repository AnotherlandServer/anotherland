use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::Uuid;

use crate::util::AnotherlandResult;

use super::DatabaseRecord;

#[derive(Clone, Serialize, Deserialize)]
pub struct WorldServerEntry {
    pub world_id: u16,
    pub external_ip: String,
    pub external_port: u16,
}

#[async_trait]
impl DatabaseRecord<'_> for WorldServerEntry {
    type Key = u16;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("worldservers")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "world_id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.world_id
    }
}
