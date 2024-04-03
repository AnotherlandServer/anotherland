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

use std::{io, env, collections::HashMap, cell::RefCell, rc::Rc, path::{PathBuf, Path}, fs};

use proc_macro2::TokenStream;
use quote::format_ident;
use ::quote::quote;

use crate::{packet_code_generator::struct_generator::GeneratedStructSource, write_source};

use super::{yaml_reader::{load_definitions, PacketDefinitionReference}, struct_generator::GeneratedStruct, code_generator::{generate_enum_code, generate_struct_code, generate_implementation_code}};

pub fn generate_packet_code() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    let mut packet_definitions = HashMap::new();
    let mut struct_definitions = HashMap::new();

    {
        let (parsed_packet_definitions, parsed_struct_definitions) 
            = load_definitions("../packet_definitions/structs")?;

        parsed_packet_definitions.into_iter().for_each(|(k, v)| { packet_definitions.insert(k, v); });
        parsed_struct_definitions.into_iter().for_each(|(k, v)| { struct_definitions.insert(k, v); });
    }
    
    {
        let (parsed_packet_definitions, parsed_struct_definitions) 
            = load_definitions("../packet_definitions/packets")?;

        parsed_packet_definitions.into_iter().for_each(|(k, v)| { packet_definitions.insert(k, v); });
        parsed_struct_definitions.into_iter().for_each(|(k, v)| { struct_definitions.insert(k, v); });
    }

    // resolve packet inheritance
    for (name, definition) in &packet_definitions {
        let mut definition = definition.borrow_mut();

        if let Some(inherit) = &definition.inherit {
            match inherit {
                PacketDefinitionReference::Unresolved(parent_name) => {
                    if let Some(parent) = packet_definitions.get(parent_name) {
                        println!("Got parent: {}", parent_name);
                        definition.inherit = Some(PacketDefinitionReference::Resolved(parent.clone()));
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::NotFound, 
                            format!("Inherited struct {} not found for packet {}!", parent_name, name)
                        ))
                    }
                },
                _ => Ok(()),
            }?;
        }
    }

    let mut generated_structs = HashMap::new();
    let mut generated_enums = Vec::new();

    // resolve type references
    for def in struct_definitions.values() {
        def.borrow_mut().resolve_references(&struct_definitions)?;
        def.borrow_mut().normalize();
    }

    for def in packet_definitions.values() {
        def.borrow_mut().resolve_references(&packet_definitions, &struct_definitions)?;
        def.borrow_mut().normalize();
    }

    // generate struct layouts & enums
    for struct_definition in struct_definitions.values() {
        let mut generated_struct = 
            GeneratedStruct::generate_from_struct_definition(struct_definition)?;
        
        let mut enums = generated_struct.generate_and_resolve_enums();
        generated_enums.append(&mut enums);

        //println!("Struct\n{:#?}", generated_struct);
        generated_structs.insert(generated_struct.name.clone(), Rc::new(RefCell::new(generated_struct)));
    }

    for generated in generated_structs.values() {
        generated.borrow_mut().resolve_references(&generated_structs)?;
    }

    // generate packets layouts & enums
    for packet_definition in packet_definitions.values() {
        let mut generated_struct = 
            GeneratedStruct::generate_from_packet_definition(packet_definition, &generated_structs)?;
        
        let mut enums = generated_struct.generate_and_resolve_enums();
        generated_enums.append(&mut enums);

        //println!("Struct\n{:#?}", generated_struct);
        generated_structs.insert(generated_struct.name.clone(), Rc::new(RefCell::new(generated_struct)));
    }

    let struct_list = generated_structs.clone().into_values().collect();

    // generate code
    let enum_code = generate_enum_code(&generated_enums);
    let struct_code = generate_struct_code(&struct_list);
    let impl_code = generate_implementation_code(&struct_list);

    let packet_struct_enums: Vec<_> = struct_list.iter()
        .filter(|v| {
            matches!(v.borrow().definition, super::struct_generator::GeneratedStructSource::PacketDefintion(_))
        })
        .map(|v| {
            let struct_ident = format_ident!("{}", v.borrow().name);
            let (id, _) = match &v.borrow().definition {
                GeneratedStructSource::PacketDefintion(def) => {
                    (def.borrow().id, def.borrow().sub_id)
                },
                _ => unreachable!(),
            };

            if id == 0 {
                quote! {}
            } else {
                quote! { #struct_ident(Box<#struct_ident>), }
            }
        }).collect();

    let packet_enum_ids: Vec<_> = struct_list.iter()
        .filter(|v| {
            matches!(v.borrow().definition, super::struct_generator::GeneratedStructSource::PacketDefintion(_))
        })
        .map(|v| {
            let struct_ident = format_ident!("{}", v.borrow().name);
            let (id, subid) = match &v.borrow().definition {
                GeneratedStructSource::PacketDefintion(def) => {
                    (def.borrow().id, def.borrow().sub_id)
                },
                _ => unreachable!(),
            };

            if id == 0 {
                quote! {}
            } else {
                quote! { Self::#struct_ident(..) => (#id, #subid), }
            }
        }).collect();

    let packet_struct_parser: Vec<_> = struct_list.iter()
        .filter(|v| {
            matches!(v.borrow().definition, super::struct_generator::GeneratedStructSource::PacketDefintion(_))
        })
        .map(|v| {
            let v = v.borrow();
            let struct_ident = format_ident!("{}", v.name);
            let (id, sub_id) = match &v.definition {
                GeneratedStructSource::PacketDefintion(def) => {
                    let def = def.borrow();
                    (def.id, def.sub_id)
                },
                _ => unreachable!(),
            };

            if id == 0 {
                quote!()
            } else {
                quote! { (#id, #sub_id) => #struct_ident::from_bytes, }
            }
        }).collect();

    let packet_writer_enum: Vec<_> = struct_list.iter()
    .filter(|v| {
        matches!(v.borrow().definition, super::struct_generator::GeneratedStructSource::PacketDefintion(_))
    })
    .map(|v| {
        let v = v.borrow();
        let struct_ident = format_ident!("{}", v.name);
        let (id, _) = match &v.definition {
            GeneratedStructSource::PacketDefintion(def) => {
                let def = def.borrow();
                (def.id, def.sub_id)
            },
            _ => unreachable!(),
        };

        if id == 0 {
            quote! ()
        } else {
            quote! { CPkt::#struct_ident(pkt) => pkt.to_bytes(), }
        }
    }).collect();

    write_source("generated_packets.rs", quote! {

        #[allow(clippy::all)]

        mod generated_packets {
            use nom::{IResult, error::{VerboseError, context}, combinator::*, sequence::*, number::complete::*, bytes::complete::take};
            use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
            use crate::parsers::*;
            use std::io;
            use log::error;
            use crate::raknet::Message;
            use crate::NativeParam;
            use crate::Uuid;

            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone)]
            pub enum CPkt {
                #(#packet_struct_enums)*
            }

            impl CPkt {
                pub fn from_bytes(i: &[u8]) -> IResult<&[u8], CPkt, VerboseError<&[u8]>> {
                    context("CPkt", flat_map(tuple((le_u8, le_u8)), 
                        |v| {
                            match v {
                                #(#packet_struct_parser)*
                                _ => {
                                    error!("Unknown packet id {:#?}", v);
                                    Self::pkt_fail
                                },
                            }
                        }
                    ))(i)
                }

                fn pkt_fail(i: &[u8]) -> IResult<&[u8], CPkt, VerboseError<&[u8]>> {
                    fail(i)
                }

                pub fn to_bytes(&self) -> Vec<u8> {
                    match self {
                        #(#packet_writer_enum)*
                    }
                }

                pub fn get_id(&self) -> (u8, u8) {
                    match self {
                        #(#packet_enum_ids)*
                    }
                }
            }

            #enum_code
            #struct_code
            #impl_code
        }

        pub use generated_packets::*;
    })?;

    Ok(())
}
