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

use atlas::{NonClientBaseComponent, PlayerClass, PlayerParams};
use bevy_ecs::{query::{With, Without}, system::{Query, Res}};
use tokio::runtime::Handle;
use tokio_util::task::TaskTracker;

use crate::actors::{zone::{components::{AvatarComponent, AvatarEvent, InterestList, Position}, resources::Tasks}, AvatarEventServer, Spawned};

pub fn update_interests(
    mut players: Query<(&AvatarComponent, &PlayerClass, &Position, &AvatarEventServer, &mut InterestList, Without<NonClientBaseComponent>)>,
    non_player_avatars: Query<(&AvatarComponent, &Position, &NonClientBaseComponent, With<Spawned>, Without<PlayerClass>)>,
    tasks: Res<Tasks>,
) {
    for (avatar, player, position, sender, mut interests, _) in players.iter_mut() {
        let mut new_interests = HashSet::new();

        // determine interests
        for (other_avatar, other_pos, base, _, _) in non_player_avatars.iter() {
            if other_avatar.id == avatar.id {
                continue;
            }

            if (position.position.distance(other_pos.position) <= player.aware_dist() || base.always_visible_to_players()) && 
                base.visible_on_quest_available().is_none() && 
                base.visible_on_quest_complete().is_none() && 
                base.visible_on_quest_finished().is_none() && 
                base.visible_on_quest_in_progress().is_none() {

                new_interests.insert(other_avatar.id.to_owned());
            }
        }

        // check for changes
        let added_interests: Vec<_> = new_interests.iter().filter(|v| !interests.interests.contains(v)).map(|v| v.to_owned()).collect();
        let removed_interests: Vec<_> = interests.interests.iter().filter(|v| !new_interests.contains(v)).map(|v| v.to_owned()).collect();

        // send updates
        if !added_interests.is_empty() || !removed_interests.is_empty() {
            interests.interests = new_interests.into_iter().collect();

            let sender = sender.sender.clone();
            let _guard = tasks.handle.enter();

            tasks.tasks.spawn(async move {
                if !added_interests.is_empty() {
                    let _ = sender.send(AvatarEvent::InterestAdded { ids: added_interests }).await;
                }

                if !removed_interests.is_empty() {
                    let _ = sender.send(AvatarEvent::InterestRemoved { ids: removed_interests }).await;
                }
            });

            drop(_guard);
        }
    }
}
