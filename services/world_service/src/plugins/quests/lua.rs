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
use bevy::{app::App, ecs::{hierarchy::{ChildOf, Children}, message::MessageReader, query::With, system::{Commands, In, Query, Res}}};
use mlua::{FromLua, Function, IntoLua, Lua, ObjectLike, Table, Value};
use obj_params::{tags::NonClientBaseTag};
use realm_api::{AvatarSelector, Condition, QuestProgressionState};
use scripting::{LuaEntity, LuaRuntime, LuaScriptReloaded, ScriptAppExt, ScriptCommandsExt};

use crate::{error::{WorldError, WorldResult}, plugins::{AcceptQuest, AttachedQuest, ConditionUpdate, FailQuest, QuestAvailable, QuestConditionUpdate, QuestLog, QuestPlayer, QuestProgress, Quests, ReturnQuest}};

#[derive(Clone)]
pub struct AvatarSelectorLua(AvatarSelector);

impl FromLua for AvatarSelectorLua {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = match &value {
            mlua::Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarSelector".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let filter_type: String = table.get("type")?;
        let filter_value: String = table.get("filter")?;

        match filter_type.as_str() {
            "Content" => Ok(AvatarSelectorLua(
                AvatarSelector::ContentId(filter_value.parse()
                    .map_err(mlua::Error::external)?)
            )),
            "Instance" => Ok(AvatarSelectorLua(
                AvatarSelector::InstanceId(filter_value.parse()
                    .map_err(mlua::Error::external)?)
            )),
            "QuestTag" => Ok(AvatarSelectorLua(
                AvatarSelector::QuestTag(filter_value.parse()
                    .map_err(mlua::Error::external)?)
            )),
            "LootItem" => Ok(AvatarSelectorLua(
                AvatarSelector::LootItem(filter_value.parse()
                    .map_err(mlua::Error::external)?)
            )),
            "Dialog" => Ok(AvatarSelectorLua(
                AvatarSelector::DialogId(filter_value.parse()
                    .map_err(mlua::Error::external)?)
            )),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarSelector".to_string(),
                message: Some(format!("unknown filter type: {}", filter_type)),
            }),
        } 
    }
}

