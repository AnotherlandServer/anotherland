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

use std::sync::atomic::AtomicI32;

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{message::{Message, MessageReader, MessageWriter}, schedule::IntoScheduleConfigs, world::World}, prelude::{Added, App, Changed, Commands, Component, Entity, In, Or, Query, With}};
use mlua::{FromLua, IntoLua, Lua, Table};
use obj_params::{tags::{EdnaContainerTag, EdnaReceptorTag, NpcBaseTag, NpcOtherlandTag, PlayerTag, SpawnerTag, StructureTag, VehicleBaseTag}, GameObjectData, Player};
use protocol::{oaPkt_Combat_HpUpdate, CPktTargetRequest};
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use anyhow::anyhow;

use crate::error::WorldResult;

use super::{Avatar, Interests, NetworkExtPriv, PlayerController, spawn_init_entity};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
        app.add_systems(PreUpdate, init_health.after(spawn_init_entity));
        app.add_systems(Update, (
            process_health_events,
            store_health.after(process_health_events),
            trigger_lua_events,
        ));

        app.add_message::<HealthUpdateRequest>();
        app.add_message::<CombatEvent>();

        insert_combat_api(app.world_mut()).unwrap();
    }
}

static LAST_HEALTH_UPDATE_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Message)]
pub struct HealthUpdateRequest {
    entity: Entity,
    instigator: Option<Entity>,
    source: Option<EffectSource>,
    id: i32,
    update: HealthUpdateType,
}

#[derive(Clone, Copy)]
pub enum EffectSource {
    Ability(Entity),
    Buff(Entity),
    Item(Entity),
}

impl IntoLua for EffectSource {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        match self {
            EffectSource::Ability(ent) => {
                table.set("type", "Ability")?;
                table.set("entity", LuaEntity(ent))?;
            },
            EffectSource::Buff(ent) => {
                table.set("type", "Buff")?;
                table.set("entity", LuaEntity(ent))?;
            },
            EffectSource::Item(ent) => {
                table.set("type", "Item")?;
                table.set("entity", LuaEntity(ent))?;
            },
        }
        Ok(mlua::Value::Table(table))
    }
}

impl FromLua for EffectSource {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = value.as_table().ok_or(mlua::Error::runtime("table expected"))?;
        let typ: String = table.get("type")?;
        let ent = table.get::<LuaEntity>("entity")?.take();

        match typ.as_str() {
            "Ability" => Ok(EffectSource::Ability(ent)),
            "Buff" => Ok(EffectSource::Buff(ent)),
            "Item" => Ok(EffectSource::Item(ent)),
            _ => Err(mlua::Error::runtime("invalid effect source type")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum HealthUpdateType {
    Damage(EffectAmount),
    Heal(EffectAmount),
    Kill,
    Revive(Option<EffectAmount>),
}

#[derive(Clone, Copy)]
pub enum EffectAmount {
    Normal(u32),
    Critical(u32),
}

impl IntoLua for EffectAmount {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        match self {
            EffectAmount::Normal(amount) => {
                table.set("type", "Normal")?;
                table.set("amount", amount)?;
            },
            EffectAmount::Critical(amount) => {
                table.set("type", "Critical")?;
                table.set("amount", amount)?;
            },
        }
        Ok(mlua::Value::Table(table))
    }
}

impl FromLua for EffectAmount {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = value.as_table().ok_or(mlua::Error::runtime("table expected"))?;
        let typ: String = table.get("type")?;
        let amount: u32 = table.get("amount")?;

        match typ.as_str() {
            "Normal" => Ok(EffectAmount::Normal(amount)),
            "Critical" => Ok(EffectAmount::Critical(amount)),
            _ => Err(mlua::Error::runtime("invalid effect type")),
        }
    }
}

impl EffectAmount {
    pub fn value(&self) -> i32 {
        match self {
            &EffectAmount::Normal(amount) | &EffectAmount::Critical(amount) => 
                amount.try_into().expect("effect amount overflow"),
        }
    }
}

impl HealthUpdateRequest {
    fn next_id() -> i32 {
        loop {
            // Avoid id 0 if LAST_HEALTH_UPDATE_ID wraps around
            let id = LAST_HEALTH_UPDATE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if id != 0 {
                break id;
            }
        }
    }

