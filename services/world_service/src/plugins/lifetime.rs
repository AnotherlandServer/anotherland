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

use std::time::Instant;

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{component::Component, entity::Entity, query::{Added, Changed}, system::{Commands, Query}}};
use obj_params::GameObjectData;

use crate::plugins::{DespawnAvatar, SpawnState};

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, create_lifetime_trackers);
        app.add_systems(Update, check_lifetime);
    }
}

#[derive(Component)]
pub struct LifetimeTracker {
    pub created: Instant,
}

fn create_lifetime_trackers(
    query: Query<(Entity, &GameObjectData, &SpawnState), Changed<SpawnState>>,
    mut commands: Commands,
) {
    for (entity, obj, state) in query.iter() {
        if 
            matches!(state, SpawnState::Alive) &&  
            let Ok(lifetime) = obj.get_named::<f32>("aliveTime") &&
            *lifetime > 0.0
        {
            commands.entity(entity)
                .insert(LifetimeTracker { created: Instant::now() });
        }
    }
}

fn check_lifetime(
    query: Query<(Entity, &LifetimeTracker, &GameObjectData, &SpawnState)>,
    mut commands: Commands,
) {
    for (entity, tracker, obj, state) in query.iter() {
        if 
            matches!(state, SpawnState::Alive) &&
            let Ok(lifetime) = obj.get_named::<f32>("aliveTime") &&
            tracker.created.elapsed().as_secs_f32() > *lifetime
        {
            commands.send_event(DespawnAvatar(entity));
        }
    }
}