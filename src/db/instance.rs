use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};

use atlas::{Uuid, CParamClass};

use super::DatabaseRecord;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub id: i64,
    pub guid: Uuid,
    pub zone_guid: Uuid,
    pub class: i64,
    pub content_guid: Uuid,
    pub editor_name: String,
    pub data: Option<CParamClass>,
    pub phase_tag: String,
}

#[async_trait]
impl DatabaseRecord<'_> for Instance {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("instances")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.guid
    }
}