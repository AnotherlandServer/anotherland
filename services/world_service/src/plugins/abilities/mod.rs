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

mod item_ability;
mod npc_abilities;

pub use item_ability::*;
pub use npc_abilities::*;

use std::{sync::Arc, time::{Duration, Instant}};

use bevy::{app::{App, Plugin, PostUpdate, Update}, ecs::{component::Component, event::{Event, EventReader}, message::Message, query::Changed, resource::Resource, system::{Commands, Res}, world::World}, platform::collections::HashMap, prelude::{Entity, In, Query}};
use futures::TryStreamExt;
use mlua::{FromLua, Function, IntoLua, Lua, Table, Value};
use obj_params::{Class, GameObjectData};
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, oaPktCooldownUpdate, oaPktInteractionUpdate, AbilityEffect, CooldownEntry, CooldownUpdate, OaPktAbilityUseAbilityType, OaPktInteractionUpdateEventType, OaPktInteractionUpdateInteractionType};
use realm_api::{ObjectTemplate, RealmApi};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, EntityScriptCommandsExt, ScriptObject, ScriptResult};
use toolkit::{types::{AvatarId, Uuid}, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{ConnectionState, ContentCache, ContentCacheRef, ParamValue, WeakCache}};

use super::{AvatarIdManager, Avatar, StaticObject, ContentInfo, CurrentState, Interests, NetworkExtPriv, PlayerController, SkillbookEntry};

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

pub enum AbilityKind {
    Item(Entity, StaticObject),
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

#[derive(Event, Message)]
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction: Interaction,
}


enum CooldownState {
    Ready,
    Consumed,
    Cooldown(Instant, Duration),
}

#[derive(Resource)]
pub struct CooldownGroups(Vec<Arc<ObjectTemplate>>);

impl CooldownGroups {
    pub async fn load() -> WorldResult<Self> {
        let mut groups = vec![];

        let mut cursor = RealmApi::get()
            .query_object_templates()
            .class(Class::CooldownGroupExternal)
            .query().await?;
        
        while let Some(cooldown) = cursor.try_next().await.unwrap() {
            let cooldown = ContentCache::get(&ContentCacheRef::Uuid(cooldown.id)).await.unwrap().unwrap();

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
pub struct Cooldowns(HashMap<Uuid, (Arc<ObjectTemplate>, CooldownState)>);

impl Cooldowns {
    pub fn insert(&mut self, group: Arc<ObjectTemplate>) {
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


fn send_interaction_events(
    mut events: EventReader<InteractionEvent>,
    players: Query<(&Avatar, &PlayerController)>,
    targets: Query<(&Avatar, &ScriptObject)>,
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