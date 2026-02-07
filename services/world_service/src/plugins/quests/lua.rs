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

use std::ops::Deref;

use anyhow::anyhow;
use bevy::ecs::{hierarchy::{ChildOf, Children}, message::MessageReader, query::With, system::{Commands, In, Query, Res}, world::World};
use mlua::{FromLua, Function, IntoLua, Lua, Table};
use obj_params::{tags::NonClientBaseTag};
use protocol::AvatarFilter;
use realm_api::QuestProgressionState;
use scripting::{LuaExt, LuaRuntime, LuaScriptReloaded, LuaTableExt, ScriptCommandsExt, ScriptResult};

use crate::{error::{WorldError, WorldResult}, plugins::{AttachedQuest, ConditionUpdate, FailQuest, QuestAvailable, QuestConditionUpdate, QuestLog, QuestPlayer, QuestProgress, Quests, ReturnQuest}};

#[derive(Clone)]
pub struct AvatarFilterLua(protocol::AvatarFilter);

impl Deref for AvatarFilterLua {
    type Target = protocol::AvatarFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromLua for AvatarFilterLua {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = match &value {
            mlua::Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarFilter".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let filter_type: String = table.get("type")?;
        let filter_value: String = table.get("filter")?;

        let kind = match filter_type.as_str() {
            "Content" => 1,
            "Instance" => 2,
            "QuestTags" => 3,
            "LootItem" => 4,
            "Dialog" => 5,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarFilter".to_string(),
                message: Some(format!("unknown filter type: {}", filter_type)),
            }),
        };

        Ok(AvatarFilterLua(AvatarFilter {
            kind,
            filter: filter_value,
        }))
    }
}

impl IntoLua for AvatarFilterLua {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("type", match self.0.kind {
            1 => "Content",
            2 => "Instance",
            3 => "QuestTags",
            4 => "LootItem",
            5 => "Dialog",
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: "AvatarFilter",
                to: "table".to_string(),
                message: Some(format!("unknown filter kind: {}", self.0.kind)),
            }),
        })?;
        table.set("filter", self.0.filter)?;
        Ok(mlua::Value::Table(table))
    }
}

impl From<AvatarFilterLua> for AvatarFilter {
    fn from(value: AvatarFilterLua) -> Self {
        value.0
    }
}

pub fn hot_reload_quests(
    mut events: MessageReader<LuaScriptReloaded>,
    quests: Res<Quests>,
    mut commands: Commands,
) {
    for _ in events.read() {
        for quest in quests.values() {
            if 
                let Ok(true) = quest.obj.get::<bool>("__hot_reload") &&
                let Ok(func) = quest.obj.get::<Function>("HotReload")
            {
                    commands
                        .call_lua_method(func, quest.obj.clone());
                }
            }
    }
}


enum LuaQuestState {
    Available,
    InProgress,
    Completed,
    Finished,
    Unavailable,
}

impl IntoLua for LuaQuestState {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        match self {
            LuaQuestState::Available => "AVAILABLE".into_lua(lua),
            LuaQuestState::InProgress => "IN_PROGRESS".into_lua(lua),
            LuaQuestState::Completed => "COMPLETED".into_lua(lua),
            LuaQuestState::Finished => "FINISHED".into_lua(lua),
            LuaQuestState::Unavailable => "UNAVAILABLE".into_lua(lua),
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn insert_questlog_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let quest_api = lua.create_table().unwrap();
    runtime.register_native("questlog", quest_api.clone()).unwrap();

