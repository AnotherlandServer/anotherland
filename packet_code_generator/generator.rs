use std::{io, env, collections::HashMap, cell::RefCell, rc::Rc, path::{PathBuf, Path}, fs};

use proc_macro2::TokenStream;
use ::quote::quote;

use super::{yaml_reader::{load_struct_definitions, load_packet_definitions}, struct_generator::GeneratedStruct, code_generator::{generate_enum_code, generate_struct_code, generate_implementation_code}};

pub fn generate_packet_code() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    let struct_definitions = load_struct_definitions("./packet_definitions/structs")?;
    let packet_definitions = load_packet_definitions("./packet_definitions/packets")?;
    
    let mut generated_structs = HashMap::new();
    let mut generated_enums = Vec::new();
    //let mut generated_packet_structs = Vec::new();

    // resolve type references
    for (_, def) in &struct_definitions {
        def.borrow_mut().resolve_references(&packet_definitions, &struct_definitions)?;
        def.borrow_mut().normalize();
    }

    for (_, def) in &packet_definitions {
        def.borrow_mut().resolve_references(&packet_definitions, &struct_definitions)?;
        def.borrow_mut().normalize();
    }

    for (_, def) in &struct_definitions {
        println!("{:#?}", def);
    }

    for (_, def) in &packet_definitions {
        println!("{:#?}", def);
    }

    // generate packet layouts & enums
    for (name, struct_definition) in &struct_definitions {
        let mut generated_struct = 
            GeneratedStruct::generate_from_struct_definition(&struct_definition)?;
        
        let mut enums = generated_struct.generate_and_resolve_enums();
        generated_enums.append(&mut enums);

        println!("Struct\n{:#?}", generated_struct);
        generated_structs.insert(generated_struct.name.clone(), Rc::new(RefCell::new(generated_struct)));
    }

    // generate struct layouts & enums
    for (name, packet_definition) in &packet_definitions {
        let mut generated_struct = 
            GeneratedStruct::generate_from_packet_definition(&packet_definition, &generated_structs)?;
        
        let mut enums = generated_struct.generate_and_resolve_enums();
        generated_enums.append(&mut enums);

        println!("Struct\n{:#?}", generated_struct);
        generated_structs.insert(generated_struct.name.clone(), Rc::new(RefCell::new(generated_struct)));
    }

    let struct_list = generated_structs.clone().into_values().collect();

    // generate code
    let enum_code = generate_enum_code(&generated_enums);
    let struct_code = generate_struct_code(&struct_list);
    let impl_code = generate_implementation_code(&struct_list);

    write_source(&out_dir_path.join("generated_packets.rs"), quote! {
        #enum_code
        #struct_code
        #impl_code
    });

    Ok(())
}

fn write_source(dest: &PathBuf, tokens: TokenStream) -> io::Result<()> {
    let source = if tokens.is_empty() { "".to_owned() } else {
        let item: syn::File = match syn::parse2(tokens) {
            Ok(v) => v,
            Err(e) => {
                println!("Code generation error for {}!", dest.to_str().unwrap());
                println!("Error: {}", e.to_string());
                println!("Line: {:#?}", e.span());
                panic!();
            }
        };

        prettyplease::unparse(&item)
    };

    fs::write(dest, source)
}