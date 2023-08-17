use std::{rc::Rc, cell::RefCell};

use quote::{format_ident, TokenStreamExt};
use ::quote::quote;
use proc_macro2::{TokenStream, Ident};

use super::{struct_generator::{GeneratedEnum, GeneratedStruct, GeneratedFieldType, GeneratedEnumReference, GeneratedStructReference, GeneratedStructSource}, yaml_reader::{FieldDefinition, FieldTypeDefinition}};

pub fn generate_enum_code(enums: &Vec<Rc<RefCell<GeneratedEnum>>>) -> TokenStream {
    let mut code = quote!();

    for generated_enum in enums {
        let generated_enum = generated_enum.borrow();

        let enum_identifier = format_ident!("{}", generated_enum.name);
        let values: Vec<Ident> = generated_enum.values.iter()
            .map(|v| format_ident!("{}", v.1)).collect();

        code.extend(quote! {
            pub enum #enum_identifier {
                #(#values),*
            }
        });
    }

    code
}

fn generate_field_type_code(r#type: &GeneratedFieldType) -> TokenStream {
    match r#type {
        super::struct_generator::GeneratedFieldType::Array(r#type) => {
            let subtype = generate_field_type_code(r#type);
            quote! {Vec<#subtype>}
        },
        GeneratedFieldType::Enum(enum_ref) => {
            match enum_ref {
                GeneratedEnumReference::Resolved(generated_enum) => {
                    let enum_ident = format_ident!("{}", generated_enum.borrow().name);
                    quote! {#enum_ident}
                },
                _ => panic!("Unresolved enum!"),
            }
        },
        GeneratedFieldType::Struct(struct_ref) => {
            match struct_ref {
                GeneratedStructReference::Resolved(generated_struct) => {
                    let struct_ident = format_ident!("{}", generated_struct.borrow().name);
                    quote! {#struct_ident}
                },
                _ => panic!("Unresolved struct!"),
            }            
        },
        GeneratedFieldType::String => quote! {String},
        GeneratedFieldType::Bool => quote! {bool},
        GeneratedFieldType::U8 => quote! {u8},
        GeneratedFieldType::U16 => quote! {u16},
        GeneratedFieldType::U32 => quote! {u32},
        GeneratedFieldType::U64 => quote! {u64},
        GeneratedFieldType::I8 => quote! {i8},
        GeneratedFieldType::I16 => quote! {i16},
        GeneratedFieldType::I32 => quote! {i32},
        GeneratedFieldType::I64 => quote! {i64},
    }
}

pub fn generate_struct_code(structs: &Vec<Rc<RefCell<GeneratedStruct>>>) -> TokenStream{
    let mut code = quote!();

    for generated_struct in structs {
        let generated_struct = generated_struct.borrow();

        let struct_ident = format_ident!("{}", generated_struct.name);
        let field: Vec<_> = generated_struct.fields.iter().map(|v| {
            let field = v.borrow();
            let field_ident = format_ident!("{}", field.name);

            let type_ident = generate_field_type_code(&field.r#type);

            if field.optional {
                quote! {#field_ident: Option<#type_ident>}
            } else {
                quote! {#field_ident: #type_ident}
            }
        }).collect();

        code.extend(quote! {
            pub struct #struct_ident {
                #(#field),*
            }
        })
    }

    code
}

pub fn generate_nom_parser_for_primitive(primitive: &str) -> TokenStream {
    match primitive {
        "bool" => quote! {le_u8},
        "u8" => quote! {le_u8},
        "u16" => quote! {le_u16},
        "u32" => quote! {le_u32},
        "u64" => quote! {le_u64},
        "i8" => quote! {le_i8},
        "i16" => quote! {le_i16},
        "i32" => quote! {le_i32},
        "i64" => quote! {le_i64},
        _ => panic!("Unknown primitive")
    }
}

pub fn generate_nom_parser_for_field(generated_struct: &GeneratedStruct, field: &FieldDefinition) -> TokenStream {
    match field {
        FieldDefinition::Field { name, r#type } => {
            match r#type {
                FieldTypeDefinition::Primitive(primitive) => {
                    let primitive_parser = generate_nom_parser_for_primitive(primitive);
        
                    quote! { (|i| #primitive_parser(i)) }
                },
                FieldTypeDefinition::Enum { primitive, values } => {
                    let primitive_parser = match primitive.as_ref() {
                        FieldTypeDefinition::Primitive(primitive) => generate_nom_parser_for_primitive(primitive),
                        _ => panic!("Enum primitive type is not a primitive"),
                    };

                    let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
                    
                    let enum_fields: Vec<_> = match &generated_field.r#type {
                        GeneratedFieldType::Enum(GeneratedEnumReference::Resolved(enum_definition)) => {
                            let enum_definition = enum_definition.borrow();
                            let enum_ident = format_ident!("{}", enum_definition.name);
                            
                            match primitive.as_ref() {
                                FieldTypeDefinition::Primitive(primitive) => {
                                    match primitive.as_str() {
                                        "u8" => {
                                            enum_definition.values.iter().map(|(val, name)| {
                                                let val = *val as u8;
                                                let enum_val_name = format_ident!("{}", name);
                                                quote! { #val => #enum_ident::#enum_val_name, }
                                            }).collect()
                                        },
                                        "u16" => {
                                            enum_definition.values.iter().map(|(val, name)| {
                                                let val = *val as u16;
                                                let enum_val_name = format_ident!("{}", name);
                                                quote! { #val => #enum_ident::#enum_val_name, }
                                            }).collect()
                                        },
                                        "u32" => {
                                            enum_definition.values.iter().map(|(val, name)| {
                                                let val = *val as u32;
                                                let enum_val_name = format_ident!("{}", name);
                                                quote! { #val => #enum_ident::#enum_val_name, }
                                            }).collect()
                                        }  
                                        _ => panic!("Invalid enum primitive"),
                                    }
                                },
                                _ => unreachable!(),
                            }


                        },
                        _ => panic!("Generated field type does not match field definition"),
                    };

                    quote! {
                        (|i| map_parser(#primitive_parser, |i| {
                            match i {
                                #(#enum_fields)*
                                _ => fail(i),
                            }
                        })(i))
                    }
                },
                FieldTypeDefinition::Array { len, r#type } => {
                    quote!()
                },
                FieldTypeDefinition::CString { maxlen } => {
                    quote!()
                },
                FieldTypeDefinition::WString { maxlen } => {
                    quote!()
                },
                FieldTypeDefinition::Struct(_) => {
                    quote!()
                }
            }
        },
        _ => panic!("generate_nom_parser_for_field can only be called for typed fields"),
    }
}

pub fn generate_field_parser_code(generated_struct: &GeneratedStruct, field: &FieldDefinition) -> TokenStream {
    match field {
        FieldDefinition::Branch { field, is_true, is_false } => {
            quote!()
        },
        FieldDefinition::Field { name, r#type } => {
            let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
            let field_ident = format_ident!("{}", generated_field.name);
            let parser = generate_nom_parser_for_field(generated_struct, field);

            quote! {
                let (i, #field_ident) = #parser(i);
            }
        }
    }
}

pub fn generate_parser_code(generated_struct: &GeneratedStruct) -> TokenStream {
    let mut field_parser = Vec::new();

    match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => {
            for field in &def.borrow().fields {
                field_parser.push(generate_field_parser_code(generated_struct, field));
            }
        },
        GeneratedStructSource::StructDefinition(def) => {
            for field in &def.borrow().fields {
                field_parser.push(generate_field_parser_code(generated_struct, field));
            }
        }
    }

    let struct_name = &generated_struct.name;

    quote! {
        pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], CPkt, VerboseError<&'a [u8]>> { 
            context(#struct_name, |i| {
                #(#field_parser)*
            })(i)
        }
    }
}

pub fn generate_implementation_code(structs: &Vec<Rc<RefCell<GeneratedStruct>>>) -> TokenStream {
    let mut code = quote!();

    for generated_struct in structs {
        let generated_struct = generated_struct.borrow();

        let struct_ident = format_ident!("{}", generated_struct.name);
        let parser_code = generate_parser_code(&generated_struct);

        code.extend(quote! {
            impl #struct_ident {
                #parser_code
            }
        });
    }

    code
}