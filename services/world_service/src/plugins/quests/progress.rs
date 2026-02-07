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

use bevy::ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld};
use realm_api::{QuestCondition, QuestProgressionState};

use crate::plugins::{ActiveQuest, AutoReturnQuest, Quests};

#[derive(Component)]
#[component(on_insert = on_insert_quest_progress)]
#[component(on_remove = on_remove_quest_progress)]
pub struct QuestProgress(realm_api::QuestState);

impl QuestProgress {
    pub fn from_state(state: realm_api::QuestState) -> Self {
        Self(state)
    }
}

impl QuestProgress {
    pub fn state(&self) -> &realm_api::QuestState {
        &self.0
    }

    pub fn state_mut(&mut self) -> &mut realm_api::QuestState {
        &mut self.0
    }

    pub fn replace(&mut self, new_state: realm_api::QuestState) -> realm_api::QuestState {
        std::mem::replace(&mut self.0, new_state)
    }

    pub fn active_condition(&self) -> Option<&QuestCondition> {
        self.0.conditions
            .iter()
            .find(|condition| condition.current_count < condition.required_count)
    }

    pub fn active_condition_mut(&mut self) -> Option<&mut QuestCondition> {
        self.0.conditions
            .iter_mut()
            .find(|condition| condition.current_count < condition.required_count)
    }
}

fn on_insert_quest_progress(mut world: DeferredWorld, context: HookContext) {
    let Some((id, state)) = world.get::<QuestProgress>(context.entity)
        .map(|progress| (progress.state().quest_id, progress.state().state)) 
    else {
        return;
    };

    let registry = world.resource_ref::<Quests>();

    if 
        state == QuestProgressionState::Active &&
        let Some(quest) = registry.get(&id)
    {
        let obj = quest.obj.clone();
        let template = quest.template.clone();

        if quest.template.completion_dialogue_id.is_none() {
            world
                .commands()
                .entity(context.entity)
                .insert(AutoReturnQuest);
        }

        world
            .commands()
            .entity(context.entity)
            .insert(ActiveQuest {
                obj,
                template,
            });
    }
}

fn on_remove_quest_progress(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .try_remove::<ActiveQuest>();
}