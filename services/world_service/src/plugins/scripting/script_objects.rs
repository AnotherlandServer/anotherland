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

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{system::{In, Res}, world::World}, prelude::{Added, App, Commands, Entity, EntityWorldMut, Query, ResMut}};
use convert_case::{Case, Casing};
use log::{error, warn};
use mlua::{IntoLua, Lua, Table};
use obj_params::{Class, GameObjectData, GenericParamSet, GenericParamSetBoxExt, NonClientBase};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptApi, ScriptCommandsExt, ScriptObject, ScriptResult};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{AvatarInfo, ContentInfo, PlayerLocalSets}};

use super::{create_log_table, insert_portalbook_api, insert_timer_api, insert_world_api, param::ParamValue, timers::update_timers};

pub struct ScriptObjectInfoPlugin;

impl Plugin for ScriptObjectInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, attach_scripts);
        app.add_systems(Update, update_timers);

        insert_game_object_api(app.world_mut()).unwrap();
        insert_log_api(app.world_mut()).unwrap();
        insert_timer_api(app.world_mut()).unwrap();
        insert_world_api(app.world_mut()).unwrap();
        insert_portalbook_api(app.world_mut()).unwrap();
    }
}

fn insert_game_object_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("gameobject", object_api.clone()).unwrap();

    object_api.set("Get", lua.create_bevy_function(world, lua_gameobject_get)?)?;
    object_api.set("Set", lua.create_bevy_function(world, lua_gameobject_set)?)?;
    object_api.set("Reset", lua.create_bevy_function(world, lua_gameobject_reset)?)?;

    object_api.set("GetPlayerLocal", lua.create_bevy_function(world, |
            In((object, player, index)): In<(Table, Table, String)>,
            query: Query<(&GameObjectData, &PlayerLocalSets)>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let (gameobject, local_sets) = query.get(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let val = local_sets.0
                .get(&player.entity()?)
                .and_then(|local_set| local_set.get_param(&index))
                .unwrap_or(
                    gameobject.get_named::<obj_params::Value>(&index)
                        .map_err(mlua::Error::external)?
                );
        
            Ok(ParamValue::new(val.clone())
                .into_lua(runtime.vm())?)
        })?
    )?;

    object_api.set("SetPlayerLocal", lua.create_bevy_function(world, |
            In((object, player, index, value)): In<(Table, Table, String, mlua::Value)>,
            mut query: Query<(&GameObjectData, &mut PlayerLocalSets)>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let (gameobject, mut local_sets) = query.get_mut(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let attr = gameobject.class().get_attribute(&index)
                .ok_or(mlua::Error::runtime("attribute not found"))?;

            let value = ParamValue::from_lua(attr, value, runtime.vm())?;
            let prev_val =  local_sets.0
                .entry(player.entity()?)
                .or_insert(Box::<dyn GenericParamSet>::new_for_class(gameobject.class()))
                .set_param(&index, value.into());

            if let Some(prev_val) = prev_val {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(ParamValue::new(attr.default().clone()).into_lua(runtime.vm())?)
            }
        })?
    )?;

    object_api.set("ResetPlayerLocal", lua.create_bevy_function(world, |
            In((object, player, index,)): In<(Table, Table, String)>,
            mut query: Query<&mut PlayerLocalSets>,
            runtime: Res<LuaRuntime>,
        | -> WorldResult<mlua::Value> {
            let mut local_sets = query.get_mut(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;

            let prev_val = local_sets.0
                .get_mut(&player.entity()?)
                .and_then(|local_set| local_set.remove_param(&index));

            if let Some(prev_val) = prev_val {
                Ok(ParamValue::new(prev_val).into_lua(runtime.vm())?)
            } else {
                Ok(mlua::Value::Nil)
            }
        })?
    )?;

    Ok(())
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
    In((object, index)): In<(Table, String)>,
    query: Query<&GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let gameobject = query.get(object.entity()?)
        .map_err(|_| anyhow!("object not found"))?;

    let val = gameobject.get_named::<obj_params::Value>(&index)
        .map_err(mlua::Error::external)?;

    Ok(ParamValue::new(val.clone())
        .into_lua(runtime.vm())?)
}

fn lua_gameobject_set(
    In((object, index, value)): In<(Table, String, mlua::Value)>,
    mut query: Query<&mut GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let mut gameobject = query.get_mut(object.entity()?)
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

fn lua_gameobject_reset(
    In((object, index,)): In<(Table, String)>,
    mut query: Query<&mut GameObjectData>,
    runtime: Res<LuaRuntime>,
) -> WorldResult<mlua::Value> {
    let mut gameobject = query.get_mut(object.entity()?)
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
        match runtime.load_script(&format!("{}.{}", class.name().to_case(Case::Snake), script_name)) {
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
        match runtime.load_script(script_name) {
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

pub fn attach_scripts(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut runtime: ResMut<LuaRuntime>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        match load_class_script(&mut runtime, obj.class(), obj.get::<_, String>(NonClientBase::LuaScript).ok().map(|s| s.as_str())) {
            Ok(lua_class) => {
                commands.entity(ent)
                    .insert(ScriptObject::new(&runtime, Some(lua_class)).unwrap())
                    .queue(insert_object_info)
                    .call_named_lua_method(ScriptApi::Attach, ());
            },
            Err(e) => {
                error!("Failed to load script: {e}");
            },
        }
    }
}

fn insert_object_info(entity: EntityWorldMut<'_>) {
    let script = entity.get::<ScriptObject>().unwrap();
    let object = entity.get::<GameObjectData>().unwrap();
    let content_info = entity.get::<ContentInfo>();

    if let Some(avatar_info) = entity.get::<AvatarInfo>() {
        script.object().raw_set("avatar_id", avatar_info.id.as_u64()).unwrap();
        script.object().raw_set("name", avatar_info.name.clone()).unwrap();
    } else if let Some(content_info) = content_info {
        script.object().raw_set("name", content_info.template.name.clone()).unwrap();
    }

    script.object().raw_set("class", object.class().name()).unwrap();

    if let Some(content_info) = content_info {
        script.object().raw_set("placement_guid", content_info.placement_id.to_string()).unwrap();
        script.object().raw_set("template_guid", content_info.template.id.to_string()).unwrap();
    }
}

