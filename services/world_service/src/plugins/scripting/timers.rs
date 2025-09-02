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

use bevy::{ecs::{component::Component, entity::Entity, hierarchy::ChildOf, system::{Commands, In, Query, Res}, world::World}, time::{Stopwatch, Time, Virtual}};
use mlua::{Function, Lua, Table};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, EntityScriptCommandsExt, ScriptObject, ScriptResult};

use crate::error::WorldResult;

#[derive(Component)]
pub struct LuaTimer {
    interval: f32,
    stopwatch: Stopwatch,
    callback: Function,
}

pub fn update_timers(
    mut query: Query<(Entity, &ScriptObject, &mut LuaTimer)>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
) {
    for (entity, obj, mut timer) in query.iter_mut() {
        timer.stopwatch.tick(time.delta());

        if timer.stopwatch.elapsed_secs() >= timer.interval {
            commands
                .entity(entity)
                .call_lua_method(timer.callback.clone(), obj.object().clone());

            timer.stopwatch.reset();
        }
    }
}

pub fn insert_timer_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("timer", object_api.clone()).unwrap();

    object_api.set("CreateTimer", lua.create_bevy_function(world,         |
        In((owner, interval, callback)): In<(Table, f32, Function)>,
        runtime: Res<LuaRuntime>,
        mut commands: Commands
    | -> WorldResult<Table> {
        let owner_ent = owner.entity()?;

        let obj = ScriptObject::new(&runtime, None)?;
        let table = obj.object().clone();

        commands
            .spawn((
                obj,
                LuaTimer {
                    interval,
                    stopwatch: Stopwatch::new(),
                    callback: callback.clone(),
                },
                ChildOf(owner_ent),
            ));

        Ok(table)
    })?)?;

    object_api.set("DestroyTimer", lua.create_bevy_function(world, |
        In(timer): In<Table>,
        mut commands: Commands
    | -> WorldResult<()> {
        let ent = timer.entity()?;

        commands
            .entity(ent)
            .despawn();

        Ok(())
    })?)?;
    
    Ok(())
}