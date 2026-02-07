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
use bevy::ecs::{component::Component, entity::Entity, error::Result};
use realm_api::QuestDialogue;

use crate::plugins::{LoadContext, LoadableComponent, WeakCache, dialogue::cache::DialogueCache};

#[derive(Component)]
pub struct Dialogue(pub Arc<QuestDialogue>);

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