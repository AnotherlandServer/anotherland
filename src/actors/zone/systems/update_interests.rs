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

use std::collections::HashSet;

use atlas::{NonClientBaseComponent, NonClientBase, PlayerComponent, Player, AvatarId};
use glam::Vec3;
use legion::{system, Query, SystemBuilder, world::SubWorld, Entity};
use tokio::runtime::Handle;
use tokio_util::task::TaskTracker;

use crate::actors::zone::components::{InterestList, Position, AvatarComponent, EntityType, InterestEvent};

#[system]
pub fn update_interests(
    world: &mut SubWorld,
    players: &mut Query<(&AvatarComponent, &mut InterestList, &Position, &PlayerComponent)>, 
    avatars: &mut Query<(&EntityType, &AvatarComponent, &Position)>,
    #[resource] handle: &Handle,
    #[resource] tasks: &TaskTracker) 
{
    let mut avatar_positions = Vec::new();

    // collect avatar positions
    avatars.for_each(world, |(entity_type, avatar, position)| {
        avatar_positions.push((avatar.id.clone(), position.position));
    });

    // check for avatars in range
    players.for_each_mut(world, |(avatar, interests, position, player)| {
        let awareness_range = *player.aware_dist().unwrap_or(&0f32);
        let mut new_interests = HashSet::new();

        // determine interests
        for (other_avatar, other_pos) in avatar_positions.iter() {
            if *other_avatar == avatar.id {
                continue;
            }

            if position.position.distance(*other_pos) <= awareness_range {
                new_interests.insert(other_avatar.to_owned());
            }
        }

        // check for changes
        let added_interests: Vec<_> = new_interests.iter().filter(|v| !interests.interests.contains(v)).map(|v| v.to_owned()).collect();
        let removed_interests: Vec<_> = interests.interests.iter().filter(|v| !new_interests.contains(v)).map(|v| v.to_owned()).collect();

        // send updates
        if !added_interests.is_empty() || !removed_interests.is_empty() {
            interests.interests = new_interests.into_iter().collect();

            let sender = interests.update_sender.clone();
            let _guard = handle.enter();

            tasks.spawn(async move {
                if !added_interests.is_empty() {
                    let _ = sender.send(InterestEvent::InterestAdded { ids: added_interests }).await;
                }

                if !removed_interests.is_empty() {
                    let _ = sender.send(InterestEvent::InterestRemoved { ids: removed_interests }).await;
                }
            });

            drop(_guard);
        }

    });
}