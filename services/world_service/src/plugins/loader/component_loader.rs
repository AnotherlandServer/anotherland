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

use std::collections::LinkedList;

use bevy::{ecs::{component::Component, entity::Entity, error::{BevyError, Result}, resource::Resource, system::{Commands, EntityCommands, ResMut}, world::EntityWorldMut}, tasks::{IoTaskPool, Task, block_on, poll_once}};
use log::{debug, error};

pub struct LoadContext<T: Send + Sync + Sized> {
    entity: Entity,
    dependant_loaders: Vec<Box<dyn TypeErasedComponentLoader + 'static>>,
    data: Option<T>,
}

impl<D: Send + Sync + Sized> LoadContext<D> {
    pub fn set_data(&mut self, data: D) {
        self.data = Some(data);
    }

    pub fn data(&self) -> &Option<D> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Option<D> {
        &mut self.data
    }

    pub fn load_dependency<T: LoadableComponent + 'static>(&mut self, parameters: T::Parameters) -> &mut Self {
        self.load_dependency_with_error_handler::<T, _>(parameters, default_error_handler)
    }

    pub fn load_cross_dependency<T: LoadableComponent + 'static>(
        &mut self,
        entity: Entity,
        parameters: T::Parameters,
    ) -> &mut Self {
        self.load_cross_dependency_with_error_handler::<T, _>(entity, parameters, default_error_handler)
    }

    pub fn load_dependency_with_error_handler<
        T: LoadableComponent + 'static, 
        F: FnOnce(BevyError, &mut EntityCommands<'_>) + Send + Sync + 'static
    >(
        &mut self,
        parameters: T::Parameters,
        error_handler: F,
    ) -> &mut Self {
        self.load_cross_dependency_with_error_handler::<T, F>(self.entity, parameters, error_handler)
    }

    pub fn load_cross_dependency_with_error_handler<
        T: LoadableComponent + 'static, 
        F: FnOnce(BevyError, &mut EntityCommands<'_>) + Send + Sync + 'static
    >(
        &mut self,
        entity: Entity,
        parameters: T::Parameters,
        error_handler: F,
    ) -> &mut Self {
        self.dependant_loaders.push(Box::new(ComponentLoader::<T, F> {
            entity,
            task: IoTaskPool::get()
                .spawn(run_loader::<T>(entity, parameters)),
            error_handler: Some(error_handler),
            component: None,
            context: None,
        }));

        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

pub trait VirtualComponent: Sized {}

trait MaybeVirtualComponent {
    fn is_virtual() -> bool;
}

impl<T: LoadableComponent> MaybeVirtualComponent for T {
    default fn is_virtual() -> bool {
        false
    }
}

impl<T: LoadableComponent + VirtualComponent> MaybeVirtualComponent for T {
    fn is_virtual() -> bool {
        true
    }
}

pub trait LoadableComponent: Component + Sized + Send + Sync {
    type Parameters: Send + Sync;
    type ContextData: Send + Sync = ();

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self>;
    fn load_dependencies(&mut self, _commands: &mut EntityCommands<'_>, _context: &mut LoadContext<Self::ContextData>) -> Result<()> { Ok(()) }
    fn post_load(&mut self, _commands: &mut EntityCommands<'_>, _data: Option<Self::ContextData>) -> Result<()> { Ok(()) }
    fn post_insert(_entity: EntityWorldMut<'_>) {}
}

#[derive(Resource, Default)]
pub struct LoadingComponents {
    loaders: LinkedList<Box<dyn TypeErasedComponentLoader + Send + Sync>>,
}

impl LoadingComponents {
    pub fn load_component<T: LoadableComponent + 'static + Send + Sync>(&mut self, entity: Entity, parameters: T::Parameters) {
        self.load_component_with_error_handler::<T, _>(entity, parameters, default_error_handler)
    }

    pub fn load_component_with_error_handler<
        T: LoadableComponent + 'static + Send + Sync, 
        F: FnOnce(BevyError, &mut EntityCommands<'_>) + Send + Sync + 'static
    >(
        &mut self,
        entity: Entity,
        parameters: T::Parameters,
        error_handler: F,
    ) {
        self.loaders.push_back(Box::new(ComponentLoader::<T, F> {
            entity,
            task: IoTaskPool::get()
                .spawn(run_loader::<T>(entity, parameters)),
            error_handler: Some(error_handler),
            component: None,
            context: None,
        }));
    }
}

async fn run_loader<T: LoadableComponent>(entity: Entity, parameters: T::Parameters) -> Result<(T, LoadContext<T::ContextData>)> {
    let mut context = LoadContext {
        entity,
        dependant_loaders: Vec::new(),
        data: None,
    };

    T::load(parameters, &mut context)
        .await
        .map(|c| (c, context))
}

pub struct ComponentLoader<T: LoadableComponent, F: FnOnce(BevyError, &mut EntityCommands<'_>)> {
    entity: Entity,
    task: Task<Result<(T, LoadContext<T::ContextData>)>>,
    error_handler: Option<F>,
    component: Option<T>,
    context: Option<LoadContext<T::ContextData>>,
}

trait TypeErasedComponentLoader: Send + Sync {
    fn load_component(&mut self, commands: &mut Commands<'_, '_>) -> bool;
}

impl <
    T: LoadableComponent + MaybeVirtualComponent, 
    F: FnOnce(BevyError, &mut EntityCommands<'_>) + Send + Sync + 'static
> TypeErasedComponentLoader for ComponentLoader<T, F> {
    fn load_component(&mut self, commands: &mut Commands<'_, '_>) -> bool {
        if self.component.is_none() {
            let Some(res) = block_on(poll_once(&mut self.task)) else {
                return false;
            };

            let mut entity_commands = commands.entity(self.entity);

            let Ok((mut component, mut context)) = res else {
                debug!("ComponentLoader: Error loading component for entity {:?}", self.entity);
                (self.error_handler.take().unwrap())(res.err().unwrap(), &mut entity_commands);
                return true;
            };

            if let Err(e) = component.load_dependencies(&mut entity_commands, &mut context) {
                debug!("ComponentLoader: Error loading dependencies for component for entity {:?}: {:?}", self.entity, e);
                (self.error_handler.take().unwrap())(e, &mut entity_commands);
                return true;
            }

            self.component = Some(component);
            self.context = Some(context);

            false
        } else {
            let context = self.context.as_mut().unwrap();

            context
                .dependant_loaders
                .retain_mut(|c| {
                    !c.load_component(commands)
                });

            if context.dependant_loaders.is_empty() {
                let mut component = self.component.take().unwrap();

                if let Err(e) = component.post_load(&mut commands.entity(self.entity), self.context.take().unwrap().data) {
                    debug!("ComponentLoader: Error in on_load for component for entity {:?}: {:?}", self.entity, e);
                    (self.error_handler.take().unwrap())(e, &mut commands.entity(self.entity));
                    return true;
                }

                if !T::is_virtual() {
                    commands
                        .entity(self.entity)
                        .insert(component)
                        .queue(T::post_insert);
                }
                
                true
            } else {
                false
            }
        }
    }
}

fn default_error_handler(error: BevyError, _commands: &mut EntityCommands<'_>) {
    error!("Error loading component: {:?}", error);
}

pub fn process_loading_components(
    mut loading: ResMut<LoadingComponents>,
    mut commands: Commands,
) {
    let loading = loading.as_mut();

    loading.loaders
        .retain(|c| {
            !c.load_component(&mut commands) 
        });
}