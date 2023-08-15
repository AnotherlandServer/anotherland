use std::{env, path::Path, fs};

use crate::packet_parser::load_definitions;

mod packet_parser;



fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    //let out_dir_path = Path::new(&out_dir);

    let definitions = load_definitions("./packet_definitions/packets")
        .expect("Failed to parse packet defintions");

    panic!();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packet_definitions/");
    println!("cargo:rerun-if-changed=packet_definitions/packets/");
    println!("cargo:rerun-if-changed=packet_definitions/structs/");
}