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

use bevy::{app::{App, Plugin, PostUpdate, Update}, ecs::{component::Component, event::{Event, EventReader}, query::Changed, resource::Resource, system::{Commands, Res}, world::World}, platform::collections::HashMap, prelude::{Entity, In, Query}};
use futures::TryStreamExt;
use mlua::{FromLua, Function, IntoLua, Lua, Table, Value};
use obj_params::{Class, GameObjectData};
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, oaPktCooldownUpdate, oaPktInteractionUpdate, AbilityEffect, CooldownEntry, CooldownUpdate, OaPktAbilityUseAbilityType, OaPktInteractionUpdateEventType, OaPktInteractionUpdateInteractionType};
use realm_api::RealmApi;
use scripting::{LuaExt, LuaRuntime, LuaTableExt, EntityScriptCommandsExt, ScriptObject, ScriptResult};
use serde::{Deserialize, Serialize};
use toolkit::{types::{AvatarId, Uuid}, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::{error::WorldResult, object_cache::CacheEntry, plugins::{ConnectionState, ParamValue}, OBJECT_CACHE};

use super::{AvatarIdManager, AvatarInfo, CachedObject, ContentInfo, CurrentState, Interests, NetworkExtPriv, PlayerController, SkillbookEntry};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent>();

        app.register_message_handler(handle_ability_request);
        app.add_systems(PostUpdate, send_cooldown_updates);
        app.add_systems(Update, send_interaction_events);

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

#[derive(Debug, Clone, Copy)]
pub enum Interaction {
    Interact { duration: f32 },
    Extract { duration: f32 },
    Capture { duration: f32 },
    CastComplete,
    CastInterrupt,
}

impl FromLua for Interaction {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = match value {
            Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Interaction".to_string(),
                message: Some("expected a table".into()),
            }),
        };

        let kind: String = table.get("kind")?;
        match kind.as_str() {
            "interact" => {
                let duration: f32 = table.get("duration")?;
                Ok(Interaction::Interact { duration })
            },
            "extract" => {
                let duration: f32 = table.get("duration")?;
                Ok(Interaction::Extract { duration })
            },
            "capture" => {
                let duration: f32 = table.get("duration")?;
                Ok(Interaction::Capture { duration })
            },
            "cast_complete" => Ok(Interaction::CastComplete),
            "cast_interrupt" => Ok(Interaction::CastInterrupt),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "Interaction".to_string(),
                message: Some(format!("unknown interaction kind: {}", kind)),
            }),
        }
    }
}

impl IntoLua for Interaction {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let require = lua.globals().get::<Function>("require")?;

        // Requiring another module in into_lua seems wrong. 
        // Is there a better way of doing this?
        let interaction = require.call::<Table>("engine.interaction")?; 

        let metatable = lua.create_table()?;
        metatable.set("__index", interaction)?;

        let table = lua.create_table()?;
        table.set_metatable(Some(metatable));

        match self {
            Interaction::Interact { duration } => {
                table.set("kind", "interact")?;
                table.set("duration", duration)?;
            },
            Interaction::Extract { duration } => {
                table.set("kind", "extract")?;
                table.set("duration", duration)?;
            },
            Interaction::Capture { duration } => {
                table.set("kind", "capture")?;
                table.set("duration", duration)?;
            },
            Interaction::CastComplete => {
                table.set("kind", "cast_complete")?;
            },
            Interaction::CastInterrupt => {
                table.set("kind", "cast_interrupt")?;
            },
        }

        Ok(Value::Table(table))
    }
}

#[derive(Event)]
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction: Interaction,
}

#[derive(Component)]
pub struct NpcAbilities(pub Vec<(GameObjectData, Arc<CacheEntry>)>);

enum CooldownState {
    Ready,
    Consumed,
    Cooldown(Instant, Duration),
}

#[derive(Resource)]
pub struct CooldownGroups(Vec<Arc<CacheEntry>>);

