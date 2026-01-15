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

use bevy::{ecs::{component::Component, entity::Entity, system::{Commands, Query}}, platform::cell::SyncCell};

pub(super) trait RunnableAsyncOperation {
    fn run(&mut self, commands: &mut Commands) -> bool;
}

#[derive(Component)]
pub(super) struct AsyncOperationRunner {
    operation: SyncCell<Box<dyn RunnableAsyncOperation + Send>>,
}

impl AsyncOperationRunner {
    pub fn new<T: RunnableAsyncOperation + Send + 'static>(operation: T) -> Self {
        Self {
            operation: SyncCell::new(Box::new(operation)),
        }
    }
}

pub(super) fn run_async_operations(
    mut query: Query<(Entity, &mut AsyncOperationRunner)>,
    mut commands: Commands,
) {
    for (entity, mut runner) in query.iter_mut() {
        if runner.operation.get().run(&mut commands) {
            commands.entity(entity).despawn();
        }
    }
}