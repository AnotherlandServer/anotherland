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

use std::{sync::Arc, time::{Duration, Instant}};

use bevy::{app::{App, Plugin, PostUpdate, Update}, ecs::{component::Component, event::{Event, EventReader, EventWriter}, query::{Changed, With}, system::Res}, math::{Quat, Vec3}, prelude::{Entity, In, Query}, utils::HashMap};
use log::debug;
use obj_params::{tags::ItemBaseTag, ContentRefList, EdnaAbility, EdnaFunction, GameObjectData, NpcOtherland};
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, oaPktCooldownUpdate, AbilityEffect, CooldownEntry, CooldownUpdate, OaPktAbilityUseAbilityType, OaPktAbilityUseEventType};
use rand::Rng;
use serde::{Deserialize, Serialize};
use toolkit::types::{AvatarId, Uuid};

use crate::{object_cache::CacheEntry, plugins::ConnectionState};

use super::{AvatarIdManager, AvatarInfo, ContentInfo, CurrentState, HealthUpdateEvent, Interests, Inventory, ItemAbilities, Movement, NetworkExtPriv, PlayerController};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);

        app.add_systems(Update, perform_abilities);
        app.add_systems(PostUpdate, send_cooldown_updates);

        app.add_event::<AbilityTriggerEvent>();
    }
}

#[derive(Debug)]
pub enum AbilityToggleMode {
    Once,
}

#[derive(Debug)]
#[allow(unused)]
pub enum AbilityKind {
    Item(Arc<CacheEntry>),
    Skill(Arc<CacheEntry>),
    Buff(Arc<CacheEntry>),
}

#[derive(Event)]
#[allow(unused)]
pub struct AbilityTriggerEvent {
    pub source: Entity,
    pub ability: Arc<CacheEntry>,
    pub kind: AbilityKind,
    pub toggle_mode: AbilityToggleMode,
    pub target: Option<Entity>,
    pub position: Vec3,
    pub rotation: Option<Quat>,
    pub prediction_id: Option<i32>,
    pub combo_stage_id: Option<i32>,
    pub effects: Vec<Effect>
}

#[derive(Debug)]
pub enum Effect {
    Targeted { target: Entity, kind: EffectType },
    TargetFactory { factory: TargetFactory, kind: EffectType },
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum EffectType {
    Damage { min: f32, max: f32 },
    Heal { min: f32, max: f32 },
}

#[derive(Component)]
#[allow(unused)]
pub struct Abilities(Vec<Arc<CacheEntry>>);

enum CooldownState {
    Ready,
    Cooldown(Instant, Duration),
}

#[derive(Component, Default)]
pub struct Cooldowns(HashMap<Uuid, (Arc<CacheEntry>, CooldownState)>);

impl Cooldowns {
    pub fn insert(&mut self, group: Arc<CacheEntry>) {
        self.0.insert(group.id, (group, CooldownState::Ready));
    }

    #[allow(unused)]
    pub fn is_ready(&self, group: Uuid) -> bool {
        self.0.get(&group).map_or_else(|| false, |(_, state)| matches!(state, CooldownState::Ready))
    }

    pub fn update(&mut self) {
        for (_, (_, state)) in self.0.iter_mut() {
            if let CooldownState::Cooldown(start, duration) = state {
                if start.elapsed() >= *duration {
                    *state = CooldownState::Ready;
                }
            }
        }
    }

