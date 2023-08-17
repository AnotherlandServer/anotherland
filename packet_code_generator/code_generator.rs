use std::{rc::Rc, cell::RefCell, primitive};

use quote::{format_ident, TokenStreamExt};
use ::quote::quote;
use proc_macro2::{TokenStream, Ident};

use super::{struct_generator::{GeneratedEnum, GeneratedStruct, GeneratedFieldType, GeneratedEnumReference, GeneratedStructReference, GeneratedStructSource}, yaml_reader::{FieldDefinition, FieldTypeDefinition, FieldLengthDefinition, PacketDefinitionReference, StructDefinitionReference}};

pub fn generate_enum_code(enums: &Vec<Rc<RefCell<GeneratedEnum>>>) -> TokenStream {
    let mut code = quote!();

    for generated_enum in enums {
        let generated_enum = generated_enum.borrow();

        let enum_identifier = format_ident!("{}", generated_enum.name);
        let values: Vec<Ident> = generated_enum.values.iter()
            .map(|v| format_ident!("{}", v.1)).collect();

        code.extend(quote! {
            #[derive(Debug)]
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

        match &generated_struct.definition {
            GeneratedStructSource::PacketDefintion(pkg) => if pkg.borrow().id == 0 { continue; }
            _ => (),
        }

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
            #[derive(Debug)]
            pub struct #struct_ident {
                #(#field),*
            }
        })
    }

    code
}

pub fn generate_nom_parser_for_primitive(primitive: &str) -> TokenStream {
    match primitive {
        "bool" => quote! {map(le_u8, |v| v != 0)},
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
            let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
                    
            match r#type {
                FieldTypeDefinition::Primitive(primitive) => {
                    let primitive_parser = generate_nom_parser_for_primitive(primitive);
        
                    quote! { #primitive_parser }
                },
                FieldTypeDefinition::Enum { primitive, values } => {
                    let primitive_parser = match primitive.as_ref() {
                        FieldTypeDefinition::Primitive(primitive) => generate_nom_parser_for_primitive(primitive),
                        _ => panic!("Enum primitive type is not a primitive"),
                    };

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
                        map(#primitive_parser, |i| {
                            match i {
                                #(#enum_fields)*
                                _ => panic!("Invalid enum value"),
                            }
                        })
                    }
                },
                FieldTypeDefinition::Array { len, r#type } => {
                    let parser = match r#type.as_ref() {
                        FieldTypeDefinition::Primitive(primitive) => generate_nom_parser_for_primitive(primitive),
                        FieldTypeDefinition::CString { .. } => quote! { parse_pkt_cstring },
                        FieldTypeDefinition::WString { .. } => quote! { parse_pkt_wstring },
                        FieldTypeDefinition::Struct(_) => {
                            match &generated_field.r#type {
                                GeneratedFieldType::Struct(GeneratedStructReference::Resolved(generated_struct)) => {
                                    let struct_ident = format_ident!("{}", generated_struct.borrow().name);
                                    quote! { #struct_ident::from_bytes }
                                },
                                _ => panic!("Generated field type does not match field definition"),
                            }
                        },
                        _ => panic!("Array types must be simple types"),
                    };

                    let len_ident = match len {
                        FieldLengthDefinition::ConstLen(len) => {
                            quote!(#len)
                        },
                        FieldLengthDefinition::DynamicLen(field) => {
                            let array_len_field = generated_struct.fields_mapped.get(field).unwrap().borrow();
                            let array_len_field_ident = format_ident!("{}", array_len_field.name);

                            if array_len_field.optional {
                                quote!(#array_len_field_ident.unwrap_or(0) as usize)
                            } else {
                                quote!(#array_len_field_ident as usize)
                            }
                        }
                    };
                    
                    quote! {
                        count(#parser, #len_ident)
                    }
                },
                FieldTypeDefinition::CString { maxlen } => {
                    quote! { parse_pkt_cstring }
                },
                FieldTypeDefinition::WString { maxlen } => {
                    quote! { parse_pkt_wstring }
                },
                FieldTypeDefinition::Struct(_) => {
                    match &generated_field.r#type {
                        GeneratedFieldType::Struct(GeneratedStructReference::Resolved(generated_struct)) => {
                            let struct_ident = format_ident!("{}", generated_struct.borrow().name);
                            quote! { #struct_ident::from_bytes }
                        },
                        _ => panic!("Generated field type does not match field definition"),
                    }
                }
            }
        },
        _ => panic!("generate_nom_parser_for_field can only be called for typed fields"),
    }
}

