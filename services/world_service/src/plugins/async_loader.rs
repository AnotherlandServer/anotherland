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

use std::pin::Pin;

use bevy::{app::{First, Plugin}, ecs::{component::Component, entity::Entity, system::{Command, Commands, In, Query, SystemId}}, prelude::App, tasks::futures_lite::future, utils::synccell::SyncCell};
use futures::executor::block_on;
use tokio::task::JoinHandle;

use crate::instance::ZoneInstance;

#[derive(Component)]
#[allow(clippy::type_complexity)]
pub struct FutureTaskComponent(SyncCell<Box<dyn FnMut(&mut Commands) -> bool + Send>>);

impl FutureTaskComponent {
    pub fn new<T: Send + 'static>(mut task: JoinHandle<T>, system: SystemId<In<T>>) -> Self {
        Self(SyncCell::new(Box::new(move |commands| {
            if let Some(res) = block_on(future::poll_once(&mut task)) {
                commands.run_system_with(system, res.unwrap());
                true
            } else {
                false
            }
        })))
    }
}

pub struct AsyncLoaderPlugin;

impl Plugin for AsyncLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, run_futures);
    }
}

fn run_futures(
    mut futures: Query<(Entity, &mut FutureTaskComponent)>,
    mut commands: Commands,
) {
    for (entity, mut future) in futures.iter_mut() {
        if future.0.get()(&mut commands) {
            commands.entity(entity).despawn();
        }
    }
}

struct FutureCommand<T: Send + 'static> {
    future: Pin<Box<dyn Future<Output = T> + Send + 'static>>,
    system: SystemId<In<T>>,
}

impl <T: Send + 'static> Command for FutureCommand<T> {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let instance = world.get_resource::<ZoneInstance>().unwrap();

        let join_handle = instance.spawn_task(self.future);

        world.spawn(FutureTaskComponent::new(join_handle, self.system));
    }
}

pub trait FutureCommands {
    fn run_system_async<T: Send + 'static>(&mut self, task: impl Future<Output = T> + Send + 'static, system: SystemId<In<T>>);
}

impl FutureCommands for Commands<'_, '_> {
    fn run_system_async<T: Send + 'static>(&mut self, future: impl Future<Output = T> + Send + 'static, system: SystemId<In<T>>) { 
        self.queue(FutureCommand {
            future: Box::pin(future),
            system,
        });
        //self.spawn(FutureTaskComponent::new(, system));
    }
}