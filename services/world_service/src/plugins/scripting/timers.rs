// Copyright (C) 2026 AnotherlandServer
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

use bevy::{app::App, ecs::{component::Component, entity::Entity, hierarchy::ChildOf, system::{Commands, In, Query, Res}, world::EntityWorldMut}, time::{Stopwatch, Time, Virtual}};
use log::debug;
use mlua::{Function, Table};
use scripting::{EntityScriptCommandsExt, LuaEntity, LuaRuntime, ScriptAppExt, ScriptObject};

use crate::{error::WorldResult, plugins::ScriptingEntityCommandsExt};

#[derive(Component)]
pub struct LuaTimer {
    interval: f32,
    stopwatch: Stopwatch,
    callback: Function,
}

pub fn update_timers(
    mut query: Query<(Entity, &mut LuaTimer)>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.stopwatch.tick(time.delta());

        if timer.stopwatch.elapsed_secs() >= timer.interval {
            commands
                .entity(entity)
                .call_lua_method(timer.callback.clone(), LuaEntity(entity));

            timer.stopwatch.reset();
        }
    }
}

pub fn insert_timer_api(app: &mut App) {
    app
        .add_lua_api("timer", "CreateTimer", 
        |
            In((owner, interval, callback)): In<(LuaEntity, f32, Function)>,
            runtime: Res<LuaRuntime>,
            mut commands: Commands
        | -> WorldResult<Table> {
            let id = commands.spawn_empty().id();
            let obj = ScriptObject::new(id, runtime.vm(), None)?;
            let table = obj.object().clone();

            commands
                .entity(id)
                .insert((
                    obj,
                    LuaTimer {
                        interval,
                        stopwatch: Stopwatch::new(),
                        callback: callback.clone(),
                    },
                    ChildOf(owner.entity()),
                ));

            Ok(table)
        })
        .add_lua_api("timer", "DestroyTimer", 
        |
            In(timer): In<LuaEntity>,
            mut commands: Commands
        | -> WorldResult<()> {
            commands
                .entity(timer.entity())
                .deferred_despawn();

            Ok(())
        });
}