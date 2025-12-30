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

use bevy::{app::{App, Plugin}, ecs::lifecycle::HookContext};

mod component;
mod id_manager;
mod loader;

pub use component::*;
pub use id_manager::*;
pub use loader::*;

pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvatarIdManager>();
        app.world_mut().register_component_hooks::<Avatar>()
            .on_insert(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<Avatar>().unwrap().id;
                let mut manager = world.get_resource_mut::<AvatarIdManager>().unwrap();


                manager.avatar_entry(id).or_insert(entity);
            })
            .on_remove(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<Avatar>().unwrap().id;
                let mut manager = world.get_resource_mut::<AvatarIdManager>().unwrap();

                manager.entities.remove(&id);
            });
    }
}