pub fn generate_field_parser_code(generated_struct: &GeneratedStruct, field: &FieldDefinition, condition: Option<TokenStream>) -> TokenStream {
    match field {
        FieldDefinition::Branch { field, is_true, is_false } => {
            let generated_cond_field = generated_struct.fields_mapped.get(field).unwrap().borrow();
            let cond_field_ident = format_ident!("{}", generated_cond_field.name);

            let mut parser_code = quote!();

            for field in is_true {
                parser_code.extend(generate_field_parser_code(generated_struct, field, Some(quote!{#cond_field_ident})))
            }

            for field in is_false {
                parser_code.extend(generate_field_parser_code(generated_struct, field, Some(quote!{!#cond_field_ident})))
            }

            parser_code
        },
        FieldDefinition::Field { name, r#type } => {
            let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
            let field_ident = format_ident!("{}", generated_field.name);
            let parser = generate_nom_parser_for_field(generated_struct, field);

            if let Some(condition) = condition {
                quote! {
                    let (i, #field_ident) = cond(#condition, #parser)(i)?;
                }
            } else {
                quote! {
                    let (i, #field_ident) = #parser(i)?;
                }
            }
        }
    }
}

pub fn generate_parser_code(generated_struct: &GeneratedStruct) -> TokenStream {
    let mut field_parser = Vec::new();

    match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => {
            let def = def.borrow();

            if let Some(inherit) = &def.inherit {
                match inherit {
                    PacketDefinitionReference::Resolved(parent_def) => {
                        let parent_def = parent_def.borrow();

                        for field in &parent_def.fields {
                            field_parser.push(generate_field_parser_code(generated_struct, field, None));
                        }
                    },
                    _ => unreachable!()
                }
            }

            for field in &def.fields {
                field_parser.push(generate_field_parser_code(generated_struct, field, None));
            }
        },
        GeneratedStructSource::StructDefinition(def) => {
            let def = def.borrow();

            if let Some(inherit) = &def.inherit {
                match inherit {
                    StructDefinitionReference::Resolved(parent_def) => {
                        let parent_def = parent_def.borrow();

                        for field in &parent_def.fields {
                            field_parser.push(generate_field_parser_code(generated_struct, field, None));
                        }
                    },
                    _ => unreachable!()
                }
            }

            for field in &def.fields {
                field_parser.push(generate_field_parser_code(generated_struct, field, None));
            }
        }
    }

    let struct_name = &generated_struct.name;
    let struct_ident = format_ident!("{}", struct_name);

    let struct_fields: Vec<_> = generated_struct.fields.iter().map(|v| format_ident!("{}", v.borrow().name)).collect();

    match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(_) => {
            quote! {
                pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], CPkt, VerboseError<&'a [u8]>> { 
                    context(#struct_name, |i| {
                        #(#field_parser)*
        
                        Ok((i, CPkt::#struct_ident(Box::new(#struct_ident {
                            #(#struct_fields),*
                        }))))
                    })(i)
                }
            }
        },
        GeneratedStructSource::StructDefinition(_) => {
            quote! {
                pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], #struct_ident, VerboseError<&'a [u8]>> { 
                    context(#struct_name, |i| {
                        #(#field_parser)*
        
                        Ok((i, #struct_ident {
                            #(#struct_fields),*
                        }))
                    })(i)
                }
            }
        }
    }

}

pub fn generate_implementation_code(structs: &Vec<Rc<RefCell<GeneratedStruct>>>) -> TokenStream {
    let mut code = quote!();

    for generated_struct in structs {
        let generated_struct = generated_struct.borrow();

        match &generated_struct.definition {
            GeneratedStructSource::PacketDefintion(pkg) => if pkg.borrow().id == 0 { continue; }
            _ => (),
        }

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