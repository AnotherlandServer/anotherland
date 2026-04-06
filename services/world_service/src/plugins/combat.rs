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

use bevy::{app::{Last, Plugin, PreUpdate, Update}, ecs::{message::{Message, MessageReader}, schedule::IntoScheduleConfigs}, platform::collections::HashMap, prelude::{Added, App, Changed, Commands, Component, Entity, In, Or, Query, With}};
use log::debug;
use mlua::{FromLua, IntoLua, Lua, Table, UserData, UserDataFields, UserDataMethods};
use obj_params::{Class, GameObjectData, GenericParamSet, Player, Value, tags::{EdnaContainerTag, EdnaReceptorTag, NpcBaseTag, NpcOtherlandTag, PlayerTag, SpawnerTag, StructureTag, VehicleBaseTag}};
use protocol::{oaPkt_Combat_HpUpdate, CPktTargetRequest};
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaTableExt, ScriptAppExt};
use anyhow::anyhow;
use toolkit::types::AvatarId;

use crate::error::WorldResult;

use super::{Avatar, Interests, NetworkExtPriv, PlayerController, spawn_init_entity};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
        app.add_systems(PreUpdate, init_health);
        app.add_systems(Update, (
            process_health_events,
            store_health.after(process_health_events),
            trigger_lua_events,
        ));
        app.add_systems(Last, send_health_update_events);

        app.add_message::<HealthUpdateRequest>();
        app.add_message::<CombatEvent>();

        insert_combat_api(app);
    }
}

static LAST_HEALTH_UPDATE_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Message)]
pub struct HealthUpdateRequest {
    pub entity: Entity,
    pub instigator: Option<Entity>,
    pub source: Option<EffectSource>,
    pub id: i32,
    pub update: HealthUpdateType,
    pub canceled: bool,
}

impl UserData for HealthUpdateRequest {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("entity", |_, this| Ok(LuaEntity(this.entity)));
        fields.add_field_method_get("instigator", |_, this| Ok(this.instigator.map(LuaEntity)));
        fields.add_field_method_get("source", |_, this| Ok(this.source));
        fields.add_field_method_get("id", |_, this| Ok(this.id));
        fields.add_field_method_get("update", |_, this| Ok(this.update));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("Cancel", |_, this, ()| {
            this.canceled = true;
            Ok(())
        });

        methods.add_method("IsCanceled", |_, this, ()| Ok(this.canceled));

        methods.add_method_mut("ModifyAmount", |_, this, amount: EffectAmount| {
            match &mut this.update {
                HealthUpdateType::Damage(current) => {
                    if let EffectAmount::Normal(_) = current {
                        *current = amount;
                    }
                },
                HealthUpdateType::Heal(current) => {
                    if let EffectAmount::Normal(_) = current {
                        *current = amount;
                    }
                },
                HealthUpdateType::Revive(_) => {
                    if let EffectAmount::Normal(_) = amount {
                        this.update = HealthUpdateType::Revive(Some(amount));
                    }
                },
                _ => {},
            }
            Ok(())
        });
    }
}

#[derive(Clone, Copy)]
pub enum EffectSource {
    Ability(Entity),
    Buff(Entity),
    Item(Entity),
}

impl EffectSource {
    pub fn from_class(class: Class, entity: Entity) -> Self {
        match class {
            Class::EdnaAbility => EffectSource::Ability(entity),
            Class::OaBuff2 => EffectSource::Buff(entity),
            Class::EdnaFunction => EffectSource::Item(entity),
            Class::EdnaModule => EffectSource::Item(entity),
            _ => panic!("invalid effect source class"),
        }
    }

    pub fn entity(&self) -> Entity {
        match self {
            EffectSource::Ability(ent) => *ent,
            EffectSource::Buff(ent) => *ent,
            EffectSource::Item(ent) => *ent,
        }
    }
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

impl IntoLua for HealthUpdateType {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        match self {
            HealthUpdateType::Damage(amount) => {
                table.set("type", "Damage")?;
                table.set("amount", amount)?;
            },
            HealthUpdateType::Heal(amount) => {
                table.set("type", "Heal")?;
                table.set("amount", amount)?;
            },
            HealthUpdateType::Kill => {
                table.set("type", "Kill")?;
            },
            HealthUpdateType::Revive(hitpoints) => {
                table.set("type", "Revive")?;
                if let Some(hitpoints) = hitpoints {
                    table.set("hitpoints", hitpoints)?;
                }
            },
        }
        Ok(mlua::Value::Table(table))
    }
}

