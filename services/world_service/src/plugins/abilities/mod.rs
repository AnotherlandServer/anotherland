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

mod interrupt;
mod hierarchy;
mod lua;
mod cooldown;

pub use interrupt::*;
pub use hierarchy::*;
pub use lua::*;
pub use cooldown::*;

use bevy::{app::{App, Plugin, PostUpdate, Update}, ecs::{message::{Message, MessageReader}, system::{Commands, Res}}, prelude::{Entity, In, Query}};
use mlua::{FromLua, Function, IntoLua, Lua, Table, Value};
use protocol::{oaPktAbilityRequest, oaPktInteractionUpdate, OaPktInteractionUpdateEventType, OaPktInteractionUpdateInteractionType};
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaRuntime};
use toolkit::{types::AvatarId, QuatWrapper};

use super::{Avatar, AvatarIdManager, NetworkExtPriv, PlayerController};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InteractionEvent>();
        app.add_message::<Interruption>();

        app.register_message_handler(handle_ability_request);
        app.add_systems(PostUpdate, send_cooldown_updates);
        app.add_systems(Update, (send_interaction_events, process_interruptions, generate_combat_interrupt_events));

        insert_cooldown_api(app);
        insert_ability_api(app);
        insert_interrupt_api(app);
    }
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
        table.set_metatable(Some(metatable))?;

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

#[derive(Message)]
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction: Interaction,
}

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    runtime: Res<LuaRuntime>,
    avatar_man: Res<AvatarIdManager>,
    mut commands: Commands,
) {
    let request = runtime.vm().create_table().unwrap();

    let target = pkt.params
        .and_then(|s| s.parse::<AvatarId>().ok())
        .and_then(|id| avatar_man.resolve_avatar_id(id))
        .map(LuaEntity);

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

fn send_interaction_events(
    mut events: MessageReader<InteractionEvent>,
    players: Query<(&Avatar, &PlayerController)>,
    targets: Query<&Avatar>,
    mut commands: Commands,
) {
    for &InteractionEvent { source, target, interaction } in events.read() {
        let Ok((player, controller)) = players.get(source) else { continue; };
        let Ok(target_avatar) = targets.get(target) else { continue; };

        controller.send_packet(oaPktInteractionUpdate {
            instigator: player.id,
            target: target_avatar.id,
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
                    .fire_lua_event("OnInteractionStart", (LuaEntity(target), interaction));
            },
            Interaction::Extract { .. } => {
                commands.entity(source)
                    .fire_lua_event("OnInteractionStart", (LuaEntity(target), interaction));
            },
            Interaction::Capture { .. } => {
                commands.entity(source)
                    .fire_lua_event("OnInteractionStart", (LuaEntity(target), interaction));
            },
            Interaction::CastComplete => {
                commands.entity(source)
                    .fire_lua_event("OnCastCompleted", (LuaEntity(target), interaction));
            },
            Interaction::CastInterrupt => {
                commands.entity(source)
                    .fire_lua_event("OnCastInterrupted", (LuaEntity(target), interaction));
            },
        }
    }
}