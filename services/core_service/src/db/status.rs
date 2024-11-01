// Copyright (C) 2024 AnotherlandServer
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

use bson::{doc, Document};
use database::{DBResult, DatabaseRecord};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub cluster_locked: Option<bool>,
}

impl Status {
    pub async fn lock_cluster(db: &Database) -> DBResult<Status> {
        Self::collection(db).update_one(
            Self::query_one(&()), 
            doc!{"$set": { "cluster_locked": true }}
        ).upsert(true).await?;

        Self::get(db, &()).await
            .map(|status| status.unwrap_or_default())
    }

    pub async fn unlock_cluster(db: &Database) -> DBResult<Status> {
        Self::collection(db).update_one(
            Self::query_one(&()), 
            doc!{"$set": { "cluster_locked": false }}
        ).upsert(true).await?;

        Self::get(db, &()).await
            .map(|status| status.unwrap_or_default())
    }
}

impl DatabaseRecord<'_> for Status {
    type PrimaryKey = ();

    fn key(&self) -> &Self::PrimaryKey {
        &()
    }

    fn collection_name() -> &'static str {
        "status"
    }

    fn query_one(_key: &Self::PrimaryKey) -> Document {
        doc!{}
    }
}