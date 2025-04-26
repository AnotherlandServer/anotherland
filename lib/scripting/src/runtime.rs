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

use std::{fs, path::{Path, PathBuf}, sync::mpsc::Receiver, time::Duration};

use bevy::prelude::{Commands, Entity, NonSendMut, Query, ResMut, Resource, World};
use derive_builder::Builder;
use log::{debug, error, info, trace, warn};
use mlua::{ffi::{LUA_LOADED_TABLE, LUA_PRELOAD_TABLE}, Function, IntoLua, Lua, LuaOptions, StdLib, Table, Value};
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use toolkit::init_vector_api;

use crate::{api_names::ScriptApi, ScriptCommandsExt, ScriptError, ScriptResult, ScriptObject};

pub(crate) const REG_WORLD: &str = "world";

#[derive(Resource, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "build_private"))]
pub struct LuaRuntime {
    #[builder(private)]
    pub(crate) lua: Lua,

    //#[builder(private)]
    //pub(crate) scoped_global: Table,

    #[builder(setter(custom), field(ty = "Vec<PathBuf>"))]
    pub(crate) require_lookup_directories: Vec<PathBuf>,

    #[builder(default)]
    pub(crate) entity_meta_tables: Vec<Table>,

    #[builder(default)]
    pub(crate) hot_reload: bool,
}

impl LuaRuntimeBuilder {
    pub fn add_require_lookup_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.require_lookup_directories.push(path.into());
        self
    }

    pub fn build(self) -> ScriptResult<LuaRuntime> {
        let lua = Lua::new_with(
        StdLib::TABLE | 
            StdLib::STRING |
            StdLib::UTF8 |
            StdLib::MATH |
            StdLib::PACKAGE, 
            LuaOptions::default()
        )?;

        let configured_paths = self.require_lookup_directories.iter()
            .map(|p| {
                [
                    p.join("?.lua").display().to_string(),
                    p.join("?/init.lua").display().to_string()
                ].join(";")
            })
            .collect::<Vec<_>>()
            .join(";");

        // Prepare require
        let package = lua.globals().get::<Table>("package")?;
        package.set("path", format!("./?.lua;./?/init.lua;{configured_paths}"))?;

        // Scope global context for module loading
        let scoped_global = lua.create_table()?;
        scoped_global.set("__index", lua.globals())?;

        // Replace require to facilitate hot-reload
        if self.hot_reload.unwrap_or_default() {
            let require = lua.globals().get::<Function>("require")?;

            lua.set_named_registry_value("_MODULE_PATHS", lua.create_table()?)?;
            lua.set_named_registry_value("_MODULE_ENVS", lua.create_table()?)?;

            lua.globals().set("require", lua.create_function(move |lua: &Lua, modname: String| {
                let (module, loader_data): (Value, Value) = require.call(modname.clone())?;

                if let Some(loader_data) = loader_data.as_string() {
                    let paths = lua.named_registry_value::<Table>("_MODULE_PATHS")?;
                    let loaded = lua.named_registry_value::<Table>(LUA_LOADED_TABLE)?;

                    if loader_data != ":preload:" {
                        let path = loader_data.to_str()?.parse::<PathBuf>().unwrap().canonicalize().unwrap();
                        paths.set(path.display().to_string(), modname.clone())?;

                        debug!("Loaded: {} from {}", modname, path.display());
                    }

                    if let Some(module) = module.as_table() {
                        let hotreload = lua.create_table()?;
                        hotreload.set_metatable(Some(create_hotreload_indirection(lua, module.clone())?));

                        loaded.set(modname, &hotreload)?;

                        Ok((hotreload.into_lua(lua)?, loader_data.into_lua(lua)?))
                    } else {
                        Ok((module, loader_data.into_lua(lua)?))
                    }
                } else {
                    Ok((module, loader_data))
                }
            })?)?;
        }

        lua.globals().set("__engine", lua.create_table()?)?;

        init_vector_api(&lua)?;

        Ok(self
            .lua(lua.clone())
            //.scoped_global(scoped_global)
            .build_private()?)
    }
}

