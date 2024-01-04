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
use bson::doc;
use glam::{Vec3, Vec4};
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use serde::Serialize;
use serde_derive::Deserialize;
use bson::Uuid;

use crate::util::AnotherlandResult;

use super::{Account, DatabaseRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentUuid {
    NoBinding(Uuid),
    Buff(Uuid),
    Drop(Uuid),
    Enemie(Uuid),
    Faction(Uuid),
    Item(Uuid),
    Metagame(Uuid),
    Misc(Uuid),
    Npc(Uuid),
    Projectile(Uuid),
    Quest(Uuid),
    Recipe(Uuid),
    Skill(Uuid),
    Spawner(Uuid),
    Structure(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPlacement {
    pub id: Uuid,
    pub world_id: u16,
    pub position: Vec3,
    pub rotation: Vec4,
    pub content_guid: ContentUuid,
}

impl ContentPlacement {
    pub async fn create(db: Database, world_id: u16, position: Vec3, rotation: Vec4, content_guid: ContentUuid) -> AnotherlandResult<ContentPlacement> {
        let placement = ContentPlacement {
            id: uuid::Uuid::new_v4().into(),
            world_id,
            position,
            rotation,
            content_guid,
        };

        let collection = db.collection::<ContentPlacement>("content_placements");
        collection.insert_one(&placement, None).await?;

        Ok(placement)
    }

    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = db.collection::<Account>("content_placements");
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("world_id": 1))
            .options(IndexOptions::builder().build())
            .build(), 
            None).await?;

        Ok(())
    }
}

#[async_trait]
impl DatabaseRecord<'_> for ContentPlacement {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("content_placements")
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}