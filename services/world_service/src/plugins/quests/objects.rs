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

use bevy::ecs::{component::Component, entity::Entity, query::Added, system::{Commands, Query, Res}};
use log::debug;
use obj_params::{GameObjectData, NonClientBase, tags::NonClientBaseTag};
use realm_api::QuestTemplate;

use crate::plugins::{QuestLog, Quests};

#[derive(Component)]
pub struct QuestVisibility {
    visible_on_available: Vec<i32>,
    visible_on_complete: Vec<i32>,
    visible_on_finished: Vec<i32>,
    visible_on_in_progress: Vec<i32>,
}

impl QuestVisibility {
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

pub(super) fn init_quest_visibility(
    objects: Query<(Entity, &GameObjectData), Added<NonClientBaseTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in objects.iter() {
        let quest_visibility = QuestVisibility {
            visible_on_available: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestAvailable)
                .unwrap_or(&vec![]).clone(),
            visible_on_complete: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestComplete)
                .unwrap_or(&vec![]).clone(),
            visible_on_finished: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestFinished)
                .unwrap_or(&vec![]).clone(),
            visible_on_in_progress: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestInProgress)
                .unwrap_or(&vec![]).clone(),
        };

        if !quest_visibility.visible_on_available.is_empty() ||
            !quest_visibility.visible_on_complete.is_empty() ||
            !quest_visibility.visible_on_finished.is_empty() ||
            !quest_visibility.visible_on_in_progress.is_empty()
        {
            commands.entity(ent)
                .insert(quest_visibility);
        }
    }
}

#[derive(Component)]
#[relationship(relationship_target = QuestTags)]
pub struct QuestTagOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = QuestTagOf, linked_spawn)]
pub struct QuestTags(Vec<Entity>);

#[derive(Component)]
pub struct QuestGiver {
    pub template: Arc<QuestTemplate>,
}

pub(super) fn add_npc_quest_tags(
    objects: Query<(Entity, &GameObjectData), Added<NonClientBaseTag>>,
    quests: Res<Quests>,
    mut commands: Commands,
) {
    for (ent, obj) in objects.iter() {

        for quest in quests.values() {
            // Check for quest givers
            if 
                let Some(dialogue_id) = quest.template.available_dialogue_id &&
                let Ok(dialogs) = obj.get::<_, Vec<i32>>(NonClientBase::Dialogs) &&
                dialogs.contains(&dialogue_id)
            {
                debug!("Adding QuestGiver component to entity {} for quest {}", ent, quest.id);

                commands
                    .spawn((
                        QuestTagOf(ent),
                        QuestGiver {
                            template: quest.template.clone(),
                        },
                    ));
            }
        }
    }
}
