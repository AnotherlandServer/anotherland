use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Client, Database, Collection, options::UpdateOptions};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio_stream::StreamExt;

use crate::{ARGS, util::AnotherlandResult};

#[async_trait]
pub trait DatabaseRecord<'de>: DeserializeOwned + Serialize + Send + Sync + Unpin {
    //type Record: DatabaseRecord<'de>;
    type Key: Deserialize<'de> + Serialize + Send + Sync;

    fn collection(db: Database) -> Collection<Self>;

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key;

    async fn get(db: Database, key: &Self::Key) -> AnotherlandResult<Option<Self>> {
        let collection = Self::collection(db);

        Ok(collection.find_one(
            Self::query_one(key), 
            None
        ).await?)
    }

    async fn create(db: Database, record: Self) -> AnotherlandResult<Self> {
        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(record.key()), 
            doc!{"$set": bson::to_bson(&record).unwrap().as_document().unwrap()},
            UpdateOptions::builder().upsert(true).build()
        ).await?;

        Ok(record)
    }

    async fn save(&mut self, db: Database) -> AnotherlandResult<()> {
        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": bson::to_bson(self).unwrap().as_document().unwrap()},
            None
        ).await?;

        Ok(())
    }

    async fn delete(&self, db: Database) -> AnotherlandResult<()> {
        let collection = Self::collection(db);

        collection.delete_one(
            Self::query_one(self.key()),
            None
        ).await?;

        Ok(())
    }

    async fn list(db: Database) -> AnotherlandResult<Vec<Self>> {
        let mut rows = Vec::new();

        let mut result = Self::collection(db).find(None, None).await?;
        while let Some(row) = result.try_next().await? {
            rows.push(row);
        }

        Ok(rows)
    }
}


pub async fn client() -> Client {
    Client::with_uri_str(&ARGS.mongo_uri).await
        .expect("Database connection failed")
}

pub async fn cluster_database() -> Database {
    client().await.database(&ARGS.mongo_cluster_db)
}

pub async fn realm_database() -> Database {
    match ARGS.mongo_realm_db() {
        Some(db) => client().await.database(&db),
        None => panic!("Realm database not set!"),
    }
}

pub async fn database(db: &str) -> Database {
    client().await.database(db)
}