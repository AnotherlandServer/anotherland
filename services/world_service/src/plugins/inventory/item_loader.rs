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
use bevy::ecs::{component::Component, error::Result};
use log::debug;
use obj_params::{ContentRefList, EdnaFunction, GameObjectData, ItemEdna};
use realm_api::ObjectTemplate;
use serde_json::Value;
use toolkit::types::{UUID_NIL, Uuid};

use crate::plugins::{ContentCache, ContentCacheRef, ContentInfo, ItemAbilities, ItemAbilityRef, ItemEdnaAbilities, LoadContext, LoadableComponent, WeakCache};

#[derive(Component)]
pub struct Item;

pub struct ItemParameters {
    pub template_id: Uuid,
    pub instance: ItemInstance,
}

pub struct ItemInstance {
    pub id: Uuid,
    pub object: GameObjectData,
}

impl LoadableComponent for Item {
    type Parameters = ItemParameters;
    type ContextData = (ItemInstance, Arc<ObjectTemplate>);

    async fn load(mut parameters: Self::Parameters, context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        let Some(template) = 
            ContentCache::get(&ContentCacheRef::Uuid(parameters.template_id)).await? 
        else {
            return Err(anyhow!(
                "Failed to load object template {}", 
                parameters.template_id,
            ).into());
        };

        parameters.instance.object.set_parent(Some(template.clone()));
        context.set_data((parameters.instance, template));

        Ok(Item)
    }

    fn load_dependencies(&mut self, commands: &mut bevy::ecs::system::EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        let (item, _) = context.data().as_ref().unwrap();

        let mut item_abilities = vec![];

        if 
            let Ok(abilities) = item.object.get::<_, Value>(ItemEdna::Abilities)
        {
            let abilities = serde_json::from_value::<ItemEdnaAbilities>(abilities.to_owned())?;
            
            item_abilities.extend(
                abilities.0
                    .into_iter()
                    .map(|a| ItemAbilityRef::Name(a.ability_name))
            );
        }

        if let Ok(skills) = item.object.get::<_, ContentRefList>(EdnaFunction::DefaultSkills) {
            item_abilities.extend(
                skills
                    .iter()
                    .map(|a| ItemAbilityRef::ContentRef(*a))
            );
        }


        context
            .load_dependency::<ItemAbilities>(item_abilities);

        Ok(())
    }

    fn on_load(&mut self, commands: &mut bevy::ecs::system::EntityCommands<'_>, data: Option<Self::ContextData>) -> Result<()> {
        let (instance, template) = data.unwrap();

        commands
            .insert((
                ContentInfo {
                    placement_id: instance.id,
                    template,
                },
                instance.object,
            ));

        Ok(())
    }
}