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

use bevy::{ecs::{component::Component, entity::Entity, error::Result, system::{EntityCommands, In}, world::EntityWorldMut}};
use realm_api::StorageOwner::Character;
use toolkit::types::Uuid;

use crate::plugins::{Inventory, InventoryParameter, LoadContext, LoadableComponent, VirtualComponent};

#[derive(Component)]
pub struct InitialInventoryTransfer(pub Option<Vec<Entity>>);

impl VirtualComponent for InitialInventoryTransfer {}

impl LoadableComponent for InitialInventoryTransfer {
    type Parameters = Uuid;
    type ContextData = InventoryParameter;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        context.set_data(InventoryParameter::Persistent(Character(parameters), "inventory".to_string()));

        Ok(InitialInventoryTransfer(None))
    }

    fn load_dependencies(&mut self, _commands: &mut EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        let params = context.data_mut().take().unwrap();
        
        context
            .load_dependency::<Inventory>(params);

        Ok(())
    }

    fn on_load(&mut self, commands: &mut EntityCommands<'_>, _data: Option<Self::ContextData>) -> Result<()> {
        commands
            .queue(|mut ent: EntityWorldMut<'_>| {
                let items = ent.get::<Inventory>().unwrap()
                    .items
                    .iter()
                    .map(|item| *item.1)
                    .collect();

                ent.insert(InitialInventoryTransfer(Some(items)));
            });

        Ok(())
    }
}