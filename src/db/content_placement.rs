use async_trait::async_trait;
use bson::doc;
use chrono::{DateTime, Utc};
use glam::{Vec3, Vec4};
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection, bson::oid::ObjectId};
use serde::Serialize;
use serde_derive::Deserialize;

use crate::{util::AnotherlandResult};
use atlas::Uuid;

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
            id: Uuid::new_v4(),
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