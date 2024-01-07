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
use bson::{doc, Document};
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use serde_with::serde_as;
use sha1::{Sha1, Digest};
use tokio_stream::StreamExt;

use crate::util::AnotherlandResult;
use atlas::{PlayerParam, Player, Uuid};

use super::{DatabaseRecord, ItemContent};

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub guid: Uuid,
    pub account: Uuid,
    pub name: String,
    pub world_id: u32,
    
    pub data: PlayerParam,
}

static NEW_CHARACTER_TEMPLATE: Lazy<PlayerParam> = Lazy::new(|| {
    let mut player = PlayerParam::default();

    player.set_world_map_guid("f6b8f8b7-a726-4d36-9634-f6d403943fff");
    player.set_zone_guid(Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap());
    player.set_zone("ClassSelection_P");

    // default customization
    player.set_visible_item_info(vec![3840]);
    player.set_customization_gender(1.0);
    player.set_customization_height(0.5);
    player.set_customization_bust_size(0.5);
    player.set_customization_fat(0.0);
    player.set_customization_skinny(0.7);
    player.set_customization_muscular(0.3);

    player
}); 

impl Character {
    pub async fn create(db: Database, account_id: &Uuid, name: &str) -> AnotherlandResult<Character> {
        let guid = Uuid::new();

        // Compute numeric character id, similar to how we build account ids..
        let mut hasher = Sha1::new();
        hasher.update(guid.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());

        let mut avatar_data = NEW_CHARACTER_TEMPLATE.clone();
        let default_items = ItemContent::list_by_categories(db.clone(), vec![
            Uuid::parse_str("6B74CF2D-79A3-48B8-B752-995179A064BD").unwrap().into()
        ].as_slice()).await?;

        debug!("Default item count: {}", default_items.len());

        avatar_data.set_default_items_content_guid(default_items.into_iter().map(|v| v.id as i32).collect());

        let character = Character {
            id: numeric_id,
            guid: guid,
            account: account_id.clone(),
            name: name.to_owned(),
            world_id: 130,
            data: avatar_data,
        };

        let collection = Character::collection(db.clone());
        collection.insert_one(&character, None).await?;

        Ok(character)
    }

    pub async fn list(db: Database, account_id: &Uuid) -> AnotherlandResult<Vec<Character>> {
        let collection = Character::collection(db.clone());
        let mut chracters = Vec::new();

        let mut result = collection.find(doc!{"account": {"$eq":account_id}}, None).await?;
        while let Some(character) = result.try_next().await? {
            chracters.push(character);
        }

        Ok(chracters)
    }

    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = Character::collection(db.clone());
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("guid": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("name": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(), 
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("account": 1))
            .options(IndexOptions::builder().build())
            .build(), 
            None).await?;

        Ok(())
    }

    pub async fn save(&mut self, db: Database) -> AnotherlandResult<()> {
        let collection = db.collection::<Character>("characters");
        collection.update_one(doc!{"id": {"$eq": self.id}}, doc!{"$set": &bson::to_bson(self).unwrap().as_document()}, None).await?;

        Ok(())
    }
}

#[async_trait]
impl DatabaseRecord<'_> for Character {
    type Key = u32;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("characters")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}