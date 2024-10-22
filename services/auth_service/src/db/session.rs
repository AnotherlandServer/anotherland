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

use bson::{doc, Uuid};
use chrono::{DateTime, Utc};
use database::{DBResult, DatabaseRecord};
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions};
use serde::{Deserialize, Serialize};

use super::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub account: Uuid,
    pub is_gm: bool,
    pub realm_id: Option<u32>,
    pub world_id: Option<u16>,
    pub zone_guid: Option<Uuid>,
    pub character_id: Option<u32>,
    pub created: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl Session {
    pub async fn create(db: &Database, account: &Account) -> DBResult<Session> {
        debug!("Creating session for {}", account.unique_name());

        let session = Session {
            id: Uuid::new(),
            account: account.id,
            is_gm: account.is_gm,
            realm_id: None,
            world_id: None,
            zone_guid: None,
            character_id: None,
            created: Utc::now(),
            last_seen: Utc::now(),
        };

        let collection = db.collection::<Session>("sessions");
        collection.insert_one(&session).await?;

        debug!("Session created session for {}", account.unique_name());

        Ok(session)
    }

    pub async fn select_realm(&mut self, db: &Database, realm_id: u32) -> DBResult<()> {
        self.realm_id = Some(realm_id);

        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": {"realm_id": realm_id}},
        ).await?;

        Ok(())
    }

    pub async fn select_world(&mut self, db: &Database, world_id: u16) -> DBResult<()> {
        self.world_id = Some(world_id);

        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": {"world_id": world_id as u32}},
        ).await?;

        Ok(())
    }

    pub async fn select_character(&mut self, db: &Database, character_id: u32) -> DBResult<()> {
        self.character_id = Some(character_id);

        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": {"character_id": character_id}},
        ).await?;

        Ok(())
    }

    pub async fn select_zone(&mut self, db: &Database, zone_guid: Uuid) -> DBResult<()> {
        self.zone_guid = Some(zone_guid);

        let collection = Self::collection(db);

        collection.update_one(
            Self::query_one(self.key()), 
            doc!{"$set": {"zone_guid": zone_guid}},
        ).await?;

        Ok(())
    }
}

impl DatabaseRecord<'_> for Session {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }
    
    fn collection_name() -> &'static str {
        "sessions"
    }

    async fn build_index(db: &Database) -> DBResult<()> {
        let collection = db.collection::<Session>("sessions");
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build()
        ).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("account": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build()
        ).await?;

        Ok(())
    }
}