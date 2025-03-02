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

use std::{sync::atomic::AtomicI32, time::Duration};

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::event::{Event, EventReader, EventWriter}, prelude::{Added, App, Changed, Commands, Component, Entity, In, IntoSystemConfigs, Mut, Or, Query, With}, time::common_conditions::on_timer};
use obj_params::{tags::{EdnaContainerTag, EdnaReceptorTag, NpcBaseTag, NpcOtherlandTag, PlayerTag, SpawnerTag, StructureTag, VehicleBaseTag}, GameObjectData, Player};
use protocol::{oaPkt_Combat_HpUpdate, CPktTargetRequest};

use super::{spawn_init_entity, AvatarInfo, Interests, NetworkExtPriv, PlayerController};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
        app.add_systems(PreUpdate, init_health.after(spawn_init_entity));
        app.add_systems(Update, (
            process_health_events,
            store_health.after(process_health_events),
            update_energy.run_if(on_timer(Duration::from_secs(1))),
        ));

        app.add_event::<HealthUpdateEvent>();
    }
}

static LAST_HEALTH_UPDATE_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Event)]
pub struct HealthUpdateEvent {
    entity: Entity,
    id: i32,
    update: HealthUpdateType,
}

pub enum HealthUpdateType {
    Damage(i32),
    Heal(i32),
    Kill,
    Revive(Option<i32>),
}

impl HealthUpdateEvent {
    fn next_id() -> i32 {
        loop {
            // Avoid id 0 if LAST_HEALTH_UPDATE_ID wraps around
            let id = LAST_HEALTH_UPDATE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if id != 0 {
                break id;
            }
        }
    }

    #[allow(dead_code)]
    pub fn damage(entity: Entity, amount: i32) -> Self {
        Self { 
            entity, 
            id: Self::next_id(), 
            update: HealthUpdateType::Damage(amount.max(0)),
        }
    }

    #[allow(dead_code)]
    pub fn heal(entity: Entity, amount: i32) -> Self {
        Self { 
            entity, 
            id: Self::next_id(), 
            update: HealthUpdateType::Heal(amount.max(0)),
        }
    }

    pub fn kill(entity: Entity) -> Self {
        Self { 
            entity, 
            id: Self::next_id(), 
            update: HealthUpdateType::Kill,
        }
    }

    pub fn revive(entity: Entity, hitpoints: Option<i32>) -> Self {
        Self { 
            entity, 
            id: Self::next_id(), 
            update: HealthUpdateType::Revive(hitpoints),
        }
    }

    #[allow(dead_code)]
    pub fn send(self, writer: &mut EventWriter<Self>) -> i32 {
        let id = self.id;
        writer.send(self);
        id
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
    min: i32,
    max: i32,
    current: i32,
    alive: bool,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Energy {
    pub min: i32,
    pub max: i32,
    pub current: i32,
}

#[allow(clippy::type_complexity)]
fn init_health(
    query: Query<(Entity, &GameObjectData), Or<(
        Added<PlayerTag>, 
        Added<VehicleBaseTag>,
        Added<StructureTag>,
        Added<SpawnerTag>,
        Added<EdnaContainerTag>,
        Added<EdnaReceptorTag>,
        Added<NpcOtherlandTag>,
    )>>,
    mut commands: Commands,
) {
    for (ent, obj) in query.iter() {
        if 
            let Ok(&min) = obj.get_named::<i32>("hpMin") &&
            let Ok(&max) = obj.get_named::<i32>("hpMax") &&
            let Ok(&current) = obj.get_named_or_default::<i32>("hpCur", &max) && 
            let Ok(&alive) = obj.get_named_or_default::<bool>("alive", &true)
        {
            commands.entity(ent)
                .insert(Health { min, max, current, alive });
        }
    }
}

fn store_health(
    mut query: Query<(&mut GameObjectData, &Health), Changed<Health>>,
) {
    for (mut obj, health) in query.iter_mut() {
        obj.set_named("hpCur", health.current);
        obj.set_named("hpMax", health.max);
        obj.set_named("hpMin", health.min);
        obj.set_named("alive", health.alive);
    }
}

#[allow(clippy::type_complexity)]
fn process_health_events(
    mut events: EventReader<HealthUpdateEvent>,
    mut target: Query<(&AvatarInfo, &mut Health), Or<(With<PlayerTag>, With<NpcBaseTag>)>>,
    receivers: Query<(&PlayerController, &Interests)>,
) {
    for event in events.read() {
        if let Ok((avatar, mut health)) = target.get_mut(event.entity) {
            // Apply update
            match event.update {
                HealthUpdateType::Damage(amount) => {
                    health.current = (health.current - amount)
                        .clamp(health.min, health.max);

                    if health.current <= health.min {
                        health.alive = false;
                    }
                },
                HealthUpdateType::Heal(amount) => {
                    if health.alive {
                        health.current = (health.current + amount)
                            .clamp(health.min, health.max);
                    }
                },
                HealthUpdateType::Kill => {
                    health.current = health.min;
                    health.alive = false;
                },
                HealthUpdateType::Revive(hitpoints) => {
                    if !health.alive {
                        health.current = hitpoints.unwrap_or(
                            (health.max - health.min) / 4 + health.min
                        ).clamp(health.min + 1, health.max);

                        health.alive = true;
                    }
                },
            }

            let pkt = oaPkt_Combat_HpUpdate {
                avatar_id: avatar.id,
                hp: health.current,
                id: event.id,
                ..Default::default()
            };

            for (controller, interests) in receivers.iter() {
                if interests.contains_key(&event.entity) || avatar.id == controller.avatar_id() {
                    controller.send_packet(pkt.clone());
                }
            }
        };
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
                regen = 1.0;
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

