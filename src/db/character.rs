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

use async_trait::async_trait;
use bson::{doc, Document};
use log::debug;
use mongodb::{Database, IndexModel, options::IndexOptions, Collection};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;

use sha1::{Sha1, Digest};
use tokio_stream::StreamExt;

use crate::util::AnotherlandResult;
use atlas::{PlayerClass, Uuid, PlayerParams};

use super::{DatabaseRecord, ItemContent};

pub enum Race {
    Simuloid, // 0
    Human, // 1
    Alien, // 2
}

pub enum Gender {
    Male, // 0
    Female, // 1
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub guid: Uuid,
    pub account: Uuid,
    pub name: String,
    pub world_id: u32,
    
    pub data: PlayerClass,
}

impl Character {
    pub async fn create(db: Database, account_id: &Uuid, name: &str) -> AnotherlandResult<Character> {
        let template = Lazy::new(|| {
            let mut player = PlayerClass::default();
        
            player.set_world_map_guid("f6b8f8b7-a726-4d36-9634-f6d403943fff");
            player.set_zone_guid(Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap());
            player.set_zone("ClassSelection_P");

            player.set_tutorial_mode(true);
        
            // default customization
            player.set_current_skin("Simuloid");
            player.set_visible_item_info(vec![
                20647, // PlayerCharSkinSimuloid0001Default0004
                21190, // SkinColorSimuloid0006Default0002
                21566, // TattooFace0001Default0002
                21550, // Scars0002Default0002
                21633, // CharFaceMale0012Default0002
                21184, // EyeColor0015Default0002
                21571, // LipColor0011Default0002
                21585, // HairSkin0002Default0002
                21638, // HairColor0017Default0002
            ]);
            player.set_default_items_content_guid(vec![
                20647, // PlayerCharSkinSimuloid0001Default0004
                21190, // SkinColorSimuloid0006Default0002
                21566, // TattooFace0001Default0002
                21550, // Scars0002Default0002
                21633, // CharFaceMale0012Default0002
                21184, // EyeColor0015Default0002
                21571, // LipColor0011Default0002
                21585, // HairSkin0002Default0002
                21638, // HairColor0017Default0002
            ]);
            player.set_customization_gender(1.0);
            player.set_customization_height(0.5);
            player.set_customization_bust_size(0.5);
            player.set_customization_fat(0.0);
            player.set_customization_skinny(0.7);
            player.set_customization_muscular(0.3);
            player.set_move_speed(292.0);

            player.set_bling(0);
            player.set_game_cash(0);
        
            player
        }); 

        let guid = Uuid::new();

        // Compute numeric character id, similar to how we build account ids..
        let mut hasher = Sha1::new();
        hasher.update(guid.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());

        let mut avatar_data = template.clone();
        let default_items = ItemContent::list_by_categories(db.clone(), vec![
            Uuid::parse_str("B1F4F5F2-E3E1-46d3-8E13-BF87769F18EA").unwrap(),
            Uuid::parse_str("6B74CF2D-79A3-48B8-B752-995179A064BD").unwrap(),
            Uuid::parse_str("393B5E54-457F-41a4-A60E-7DF834FFFB13").unwrap(),
            Uuid::parse_str("557F6393-B875-4af0-86A3-6107FE1225BB").unwrap(),
        ].as_slice()).await?;

        debug!("Default item count: {}", default_items.len());

        avatar_data.set_metamorph_item_list(default_items.into_iter().map(|v| v.guid).collect());

        let character = Character {
            id: numeric_id,
            guid,
            account: *account_id,
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