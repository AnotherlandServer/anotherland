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
use bevy::ecs::{component::Component, entity::Entity, error::Result, hierarchy::ChildOf, system::EntityCommands, world::EntityWorldMut};
use mlua::Function;
use obj_params::{Class, ContentRefList, GameObjectData, NpcOtherland};
use realm_api::{ObjectPlacement, ObjectTemplate};
use toolkit::types::{AvatarType, Uuid};

use crate::plugins::{Active, Avatar, AvatarIdManager, ContentCache, ContentCacheRef, ContentInfo, CooldownGroups, DynamicInstance, Factions, FactionsParameters, Inventory, InventoryParameter, LoadContext, LoadableComponent, NpcAbilities, PlayerLocalSets, SpawnCallback, SpawnState, VirtualComponent, WeakCache};

#[derive(Component)]
pub struct NonPlayerGameObjectLoader;

pub enum NonPlayerGameObjectLoaderParams {
    Placement(ObjectPlacement),
    Dynamic {
        id: Uuid,
        owner: Option<Entity>,
        name: String,
        template: ContentCacheRef,
        data: GameObjectData,
        callback: Option<Function>,
    }
}

pub struct ContextData {
    id: Uuid,
    owner: Option<Entity>,
    name: String,
    object: GameObjectData,
    template: Arc<ObjectTemplate>,
    is_dynamic: bool,
    callback: Option<Function>,
}

impl VirtualComponent for NonPlayerGameObjectLoader {}

impl LoadableComponent for NonPlayerGameObjectLoader {
    type Parameters = NonPlayerGameObjectLoaderParams;
    type ContextData = ContextData;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        match parameters {
            NonPlayerGameObjectLoaderParams::Placement(mut placement) => {
                if let Some(template) = ContentCache::get(&ContentCacheRef::Uuid(placement.content_guid)).await? {
                    placement.data.set_parent(Some(template.clone()));

                    context.set_data(ContextData { 
                        id: placement.id,
                        owner: None,
                        name: placement.editor_name,
                        object: placement.data,
                        template,
                        is_dynamic: false,
                        callback: None,
                    });
                } else {
                    return Err(anyhow!(
                        "Failed to load object template {} for placement {}", 
                        placement.content_guid, 
                        placement.id
                    ).into());
                }
            },
            NonPlayerGameObjectLoaderParams::Dynamic { id, owner, name, template, mut data, callback } => {
                if let Some(template) = ContentCache::get(&template).await? {
                    data.set_parent(Some(template.clone()));

                    context.set_data(ContextData { 
                        id,
                        owner,
                        name,
                        object: data,
                        template,
                        is_dynamic: true,
                        callback,
                    });
                } else {
                    return Err(anyhow!(
                        "Failed to load object template {:?} for dynamic object {}", 
                        template, 
                        name
                    ).into());
                }
            },
        }

        Ok(NonPlayerGameObjectLoader)
    }

    fn load_dependencies(&mut self, _commands: &mut EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        let object = &context.data().as_ref().unwrap().object;
        
        if object.class() == Class::NpcOtherland {
            let factions = object.get::<_, ContentRefList>(NpcOtherland::Faction)?.clone();
            let weapons = object.get::<_, ContentRefList>(NpcOtherland::DefaultWeapon)?.clone();
            let items = object.get::<_, ContentRefList>(NpcOtherland::DefaultItems)?.clone();
            let abilities = object.get::<_, ContentRefList>(NpcOtherland::Abilities)?.clone();

            context
                .load_dependency::<Factions>(FactionsParameters {
                    factions,
                })
                .load_dependency::<Inventory>(InventoryParameter::Static {
                    weapons,
                    items,
                })
                .load_dependency::<NpcAbilities>(abilities);
        }

        Ok(())
    }

    fn on_load(&mut self, commands: &mut EntityCommands<'_>, mut data: Option<Self::ContextData>) -> Result<()> {
        let placement = data.take().unwrap();
        let owner = placement.owner;
        let is_dynamic = placement.is_dynamic;
        let class = placement.object.class();
        let callback = placement.callback.clone();

        commands
            .queue(move |mut entity: EntityWorldMut<'_>| {
                let mut avatar_manager = entity.resource_mut::<AvatarIdManager>();

                let entry = avatar_manager.new_avatar_entry(AvatarType::Npc);
                let id = *entry.key();

                let placement_id = placement.id;
                let name = placement.name;
                let template = placement.template.clone();

                entity.insert((
                    Avatar {
                        id,
                        name,
                    },
                    ContentInfo {
                        placement_id,
                        template,
                    },
                    if placement.is_dynamic {
                        placement.object
                    } else {
                        GameObjectData::instantiate(Arc::new(placement.object))
                    },
                    Active,
                    SpawnState::default(),
                    PlayerLocalSets::default(),
                ));

                if class == Class::NpcOtherland {
                    let cooldown_groups = entity.resource::<CooldownGroups>();

                    entity.insert(cooldown_groups.create_cooldowns());
                }
            });

        if let Some(owner) = owner {
            commands
                .insert(ChildOf(owner));
        }

        if is_dynamic {
            commands
                .insert(DynamicInstance);
        }

        if let Some(callback) = callback {
            commands
                .insert(SpawnCallback(callback));
        }

        Ok(())
    }
}