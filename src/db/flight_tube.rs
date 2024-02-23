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

use atlas::Uuid;
use bson::{doc, Document};
use glam::Vec3;
use mongodb::{Collection, Database};
use poem::async_trait;
use serde::{Deserialize, Serialize};

use super::DatabaseRecord;

#[derive(Clone, Serialize, Deserialize)]
pub struct FlightTube {
    pub id: Uuid,
    pub name: String,
    pub travel_target: Uuid,
    pub surfing_speed: f32,
    pub location: Vec3,
    pub control_points: Vec<ControlPoint>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlPoint {
    pub point: Vec3,
    pub tangent: Vec3,
}

#[async_trait]
impl DatabaseRecord<'_> for FlightTube {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("flight_tubes")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "id": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}