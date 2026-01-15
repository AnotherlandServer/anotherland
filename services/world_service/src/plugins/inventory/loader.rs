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

use bevy::ecs::{error::Result, hierarchy::ChildOf, system::EntityCommands};
use obj_params::{ContentRefList, GameObjectData};
use realm_api::{RealmApi, StorageOwner};

use crate::plugins::{Inventory, ItemInstance, ItemParameters, LoadContext, LoadableComponent};

pub enum InventoryParameter {
    Persistent(StorageOwner, String),
    Static {
        weapons: ContentRefList,
        items: ContentRefList,
    }
}

impl LoadableComponent for Inventory {
    type Parameters = InventoryParameter;
    type ContextData = Vec<ItemParameters>;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        match parameters {
            InventoryParameter::Persistent(owner, name) => {
                let storage = RealmApi::get()
                    .get_or_create_item_storage(owner, &name)
                    .await?;

                let mut inventory = Inventory::new(
                    storage.id, 
                    storage.name, 
                    storage.bling, 
                    storage.game_cash,
                    storage.capacity,
                );

                context
                    .set_data(
                        storage.items
                            .into_iter()
                            .map(|item| ItemParameters {
                                template_id: item.template_id,
                                instance: ItemInstance {
                                    id: item.id,
                                    object: item.instance,
                                },
                            })
                            .collect(),
                    );

                inventory.observing_players.insert(context.entity());
    
                Ok(inventory)
            }
            InventoryParameter::Static { weapons, items } => {
                let inventory = Inventory::default();

                context
                    .set_data(
                        weapons.iter()
                            .chain(items.iter())
                            .map(|item| {
                                ItemParameters {
                                    template_id: item.id,
                                    instance: ItemInstance {
                                        id: item.id,
                                        object: GameObjectData::new_for_class(item.class),
                                    },
                                }    
                            })
                            .collect()
                        );


                Ok(inventory)
            }
        }
    }

    fn load_dependencies(&mut self, commands: &mut EntityCommands<'_>, context: &mut crate::plugins::LoadContext<Self::ContextData>) -> Result<()> {
        let parent_ent = commands.id();

        for item in context.data_mut().take().unwrap() {
            let ent = commands
                .commands()
                .spawn(ChildOf(parent_ent))
                .id();

            self.items.insert(item.instance.id, ent);

            context.load_cross_dependency::<super::item_loader::Item>(ent, item);
        }

        Ok(())
    }
}