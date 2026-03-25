// Copyright (C) 2026 AnotherlandServer
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

use anyhow::anyhow;
use bevy::ecs::{entity::Entity, query::With, relationship::RelationshipTarget, system::{Commands, In, Query}, world::World};
use mlua::{Lua, Table};
use obj_params::Class;
use protocol::{AbilityEffect, OaPktAbilityUseAbilityType, oaPktAbilityUse};
use scripting::{LuaEntity, LuaExt, LuaRuntime, ScriptResult};
use toolkit::{QuatWrapper, Vec3Wrapper};

use crate::{error::WorldResult, plugins::{Abilities, Active, Avatar, ContentInfo, Interaction, InteractionEvent, Interests, PlayerController}};

pub fn insert_ability_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let ability_api = lua.create_table().unwrap();
    runtime.register_native("ability", ability_api.clone()).unwrap();

    ability_api.set("GetAbilities", lua.create_bevy_function(world, 
        |
            In(obj): In<LuaEntity>,
            abilities: Query<&Abilities>,
        | -> WorldResult<Vec<LuaEntity>> {
            let Ok(abilities) = abilities.get(obj.entity()) else {
                return Ok(vec![]);
            };

            Ok(
                abilities
                    .iter()
                    .map(LuaEntity)
                    .collect()
            )
        })?)?;

    ability_api.set("FireEvent", lua.create_bevy_function(world, 
        |
            In(params): In<Table>,
            players: Query<(Entity, &PlayerController, &Interests), With<Active>>,
            content: Query<&ContentInfo>,
            targets: Query<&Avatar>,
        | -> WorldResult<()> {
            let ability = params.get::<Option<LuaEntity>>("ability")?;
            let buff = params.get::<Option<LuaEntity>>("buff")?;
            let effect_source = params.get::<LuaEntity>("effect_source")?;

            if ability.is_none() && buff.is_none() {
                return Err(anyhow!("ability or buff must be set").into());
            }

            let (
                source_id,
                ability_type
            ) = 
                if let Ok(content_info) = content.get(effect_source.entity()) {
                    (
                        content_info.template.id, 
                        match content_info.template.class {
                            Class::EdnaFunction => OaPktAbilityUseAbilityType::Item,
                            Class::EdnaModule => OaPktAbilityUseAbilityType::Item,
                            Class::OaBuff2 => OaPktAbilityUseAbilityType::Buff,
                            Class::EdnaAbility => OaPktAbilityUseAbilityType::Skill,
                            _ => return Err(anyhow!("invalid effect_source class").into()),
                        }
                    )
                } else {
                    return Err(anyhow!("effect_source ent not found").into());
                };

            let skill_id = 
                ability
                    .and_then(|ent| content.get(ent.entity()).ok())
                    .map(|c| c.template.id)
                    .or(
                        buff
                            .and_then(|ent| content.get(ent.entity()).ok())
                            .map(|c| c.placement_id)
                    )
                    .ok_or(anyhow!("ability or buff must be valid"))?;

            let source_ent = params.get::<LuaEntity>("source")?.entity();
            let target_ent = params.get::<LuaEntity>("target").ok()
                .map(LuaEntity::take);
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

            let effects = effects.sequence_values()
                .flatten()
                .map(|effect: Table| -> WorldResult<AbilityEffect> {
                    let target = effect.get::<LuaEntity>("target")?.entity();
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
                        source_avatar: source.id,
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
            In((sender, interaction, target)): In<(LuaEntity, Interaction, LuaEntity)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.write_message(InteractionEvent {
                source: sender.entity(),
                target: target.entity(),
                interaction,
            });
        Ok(())
    })?)?;

    Ok(())
}