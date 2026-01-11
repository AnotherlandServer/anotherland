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

use std::sync::{Arc, OnceLock};

use bevy::{app::Plugin, ecs::{component::Component, error::Result, world::World}, platform::collections::HashMap, prelude::{App, Entity, In, Query}};
use futures::{TryStreamExt, future::join_all};
use log::{debug, trace};
use mlua::{Lua, Table};
use obj_params::{Class, ContentRefList};
use protocol::{oaPktFactionRequest, oaPktFactionResponse, FactionRelation, FactionRelationList};
use realm_api::RealmApi;
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::{RwLock, RwLockWriteGuard};
use toolkit::{types::Uuid, NativeParam};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{Cache, LoadContext, LoadableComponent, StrongCache}};

use super::{NetworkExtPriv, PlayerController};

struct FactionCache;

#[derive(Hash, PartialEq, Eq, Clone)]
enum FactionRef {
    Id(Uuid),
    Name(String),
}

impl Cache for FactionCache {
    type CacheKey = FactionRef;
    type CacheData = Faction;

    async fn load(key: &Self::CacheKey) -> Result<Option<Self::CacheData>> {
        let Some(faction) = 
            (match key {
                FactionRef::Id(id) => 
                    RealmApi::get()
                        .get_object_template(*id)
                        .await?,
                FactionRef::Name(name) => 
                    RealmApi::get()
                        .query_object_templates()
                        .class(Class::Faction)
                        .name(name.to_string())
                        .query().await?
                        .try_next().await?
            })
        else {
            return Ok(None);
        };

        debug!("Loading faction {} ({})", faction.name, faction.id);

        let relations = 
            if 
                let Ok(value) = faction.data.get::<_, Value>(obj_params::Faction::Relations) && 
                value.is_array() 
            {
                serde_json::from_value::<Vec<Relation>>(
                    value.clone()
                ).unwrap()
            } else {
                vec![]
            };


        Ok(Some(
            Faction {
                id: faction.id,
                name: faction.name.clone(),
                relations,
            }
        ))
    }

    async fn post_load(data: &Arc<Self::CacheData>) -> Result<()> {
        let _ = join_all(
        data
            .relations()
            .iter()
            .map(async |relation| {
                Self::get(&FactionRef::Name(relation.faction.clone())).await
            })
        ).await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}

impl StrongCache for FactionCache {
    fn cache() -> &'static RwLock<HashMap<Self::CacheKey, Arc<Self::CacheData>>> {
        static CACHE: OnceLock<RwLock<HashMap<FactionRef, Arc<Faction>>>> = OnceLock::new();
        CACHE.get_or_init(|| RwLock::new(HashMap::new()))
    }

    async fn cache_insert(cache: &mut RwLockWriteGuard<'_, HashMap<Self::CacheKey, Arc<Self::CacheData>>>, _key: Self::CacheKey, data: &Arc<Self::CacheData>) {
        cache.insert(FactionRef::Id(data.id), data.clone());
        cache.insert(FactionRef::Name(data.name.clone()), data.clone());
    }
}

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

#[derive(Component, Default)]
pub struct Factions(Vec<(Arc<Faction>, i32)>);

pub struct FactionsParameters {
    pub factions: ContentRefList,
}

impl LoadableComponent for Factions {
    type Parameters = FactionsParameters;

    async fn load(parameters: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        let factions = 
            join_all(
                parameters.factions.iter()
                    .map(async |faction_ref| {
                        FactionCache::get(&FactionRef::Id((*faction_ref.id).into())).await
                    })
                    .collect::<Vec<_>>()
            ).await
            .into_iter()
            .collect::<Result<Vec<Option<Arc<Faction>>>>>()?
            .into_iter()
            .flatten()
            .map(|faction| (faction, 1000))
            .collect();

        Ok(Factions(factions))
    }
}

impl Factions {
    pub fn relation_to(&self, other: &Factions) -> i32 {
        for (my_faction, _) in &self.0 {
            for relation in my_faction.relations() {
                for (other_faction, _) in &other.0 {
                    if relation.faction == other_faction.name() {
                        trace!("Faction relation: {} -> {} = {}", relation.faction, other_faction.name(), relation.standing);
                        return relation.standing;
                    }
                }
            }
        }

        for (other_faction, _) in &other.0 {
            for relation in other_faction.relations() {
                for (my_faction, _) in &self.0 {
                    if relation.faction == my_faction.name() {
                        return relation.standing;
                    }
                }
            } 
        }

        0
    }

    #[allow(dead_code)]
    pub fn relations(&self) -> Vec<(Arc<Faction>, i32)> {
        self.0.clone()
    }
}

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_faction_request);

        insert_faction_api(app.world_mut()).unwrap();
    }
}

fn handle_faction_request(
    In((ent, _pkt)): In<(Entity, oaPktFactionRequest)>,
    query: Query<(&PlayerController, &Factions)>,
) {
    if let Ok((controller, factions)) = query.get(ent) {
        let factions = factions.0
            .iter()
            .map(|(faction, standing)| FactionRelation {
                id: faction.id(),
                name: faction.name().to_string(),
                standing: *standing as f32,
            })
            .collect::<Vec<_>>();

        controller.send_packet(oaPktFactionResponse {
            field_1: controller.avatar_id(),
            field_2: 1,
            field_3: NativeParam::Struct(vec![
                NativeParam::Buffer(FactionRelationList {
                    count: factions.len() as u32,
                    factions
                }.to_bytes())
            ]),
            ..Default::default()
        });
    }
}

pub fn insert_faction_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("faction", object_api.clone()).unwrap();

    object_api.set("EntityRelationship", lua.create_bevy_function(world,         |
        In((a, b)): In<(Table, Table)>,
        avatars: Query<&Factions>,
    | -> WorldResult<i32> {
        if 
            let Ok(a) = avatars.get(a.entity()?) &&
            let Ok(b) = avatars.get(b.entity()?)
        {
            Ok(a.relation_to(b))
        } else {
            Err(anyhow!("entity not found").into())
        }
    })?)?;
    
    Ok(())
}