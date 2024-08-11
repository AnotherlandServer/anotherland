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

use atlas::AvatarId;
use bevy::utils::hashbrown::HashSet;
use bevy_ecs::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct InterestList {
    pub interests: HashSet<Entity>,
}

impl InterestList {
    pub fn new() -> Self {
        Self {
            interests: HashSet::new(),
        }
    }

    pub fn contains(&self, id: Entity) -> bool {
        self.interests.contains(&id)
    }
}