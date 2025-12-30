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

use bevy::{ecs::{component::Component, entity::Entity, error::{BevyError, Result}, resource::Resource, system::{Commands, EntityCommands, ResMut}}, platform::collections::HashSet, tasks::{IoTaskPool, Task, block_on, poll_once}};
use log::{debug, error};

pub struct LoadContext {
    entity: Entity,
    dependant_loaders: Vec<Box<dyn TypeErasedComponentLoader + 'static>>,
}

impl LoadContext {
    pub fn load_dependency<T: LoadableComponent + 'static>(&mut self, parameters: T::Parameters) {
        self.load_dependency_with_error_handler::<T>(parameters, default_error_handler)
    }

    pub fn load_dependency_with_error_handler<T: LoadableComponent + 'static>(
        &mut self,
        parameters: T::Parameters,
        error_handler: fn(BevyError, &mut EntityCommands<'_>),
    ) {
        self.dependant_loaders.push(Box::new(ComponentLoader::<T> {
            entity: self.entity,
            task: IoTaskPool::get().spawn(T::load(parameters)),
            error_handler,
            dependant_loaders: Vec::new(),
            component: None,
        }));
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

    async fn load(parameters: Self::Parameters) -> Result<Self>;
    fn on_load(&mut self, commands: &mut EntityCommands<'_>, context: &mut LoadContext) -> Result<()> { Ok(()) }
}

#[derive(Resource, Default)]
pub struct LoadingComponents {
    loaders: LinkedList<Box<dyn TypeErasedComponentLoader + Send + Sync>>,
}

impl LoadingComponents {
    pub fn load_component<T: LoadableComponent + 'static + Send + Sync>(&mut self, entity: Entity, parameters: T::Parameters) {
        self.load_component_with_error_handler::<T>(entity, parameters, default_error_handler)
    }

    pub fn load_component_with_error_handler<T: LoadableComponent + 'static + Send + Sync>(
        &mut self,
        entity: Entity,
        parameters: T::Parameters,
        error_handler: fn(BevyError, &mut EntityCommands<'_>),
    ) {
        let task_pool = IoTaskPool::get();
        self.loaders.push_back(Box::new(ComponentLoader::<T> {
            entity,
            task: task_pool.spawn(T::load(parameters)),
            error_handler,
            dependant_loaders: Vec::new(),
            component: None,
        }));
    }
}

pub struct ComponentLoader<T: LoadableComponent> {
    entity: Entity,
    task: Task<Result<T>>,
    error_handler: fn(BevyError, &mut EntityCommands<'_>),
    dependant_loaders: Vec<Box<dyn TypeErasedComponentLoader>>,
    component: Option<T>,
}

trait TypeErasedComponentLoader: Send + Sync {
    fn entity(&self) -> Entity;
    fn load_component<'a>(&mut self, commands: &'a mut Commands<'_, '_>) -> bool;
}

impl<T: LoadableComponent + MaybeVirtualComponent> TypeErasedComponentLoader for ComponentLoader<T> {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn load_component<'a>(&mut self, commands: &'a mut Commands<'_, '_>) -> bool {
        if self.component.is_none() {
            let Some(res) = block_on(poll_once(&mut self.task)) else {
                return false;
            };

            let mut entity_commands = commands.entity(self.entity);

            let Ok(mut component) = res else {
                (self.error_handler)(res.err().unwrap(), &mut entity_commands);
                return true;
            };

            let mut context = LoadContext {
                entity: self.entity,
                dependant_loaders: Vec::new(),
            };

            if let Err(e) = component.on_load(&mut entity_commands, &mut context) {
                (self.error_handler)(e, &mut entity_commands);
                return true;
            }

            self.component = Some(component);

            self.dependant_loaders = context.dependant_loaders.drain(..).collect();
            false
        } else {
            self.dependant_loaders
                .retain_mut(|c| {
                    !c.load_component(commands)
                });

            if self.dependant_loaders.is_empty() {
                if !T::is_virtual() {
                    commands
                        .entity(self.entity)
                        .insert(self.component.take().unwrap());
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