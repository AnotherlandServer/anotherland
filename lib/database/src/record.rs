// Copyright (C) 2025 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use mongodb::{action::Update, bson::{self, doc, Document}, Collection, Cursor, Database};
use serde::{de::DeserializeOwned, Serialize};
use crate::DBResult;

#[allow(async_fn_in_trait)]
pub trait DatabaseRecord: DeserializeOwned + Serialize + Send + Sync + Unpin {
    type PrimaryKey: DeserializeOwned + Serialize + Send + Sync;
    
    fn key(&self) -> &Self::PrimaryKey;
    fn key_name() -> &'static str;
    fn collection_name() -> &'static str;
    fn relations() -> &'static[(&'static str, &'static str)] { &[] }

    fn collection(db: &Database) -> Collection<Self> {
        db.collection::<Self>(Self::collection_name())
    }

    async fn build_index(_db: &Database) -> DBResult<()> { Ok(()) }

    fn query_one(key: &Self::PrimaryKey) -> Document {
        doc!{ Self::key_name(): { "$eq": bson::to_bson(key).unwrap() } }
    }

    async fn get(db: &Database, key: &Self::PrimaryKey) -> DBResult<Option<Self>> {
        let collection = Self::collection(db);

        Ok(collection.find_one(
            Self::query_one(key), 
        ).await?)
    }

    async fn create(db: &Database, record: Self) -> DBResult<Self> {
        let collection = Self::collection(db);

        collection.insert_one(
            &record
        ).await?;

        Ok(record)
    }

    fn save_uncommited<'a>(&mut self, collection: &'a Collection<Self>) -> Update<'a> {
        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": bson::to_bson(self).unwrap().as_document().unwrap()},
        )
    }

    async fn save(&mut self, db: &Database) -> DBResult<()> {
        let collection = Self::collection(db);

        self.save_uncommited(&collection).await?;

        Ok(())
    }

    async fn delete(&self, db: &Database) -> DBResult<()> {
        let collection = Self::collection(db);

        let mut session = db.client()
            .start_session()
            .await?;

        
        session.start_transaction().await?;

        collection.delete_one(
            Self::query_one(self.key()),
        )
        .session(&mut session)
        .await?;

        for relation in Self::relations() {
            let collection = db.collection::<Document>(relation.0);

            collection.delete_one(
                doc! { relation.1: { "$eq": bson::to_bson(self.key()).unwrap() } }
            )
            .session(&mut session)
            .await?;
        }

        session.commit_transaction().await?;

        Ok(())
    }

    async fn list(db: &Database) -> DBResult<Cursor<Self>> {
        Ok(Self::collection(db).find(doc!{}).await?)
    }
}
