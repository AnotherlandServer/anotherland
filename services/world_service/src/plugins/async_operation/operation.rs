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

use bevy::{ecs::{error::{BevyError, Result}, system::{Commands, In, IntoSystem, IsFunctionSystem}}, tasks::{block_on, poll_once}};

use crate::plugins::async_operation::{noop_system::NoOpSystem, runner::{AsyncOperationRunner, RunnableAsyncOperation}};

pub trait AsyncOperationCommandsExt<'a> {
    fn perform_async_operation<
        R: Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send + 'static,
    >(self, operation: T) -> AsyncOperationCmd<'a, R, T, IsFunctionSystem, IsFunctionSystem, NoOpSystem<R>, NoOpSystem<BevyError>>;
}

impl <'a>AsyncOperationCommandsExt<'a> for Commands<'a, 'a> {
    fn perform_async_operation<
        R: Send + Sync + 'static, 
        T: Future<Output = Result<R>> + Send + 'static
    >(self, operation: T) -> AsyncOperationCmd<'a, R, T, IsFunctionSystem, IsFunctionSystem, NoOpSystem<R>, NoOpSystem<BevyError>> {
        AsyncOperationCmd {
            operation: Some(AsyncOperation { 
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

pub struct AsyncOperation
<
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<R>, (), M1> + Send + Sync + 'static = NoOpSystem<R>,
    ErrorSystem: IntoSystem<In<BevyError>, (), M2> + Send + Sync + 'static = NoOpSystem<BevyError>,
> {
    operation: Option<Pin<Box<T>>>,
    on_finish_system: Option<FinishSystem>,
    on_error_system: Option<ErrorSystem>,
    _phantom1: PhantomData<M1>,
    _phantom2: PhantomData<M2>,
}

pub struct AsyncOperationCmd <
    'a,
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<R>, (), M1> + Send + Sync + 'static = NoOpSystem<R>,
    ErrorSystem: IntoSystem<In<BevyError>, (), M2> + Send + Sync + 'static = NoOpSystem<BevyError>,
> {
    operation: Option<AsyncOperation<R, T, M1, M2, FinishSystem, ErrorSystem>>,
    build_commands: Option<Commands<'a, 'a>>,
    _phantom1: PhantomData<M1>,
    _phantom2: PhantomData<M2>,
}

impl <
    'a, 
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<R>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<BevyError>, (), M2> + Send + Sync + 'static,
> AsyncOperationCmd<'a, R, T, M1, M2, FinishSystem, ErrorSystem> {
    pub fn on_finish_run_system<
        M3: Send + 'static,
        FinishSystem2: IntoSystem<In<R>, (), M3> + Send + Sync + 'static,
    > (mut self, system: FinishSystem2) -> AsyncOperationCmd<'a, R, T, M3, M2, FinishSystem2, ErrorSystem> {
        let mut operation = self.operation.take().unwrap();

        AsyncOperationCmd {
            operation: Some(AsyncOperation { 
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
        ErrorSystem2: IntoSystem<In<BevyError>, (), M3> + Send + Sync + 'static,
    > (mut self, system: ErrorSystem2) -> AsyncOperationCmd<'a, R, T, M1, M3, FinishSystem, ErrorSystem2> {
        let mut operation = self.operation.take().unwrap();

        AsyncOperationCmd {
            operation: Some(AsyncOperation { 
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
    FinishSystem: IntoSystem<In<R>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<BevyError>, (), M2> + Send + Sync + 'static,
> Drop for AsyncOperationCmd<'a, R, T, M1, M2, FinishSystem, ErrorSystem> {
    fn drop(&mut self) {
        if 
            let Some(operation) = self.operation.take() &&
            let Some(mut commands) = self.build_commands.take() 
        {
            commands.spawn(AsyncOperationRunner::new(operation));
        }
    }
}

impl 
<
    R: Send + Sync + 'static,
    T: Future<Output = Result<R>> + Send + 'static,
    M1: Send + 'static,
    M2: Send + 'static,
    FinishSystem: IntoSystem<In<R>, (), M1> + Send + Sync + 'static,
    ErrorSystem: IntoSystem<In<BevyError>, (), M2> + Send + Sync + 'static,
> 
RunnableAsyncOperation for AsyncOperation<R, T, M1, M2, FinishSystem, ErrorSystem> {

    fn run(&mut self, commands: &mut Commands) -> bool {
        let Some(operation) = self.operation.as_mut() else {
            return true;
        };

        if let Some(res) = block_on(poll_once(operation)) {
            match res {
                Ok(result) => {
                    if let Some(on_finish_system) = self.on_finish_system.take() {
                        commands.run_system_cached_with(on_finish_system, result);
                    }
                },
                Err(err) => {
                    if let Some(on_error_system) = self.on_error_system.take() {
                        commands.run_system_cached_with(on_error_system, err);
                    }
                },
            }

            true
        } else {
            false
        }
    }
}