    #[allow(dead_code)]
    pub fn damage(entity: Entity, instigator: Option<Entity>, source: Option<EffectSource>, amount: EffectAmount) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id: Self::next_id(), 
            update: HealthUpdateType::Damage(amount),
        }
    }

    #[allow(dead_code)]
    pub fn heal(entity: Entity, instigator: Option<Entity>, source: Option<EffectSource>, amount: EffectAmount) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id: Self::next_id(), 
            update: HealthUpdateType::Heal(amount),
        }
    }

    pub fn kill(entity: Entity, instigator: Option<Entity>, source: Option<EffectSource>) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id: Self::next_id(), 
            update: HealthUpdateType::Kill,
        }
    }

    pub fn revive(entity: Entity, instigator: Option<Entity>, source: Option<EffectSource>, hitpoints: Option<EffectAmount>) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id: Self::next_id(), 
            update: HealthUpdateType::Revive(hitpoints),
        }
    }

    #[allow(dead_code)]
    pub fn send(self, writer: &mut MessageWriter<Self>) -> i32 {
        let id = self.id;
        writer.write(self);
        id
    }
}

#[derive(Message)]
pub struct CombatEvent {
    pub target: Entity,
    pub instigator: Option<Entity>,
    pub source: Option<EffectSource>,
    pub update: CombatEventType,
    pub id: i32,
}

#[derive(Clone, Copy)]
pub enum CombatEventType {
    Damaged(EffectAmount),
    Healed(EffectAmount),
    Death,
    Revived,
}

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, CPktTargetRequest)>,
    mut player: Query<&mut GameObjectData>,
) {
    if let Ok(mut player) = player.get_mut(ent) {
        player.set(Player::Target,  pkt.target_avatar_id);
    }
}

#[derive(Component)]
pub struct Health {
    pub min: i32,
    pub max: i32,
    pub current: i32,
    pub alive: bool,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Energy {
    pub min: i32,
    pub max: i32,
    pub current: i32,
}

#[allow(clippy::type_complexity)]
fn init_health(
    query: Query<(Entity, &GameObjectData), Or<(
        Added<PlayerTag>, 
        Added<VehicleBaseTag>,
        Added<StructureTag>,
        Added<SpawnerTag>,
        Added<EdnaContainerTag>,
        Added<EdnaReceptorTag>,
        Added<NpcOtherlandTag>,
    )>>,
    mut commands: Commands,
) {
    for (ent, obj) in query.iter() {
        if 
            let Ok(&min) = obj.get_named::<i32>("hpMin") &&
            let Ok(&max) = obj.get_named::<i32>("hpMax") &&
            let Ok(&current) = obj.get_named_or_default::<i32>("hpCur", &max) && 
            let Ok(&alive) = obj.get_named_or_default::<bool>("alive", &true)
        {
            commands.entity(ent)
                .insert(Health { min, max, current, alive });
        }
    }
}

fn store_health(
    mut query: Query<(&GameObjectData, &mut Health), Changed<GameObjectData>>,
) {
    for (obj, mut health) in query.iter_mut() {
        health.current = *obj.get_named("hpCur").unwrap();
        health.max = *obj.get_named("hpMax").unwrap();
        health.min = *obj.get_named("hpMin").unwrap();
        health.alive = *obj.get_named("alive").unwrap();
    }
}

#[allow(clippy::type_complexity)]
pub fn process_health_events(
    mut messages: MessageReader<HealthUpdateRequest>,
    mut target: Query<(&Avatar, &mut Health, &mut GameObjectData), Or<(With<PlayerTag>, With<NpcBaseTag>)>>,
    receivers: Query<(&PlayerController, &Interests)>,
    mut commands: Commands,
) {
    for event in messages.read() {
        if let Ok((avatar, mut health, mut obj)) = target.get_mut(event.entity) {
            // Apply update
            match event.update {
                HealthUpdateType::Damage(amount) => {
                    health.current = (health.current - amount.value())
                        .clamp(health.min, health.max);

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Damaged(amount),
                            id: event.id,
                        });

                    if health.current <= health.min {
                        health.alive = false;
                        commands
                            .write_message(CombatEvent { 
                                target: event.entity, 
                                instigator: event.instigator,
                                source: event.source,
                                update: CombatEventType::Death,
                                id: event.id,
                            });
                    }
                },
                HealthUpdateType::Heal(amount) => {
                    if health.alive {
                        health.current = (health.current + amount.value())
                            .clamp(health.min, health.max);
                    }
                },
                HealthUpdateType::Kill => {
                    let damage = health.current - health.min;

                    health.current = health.min;
                    health.alive = false;

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Damaged(EffectAmount::Normal(damage as u32)),
                            id: event.id,
                        });

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Death,
                            id: event.id,
                        });
                },
                HealthUpdateType::Revive(hitpoints) => {
                    // Force value update after revive, by setting it explicitly to false here
                    obj.force_set_named("alive", false);

                    let heal_amount = (
                        hitpoints
                            .map(|a| a.value())
                            .unwrap_or(health.max) - health.current)
                            .clamp(health.min + 1, health.max);

                    if !health.alive {
                        health.current = heal_amount;
                        health.alive = true;
                    }

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Healed(EffectAmount::Normal(heal_amount as u32)),
                            id: event.id,
                        });

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Revived,
                            id: event.id,
                        });
                },
            }

            obj.set_named("hpCur", health.current);
            obj.set_named("hpMax", health.max);
            obj.set_named("hpMin", health.min);
            obj.set_named("alive", health.alive);

            let pkt = oaPkt_Combat_HpUpdate {
                avatar_id: avatar.id,
                hp: health.current,
                id: event.id,
                ..Default::default()
            };

            for (controller, interests) in receivers.iter() {
                if interests.contains(&event.entity) || avatar.id == controller.avatar_id() {
                    controller.send_packet(pkt.clone());
                }
            }
        };
    }
}

