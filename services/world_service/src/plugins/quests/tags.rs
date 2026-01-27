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

use bevy::ecs::{component::Component, entity::Entity, lifecycle::RemovedComponents, message::MessageReader, query::{Added, Changed, With}, system::{Commands, ParamSet, Query, Res}};
use log::{debug, error, warn};
use mlua::Function;
use obj_params::{GameObjectData, NonClientBase, tags::{NonClientBaseTag, PlayerTag}};
use scripting::{ScriptCommandsExt, ScriptObject};

use crate::plugins::{InterestState, InterestTransmitted, Interests, QuestLog, QuestPlayer, QuestProgress, QuestRegistry, QuestState, QuestStateUpdated};

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

pub(super) fn init_quest_entities(
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
pub(super) fn update_quest_markers(
    mut interest_events: MessageReader<InterestTransmitted>,
    updated_questlogs: Query<Entity, Changed<QuestLog>>,
    players: Query<(&QuestLog, &ScriptObject, &Interests), With<PlayerTag>>,
    entities: Query<&ScriptObject, With<NonClientBaseTag>>,
    mut commands: Commands,
) {
    // Check all interests if questlog changed
    for player_ent in updated_questlogs.iter() {
        let Ok((quest_log, player_script, interests)) = players.get(player_ent) else {
            warn!("Player not yet fully initialized!");
            continue;
        };

        for (_, QuestProgress { template: quest, .. }) in quest_log.quests.iter() {
            if !quest.owned {
                continue;
            }

            let Ok(func) = quest.table.get::<Function>("UpdateQuestMarker") else {
                error!("Failed to get UpdateQuestMarker function for quest: {}", quest.id);
                continue;
            };

            for (&interest, (_, state)) in interests.iter() {
                if !matches!(state, InterestState::Transmitted) {
                    continue;
                }

                let Ok(interest_script) = entities.get(interest) else {
                    continue;
                };

                commands
                    .call_lua_method(func.clone(), (quest.table.clone(), player_script.object().clone(), interest_script.object().clone()));
            }
        }
    }

    // Check newly added interests
    for &InterestTransmitted(player, target) in interest_events.read() {
        let Ok((quest_log, player_script, _)) = players.get(player) else {
            continue;
        };

        let Ok(interest_script) = entities.get(target) else {
            continue;
        };

        for (_, QuestProgress { template: quest, .. }) in quest_log.quests.iter() {
            if !quest.owned {
                continue;
            }

            let Ok(func) = quest.table.get::<Function>("UpdateQuestMarker") else {
                continue;
            };

            commands
                .call_lua_method(func.clone(), (quest.table.clone(), player_script.object().clone(), interest_script.object().clone()));
        }

    }
}

#[allow(clippy::type_complexity)]
pub(super) fn update_available_quests(
    mut events: MessageReader<QuestStateUpdated>,
    mut players: ParamSet<(
        Query<(&mut QuestLog, &ScriptObject)>,
        Query<Entity, Added<QuestLog>>,
    )>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, state, .. } in events.read() {
        if  (
                matches!(state, QuestState::Finished) ||
                matches!(state, QuestState::Completed) ||
                matches!(state, QuestState::Accepted) ||
                matches!(state, QuestState::Abandoned)
            ) &&
            let Ok((mut questlog, script_object)) = players.p0().get_mut(player)
        {
            debug!("Testing available quests for player: {}", player);

            // Clear all available quests
            questlog.available.clear();
            questlog.quests.retain(|_, q| q.state.is_some());

            // Check quest availability
            for (quest_id, quest) in quests.0.iter() {
                if questlog.quests.contains_key(quest_id) || !quest.owned {
                    continue;
                }

                let Ok(func) = quest.table.get::<Function>("MarkAvailableConditional") else {
                    error!("Failed to get MarkAvailableConditional function for quest: {}", quest.id);
                    continue;
                };

                commands.call_lua_method(
                    func, 
                    (quest.table.clone(), script_object.object().clone())
                );
            }
        }
    }

    let newly_loaded: Vec<Entity> = players.p1().iter().collect();
    for player in newly_loaded {
        if let Ok((mut questlog, script_object)) = players.p0().get_mut(player) {
            debug!("Testing available quests for newly loaded questlog: {}", player);

            // Clear all available quests
            questlog.available.clear();
            questlog.quests.retain(|_, q| q.state.is_some());

            // Check quest availability
            for (quest_id, quest) in quests.0.iter() {
                if questlog.quests.contains_key(quest_id) || !quest.owned {
                    continue;
                }

                let Ok(func) = quest.table.get::<Function>("MarkAvailableConditional") else {
                    error!("Failed to get MarkAvailableConditional function for quest: {}", quest.id);
                    continue;
                };

                commands.call_lua_method(
                    func, 
                    (quest.table.clone(), script_object.object().clone())
                );
            }
        }
    }
}

pub(super) fn cleanup_quest_markers(
    mut removed_players: RemovedComponents<PlayerTag>,
    markers: Query<(Entity, &QuestPlayer)>,
    mut commands: Commands,
) {
    for player in removed_players.read() {
        debug!("Cleaning up quest markers for removed player: {}", player);

        for (marker, quest_player) in markers.iter() {
            if quest_player.0 == player {
                commands.entity(marker).despawn();
            }
        }
    }
}