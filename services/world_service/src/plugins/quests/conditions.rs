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

use bevy::ecs::{entity::Entity, hierarchy::ChildOf, message::{Message, MessageReader}, query::{Changed, With, Without}, system::{Commands, Query, Res}};
use chrono::Utc;
use log::debug;
use obj_params::{GameObjectData, Player, tags::{ItemBaseTag, PlayerTag}};
use realm_api::Condition;
use spart::kd_tree;

use crate::plugins::{ActiveQuest, AsyncOperationEntityCommandsExt, AvatarKilled, AvatarSelectorMatcher, ContentInfo, DialogueFinished, Interaction, InteractionEvent, Interests, Inventory, Movement, QuestLog, QuestProgress, QuestStatePending, Quests, player_error_handler_system, quests::handle_db_quest_update};

#[derive(Message, Clone, Copy)]
pub struct QuestConditionUpdate {
    pub player: Entity,
    pub quest_id: i32,
    pub condition_id: i32,
    pub update: ConditionUpdate,
}

#[derive(Clone, Copy)]
pub enum ConditionUpdate {
    Added(i32),
    Removed(i32),
    Set(i32),
}

pub(super) fn handle_quest_condition_update(
    mut events: MessageReader<QuestConditionUpdate>,
    players: Query<&QuestLog>,
    quests: Query<&QuestProgress>,
    mut commands: Commands,
) {
    for &QuestConditionUpdate { player, quest_id, condition_id, update } in events.read() {
        let Ok(quest_log) = players.get(player) else {
            continue;
        };

        let Some(quest_ent) = quest_log.quests.get(&quest_id) else {
            continue;
        };

        let Ok(mut quest_state) = quests.get(*quest_ent).map(|q| q.state().clone()) else {
            continue;
        };

        let Some((condition_idx, _)) = quest_state.conditions.iter().enumerate().find(|(_, c)| c.id == condition_id) else {
            continue;
        };

        commands
            .entity(*quest_ent)
            .insert(QuestStatePending);

        commands
            .entity(player)
            .perform_async_operation(async move {
                quest_state.update_condition(condition_idx as i32, 
                    match update {
                        ConditionUpdate::Added(_) => realm_api::ConditionUpdate::Increment,
                        ConditionUpdate::Removed(_) => realm_api::ConditionUpdate::Increment,
                        ConditionUpdate::Set(_) => realm_api::ConditionUpdate::Set,
                    },
                    match update {
                        ConditionUpdate::Added(v) => v,
                        ConditionUpdate::Removed(v) => -v,
                        ConditionUpdate::Set(v) => v,
                    }).await?;

                Ok((quest_id, Some(quest_state)))
            })
            .on_finish_run_system(handle_db_quest_update)
            .on_error_run_system(player_error_handler_system);
    }
}

pub(super) fn interaction_event_listener(
    mut events: MessageReader<InteractionEvent>,
    players: Query<&QuestLog>,
    targets: Query<(&ContentInfo, &GameObjectData)>,
    active_quests: Query<&QuestProgress, With<ActiveQuest>>,
    quests: Res<Quests>,
    mut commands: Commands,
) {
    for &InteractionEvent { source, target, interaction } in events.read() {
        if !matches!(interaction, Interaction::CastComplete) {
            continue;
        }

        let Ok(quest_log) = players.get(source) else {
            continue;
        };

        let Ok((target_info, target_data)) = targets.get(target) else {
            continue;
        };

        for quest_ent in quest_log.quests.values() {
            let Ok(quest_progress) = active_quests.get(*quest_ent) else {
                continue;
            };

            let Some(quest) = quests.get(&quest_progress.state().quest_id) else {
                continue;
            };

            let Some(condition) = quest_progress.active_condition() else {
                continue;
            };

            let Some(Condition::Interact { avatar_selector, .. }) = quest.template.conditions
                .iter()
                .find(|tpl| tpl.id() == condition.id) else {
                continue;
            };

            if avatar_selector.matches(target_info, target_data) {
                commands
                    .write_message(QuestConditionUpdate {
                        player: source,
                        quest_id: quest_progress.state().quest_id,
                        condition_id: condition.id,
                        update: ConditionUpdate::Added(1),
                    });
            }
        }
    }
}

