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

use bevy::{app::{App, Plugin, PostUpdate, Update}, ecs::{component::Component, event::{Event, EventReader, EventWriter}, query::{Changed, With}, system::{Commands, Res, ResMut}, world::World}, math::{Quat, Vec3}, prelude::{Entity, In, Query}, utils::HashMap};
use log::{debug, error};
use mlua::{Lua, LuaSerdeExt, Table, Value};
use obj_params::{tags::{ItemBaseTag, PlayerTag}, ContentRefList, EdnaAbility, EdnaFunction, GameObjectData, NpcOtherland};
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, oaPktCooldownUpdate, AbilityEffect, CooldownEntry, CooldownUpdate, OaPktAbilityUseAbilityType, OaPktAbilityUseEventType};
use rand::Rng;
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use serde::{Deserialize, Serialize};
use toolkit::{types::{AvatarId, Uuid, UUID_NIL}, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::{error::WorldResult, object_cache::CacheEntry, plugins::ConnectionState};

use super::{interests, AvatarIdManager, AvatarInfo, CachedObject, ContentInfo, CurrentState, HealthUpdateEvent, Interests, Inventory, ItemAbilities, Movement, NetworkExtPriv, PlayerController, SkillbookEntry};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);

        //app.add_systems(Update, perform_abilities);
        app.add_systems(PostUpdate, send_cooldown_updates);

        app.add_event::<AbilityTriggerEvent>();

        insert_cooldown_api(app.world_mut()).unwrap();
        insert_ability_api(app.world_mut()).unwrap();
    }
}

#[derive(Debug)]
pub enum AbilityToggleMode {
    Once,
}

#[derive(Debug)]
#[allow(unused)]
pub enum AbilityKind {
    Item(Entity, CachedObject),
    Skill(SkillbookEntry),
}

#[derive(Event)]
#[allow(unused)]
pub struct AbilityTriggerEvent {
    pub source: Entity,
    pub ability: CachedObject,
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


fn insert_cooldown_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let cooldown_api = lua.create_table().unwrap();
    runtime.register_native("cooldown", cooldown_api.clone()).unwrap();

