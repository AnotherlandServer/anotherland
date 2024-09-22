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

use atlas::{oaPktCombatUpdate, oaPktThreatListUpdate, oaPkt_Combat_HpUpdate};
use bevy::prelude::{Commands, Component};
use bevy_ecs::{entity::Entity, query::{Added, Changed}, removal_detection::RemovedComponents, system::Query};
use log::debug;

use crate::actors::{zone::plugins::{HitPoints, InCombat, ThreatList}, AvatarComponent, InterestList};

use super::PlayerController;

#[derive(Component)]
pub struct PrevThreatList(ThreatList);

pub fn send_hitpoint_updates(
    hitpoints: Query<(Entity, &AvatarComponent, &HitPoints), Changed<HitPoints>>,
    players: Query<(&AvatarComponent, &InterestList, &PlayerController)>,
) {
    for (entity, avatar, hp) in hitpoints.iter() {
        // check player interest list to dispatch updates
        for (player_avatar, interests, controller) in players.iter() {
            if interests.contains(entity) || avatar.id == player_avatar.id {
                debug!("Send hp update: {}-{}", avatar.id, hp.current());

                controller.send_message(oaPkt_Combat_HpUpdate {
                    avatar_id: avatar.id,
                    hp: hp.current(),
                    ..Default::default()
                }.into_message());
            }
        }
    }
}

pub fn send_threat_list(
    mut players: Query<(Entity, &PlayerController, &AvatarComponent, &ThreatList, Option<&mut PrevThreatList>), Changed<ThreatList>>,
    avatars: Query<&AvatarComponent>,
    mut cmds: Commands,
) {
    for (ent, controller, player_avatar, threats, prev_threats) in players.iter_mut() {
        if prev_threats.is_none() {
            for threat in &threats.0 {
                if let Ok(avatar) = avatars.get(*threat) {
                    debug!("Send threat to player {}: {:?}", player_avatar.name, avatar.id);

                    controller.send_message(oaPktThreatListUpdate {
                        field_1: player_avatar.id,
                        field_2: true,
                        field_3: avatar.id,
                        ..Default::default()
                    }.into_message());
                }
            }

            cmds.entity(ent).insert(PrevThreatList(threats.clone()));
        } else if let Some(mut prev_threats) = prev_threats {
            // send new threats
            for threat in &threats.0 {
                if 
                    !prev_threats.0.0.contains(threat) && 
                    let Ok(avatar) = avatars.get(*threat) 
                {
                    debug!("Send threat to player {}: {:?}", player_avatar.name, avatar.id);

                    controller.send_message(oaPktThreatListUpdate {
                        field_1: player_avatar.id,
                        field_2: true,
                        field_3: avatar.id,
                        ..Default::default()
                    }.into_message());
                }
            }

            // remove ald threats
            for threat in &prev_threats.0.0 {
                if 
                    !threats.0.contains(threat) &&
                    let Ok(avatar) = avatars.get(*threat) 
                {
                    debug!("Remove threat from player {}: {:?}", player_avatar.name, avatar.id);

                    controller.send_message(oaPktThreatListUpdate {
                        field_1: player_avatar.id,
                        field_2: false,
                        field_3: avatar.id,
                        ..Default::default()
                    }.into_message());
                }
            }

            prev_threats.0 = threats.clone();
        }
    }
}

pub fn toggle_on_combat(
    avatars: Query<(Entity, &AvatarComponent), Added<InCombat>>,
    players: Query<(&AvatarComponent, &PlayerController, &InterestList)>,
) {
    for (entity, avatar) in avatars.iter() {
        for (player, controller, interests) in players.iter() {
            if interests.contains(entity) || player.id == avatar.id {
                controller.send_message(
                    oaPktCombatUpdate {
                        field_1: avatar.id,
                        field_2: atlas::OaPktCombatUpdateField2::ToggleOnCombat,
                        ..Default::default()
                    }.into_message()
                )
            }
        }
    }
}

pub fn toggle_off_combat(
    mut removals: RemovedComponents<InCombat>,
    avatars: Query<&AvatarComponent>,
    players: Query<(&AvatarComponent, &PlayerController, &InterestList)>,
) {
    for entity in removals.read() {
        if let Ok(avatar) = avatars.get(entity) {
            for (player, controller, interests) in players.iter() {
                if interests.contains(entity) || player.id == avatar.id {
                    controller.send_message(
                        oaPktCombatUpdate {
                            field_1: avatar.id,
                            field_2: atlas::OaPktCombatUpdateField2::ToggleOffCombat,
                            ..Default::default()
                        }.into_message()
                    )
                }
            }
        }
    }
}
