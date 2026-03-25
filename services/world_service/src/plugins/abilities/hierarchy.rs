// Copyright (C) 2026 AnotherlandServer
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

use bevy::ecs::{component::Component, entity::Entity};
use realm_api::State;
use toolkit::types::Uuid;

#[derive(Component)]
#[relationship(relationship_target = Abilities)]
pub struct AbilityOf {
    #[relationship]
    pub owner: Entity,
    kind: AbilityType,
}

impl AbilityOf {
    pub fn new(owner: Entity, kind: AbilityType) -> Self {
        Self { owner, kind }
    }

    pub fn owner(&self) -> Entity { self.owner }
    pub fn kind(&self) -> &AbilityType { &self.kind }
}

#[derive(Default)]
pub enum AbilityType {
    #[default]
    Ability,
    ClassSkill { 
        id: Uuid,
        group: String,
        state: State,
        stance: i32,
    },
    ItemAbility
}

#[derive(Component)]
#[relationship_target(relationship = AbilityOf)]
pub struct Abilities(Vec<Entity>);