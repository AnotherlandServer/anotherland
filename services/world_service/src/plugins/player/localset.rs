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

use bevy::{ecs::{component::Component, entity::Entity, lifecycle::RemovedComponents, system::Query}, platform::collections::HashMap};
use obj_params::{GenericParamSet, tags::PlayerTag};

#[derive(Component, Default)]
pub struct PlayerLocalSets(pub HashMap<Entity, Box<dyn GenericParamSet>>);

pub fn cleanup_local_sets(
    mut removed: RemovedComponents<PlayerTag>,
    mut query: Query<(Entity, &mut PlayerLocalSets)>,
) {
    for ent in removed.read() {
        if let Ok((_, mut changes)) = query.get_mut(ent) {
            changes.0.remove(&ent);
        }
    }
}