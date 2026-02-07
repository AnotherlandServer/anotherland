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

use anyhow::anyhow;
use bevy::{ecs::{entity::Entity, hierarchy::ChildOf, message::MessageReader, query::{With, Without}, system::{Commands, In, Query, Res}}};
use log::{debug, warn};
use mlua::Function;
use obj_params::{GameObjectData, Player};
use protocol::{OaPktQuestEventEvent, QuestUpdateData, oaPktQuestEvent, oaPktQuestUpdate};
use realm_api::{Condition, QuestCondition, QuestProgressionState, RealmApi};
use scripting::{ScriptCommandsExt, ScriptObject};

use crate::plugins::{AbandonQuest, AcceptQuest, AsyncOperationEntityCommandsExt, AutoReturnQuest, PlayerController, Quest, QuestLog, QuestProgress, QuestState, QuestStatePending, QuestStateUpdated, Quests, ReturnQuest, RunDeferredQuestDialogues, UpdateAvailableQuests, WeakCache, player_error_handler_system, quests::cache::QuestTemplateCache};

pub(super) fn quest_accepter(
    mut events: MessageReader<AcceptQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    mut commands: Commands,
) {
    for &AcceptQuest { player, quest_id } in events.read() {
        let Ok((questlog, player_controller)) = players.get(player) else {
            continue;
        };

        let controller = player_controller.clone();

        if questlog.available.contains(&quest_id) {
            commands
                .entity(player)
                .perform_async_operation(async move {
                    let Some(quest) = QuestTemplateCache::get(&quest_id).await? else {
                        return Err(anyhow!("Quest with ID {} not found", quest_id).into());
                    };

                    debug!("Player {} is trying to accept quest {}", player, quest_id);
                
                    let mut state = RealmApi::get()
                        .create_empty_queststate(controller.character_id(), quest_id, QuestProgressionState::Active);

                    // Init quest conditions on accept
                    for condition in quest.conditions.iter() {
                        state.conditions.push(
                        match *condition {
                            Condition::Interact { id, required_count, .. } => 
                                QuestCondition { id, current_count: 0, required_count },
                            Condition::Dialogue { id, required_count, .. } => 
                                QuestCondition { id, current_count: 0, required_count },
                            Condition::Wait { id, .. } => 
                                QuestCondition { id, current_count: 0, required_count: 1 },
                            Condition::Kill { id, required_count, .. } => 
                                QuestCondition { id, current_count: 0, required_count },
                            Condition::Loot { id, required_count, .. } => 
                                QuestCondition { id, current_count: 0, required_count },
                        });
                    }

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
    quests: Query<&QuestProgress>,
    mut commands: Commands,
) {
    for &ReturnQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest_ent) = questlog.quests.get(&quest_id) &&
            let Ok(progress) = quests.get(*quest_ent) &&
            matches!(progress.state().state, QuestProgressionState::Completed)
        {
            let mut state = progress.state().clone();

            commands
                .entity(*quest_ent)
                .insert(QuestStatePending);

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
    quests: Query<&QuestProgress>,
    mut commands: Commands,
) {
    for &AbandonQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest_ent) = questlog.quests.get(&quest_id) &&
            let Ok(progress) = quests.get(*quest_ent) &&

            (
                matches!(progress.state().state, QuestProgressionState::Active) ||
                matches!(progress.state().state, QuestProgressionState::Completed)
            )
        {
            commands
                .entity(*quest_ent)
                .insert(QuestStatePending);

            let state = progress.state().clone();
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
    mut players: Query<(Entity, &PlayerController, &mut QuestLog)>,
    mut quests: Query<&mut QuestProgress>,
    mut commands: Commands,
) {
    let Ok((player_ent, controller, mut quest_log)) = players.get_mut(player) else {
        return;
    };

    if let Some(state) = &db_state {
        if
            let Some(quest_ent) = quest_log.quests.get_mut(&quest_id) &&
            let Ok(mut progress) = quests.get_mut(*quest_ent)
        {
            if progress.state().state != state.state {
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

            progress.replace(state.clone());

            commands
                .entity(*quest_ent)
                .remove::<QuestStatePending>();
        } else {
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

            let quest_ent = commands
                .spawn((
                    ChildOf(player_ent),
                    QuestProgress::from_state(state.clone()),
                ))
                .id();

            quest_log.quests.insert(quest_id, quest_ent);
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
    } else if 
        db_state.is_none() &&
        let Some(quest_ent) = quest_log.quests.remove(&quest_id)
    {
        commands
            .entity(quest_ent)
            .despawn();

        // Quest was removed from DB, but player had it in progress
        // Mark it as abandoned
        commands.write_message(QuestStateUpdated {
            player,
            quest_id,
            state: QuestState::Abandoned,
        });
    }

    commands
        .write_message(RunDeferredQuestDialogues{
            player
        });
}


pub(super) fn handle_quest_state_changes(
    mut events: MessageReader<QuestStateUpdated>,
    mut players: Query<(&ScriptObject, &mut QuestLog)>,
    quests: Res<Quests>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, quest_id, state } in events.read() {
        let Ok((script_object, mut quest_log)) = players.get_mut(player) else {
            continue;
        };

        quest_log.update_state(quest_id, state);
        commands
            .write_message(UpdateAvailableQuests(player));

        let Some(quest) = quests.get(&quest_id) else {
            continue;
        };

        let func_name = match state {
            QuestState::Accepted => "OnQuestAccepted",
            QuestState::Completed => "OnQuestCompleted",
            QuestState::Finished => "OnQuestFinished",
            QuestState::Failed => "OnQuestFailed",
            QuestState::Abandoned => "OnQuestAbandoned",
        };

        let Ok(func) = quest.obj.get::<Function>(func_name) else {
            continue;
        };

        commands.call_lua_method(
            func, 
            (quest.obj.clone(), script_object.object().clone())
        );
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

pub(super) fn update_available_quests(
    mut events: MessageReader<UpdateAvailableQuests>,
    quests: Res<Quests>,
    mut players: Query<(&GameObjectData, &mut QuestLog)>,
) {
    for &UpdateAvailableQuests(player) in events.read() {
        let Ok((object, mut quest_log)) = players.get_mut(player) else {
            continue;
        };

        quest_log.clear_available();

        for quest in quests.values() {
            // Skip if quest is already active in any way. 
            if quest_log.quests.contains_key(&quest.id) {
                debug!("Player {} already has quest {} in progress, skipping availability check", player, quest.id);
                continue;
            }

            if 
                verify_combat_style_prerequisite(quest, object) &&
                verify_level_prerequisite(quest, object) &&
                verify_quests_finished_prerequisite(quest, &quest_log)
            {
                debug!("Marking quest {} as available for player {}", quest.id, player);

                // If all preconditions are met, mark quest as available
                quest_log.mark_available(quest.id);
            } else {
                debug!("Quest {} is not available for player {}", quest.id, player);
            }
        }
    }

}

fn verify_combat_style_prerequisite(quest: &Quest, object: &GameObjectData) -> bool {
    if 
        let Some(prerequisites) = &quest.template.prerequisites &&
        let Some(required_style) = prerequisites.combat_style
    {
        if let Ok(&player_combat_style) = object.get::<_, i32>(Player::CombatStyle) {
            return player_combat_style == i32::from(required_style);
        } else {
            return false;
        }
    }

    true
}

fn verify_level_prerequisite(quest: &Quest, object: &GameObjectData) -> bool {
    if 
        let Some(prerequisites) = &quest.template.prerequisites &&
        let Some(level) = prerequisites.level
    {
        if let Ok(&player_level) = object.get::<_, i32>(Player::Lvl) {
            return player_level >= level;
        } else {
            return false;
        }
    }

    true
}

fn verify_quests_finished_prerequisite(quest: &Quest, quest_log: &QuestLog) -> bool {
    if 
        let Some(prerequisites) = &quest.template.prerequisites &&
        let Some(required_finished_quests) = &prerequisites.quests_finished
    {
        for &required_quest_id in required_finished_quests.iter() {
            if !quest_log.finished.contains(&required_quest_id) {
                return false;
            }
        }
    }

    true
}

pub fn auto_return_quests(
    progress: Query<(&ChildOf, &QuestProgress), (With<AutoReturnQuest>, Without<QuestStatePending>)>,
    mut commands: Commands,
) {
    for (child_of, progress) in progress.iter() {
        if matches!(progress.state().state, QuestProgressionState::Completed) {
            commands
                .write_message(ReturnQuest {
                    player: child_of.parent(),
                    quest_id: progress.state().quest_id
                });
        }
    }
}