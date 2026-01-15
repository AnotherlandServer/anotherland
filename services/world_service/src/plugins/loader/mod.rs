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

mod components;
mod events;
mod resources;
mod systems;
mod commands_component_loader;
mod component_loader;
mod cache;

use bevy::{app::{First, Last, Plugin, PostStartup, PreUpdate, Update}, ecs::{lifecycle::HookContext, schedule::IntoScheduleConfigs, system::Commands}};
pub use components::*;
pub use events::*;
pub use resources::*;
pub use systems::*;
pub use commands_component_loader::*;
pub use component_loader::*;
pub use cache::*;

use crate::{instance::ZoneInstance, plugins::{CommandExtPriv, ZoneLoader, ZoneLoaderParameter, navigation}};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(InstanceManager::default());
        app.world_mut().register_component_hooks::<ContentInfo>()
            .on_insert(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.insert(id, entity);
            })
            .on_remove(|mut world, HookContext { entity, .. }| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.remove(&id);
            });

        app.add_systems(First, (
            process_loading_components,
            init_gameobjects
        ).chain());
        app.add_systems(PreUpdate, (
            update_spawn_state,
            spawn_init_entity
        ).chain());
        app.add_systems(Update, (
            sync_debug_pos.after(navigation::update),
            avatar_despawner,
        ));
        app.add_systems(Last, cleanup_dynamic_instances);

        app.add_message::<DespawnAvatar>();

        app.register_command("get_avatar_info", command_get_avatar_info);

        insert_loader_api(app.world_mut()).expect("Failed to insert loader API");

        app.init_resource::<LoadingComponents>();

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let zone = instance.zone.clone();
        app.add_systems(PostStartup, move |mut commands: Commands| {
            commands
                .spawn_empty()
                .load_component::<ZoneLoader>(ZoneLoaderParameter {
                    zone: zone.clone()
                });
        });
    }
}