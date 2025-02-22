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

use bevy::{app::{First, Plugin}, ecs::{component::Component, entity::Entity, system::{Commands, In, Query, SystemId}}, prelude::App, tasks::{block_on, futures_lite::future}, utils::synccell::SyncCell};

#[derive(Component)]
pub struct FutureTaskComponent(SyncCell<Box<dyn FnMut(&mut Commands) -> bool + Send>>);

impl FutureTaskComponent {
    pub fn new<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static, system: SystemId<In<T>>) -> Self {
        let mut future = Box::pin(future);

        Self(SyncCell::new(Box::new(move |commands| {
            if let Some(res) = block_on(future::poll_once(&mut future)) {
                commands.run_system_with_input(system, res);
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

pub trait FutureCommands {
    fn run_system_async<T: Send + 'static>(&mut self, future: impl Future<Output = T> + Send + 'static, system: SystemId<In<T>>);
}

impl FutureCommands for Commands<'_, '_> {
    fn run_system_async<T: Send + 'static>(&mut self, future: impl Future<Output = T> + Send + 'static, system: SystemId<In<T>>) {        
        self.spawn(FutureTaskComponent::new(future, system));
    }
}