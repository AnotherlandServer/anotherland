// Copyright (C) 2024 AnotherlandServer
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

use std::{env::current_dir, path::{Path, PathBuf}};

pub mod dialogue;
pub mod quest;
pub mod script;

pub fn get_content_path() -> PathBuf {
    let mut content_path = if let Ok(mut current_dir) = current_dir() {
        current_dir.push("content");
        if current_dir.exists() {
            Some(current_dir)
        } else {
            None
        }
    } else {
        None
    };

    if content_path.is_none() {
        let path = PathBuf::from("/usr/local/lib/anotherland/content");
        if path.exists() {
            content_path = Some(path);
        }
    }

    content_path.expect("content directory not found!")
}