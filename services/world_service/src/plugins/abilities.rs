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

use bevy::{app::{App, Plugin, PostUpdate}, ecs::{component::Component, query::Changed, system::{Commands, Res}, world::World}, prelude::{Entity, In, Query}, utils::HashMap};
use mlua::{Lua, Table};
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, oaPktCooldownUpdate, AbilityEffect, CooldownEntry, CooldownUpdate};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use serde::{Deserialize, Serialize};
use toolkit::{types::{AvatarId, Uuid, UUID_NIL}, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::{error::WorldResult, object_cache::CacheEntry, plugins::ConnectionState};

use super::{AvatarIdManager, AvatarInfo, CachedObject, ContentInfo, CurrentState, Interests, NetworkExtPriv, PlayerController, SkillbookEntry};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
        app.add_systems(PostUpdate, send_cooldown_updates);

        insert_cooldown_api(app.world_mut()).unwrap();
        insert_ability_api(app.world_mut()).unwrap();
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum AbilityKind {
    Item(Entity, CachedObject),
    Skill(SkillbookEntry),
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
            if 
                let CooldownState::Cooldown(start, duration) = state &&
                start.elapsed() >= *duration
            {
                *state = CooldownState::Ready;
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
) {
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

            let source_id = if let Ok(item) = params.get::<Table>("item").and_then(|i| i.entity()) {
                if let Ok(content_info) = content.get(item) {
                    content_info.template.id
                } else {
                    return Err(anyhow!("item not found").into());
                }
            } else {
                UUID_NIL
            };

            let source_ent = params.get::<Table>("source")?.entity()?;
            let target_ent = params.get::<Table>("target").ok()
                .and_then(|t| t.entity().ok());
            let source = targets.get(source_ent)
                .map_err(|_| anyhow!("source not found"))?;
            let target = target_ent
                .and_then(|ent| targets.get(ent).ok());
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
                        flag: 
                            if target.is_some() { 1 } else { 0 } |
                            if rotation.is_some() { 2 } else { 0 },
                        target: target.map(|t| t.id),
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