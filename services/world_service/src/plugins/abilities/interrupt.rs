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
use bevy::ecs::{entity::Entity, message::{Message, MessageReader}, system::{Commands, In}, world::World};
use mlua::Lua;
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaExt, LuaRuntime, ScriptResult};

use crate::{error::WorldResult, plugins::{CombatEvent, CombatEventType, EffectAmount}};

#[derive(Message)]
pub struct Interruption {
    pub source: Option<Entity>,
    pub target: Entity,
    pub kind: Kind,
}

pub enum Kind {
    Cancellation,
    Unspecific,
    EnterCombat,
    LeaveCombat,
    Damage,
    DamageCritical,
    Movement,
    Death,
}

pub fn process_interruptions(
    mut messages: MessageReader<Interruption>,
    mut commands: Commands,
) {
    for Interruption { source, target, kind } in messages.read() {
        commands
            .entity(*target)
            .call_named_lua_method("OnInterrupt", (match kind {
                Kind::Cancellation => "Cancellation",
                Kind::Unspecific => "Unspecific",
                Kind::EnterCombat => "EnterCombat",
                Kind::LeaveCombat => "LeaveCombat",
                Kind::Damage => "Damage",
                Kind::DamageCritical => "DamageCritical",
                Kind::Movement => "Movement",
                Kind::Death => "Death",
            }, source.map(LuaEntity)));
    }
}

pub fn insert_interrupt_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let interrupt_api = lua.create_table().unwrap();
    runtime.register_native("interrupt", interrupt_api.clone()).unwrap();

    interrupt_api.set("TriggerInterruption", lua.create_bevy_function(world, 
        |
            In((kind, target, source)): In<(String, LuaEntity, Option<LuaEntity>)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.write_message(Interruption {
                kind: match kind.as_str() {
                    "Cancellation" => Kind::Cancellation,
                    "Unspecific" => Kind::Unspecific,
                    "EnterCombat" => Kind::EnterCombat,
                    "LeaveCombat" => Kind::LeaveCombat,
                    "Damage" => Kind::Damage,
                    "DamageCritical" => Kind::DamageCritical,
                    "Movement" => Kind::Movement,
                    "Death" => Kind::Death,
                    _ => return Err(anyhow!("Invalid interruption kind: {kind}").into())
                },
                target: target.entity(),
                source: source.map(LuaEntity::take),
            });

            Ok(())
        })?)?;

    Ok(())
}

pub fn generate_combat_interrupt_events(
    mut events: MessageReader<CombatEvent>,
    mut commands: Commands,
) {
    for &CombatEvent { target, instigator: source, update, .. } in events.read() {
        let kind = match update {
            CombatEventType::Damaged(EffectAmount::Normal(_)) => Kind::Damage,
            CombatEventType::Damaged(EffectAmount::Critical(_)) => Kind::DamageCritical,
            CombatEventType::Death => Kind::Death,
            _ => continue,
        };

        commands.write_message(Interruption {
            source,
            target,
            kind,
        });
    }
}