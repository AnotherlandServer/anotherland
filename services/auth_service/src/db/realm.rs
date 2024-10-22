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

use std::net::SocketAddr;

use bson::doc;
use database::{DBResult, DatabaseRecord};
use mongodb::{options::IndexOptions, Database, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub population: f32,
    pub endpoint: Option<SocketAddr>,
}

impl DatabaseRecord<'_> for Realm {
    type PrimaryKey = i32;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }
    
    fn collection_name() -> &'static str {
        "realms"
    }

    async fn build_index(db: &Database) -> DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}