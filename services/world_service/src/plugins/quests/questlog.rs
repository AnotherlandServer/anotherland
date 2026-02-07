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

use bevy::{ecs::{component::Component, entity::Entity, error::Result, hierarchy::ChildOf, system::EntityCommands}, platform::collections::{HashMap, HashSet}};
use futures::TryStreamExt;
use log::debug;
use realm_api::{QuestProgressionState, RealmApi};
use toolkit::types::Uuid;

use crate::plugins::{LoadContext, LoadableComponent, QuestProgress, QuestState, UpdateAvailableQuests, quests::network::QuestGiverStatus};

#[derive(Component, Default)]
pub struct QuestLog {
    pub available: HashSet<i32>,
    pub completed: HashSet<i32>,
    pub finished: HashSet<i32>,
    pub in_progress: HashSet<i32>,

    pub quests: HashMap<i32, Entity>,
}

impl LoadableComponent for QuestLog {
    type Parameters = Uuid;
    type ContextData = Vec<realm_api::QuestState>;

    async fn load(character_id: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        let mut res = RealmApi::get()
            .query_quest_states()
            .character_id(character_id)
            .query()
            .await?;

        let mut states = vec![];
        let mut questlog = QuestLog::default();

        while let Some(state) = res.try_next().await? {
            match state.state {
                QuestProgressionState::Active => {
                    questlog.in_progress.insert(state.quest_id);
                },
                QuestProgressionState::Completed => {
                    questlog.completed.insert(state.quest_id);
                },
                QuestProgressionState::Finished => {
                    questlog.finished.insert(state.quest_id);
                },
                QuestProgressionState::Failed => {
                    // Failed quests are not tracked in fast access maps
                }
            }

            states.push(state);
        }

        context.set_data(states);

        Ok(questlog)
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, data: Option<Self::ContextData>) -> Result<()> {
        let ent = commands.id();

        commands
            .insert(QuestGiverStatus::default());

        for state in data.unwrap().drain(..) {
            let id = state.quest_id;
            let player = commands.id();
            let ent = 
                commands
                    .commands()
                    .spawn((
                        ChildOf(player),
                        QuestProgress::from_state(state)
                    ))
                    .id();

            debug!("Inserted quest progress for quest {} into questlog", id);

            self.quests.insert(id, ent);
        }

        commands
            .commands()
            .write_message(UpdateAvailableQuests(ent));
        
        Ok(())
    }
}

impl QuestLog {
    pub fn clear_available(&mut self) {
        self.available.clear();
    }

    pub fn mark_available(&mut self, id: i32) {
        self.available.insert(id);
    }

    pub fn update_state(&mut self, id: i32, state: QuestState) {
        self.available.remove(&id);
        self.in_progress.remove(&id);
        self.completed.remove(&id);
        self.finished.remove(&id);

        match state {
            QuestState::Accepted => {
                self.in_progress.insert(id);
            },
            QuestState::Completed => {
                self.completed.insert(id);
            },
            QuestState::Finished => {
                self.finished.insert(id);
            },
            QuestState::Abandoned | QuestState::Failed => {
                // untracked
            }
        }
    }
}