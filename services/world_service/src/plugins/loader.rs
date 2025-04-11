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

use std::{sync::Arc, time::Instant};

use bevy::{app::{First, Plugin, PreUpdate}, ecs::{component::Component, event::EventWriter, query::{Or, With}, schedule::IntoSystemConfigs, system::{In, Resource}}, prelude::{Added, Commands, Entity, NextState, Query, ResMut}, utils::HashMap};
use futures_util::TryStreamExt;
use log::{debug, info, trace, warn};
use obj_params::{tag_gameobject_entity, tags::{NpcBaseTag, StructureBaseTag}, GameObjectData, NonClientBase};
use realm_api::ObjectPlacement;
use toolkit::types::Uuid;

use crate::{instance::{InstanceState, ZoneInstance}, object_cache::CacheEntry};

use super::{AvatarIdManager, AvatarInfo, FutureTaskComponent, HealthUpdateEvent};

#[derive(Component)]
pub struct ContentInfo {
    pub placement_id: Uuid,
    pub template: Arc<CacheEntry>,
}

#[derive(Default, Resource)]
pub struct InstanceManager(HashMap<Uuid, Entity>);

impl InstanceManager {
    #[allow(dead_code)]
    pub fn find_instance(&self, id: Uuid) -> Option<Entity> {
        self.0.get(&id).cloned()
    }
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        //let (content_sender, content_receiver) = tokio::sync::mpsc::channel::<Content>(100);

        //app.insert_resource(ForeignResource(content_receiver));
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

        app.add_systems(First, init_gameobjects);
        app.add_systems(PreUpdate, (
            update_spawn_state,
            spawn_init_entity
        ).chain());

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let realm_api = instance.realm_api.clone();
        let zone = instance.zone.clone();
        let object_cache = instance.object_cache.clone();

        let init_task = FutureTaskComponent::new(
            async move {
                // Query
                let mut query = realm_api.query_object_placements()
                    .zone_guid(*zone.guid())
                    .query().await.unwrap();
        
                let mut content = vec![];
                
                while let Some(placement) = query.try_next().await.unwrap() {
                    if let Some(template) = object_cache.get_object_by_guid(placement.content_guid).await.unwrap() {
                        content.push((placement, template));
                    } else {
                        warn!("Template '{}' not found for placement '{}'", placement.content_guid, placement.id);
                    }
                }
        
                info!("Instance {} load completed.", zone.guid());
                content
            }, 
            app.world_mut().register_system(ingest_content)
        );

        app.world_mut().spawn(init_task);
    }
}

fn ingest_content(
    In(content): In<Vec<(ObjectPlacement, Arc<CacheEntry>)>>,
    mut next_state: ResMut<NextState<InstanceState>>,
    mut avatar_manager: ResMut<AvatarIdManager>,
    mut commands: Commands,
) {
    for (mut placement, template) in content {
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
            SpawnState::default(),
        )).id();

        entry.insert(entity);
    }

    next_state.set(InstanceState::Initializing);
}

pub fn init_gameobjects(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        tag_gameobject_entity(obj, &mut commands.entity(ent));
    }
}

#[allow(clippy::type_complexity)]
pub fn update_spawn_state(
    mut entities: Query<(Entity, &GameObjectData, &mut SpawnState), Or<(With<NpcBaseTag>, With<StructureBaseTag>)>>,
    mut health_events: EventWriter<HealthUpdateEvent>,
    mut commands: Commands,
) {
    for (ent, obj, mut state) in entities.iter_mut() {
        match *state {
            SpawnState::Alive => {
                if !*obj.get_named::<bool>("alive").unwrap() {
                    debug!("Entity {} is dead", ent);
                    state.mark_killed();
                }
            }
            SpawnState::Killed(instant) => {
                let despawn_delay = *obj.get::<_, f32>(NonClientBase::DespawnDelay).unwrap();

                if instant.elapsed().as_secs_f32() >= despawn_delay {
                    debug!("Despawning entity {}", ent);

                    state.mark_despawned();
                    commands.entity(ent).remove::<Active>();
                }
            },
            SpawnState::Despawned(instant) => {
                let despawn_delay = *obj.get::<_, f32>(NonClientBase::DespawnDelay).unwrap();

                if instant.elapsed().as_secs_f32() >= despawn_delay {
                    debug!("Respawning entity {}", ent);

                    state.mark_alive();
                    health_events.send(HealthUpdateEvent::revive(ent, None));
                    commands.entity(ent).insert(Active);
                }
            },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn spawn_init_entity(
    mut entities: Query<&mut GameObjectData, Or<(Added<NpcBaseTag>, Added<StructureBaseTag>)>>,
) {
    for mut obj in entities.iter_mut() {
        let hp_mod = *obj.get_named::<f32>("HpMod").unwrap_or(&1.0);
        let hp_max = *obj.get_named::<i32>("hpMax").unwrap();
        let bonus_hp = *obj.get_named::<f32>("BonusHP").unwrap_or(&0.0);

        obj.set_named("alive", true);
        obj.set_named("hpMax", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);
        obj.set_named("hpCur", (hp_max as f32 * hp_mod + bonus_hp).round() as i32);
    }
}

#[derive(Component)]
pub struct Active;

#[derive(Component, Default, Clone, Copy)]
pub enum SpawnState {
    #[default]
    Alive,
    Killed(Instant),
    Despawned(Instant),
}

impl SpawnState {
    pub fn mark_killed(&mut self) {
        *self = SpawnState::Killed(Instant::now());
    }

    pub fn mark_despawned(&mut self) {
        *self = SpawnState::Despawned(Instant::now());
    }

    pub fn mark_alive(&mut self) {
        *self = SpawnState::Alive;
    }
}