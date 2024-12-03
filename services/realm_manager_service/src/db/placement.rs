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

use database::DatabaseRecord;
use obj_params::GameObjectData;
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "placement")]
pub struct Placement {
    id: Uuid,
    zone_guid: Uuid,
    class: i64,
    content_guid: Uuid,
    editor_name: String,
    #[graphql_crud(serialize_as = serde_json::Value)]
    data: GameObjectData,
    phase_tag: String,
}

impl DatabaseRecord for Placement {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "placements"
    }
}