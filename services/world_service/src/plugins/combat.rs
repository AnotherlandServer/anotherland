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

use std::time::Duration;

use bevy::{app::{Plugin, PreUpdate, Update}, prelude::{Added, App, Changed, Commands, Component, Entity, In, IntoSystemConfigs, Mut, Or, Query, With}, time::common_conditions::on_timer};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase, Player};
use protocol::{oaPkt_Combat_HpUpdate, CPktTargetRequest};

use super::{AvatarInfo, Interests, NetworkExtPriv, PlayerController};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
        app.add_systems(PreUpdate, init_health);
        app.add_systems(Update, (
            sync_health,
            update_energy.run_if(on_timer(Duration::from_secs(1))),
        ));
    }
}

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, CPktTargetRequest)>,
    mut player: Query<&mut GameObjectData>,
) {
    if let Ok(mut player) = player.get_mut(ent) {
        player.set(Player::Target,  pkt.target_avatar_id);
    }
}

#[derive(Component)]
pub struct Health {
    pub min: i32,
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Energy {
    pub min: i32,
    pub max: i32,
    pub current: i32,
}

fn init_health(
    query: Query<(Entity, &GameObjectData), Or<(Added<PlayerTag>, Added<NonClientBaseTag>)>>,
    mut commands: Commands,
) {
    for (ent, obj) in query.iter() {
        if 
            let Ok(&current) = obj.get_named::<i32>("hpCur") &&
            let Ok(&min) = obj.get_named::<i32>("hpMin") &&
            let Ok(&max) = obj.get_named::<i32>("hpMax")
        {
            commands.entity(ent)
                .insert(Health { min, max, current });
        }
    }
}

fn sync_health(
    query: Query<(Entity, &AvatarInfo, &Health), Changed<Health>>,
    players: Query<(&PlayerController, &Interests)>,
) {
    for (ent, avatar, health) in query.iter() {
        for (controller, interests) in players.iter() {
            if interests.contains(&ent) || avatar.id == controller.avatar_id() {
                controller.send_packet(oaPkt_Combat_HpUpdate {
                    avatar_id: avatar.id,
                    hp: health.current,
                    ..Default::default()
                });
            }
        }
    }
}

fn update_energy(
    mut query: Query<&mut GameObjectData, With<PlayerTag>>,
) {
    fn regenerate_energy(obj: &mut Mut<'_, GameObjectData>, attr: Player, max: f32) {
        let current = *obj.get::<_, f32>(attr).unwrap();
        if current < max {
            let mut regen = *obj.get::<_, f32>(Player::AttributeEnergyRegen).unwrap();
            if regen == 0.0 {
                regen = 0.1;
            }

            let new = (current + regen).min(max);
            obj.set(attr, new);
        }
    }

    for mut obj in query.iter_mut() {
        let energy_max = *obj.get::<_, f32>(Player::AttributeEnergyMax).unwrap();
        
        regenerate_energy(&mut obj, Player::StatEnergyCurrentH1, energy_max);
        regenerate_energy(&mut obj, Player::StatEnergyCurrentH2, energy_max);
        regenerate_energy(&mut obj, Player::StatEnergyCurrentH3, energy_max);
        regenerate_energy(&mut obj, Player::StatEnergyCurrentS1, energy_max);
        regenerate_energy(&mut obj, Player::StatEnergyCurrentS2, energy_max);
        regenerate_energy(&mut obj, Player::StatEnergyCurrentS3, energy_max);
    }
}

