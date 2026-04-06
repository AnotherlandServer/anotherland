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
use bevy::{app::App, ecs::{entity::Entity, message::{Message, MessageReader}, system::{Commands, In}}};
use log::debug;
use scripting::{EntityScriptCommandsExt, LuaEntity, ScriptAppExt};

use crate::{error::WorldResult, plugins::{CombatEvent, CombatEventType, EffectAmount}};

#[derive(Message, Debug)]
pub struct Interruption {
    pub source: Option<Entity>,
    pub target: Entity,
    pub kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    Cancellation,
    Unspecific,
    EnterCombat,
    LeaveCombat,
    Damage,
    DamageCritical,
    Movement,
    Death,
    AbilityStart,
    AbilityUse,
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
                Kind::AbilityStart => "AbilityStart",
                Kind::AbilityUse => "AbilityUse",
            }, source.map(LuaEntity)));
    }
}

pub fn insert_interrupt_api(app: &mut App) {
    app
        .add_lua_api("interrupt", "TriggerInterruption",
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
                    "AbilityStart" => Kind::AbilityStart,
                    "AbilityUse" => Kind::AbilityUse,
                    _ => return Err(anyhow!("Invalid interruption kind: {kind}").into())
                },
                target: target.entity(),
                source: source.map(LuaEntity::take),
            });

            Ok(())
        });
}

pub fn generate_combat_interrupt_events(
    mut events: MessageReader<CombatEvent>,
    mut commands: Commands,
) {
    for &CombatEvent { target, instigator: source, update, .. } in events.read() {
        let kind = match update {
            CombatEventType::Damaged(EffectAmount::Normal(_)) => Kind::Damage,
            CombatEventType::Damaged(EffectAmount::Critical(_)) => {
                // Critical damage is also normal damage, when it comes to interruptions
                commands.write_message(Interruption {
                    source,
                    target,
                    kind: Kind::Damage,
                });

                Kind::DamageCritical
            },
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