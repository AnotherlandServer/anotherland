use std::{env, path::Path, fs};

use crate::packet_code_generator::generate_packet_code;

mod packet_code_generator;

fn main() {
    generate_packet_code()
        .expect("Failed to generate packet handling code");

    println!("cargo:rerun-if-changed=packet_definitions/");
    println!("cargo:rerun-if-changed=packet_definitions/packets/");
    println!("cargo:rerun-if-changed=packet_definitions/structs/");
}