#[derive(Clone, Copy)]
pub enum ApiType {
    Script,
    Player,
    Npc,
}

impl ApiType {
    pub fn name(&self) -> &str {
        match self {
            ApiType::Script => "SCRIPT",
            ApiType::Player => "PLAYER",
            ApiType::Npc => "NPC",
        }
    }

    pub fn base(&self) -> &str {
        match self {
            ApiType::Script => "_script",
            ApiType::Player => "_player",
            ApiType::Npc => "_npc",
        }
    }
}

impl LuaRuntime {
    pub fn vm(&self) -> &Lua { &self.lua }

    pub fn register_native(&self, name: &str, functions: Table) -> ScriptResult<()> {
        let globals = self.lua.globals();
        let native_api = globals.get::<Table>("__engine")?;
        native_api.set(name, functions)?;
        Ok(())
    }

    pub fn load_script(&mut self, name: &str) -> ScriptResult<Table> {
        let loaded = self.lua.named_registry_value::<Table>(LUA_LOADED_TABLE)?;
        if let Ok(script) = loaded.get::<Table>(name) {
            return Ok(script);
        }

        let file_path = self.require_lookup_directories.iter()
            .flat_map(|p| {
                [
                    p.join(format!("{}.lua", name.replace('.', std::path::MAIN_SEPARATOR_STR))),
                    p.join(format!("{}/init.lua", name.replace('.', std::path::MAIN_SEPARATOR_STR)))
                ]
            })
            .find(|p| p.is_file())
            .and_then(|p| p.canonicalize().ok());

        if let Some(file_path) = file_path {
            if let Ok(content) = fs::read_to_string(&file_path) {
                debug!("Loading '{}' from {}", name, file_path.display());

                // Load chunk
                let res = self.lua.load(content)
                    .set_name(format!("@{}", file_path.display()))
                    .call::<Table>(());

                if let Ok(mut base) = res.clone() {
                    // Register module
                    if self.hot_reload {
                        debug!("Register: {} for hot reloading", file_path.display());

                        let paths = self.lua.named_registry_value::<Table>("_MODULE_PATHS")?;
                        paths.set(file_path.display().to_string(), name)?;

                        let indirect_base = self.lua.create_table()?;
                        indirect_base.set_metatable(Some(create_hotreload_indirection(&self.lua, base)?));

                        base = indirect_base;
                    }

                    let loaded = self.lua.named_registry_value::<Table>(LUA_LOADED_TABLE)?;
                    loaded.set(name, &base)?;

                    // Register the metatable
                    self.lua.set_named_registry_value(name, &base)?;
                    self.entity_meta_tables.push(base.clone());

                    Ok(base.clone())
                } else if self.hot_reload {
                    // Warn instead of hard error in hot reload mode,
                    // so we can try to reload the script later and still
                    // have all the references ready.
                    warn!("{:?}", res.err().unwrap());

                    let loaded = self.lua.named_registry_value::<Table>(LUA_LOADED_TABLE)?;
                    let base = if let Ok(base) = loaded.get(name) {
                        base
                    } else {
                        self.lua.create_table()?
                    };

                    Ok(base)
                } else {
                    Err(res.err().unwrap().into())
                }
            } else {
                Err(anyhow::Error::msg("failed to read file").into())    
            }
        } else {
            Err(ScriptError::FileNotFound(name.to_string()))
        }
    }

