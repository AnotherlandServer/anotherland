// Copyright (C) 2023 AnotherlandServer
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

use async_trait::async_trait;
use bson::{Document, doc};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::{Uuid, ParamClassContainer};

use crate::util::AnotherlandResult;

use super::DatabaseRecord;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub id: i64,
    pub guid: Uuid,
    pub zone_guid: Uuid,
    pub class: i64,
    pub content_guid: Uuid,
    pub editor_name: String,
    pub data: Option<ParamClassContainer>,
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

impl Instance {
    pub async fn load_for_zone(db: Database, zone: &Uuid) -> AnotherlandResult<Vec<Instance>> {
        let mut rows = Vec::new();

        let mut result = Self::collection(db).find(doc!{"zone_guid": {"$eq": zone.to_string()}}, None).await?;
        while let Some(row) = result.try_next().await? {
            rows.push(row);
        }

        Ok(rows)
    }
}