impl FromLua for HealthUpdateType {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = value.as_table().ok_or(mlua::Error::runtime("table expected"))?;
        let typ: String = table.get("type")?;

        match typ.as_str() {
            "Damage" => {
                let amount = table.get("amount")?;
                Ok(HealthUpdateType::Damage(amount))
            },
            "Heal" => {
                let amount = table.get("amount")?;
                Ok(HealthUpdateType::Heal(amount))
            },
            "Kill" => Ok(HealthUpdateType::Kill),
            "Revive" => {
                let hitpoints = table.get::<Option<EffectAmount>>("hitpoints")?;
                Ok(HealthUpdateType::Revive(hitpoints))
            },
            _ => Err(mlua::Error::runtime("invalid health update type")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn next_id() -> i32 {
        loop {
            // Avoid id 0 if LAST_HEALTH_UPDATE_ID wraps around
            let id = LAST_HEALTH_UPDATE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if id != 0 {
                break id;
            }
        }
    }

    #[allow(dead_code)]
    pub fn damage(entity: Entity, id: i32, instigator: Option<Entity>, source: Option<EffectSource>, amount: EffectAmount) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id, 
            update: HealthUpdateType::Damage(amount),
            canceled: false,
        }
    }

    #[allow(dead_code)]
    pub fn heal(entity: Entity, id: i32, instigator: Option<Entity>, source: Option<EffectSource>, amount: EffectAmount) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id, 
            update: HealthUpdateType::Heal(amount),
            canceled: false,
        }
    }

    pub fn kill(entity: Entity, id: i32, instigator: Option<Entity>, source: Option<EffectSource>) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id, 
            update: HealthUpdateType::Kill,
            canceled: false,
        }
    }

    pub fn revive(entity: Entity, id: i32, instigator: Option<Entity>, source: Option<EffectSource>, hitpoints: Option<EffectAmount>) -> Self {
        Self { 
            entity, 
            instigator,
            source,
            id, 
            update: HealthUpdateType::Revive(hitpoints),
            canceled: false,
        }
    }

    pub fn send(self, commands: &mut Commands) -> i32 {
        let id = self.id;

        commands
            .write_message(self);

        id
    }
}

#[derive(Message, Clone)]
pub struct CombatEvent {
    pub target: Entity,
    pub instigator: Option<Entity>,
    pub source: Option<EffectSource>,
    pub update: CombatEventType,
    pub id: i32,
}

#[derive(Clone, Copy, Debug)]
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
    mut query: Query<(Entity, &GameObjectData, &mut Health), Changed<GameObjectData>>,
) {
    for (ent, obj, mut health) in query.iter_mut() {
        let diff = obj.changes()
            .collect::<Box<dyn GenericParamSet>>();

        if let Some(&Value::Int(max)) = diff.get_param("hpMax") {
            health.max = max;
        }

        if let Some(&Value::Int(min)) = diff.get_param("hpMin") {
            health.min = min;
        }

        if let Some(&Value::Int(current)) = diff.get_param("hpCur") {
            debug!("Health current changed: entity={:?}, old={}, new={}", ent, health.current, current);
            health.current = current;
        } 

        if let Some(&Value::Bool(alive)) = diff.get_param("alive") {
            health.alive = alive;
        } 
    }
}

