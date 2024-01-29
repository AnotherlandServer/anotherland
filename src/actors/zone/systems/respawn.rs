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

use specs::{Entities, Join, System, WriteStorage};

use crate::actors::{Spawned, SpawnerState};

pub struct RespawnEntities;

impl<'a> System<'a> for RespawnEntities {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Spawned>,
        WriteStorage<'a, SpawnerState>,
        WriteStorage<'a, NonClientBaseComponent>,
    );

    fn run(&mut self, (
        entities, 
        mut spawned, 
        mut state, 
        base
    ): Self::SystemData) {
        let mut respawn_ents = Vec::new();
        let now = Instant::now();

        // iterate trough all despawned entities
        for (entity, base, state, _) in (&entities, &base, &mut state, !&spawned).join() {
            // first check if the entity is enabled at all
            if base.enable_in_game() {
                if let Some(instant) = state.respawn_instant {
                    // respawn when we reached the set time
                    if instant <= now {
                        // reset state
                        state.despawn_instant = None;
                        state.respawn_instant = None;

                        respawn_ents.push(entity);
                    }

                    // if no explicit instant for respaning is set, respawn immediately
                } else {
                    respawn_ents.push(entity);
                }
            }
        }

        // respawn qualified ents
        for ent in respawn_ents.into_iter() {
            let _ = spawned.insert(ent, Spawned);

            // todo: run entity specific respawn logic
        }
    }    
}
