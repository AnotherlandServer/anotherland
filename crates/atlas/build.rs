use std::{env, path::{Path, PathBuf}, fs, io};

use proc_macro2::TokenStream;

use crate::{packet_code_generator::generate_packet_code, param_code_generator::generate_param_code};

mod packet_code_generator;
mod param_code_generator;

pub fn write_source(name: &str, tokens: TokenStream) -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir).join(name);

    let source = if tokens.is_empty() { "".to_owned() } else {
        let item: syn::File = match syn::parse2(tokens) {
            Ok(v) => v,
            Err(e) => {
                println!("Code generation error for {}!", out_dir_path.to_str().unwrap());
                println!("Error: {}", e.to_string());
                println!("Line: {:#?}", e.span());
                panic!();
            }
        };

        prettyplease::unparse(&item)
    };

    fs::write(out_dir_path, source)
}

fn main() {
    let otherland_client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
    let otherland_client_path = Path::new(&otherland_client_env);

    // Generate code for cparam classes
    generate_param_code(otherland_client_path)
        .expect("Failed to generate param code");

    // Generate code for atlas packages
    generate_packet_code()
        .expect("Failed to generate packet handling code");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../packet_definitions");
    println!("cargo:rerun-if-changed=../../packet_definitions/packets");
    println!("cargo:rerun-if-changed=../../packet_definitions/structs");
    println!("cargo:rerun-if-changed=packet_code_generator");
    println!("cargo:rerun-if-changed=param_code_generator");
}