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

use atlas::PlayerComponent;
use bevy::app::{Plugin, PostUpdate, Update};
use bevy_ecs::{component::Component, query::With, schedule::IntoSystemConfigs, system::{Commands, Query}};

use crate::actors::{zone::systems::update_interests, InterestList};

#[derive(Component)]
pub struct InCombat;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, check_combat_status.after(update_interests));
    }
}

fn check_combat_status(
    mut query: Query<(&InterestList, Option<&InCombat>), With<PlayerComponent>>,
    mut cmds: Commands
) {
    for (interests, in_combat) in query.iter() {

    }
}