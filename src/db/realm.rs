use bson::doc;
use mongodb::Database;
use serde::Serialize;
use serde_derive::Deserialize;
use tokio_stream::StreamExt;

use crate::util::AnotherlandResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    pub id: u32,
    pub name: String,
    pub external_ip: String,
    pub external_port: u16,
    pub population: f32,
}

impl Realm {
    pub async fn list(db: Database) -> AnotherlandResult<Vec<Realm>> {
        let collection = db.collection::<Realm>("realms");
        let mut realms = Vec::new();

        let mut result = collection.find(None, None).await?;
        while let Some(realm) = result.try_next().await? {
            realms.push(realm);
        }

        Ok(realms)
    }
}
