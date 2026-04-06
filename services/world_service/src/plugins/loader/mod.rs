// Copyright (C) 2026 AnotherlandServer
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

use bevy::{app::{First, Last, Plugin, PreUpdate, Update}, ecs::{entity::Entity, lifecycle::HookContext, query::With, schedule::IntoScheduleConfigs, system::{Commands, In, Query}}, state::state::OnEnter};
pub use components::*;
pub use events::*;
use obj_params::{Class, GameObjectData, NonClientBase, tags::StructureTag};
pub use resources::*;
pub use systems::*;
pub use commands_component_loader::*;
pub use component_loader::*;
pub use cache::*;
use toolkit::{NativeParam, OtherlandQuatExt, types::Uuid};

use crate::{instance::{InstanceState, ZoneInstance}, plugins::{Avatar, CommandExtPriv, Movement, NonPlayerGameObjectLoader, NonPlayerGameObjectLoaderParams, ZoneLoader, ZoneLoaderParameter, navigation}};

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

        app.add_systems(First, process_loading_components);
        app.add_systems(PreUpdate, (
            update_spawn_state,
        ).chain());
        app.add_systems(Update, sync_debug_pos.after(navigation::update));
        app.add_systems(Last, cleanup_dynamic_instances);

        app.register_command("get_avatar_info", command_get_avatar_info);
        app.register_command("show_hidden_structures", |
            _: In<(Entity, Vec<NativeParam>)>,
            query: Query<(Entity, &Avatar, &Movement), With<StructureTag>>,
            mut commands: Commands,
        | {
            for (ent, avatar, movement) in query.iter() {
                let mut data = GameObjectData::new_for_class(Class::NpcOtherland);

                data.set(NonClientBase::Pos, movement.position);
                data.set(NonClientBase::Rot, movement.rotation.as_unit_vector());

                commands
                    .spawn(DebugNpc)
                    .load_component::<NonPlayerGameObjectLoader>(NonPlayerGameObjectLoaderParams::Dynamic { 
                        id: Uuid::new(), 
                        owner: Some(ent), 
                        name: avatar.name.clone(), 
                        template: super::ContentCacheRef::Name("Quest_8_MG_11_Pylon".to_owned()), 
                        data, 
                        callback: None,
                    });
            }
        });

        app.add_observer(spawn_init_entity);
        app.add_observer(avatar_despawner);
        app.add_observer(on_remove_object);

        insert_loader_api(app);

        app.init_resource::<LoadingComponents>();

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let zone = instance.zone.clone();
        app.add_systems(OnEnter(InstanceState::ObjectLoad), move |mut commands: Commands| {
            commands
                .spawn_empty()
                .load_component::<ZoneLoader>(ZoneLoaderParameter {
                    zone: zone.clone()
                });
        });
    }
}