fn insert_combat_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let combat_api = lua.create_table().unwrap();
    runtime.register_native("combat", combat_api.clone()).unwrap();

    combat_api.set("Damage", lua.create_bevy_function(world, 
        |
            In((target, instigator, source, amount)): In<(Table, Option<LuaEntity>, Option<EffectSource>, EffectAmount)>,
            mut health_messages: MessageWriter<HealthUpdateRequest>,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            Ok(
                HealthUpdateRequest::damage(ent, instigator.map(LuaEntity::take), source, amount)
                    .send(&mut health_messages)
            )
        })?)?;

    combat_api.set("Heal", lua.create_bevy_function(world, 
        |
            In((target, instigator, source, amount)): In<(Table, Option<LuaEntity>, Option<EffectSource>, EffectAmount)>,
            mut health_messages: MessageWriter<HealthUpdateRequest>,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            Ok(
                HealthUpdateRequest::heal(ent, instigator.map(LuaEntity::take), source, amount)
                    .send(&mut health_messages)
            )
        })?)?;

    combat_api.set("Revive", lua.create_bevy_function(world, 
        |
            In((target, instigator, source, amount)): In<(Table, Option<LuaEntity>, Option<EffectSource>, EffectAmount)>,
            mut health_messages: MessageWriter<HealthUpdateRequest>,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            Ok(
                HealthUpdateRequest::revive(ent, instigator.map(LuaEntity::take), source, Some(amount))
                    .send(&mut health_messages)
            )
        })?)?;

    Ok(())
}

fn trigger_lua_events(
    mut events: MessageReader<CombatEvent>,
    mut commands: Commands,
) {
    for &CombatEvent { target, instigator, update, .. } in events.read() {
        let (event_name, amount) = match update {
            CombatEventType::Damaged(amount) => ("OnDamage", Some(amount)),
            CombatEventType::Healed(amount) => ("OnHeal", Some(amount)),
            CombatEventType::Death => ("OnDeath", None),
            CombatEventType::Revived => ("OnRevived", None),
        };

        commands
            .entity(target)
            .fire_lua_event(event_name, (
                instigator.map(LuaEntity),
                amount,
            ));
    }
}