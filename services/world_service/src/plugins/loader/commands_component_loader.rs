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

use bevy::ecs::{component::Component, system::EntityCommands, world::EntityWorldMut};

use crate::plugins::{LoadableComponent, LoadingComponents};

pub trait ComponentLoaderCommandsTrait {
    fn load_component<T: Component + LoadableComponent>(&mut self, parameters: T::Parameters) -> &mut Self;
    fn load_component_with_error_handler<T: Component + LoadableComponent>(
        &mut self,
        parameters: T::Parameters,
        error_handler: fn(bevy::ecs::error::BevyError, &mut EntityCommands<'_>),
    ) -> &mut Self;
}

impl ComponentLoaderCommandsTrait for EntityCommands<'_> {
    fn load_component<T: Component + LoadableComponent>(&mut self, parameters: T::Parameters) -> &mut Self {
        self.queue(move |mut entity: EntityWorldMut| {
            let id = entity.id();
            let mut loading = entity.resource_mut::<LoadingComponents>();
            loading.load_component::<T>(id, parameters);
        })
    }

    fn load_component_with_error_handler<T: Component + LoadableComponent>(
        &mut self,
        parameters: T::Parameters,
        error_handler: fn(bevy::ecs::error::BevyError, &mut EntityCommands<'_>),
    ) -> &mut Self {
        self.queue(move |mut entity: EntityWorldMut| {
            let id = entity.id();
            let mut loading = entity.resource_mut::<LoadingComponents>();
            loading.load_component_with_error_handler::<T>(id, parameters, error_handler);
        })
    }
}