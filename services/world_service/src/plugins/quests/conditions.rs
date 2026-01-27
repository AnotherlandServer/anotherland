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

use bevy::ecs::{entity::Entity, message::{Message, MessageReader}, system::{Commands, Query}};

use crate::plugins::{AsyncOperationEntityCommandsExt, QuestLog, player_error_handler_system, quests::handle_db_quest_update};

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
    mut commands: Commands,
) {
    for &QuestConditionUpdate { player, quest_id, condition_id, update } in events.read() {
        let Ok(quest_log) = players.get(player) else {
            continue;
        };

        let Some(mut quest_state) = quest_log.quests.get(&quest_id).and_then(|q| q.state.clone()) else {
            continue;
        };

        commands
            .entity(player)
            .perform_async_operation(async move {
                quest_state.update_condition(condition_id, 
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