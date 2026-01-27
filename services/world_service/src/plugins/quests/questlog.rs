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

use std::sync::Arc;

use bevy::{ecs::{component::Component, entity::Entity, query::{Added, With}, system::{Commands, In, Query, Res}}, platform::collections::{HashMap, HashSet}};
use futures::TryStreamExt;
use obj_params::tags::PlayerTag;
use realm_api::{QuestProgressionState, RealmApi};
use scripting::ScriptObject;

use crate::plugins::{AsyncOperationEntityCommandsExt, PlayerController, Quest, QuestProgress, QuestRegistry, player_error_handler_system};

#[derive(Component, Default)]
pub struct QuestLog {
    // Quests ids per status, for fast access when determining interests
    pub available: HashSet<i32>,
    pub completed: HashSet<i32>,
    pub finished: HashSet<i32>,
    pub in_progress: HashSet<i32>,

    pub quests: HashMap<i32, QuestProgress>,
}

impl QuestLog {
    pub fn mark_available(&mut self, quest: Arc<Quest>) {
        let id = quest.id;
        self.quests.entry(id).or_insert(QuestProgress {
            template: quest,
            state: None,
        });
        self.update_fast_access_maps();
    }

    pub fn update_fast_access_maps(&mut self) {
        self.available.clear();
        self.completed.clear();
        self.finished.clear();
        self.in_progress.clear();

        for (&id, progress) in &self.quests {
            match progress.state.as_ref().map(|s| s.state) {
                Some(QuestProgressionState::Active) => {
                    self.in_progress.insert(id);
                }
                Some(QuestProgressionState::Completed) => {
                    self.completed.insert(id);
                }
                Some(QuestProgressionState::Finished) => {
                    self.finished.insert(id);
                }
                Some(QuestProgressionState::Failed) => {
                    // Failed quests are not tracked in fast access maps
                }
                None => {
                    self.available.insert(id);
                }
            }
        }
    }
}

pub(super) fn load_questlogs_for_joined_players(
    players: Query<(Entity, &PlayerController), (Added<ScriptObject>, With<PlayerTag>)>,
    mut commands: Commands,
) {
    for (entity, controller) in players.iter() {
        let controller = controller.clone();

        commands
            .entity(entity)
            .perform_async_operation(async move {
                let mut res = RealmApi::get()
                    .query_quest_states()
                    .character_id(controller.character_id())
                    .query()
                    .await?;

                let mut quests = vec![];

                while let Some(quest_state) = res.try_next().await? {
                    quests.push(quest_state);
                }

                Ok(quests)
            })
            .on_finish_run_system(insert_questlog_for_player)
            .on_error_run_system(player_error_handler_system);
    }
}

pub(super) fn insert_questlog_for_player(
    In((entity, quests)): In<(Entity, Vec<realm_api::QuestState>)>,
    quest_registry: Res<QuestRegistry>,
    mut commands: Commands,
) {
    let mut quest_log = QuestLog {
        quests: quests.into_iter()
            .filter_map(|q| {
                let quest = quest_registry.0.get(&q.quest_id)?.clone();
                    Some((q.quest_id, QuestProgress {
                        template: quest,
                        state: Some(q),
                    }))
                })
                .collect(),
            ..Default::default()
    };

    quest_log.update_fast_access_maps();

    commands.entity(entity)
        .insert(quest_log);
}