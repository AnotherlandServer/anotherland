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

use anyhow::anyhow;
use bevy::{app::App, ecs::{component::Component, entity::Entity, resource::Resource, system::{In, ResMut}, world::World}, platform::collections::HashMap};
use bonsai_bt::{Behavior, Event, Status, UpdateArgs, BT};
use log::error;
use mlua::Table;
use obj_params::GameObjectData;
use scripting::{LuaEntity, LuaFunctionExt, LuaRuntime, ScriptAppExt, ScriptObject};

use crate::{error::{WorldError, WorldResult}, plugins::Active};

#[derive(Resource, Default)]
pub struct AiStates(pub HashMap<Entity, BT<Actions, HashMap<String, i32>>>);

pub(super) fn insert_npc_ai_api(app: &mut App) {
    app
        .add_lua_api("ai", "InstallBehavior",
        |
            In((obj, behavior)): In<(LuaEntity, Table)>,
            mut states: ResMut<AiStates>,
        | -> WorldResult<()> {
            states.0.insert(
                obj.take(), 
                BT::new(
                    parse_lua_behavior(behavior)?, 
                    HashMap::new(),
                ));

            Ok(())
        })
        .add_lua_api("ai", "CancelBehavior",
        |
            In(obj): In<LuaEntity>,
            mut states: ResMut<AiStates>,
        | -> WorldResult<()> {
            if let Some(state) = states.0.get_mut(&obj.entity()) {
                state.reset_bt();
            } else {
                return Err(WorldError::Other(anyhow!("Object not found or has no AI state")));
            }

            Ok(())
        });
}

pub(super) fn parse_lua_behavior(table: Table) -> WorldResult<Behavior<Actions>> {
    let tag = table.get::<String>("tag")?;

    match tag.as_str() {
        "Wait" => Ok(Behavior::Wait(table.get::<f64>("value")?)),
        "WaitForever" => Ok(Behavior::WaitForever),
        "ActionScript" => Ok(Behavior::Action(
            Actions::Script(table.get::<mlua::Function>("value")?)
        )),
        "Invert" => {
            Ok(Behavior::Invert(Box::new(
                parse_lua_behavior(table.get::<Table>("value")?)?
            )))
        },
        "AlwaysSucceed" => {
            Ok(Behavior::AlwaysSucceed(Box::new(
                parse_lua_behavior(table.get::<Table>("value")?)?
            )))
        },
        "Select" => {
            let children: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::Select(children))
        },
        "If" => {
            let condition = parse_lua_behavior(table.get::<Table>("condition")?)?;
            let then_branch = parse_lua_behavior(table.get::<Table>("cond_true")?)?;
            let else_branch = parse_lua_behavior(table.get::<Table>("cond_false")?)?;

            Ok(Behavior::If(Box::new(condition), Box::new(then_branch), Box::new(else_branch)))
        },
        "Sequence" => {
            let children: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::Sequence(children))
        },
        "While" => {
            let condition = parse_lua_behavior(table.get::<Table>("condition")?)?;
            let body: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::While(Box::new(condition), body))
        },
        "WhileAll" => {
            let condition = parse_lua_behavior(table.get::<Table>("condition")?)?;
            let body: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::WhileAll(Box::new(condition), body))
        },
        "WhenAll" => {
            let children: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::WhenAll(children))
        },
        "WhenAny" => {
            let children: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::WhenAny(children))
        },
        "After" => {
            let children: Vec<Behavior<Actions>> = table.get::<Vec<Table>>("children")?
                .into_iter()
                .map(parse_lua_behavior)
                .collect::<WorldResult<Vec<_>>>()?;
            Ok(Behavior::After(children))
        },
        _ => Err(WorldError::Other(anyhow!("Unknown behavior tag: {}", tag)))
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub enum Actions {
    Script(mlua::Function)
}

#[derive(Component)]
pub struct AiAgent;

pub(super) fn ai_tick(world: &mut World) {
    let _ = world.resource_scope::<AiStates, WorldResult<()>>(|world, mut states| {
        let lua = world.get_resource::<LuaRuntime>()
            .expect("Lua runtime not created")
            .vm()
            .clone();

        for (ent, state) in states.0.iter_mut() {
            let update_event: Event = UpdateArgs { dt: 0.1 }.into();

            if 
                let Ok(entity) = world.get_entity(*ent) &&
                let Some(obj) = entity.get::<ScriptObject>() &&
                let Some(data) = entity.get::<GameObjectData>() &&
                entity.get::<Active>().is_some() &&
                *data.get_named::<bool>("alive").unwrap_or(&false)
            {
                let obj = obj.object().clone();

                let status = state.tick(&update_event, &mut |args, _blackboard| {
                    #[allow(unreachable_patterns)]
                    match args.action {
                        Actions::Script(func) => {
                            match func.call_with_world::<(i32, f64)>(&lua, world, (obj.clone(), args.dt)) {
                                Ok((0, dt)) => (Status::Success, dt),
                                Ok((1, dt)) => (Status::Running, dt),
                                Ok((2, dt)) => (Status::Failure, dt),
                                Ok((ret, _)) => {
                                    error!("Invalid return value '{ret}' from AI script");
                                    (Status::Failure, args.dt)
                                }
                                Err(e) => {
                                    error!("Error executing AI script: {e}");
                                    (Status::Failure, args.dt)
                                }
                            }
                        },
                        _ => {
                            (Status::Running, args.dt)
                        }
                    }
                });

                if 
                    let Some((tree_state, _)) = status && 
                    tree_state != Status::Running 
                {
                    state.reset_bt();
                }
            }
        }

        Ok(())
    });
}
