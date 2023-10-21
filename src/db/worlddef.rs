use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::Uuid;

use crate::{util::AnotherlandResult};

use super::DatabaseRecord;

#[derive(Clone, Serialize, Deserialize)]
pub struct WorldDef {
    pub id: u16,
    pub guid: Uuid,
    pub name: String,
    pub umap_guid: Uuid,
}

#[async_trait]
impl DatabaseRecord<'_> for WorldDef {
    type Key = u16;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("worlddefs")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

impl WorldDef {
    pub async fn get_by_name(db: Database, name: &str) -> AnotherlandResult<Option<WorldDef>> {
        let collection = Self::collection(db);
        Ok(collection.find_one(doc!{"name": {"$eq": name}}, None).await?)
    }

    pub async fn list(db: Database) -> AnotherlandResult<Vec<WorldDef>> {
        let collection = Self::collection(db);
        let mut worlddefs = Vec::new();

        let mut result = collection.find(None, None).await?;
        while let Some(worlddef) = result.try_next().await? {
            worlddefs.push(worlddef);
        }

        Ok(worlddefs)
    }
}
