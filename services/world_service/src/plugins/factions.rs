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

use std::{collections::HashMap, sync::Arc};

use bevy::{app::Plugin, ecs::{component::Component, world::World}, prelude::{App, Entity, In, Query}};
use log::trace;
use mlua::{Lua, Table};
use protocol::{oaPktFactionRequest, oaPktFactionResponse, FactionRelation, FactionRelationList};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{types::Uuid, NativeParam};
use anyhow::anyhow;

use crate::{error::WorldResult, factions::Faction, FACTIONS};

use super::{NetworkExtPriv, PlayerController};

#[derive(Component, Default)]
pub struct Factions(Vec<(Arc<Faction>, i32)>);

impl Factions {
    pub fn add_faction(&mut self, id: Uuid) {
        if let Some(faction) = FACTIONS.get().unwrap().get_faction(id) {
            self.0.push((faction.clone(), 1000));
        }
    }

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
        let mut relations = HashMap::new();

        for (faction, _) in &self.0 {
            for relation in faction.relations() {
                relations.insert(&relation.faction, relation.standing);
            }
        }

        relations.into_iter()
            .filter_map(|(faction_name, standing)| {
                FACTIONS.get().unwrap().get_faction_by_name(faction_name)
                    .map(|faction| {
                        (faction.clone(), standing)
                    })
            })
            .collect()
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