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

use bevy::{app::{Last, Plugin, Update}, ecs::{component::Component, query::With, system::{In, Res}, world::World}, prelude::{App, Entity, EntityWorldMut, Query}};
use convert_case::{Case, Casing};
use log::{error, warn};
use mlua::{Function, IntoLua, Table};
use obj_params::{Class, GameObjectData, GenericParamSet, GenericParamSetBoxExt};
use scripting::{LuaEntity, LuaRuntime, ScriptAppExt, ScriptObject};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{Avatar, ContentInfo, DespawnEntity, PlayerLocalSets, handle_despawn_entity, on_init_script, on_remove_script}};

use super::{create_log_table, insert_timer_api, insert_world_api, param::ParamValue, timers::update_timers};

pub struct ScriptObjectInfoPlugin;

impl Plugin for ScriptObjectInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_timers);
        app.add_systems(Last, handle_despawn_entity);

        app.add_message::<DespawnEntity>();

        insert_game_object_api(app);
        insert_log_api(app.world_mut()).unwrap();
        insert_timer_api(app);
        insert_world_api(app);

        app.add_observer(on_init_script);
        app.add_observer(on_remove_script);
    }
}

#[derive(Component)]
pub struct SpawnCallback(pub Function);

fn insert_game_object_api(app: &mut App) {
    app
        .add_lua_api("gameobject", "IsValid", 
        |
            In(object): In<LuaEntity>,
            query: Query<Entity, With<GameObjectData>>,
        | -> WorldResult<bool> {
            Ok(
                query
                    .contains(object.entity())
            )
        })
        .add_lua_api("gameobject", "Get", lua_gameobject_get)
        .add_lua_api("gameobject", "Set", lua_gameobject_set)
        .add_lua_api("gameobject", "ForceSet", lua_gameobject_force_set)
        .add_lua_api("gameobject", "Reset", lua_gameobject_reset)
        .add_lua_api("gameobject", "GetPlayerLocal", 
        |
            In((object, player, index)): In<(LuaEntity, LuaEntity, String)>,
            query: Query<(&GameObjectData, &PlayerLocalSets)>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let (gameobject, local_sets) = query.get(object.entity())
                .map_err(|_| anyhow!("object not found"))?;

            let val = local_sets.0
                .get(&player.entity())
                .and_then(|local_set| local_set.get_param(&index))
                .unwrap_or(
                    gameobject.get_named::<obj_params::Value>(&index)
                        .map_err(mlua::Error::external)?
                );
        
            Ok(ParamValue::new(val.clone())
                .into_lua(runtime.vm())?)
        })
        .add_lua_api("gameobject", "SetPlayerLocal", 
        |
            In((object, player, index, value)): In<(LuaEntity, LuaEntity, String, mlua::Value)>,
            mut query: Query<(&GameObjectData, &mut PlayerLocalSets)>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let (gameobject, mut local_sets) = query.get_mut(object.entity())
                .map_err(|_| anyhow!("object not found"))?;

            let attr = gameobject.class().get_attribute(&index)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            let value = ParamValue::from_lua(attr, value, runtime.vm())?;
            let prev_val =  local_sets.0
                .entry(player.entity())
                .or_insert(Box::<dyn GenericParamSet>::new_for_class(gameobject.class()))
                .set_param(&index, value.into());

            if let Some(prev_val) = prev_val {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
            }
        })
        .add_lua_api("gameobject", "ResetPlayerLocal", 
        |
            In((object, player, index,)): In<(LuaEntity, LuaEntity, String)>,
            mut query: Query<&mut PlayerLocalSets>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let mut local_sets = query.get_mut(object.entity())
                .map_err(|_| anyhow!("object not found"))?;

            let prev_val = local_sets.0
                .get_mut(&player.entity())
                .and_then(|local_set| local_set.remove_param(&index));

            if let Some(prev_val) = prev_val {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(mlua::Value::Nil)
            }
        });
}

fn insert_log_api(
    world: &mut World,
) -> WorldResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();

    let table = create_log_table(runtime.vm())?;
    runtime.vm().globals().set("Log", table)?;

    Ok(())
}

