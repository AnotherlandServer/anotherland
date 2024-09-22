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

use atlas::{AvatarId, NpcOtherlandClass, NpcOtherlandParams, Param, ParamBox, PlayerComponent};
use bevy::{app::{Plugin, PostUpdate, Update}, prelude::{Changed, Entity, Mut}, utils::HashSet};
use bevy_ecs::{component::Component, query::With, schedule::IntoSystemConfigs, system::{Commands, Query}};

use crate::actors::{zone::systems::update_interests, AvatarComponent, InterestList};

#[derive(Component)]
pub struct InCombat;

pub struct CombatPlugin;

#[derive(Component, Clone)]
pub struct ThreatList(pub HashSet<Entity>);

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (
            check_threat_list,
            check_combat_status.after(update_interests),
            check_threat_list.after(update_interests)
        ));
    }
}

fn check_combat_status(
    mut query: Query<(&InterestList, Option<&InCombat>), With<PlayerComponent>>,
    mut cmds: Commands
) {
    for (interests, in_combat) in query.iter() {

    }
}

fn check_threat_list(
    mut query: Query<(&InterestList, &mut ThreatList), (Changed<InterestList>, With<PlayerComponent>)>,
    other_avatars: Query<(&AvatarComponent, &ParamBox)>,
) {
    for (interests, mut threats) in query.iter_mut() {
        for ent in &threats.0.clone() {
            if !interests.contains(*ent) {
                threats.0.remove(ent);
            }
        }

        for ent in &interests.interests {
            if 
                let Ok((avatar, params)) = other_avatars.get(*ent) &&
                let Some(Param::ContentRefList(content)) = params.get_param("Faction") &&
                (
                    content.contains("20633549-7dd4-48db-ad9f-af76c800329d") ||
                    content.contains("2dbcbd04-8c86-4e63-8b95-3f6ca9de9a27") ||
                    content.contains("508aa064-4fae-4e65-805e-880075f4015a") ||
                    content.contains("542e76c3-ff64-4aeb-948d-603a49e86450") ||
                    content.contains("76aabf8f-552d-4987-839d-1329660769f9") ||
                    content.contains("a50a0337-8fbf-4a6b-ac0e-553877ac11a1") ||
                    content.contains("bb1cdebb-54ab-43b7-8ed8-5181bfefebc9") ||
                    content.contains("f3beabe9-3bfa-41fa-b1de-b9b87125ddcb") ||
                    content.contains("1cc003b5-089d-4ad8-9bd4-a8c0b153c625") ||
                    content.contains("c01ec9ae-e3f0-49eb-876e-cc9a83da2994") ||
                    content.contains("dc5b3d2d-4242-47dc-bee6-01f4715723e1") ||
                    content.contains("e038a7ef-746c-4c14-84c5-33ce454ee9e5") ||
                    content.contains("2d99e04a-473d-4173-b993-ff5d4272411e") ||
                    content.contains("fb90a61b-a6f6-40a4-b3ce-411d4dc5c982") ||
                    content.contains("5c4d468d-5851-470c-9548-8029f98eeefc") ||
                    content.contains("9fe03dc6-82ef-4052-9cec-fd54951153b9") ||
                    content.contains("fa54497a-c69a-4ad4-9624-58e969c241f3") ||
                    content.contains("37efcb08-cf2c-4aa4-b961-a7da520d28b5")
                )
            {
                threats.0.insert(*ent);
            } else if threats.0.contains(ent) {
                threats.0.remove(ent);
            }
        }
    }
}