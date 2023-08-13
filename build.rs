use std::borrow::{BorrowMut, Cow};
use std::collections::HashSet;
use std::io::Write;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{collections::HashMap, env};
use std::{fs, io};
use convert_case::{Case, Casing, Converter, Boundary, Pattern};

use proc_macro2::Ident;
use quote::__private::TokenStream;
use quote::format_ident;
use serde::de::IntoDeserializer;
use serde::{Serialize, Deserialize};

use ::quote::quote;
use serde_yaml::{Value, Mapping};
use syn::Item;

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

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    let mut packet_definition_generator = PacketDefinitionGenerator {
        inheritance_cache: HashMap::<String, PacketDefinition>::new(),
        in_dir: Path::new("./packet_definitions/definitions").to_path_buf(),
        out_dir: out_dir_path.to_path_buf(),
    };
    
    packet_definition_generator.generate_packet_definitions().expect("Failed to generate packet definitions");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packet_definitions/");
    println!("cargo:rerun-if-changed=packet_definitions/definitions/");
}

struct PacketDefinitionGenerator {
    inheritance_cache: HashMap::<String, PacketDefinition>,
    in_dir: PathBuf,
    out_dir: PathBuf,
}

impl PacketDefinitionGenerator {
    fn generate_packet_definitions(&mut self) -> io::Result<()> {
        let mut packet_ids: Vec<TokenStream> = vec![];
        let mut packet_structs: Vec<TokenStream> = vec![];
        let mut packet_enum_entries: Vec<TokenStream> = vec![];
        let mut packet_parser: Vec<TokenStream> = vec![];
        let mut packet_implementations: Vec<TokenStream> = vec![];
    
        // iterate over all packet definitions
        for entry in fs::read_dir(&self.in_dir)? {
            let entry = entry?;
    
            if !entry.file_type()?.is_file() || !(Path::new(&entry.file_name()).extension().unwrap() == "yaml") { continue; }
    
            println!("Parsing definition {}...", entry.file_name().to_string_lossy());
    
            let definition = self.parse_definition(&entry.path())?;
            if let Some(packetId) = definition.id {
                let packet_name = entry.path().file_stem().map(|v| v.to_str().unwrap()).unwrap().to_owned();
                let packet_identifier = definition.get_identifier();
                let packet_id_identifier = definition.get_id_identifier();

                // generate code
                packet_structs.push(definition.generate_struct());

                let packet_nom_parser = definition.generate_nom_parser();

                packet_implementations.push(quote! {
                    impl #packet_identifier {
                        #packet_nom_parser
                    }
                });

                packet_enum_entries.push(quote! {
                    #packet_identifier(Box<#packet_identifier>)
                });

                packet_ids.push(quote! {
                    pub const #packet_id_identifier: u8 = #packetId;
                });

                packet_parser.push(quote! {
                    #packet_id_identifier => #packet_identifier::from_bytes,
                });
            }
        }

        let packet_lib_code = quote! {
            use nom::{IResult, error::{VerboseError, context}, combinator::*, sequence::*, multi::*, number::complete::*};
            use parsers::*;

            #(#packet_ids)*

            #(#packet_structs)*

            #[derive(Debug)]
            pub enum CPkt {
                #(#packet_enum_entries),*
            }

            impl CPkt {
                pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], CPkt, VerboseError<&'a [u8]>> {
                    context("CPkt", flat_map(
                        le_u8,
                        |packet_id| {
                            match packet_id {
                                #(#packet_parser)*
                                _ => panic!(),
                            }
                        } 
                    ))(i)
                }

