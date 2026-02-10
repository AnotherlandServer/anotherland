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

use std::sync::Arc;

use anyhow::anyhow;
use bevy::ecs::{component::Component, entity::Entity, error::Result, query::With, system::{Commands, In, Query, Res}, world::World};
use log::debug;
use mlua::{IntoLua, Lua, Table, Value};
use obj_params::{Class, ContentRefList, GameObjectData};
use protocol::{AbilityEffect, OaPktAbilityUseAbilityType, oaPktAbilityUse};
use realm_api::ObjectTemplate;
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{QuatWrapper, Vec3Wrapper, types::Uuid};

use crate::{error::WorldResult, plugins::{Active, Avatar, ContentCache, ContentCacheRef, ContentInfo, Interaction, InteractionEvent, Interests, LoadContext, LoadableComponent, ParamValue, PlayerController, SkillbookEntry, StaticObject, WeakCache}};

#[derive(Component)]
pub struct NpcAbilities(pub Vec<(GameObjectData, Arc<ObjectTemplate>)>);

impl LoadableComponent for NpcAbilities {
    type Parameters = ContentRefList;

    async fn load(parameters: Self::Parameters, _context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        let mut abilities = Vec::with_capacity(parameters.len());

        for content_ref in parameters.iter() {
            let Some(template) = 
                ContentCache::get(&ContentCacheRef::ContentRef(*content_ref)).await? 
            else {
                return Err(anyhow!(
                    "Failed to load object template {}", 
                    content_ref,
                ).into());
            };

            abilities.push((
                GameObjectData::instantiate(template.clone()), 
                template
            ));
        }

        Ok(NpcAbilities(abilities))
    }
}

pub fn insert_ability_api(
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
            players: Query<(Entity, &PlayerController, &Interests), With<Active>>,
            npc_abilities: Query<&NpcAbilities>,
            content: Query<&ContentInfo>,
            targets: Query<&Avatar>,
        | -> WorldResult<()> {
            let ability = if let Ok(ability) = params.get::<Table>("ability") {
                if let Ok(skill) = ability.get::<SkillbookEntry>("__skill") {
                    Some(skill.ability.clone())
                } else if let Ok(ability) = ability.get::<StaticObject>("__static_object") {
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
                    interests.contains(&source_ent) ||
                    target_ent.map(|t| interests.contains(&t)).unwrap_or(false)
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
            commands.write_message(InteractionEvent {
                source: sender.entity()?,
                target: target.entity()?,
                interaction,
            });
        Ok(())
    })?)?;

    Ok(())
}