#[allow(clippy::type_complexity)]
pub fn process_health_events(
    mut messages: MessageReader<HealthUpdateRequest>,
    mut target: Query<(&mut Health, &mut GameObjectData), Or<(With<PlayerTag>, With<NpcBaseTag>)>>,
    mut commands: Commands,
) {
    for event in messages.read() {
        if let Ok((mut health, mut obj)) = target.get_mut(event.entity) {
            // Apply update
            match event.update {
                HealthUpdateType::Damage(amount) => {
                    debug!("Processing damage event: entity={:?}, source={:?}, damage={}, current_hp={}", event.entity, event.instigator, amount.value(), health.current);

                    health.current = (health.current - amount.value())
                        .clamp(health.min, health.max);

                    debug!("Damage event: entity={:?}, source={:?}, damage={}, current_hp={}", event.entity, event.instigator, amount.value(), health.current);

                    commands
                        .write_message(CombatEvent { 
                            target: event.entity, 
                            instigator: event.instigator,
                            source: event.source,
                            update: CombatEventType::Damaged(amount),
                            id: event.id,
                        });

                    if health.current <= 0 {
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

                        debug!("Heal event: entity={:?}, source={:?}, heal={}, current_hp={}", event.entity, event.instigator, amount.value(), health.current);
                    }
                },
                HealthUpdateType::Kill => {
                    let damage = health.current;

                    health.current = 0;
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
                    let heal_amount = (
                        hitpoints
                            .map(|a| a.value())
                            .unwrap_or(health.max - health.current)
                        )
                        .clamp(health.min, health.max);

                    if !health.alive {
                        health.current += heal_amount;
                        health.alive = true;
                    }

                    //debug!("Revive event: entity={:?}, heal_amount={}, current_hp={}", event.entity, heal_amount, health.current);

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

            obj.force_set_named("hpCur", health.current);
            obj.set_named("hpMax", health.max);
            obj.set_named("hpMin", health.min);
            obj.force_set_named("alive", health.alive);
        };
    }
}

fn send_health_update_events(
    mut events: MessageReader<CombatEvent>,
    avatars: Query<(&Avatar, &Health)>,
    receivers: Query<(Entity, &PlayerController, &Interests)>,
) {
    let mut debounce_map: HashMap<AvatarId, CombatEvent> = HashMap::new();

    for event in events.read() {
        let avatar_id = if let Ok((avatar, _)) = avatars.get(event.target) {
            avatar.id
        } else {
            continue;
        };

        if let Some(prev_event) = debounce_map.get_mut(&avatar_id) {
            if prev_event.id < event.id {
                *prev_event = event.clone();
            }
        } else {
            debounce_map.insert(avatar_id, event.clone());
        }
    }

    for (_, event) in debounce_map.iter() {
        let Ok((avatar, health)) = avatars.get(event.target) else {
            continue;
        };

        let pkt = oaPkt_Combat_HpUpdate {
            avatar_id: avatar.id,
            hp: health.current,
            id: event.id,
            ..Default::default()
        };

        for (ent, controller, interests) in receivers.iter() {
            if event.instigator == Some(ent) {
                debug!("Sending health update event to instigator: avatar_id={}, hp={}, event_id={}", avatar.id, health.current, event.id);   
            }

            if interests.contains(&event.target) || avatar.id == controller.avatar_id() {
                controller.send_packet(pkt.clone());
            }
        }
    }
}

fn insert_combat_api(app: &mut App) {
    app
        .add_lua_api("combat", "GenerateId",|In(()): In<()>| -> WorldResult<i32> {
            Ok(HealthUpdateRequest::next_id())
        })
        .add_lua_api("combat", "FireDamageEvent",
        |
            In((target, id, instigator, source, amount)): In<(Table, i32, Option<LuaEntity>, Option<LuaEntity>, EffectAmount)>,
            objects: Query<&GameObjectData>,
            mut commands: Commands,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            let source = source
                .and_then(|e| Some((e.entity(), objects.get(e.entity()).ok()?)))
                .map(|(e, obj)| EffectSource::from_class(obj.class(), e));

            Ok(
                HealthUpdateRequest::damage(ent, id, instigator.map(LuaEntity::take), source, amount)
                    .send(&mut commands)
            )
        })
        .add_lua_api("combat", "FireHealEvent", 
        |
            In((target, id, instigator, source, amount)): In<(Table, i32, Option<LuaEntity>, Option<LuaEntity>, EffectAmount)>,
            objects: Query<&GameObjectData>,
            mut commands: Commands,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            let source = source
                .and_then(|e| Some((e.entity(), objects.get(e.entity()).ok()?)))
                .map(|(e, obj)| EffectSource::from_class(obj.class(), e));

            Ok(
                HealthUpdateRequest::heal(ent, id, instigator.map(LuaEntity::take), source, amount)
                    .send(&mut commands)
            )
        })
        .add_lua_api("combat", "FireReviveEvent", 
        |
            In((target, id, instigator, source, amount)): In<(Table, i32, Option<LuaEntity>, Option<LuaEntity>, EffectAmount)>,
            objects: Query<&GameObjectData>,
            mut commands: Commands,
        | -> WorldResult<i32> {
            let ent = target.entity()
                .map_err(|_| anyhow!("entity not found"))?;

            let source = source
                .and_then(|e| Some((e.entity(), objects.get(e.entity()).ok()?)))
                .map(|(e, obj)| EffectSource::from_class(obj.class(), e));

            Ok(
                HealthUpdateRequest::revive(ent, id, instigator.map(LuaEntity::take), source, Some(amount))
                    .send(&mut commands)
            )
        });
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