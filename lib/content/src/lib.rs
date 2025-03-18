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

use std::path::PathBuf;
use anyhow::anyhow;
use once_cell::sync::OnceCell;

mod combat_style;
pub mod error;

pub use combat_style::*;

pub(crate) static CONTENT_PATH: OnceCell<PathBuf> = OnceCell::new();

pub fn set_content_path(path: PathBuf) -> Result<(), error::Error> {
    CONTENT_PATH.set(path)
        .map_err(|_| error::Error::Other(anyhow!("content path already set")))
}

pub(crate) fn get_content_path(relative: impl Into<PathBuf>) -> Result<PathBuf, error::Error> {
    let content_path = CONTENT_PATH.get()
        .ok_or(error::Error::ContentPathNotSet)?;
    
    Ok(content_path.join(relative.into()))
}
