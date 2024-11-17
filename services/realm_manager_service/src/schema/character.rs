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

use async_graphql::{futures_util::TryStreamExt, Context, Error, InputObject, Object, SimpleObject};
use database::DatabaseRecord;
use mongodb::{bson::{doc, Uuid}, Database};
use obj_params::{GameObjectData, Player, Value};

use crate::db;

#[derive(Default)]
pub struct CharacterRoot;

#[derive(Default)]
pub struct CharacterMutationRoot;

#[Object]
impl CharacterRoot {
    async fn character(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Character>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = db::Character::get(&db, &id).await?;

        if let Some(character) = res {
            Ok(Some(Character::from_db(character)?))
        } else {
            Ok(None)
        }
    }

    async fn account_character(&self, ctx: &Context<'_>, account_id: Uuid, index: i32) -> Result<Option<Character>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = db::Character::collection(&db).find_one(doc! {
            "$and": [
                { "account": account_id },
                { "index": index }
            ]
        }).await?;

        if let Some(character) = res {
            Ok(Some(Character::from_db(character)?))
        } else {
            Ok(None)
        }
    }

    async fn characters_for_account(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<Vec<Character>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut res = db::Character::collection(&db)
            .find(doc! {"account": account_id})
            .sort(doc! {"index": 1})
            .await?;
        let mut characters = Vec::new();

        while let Some(character) = res.try_next().await? {
            characters.push(Character::from_db(character)?);
        }

        Ok(characters)
    }
}

#[Object]
impl CharacterMutationRoot {
    async fn create_character(&self, ctx: &Context<'_>, input: CreateCharacterInput) -> Result<Character, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut cursor = db::Character::collection(&db).aggregate(vec![
            doc! {
                "$match": { "account": input.account }
            },
            doc! {
                "$sort": { "index": -1 }
            },
            doc! {
                "$limit": 1
            }
        ]).with_type::<db::Character>().await?;

        let next_index = if cursor.advance().await? {
            let character = cursor.deserialize_current()?;
            character.index + 1
        } else {
            1
        };

        let mut data = GameObjectData::new::<Player>();
        data.set(Player::WorldMapGuid, "f6b8f8b7-a726-4d36-9634-f6d403943fff");
        data.set(Player::ZoneGuid, Uuid::parse_str("4635f288-ec24-4e73-b75c-958f2607a30e").unwrap());
        data.set(Player::Zone, "ClassSelection_P");
        data.set(Player::TutorialMode, true);
        data.set(Player::CurrentSkin, "Simuloid");
        data.set(Player::VisibleItemInfo, vec![
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
        data.set(Player::DefaultItemsContentGuid, vec![
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
        data.set(Player::CustomizationGender, 1.0);
        data.set(Player::CustomizationHeight, 0.5);
        data.set(Player::CustomizationBustSize, 0.5);
        data.set(Player::CustomizationFat, 0.0);
        data.set(Player::CustomizationSkinny, 0.7);
        data.set(Player::CustomizationMuscular, 0.3);
        data.set(Player::MoveSpeed, 292.0);

        let character = db::Character::create(&db, db::Character {
            id: Uuid::new(),
            account: input.account,
            index: next_index,
            name: input.name,
            data,
        }).await?;

        Character::from_db(character)
    }

    async fn delete_character(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Character>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = db::Character::get(&db, &id).await?;

        if let Some(character) = res {
            character.delete(&db).await?;
            Ok(Some(Character::from_db(character)?))
        } else {
            Ok(None)
        }
    }
}

#[derive(InputObject)]
pub struct CreateCharacterInput {
    account: Uuid,
    name: String,
}

#[derive(SimpleObject)]
pub struct Character {
    id: Uuid,
    account: Uuid,
    index: i32,
    name: String,
    data: serde_json::Value
}

impl Character {
    pub fn from_db(character: db::Character) -> Result<Self, Error> {
        Ok(Self {
            id: character.id,
            account: character.account,
            index: character.index,
            name: character.name,
            data: serde_json::to_value(character.data)?,
        })
    }
}