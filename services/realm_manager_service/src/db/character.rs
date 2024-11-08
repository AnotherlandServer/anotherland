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

use cynic::serde::{Deserialize, Serialize};
use database::DatabaseRecord;
use mongodb::bson::Uuid;
use obj_params::GameObjectData;

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub account: Uuid,
    pub index: i32,
    pub name: String,
    pub data: GameObjectData,
}

impl DatabaseRecord<'_> for Character {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn collection_name() -> &'static str {
        "characters"
    }
}