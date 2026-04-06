// Copyright (C) 2026 AnotherlandServer
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
use bevy::ecs::{component::Component, error::Result, system::EntityCommands};
use obj_params::{ContentRefList, GameObjectData, ObjectInserter};
use realm_api::ObjectTemplate;
use toolkit::types::UUID_NIL;

use crate::plugins::{AbilityOf, AbilityType, ContentCache, ContentCacheRef, ContentInfo, InitializeObject, LoadContext, LoadableComponent, Scripted, VirtualComponent, WeakCache};

#[derive(Component)]
pub struct NpcAbilityLoader;

impl VirtualComponent for NpcAbilityLoader {}

impl LoadableComponent for NpcAbilityLoader {
    type Parameters = ContentRefList;
    type ContextData = Vec<(GameObjectData, Arc<ObjectTemplate>)>;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        let mut abilities = Vec::with_capacity(parameters.len());

        for content_ref in parameters.iter() {
            let Some(template) = 
                ContentCache::get(&ContentCacheRef::ContentRef(*content_ref)).await? 
            else {
                return Err(anyhow!(
                    "Failed to load object template {}", 
                    content_ref,
                ).into());
            };

            abilities.push((
                GameObjectData::instantiate(template.clone()), 
                template
            ));
        }

        context.set_data(abilities);

        Ok(Self)
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, mut data: Option<Self::ContextData>) -> Result<()> {
        let abilities = data.take().unwrap_or_default();
        let ent = commands.id();

        for (ability_data, template) in abilities {
            commands
                .commands()
                .spawn((
                    AbilityOf::new(ent, AbilityType::Ability),
                    ContentInfo {
                        placement_id: UUID_NIL,
                        template
                    }
                ))
                .insert_object(ability_data)
                .insert(Scripted)
                .trigger(InitializeObject);
        }

        Ok(())
    }
}