    pub fn consume(&mut self, groups: &[Uuid], duration: Duration) -> bool {
        self.update();

        let states = self.0.iter_mut()
            .filter(|(group, (_, state))| {
                groups.contains(group) && matches!(state, CooldownState::Ready)
            })
            .collect::<Vec<_>>();
        
        if states.len() == groups.len() {
            for (_, (_, state)) in states {
                *state = CooldownState::Cooldown(Instant::now(), duration);
            }

            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize)]
enum EffectorSettings {
    Damage {
        #[serde(default)]
        delay: f32,
        #[serde(default)]
        children: EffectorChildren,
        #[serde(default)]
        aoe_target_cap: Option<f32>,
        #[serde(default)]
        aoe_coefficient: Option<f32>,
        #[serde(default)]
        target_factory: Option<TargetFactory>,
    }
}

#[derive(Serialize, Deserialize, Default)]
struct EffectorChildren {
    buff: Option<EffectorBuff>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EffectorBuff {
    buff_name: String,
    buff_duration: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetFactory {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    settings: TargetFactorySettings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct TargetFactorySettings {
    radius_max: Option<f32>,
    angle: Option<f32>,
    target_self: bool,
    affect_enemies: bool,
}

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    avatar_man: Res<AvatarIdManager>,
    players: Query<(&Movement, &Inventory)>,
    items: Query<(&ContentInfo, &GameObjectData, &ItemAbilities), With<ItemBaseTag>>,
    mut ability_trigger: EventWriter<AbilityTriggerEvent>,
) {
    if let Ok((movement, inventory)) = players.get(ent) {
        // Lookup the ability, either directly or via provided item
        let ability_info = if let Some(item_id) = pkt.item_id {
            if 
                let Some(item_ent) = inventory.items.get(&item_id) &&
                let Ok((item_info, _, abilities)) = items.get(*item_ent)
            {
                let activation_type = match pkt.toggle_mode {
                    None => "Instant",
                    Some(1) => "heldDown",
                    _ => {
                        debug!("Unknown activation type: {:?}", pkt.toggle_mode);
                        ""
                    },
                };

                abilities.iter()
                    .find(|ability| ability.data.get::<_, String>(EdnaAbility::ActivationType).unwrap() == activation_type)
                    .cloned()
                    .map(|ability| (ability, AbilityKind::Item(item_info.template.clone())))
            } else {
                debug!("Item {} not found", item_id);
                None
            }
        } else {
            todo!()
        };

        let target = pkt.params
            .and_then(|s| s.parse::<AvatarId>().ok())
            .and_then(|id| avatar_man.entity_from_avatar_id(id));

        if let Some((ability, kind)) = ability_info {
            let effect = match ability.data.get::<_, String>(EdnaAbility::EffectType).unwrap().as_str() {
                "Damage" => {
                    match &kind {
                        AbilityKind::Item(item) => {
                            EffectType::Damage { 
                                min: item.data.get::<_, f32>(EdnaFunction::WepMinDmg).cloned().unwrap_or_default(),
                                max: item.data.get::<_, f32>(EdnaFunction::WepMaxDmg).cloned().unwrap_or_default() 
                            }
                        },
                        AbilityKind::Skill(_skill) => todo!(),
                        AbilityKind::Buff(_buff) => todo!(),
                    }
                }
                _ => {
                    debug!("Unknown effect type: {}", ability.data.get::<_, String>(EdnaAbility::EffectType).unwrap());
                    return;
                }
            };  

            let mut effects = vec![];

            debug!("{:#?}", serde_json::from_value::<TargetFactory>(
                ability.data.get::<_, serde_json::Value>(EdnaAbility::TargetFactory).ok().and_then(|v| v.get("targetFactory")).cloned().unwrap_or_default()
            ));

            if let Ok(target_factory) = serde_json::from_value::<TargetFactory>(
                ability.data.get::<_, serde_json::Value>(EdnaAbility::TargetFactory).ok().and_then(|v| v.get("targetFactory")).cloned().unwrap_or_default()
                ) {

                effects.push(Effect::TargetFactory {
                    factory: target_factory,
                    kind: effect,
                });
            } else if let Some(target) = target {
                effects.push(Effect::Targeted {
                    target,
                    kind: effect,
                });
            }

            ability_trigger.send(AbilityTriggerEvent {
                source: ent,
                ability,
                kind,
                toggle_mode: AbilityToggleMode::Once,
                target,
                position: movement.position,
                rotation: pkt.target_rotation.map(|v| v.into()),
                prediction_id: if pkt.prediction_id != 0 { Some(pkt.prediction_id) } else { None },
                combo_stage_id: pkt.combo_stage_id,
                effects
            });
        }
    }
}

fn perform_abilities(
    mut events: EventReader<AbilityTriggerEvent>,
    mut sources: Query<(&AvatarInfo, &mut Cooldowns, &Movement, &Interests)>,
    targets: Query<(Entity, &AvatarInfo, &GameObjectData, &Movement)>,
    players: Query<(Entity, &PlayerController, &Interests)>,
    mut health_events: EventWriter<HealthUpdateEvent>,
) {
    for event in events.read() {
        let (source_avatar, mut cooldowns, movement, interests) = if let Ok((source_avatar, cooldowns, movement, interests)) = sources.get_mut(event.source) {
            (source_avatar, cooldowns, movement, interests)
        } else {
            continue;
        };

        // Consume cooldowns
        let cooldown = *event.ability.data.get::<_, f32>(EdnaAbility::InternalCooldown).unwrap();

        if let Ok(external_cooldowns) = event.ability.data.get::<_, ContentRefList>(EdnaAbility::ExternalCooldownsConsumed) {
            let groups = external_cooldowns.iter()
                .map(|content_ref| content_ref.id)
                .collect::<Vec<_>>();

            if !cooldowns.consume(&groups, Duration::from_secs_f32(cooldown)) {
                debug!("Cooldowns not ready");
                continue;
            }
        }

        // Produce targets
        let mut target_effects = vec![];
        let rotation = event.rotation.unwrap_or(movement.rotation);
        let forward = rotation.mul_vec3(Vec3::Z).normalize();
        
        for effect in &event.effects {
            match effect {
                Effect::Targeted { target, kind } => target_effects.push((*target, *kind)),
                Effect::TargetFactory { factory, kind } => {
                    if factory.kind == "pie" {
                        let angle = factory.settings.angle.unwrap_or(90.0);
                        let radius = factory.settings.radius_max.unwrap_or(10.0);
                    
                        for ent in interests.keys() {
                            if let Ok((ent, _, data, target_movement)) = targets.get(*ent) {
                                let size = data.get::<_, f32>(NpcOtherland::Size).copied().unwrap_or(0.0) * 100.0;
                                let direction = (target_movement.position - event.position).with_y(0.0);

                                if direction.length() - size <= radius{
                                    let angle_diff = forward.angle_between(direction.with_y(0.0).normalize());

                                    if angle_diff <= angle / 2.0 {
                                        target_effects.push((ent, *kind));
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }

        // Setup ability source
        let (source_id, ability_type) = match &event.kind {
            AbilityKind::Item(item) => {
                (item.id, OaPktAbilityUseAbilityType::Item)
            },
            AbilityKind::Skill(skill) => {
                (skill.id, OaPktAbilityUseAbilityType::Skill)
            },
            AbilityKind::Buff(buff) => {
                (buff.id, OaPktAbilityUseAbilityType::Buff)
            },
        };

        let event_type = match event.toggle_mode {
            AbilityToggleMode::Once => OaPktAbilityUseEventType::Charge,
        };

        // Apply effects
        let effects = target_effects.into_iter()
            .filter_map(|(ent, effect)| {
                match effect {
                    EffectType::Damage { min, max } => {
                        if let Ok((ent, avatar, _data, _target_movement)) = targets.get(ent) {
                            let dmg = rand::thread_rng().gen_range(min..=max).round();
                            let id = HealthUpdateEvent::damage(ent, dmg as i32)
                                .send(&mut health_events);

                            Some(AbilityEffect {
                                effect_type: 1,
                                target_actor: avatar.id,
                                flags: 72,
                                total_damage_or_heal_amount: Some(dmg),
                                delta_hp_id: Some(id),
                                ..Default::default()
                            })
                        } else {
                            None
                        }
                    },
                    EffectType::Heal { .. } => todo!(),
                }
            })
            .collect::<Vec<_>>();
        
        // Send ability use to all interested players
        for (ent, controller, interests) in players.iter() {
            if ent == event.source {
                let pkt = oaPktAbilityUse {
                    player: source_avatar.id,
                    source_avatar: source_avatar.id,
                    skill_id: event.ability.id,
                    source_id,
                    event_type,
                    ability_invoke_location: event.position.into(),
                    ability_type,
                    server_event_duration: *event.ability.data
                        .get::<_, f32>(EdnaAbility::ExecutionTime).unwrap(),
                    flag: if event.rotation.is_some() { 2 } else { 0 },
                    rotation: event.rotation.map(|v| v.into()).clone(),
                    effect_count: effects.len() as u32,
                    effects: effects.clone(),
                    prediction_id: event.prediction_id.unwrap_or_default(),
                    combo_stage_id: event.combo_stage_id.unwrap_or_default(),
                    ..Default::default()
                };

                controller.send_packet(pkt);
            } else if interests.contains_key(&event.source) {

            }
        }
    }
}

fn send_cooldown_updates(
    players: Query<(&PlayerController, &mut Cooldowns, &CurrentState), Changed<Cooldowns>>,
) {
    for (controller, cooldowns, state) in players.iter() {
        if state.state < ConnectionState::PlayerLoaded {
            continue;
        }

        controller.send_packet(oaPktCooldownUpdate {
            avatar_id: controller.avatar_id(),
            field_2: CooldownUpdate {
                entry_count: cooldowns.0.len() as u32,
                entries: cooldowns.0.iter().map(|(_, (cooldown, state))| {
                    match state {
                        CooldownState::Ready => {
                            CooldownEntry {
                                key: cooldown.numeric_id,
                                field_1: true,
                                total_duration: 0.0,
                                remaining_duration: 0.0,
                            }
                        },
                        CooldownState::Cooldown(start, duration) => {
                            let elapsed = start.elapsed().as_secs_f32();
                            let remaining = duration.as_secs_f32() - elapsed;

                            CooldownEntry {
                                key: cooldown.numeric_id,
                                field_1: false,
                                total_duration: duration.as_secs_f32(),
                                remaining_duration: remaining,
                            }
                        }
                    }
                })
                .collect()
            },
            ..Default::default()
        });
    }
}