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

use bevy::{app::{Plugin, PreUpdate}, ecs::{system::In, world::World}, prelude::{Added, App, Commands, Component, Entity, Query}};
use mlua::{Lua, Table};
use obj_params::{tags::NonClientBaseTag, GameObjectData, NonClientBase};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use anyhow::anyhow;

use crate::error::WorldResult;

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_quest_entities);

        insert_questlog_api(app.world_mut()).unwrap();
    }
}

#[derive(Component)]
pub struct QuestEntity {
    visible_on_available: Vec<i32>,
    visible_on_complete: Vec<i32>,
    visible_on_finished: Vec<i32>,
    visible_on_in_progress: Vec<i32>,
}

impl QuestEntity {
    pub fn is_visible(&self, log: &QuestLog) -> bool {
        for id in &self.visible_on_available {
            if log.available.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_complete {
            if log.completed.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_finished {
            if log.finished.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_in_progress {
            if log.in_progress.contains(id) {
                return true;
            }
        }

        false
    }
}

#[derive(Component, Default)]
pub struct QuestLog {
    available: Vec<i32>,
    completed: Vec<i32>,
    finished: Vec<i32>,
    in_progress: Vec<i32>,
}

fn init_quest_entities(
    objects: Query<(Entity, &GameObjectData), Added<NonClientBaseTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in objects.iter() {
        let quest_entity = QuestEntity {
            visible_on_available: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestAvailable)
                .unwrap_or(&vec![]).clone(),
            visible_on_complete: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestComplete)
                .unwrap_or(&vec![]).clone(),
            visible_on_finished: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestFinished)
                .unwrap_or(&vec![]).clone(),
            visible_on_in_progress: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestInProgress)
                .unwrap_or(&vec![]).clone(),
        };

        if !quest_entity.visible_on_available.is_empty() ||
            !quest_entity.visible_on_complete.is_empty() ||
            !quest_entity.visible_on_finished.is_empty() ||
            !quest_entity.visible_on_in_progress.is_empty()
        {
            commands.entity(ent)
                .insert(quest_entity);
        }
    }
}


#[allow(clippy::type_complexity)]
pub fn insert_questlog_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("questlog", object_api.clone()).unwrap();

    object_api.set("MarkQuestAvailable", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut query: Query<&mut QuestLog>,
    | -> WorldResult<()> {
        if let Ok(mut log) = query.get_mut(owner.entity()?) {
            log.available.push(quest_id);
            log.completed.retain(|&id| id != quest_id);
            log.finished.retain(|&id| id != quest_id);
            log.in_progress.retain(|&id| id != quest_id);

            Ok(())
        } else {
            Err(anyhow!("QuestLog not found").into())
        }
    })?)?;

    object_api.set("MarkQuestCompleted", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut query: Query<&mut QuestLog>,
    | -> WorldResult<()> {
        if let Ok(mut log) = query.get_mut(owner.entity()?) {
            log.available.retain(|&id| id != quest_id);
            log.completed.push(quest_id);
            log.finished.retain(|&id| id != quest_id);
            log.in_progress.retain(|&id| id != quest_id);

            Ok(())
        } else {
            Err(anyhow!("QuestLog not found").into())
        }
    })?)?;

    object_api.set("MarkQuestFinished", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut query: Query<&mut QuestLog>,
    | -> WorldResult<()> {
        if let Ok(mut log) = query.get_mut(owner.entity()?) {
            log.available.retain(|&id| id != quest_id);
            log.completed.retain(|&id| id != quest_id);
            log.finished.push(quest_id);
            log.in_progress.retain(|&id| id != quest_id);

            Ok(())
        } else {
            Err(anyhow!("QuestLog not found").into())
        }
    })?)?;

    object_api.set("MarkQuestInProgress", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut query: Query<&mut QuestLog>,
    | -> WorldResult<()> {
        if let Ok(mut log) = query.get_mut(owner.entity()?) {
            log.available.retain(|&id| id != quest_id);
            log.completed.retain(|&id| id != quest_id);
            log.finished.retain(|&id| id != quest_id);
            log.in_progress.push(quest_id);

            Ok(())
        } else {
            Err(anyhow!("QuestLog not found").into())
        }
    })?)?;

    Ok(())
}