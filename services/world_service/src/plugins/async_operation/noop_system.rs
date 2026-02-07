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

use std::{any::TypeId, marker::PhantomData, result::Result};

use bevy::ecs::{change_detection::{CheckChangeTicks, Tick}, query::FilteredAccessSet, system::{In, IntoSystem, IsFunctionSystem, RunSystemError, System, SystemParamValidationError, SystemStateFlags}, world::{DeferredWorld, World, unsafe_world_cell::UnsafeWorldCell}};

pub struct NoOpSystem<T: Send + Sync + 'static>(PhantomData<T>);

impl<T: Send + Sync + 'static> System for NoOpSystem<T> {
    type In = In<T>;
    type Out = ();

    fn name(&self) -> bevy::utils::prelude::DebugName {
        unreachable!()
    }

    fn flags(&self) -> SystemStateFlags {
        unreachable!()
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: bevy::ecs::system::SystemIn<'_, Self>,
        _world: UnsafeWorldCell,
    ) -> Result<Self::Out, RunSystemError> {
        unreachable!()
    }

    fn apply_deferred(&mut self, _world: &mut World) {
        unreachable!()
    }

    fn queue_deferred(&mut self, _world: DeferredWorld) {
        unreachable!()
    }

    unsafe fn validate_param_unsafe(
        &mut self,
        _world: UnsafeWorldCell,
    ) -> Result<(), SystemParamValidationError> {
        unreachable!()
    }

    fn initialize(&mut self, _world: &mut World) -> FilteredAccessSet {
        unreachable!()
    }

    fn check_change_tick(&mut self, _check: CheckChangeTicks) {
        unreachable!()
    }

    fn get_last_run(&self) -> Tick {
        unreachable!()
    }

    fn set_last_run(&mut self, _last_run: Tick) {
        unreachable!()
    }
    
    fn type_id(&self) -> TypeId {
        unreachable!()
    }
    
    fn is_send(&self) -> bool {
        unreachable!()
    }
    
    fn is_exclusive(&self) -> bool {
        unreachable!()
    }
    
    fn has_deferred(&self) -> bool {
        unreachable!()
    }
    
    fn run(
        &mut self,
        _input: bevy::ecs::system::SystemIn<'_, Self>,
        _world: &mut World,
    ) -> Result<Self::Out, RunSystemError> {
        unreachable!()
    }
    
    fn run_without_applying_deferred(
        &mut self,
        _input: bevy::ecs::system::SystemIn<'_, Self>,
        _world: &mut World,
    ) -> Result<Self::Out, RunSystemError> {
        unreachable!()
    }
    
    fn validate_param(&mut self, _world: &World) -> Result<(), SystemParamValidationError> {
        unreachable!()
    }
    
    fn default_system_sets(&self) -> Vec<bevy::ecs::schedule::InternedSystemSet> {
        unreachable!()
    }
}

impl <T: Send + Sync + 'static> IntoSystem<In<T>, (), IsFunctionSystem> for NoOpSystem<T> {
    type System = NoOpSystem<T>;

    fn into_system(_this: Self) -> Self::System {
        unreachable!()
    }
}
