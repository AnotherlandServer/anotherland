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

use std::{marker::PhantomData, pin::Pin};

use bevy::{ecs::{entity::Entity, error::{BevyError, Result}, system::{Commands, EntityCommands, In, IntoSystem, IsFunctionSystem}}, tasks::{block_on, poll_once}};

use crate::plugins::async_operation::{noop_system::NoOpSystem, runner::{AsyncOperationRunner, RunnableAsyncOperation}};

#[allow(clippy::type_complexity)]
pub trait AsyncOperationEntityCommandsExt<'a> {
    fn perform_async_operation<
        R: Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send + 'static,
    >(self, operation: T) -> EntityAsyncOperationCmd<'a, R, T, IsFunctionSystem, IsFunctionSystem, NoOpSystem<(Entity, R)>, NoOpSystem<(Entity, BevyError)>>;
}

impl<'a> AsyncOperationEntityCommandsExt<'a> for EntityCommands<'a> {
    fn perform_async_operation<
        R: Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send + 'static,
    >(self, operation: T) -> EntityAsyncOperationCmd<'a, R, T, IsFunctionSystem, IsFunctionSystem, NoOpSystem<(Entity, R)>, NoOpSystem<(Entity, BevyError)>> {
        let entity = self.id();
        EntityAsyncOperationCmd {
            operation: Some(EntityAsyncOperation {
                entity,
                operation: Some(Box::pin(operation)),
                on_finish_system: None,
                on_error_system: None,
                _phantom1: PhantomData,
                _phantom2: PhantomData,
            }),
            build_commands: Some(self),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

pub struct EntityAsyncOperation
<
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<(Entity, R)>, (), M1> + Send + Sync + 'static = NoOpSystem<(Entity, R)>,
    ErrorSystem: IntoSystem<In<(Entity, BevyError)>, (), M2> + Send + Sync + 'static = NoOpSystem<(Entity, BevyError)>,
> {
    entity: Entity,
    operation: Option<Pin<Box<T>>>,
    on_finish_system: Option<FinishSystem>,
    on_error_system: Option<ErrorSystem>,
    _phantom1: PhantomData<M1>,
    _phantom2: PhantomData<M2>,
}

pub struct EntityAsyncOperationCmd <
    'a,
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<(Entity, R)>, (), M1> + Send + Sync + 'static = NoOpSystem<(Entity, R)>,
    ErrorSystem: IntoSystem<In<(Entity, BevyError)>, (), M2> + Send + Sync + 'static = NoOpSystem<(Entity, BevyError)>,
> {
    operation: Option<EntityAsyncOperation<R, T, M1, M2, FinishSystem, ErrorSystem>>,
    build_commands: Option<EntityCommands<'a>>,
    _phantom1: PhantomData<M1>,
    _phantom2: PhantomData<M2>,
}

impl <
    'a,
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<(Entity, R)>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<(Entity, BevyError)>, (), M2> + Send + Sync + 'static,
> EntityAsyncOperationCmd<'a, R, T, M1, M2, FinishSystem, ErrorSystem> {
    pub fn on_finish_run_system<
        M3: Send + 'static,
        FinishSystem2: IntoSystem<In<(Entity, R)>, (), M3> + Send + Sync + 'static,
    > (mut self, system: FinishSystem2) -> EntityAsyncOperationCmd<'a, R, T, M3, M2, FinishSystem2, ErrorSystem> {
        let mut operation = self.operation.take().unwrap();

        EntityAsyncOperationCmd {
            operation: Some(EntityAsyncOperation {
                entity: operation.entity,
                operation: operation.operation.take(),
                on_finish_system: Some(system),
                on_error_system: operation.on_error_system.take(),
                _phantom1: PhantomData,
                _phantom2: PhantomData,
            }),
            build_commands: self.build_commands.take(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn on_error_run_system<
        M3: Send + 'static,
        ErrorSystem2: IntoSystem<In<(Entity, BevyError)>, (), M3> + Send + Sync + 'static,
    > (mut self, system: ErrorSystem2) -> EntityAsyncOperationCmd<'a, R, T, M1, M3, FinishSystem, ErrorSystem2> {
        let mut operation = self.operation.take().unwrap();

        EntityAsyncOperationCmd {
            operation: Some(EntityAsyncOperation {
                entity: operation.entity,
                operation: operation.operation.take(),
                on_finish_system: operation.on_finish_system.take(),
                on_error_system: Some(system),
                _phantom1: PhantomData,
                _phantom2: PhantomData,
            }),
            build_commands: self.build_commands.take(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl <
    'a,
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<(Entity, R)>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<(Entity, BevyError)>, (), M2> + Send + Sync + 'static,
> Drop for EntityAsyncOperationCmd<'a, R, T, M1, M2, FinishSystem, ErrorSystem> {
    fn drop(&mut self) {
        if
            let Some(operation) = self.operation.take() &&
            let Some(mut commands) = self.build_commands.take()
        {
            commands.commands().spawn(AsyncOperationRunner::new(operation));
        }
    }
}

impl
<
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<(Entity, R)>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<(Entity, BevyError)>, (), M2> + Send + Sync + 'static,
>
RunnableAsyncOperation for EntityAsyncOperation<R, T, M1, M2, FinishSystem, ErrorSystem> {

    fn run(&mut self, commands: &mut Commands) -> bool {
        let Some(operation) = self.operation.as_mut() else {
            return true;
        };

        if let Some(res) = block_on(poll_once(operation)) {
            match res {
                Ok(result) => {
                    if let Some(on_finish_system) = self.on_finish_system.take() {
                        commands.run_system_cached_with(on_finish_system, (self.entity, result));
                    }
                },
                Err(err) => {
                    if let Some(on_error_system) = self.on_error_system.take() {
                        commands.run_system_cached_with(on_error_system, (self.entity, err));
                    }
                },
            }

            true
        } else {
            false
        }
    }
}

