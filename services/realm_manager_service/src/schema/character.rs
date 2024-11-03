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
use obj_params::{GameObjectData, Player};
use serde_json::Value;

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

    async fn characters_for_account(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<Vec<Character>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut res = db::Character::collection(&db).find(doc! {"account": account_id}).await?;
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
        let character = db::Character::create(&db, db::Character {
            id: Uuid::new(),
            account: input.account,
            index: db::NextCharacterId::get_next_id(&db, &input.account).await?,
            name: input.name,
            data: GameObjectData::new::<Player>()
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
    data: Value
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