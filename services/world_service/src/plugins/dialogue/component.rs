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

use anyhow::anyhow;
use bevy::ecs::{component::Component, entity::Entity, error::Result, lifecycle::HookContext, world::DeferredWorld};
use realm_api::Condition;

use crate::plugins::{LoadContext, LoadableComponent, Quests, WeakCache, dialogue::cache::DialogueCache};

#[derive(Component)]
#[component(on_add = on_dialogue_add)]
pub struct Dialogue(pub Arc<realm_api::QuestDialogue>);

#[derive(Component)]
pub struct QuestDialogue {
    pub quest_id: i32,
    pub condition_id: i32,
}

impl LoadableComponent for Dialogue {
    type Parameters = i32;
    type ContextData = ();

    async fn load(id: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        if let Some(dialogue) = DialogueCache::get(&id).await? {
            Ok(Dialogue(dialogue))
        } else {
            Err(anyhow!("Dialogue with ID {} not found", id).into())
        }
    }
}

#[derive(Component)]
pub struct DeferredQuestDialogueResponse {
    pub speaker: Entity,
}

fn on_dialogue_add(mut world: DeferredWorld, context: HookContext) {
    let quests = world.resource::<Quests>();
    let my_dialogue_id = world.get::<Dialogue>(context.entity)
        .map(|d| d.0.id)
        .unwrap();

    for (_, quest) in quests.iter() {
        let relevant_condition_id = quest.template.conditions
            .iter()
            .find_map(|c| {
                if 
                    let Condition::Dialogue { id, dialogue_id, .. } = c &&
                    *dialogue_id == my_dialogue_id
                {
                    Some(*id)
                } else {
                    None
                }
            });
        
        if let Some(condition_id) = relevant_condition_id {
            let quest_id = quest.id;
            world
                .commands()
                .entity(context.entity)
                .insert(QuestDialogue {
                    quest_id,
                    condition_id
                });
            
            return;
        }
    }
}