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

use std::ops::{Deref, DerefMut};

use bson::{doc, Document};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::{ParamClassContainer, OaBuff2Param, LootScatterContainerParam, FactionParam, NpcOtherlandParam, SpawnerParam, StructureParam, BoundParamClass, ParamEntity, ParamError, Uuid};

use crate::util::AnotherlandResult;

use super::DatabaseRecord;

#[derive(Serialize, Deserialize, Clone)]
pub struct Content {
    pub id: i64,
    pub guid: Uuid,
    pub name: String,
    pub class: u16,
    pub data: Option<ParamClassContainer>,
}

// buffs
#[derive(Serialize, Deserialize, Clone)]
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

impl BuffContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// drops
#[derive(Serialize, Deserialize, Clone)]
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

impl DropsContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// factions
#[derive(Serialize, Deserialize, Clone)]
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

impl FactionContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// items
#[derive(Serialize, Deserialize, Clone)]
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

impl ItemContent {
    pub async fn list_by_categories(db: Database, categories: &[Uuid]) -> AnotherlandResult<Vec<Self>> {
        let collection = Self::collection(db);
        let mut items = Vec::new();

        let string_categories: Vec<_> = categories.iter().map(|v| v).collect();

        let mut result = collection.find(doc!{"data.ednaModule.Category.v": {"$in":string_categories}}, None).await?;
        while let Some(item) = result.try_next().await? {
            items.push(item);
        }

        Ok(items)
    }
}

// items
#[derive(Serialize, Deserialize, Clone)]
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

impl NpcContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// spawners
#[derive(Serialize, Deserialize, Clone)]
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

impl SpawnerContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// structures
#[derive(Serialize, Deserialize, Clone)]
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

impl StructureContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }
}

// misc
#[derive(Serialize, Deserialize, Clone)]
pub struct MiscContent(Content);

impl Deref for MiscContent {
    type Target = Content;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MiscContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseRecord<'_> for MiscContent {
    type Key = Uuid;

    fn collection(db:Database) -> Collection<Self>  {
        db.collection::<Self>("misc")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) ->  &Self::Key {
        &self.guid
    }
}

impl MiscContent {
    pub fn into_param<T>(self) -> Option<T> 
        where
            T: TryFrom<ParamClassContainer, Error = ParamError>
    {
        self.0.data.map(|v| v.try_into().unwrap())
    }

    pub async fn get_by_name(db: Database, name: &str) -> AnotherlandResult<Option<Self>> {
        Ok(Self::collection(db).find_one(doc! { "name": mongodb::bson::Regex {
            pattern: format!("^{}$", name),
            options: "i".to_string(),
        }}, None).await?)
    }
}