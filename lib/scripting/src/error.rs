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

use std::fmt::Error;

use log::error;
use thiserror::Error;

use crate::LuaRuntimeBuilderError;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error(transparent)]
    LuaError(#[from] mlua::Error),

    #[error(transparent)]
    LuaRuntimeBuilderError(#[from] LuaRuntimeBuilderError),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ScriptResult<T> = Result<T, ScriptError>;

pub trait ScriptResultExt {
    fn handle(self);
}

impl<T> ScriptResultExt for ScriptResult<T> {
    fn handle(self) {
        if let Err(e) = self {
            error!("Script error: {e}");
        }
    }
}