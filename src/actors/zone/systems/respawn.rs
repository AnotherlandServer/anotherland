// Copyright (C) 2023 AnotherlandServer
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

use atlas::NonClientBaseComponent;
use bevy_ecs::{entity::Entity, query::Without, system::{Commands, Query}};

use crate::actors::{Spawned, SpawnerState};

pub fn respawn(mut commands: Commands, mut query: Query<(Entity, &NonClientBaseComponent, &mut SpawnerState, Without<Spawned>)>) {
    let now = Instant::now();

    for (entity, base, mut state, _) in query.iter_mut() {
        // first check if the entity is enabled at all
        if base.enable_in_game() {
            if let Some(instant) = state.respawn_instant {
                // respawn when we reached the set time
                if instant <= now {
                    // reset state
                    state.despawn_instant = None;
                    state.respawn_instant = None;

                    commands.entity(entity)
                        .insert(Spawned);
                }

                // if no explicit instant for respaning is set, respawn immediately
            } else {
                commands.entity(entity)
                        .insert(Spawned);
            }
        }

        
    }
}