impl IntoLua for AvatarSelectorLua {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        match self.0 {
            AvatarSelector::ContentId(id) => {
                table.set("type", "Content")?;
                table.set("filter", id.to_string())?;
            },
            AvatarSelector::InstanceId(id) => {
                table.set("type", "Instance")?;
                table.set("filter", id.to_string())?;
            },
            AvatarSelector::QuestTag(id) => {
                table.set("type", "QuestTag")?;
                table.set("filter", id.to_string())?;
            },
            AvatarSelector::LootItem(id) => {
                table.set("type", "LootItem")?;
                table.set("filter", id.to_string())?;
            },
            AvatarSelector::DialogId(id) => {
                table.set("type", "Dialog")?;
                table.set("filter", id.to_string())?;
            },
        }
        Ok(mlua::Value::Table(table))
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
pub fn insert_questlog_api(app: &mut App) {
    app
        .add_lua_api("questlog", "FailQuest", 
        |
            In((owner, quest_id)): In<(LuaEntity, i32)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.write_message(FailQuest { 
                player: owner.entity(),
                quest_id,
            });

            Ok(())
        })
        .add_lua_api("questlog", "AcceptQuest", 
        |
            In((owner, quest_id)): In<(LuaEntity, i32)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.write_message(AcceptQuest { 
                player: owner.entity(),
                quest_id,
            });

            Ok(())
        })
        .add_lua_api("questlog", "GetQuestState", 
        |
            In((owner, quest_id)): In<(LuaEntity, i32)>,
            questlogs: Query<&QuestLog>,
            quests: Query<&QuestProgress>,
        | -> WorldResult<LuaQuestState> {
            let questlog = questlogs.get(owner.entity())
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
        })
        .add_lua_api("questlog", "UpdateQuestMarker",
        |
            In((player, target, quest, state)): In<(LuaEntity, LuaEntity, Table, i32)>,
            targets: Query<&Children, With<NonClientBaseTag>>,
            markers: Query<(&AttachedQuest, &QuestPlayer)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            let target_entity = target.entity();

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
                        quest_player.0 == player.entity() && attached_quest.quest_id == quest_id
                    })
                });

            if state == 0 && let Some((entity, _)) = marker {
                commands.entity(entity)
                    .remove::<(QuestAvailable, AttachedQuest)>();
            } else {
                let quest_ent = commands.spawn(
                    (
                        AttachedQuest { quest_id },
                        QuestPlayer(player.entity()),
                        ChildOf(target_entity)
                    )
                ).id();

                if state == 1 {
                    commands.entity(quest_ent)
                        .insert(QuestAvailable);
                }
            }

            Ok(())
        })
        .add_lua_api("questlog", "UpdateQuestProgress", 
        |
            In((player, quest_id, condition_id, action, value)): In<(LuaEntity, i32, i32, String, i32)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands.write_message(QuestConditionUpdate {
                player: player.entity(),
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
        })
        .add_lua_api("questlog", "GetCondition", 
        |
            In((quest, player, condition_id)): In<(Table, LuaEntity, i32)>,
            players: Query<&QuestLog>,
            progress: Query<&QuestProgress>,
            quests: Res<Quests>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<i32> {
            let Ok(quest_id) = quest.get::<i32>("id") else {
                return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
            };

            let Some(quest_def) = quests.get(&quest_id) else {
                return Err(WorldError::Other(anyhow!("Quest with id {} not found", quest_id)));
            };

            let Ok(quest_log) = players.get(player.entity()) else {
                return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
            };

            let Some(quest_ent) = quest_log.quests.get(&quest_id) else {
                return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
            };

            let Ok(progress) = progress.get(*quest_ent) else {
                return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
            };

            let Some(condition) = progress.state().conditions.iter().find(|c| c.id == condition_id) else {
                return Err(WorldError::Other(anyhow!("Quest {} does not have condition {}", quest_id, condition_id)));
            };

            let Some(condition_def) = quest_def.template.conditions.iter().find(|c| c.id() == condition_id) else {
                return Err(WorldError::Other(anyhow!("Quest {} does not have condition {}", quest_id, condition_id)));
            };

            let table = runtime.vm().create_table()?;
            table.set("id", condition.id)?;
            table.set("current_count", condition.current_count)?;
            table.set("required_count", condition.required_count)?;
            
            match condition_def {
                Condition::Interact { avatar_selector, .. } => {
                    table.set("type", "interact")?;
                    table.set("avatar_filter", AvatarSelectorLua(*avatar_selector))?;
                },
                Condition::Dialogue {  dialogue_id, .. } => {
                    table.set("type", "dialogue")?;
                    table.set("avatar_filter", AvatarSelectorLua(AvatarSelector::DialogId(*dialogue_id)))?;
                },
                Condition::Wait { .. } => {
                    table.set("type", "wait")?;
                },
                Condition::Kill { avatar_selector, .. } => {
                    table.set("type", "kill")?;
                    table.set("avatar_filter", AvatarSelectorLua(*avatar_selector))?;
                },
                Condition::Loot { item_name, .. } => {
                    todo!()
                },
                Condition::Proximity { .. } => {
                    table.set("type", "proximity")?;
                },
            }

            Ok(condition.current_count)
        })
        .add_lua_api("questlog", "ReturnQuest",
        |
            In((quest, player)): In<(Table, LuaEntity)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            let Ok(quest_id) = quest.get::<i32>("id") else {
                return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
            };

            commands.write_message(ReturnQuest { player: player.entity(), quest_id });

            Ok(())
        })
        .add_lua_api("questlog", "GetActiveCondition",
        |
            In((quest_id, player)): In<(i32, LuaEntity)>,
            players: Query<&QuestLog>,
            quests: Query<&QuestProgress>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<Value> {
            let Ok(quest_log) = players.get(player.entity()) else {
                return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
            };

            let Some(quest_ent) = quest_log.quests.get(&quest_id) else {
                return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
            };

            let Ok(progress) = quests.get(*quest_ent) else {
                return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
            };

            if let Some(condition) = progress.active_condition() {
                let table = runtime.vm().create_table()?;
                table.set("id", quest_id)?;
                table.set("condition", condition.id)?;
                table.set("current_count", condition.current_count)?;
                table.set("required_count", condition.required_count)?;

                Ok(table.to_value())
            } else {
                Ok(Value::Nil)
            }
        })
        .add_lua_api("questlog", "GetActiveQuests", 
        |
            In(player): In<LuaEntity>,
            players: Query<&QuestLog>,
        | -> WorldResult<Vec<LuaEntity>> {
            let Ok(quest_log) = players.get(player.entity()) else {
                return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
            };

            let quests = quest_log.quests
                .iter()
                .map(|(_, ent)| {
                    LuaEntity(*ent)
                })
                .collect::<Vec<_>>();

            Ok(quests)
        });
}