// Copyright (C) 2024 AnotherlandServer
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

use atlas::{NonClientBaseComponent, PlayerClass, PlayerParams, SpawnerClass};
use bevy_ecs::{entity::Entity, event::EventWriter, query::{With, Without}, system::Query};

use crate::actors::{zone::{components::{AvatarComponent, AvatarEvent, InterestList, Position}, zone_events::AvatarEventFired}, Spawned};

#[allow(clippy::type_complexity)]
pub fn update_interests(
    mut players: Query<(Entity, &PlayerClass, &mut InterestList)>,
    positioned: Query<(Entity, &AvatarComponent, &Position, Option<&NonClientBaseComponent>), (With<Spawned>, Without<SpawnerClass>)>,
    mut ev_avatar_event: EventWriter<AvatarEventFired>,
) {
    for (entity, player, mut interests) in players.iter_mut() {
        let mut new_interests = HashSet::new();

        let (_, _, position, _) = positioned.get(entity).unwrap();

        // determine interests
        for (other_ent, other_avatar, other_pos, base) in positioned.iter() {
            // skip over self
            if other_ent == entity { continue; }

            if let Some(base) = base {
                if (position.position.distance(other_pos.position) <= player.aware_dist() || base.always_visible_to_players()) && 
                    base.visible_on_quest_available().is_none() && 
                    base.visible_on_quest_complete().is_none() && 
                    base.visible_on_quest_finished().is_none() && 
                    base.visible_on_quest_in_progress().is_none()
                {
                    new_interests.insert(other_avatar.id.to_owned());
                }
            } else if position.position.distance(other_pos.position) <= player.aware_dist() {
                new_interests.insert(other_avatar.id.to_owned());
            }
        }

        // check for changes
        let added_interests: Vec<_> = new_interests.iter().filter(|v| !interests.interests.contains(v)).cloned().collect();
        let removed_interests: Vec<_> = interests.interests.iter().filter(|v| !new_interests.contains(v)).cloned().collect();

        if !added_interests.is_empty() {
            ev_avatar_event.send(AvatarEventFired(entity, AvatarEvent::InterestAdded { ids: added_interests }));
        }

        if !removed_interests.is_empty() {
            ev_avatar_event.send(AvatarEventFired(entity, AvatarEvent::InterestRemoved { ids: removed_interests }));
        }

        // remember interests
        interests.interests = new_interests.iter().cloned().collect();
    }
}
