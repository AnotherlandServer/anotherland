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

use bevy::ecs::{entity::Entity, message::MessageReader, query::Added, system::{Commands, In, Query, Res}};
use log::{debug, error, warn};
use mlua::{Function, Table, Value};
use protocol::{OaPktQuestEventEvent, QuestUpdateData, oaPktQuestEvent, oaPktQuestUpdate};
use realm_api::{QuestCondition, QuestProgressionState, RealmApi};
use scripting::{ScriptCommandsExt, ScriptObject};

use crate::{plugins::{AbandonQuest, AcceptQuest, AsyncOperationEntityCommandsExt, DespawnAvatar, PlayerController, QuestLog, QuestRegistry, QuestState, QuestStateUpdated, RequestNextQuest, ReturnQuest, player_error_handler_system}};

pub(super) fn quest_accepter(
    mut events: MessageReader<AcceptQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &AcceptQuest { player, quest_id } in events.read() {
        let Ok((questlog, player_controller)) = players.get(player) else {
            continue;
        };

        let Some(quest) = quests.0.get(&quest_id).cloned() else {
            warn!("Player {} tried to accept unknown quest {}", player, quest_id);
            continue;
        };

        debug!("Player {} is trying to accept quest {}", player, quest_id);

        if questlog.available.contains(&quest_id) {
            let controller = player_controller.clone();

            let mut state = RealmApi::get()
                .create_empty_queststate(controller.character_id(), quest_id, QuestProgressionState::Active);

            // Init quest conditions on accept
            if let Ok(conditions) = quest.table.get::<Table>("conditions") {
                for pairs in conditions.pairs::<Value, Table>() {
                    let Ok((_, condition)) = pairs else {
                        continue;
                    };

                    state.conditions.push(QuestCondition {
                        id: condition.get::<i32>("id").unwrap_or(0),
                        required_count: condition.get::<i32>("required_count").unwrap_or(1),
                        current_count: 0,
                    });
                }
            }

            commands
                .entity(player)
                .perform_async_operation(async move {
                    Ok((
                        quest_id,
                        Some(RealmApi::get().create_queststate(&state).await?)
                    ))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to accept unavailable quest {}", player, quest_id);
        }
    }
}

pub(super) fn quest_returner(
    mut events: MessageReader<ReturnQuest>,
    players: Query<&QuestLog>,
    mut commands: Commands,
) {
    for &ReturnQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest) = questlog.quests.get(&quest_id) &&
            let Some(state) = &quest.state &&
            matches!(state.state, QuestProgressionState::Completed)
        {
            let mut state = state.clone();

            commands
                .entity(player)
                .perform_async_operation(async move {
                    state.update_state(QuestProgressionState::Finished).await?;

                    Ok((
                        quest_id,
                        Some(state)
                    ))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to finish uncompleted quest {}", player, quest_id);
        }
    }
}

pub(super) fn quest_abandoner(
    mut events: MessageReader<AbandonQuest>,
    players: Query<&QuestLog>,
    mut commands: Commands,
) {
    for &AbandonQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest) = questlog.quests.get(&quest_id) &&
            let Some(state) = &quest.state &&
            (
                matches!(state.state, QuestProgressionState::Active) ||
                matches!(state.state, QuestProgressionState::Completed)
            )
        {
            let state = state.clone();

            commands
                .entity(player)
                .perform_async_operation(async move {
                    state.delete().await?;
                    Ok((quest_id, None))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to abandon unstarted quest {}", player, quest_id);
        }
    }
}

pub(super) fn handle_db_quest_update(
    In((player, (quest_id, db_state))): In<(Entity, (i32, Option<realm_api::QuestState>))>,
    mut players: Query<(&ScriptObject, &PlayerController, &mut QuestLog)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    let Ok((script_object, controller, mut quest_log)) = players.get_mut(player) else {
        return;
    };

    let Some(quest_template) = quests.0.get(&quest_id) else {
        error!("Received quest update for unknown quest id: {}", quest_id);
        return;
    };

    if 
        let Some(state) = &db_state &&
        let Some(progress) = quest_log.quests.get_mut(&quest_id) 
    {
        if progress.state.as_ref().map(|s| s.state != state.state).unwrap_or(true) {
            // Quest state changed
            let quest_state = match state.state {
                QuestProgressionState::Active => QuestState::Accepted,
                QuestProgressionState::Completed => QuestState::Completed,
                QuestProgressionState::Finished => QuestState::Finished,
                QuestProgressionState::Failed => QuestState::Failed,
            };

            commands.write_message(QuestStateUpdated {
                player,
                quest_id,
                state: quest_state,
            });
        }

        controller.send_packet(oaPktQuestUpdate {
            player: controller.avatar_id(),
            quest_id: state.quest_id as u32,
            entry_count: state.conditions.len() as u32,
            quest_failed: false,
            accepted_time: state.accepted_time.timestamp_millis(),
            conditions: state.conditions.iter()
                .map(|&QuestCondition { id, current_count, .. }| QuestUpdateData {
                    condition_id: id,
                    count: current_count,
                    ..Default::default()
                }).collect(),
            ..Default::default()
        });

        progress.state = db_state;
        quest_log.update_fast_access_maps();
    } else if 
        db_state.is_none() &&
        let Some(progress) = quest_log.quests.remove(&quest_id) 
    {
        if progress.state.is_some() {
            // Quest was removed from DB, but player had it in progress
            // Mark it as abandoned
            commands.write_message(QuestStateUpdated {
                player,
                quest_id,
                state: QuestState::Abandoned,
            });
        }

        quest_log.update_fast_access_maps();
    }

    let Ok(func) = quest_template.table.get::<Function>("StateUpdated") else {
        error!("Failed to get StateUpdated function for quest: {}", quest_template.id);
        return;
    };

    commands.call_lua_method(
        func,
        (quest_template.table.clone(), script_object.object().clone())
    );
}


pub(super) fn handle_quest_state_changes(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<&ScriptObject>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, quest_id, state } in events.read() {
        let Ok(script_object) = players.get(player) else {
            continue;
        };

        let Some(quest) = quests.0.get(&quest_id) else {
            continue;
        };

        let func_name = match state {
            QuestState::Available => "OnQuestAvailable",
            QuestState::Accepted => "OnQuestAccepted",
            QuestState::Completed => "OnQuestCompleted",
            QuestState::Finished => "OnQuestFinished",
            QuestState::Failed => "OnQuestFailed",
            QuestState::Abandoned => "OnQuestAbandoned",
        };

        if let QuestState::Finished = state {
            commands.write_message(RequestNextQuest { player });
        }

        let Ok(func) = quest.table.get::<Function>(func_name) else {
            continue;
        };

        commands.call_lua_method(
            func, 
            (quest.table.clone(), script_object.object().clone())
        );
    }
}


pub(super) fn attach_active_quests(
    players: Query<(&QuestLog, &ScriptObject), Added<QuestLog>>,
    mut commands: Commands,
) {
    for (quest_log, script_object) in players.iter() {
        for (_, progress) in quest_log.quests.iter() {
            if progress.state.as_ref().map(|s| s.state) == Some(QuestProgressionState::Active) {
                let Ok(attach_quest_to_player) = progress.template.table.get::<Function>("AttachToPlayer") else {
                    error!("Failed to get AttachToPlayer function for quest: {}", progress.template.id);
                    continue;
                };

                // Newly accepted quest, attach to player
                commands.call_lua_method(
                    attach_quest_to_player.clone(),
                    (progress.template.table.clone(), script_object.object().clone())
                );
            }
        }
    }
}

pub(super) fn attach_or_detach_quest_on_state_change(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<(&ScriptObject, &PlayerController)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, quest_id, state } in events.read() {
        let Ok((script_object, _controller)) = players.get(player) else {
            continue;
        };

        let Some(quest_template) = quests.0.get(&quest_id) else {
            error!("Received quest update for unknown quest id: {}", quest_id);
            return;
        };

        match state {
            QuestState::Accepted => {
                let Ok(attach_quest_to_player) = quest_template.table.get::<Function>("AttachToPlayer") else {
                    error!("Failed to get AttachToPlayer function for quest: {}", quest_template.id);
                    return;
                };

                // Newly accepted quest, attach to player
                commands.call_lua_method(
                    attach_quest_to_player.clone(),
                    (quest_template.table.clone(), script_object.object().clone())
                );
            },
            QuestState::Completed | QuestState::Finished | QuestState::Failed | QuestState::Abandoned => {
                let Ok(detach_quest_from_player) = quest_template.table.get::<Function>("DetachFromPlayer") else {
                    error!("Failed to get DetachFromPlayer function for quest: {}", quest_template.id);
                    return;
                };

                // Quest finished/failed/abandoned, detach from player
                commands.call_lua_method(
                    detach_quest_from_player.clone(),
                    (quest_template.table.clone(), script_object.object().clone())
                );
            },
            _ => { /* No action needed for other states */ }
        }
    }
}

