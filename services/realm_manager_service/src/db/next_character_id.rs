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

use database::{DBResult, DatabaseError, DatabaseRecord};
use mongodb::{bson::{doc, Uuid}, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NextCharacterId {
    account_id: Uuid,
    next_character_id: i32,
}

impl NextCharacterId {
    pub async fn get_next_id(db: &Database, account_id: &Uuid) -> DBResult<i32> {
        let collection = Self::collection(db);
        let res = collection.find_one_and_update(doc!{
                "account_id": account_id
            }, doc!{
                "$inc": { "next_character_id": 1 }
            })
            .upsert(true)
            .await?
            .unwrap();

        Ok(res.next_character_id)
    }
}

impl DatabaseRecord<'_> for NextCharacterId {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.account_id
    }

    fn collection_name() -> &'static str {
        "next_character_ids"
    }
}