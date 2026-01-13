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

use std::{ops::Deref, sync::Arc};

use bevy::ecs::{component::Component, error::Result, query::Changed, system::Query};
use futures::future::join_all;
use log::{debug, warn};
use mlua::{FromLua, IntoLua, Table, UserData};
use obj_params::{EdnaAbility, GameObjectData, Player};
use protocol::{oaAbilityDataPlayer, oaAbilityDataPlayerArray};
use realm_api::{ObjectTemplate, RealmApi, State};
use scripting::LuaRuntime;
use toolkit::types::Uuid;

use crate::{error::WorldResult, plugins::{CombatStyle, ContentCache, ContentCacheRef, LoadContext, LoadableComponent, ParamValue, WeakCache, load_class_script}};

#[derive(Component)]
pub struct Skillbook(pub(super) Vec<SkillbookEntry>);

#[allow(unused)]
pub struct Skill {
    pub id: Uuid,
    pub ability: Arc<ObjectTemplate>,
    pub group: String,
    pub state: State,
    pub stance: i32,
}

#[derive(Clone)]
pub struct SkillbookEntry(Arc<Skill>);

impl UserData for SkillbookEntry {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Get", |lua, this, name: String| {
            let val = this.ability.data.get_named::<obj_params::Value>(&name)
                .map_err(mlua::Error::external)?;
        
            ParamValue::new(val.clone())
                .into_lua(lua)
       });
    }
}

impl FromLua for SkillbookEntry {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("object expected"))?;
        Ok(usr.borrow::<SkillbookEntry>()?.clone())
    }
}

impl Deref for SkillbookEntry {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SkillbookEntry {
    pub fn construct_lua_table(&self, runtime: &mut LuaRuntime) -> WorldResult<Table> {
        let base = load_class_script(runtime, 
            self.0.ability.class, 
            self.0.ability.data.get::<_, String>(EdnaAbility::LuaScript).ok().map(|s| s.as_str()))?;

        let metatable = runtime.vm().create_table()?;
        metatable.set("__index", base)?;

        let table = runtime.vm().create_table()?;
        table.set_metatable(Some(metatable));
        table.set("__skill", self.clone())?;

        table.set("instance_guid", self.id.to_string())?;
        table.set("template_guid", self.ability.id.to_string())?;
        table.set("name", self.ability.name.clone())?;
        table.set("class", self.ability.class.name().to_string())?;

        Ok(table)
    }
}

pub struct SkillbookParams {
    pub character_id: Uuid,
    pub level: i32,
    pub combat_style: CombatStyle,
}

impl LoadableComponent for Skillbook {
    type Parameters = SkillbookParams;

    async fn load(Self::Parameters { character_id, level, combat_style }: Self::Parameters, _context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        let mut skillbook = RealmApi::get()
            .get_or_create_skillbook(character_id).await?;

        if skillbook.combat_style != combat_style.into() {
            debug!("Player combat style does not match skillbook");

            if let Err(e) = skillbook.change_class(combat_style.into(), Some(level)).await {
                warn!("Failed to change skillbook: {e:?}");
            }
        } else if skillbook.character_level != level {
            let _ = skillbook.level_up(level).await;
        }

        let _ = skillbook.unlock_all().await;

        let skills = join_all(skillbook.skills.iter()
            .map(async |s| {
                if let Ok(Some(ability)) = ContentCache::get(&ContentCacheRef::Uuid(s.ability_id)).await {
                    Some(SkillbookEntry(Arc::new(Skill {
                        id: s.id,
                        ability,
                        group: s.group.clone(),
                        state: s.state,
                        stance: s.stance,
                    })))
                } else {
                    None
                }
            })
        ).await
        .into_iter()
        .flatten()
        .collect();

        Ok(Self(skills))
    }
}

pub fn network_sync_skillbook(
    mut query: Query<(&mut GameObjectData, &Skillbook), Changed<Skillbook>>,
) {
    for (mut player, skillbook) in query.iter_mut() {
        debug!("Updating skillbook");

        player.set(Player::CurrentClassSkills,
            oaAbilityDataPlayerArray {
                class_hash: 0x81E0A735,
                count: skillbook.0.len() as u32,
                skills: skillbook.0.iter()
                    .map(|s| oaAbilityDataPlayer {
                        version: 0,
                        id: s.id,
                        content_id: s.ability.id,
                        group: s.group.clone(),
                        field_4: s.stance,
                    })
                    .collect(),
            }.to_bytes());
    }
}