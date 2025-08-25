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

use std::{env, fs, io, path::{Path, PathBuf}};

use param_generator::generate_param_code;
use proc_macro2::TokenStream;
use toml::Table;

mod param_generator;


/// Find the workspace root by walking up from `start` and looking for a
/// `Cargo.toml` that contains a top-level `[workspace]` table.
///
/// Returns the directory path that contains that `Cargo.toml`.
/// If none is found up to the filesystem root, returns `io::ErrorKind::NotFound`.
pub fn find_workspace_root(start: &Path) -> io::Result<PathBuf> {
    let mut current = if start.is_file() { start.parent() } else { Some(start) };

    while let Some(dir) = current {
        let cargo_toml_path = dir.join("Cargo.toml");
        if cargo_toml_path.is_file() {
            match fs::read_to_string(&cargo_toml_path) {
                Ok(contents) => {
                    // Use the toml crate to parse and check for a top-level `workspace` table
                    if let Ok(table) = contents.parse::<Table>() {
                        if table.contains_key("workspace") {
                            return Ok(dir.to_path_buf());
                        }
                    }
                }
                Err(_) => {
                    // If we can't read it, skip and continue searching upwards
                }
            }
        }

        current = dir.parent();
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Workspace root not found",
    ))
}


pub fn write_source(name: &str, tokens: TokenStream) -> io::Result<()> {
    let workspace_root = find_workspace_root(&env::current_dir()?)?;
    let out_dir_path = Path::new(&workspace_root)
        .join("lib")
        .join("obj_params")
        .join("src")
        .join("generated")
        .join(name);

    fs::create_dir_all(out_dir_path.parent().unwrap())?;

    let mut source = if tokens.is_empty() { "".to_owned() } else {
        let item: syn::File = match syn::parse2(tokens) {
            Ok(v) => v,
            Err(e) => {
                println!("Code generation error for {}!", out_dir_path.to_str().unwrap());
                println!("Error: {e}");
                panic!();
            }
        };

        prettyplease::unparse(&item)
    };

    source.insert_str(0, "// Copyright (C) 2025 AnotherlandServer\n\
        // \n\
        // This program is free software: you can redistribute it and/or modify\n\
        // it under the terms of the GNU Affero General Public License as\n\
        // published by the Free Software Foundation, either version 3 of the\n\
        // License, or (at your option) any later version.\n\
        // \n\
        // This program is distributed in the hope that it will be useful,\n\
        // but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
        // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
        // GNU Affero General Public License for more details.\n\
        // \n\
        // You should have received a copy of the GNU Affero General Public License\n\
        // along with this program.  If not, see <http://www.gnu.org/licenses/>.\n\
        \n\
        // #################################################\n\
        // # This file is generated. Do not edit manually. #\n\
        // #################################################\n\
        \n");

    fs::write(out_dir_path, source)
}

fn main() {
    let otherland_client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
    let otherland_client_path = Path::new(&otherland_client_env);

    // Generate code for cparam classes
    generate_param_code(otherland_client_path)
        .expect("Failed to generate param code");

    println!("All done!");
}