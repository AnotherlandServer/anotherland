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

use bevy::{ecs::{component::Component, entity::Entity, error::Result, system::EntityCommands}, math::Vec3};
use obj_params::{Class, EdnaFunction, EdnaModule, GameObjectData, ItemBase, ItemEdna, LootScatterContainer};
use realm_api::ObjectTemplate;
use toolkit::types::{AvatarId, UUID_NIL, Uuid};

use crate::plugins::{LoadableComponent, NonPlayerGameObjectLoader, NonPlayerGameObjectLoaderParams, VirtualComponent, WeakCache, content_cache::{ContentCache, ContentCacheRef}};

#[derive(Component)]
pub struct LootLoader {
    params: LootParams,
    loot: LoadedLoot,
}

impl VirtualComponent for LootLoader {}

pub struct LootParams {
    pub spawner: (AvatarId, Uuid),
    pub allow_player: Option<AvatarId>,
    pub allow_party: Option<Uuid>,
    pub loot: Loot,
    pub pos: Vec3,
}

pub enum Loot {
    Item(String, i32),
    Soma(i32),
}

enum LoadedLoot {
    Item(Arc<ObjectTemplate>)
}

impl LoadableComponent for LootLoader {
    type Parameters = LootParams;
    type ContextData = ();

    async fn load(parameters: Self::Parameters, context: &mut crate::plugins::LoadContext<Self::ContextData>) -> Result<Self> {
        match parameters.loot {
            Loot::Item(ref item_name, _quantity) => {
                let item_template = ContentCache::get(&ContentCacheRef::Name(
                    item_name.clone()
                )).await?
                .ok_or_else(|| bevy::ecs::error::BevyError::from("Failed to load item"))?;

               Ok(LootLoader {
                    params: parameters,
                    loot: LoadedLoot::Item(item_template),
                })
            },
            Loot::Soma(_amount) => {
                todo!()
            }
        }
    }

    fn load_dependencies(&mut self, commands: &mut EntityCommands<'_>, context: &mut crate::plugins::LoadContext<Self::ContextData>) -> Result<()> {
        let mut data = GameObjectData::new::<LootScatterContainer>();

        let offset = Vec3::new(rand::random::<f32>() * 200.0 - 100.0, 0.0, rand::random::<f32>() * 200.0 - 100.0);

        data.set(LootScatterContainer::SpawnerAvatarId, self.params.spawner.0);
        data.set(LootScatterContainer::SpawnerAvatarGuid, self.params.spawner.1);
        data.set(LootScatterContainer::AllowAvatar, self.params.allow_player.unwrap_or_default());
        data.set(LootScatterContainer::AllowParty, self.params.allow_party.unwrap_or(UUID_NIL));
        data.set(LootScatterContainer::Pos, self.params.pos + offset);

        match self.loot {
            LoadedLoot::Item(ref item_template) => {
                data.set(LootScatterContainer::IsQuestItem, item_template.data.get::<_, bool>(ItemBase::IsQuestItem).copied().unwrap_or_default());
                data.set(LootScatterContainer::Rarity, item_template.data.get::<_, i32>(ItemBase::Rarity).copied().unwrap_or_default());
                data.set(LootScatterContainer::ItemContentGuid, item_template.id);
                data.set(LootScatterContainer::ItemContentName, item_template.name.clone());
                
                if item_template.data.get::<_, bool>(ItemBase::IsQuestItem).copied().unwrap_or_default() {
                    data.set(LootScatterContainer::ScatterLootVisualType, 6);
                } else if item_template.data.get::<_, bool>(ItemBase::IsRecipe).copied().unwrap_or_default() {
                    data.set(LootScatterContainer::ScatterLootVisualType, 5);
                } else if 
                    item_template.data.get::<_, bool>(ItemEdna::IsSku).copied().unwrap_or_default()
                {
                    data.set(LootScatterContainer::ScatterLootVisualType, 4);
                } else if item_template.data.get::<_, bool>(EdnaFunction::IsConsumable).copied().unwrap_or_default() {
                    data.set(LootScatterContainer::ScatterLootVisualType, 3);
                } else if item_template.data.class() == Class::EdnaFunction {
                    data.set(LootScatterContainer::ScatterLootVisualType, 1);
                } else {
                    data.set(LootScatterContainer::ScatterLootVisualType, 2);
                }
            }
        }

        context
            .load_dependency::<NonPlayerGameObjectLoader>(NonPlayerGameObjectLoaderParams::Dynamic { 
                id: Uuid::new(), 
                owner: None, 
                name: "Loot".to_string(), 
                template: ContentCacheRef::Name("LootItemContainer".to_string()), 
                data, 
                callback: None, 
            });

        Ok(())
    }
}