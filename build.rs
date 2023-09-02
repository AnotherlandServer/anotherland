use std::{env, path::Path, fs};

use crate::{packet_code_generator::generate_packet_code, param_code_generator::generate_param_code};

mod packet_code_generator;
mod param_code_generator;

fn main() {
    let otherland_client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
    let otherland_client_path = Path::new(&otherland_client_env);

    generate_param_code(otherland_client_path)
        .expect("Failed to generate param code");

    generate_packet_code()
        .expect("Failed to generate packet handling code");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packet_definitions");
    println!("cargo:rerun-if-changed=packet_definitions/packets");
    println!("cargo:rerun-if-changed=packet_definitions/structs");
    println!("cargo:rerun-if-changed=packet_code_generator");
    println!("cargo:rerun-if-changed=param_code_generator");
}