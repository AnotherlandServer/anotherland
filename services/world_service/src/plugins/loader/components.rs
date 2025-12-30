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

use bevy::ecs::component::Component;
use toolkit::types::Uuid;

use crate::object_cache::CacheEntry;

#[derive(Component)]
pub struct ContentInfo {
    pub placement_id: Uuid,
    pub template: Arc<CacheEntry>,
}

#[derive(Component)]
pub struct DynamicInstance;

#[derive(Component)]
pub struct DebugPlayer;

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
