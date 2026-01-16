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

use bevy::{ecs::component::Component, platform::collections::{HashMap, HashSet}};
use realm_api::QuestProgressionState;

use crate::plugins::{Quest, QuestProgress};

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