    quest_api.set("FailQuest", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        commands.write_message(FailQuest { 
            player: owner.entity()?,
            quest_id,
        });

        Ok(())
    })?)?;

    quest_api.set("GetQuestState", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        questlogs: Query<&QuestLog>,
        quests: Query<&QuestProgress>,
    | -> WorldResult<LuaQuestState> {
        let questlog = questlogs.get(owner.entity()?)
            .map_err(|e| WorldError::Other(anyhow!("Failed to get quest log: {}", e)))?;

        let Some(quest_ent) = questlog.quests.get(&quest_id) else {
            if questlog.available.contains(&quest_id) {
                return Ok(LuaQuestState::Available);
            } else {
                return Ok(LuaQuestState::Unavailable);
            }
        };

        let Ok(progress) = quests.get(*quest_ent) else {
            return Err(WorldError::Other(anyhow!("Failed to get quest progress")));
        };

        match progress.state().state {
            QuestProgressionState::Active => Ok(LuaQuestState::InProgress),
            QuestProgressionState::Completed => Ok(LuaQuestState::Completed),
            QuestProgressionState::Finished => Ok(LuaQuestState::Finished),
            QuestProgressionState::Failed => Ok(LuaQuestState::Unavailable),
        }
    })?)?;

    quest_api.set("UpdateQuestMarker", lua.create_bevy_function(world, |
        In((player_tbl, target_tbl, quest, state)): In<(Table, Table, Table, i32)>,
        targets: Query<&Children, With<NonClientBaseTag>>,
        markers: Query<(&AttachedQuest, &QuestPlayer)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        let target_entity = target_tbl.entity()?;

        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Failed to get quest id")));
        };

        let marker = 
            targets.get(target_entity)
            .ok()
            .and_then(|children| {
                children.iter()
                .filter_map(|&e| Some((e, markers.get(e).ok()?)))
                .find(|(_, (attached_quest, quest_player))| {
                    Some(quest_player.0) == player_tbl.entity().ok() && attached_quest.quest_id == quest_id
                })
            });

        if state == 0 && let Some((entity, _)) = marker {
            commands.entity(entity)
                .remove::<(QuestAvailable, AttachedQuest)>();
        } else {
            let quest_ent = commands.spawn(
                (
                    AttachedQuest { quest_id },
                    QuestPlayer(player_tbl.entity()?),
                    ChildOf(target_entity)
                )
            ).id();

            if state == 1 {
                commands.entity(quest_ent)
                    .insert(QuestAvailable);
            }
        }

        Ok(())
    })?)?;

    quest_api.set("UpdateQuestProgress", lua.create_bevy_function(world, |
        In((player_obj, quest_id, condition_id, action, value)): In<(Table, i32, i32, String, i32)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        commands.write_message(QuestConditionUpdate {
            player: player_obj.entity()?,
            quest_id,
            condition_id,
            update: match action.as_str() {
                "ADD" => ConditionUpdate::Added(value),
                "REMOVE" => ConditionUpdate::Removed(value),
                "SET" => ConditionUpdate::Set(value),
                _ => return Err(WorldError::Other(anyhow!("Unknown action: {}", action))),
            },
        });

        Ok(())
    })?)?;

    quest_api.set("GetConditionProgress", lua.create_bevy_function(world, |
        In((quest, player, condition_id)): In<(Table, Table, i32)>,
        players: Query<&QuestLog>,
        quests: Query<&QuestProgress>,
    | -> WorldResult<i32> {
        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
        };

        let Ok(quest_log) = players.get(player.entity()?) else {
            return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
        };

        let Some(quest_ent) = quest_log.quests.get(&quest_id) else {
            return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
        };

        let Ok(progress) = quests.get(*quest_ent) else {
            return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
        };

        let Some(condition) = progress.state().conditions.iter().find(|c| c.id == condition_id) else {
            return Err(WorldError::Other(anyhow!("Quest {} does not have condition {}", quest_id, condition_id)));
        };

        Ok(condition.current_count)
    })?)?;

    quest_api.set("GetLastConditionUpdateTime", lua.create_bevy_function(world, |
        In((quest, player)): In<(Table, Table)>,
        players: Query<&QuestLog>,
        quests: Query<&QuestProgress>,
    | -> WorldResult<i64> {
        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
        };

        let Ok(quest_log) = players.get(player.entity()?) else {
            return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
        };

        let Some(quest_ent) = quest_log.quests.get(&quest_id) else {
            return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
        };

        let Ok(progress) = quests.get(*quest_ent) else {
            return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
        };

        Ok(progress.state().last_condition_update.timestamp())
    })?)?;

    quest_api.set("ReturnQuest", lua.create_bevy_function(world, |
        In((quest, player)): In<(Table, Table)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
        };

        commands.write_message(ReturnQuest { player: player.entity()?, quest_id });

        Ok(())
    })?)?;

    Ok(())
}