    fn hot_reload_script(&mut self, path: &Path) -> ScriptResult<()> {
        let paths = self.lua.named_registry_value::<Table>("_MODULE_PATHS")?;
        let loaded = self.lua.named_registry_value::<Table>(LUA_LOADED_TABLE)?;

        if 
            let Ok(module_name) = paths.get::<String>(path.display().to_string()) &&
            let Ok(module) = loaded.get::<Table>(module_name.clone())
        {
            if let Ok(content) = fs::read_to_string(path) {
                info!("Hot-reloading: {}", path.display());
    
                let base = self.lua.load(content)
                    .set_name(format!("@{}", path.display()))
                    .call::<Table>(())?;

                // Mark object for hot reloading
                if !module.contains_key("__hot_reload")? {
                    module.raw_set("__hot_reload", true)?;
                }

                module.set_metatable(Some(create_hotreload_indirection(&self.lua, base)?));

                Ok(())
            } else {
                Err(anyhow::Error::msg("failed to read file").into())
            }
        } else {
            trace!("Ignoring hot reload event for {}. Script not loaded.", path.display());
            Ok(())
        }
    }

    pub fn add_module<F>(&self, name: &str, func: F) -> ScriptResult<()> 
    where
        F: Fn(&Lua, mlua::String) -> mlua::Result<Table> + mlua::MaybeSend + 'static,
    {
        let preload = self.lua.named_registry_value::<Table>(LUA_PRELOAD_TABLE)?;
        preload.set(name, self.lua.create_function(func)?)?;
        Ok(())
    }
}

#[derive(Resource, Default)]
pub(crate) struct HotReloadEnabled;

pub(crate) struct HotReloadEvent(Receiver<PathBuf>);

pub(crate) fn prepare_hot_reload(
    world: &mut World,
) {
    let runtime = world.get_resource::<LuaRuntime>()
        .expect("Lua runtime not created");

    if runtime.hot_reload {
        let (sender, receiver) = std::sync::mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |result: DebounceEventResult| {
            match result {
                Ok(events) => {
                    for ev in events {
                        if let EventKind::Modify(_) = ev.event.kind {
                            for path in ev.event.paths {
                                let _ = sender.send(path);
                            }
                        }
                    }
                },
                Err(errors) => {
                    for err in errors {
                        error!("{err:?}");
                    }
                },
            }
        }).unwrap();

        for path in &runtime.require_lookup_directories {
            if path.is_dir() {
                debouncer.watch(path, RecursiveMode::Recursive)
                    .expect("unable to watch path");
            }
        }

        world.init_resource::<HotReloadEnabled>();
        world.insert_non_send_resource(HotReloadEvent(receiver));
        world.insert_non_send_resource(debouncer);

        info!("Hot-reload active!");
    }
}

fn create_hotreload_indirection(lua: &Lua, base: Table) -> mlua::Result<Table> {
    let metatable = lua.create_table()?;
    metatable.set("__index", base.clone())?;
    metatable.set("__eq", lua.create_function(move |_, (a, b): (Table, Table)| {
        /*if a.contains_key("__index")? && a.get::<Table>("__index")? == b {
            Ok(true)
        } else if b.contains_key("__index")? {
            Ok(a == b.get::<Table>("__index")?)
        } else {
            Ok(a == b)
        }*/

        Ok(a == base || b == base)
    })?)?;

    Ok(metatable)
}

pub(crate) fn hot_reload(
    recv: NonSendMut<HotReloadEvent>,
    mut runtime: ResMut<LuaRuntime>,
    query: Query<(Entity, &ScriptObject)>,
    mut commands: Commands,
) {
    let mut had_events = false;

    // Ingest all events
    while let Ok(path) = recv.0.try_recv() {
        if let Err(e) = runtime.hot_reload_script(&path.canonicalize().unwrap()) {
            error!("Hot-reload failed: {e:?}");
        }

        had_events = true;
    }

    if had_events {
        // Trigger hot reload for all affected entities
        for (ent, script) in query.iter() {
            if let Ok(true) = script.object.get::<bool>("__hot_reload") {
                commands.entity(ent)
                    .call_named_lua_method(ScriptApi::HotReload, ());
            }
        }

        // Remove hot reload markers
        let loaded = runtime.lua.named_registry_value::<Table>(LUA_LOADED_TABLE).unwrap();
        let _ = loaded.for_each(|_: Value, module: Value| {
            if let Some(module) = module.as_table() {
                module.raw_remove("__hot_reload")?;
            }

            Ok(())
        });

    }
}