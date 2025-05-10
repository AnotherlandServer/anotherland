// Copyright (C) 2025 AnotherlandServer
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

use std::sync::Arc;

use bevy::platform::collections::HashMap;
use futures::TryStreamExt;
use obj_params::Class;
use realm_api::RealmApi;
use serde::Deserialize;
use serde_json::Value;
use toolkit::types::Uuid;

use crate::error::WorldResult;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relation {
    pub faction: String,
    pub standing: i32,
}

pub struct Faction {
    id: Uuid,
    name: String,
    relations: Vec<Relation>,
}

impl Faction {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_standing(&self, other: &Faction) -> i32 {
        for relation in &self.relations {
            if relation.faction == other.name {
                return relation.standing;
            }
        }

        0
    }

    pub fn relations(&self) -> &[Relation] {
        &self.relations
    }
}

pub struct FactionManager(Arc<HashMap<Uuid, Arc<Faction>>>);

impl FactionManager {
    pub async fn new(realm_api: RealmApi) -> WorldResult<Self> {
        let mut result = realm_api
            .query_object_templates()
            .class(Class::Faction)
            .query()
            .await?;

        let mut factions = HashMap::new();

        while let Some(faction) = result.try_next().await? {
            let relations = if let Ok(value) = faction.data.get::<_, Value>(obj_params::Faction::Relations) && value.is_array() {
                serde_json::from_value::<Vec<Relation>>(
                    value.clone()
                ).unwrap()
            } else {
                vec![]
            };

            factions.insert(faction.id, Arc::new(Faction {
                id: faction.id,
                name: faction.name.clone(),
                relations,
            }));
        }

        Ok(Self(Arc::new(factions)))
    }

    pub fn get_faction(&self, id: Uuid) -> Option<Arc<Faction>> {
        self.0.get(&id).cloned()
    }

    pub fn get_faction_by_name(&self, name: &str) -> Option<Arc<Faction>> {
        self.0.values().find(|f| f.name == name).cloned()
    }
}