impl CooldownGroups {
    pub async fn load(realm_api: &RealmApi) -> WorldResult<Self> {
        let realm_api = realm_api.clone();
        let mut groups = vec![];

        let mut cursor = realm_api.query_object_templates()
            .class(Class::CooldownGroupExternal)
            .query().await?;
        
        while let Some(cooldown) = cursor.try_next().await.unwrap() {
            let cooldown = OBJECT_CACHE.wait()
                .get_object_by_guid(cooldown.id).await.unwrap()
                .unwrap();

            groups.push(cooldown);
        }

        Ok(Self(groups))
    }

    pub fn create_cooldowns(&self) -> Cooldowns {
        Cooldowns(
            self.0.iter()
                .map(|group| (group.id, (group.clone(), CooldownState::Ready)))
                .collect()
        )
    }
}

#[derive(Component)]
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

    pub fn consume(&mut self, groups: &[Uuid]) -> bool {
        self.update();

        let states = self.0.iter_mut()
            .filter(|(group, (_, state))| {
                groups.contains(group) && matches!(state, CooldownState::Ready)
            })
            .collect::<Vec<_>>();
        
        if states.len() == groups.len() {
            for (_, (_, state)) in states {
                *state = CooldownState::Consumed;
            }

            true
        } else {
            false
        }
    }

    pub fn emit(&mut self, groups: &[Uuid], duration: Duration) -> bool {
        self.update();

        let states = self.0.iter_mut()
            .filter(|(group, (_, state))| {
                groups.contains(group) && matches!(state, CooldownState::Consumed)
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
            In((obj, groups)): In<(Table, Vec<String>)>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<bool> {
            let mut cooldowns = query.get_mut(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let groups = groups.into_iter()
                .map(|s| s.parse::<Uuid>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(cooldowns.consume(&groups))
        })?)?;

    cooldown_api.set("Emit", lua.create_bevy_function(world, 
        |
            In((obj, groups, duration)): In<(Table, Vec<String>, f32)>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<bool> {
            let mut cooldowns = query.get_mut(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let groups = groups.into_iter()
                .map(|s| s.parse::<Uuid>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(cooldowns.emit(&groups, Duration::from_secs_f32(duration)))
        })?)?;

    Ok(())
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
        .and_then(|id| avatar_man.resolve_avatar_id(id))
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
                        CooldownState::Consumed => {
                            CooldownEntry {
                                key: cooldown.numeric_id,
                                field_1: false,
                                total_duration: -1.0,
                                remaining_duration: -1.0,
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

    ability_api.set("GetNpcAbilityCount", lua.create_bevy_function(world, 
        |
            In(obj): In<Table>,
            abilities: Query<&NpcAbilities>,
        | -> WorldResult<i32> {
            let abilities = abilities.get(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            Ok(abilities.0.len() as i32)
        })?)?;

    ability_api.set("GetNpcAbilityInfo", lua.create_bevy_function(world, 
        |
            In((obj, idx)): In<(Table, i32)>,
            abilities: Query<&NpcAbilities>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<Table> {
            let abilities = abilities.get(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let (_, ability) = abilities.0.get(idx as usize)
                .ok_or_else(|| anyhow!("ability index out of bounds"))?;

            let table = runtime.vm().create_table()?;

            table.set("template_guid", ability.id.to_string())?;
            table.set("name", ability.name.clone())?;
            table.set("class", ability.class.name().to_string())?;

            Ok(table)
        })?)?;

    ability_api.set("GetNpcAbilityValue", lua.create_bevy_function(world, 
        |
            In((obj, idx, name)): In<(Table, i32, String)>,
            abilities: Query<&NpcAbilities>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<Value> {
            let abilities = abilities.get(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let (ability, _) = abilities.0.get(idx as usize)
                .ok_or_else(|| anyhow!("ability index out of bounds"))?;

            let val = ability.get_named::<obj_params::Value>(&name)
                .map_err(mlua::Error::external)?;

            Ok(ParamValue::new(val.clone())
                .into_lua(runtime.vm())?)
        })?)?;

    ability_api.set("SetNpcAbilityValue", lua.create_bevy_function(world, 
        |
            In((obj, idx, name, value)): In<(Table, i32, String, Value)>,
            mut abilities: Query<&mut NpcAbilities>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<Value> {
            let mut abilities = abilities.get_mut(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let (ability, _) = abilities.0.get_mut(idx as usize)
                .ok_or_else(|| anyhow!("ability index out of bounds"))?;
            
            let attr = ability.class().get_attribute(&name)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            let value = ParamValue::from_lua(attr, value, runtime.vm())?;

            if let Some(prev_val) = ability.set_named(&name, value) {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
            }
        })?)?;

    ability_api.set("ResetNpcAbilityValue", lua.create_bevy_function(world, 
        |
            In((obj, idx, name)): In<(Table, i32, String)>,
            mut abilities: Query<&mut NpcAbilities>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<Value> {
            let mut abilities = abilities.get_mut(obj.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let (ability, _) = abilities.0.get_mut(idx as usize)
                .ok_or_else(|| anyhow!("ability index out of bounds"))?;
            
            let attr = ability.class().get_attribute(&name)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            if let Some(prev_val) = ability.set_named(&name, attr.default().clone()) {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
            }
        })?)?;

    ability_api.set("FireEvent", lua.create_bevy_function(world, 
        |
            In(params): In<Table>,
            players: Query<(Entity, &PlayerController, &Interests)>,
            npc_abilities: Query<&NpcAbilities>,
            content: Query<&ContentInfo>,
            targets: Query<&AvatarInfo>,
        | -> WorldResult<()> {
            let ability = if let Ok(ability) = params.get::<Table>("ability") {
                if let Ok(skill) = ability.get::<SkillbookEntry>("__skill") {
                    Some(skill.ability.clone())
                } else if let Ok(ability) = ability.get::<CachedObject>("__item_ability") {
                    Some((*ability).clone())
                } else if 
                    let Ok(npc) = ability.get::<Table>("__npc") &&
                    let Ok(abilities) = npc_abilities.get(npc.entity()?) &&
                    let Some(idx) = ability.get::<i32>("__npc_ability_idx").ok()
                {
                    if let Some((_, ability)) = abilities.0.get(idx as usize) {
                        Some((*ability).clone())
                    } else {
                        return Err(anyhow!("npc ability index out of bounds").into());
                    }
                } else {
                    return Err(anyhow!("ability not found").into());
                }
            } else {
                None
            };

            let buff = if let Ok(buff) = params.get::<Table>("buff") {
                if let Ok(content) = content.get(buff.entity()?) {
                    Some(content)
                } else {
                    return Err(anyhow!("buff not found").into());
                }
            } else {
                None
            };

            if ability.is_none() && buff.is_none() {
                return Err(anyhow!("ability or buff must be set").into());
            }

            let (source_id, ability_type) = if let Ok(source) = params.get::<Table>("effect_source") {
                if let Ok(ent) = source.entity() {
                    if let Ok(content_info) = content.get(ent) {
                        (content_info.template.id, match content_info.template.class {
                            Class::EdnaFunction => OaPktAbilityUseAbilityType::Item,
                            Class::EdnaModule => OaPktAbilityUseAbilityType::Item,
                            Class::OaBuff2 => OaPktAbilityUseAbilityType::Buff,
                            _ => return Err(anyhow!("invalid effect_source class").into()),
                        })
                    } else {
                        return Err(anyhow!("effect_source ent not found").into());
                    }
                } else if let Ok(id) = source.get::<String>("template_guid") {
                    if let Ok(id) = id.parse::<Uuid>() {
                        (id, OaPktAbilityUseAbilityType::Skill)
                    } else {
                        return Err(anyhow!("invalid effect_source id").into());
                    }
                } else {
                    return Err(anyhow!("invalid effect_source").into());
                }
            } else {
                return Err(anyhow!("effect_source not set").into());
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
            let event_type = params.get::<i32>("event_type")?;

            let skill_id = if let Some(ability) = ability {
                ability.id
            } else if let Some(buff) = buff {
                buff.placement_id
            } else {
                unreachable!()
            };

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
                if 
                    ent == source_ent ||
                    interests.contains_key(&source_ent) ||
                    target_ent.map(|t| interests.contains_key(&t)).unwrap_or(false)
                {
                    controller.send_packet(oaPktAbilityUse {
                        player: source.id,
                        source_avatar: source.id, //controller.avatar_id()
                        skill_id,
                        source_id,
                        event_type: event_type.try_into()
                            .map_err(|_| anyhow!("invalid event type"))?,
                        ability_invoke_location: ability_invoke_location.into(),
                        ability_type,
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
                }
            }

            Ok(())
        })?)?;

    ability_api.set("FireInteractionEvent", lua.create_bevy_function(world, 
        |
            In((sender, interaction, target)): In<(Table, Interaction, Table)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.send_event(InteractionEvent {
                source: sender.entity()?,
                target: target.entity()?,
                interaction,
            });
        Ok(())
    })?)?;

    Ok(())
}

fn send_interaction_events(
    mut events: EventReader<InteractionEvent>,
    players: Query<(&AvatarInfo, &PlayerController)>,
    targets: Query<(&AvatarInfo, &ScriptObject)>,
    mut commands: Commands,
) {
    for &InteractionEvent { source, target, interaction } in events.read() {
        let Ok((player, controller)) = players.get(source) else { continue; };
        let Ok((target, target_obj)) = targets.get(target) else { continue; };

        controller.send_packet(oaPktInteractionUpdate {
            instigator: player.id,
            target: target.id,
            event_type: match interaction {
                Interaction::Interact { .. } => OaPktInteractionUpdateEventType::Interaction,
                Interaction::Extract { .. } => OaPktInteractionUpdateEventType::Interaction,
                Interaction::Capture { .. } => OaPktInteractionUpdateEventType::Interaction,
                Interaction::CastComplete => OaPktInteractionUpdateEventType::CastCompleted,
                Interaction::CastInterrupt => OaPktInteractionUpdateEventType::CastInterrupted,
            },
            interaction_type: match interaction {
                Interaction::Interact { .. } => OaPktInteractionUpdateInteractionType::QuestInteract,
                Interaction::Extract { .. } => OaPktInteractionUpdateInteractionType::EdnaExtract,
                Interaction::Capture { .. } => OaPktInteractionUpdateInteractionType::CapturingFlag,
                Interaction::CastComplete => OaPktInteractionUpdateInteractionType::QuestInteract, // undefined
                Interaction::CastInterrupt => OaPktInteractionUpdateInteractionType::QuestInteract, // undefined
            },
            duration: match interaction {
                Interaction::Interact { duration } => duration,
                Interaction::Extract { duration } => duration,
                Interaction::Capture { duration } => duration,
                Interaction::CastComplete => 0.0,
                Interaction::CastInterrupt => 0.0,
            },
            ..Default::default()
        });

        match interaction {
            Interaction::Interact { .. } => {
                commands.entity(source)
                    .fire_lua_event("OnInteractionStart", (target_obj.object().clone(), interaction));
            },
            Interaction::Extract { .. } => {
                commands.entity(source)
                    .fire_lua_event("OnInteractionStart", (target_obj.object().clone(), interaction));
            },
            Interaction::Capture { .. } => {
                commands.entity(source)
                    .fire_lua_event("OnInteractionStart", (target_obj.object().clone(), interaction));
            },
            Interaction::CastComplete => {
                commands.entity(source)
                    .fire_lua_event("OnCastCompleted", (target_obj.object().clone(), interaction));
            },
            Interaction::CastInterrupt => {
                commands.entity(source)
                    .fire_lua_event("OnCastInterrupted", (target_obj.object().clone(), interaction));
            },
        }
    }
}