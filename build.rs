use std::{env, path::Path, fs};
use glob::glob;

use crate::{packet_code_generator::generate_packet_code, param_code_generator::generate_param_code};

mod packet_code_generator;
mod param_code_generator;


fn main() {
    let otherland_client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
    let otherland_client_path = Path::new(&otherland_client_env);

    // Generate code for cparam classes
    generate_param_code(otherland_client_path)
        .expect("Failed to generate param code");

    // Generate code for atlas packages
    generate_packet_code()
        .expect("Failed to generate packet handling code");

    // Generate schema file
    {
        let mut schema = String::new();

        if let Ok(schema_files) = glob("schema/*.surql") {
            for file in schema_files {
                if let Ok(file) = file {
                    schema += &String::from_utf8(
                        fs::read(file).expect("Failed to read schema file")
                    ).expect("Invalid file encoding");
                }
            }
        }
        
        fs::write(Path::new(&env::var_os("OUT_DIR").unwrap()).join("schema.surql"), schema.as_bytes()).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packet_definitions");
    println!("cargo:rerun-if-changed=packet_definitions/packets");
    println!("cargo:rerun-if-changed=packet_definitions/structs");
    println!("cargo:rerun-if-changed=packet_code_generator");
    println!("cargo:rerun-if-changed=param_code_generator");
    println!("cargo:rerun-if-changed=schema/");
}