fn lua_gameobject_get(
    In((object, index)): In<(LuaEntity, String)>,
    query: Query<&GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let gameobject = query.get(object.entity())
        .map_err(|_| anyhow!("object not found"))?;

    let val = gameobject.get_named::<obj_params::Value>(&index)
        .map_err(mlua::Error::external)?;

    Ok(ParamValue::new(val.clone())
        .into_lua(runtime.vm())?)
}

fn lua_gameobject_set(
    In((object, index, value)): In<(LuaEntity, String, mlua::Value)>,
    mut query: Query<&mut GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let mut gameobject = query.get_mut(object.entity())
        .map_err(|_| anyhow!("object not found"))?;

    let attr = gameobject.class().get_attribute(&index)
        .ok_or(mlua::Error::runtime("attribute not found"))?;

    let value = ParamValue::from_lua(attr, value, runtime.vm())?;

    if let Some(prev_val) = gameobject.set_named(&index, value) {
        Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
    } else {
        Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
    }
}

fn lua_gameobject_force_set(
    In((object, index, value)): In<(LuaEntity, String, mlua::Value)>,
    mut query: Query<&mut GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let mut gameobject = query.get_mut(object.entity())
        .map_err(|_| anyhow!("object not found"))?;

    let attr = gameobject.class().get_attribute(&index)
        .ok_or(mlua::Error::runtime("attribute not found"))?;

    let value = ParamValue::from_lua(attr, value, runtime.vm())?;

    if let Some(prev_val) = gameobject.force_set_named(&index, value) {
        Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
    } else {
        Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
    }
}

fn lua_gameobject_reset(
    In((object, index,)): In<(LuaEntity, String)>,
    mut query: Query<&mut GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let mut gameobject = query.get_mut(object.entity())
        .map_err(|_| anyhow!("object not found"))?;

    let attr = gameobject.class().get_attribute(&index)
        .ok_or(mlua::Error::runtime("attribute not found"))?;

    if let Some(prev_val) = gameobject.set_named(&index, attr.default().clone()) {
        Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
    } else {
        Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
    }
}


pub fn load_class_script(runtime: &mut LuaRuntime, class: Class, name: Option<&str>) -> WorldResult<Table> {
    let mut object_scripts = vec![];
        
    if 
        let Some(script_name) = name &&
        !script_name.is_empty()
    {
        match runtime.load_class(&format!("{}.{}", class.name().to_case(Case::Snake), script_name)) {
            Ok(lua_class) => {
                return Ok(lua_class);
            },
            Err(e) => {
                if matches!(e, scripting::ScriptError::FileNotFound(_)) {
                    warn!("Script '{script_name}' not found, falling back to class script.");
                } else {
                    error!("Failed to load script '{script_name}': {e}");
                    return Ok(runtime.vm().create_table()?);
                }
            },
        }
    }

    let mut current_class = Some(class);
    while let Some(class) = current_class {
        object_scripts.push(format!("global.base.{}", class.name().to_case(Case::Snake)));
        current_class = class.parent();
    }

    object_scripts.push("global.base.entity".to_string());

    for script_name in &object_scripts {
        match runtime.load_class(script_name) {
            Ok(lua_class) => {
                return Ok(lua_class);
            },
            Err(e) => {
                if matches!(e, scripting::ScriptError::FileNotFound(_)) {
                    continue;
                }

                error!("Failed to load script '{script_name}': {e}");
                break;
            },
        }
    }

    Ok(runtime.vm()
        .create_table()?)
}

pub fn insert_object_info(entity: EntityWorldMut<'_>) {
    let script = entity.get::<ScriptObject>().unwrap();
    let object = entity.get::<GameObjectData>().unwrap();
    let content_info = entity.get::<ContentInfo>();

    if let Some(avatar_info) = entity.get::<Avatar>() {
        script.object().raw_set("avatar_id", avatar_info.id).unwrap();
        script.object().raw_set("name", avatar_info.name.clone()).unwrap();
    } else if let Some(content_info) = content_info {
        script.object().raw_set("name", content_info.template.name.clone()).unwrap();
    }

    script.object().raw_set("class", object.class().name()).unwrap();

    if let Some(content_info) = content_info {
        script.object().raw_set("placement_guid", content_info.placement_id.to_string()).unwrap();
        script.object().raw_set("template_guid", content_info.template.id.to_string()).unwrap();
        script.object().raw_set("template_numeric_id", content_info.template.numeric_id).unwrap();
    }
}