pub(super) fn detach_from_despawned_player(
    mut events: MessageReader<DespawnAvatar>,
    players: Query<(&ScriptObject, &QuestLog)>,
    mut commands: Commands,
) {
    for &DespawnAvatar(ent) in events.read() {
        let Ok((script_object, quest_log)) = players.get(ent) else {
            continue;
        };

        quest_log.quests.iter()
            .filter(|(_, progress)| progress.state.as_ref().map(|s| s.state) == Some(QuestProgressionState::Active))
            .for_each(|(_, progress)| {
                let Ok(detach_quest_from_player) = progress.template.table.get::<Function>("DetachFromPlayer") else {
                    error!("Failed to get DetachFromPlayer function for quest: {}", progress.template.id);
                    return;
                };

                // Player despawned, detach active quests
                commands.call_lua_method(
                    detach_quest_from_player.clone(),
                    (progress.template.table.clone(), script_object.object().clone())
                );
            });
    }
}

pub(super) fn sync_quest_state(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<&PlayerController>,
) {
    for &QuestStateUpdated { player, state, quest_id } in events.read() {
        let Ok(player_controller) = players.get(player) else {
            continue;
        };

        match state {
            QuestState::Available => {
                debug!("Player {} updated quest {} to available", player_controller.character_id(), quest_id);
            },
            QuestState::Abandoned => {
                debug!("Player {} abandoned quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestAbandoned,
                    ..Default::default()
                });
            },
            QuestState::Accepted => {
                debug!("Player {} accepted quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestAccepted,
                    ..Default::default()
                });
            },
            QuestState::Failed => {
                debug!("Player {} failed quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestFailed,
                    ..Default::default()
                });
            },
            QuestState::Completed => {
                debug!("Player {} completed quest {}", player_controller.character_id(), quest_id);
            },
            QuestState::Finished => {
                debug!("Player {} finished quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestFinished,
                    ..Default::default()
                });
            },
        }
    }
}
