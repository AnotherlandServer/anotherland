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

use bevy::{app::Plugin, ecs::component::HookContext, platform::{collections::{hash_map::Entry, HashMap}, hash::FixedHasher}, prelude::{App, Component, Entity, Resource}};
use rand::{thread_rng, Rng};
use toolkit::types::{AvatarId, AvatarType};

#[derive(Component)]
pub struct AvatarInfo {
    pub id: AvatarId,
    pub name: String,
}

pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvatarIdManager>();
        app.world_mut().register_component_hooks::<AvatarInfo>()
            .on_insert(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<AvatarInfo>().unwrap().id;
                let mut manager = world.get_resource_mut::<AvatarIdManager>().unwrap();


                manager.avatar_entry(id).or_insert(entity);
            })
            .on_remove(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<AvatarInfo>().unwrap().id;
                let mut manager = world.get_resource_mut::<AvatarIdManager>().unwrap();

                manager.entities.remove(&id);
            });
    }
}

#[derive(Resource)]
pub struct AvatarIdManager {
    entities: HashMap<AvatarId, Entity>,
}

impl AvatarIdManager {
    pub fn new_avatar_entry(&mut self) -> Entry<'_, AvatarId, Entity, FixedHasher> {
        let mut rng = thread_rng();
        let id = loop {
            let id = AvatarId::new(rng.gen_range(1..1<<56) << 0xF, AvatarType::Npc);
            if !self.entities.contains_key(&id) {
                break id;
            }
        };

        self.entities.entry(id)
    }

    pub fn avatar_entry(&mut self, id: AvatarId) -> Entry<'_, AvatarId, Entity, FixedHasher> {
        self.entities.entry(id)
    }

    pub fn entity_from_avatar_id(&self, id: AvatarId) -> Option<Entity> {
        self.entities.get(&id).copied()
    }
}

impl Default for AvatarIdManager {
    fn default() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }
}