use std::ops::{Deref, DerefMut};

use bson::{doc, Document};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::{Uuid, ParamClassContainer};

use crate::util::AnotherlandResult;

use super::DatabaseRecord;

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub guid: Uuid,
    pub name: String,
    pub class: u16,
    pub data: Option<ParamClassContainer>,
}

// buffs
#[derive(Serialize, Deserialize)]
pub struct BuffContent(Content);

impl Deref for BuffContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BuffContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for BuffContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("buffs")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

// drops
#[derive(Serialize, Deserialize)]
pub struct DropsContent(Content);

impl Deref for DropsContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DropsContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for DropsContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("drops")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

// factions
#[derive(Serialize, Deserialize)]
pub struct FactionContent(Content);

impl Deref for FactionContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FactionContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for FactionContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("factions")
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }
}

// items
#[derive(Serialize, Deserialize)]
pub struct ItemContent(Content);

impl Deref for ItemContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for ItemContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("items")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

// items
#[derive(Serialize, Deserialize)]
pub struct NpcContent(Content);

impl Deref for NpcContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NpcContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for NpcContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("npcs")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

// spawners
#[derive(Serialize, Deserialize)]
pub struct SpawnerContent(Content);

impl Deref for SpawnerContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpawnerContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for SpawnerContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("spawners")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

// structures
#[derive(Serialize, Deserialize)]
pub struct StructureContent(Content);

impl Deref for StructureContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StructureContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for StructureContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("structures")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}
