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

use atlas::{NpcOtherlandParams, ParamBox, PlayerClass, PlayerParams};
use bevy::app::{First, Plugin, PostUpdate};
use bevy_ecs::{change_detection::DetectChangesMut, component::Component, entity::Entity, event::{Event, EventReader}, query::{Changed, With}, schedule::IntoSystemConfigs, system::{Commands, Query}};

#[derive(Event)]
pub struct DamageEvent(pub Entity, pub i32);

#[derive(Event)]
pub struct HealEvent(pub Entity, pub i32);

#[derive(Event)]
pub struct ReviveEvent(pub Entity);

pub struct HitPointsPlugin;

impl Plugin for HitPointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageEvent>();
        app.add_event::<HealEvent>();
        app.add_event::<ReviveEvent>();
        app.add_systems(First, pre_sync_hitpoints);
        app.add_systems(PostUpdate, (
            (
                apply_damage, 
                apply_healing,
                revive_entities
            ).before(post_sync_hitpoints),
            post_sync_hitpoints,
        ));  
    }
}

#[derive(Component)]
pub struct HitPoints {
    current: i32,
    min: i32,
    max: i32,
}

impl HitPoints {
    pub fn current(&self) -> i32 {
        self.current
    }
}

#[derive(Component)]
pub struct Alive;

fn pre_sync_hitpoints(
    mut hitpoints: Query<(Entity, &ParamBox, Option<&mut HitPoints>), Changed<ParamBox>>,
    mut cmds: Commands,
) {
    for (ent, params, hp) in hitpoints.iter_mut() {
        if let Ok(player_params) = params.get::<PlayerClass>() {
            if let Some(mut hp) = hp {
                if hp.min != player_params.hp_min() ||
                    hp.max != player_params.hp_max() {

                    hp.min = player_params.hp_min();
                    hp.max = player_params.hp_max();

                    if hp.current < hp.min {
                        hp.current = hp.min;
                    } else if hp.current > hp.max {
                        hp.current = hp.max;
                    }
                }
            } else {
                cmds.entity(ent).insert(HitPoints {
                    current: player_params.hp_cur(),
                    min: player_params.hp_min(),
                    max: player_params.hp_max(),
                });
            }
        } else if let Some(npc) = params.get_impl::<dyn NpcOtherlandParams>() {
            if let Some(mut hp) = hp {
                if hp.min != npc.hp_min() ||
                    hp.max != npc.hp_max() {

                    hp.min = npc.hp_min();
                    hp.max = npc.hp_max();

                    if hp.current < hp.min {
                        hp.current = hp.min;
                    } else if hp.current > hp.max {
                        hp.current = hp.max;
                    }
                }
            } else {
                cmds.entity(ent).insert(HitPoints {
                    current: npc.hp_cur(),
                    min: npc.hp_min(),
                    max: npc.hp_max(),
                });
            }
        }
    }
}

fn apply_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut hitpoints: Query<&mut HitPoints, With<Alive>>,
) {
    for DamageEvent(ent, dmg) in ev_damage.read() {
        if let Ok(mut hitpoints) = hitpoints.get_mut(*ent) {
            hitpoints.current = hitpoints.current.saturating_sub(*dmg);

            if hitpoints.current < hitpoints.min {
                hitpoints.current = hitpoints.min;
            } else if hitpoints.current > hitpoints.max {
                hitpoints.current = hitpoints.max;
            }
        }
    }
}

fn apply_healing(
    mut ev_healing: EventReader<HealEvent>,
    mut hitpoints: Query<&mut HitPoints, With<Alive>>,
) {
    for HealEvent(ent, heal) in ev_healing.read() {
        if let Ok(mut hitpoints) = hitpoints.get_mut(*ent) {
            hitpoints.current = hitpoints.current.saturating_sub(*heal);
            
            if hitpoints.current < hitpoints.min {
                hitpoints.current = hitpoints.min;
            } else if hitpoints.current > hitpoints.max {
                hitpoints.current = hitpoints.max;
            }
        }
    }
}

fn revive_entities(
    mut ev_revive: EventReader<ReviveEvent>,
    mut hitpoints: Query<&mut HitPoints>,
) {
    for ReviveEvent(ent) in ev_revive.read() {
        if let Ok(mut hitpoints) = hitpoints.get_mut(*ent) {
            hitpoints.current = hitpoints.max;
        }
    }
}

fn post_sync_hitpoints(
    mut hitpoints: Query<(Entity, &HitPoints, &mut ParamBox, Option<&Alive>), Changed<HitPoints>>,
    mut cmds: Commands,
) {
    for (ent, hp, mut params, alive) in hitpoints.iter_mut() {
        let alive_state_changed;

        if hp.current <= 0 && alive.is_some() {
            cmds.entity(ent).remove::<Alive>();
            alive_state_changed = true;
        } else if hp.current > 0 && alive.is_none() {
            cmds.entity(ent).insert(Alive);
            alive_state_changed = true;
        } else {
            alive_state_changed = false;
        }

        // update hp in params without triggering changes
        if let Ok(player_params) = params.bypass_change_detection().get_mut::<PlayerClass>() {
            player_params.set_hp_cur(hp.current);
        } else if let Some(npc) = params.bypass_change_detection().get_impl_mut::<dyn NpcOtherlandParams>() {
            npc.set_hp_cur(hp.current);
        }

        // update alive flag if needed, trigger change as required
        if alive_state_changed {
            if let Ok(player_params) = params.get_mut::<PlayerClass>() {
                player_params.set_alive(hp.current > 0);
            } else if let Some(npc) = params.get_impl_mut::<dyn NpcOtherlandParams>() {
                npc.set_alive(hp.current > 0);
            }
        }
    }
}
