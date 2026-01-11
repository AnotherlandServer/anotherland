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
use bevy::ecs::{component::Component, error::Result, system::EntityCommands, world::EntityWorldMut};
use obj_params::{Class, ContentRefList, GameObjectData, NpcOtherland};
use realm_api::{ObjectPlacement, ObjectTemplate};
use toolkit::types::AvatarType;

use crate::plugins::{Active, Avatar, AvatarIdManager, ContentCache, ContentCacheRef, ContentInfo, CooldownGroups, Factions, FactionsParameters, Inventory, InventoryParameter, LoadContext, LoadableComponent, NpcAbilities, PlayerLocalSets, SpawnState, VirtualComponent, WeakCache};

#[derive(Component)]
pub struct NonPlayerGameObjectLoader {
    placement: Option<ObjectPlacement>,
    template: Arc<ObjectTemplate>,
}

pub struct NonPlayerGameObjectLoaderParams {
    pub placement: ObjectPlacement,
}

impl VirtualComponent for NonPlayerGameObjectLoader {}

impl LoadableComponent for NonPlayerGameObjectLoader {
    type Parameters = NonPlayerGameObjectLoaderParams;

    async fn load(mut parameters: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        if let Some(template) = ContentCache::get(&ContentCacheRef::Uuid(parameters.placement.content_guid)).await? {
            parameters.placement.data.set_parent(Some(template.clone()));

            Ok(NonPlayerGameObjectLoader {
                placement: Some(parameters.placement),
                template,
            })
        } else {
            Err(anyhow!(
                "Failed to load object template {} for placement {}", 
                parameters.placement.content_guid, 
                parameters.placement.id
            ).into())
        }
    }

    fn load_dependencies(&mut self, _commands: &mut EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        let placement = self.placement.as_ref().unwrap();

        if placement.class == Class::NpcOtherland {
            context
                .load_dependency::<Factions>(FactionsParameters {
                    factions: placement.data.get::<_, ContentRefList>(NpcOtherland::Faction)?.clone(),
                })
                .load_dependency::<Inventory>(InventoryParameter::Static {
                    weapons: placement.data.get::<_, ContentRefList>(NpcOtherland::DefaultWeapon)?.clone(),
                    items: placement.data.get::<_, ContentRefList>(NpcOtherland::DefaultItems)?.clone(),
                })
                .load_dependency::<NpcAbilities>(placement.data.get::<_, ContentRefList>(NpcOtherland::Abilities)?.clone());
        }

        Ok(())
    }

    fn on_load(&mut self, commands: &mut EntityCommands<'_>, _data: Option<Self::ContextData>) -> Result<()> {
        let placement = self.placement.take().unwrap();
        let template = self.template.clone();

        commands
            .queue(move |mut entity: EntityWorldMut<'_>| {
                let mut avatar_manager = entity.resource_mut::<AvatarIdManager>();

                let entry = avatar_manager.new_avatar_entry(AvatarType::Npc);
                let id = *entry.key();

                entity.insert((
                    Avatar {
                        id,
                        name: placement.editor_name,
                    },
                    ContentInfo {
                        placement_id: placement.id,
                        template,
                    },
                    GameObjectData::instantiate(Arc::new(placement.data)),
                    Active,
                    SpawnState::default(),
                    PlayerLocalSets::default(),
                ));

                if placement.class == Class::NpcOtherland {
                    let cooldown_groups = entity.resource::<CooldownGroups>();

                    entity.insert(cooldown_groups.create_cooldowns());
                }
            });

        Ok(())
    }
}