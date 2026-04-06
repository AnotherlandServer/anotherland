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

use std::{sync::Arc, time::{Duration, Instant}};

use anyhow::anyhow;
use bevy::{app::App, ecs::{component::Component, error::Result, query::Changed, system::{In, Query}}, platform::collections::HashMap};
use futures::TryStreamExt;
use mlua::{FromLua, Lua};
use obj_params::Class;
use protocol::{CooldownEntry, CooldownUpdate, oaPktCooldownUpdate};
use realm_api::{ObjectTemplate, RealmApi};
use scripting::{LuaEntity, ScriptAppExt};
use toolkit::types::Uuid;

use crate::{error::WorldResult, plugins::{ConnectionState, ContentCache, ContentCacheRef, CurrentState, LoadContext, LoadableComponent, PlayerController, WeakCache}};

#[derive(Clone)]
enum CooldownState {
    Ready,
    Consumed,
    Cooldown(Instant, Duration),
}

#[derive(Component)]
pub struct Cooldowns(HashMap<CooldownKey, (CooldownKind, CooldownState)>);

#[derive(Clone)]
pub enum CooldownKind {
    CooldownGroup(Arc<ObjectTemplate>),
    SkillGroup(Arc<ObjectTemplate>),
    Ability(i32),
}

impl CooldownKind {
    pub fn numeric_id(&self) -> i32 {
        match self {
            CooldownKind::CooldownGroup(group) => group.numeric_id,
            CooldownKind::SkillGroup(group) => group.numeric_id,
            CooldownKind::Ability(id) => *id,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum CooldownKey {
    Group(Uuid),
    Ability(i32),
}

impl FromLua for CooldownKey {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        if value.is_string() {
            let id = value.to_string()?
                .parse::<Uuid>()
                .map_err(mlua::Error::external)?;
            Ok(CooldownKey::Group(id))
        } else if value.is_integer() {
            Ok(CooldownKey::Ability(value.as_integer().unwrap() as i32))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "CooldownReq".to_string(),
                message: Some("expected string or integer".to_string()),
            })
        }
    }
}

impl LoadableComponent for Cooldowns {
    type Parameters = ();

    async fn load(_parameters: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        let mut cooldowns = HashMap::new();

        let mut cursor = RealmApi::get()
            .query_object_templates()
            .class(Class::CooldownGroupExternal)
            .query().await?;
        
        while let Some(cooldown) = cursor.try_next().await.unwrap() {
            let cooldown = ContentCache::get(&ContentCacheRef::Uuid(cooldown.id)).await.unwrap().unwrap();

            cooldowns.insert(CooldownKey::Group(cooldown.id), (CooldownKind::CooldownGroup(cooldown), CooldownState::Ready));
        }

        let mut cursor = RealmApi::get()
            .query_object_templates()
            .class(Class::SkillGroup)
            .query().await?;
        
        while let Some(cooldown) = cursor.try_next().await.unwrap() {
            let cooldown = ContentCache::get(&ContentCacheRef::Uuid(cooldown.id)).await.unwrap().unwrap();

            cooldowns.insert(CooldownKey::Group(cooldown.id), (CooldownKind::SkillGroup(cooldown), CooldownState::Ready));
        }

        Ok(Self(cooldowns))
    }
    
    type ContextData = ();
}

#[allow(unused)]
impl Cooldowns {
    #[allow(unused)]
    pub fn is_ready(&self, key: CooldownKey) -> bool {
        self.0.get(&key).map_or_else(|| false, |(_, state)| matches!(state, CooldownState::Ready))
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

    pub fn consume(&mut self, keys: &[CooldownKey]) -> bool {
        self.update();

        let mut state = self.0.clone();

        for key in keys {
            if  
                !state.contains_key(key) &&
                let CooldownKey::Ability(id) = key 
            {
                state.insert(*key, (CooldownKind::Ability(*id), CooldownState::Ready));
            }

            if let Some((_, cooldown_state)) = state.get_mut(key) {
                if !matches!(cooldown_state, CooldownState::Ready) {
                    return false;
                }

                *cooldown_state = CooldownState::Consumed;
            } else {
                return false;
            }
        }

        self.0 = state;
        true
    }

    pub fn emit(&mut self, keys: &[CooldownKey], duration: Duration) -> bool {
        self.update();

        let states = self.0.iter_mut()
            .filter(|(key, (_, state))| {
                keys.contains(key) && matches!(state, CooldownState::Consumed)
            })
            .collect::<Vec<_>>();
        
        if states.len() == keys.len() {
            for (_, (_, state)) in states {
                *state = CooldownState::Cooldown(Instant::now(), duration);
            }

            true
        } else {
            false
        }
    }
}


pub fn insert_cooldown_api(app: &mut App) {
    app
        .add_lua_api("cooldown", "Consume", 
        |
            In((obj, keys)): In<(LuaEntity, Vec<CooldownKey>)>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<bool> {
            let mut cooldowns = query.get_mut(obj.entity())
                .map_err(|_| anyhow!("object not found"))?;

            Ok(cooldowns.consume(&keys))
        })
        .add_lua_api("cooldown", "Emit",
        |
            In((obj, keys, duration)): In<(LuaEntity, Vec<CooldownKey>, f32)>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<()> {
            let mut cooldowns = query.get_mut(obj.entity())
                .map_err(|_| anyhow!("object not found"))?;

            cooldowns.emit(&keys, Duration::from_secs_f32(duration));

            Ok(())
        })
        .add_lua_api("cooldown", "Reset", 
        |
            In(obj): In<LuaEntity>,
            mut query: Query<&mut Cooldowns>,
        | -> WorldResult<()> {
            let mut cooldowns = query.get_mut(obj.entity())
                .map_err(|_| anyhow!("object not found"))?;

            cooldowns.0
                .iter_mut()
                .for_each(|(_, (_, state))| *state = CooldownState::Ready);

            Ok(())
        });
}

pub fn send_cooldown_updates(
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
                                key: cooldown.numeric_id(),
                                field_1: true,
                                total_duration: 0.0,
                                remaining_duration: 0.0,
                            }
                        },
                        CooldownState::Consumed => {
                            CooldownEntry {
                                key: cooldown.numeric_id(),
                                field_1: false,
                                total_duration: -1.0,
                                remaining_duration: -1.0,
                            }
                        },
                        CooldownState::Cooldown(start, duration) => {
                            let elapsed = start.elapsed().as_secs_f32();
                            let remaining = duration.as_secs_f32() - elapsed;

                            CooldownEntry {
                                key: cooldown.numeric_id(),
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