    cooldown_api.set("Consume", lua.create_bevy_function(world, 
        |
            In((player, groups, duration)): In<(Table, Vec<String>, f32)>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<bool> {
            let mut cooldowns = query.get_mut(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let groups = groups.into_iter()
                .map(|s| s.parse::<Uuid>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(cooldowns.consume(&groups, Duration::from_secs_f32(duration)))
        })?)?;

    Ok(())
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
    lua_objects: Query<&ScriptObject>,
    runtime: Res<LuaRuntime>,
    avatar_man: Res<AvatarIdManager>,
    mut commands: Commands,
    /*players: Query<(&Movement, &Inventory)>,
    items: Query<(&ContentInfo, &GameObjectData, &ItemAbilities), With<ItemBaseTag>>,
    mut ability_trigger: EventWriter<AbilityTriggerEvent>,*/
) {
    if let Ok(lua_player) = lua_objects.get(ent) {
        let request = runtime.vm().create_table().unwrap();

        let target = pkt.params
            .and_then(|s| s.parse::<AvatarId>().ok())
            .and_then(|id| avatar_man.entity_from_avatar_id(id))
            .and_then(|ent| lua_objects.get(ent).ok())
            .map(|obj| obj.object().clone());

        request.set("target", target).unwrap();
        request.set("ability_id", pkt.ability_id.to_string()).unwrap();
        request.set("reference_id", pkt.item_id.map(|v| v.to_string())).unwrap();
        request.set("prediction_id", pkt.prediction_id).unwrap();
        request.set("toggle_mode", pkt.toggle_mode).unwrap();
        request.set("combo_stage_id", pkt.combo_stage_id).unwrap();
        request.set("target_rotation", 
            pkt.target_rotation
                    .map(|v| QuatWrapper(v.into()))
        ).unwrap();

        commands.entity(ent)
            .fire_lua_event("OnAbilityRequest", request);
    }
    /*if let Ok((movement, inventory)) = players.get(ent) {
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
    }*/
}

/*fn perform_abilities(
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
}*/

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

fn insert_ability_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let ability_api = lua.create_table().unwrap();
    runtime.register_native("ability", ability_api.clone()).unwrap();

    ability_api.set("Invoke", lua.create_bevy_function(world, 
        |
            In(params): In<Table>,
            players: Query<(Entity, &PlayerController, &Interests)>,
            content: Query<&ContentInfo>,
            targets: Query<&AvatarInfo>,
        | -> WorldResult<()> {
            let ability = if let Ok(skill) = params.get::<Table>("ability")?.get::<SkillbookEntry>("__skill") {
                skill.ability.clone()
            } else if let Ok(ability) = params.get::<Table>("ability")?.get::<CachedObject>("__item_ability") {
                (*ability).clone()
            } else {
                return Err(anyhow!("invalid ability").into());
            };

            let source_id = if let Ok(item) = params.get::<Table>("item")?.entity() {
                if let Ok(content_info) = content.get(item) {
                    content_info.template.id
                } else {
                    return Err(anyhow!("item not found").into());
                }
            } else {
                UUID_NIL
            };

            let source_ent = params.get::<Table>("source")?.entity()?;
            let source = targets.get(source_ent)
                .map_err(|_| anyhow!("source not found"))?;
            let effects = params.get::<Table>("effects")?;
            let prediction_id = params.get::<i32>("prediction_id")?;
            let combo_stage_id = params.get::<i32>("combo_stage_id")?;
            let rotation = params.get::<QuatWrapper>("rotation").ok().map(|v| v.0);
            let ability_invoke_location = params.get::<Vec3Wrapper>("position")?.0;
            let event_duration = params.get::<f32>("event_duration")?;
            let ability_type = params.get::<i32>("ability_type")?;
            let event_type = params.get::<i32>("event_type")?;

            let effects = effects.sequence_values()
                .flatten()
                .map(|effect: Table| -> WorldResult<AbilityEffect> {
                    let target = effect.get::<Table>("target")?.entity()?;
                    let effect_type = effect.get::<i32>("type")?;
                    let total_damage_or_heal_amount = effect.get::<f32>("amount").ok();
                    let delta_hp_id = effect.get::<i32>("delta_hp_id").ok();
                    let effect_delay = effect.get::<f32>("delay").ok();
                    let effect_duration_from_server = effect.get::<f32>("effect_duration").ok();
                    let combat_flags = effect.get::<i32>("combat_flags").ok();

                    let avatar = targets.get(target)
                        .map_err(|_| anyhow!("target not found"))?;

                    Ok(AbilityEffect {
                        target_actor: avatar.id,
                        effect_type,
                        flags: 
                            if combat_flags.is_some() { 0x4 } else { 0x0 } |
                            if total_damage_or_heal_amount.is_some() { 0x8 } else { 0x0 } |
                            if effect_delay.is_some() { 0x10 } else { 0x0 } |
                            if effect_duration_from_server.is_some() { 0x20 } else { 0x0 } |
                            if delta_hp_id.is_some() { 0x40 } else { 0x0 },
                        combat_flags,
                        total_damage_or_heal_amount,
                        effect_delay,
                        effect_duration_from_server,
                        delta_hp_id,
                        ..Default::default()
                    })
                })
                .collect::<WorldResult<Vec<AbilityEffect>>>()?;

            for (ent, controller, interests) in players.iter() {
                if ent == source_ent {
                    controller.send_packet(oaPktAbilityUse {
                        player: controller.avatar_id(),
                        source_avatar: source.id,
                        skill_id: ability.id,
                        source_id,
                        event_type: event_type.try_into()
                            .map_err(|_| anyhow!("invalid event type"))?,
                        ability_invoke_location: ability_invoke_location.into(),
                        ability_type: ability_type.try_into()
                            .map_err(|_| anyhow!("invalid ability type"))?,
                        server_event_duration: event_duration,
                        flag: if rotation.is_some() { 2 } else { 0 },
                        rotation: rotation.map(|v| v.into()),
                        effect_count: effects.len() as _,
                        effects: effects.clone(),
                        prediction_id,
                        combo_stage_id,
                        ..Default::default()
                    });
                } else if interests.contains_key(&source_ent) {
                    controller.send_packet(oaPktAbilityUse {
                        player: controller.avatar_id(),
                        source_avatar: source.id,
                        skill_id: ability.id,
                        source_id,
                        event_type: event_type.try_into()
                            .map_err(|_| anyhow!("invalid event type"))?,
                        ability_invoke_location: ability_invoke_location.into(),
                        ability_type: ability_type.try_into()
                            .map_err(|_| anyhow!("invalid ability type"))?,
                        server_event_duration: event_duration,
                        flag: if rotation.is_some() { 2 } else { 0 },
                        rotation: rotation.map(|v| v.into()),
                        effect_count: effects.len() as _,
                        effects: effects.clone(),
                        ..Default::default()
                    });
                }
            }

            Ok(())
        })?)?;

    Ok(())
}