pub fn update_passive_conditions(
    progress: Query<(Entity, &ChildOf, &QuestProgress), (With<ActiveQuest>, Without<QuestStatePending>)>,
    player: Query<(&Movement, &Interests)>,
    targets: Query<(&ContentInfo, &GameObjectData, &Movement)>,
    quests: Res<Quests>,
    mut commands: Commands,
) {
    for (quest_ent, child_of, progress) in progress.iter() {
        let Some(condition) = progress.active_condition() else {
            continue;
        };

        let Some(quest) = quests.get(&progress.state().quest_id) else {
            continue;
        };

        let Some(condition_tpl) = quest.template.conditions.iter().find(|c| c.id() == condition.id) else {
            continue;
        };

        match *condition_tpl {
            Condition::Wait { id, wait_time_seconds, .. } => {
                if
                    Utc::now()
                        .signed_duration_since(progress.state().last_condition_update)
                        .as_seconds_f64() >= wait_time_seconds
                {
                    commands
                        .entity(quest_ent)
                        .insert(QuestStatePending);

                    commands
                        .write_message(QuestConditionUpdate {
                            player: child_of.parent(),
                            quest_id: progress.state().quest_id,
                            condition_id: id,
                            update: ConditionUpdate::Added(1),
                        });
                }
            },
            Condition::Proximity { avatar_selector, radius, .. } => {
                let Ok((player_movement, interests)) = player.get(child_of.parent()) else {
                    continue;
                };

                for ent in interests.collection().keys() {
                    let Ok((target_info, target_data, target_movement)) = targets.get(*ent) else {
                        continue;
                    };

                    if 
                        avatar_selector.matches(target_info, target_data) &&
                        player_movement.position.distance(target_movement.position) <= radius as f32
                    {
                        commands
                            .entity(quest_ent)
                            .insert(QuestStatePending);

                        commands
                            .write_message(QuestConditionUpdate {
                                player: child_of.parent(),
                                quest_id: progress.state().quest_id,
                                condition_id: condition.id,
                                update: ConditionUpdate::Added(1),
                            });
                    }
                }
            },
            _ => {}
        }

    }
}

pub fn update_dialogue_conditions(
    mut events: MessageReader<DialogueFinished>,
    quests: Res<Quests>,
    players: Query<&QuestLog>,
    active_quests: Query<&QuestProgress, With<ActiveQuest>>,
    mut commands: Commands,
) {
    for &DialogueFinished { player, dialogue_id } in events.read() {
        let Ok(quest_log) = players.get(player) else {
            continue;
        };

        for quest_ent in quest_log.quests.values() {
            let Ok(quest_progress) = active_quests.get(*quest_ent) else {
                continue;
            };

            if 
                let Some(condition) = quest_progress.active_condition() && 
                let Some(quest) = quests.get(&quest_progress.state().quest_id) &&
                let Some(Condition::Dialogue { dialogue_id: cond_dialogue_id, .. }) = quest.template.conditions.iter().find(|c| c.id() == condition.id) &&
                *cond_dialogue_id == dialogue_id
            {
                commands
                    .write_message(QuestConditionUpdate {
                        player,
                        quest_id: quest_progress.state().quest_id,
                        condition_id: condition.id,
                        update: ConditionUpdate::Added(1),
                    });
            }
        }
    }
}

pub fn update_kill_conditions(
    mut events: MessageReader<AvatarKilled>,
    quests: Res<Quests>,
    players: Query<&QuestLog>,
    active_quests: Query<&QuestProgress, With<ActiveQuest>>,
    targets: Query<(&ContentInfo, &GameObjectData)>,
    mut commands: Commands,
) {
    for &AvatarKilled { entity: killed_entity, killer } in events.read() {
        let Some(killer) = killer else {
            continue;
        };

        let Ok(quest_log) = players.get(killer) else {
            continue;
        };

        let Ok((killed_info, killed_data)) = targets.get(killed_entity) else {
            continue;
        };

        for quest_ent in quest_log.quests.values() {
            let Ok(quest_progress) = active_quests.get(*quest_ent) else {
                continue;
             };

            let Some(condition) = quest_progress.active_condition() else {
                continue;
            };

            let Some(quest) = quests.get(&quest_progress.state().quest_id) else {
                continue;
            };

            let Some(Condition::Kill { avatar_selector, .. }) = quest.template.conditions.iter().find(|c| c.id() == condition.id) else {
                continue;
            };

            if avatar_selector.matches(killed_info, killed_data) {
                commands
                    .entity(*quest_ent)
                    .insert(QuestStatePending);

                commands
                    .write_message(QuestConditionUpdate {
                        player: killer,
                        quest_id: quest_progress.state().quest_id,
                        condition_id: condition.id,
                        update: ConditionUpdate::Added(1),
                    });
            }
        }
    }
}

pub fn update_loot_conditions(
    players: Query<(Entity, &Inventory, &QuestLog), Changed<Inventory>>,
    items: Query<&ContentInfo>,
    progress: Query<(&ActiveQuest, &QuestProgress)>,
    mut commands: Commands,
) {
    for (entity, inventory, questlog) in players.iter() {
        for quest_ent in questlog.quests.values() {
            let Ok((active_quest, quest_progress)) = progress.get(*quest_ent) else {
                continue;
            };

            let Some(condition) = quest_progress.active_condition() else {
                continue;
            };

            let Some(Condition::Loot { item_name, .. }) = active_quest.template.conditions.iter()
                .find(|c| c.id() == condition.id) 
            else {
                continue;
            };

            // Count items in inventory
            let mut item_count = 0;

            for item in inventory.items.values() {
                let Ok(item_info) = items.get(*item) else {
                    continue;
                };

                debug!("Checking item {:?} for loot condition, looking for {item_name}", item_info.template.name);

                if &item_info.template.name == item_name {
                    item_count += 1;
                }
            }

            if item_count != condition.current_count {
                commands
                    .entity(*quest_ent)
                    .insert(QuestStatePending);

                commands
                    .write_message(QuestConditionUpdate {
                        player: entity,
                        quest_id: quest_progress.state().quest_id,
                        condition_id: condition.id,
                        update: ConditionUpdate::Set(item_count),
                    });
            }
        }
    }
}