                pub fn to_bytes(&self) -> &[u8] {
                    todo!()
                }
            }

            #(#packet_implementations)*
        };

        write_source(&self.out_dir.join("generated_packets.rs"), packet_lib_code).expect("Failed to write generated_packets.rs to output directory.");
        Ok(())
    }
    
    fn parse_definition(&mut self, source_file: &PathBuf) -> io::Result<PacketDefinition> {
        let src = String::from_utf8(fs::read(source_file)?).unwrap();
        let yaml: Value = serde_yaml::from_str(&src).unwrap();

        let id = if let Some(id_val) = yaml.get("id") {
            if let Some(id_val) = id_val.as_i64() {
                if id_val == -1 {
                    None
                } else {
                    Some(id_val as u8)
                }
            } else {
                None
            }
        } else { None };

        let mut definition = PacketDefinition::new(id, source_file.file_stem().unwrap().to_str().unwrap().to_owned());
        definition.inherit = yaml.get("inherit").map(|v| v.as_str().unwrap()).map(|v| v.to_owned());

        /*let mut definition = PacketDefinition { 
            id, 
            name: source_file.file_stem().unwrap().to_str().unwrap().to_owned(),
            inherit: yaml.get("inherit").map(|v| v.as_str().unwrap()).map(|v| v.to_owned()),
            fields: vec![],
        };*/

        // Inherit fields
        if let Some(inherit) = &definition.inherit {
            let inherited = self.resolve_inherited_definition(&inherit)?;
            definition.fields = inherited.fields;
        }

        Self::parse_fields(yaml.get("fields").unwrap().as_sequence().unwrap(), &mut definition.fields, 0)?;
    
        definition.generate_struct_fields();

        Ok(definition)
    }

    fn transform_field_name(str: &str) -> String {
        Converter::new()
        .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
        .set_pattern(Pattern::Lowercase)
        .set_delim("_")
        .convert(str)
    }

    fn parse_fields(input: &Vec<Value>, fields: &mut Vec<PacketField>, count_offset: usize) -> io::Result<()> {
        let case_converter = Converter::new()
            .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
            .set_pattern(Pattern::Lowercase)
            .set_delim("_");
        
        for field_def in input {
            if let Some(branch_def) = field_def.get("branch") {
                println!("{:#?}", branch_def);

                let field_name = Self::transform_field_name(
                        branch_def
                        .get("field").unwrap()
                        .as_str().unwrap());
                let mut is_true = Vec::new();
                let mut is_false = Vec::new();

                Self::parse_fields(branch_def.get("isTrue").unwrap().get("fields").unwrap().as_sequence().unwrap(), &mut is_true, fields.len())?;
                Self::parse_fields(branch_def.get("isFalse").unwrap().get("fields").unwrap().as_sequence().unwrap(), &mut is_false, fields.len() + is_true.len())?;

                fields.push(PacketField::Branch { field: field_name, is_true, is_false });
            } else if let Some(loop_def) = field_def.get("loop") {

            } else {
                fields.push(PacketField::Field { 
                    name: Self::transform_field_name(
                        &field_def
                        .get("name")
                        .map(|v| v.as_str().unwrap().to_owned())
                        .unwrap_or(
                            format!("field_{}", fields.len() + count_offset)
                        )), 
                    r#type: Self::parse_type(field_def.get("type").unwrap())?
                });
            }
        }

        Ok(())
    }
    
    fn parse_type(input: &Value) -> io::Result<PacketFieldType> {
        let typename = if input.is_string() {
            input.as_str().unwrap()
        } else {
            input.get("name").unwrap().as_str().unwrap()
        };

        match typename {
            "bool" => Ok(PacketFieldType::bool),
            "u8" => Ok(PacketFieldType::u8),
            "u16" => Ok(PacketFieldType::u16),
            "u32" => Ok(PacketFieldType::u32),
            "u64" => Ok(PacketFieldType::u64),
            "i8" => Ok(PacketFieldType::i8),
            "i16" => Ok(PacketFieldType::i16),
            "i32" => Ok(PacketFieldType::i32),
            "i64" => Ok(PacketFieldType::i64),
            "cstring" => Ok(PacketFieldType::cstring { maxlen: input.get("maxlen").map(|v| v.as_u64().unwrap() as usize) }),
            "wstring" => Ok(PacketFieldType::wstring { maxlen: input.get("maxlen").map(|v| v.as_u64().unwrap() as usize) }),
            "array" => {
                let len_def = input.get("len").unwrap();
                let r#type = Self::parse_type(input.get("type").unwrap())?;

                if len_def.is_u64() {
                    Ok(PacketFieldType::array { 
                        len: ArrayLenghtDefinition::ConstLen(len_def.as_u64().unwrap() as usize), 
                        r#type: Box::new(r#type)
                    })
                } else if len_def.is_string() {
                    Ok(PacketFieldType::array { 
                        len: ArrayLenghtDefinition::DynamicLen(Self::transform_field_name(&len_def.as_str().unwrap().to_owned())), 
                        r#type: Box::new(r#type)
                    })
                } else {
                    panic!()
                }
            },
            _ => panic!(),
        }
    }

    fn resolve_inherited_definition(&mut self, name: &str) -> io::Result<PacketDefinition> {
        if let Some(definition) = self.inheritance_cache.get(name) {
            Ok(definition.clone())
        } else {
            let definition = self.parse_definition(&self.in_dir.join(format!("{}.yaml", name)))?;
            self.inheritance_cache.insert(name.to_owned(), definition.clone());
            Ok(definition)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PacketStructField {
    name: String,
    r#type: PacketFieldType,
    optional: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct PacketDefinition {
    pub id: Option<u8>,
    pub name: String,
    pub inherit: Option<String>,
    pub fields: Vec<PacketField>,
    pub ordered_field_names: Vec<String>,
    pub struct_fields: HashMap<String, PacketStructField>,
}

impl PacketDefinition {
    pub fn generate_struct_fields(&mut self) {
        Self::generate_struct_fields_internal(&self.fields, &mut self.ordered_field_names, &mut self.struct_fields);
    }

    fn generate_struct_fields_internal(fields: &Vec<PacketField>, order: &mut Vec<String>, map: &mut HashMap<String, PacketStructField>) {
        for field in fields {
            match field {
                PacketField::Branch { field, is_true, is_false } => {
                    let mut is_true_map = HashMap::new();
                    let mut is_true_order = Vec::new();

                    let mut is_false_map = HashMap::new();
                    let mut is_false_order = Vec::new();
                    
                    
                    Self::generate_struct_fields_internal(is_true, &mut is_true_order, &mut is_true_map);
                    Self::generate_struct_fields_internal(is_false, &mut is_false_order, &mut is_false_map);

                    // Transfer is_true 
                    for field in &is_true_order {
                        if !map.contains_key(field) {
                            let field_def = is_true_map.get_mut(field).unwrap();
                            field_def.optional = !is_false_map.contains_key(field);

                            order.push(field.to_owned());
                            map.insert(field.to_owned(), field_def.clone());
                        }
                    }

                    // Transfer is_false
                    for field in &is_false_order {
                        if !map.contains_key(field) {
                            let field_def = is_false_map.get_mut(field).unwrap();
                            field_def.optional = !is_true_map.contains_key(field);

                            order.push(field.to_owned());
                            map.insert(field.to_owned(), field_def.clone());
                        }
                    }
                },
                PacketField::Loop { field, body } => {
                    panic!()
                },
                PacketField::Field { name, r#type } => {
                    if !map.contains_key(name) {
                        order.push(name.to_owned());
                        map.insert(name.to_owned(), PacketStructField { 
                            name: name.to_string(), 
                            r#type: r#type.clone(), 
                            optional: false,
                        });
                    }
                }
            }
        }
    }

    pub fn new(id: Option<u8>, name: String) -> Self {
        Self {
            id,
            name,
            inherit: None,
            fields: Vec::new(),
            ordered_field_names: Vec::new(),
            struct_fields: HashMap::new(),
        }
    }

    pub fn get_identifier(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    pub fn get_id_identifier(&self) -> Ident {
        format_ident!("ID_{}", self.name.to_uppercase())
    }

    pub fn generate_struct(&self) -> TokenStream {
        let mut fields = Vec::new();
        let identifier = self.get_identifier();

        for field_name in &self.ordered_field_names {
            if let Some(field_def) = self.struct_fields.get(field_name) {
                let name = format_ident!("{}", field_def.name);
                let datatype = field_def.r#type.to_rust_type();

                if field_def.optional {
                    fields.push(quote! { pub #name: Option<#datatype> });
                } else {
                    fields.push(quote! { pub #name: #datatype });
                }
            }
        }

        quote! {
            #[derive(Debug)]
            pub struct #identifier {
                #(#fields),*
            }
        }
    }

    pub fn generate_nom_parser(&self) -> TokenStream {
        let packet_name = self.name.to_owned();
        let mut field_parsers = Vec::new();

        for field in &self.fields {
            field_parsers.push(field.to_nom_parser());
        }

        let packet_identifier = self.get_identifier();
        let mut field_identifiers = Vec::new();

        for field in &self.ordered_field_names {
            field_identifiers.push(format_ident!("{}", field));
        }

        quote! {
            pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], CPkt, VerboseError<&'a [u8]>> {
                context(#packet_name, |i| {
                    #(#field_parsers)*

                    Ok((i, CPkt::#packet_identifier(Box::new(#packet_identifier {
                        #(#field_identifiers),*
                    }))))
                })(i)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum PacketField {
    Field { name: String, r#type: PacketFieldType},
    Branch { field: String, is_true: Vec<PacketField>, is_false: Vec<PacketField> },
    Loop { field: String, body: Vec<PacketField> }
}

impl PacketField {
    pub fn to_nom_parser(&self) -> TokenStream {
        match &self {
            Self::Field { name, r#type } => {
                let type_parser = r#type.to_nom_parser();
                let field_identifier = format_ident!("{}", name);

                quote! {
                    let (i, #field_identifier) = context(#name, #type_parser)(i)?;
                }
            },
            Self::Branch { field, is_true, is_false } => {
                let mut parser_segment = quote!();
                
                for field_def in is_true {
                    match field_def {
                        Self::Field { name, r#type } => {
                            let type_parser = r#type.to_nom_parser();
                            let field_identifier = format_ident!("{}", name);
                            let condition_identifier = format_ident!("{}", field);
            
                            parser_segment.extend(quote! {
                                let (i, #field_identifier) = context(#name, cond(#condition_identifier as bool,#type_parser))(i)?;
                            });
                        },
                        _ => todo!(),
                    }
                }

                for field_def in is_false {
                    match field_def {
                        Self::Field { name, r#type } => {
                            let type_parser = r#type.to_nom_parser();
                            let field_identifier = format_ident!("{}", name);
                            let condition_identifier = format_ident!("{}", field);
            
                            parser_segment.extend(quote! {
                                let (i, #field_identifier) = context(#name, cond(!(#condition_identifier  as bool),#type_parser))(i)?;
                            });
                        },
                        _ => todo!(),
                    }
                }

                parser_segment
            }
            _ => quote! {
                todo!();
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum PacketFieldType {
    bool,
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    cstring { maxlen: Option<usize> },
    wstring { maxlen: Option<usize> },
    array { len: ArrayLenghtDefinition, r#type: Box<PacketFieldType> },
}

#[derive(Debug, PartialEq, Clone)]
enum ArrayLenghtDefinition {
    ConstLen(usize),
    DynamicLen(String),
}

impl PacketFieldType {
    pub fn to_rust_type(&self) -> TokenStream {
        match self {
            PacketFieldType::bool => quote! { bool },
            PacketFieldType::u8 => quote! { u8 },
            PacketFieldType::u16 => quote! { u16 },
            PacketFieldType::u32 => quote! { u32 },
            PacketFieldType::u64 => quote! { u64 },
            PacketFieldType::i8 => quote! { i8 },
            PacketFieldType::i16 => quote! { i16 },
            PacketFieldType::i32 => quote! { i32 },
            PacketFieldType::i64 => quote! { i64 },
            PacketFieldType::cstring { .. } => quote! { String },
            PacketFieldType::wstring { .. } => quote! { String },
            PacketFieldType::array { len, r#type } => {
                let subtype = r#type.to_rust_type();
                match len {
                    ArrayLenghtDefinition::ConstLen(_) => quote! { Vec<#subtype> },
                    ArrayLenghtDefinition::DynamicLen(_) => quote! { Vec<#subtype> },
                }
            },
        }
    }

    pub fn to_nom_parser(&self) -> TokenStream {
        match self {
            PacketFieldType::bool => quote!( map(le_u8, |v| v != 0) ),
            PacketFieldType::u8 => quote!( le_u8 ),
            PacketFieldType::u16 => quote!( le_u16 ),
            PacketFieldType::u32 => quote!( le_u32 ),
            PacketFieldType::u64 => quote!( le_u64 ),
            PacketFieldType::i8 => quote!( le_i8 ),
            PacketFieldType::i16 => quote!( le_i16 ),
            PacketFieldType::i32 => quote!( le_i32 ),
            PacketFieldType::i64 => quote!( le_i64 ),
            PacketFieldType::cstring { .. } => quote!( parse_pkt_cstring ),
            PacketFieldType::wstring { .. } => quote!( parse_pkt_wstring ),
            PacketFieldType::array { len, r#type } => {
                let subtype_parser = r#type.to_nom_parser();

                match len {
                    ArrayLenghtDefinition::ConstLen(len) => quote! { count(#subtype_parser, #len) },
                    ArrayLenghtDefinition::DynamicLen(field) => {
                        let len_identifier = format_ident!("{}", field);
                        quote! { count(#subtype_parser, #len_identifier.unwrap() as usize) }
                    }
                }
            }
        }
    }
}