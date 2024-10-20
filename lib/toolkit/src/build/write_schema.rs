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

use std::process::Command;

pub fn build_schema(service: &str) {
    let output = Command::new(service)
        .arg("--sdl")
        .output()
        .unwrap_or_else(|_| panic!("failed to get {} schema", service));

    let sdl = String::from_utf8(output.stdout).unwrap();

    cynic_codegen::register_schema(service)
        .from_sdl(&sdl)
        .unwrap();
}