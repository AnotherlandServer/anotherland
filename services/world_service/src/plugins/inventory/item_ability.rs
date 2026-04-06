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
use obj_params::{ContentRef, GameObjectData, ObjectInserter};
use realm_api::ObjectTemplate;
use toolkit::types::UUID_NIL;

use crate::plugins::{AbilityOf, AbilityType, ContentCache, ContentCacheRef, ContentInfo, InitializeObject, LoadContext, LoadableComponent, Scripted, VirtualComponent, WeakCache};

#[derive(Component)]
pub struct ItemAbilities;

impl VirtualComponent for ItemAbilities {}

pub enum ItemAbilityRef {
    Name(String),
    ContentRef(ContentRef),
}

impl LoadableComponent for ItemAbilities {
    type Parameters = Vec<ItemAbilityRef>;
    type ContextData = Vec<Arc<ObjectTemplate>>;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        let mut abilities = vec![];

        for ability_ref in parameters {
            let content_ref = match ability_ref {
                    ItemAbilityRef::Name(name) => ContentCacheRef::Name(name),
                    ItemAbilityRef::ContentRef(content_ref) => ContentCacheRef::ContentRef(content_ref),
                };

            let ability = ContentCache::get(&content_ref)
                .await?
                .ok_or(anyhow!(
                    "Failed to load item ability template {:?}",
                    content_ref
                ))?;

            abilities.push(ability);
        }

        context.set_data(abilities);

        Ok(Self)
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, mut data: Option<Self::ContextData>) -> Result<()> {
        let abilities = data.take().unwrap_or_default();
        let ent = commands.id();

        for template in abilities {
            commands
                .commands()
                .spawn((
                    AbilityOf::new(ent, AbilityType::ItemAbility),
                    ContentInfo {
                        placement_id: UUID_NIL,
                        template: template.clone(),
                    }
                ))
                .insert_object(GameObjectData::instantiate(template.clone()))
                .insert(Scripted)
                .trigger(InitializeObject);
        }

        Ok(())
    }
}