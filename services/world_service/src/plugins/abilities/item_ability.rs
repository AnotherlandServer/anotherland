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

use std::ops::Deref;

use anyhow::anyhow;
use bevy::ecs::component::Component;
use obj_params::ContentRef;

use crate::plugins::{ContentCache, ContentCacheRef, LoadContext, LoadableComponent, StaticObject, WeakCache};

#[derive(Component)]
pub struct ItemAbilities(pub Vec<StaticObject>);

impl Deref for ItemAbilities {
    type Target = Vec<StaticObject>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum ItemAbilityRef {
    Name(String),
    ContentRef(ContentRef),
}

impl LoadableComponent for ItemAbilities {
    type Parameters = Vec<ItemAbilityRef>;

    async fn load(parameters: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> bevy::ecs::error::Result<Self> {
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

            abilities.push(StaticObject(ability));
        }

        Ok(ItemAbilities(abilities))
    }
}