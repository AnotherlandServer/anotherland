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

use bevy::{app::{First, Plugin, Startup, Update}, math::Vec3, prelude::{in_state, Added, Commands, Component, Entity, IntoSystemConfigs, NextState, NonSend, NonSendMut, Query, Res, ResMut, Resource}, utils::HashMap};
use futures_util::TryStreamExt;
use log::{debug, error, info, trace, warn};
use obj_params::{tag_gameobject_entity, Class, GameObjectData, NonClientBase};
use realm_api::{ObjectPlacement, ObjectTemplate, RealmApi};
use scripting::{LuaRuntime, ScriptCommandsExt, Scripted};
use tokio::sync::mpsc::{Receiver, Sender};
use toolkit::types::Uuid;

use crate::{instance::{InstanceState, ZoneInstance}, object_cache::CacheEntry};

use super::{AvatarIdManager, AvatarInfo, ForeignResource};

struct Content(Option<(ObjectPlacement, Arc<CacheEntry>)>);

#[derive(Component)]
pub struct ContentInfo {
    pub placement_id: Uuid,
    pub template: Arc<CacheEntry>,
}

#[derive(Default, Resource)]
pub struct InstanceManager(HashMap<Uuid, Entity>);

impl InstanceManager {
    pub fn find_instance(&self, id: Uuid) -> Option<Entity> {
        self.0.get(&id).cloned()
    }
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (content_sender, content_receiver) = tokio::sync::mpsc::channel::<Content>(100);

        app.insert_resource(ForeignResource(content_receiver));
        app.insert_resource(InstanceManager::default());
        app.world_mut().register_component_hooks::<ContentInfo>()
            .on_insert(|mut world, entity, _| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.insert(id, entity);
            })
            .on_remove(|mut world, entity, _| {
                let id = world.get_entity(entity).unwrap()
                    .get::<ContentInfo>().unwrap().placement_id;
                let mut manager = world.get_resource_mut::<InstanceManager>().unwrap();

                manager.0.remove(&id);
            });

        app.add_systems(First, (
                ingest_content.run_if(in_state(InstanceState::Loading)),
                init_gameobjects,
            ).chain());

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let realm_api = instance.realm_api.clone();
        let zone = instance.zone.clone();
        let object_cache = instance.object_cache.clone();

        instance.spawn_task(async move {
            // Query
            let mut query = realm_api.query_object_placements()
                .zone_guid(*zone.guid())
                .query().await.unwrap();
    
            
            while let Some(placement) = query.try_next().await.unwrap() {
                if let Some(template) = object_cache.get_object_by_guid(placement.content_guid).await.unwrap() {
                    if content_sender.send(Content(Some((placement, template)))).await.is_err() {
                        return;
                    }
                } else {
                    warn!("Template '{}' not found for placement '{}'", placement.content_guid, placement.id);
                }
            }
    
            info!("Instance {} load completed.", zone.guid());
            let _ = content_sender.send(Content(None)).await;
        });
    }
}

fn ingest_content(
    mut receiver: ResMut<ForeignResource<Receiver<Content>>>,
    mut next_state: ResMut<NextState<InstanceState>>,
    mut avatar_manager: ResMut<AvatarIdManager>,
    mut commands: Commands,
) {
    while let Ok(Content(content)) = receiver.try_recv() {
        if let Some((mut placement, template)) = content {
           
            placement.data.set_parent(Some(template.data.clone()));
            
            // Skip disabled objects
            if !*placement.data.get::<_, bool>(NonClientBase::EnableInGame).unwrap_or(&false) {
                trace!("Skipping {}", placement.id);
                continue;
            } else {
                trace!("Spawning {}", placement.id);
            }

            let entry = avatar_manager.new_avatar_entry();

            let entity = commands.spawn((
                AvatarInfo {
                    id: *entry.key(),
                    name: placement.editor_name,
                },
                ContentInfo {
                    placement_id: placement.id,
                    template,
                },
                placement.data,
                Active,
            )).id();

            entry.insert(entity);
        } else {
            debug!("Done receiving");
            next_state.set(InstanceState::Initializing);
        }
    }
}

pub fn init_gameobjects(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        tag_gameobject_entity(obj, &mut commands.entity(ent));
    }
}

#[derive(Component)]
pub struct Active;