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

use log::{logger, Level, RecordBuilder};
use mlua::{Lua, MultiValue, Table};

use crate::error::WorldResult;

pub fn create_log_table(lua: &Lua) -> WorldResult<Table> {
    let log = lua.create_table()?;

    log.set("Trace", lua.create_function(|lua, args: MultiValue| {
        lua_log(lua, Level::Trace, args)
    })?)?;

    log.set("Debug", lua.create_function(|lua, args: MultiValue| {
        lua_log(lua, Level::Debug, args)
    })?)?;

    log.set("Info", lua.create_function(|lua, args: MultiValue| {
        lua_log(lua, Level::Info, args)
    })?)?;

    log.set("Warn", lua.create_function(|lua, args: MultiValue| {
        lua_log(lua, Level::Warn, args)
    })?)?;

    log.set("Err", lua.create_function(|lua, args: MultiValue| {
        lua_log(lua, Level::Error, args)
    })?)?;

    Ok(log)
}

fn lua_log(lua: &Lua, level: log::Level, args: MultiValue) -> Result<(), mlua::Error> {
    let msg = args.iter()
        .map(|v| v.to_string())
        .collect::<Result<Vec<String>, mlua::Error>>()?
        .concat();

    let debug = lua.inspect_stack(1).unwrap();

    let target = if let Some(source) = debug.source().source {
        if debug.source().what.is_empty() {
            &format!("lua::{source}")
        } else {
            &format!("lua::{source}::{}", debug.source().what)
        }
    } else {
        "lua"
    };

    logger().log(&RecordBuilder::new()
        .target(target)
        .line(Some(debug.curr_line() as u32))
        .level(level)
        .args(format_args!("{msg}"))
        .build());

    Ok(())
}