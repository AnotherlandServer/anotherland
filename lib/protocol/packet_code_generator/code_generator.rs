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

use std::{rc::Rc, cell::RefCell, str::FromStr};

use quote::format_ident;
use ::quote::quote;
use proc_macro2::{TokenStream, Ident};

use crate::packet_code_generator::yaml_reader::BranchTestDefinition;

use super::{struct_generator::{GeneratedEnum, GeneratedStruct, GeneratedFieldType, GeneratedEnumReference, GeneratedStructReference, GeneratedStructSource, GeneratedField}, yaml_reader::{FieldDefinition, FieldTypeDefinition, FieldLengthDefinition, PacketDefinitionReference, StructDefinitionReference}};

pub fn generate_enum_code(enums: &Vec<Rc<RefCell<GeneratedEnum>>>) -> TokenStream {
    let mut code = quote!();

    for generated_enum in enums {
        let generated_enum = generated_enum.borrow();

        let enum_identifier = format_ident!("{}", generated_enum.name);
        let values: Vec<Ident> = generated_enum.values.iter()
            .map(|v| format_ident!("{}", v.1)).collect();

        let compare: Vec<_> = generated_enum.values.iter()
            .map(|(k, v)| {
                let val_ident = format_ident!("{}", v);
                let k = *k as u32;

                quote! { (rh == &#k && self == &#enum_identifier::#val_ident) }
            }).collect();

        code.extend(quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, Default, PartialEq)]
            pub enum #enum_identifier {
                #[default]
                #(#values),*
            }

            impl PartialEq<u32> for #enum_identifier {
                fn eq(&self, rh: &u32) -> bool {
                    #(#compare)||*
                }
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
        GeneratedFieldType::F32 => quote! {f32},
        GeneratedFieldType::F64 => quote! {f64},
        GeneratedFieldType::Uuid => quote! {Uuid},
        GeneratedFieldType::NativeParam => quote! {NativeParam},
        GeneratedFieldType::AvatarId => quote! {AvatarId},
        GeneratedFieldType::Packet => quote! {Option<CPkt>},
    }
}

pub fn generate_struct_code(structs: &Vec<Rc<RefCell<GeneratedStruct>>>) -> TokenStream{
    let mut code = quote!();

    for generated_struct in structs {
        let generated_struct = generated_struct.borrow();

        if let GeneratedStructSource::PacketDefintion(pkg) = &generated_struct.definition {
            if pkg.borrow().id == 0 { continue; }
        }

        let struct_ident = format_ident!("{}", generated_struct.name);
        let field: Vec<_> = generated_struct.fields.iter().map(|v| {
            let field = v.borrow();
            let field_ident = format_ident!("{}", field.name);

            println!("Generating {}::{}", generated_struct.name, field.name);

            let type_ident = generate_field_type_code(&field.r#type);

            if field.optional {
                quote! {pub #field_ident: Option<#type_ident>}
            } else {
                quote! {pub #field_ident: #type_ident}
            }
        }).collect();

        let derive = if generated_struct.derive_default {
            quote! { #[derive(Debug, Clone, Default)] }
        } else {
            quote! { #[derive(Debug, Clone)] }
        };

        code.extend(quote! {
            #[allow(non_camel_case_types)]
            #derive
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
        "f32" => quote! {le_f32},
        "f64" => quote! {le_f64},
        "uuid" => quote! {map(take(16usize), |v: &[u8]| uuid::Uuid::from_bytes_le(v.try_into().unwrap()).into())},
        "nativeparam" => quote! {NativeParam::parse_struct},
        "avatar_id" => quote! {map(le_u64, |v| v.into())},
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
                FieldTypeDefinition::Enum { primitive, .. } => {
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
                                _ => panic!("Invalid enum value: {}", i),
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
                            println!("{:#?}", field);
                            println!("{:#?}", generated_field);
                            match &generated_field.r#type {
                                GeneratedFieldType::Struct(GeneratedStructReference::Resolved(generated_struct)) => {
                                    let struct_ident = format_ident!("{}", generated_struct.borrow().name);
                                    quote! { #struct_ident::from_bytes }
                                },
                                GeneratedFieldType::Array(r#type) => {
                                    match r#type.as_ref() {
                                        GeneratedFieldType::Struct(GeneratedStructReference::Resolved(generated_struct)) => {
                                            let struct_ident = format_ident!("{}", generated_struct.borrow().name);
                                            quote! { #struct_ident::from_bytes }
                                        },
                                        _ => panic!(),
                                    }
                                }
                                _ => panic!("Generated field type does not match field definition"),
                            }
                        },
                        _ => panic!("Array types must be simple types"),
                    };

                    let len_ident = match len {
                        FieldLengthDefinition::Remainder => {
                            quote!{rest_len(i)?.1}
                        },
                        FieldLengthDefinition::ConstLen(len) => {
                            quote!(#len)
                        },
                        FieldLengthDefinition::DynamicLen(field) => {
                            let array_len_field = generated_struct.fields_mapped.get(field).unwrap().borrow();
                            let array_len_field_ident = format_ident!("{}", array_len_field.name);

                            //if array_len_field.optional {
                            //    quote!(#array_len_field_ident.unwrap_or(0) as usize)
                            //} else {
                                quote!(#array_len_field_ident as usize)
                            //}
                        }
                    };
                    
                    quote! {
                        nom::multi::count(#parser, #len_ident)
                    }
                },
                FieldTypeDefinition::CString { .. } => {
                    quote! { parse_pkt_cstring }
                },
                FieldTypeDefinition::WString { .. } => {
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
                },
                FieldTypeDefinition::Packet => {
                    quote! { map(CPkt::from_bytes, |v| Some(v)) }
                }
            }
        },
        _ => panic!("generate_nom_parser_for_field can only be called for typed fields"),
    }
}

pub fn generate_field_parser_code(generated_struct: &GeneratedStruct, field: &FieldDefinition, _condition: Option<TokenStream>) -> TokenStream {
    match field {
        FieldDefinition::Branch { field: field_name, test, is_true, is_false } => {
            let generated_cond_field = generated_struct.fields_mapped.get(field_name).unwrap().borrow();
            let cond_field_ident = format_ident!("{}", generated_cond_field.name);
            let sub_condition = match &test {
                BranchTestDefinition::BoolValue => {
                    quote!{#cond_field_ident}
                },
                BranchTestDefinition::TestFlag(flag) => {
                    let val = TokenStream::from_str(&format!("{}", flag)).unwrap();
                    quote!{(#cond_field_ident & #val) != 0}
                },
                BranchTestDefinition::TestEqual(val) => {
                    let val = TokenStream::from_str(&format!("{}", val)).unwrap();
                    quote!{#cond_field_ident == #val}
                },
            };

            let mut true_code = quote!();
            let mut false_code = quote!();

            for field in is_true {
                true_code.extend(generate_field_parser_code(generated_struct, field, Some(sub_condition.clone())))
            }

            for field in is_false {
                false_code.extend(generate_field_parser_code(generated_struct, field, Some(quote!{!#sub_condition})))
            }

            let field_names = field.contained_field_names();
            let field_name_idents: Vec<_> = field_names.iter().map(|f| format_ident!("{}", generated_struct.fields_mapped.get(f).unwrap().borrow().name)).collect();

            let true_field_results: Vec<_> = field_names.iter().map(|f| {
                let mut contained = false;
                let mut owned = false;

                for sf in is_true {
                    if sf.has_subfield(f) {
                        contained = true;
                        owned = field.owns_field(f);
                        break;
                    }
                }

                if contained {
                    let ident = format_ident!("{}", generated_struct.fields_mapped.get(f).unwrap().borrow().name);

                    if owned {
                        quote!{ Some(#ident) }
                    } else {
                        quote!{ #ident }
                    }
                } else {
                    quote!{ None }
                }
            }).collect();

            let false_field_results: Vec<_> = field_names.iter().map(|f| {
                let mut contained = false;
                let mut owned = false;

                for sf in is_false {
                    if sf.has_subfield(f) {
                        contained = true;
                        owned = field.owns_field(f);
                        break;
                    }
                }

                if contained {
                    let ident = format_ident!("{}", generated_struct.fields_mapped.get(f).unwrap().borrow().name);
                    
                    if owned {
                        quote!{ Some(#ident) }
                    } else {
                        quote!{ #ident }
                    }
                } else {
                    quote!{ None }
                }
            }).collect();

            quote! {
                let (i, #(#field_name_idents),*) = if #sub_condition {
                    #true_code

                    (i, #(#true_field_results),*)
                } else {
                    #false_code

                    (i, #(#false_field_results),*)
                };
            }
        },
        FieldDefinition::Field { name, .. } => {
            let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
            let field_ident = format_ident!("{}", generated_field.name);
            let field_name = generated_field.name.as_str();
            let parser = generate_nom_parser_for_field(generated_struct, field);

            quote! {
                let (i, #field_ident) = context(#field_name, #parser)(i)?;
            }
        }
    }
}

pub fn generate_parser_code(generated_struct: &GeneratedStruct) -> TokenStream {
    let mut field_parser = Vec::new();

    match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => {
            let def = def.borrow();
            let mut parent_ref = def.inherit.clone();

            while let Some(inherit) = parent_ref {
                match inherit {
                    PacketDefinitionReference::Resolved(parent_def) => {
                        let parent_def = parent_def.borrow();

                        let mut parent_fields = Vec::new();

                        for field in &parent_def.fields {
                            parent_fields.push(generate_field_parser_code(generated_struct, field, None));
                        }

                        field_parser = parent_fields.into_iter().chain(field_parser).collect();

                        parent_ref = parent_def.inherit.clone();
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
                pub fn from_bytes(i: &[u8]) -> IResult<&[u8], CPkt, VerboseError<&[u8]>> { 
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
                pub fn from_bytes(i: &[u8]) -> IResult<&[u8], #struct_ident, VerboseError<&[u8]>> { 
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

pub fn generate_primitive_writer_code(primitive: &str, generated_field: &GeneratedField) -> TokenStream {
    let field_name_ident = format_ident!("{}", generated_field.name);
    let field_getter = if generated_field.optional {
        quote! { self.#field_name_ident.unwrap_or_default() }
    } else {
        quote! { self.#field_name_ident }
    };

    match primitive {
        "bool" => quote! { writer.write(#field_getter as u8)?; },
        "u8" => quote! { writer.write(#field_getter as u8)?; },
        "u16" => quote! { writer.write(#field_getter as u16)?; },
        "u32" => quote! { writer.write(#field_getter as u32)?; },
        "u64" => quote! { writer.write(#field_getter as u64)?; },
        "i8" => quote! { writer.write(#field_getter as i8)?; },
        "i16" => quote! { writer.write(#field_getter as i16)?; },
        "i32" => quote! { writer.write(#field_getter as i32)?; },
        "i64" => quote! { writer.write(#field_getter as i64)?; },
        "f32" => quote! { writer.write_bytes((#field_getter as f32).to_le_bytes().as_slice())?; },
        "f64" => quote! { writer.write_bytes((#field_getter as f64).to_le_bytes().as_slice())?; },
        "avatar_id" => quote! { writer.write(#field_getter.as_u64())?; },
        "uuid" => quote! { writer.write_bytes(#field_getter.to_uuid_1().to_bytes_le().as_slice())?; }, 
        "nativeparam" => quote! { writer.write_bytes(#field_getter.to_struct_bytes().as_slice())?; },
        _ => panic!("Tried to serialize unkown primitive"),
    }
}

pub fn generate_field_writer_code(generated_struct: &GeneratedStruct, field_def: &FieldDefinition) -> TokenStream {
    match field_def {
        FieldDefinition::Branch { field, test, is_true, is_false } => {
            let generated_test_field = generated_struct.fields_mapped.get(field).unwrap().borrow();
            let generated_test_field_ident = format_ident!("{}", generated_test_field.name);

            let true_fields: Vec<_> = is_true.iter().map(|v| generate_field_writer_code(generated_struct, v)).collect();
            let false_fields: Vec<_> = is_false.iter().map(|v| generate_field_writer_code(generated_struct, v)).collect();
            
            if generated_test_field.optional {
                let condition = match &test {
                    BranchTestDefinition::BoolValue => quote!{self.#generated_test_field_ident.unwrap_or_default()},
                    BranchTestDefinition::TestFlag(flag) => {
                        let val = TokenStream::from_str(&format!("{}", flag)).unwrap();
                        quote!{(self.#generated_test_field_ident.unwrap_or_default() & #val) != 0}
                    },
                    BranchTestDefinition::TestEqual(val) => {
                        let val = TokenStream::from_str(&format!("{}", val)).unwrap();
                        quote!{self.#generated_test_field_ident.unwrap_or_default() == #val}
                    },
                };

                quote! {
                    if #condition {
                        #(#true_fields)*
                    } else {
                        #(#false_fields)*
                    }
                }
            } else {
                let condition = match &test {
                    BranchTestDefinition::BoolValue => quote!{self.#generated_test_field_ident},
                    BranchTestDefinition::TestFlag(flag) => {
                        let val = TokenStream::from_str(&format!("{}", flag)).unwrap();
                        quote!{(self.#generated_test_field_ident & #val) != 0}
                    },
                    BranchTestDefinition::TestEqual(val) => {
                        let val = TokenStream::from_str(&format!("{}", val)).unwrap();
                        quote!{self.#generated_test_field_ident == #val}
                    },
                };

                quote! {
                    if #condition {
                        #(#true_fields)*
                    } else {
                        #(#false_fields)*
                    }
                }
            }
        },
        FieldDefinition::Field { name, r#type } => {
            let generated_field = generated_struct.fields_mapped.get(name.as_ref().unwrap()).unwrap().borrow();
            let generated_field_ident = format_ident!("{}", generated_field.name);

            match r#type {
                FieldTypeDefinition::Primitive(primitive) => {
                    generate_primitive_writer_code(primitive, &generated_field)
                },
                FieldTypeDefinition::CString { maxlen } => {
                    let maxlen_ident = if let Some(maxlen) = maxlen {
                        quote! { Some(#maxlen) }
                    } else {
                        quote! { None }
                    };

                    if generated_field.optional {
                        quote! { write_pkt_cstring(&mut writer, self.#generated_field_ident.as_ref().map(|v| v.as_ref()), #maxlen_ident)?; }
                    } else {
                        quote! { write_pkt_cstring(&mut writer, Some(&self.#generated_field_ident), #maxlen_ident)?; }
                    }
                },
                FieldTypeDefinition::WString { maxlen } => {
                    let maxlen_ident = if let Some(maxlen) = maxlen {
                        quote! { Some(#maxlen) }
                    } else {
                        quote! { None }
                    };

                    if generated_field.optional {
                        quote! { write_pkt_wstring(&mut writer, self.#generated_field_ident.as_ref().map(|v| v.as_ref()), #maxlen_ident)?; }
                    } else {
                        quote! { write_pkt_wstring(&mut writer, Some(&self.#generated_field_ident), #maxlen_ident)?; }
                    }
                },
                FieldTypeDefinition::Struct(_) => {
                    if generated_field.optional {
                        quote! { writer.write_bytes(self.#generated_field_ident.as_ref().unwrap().to_bytes().as_slice())?; }
                    } else {
                        quote! { writer.write_bytes(&self.#generated_field_ident.to_bytes().as_slice())?; }
                    }
                },
                FieldTypeDefinition::Enum { primitive, .. } => {
                    let generated_enum = match &generated_field.r#type {
                        GeneratedFieldType::Enum(GeneratedEnumReference::Resolved(generated_enum)) => generated_enum.borrow(),
                        _ => panic!("Enum type mismatch"),
                    };

                    let primitive_datatype = match primitive.as_ref() {
                        FieldTypeDefinition::Primitive(primitive) => primitive.as_str(),
                        _ => panic!("Expected primitive type"),
                    };

                    let generated_enum_ident = format_ident!("{}", generated_enum.name);

                    let enum_write_arms: Vec<_> = generated_enum.values.iter()
                        .map(|v| {
                            let enum_value_ident = format_ident!("{}", v.1);
                            let enum_value = v.0;
                            let write_primitive = match primitive_datatype {
                                "bool" => quote! { writer.write( as u8)? },
                                "u8" => quote! { writer.write(#enum_value as u8)? },
                                "u16" => quote! { writer.write(#enum_value as u16)? },
                                "u32" => quote! { writer.write(#enum_value as u32)? },
                                "u64" => quote! { writer.write(#enum_value as u64)? },
                                "i8" => quote! { writer.write(#enum_value as i8)? },
                                "i16" => quote! { writer.write(#enum_value as i16)? },
                                "i32" => quote! { writer.write(#enum_value as i32)? },
                                "i64" => quote! { writer.write(#enum_value as i64)? },
                                "f32" => quote! { writer.write_bytes((#enum_value as f32).to_le_bytes().as_slice())? },
                                "f64" => quote! { writer.write_bytes((#enum_value as f64).to_le_bytes().as_slice())? },
                                _ => panic!("Tried to serialize unkown primitive"),
                            };

                            quote! {
                                #generated_enum_ident::#enum_value_ident => #write_primitive,
                            }
                        })
                        .collect();

                    quote! {
                        match self.#generated_field_ident {
                            #(#enum_write_arms)*
                        }
                    }
                },
                FieldTypeDefinition::Array { r#type, .. } => {
                    let value_writer = match r#type.as_ref() {
                        FieldTypeDefinition::Primitive(primitive) => {
                            match primitive.as_str() {
                                "bool" => quote! { writer.write(*v as u8)?; },
                                "u8" => quote! { writer.write(*v as u8)?; },
                                "u16" => quote! { writer.write(*v as u16)?; },
                                "u32" => quote! { writer.write(*v as u32)?; },
                                "u64" => quote! { writer.write(*v as u64)?; },
                                "i8" => quote! { writer.write(*v as i8)?; },
                                "i16" => quote! { writer.write(*v as i16)?; },
                                "i32" => quote! { writer.write(*v as i32)?; },
                                "i64" => quote! { writer.write(*v as i64)?; },
                                "f32" => quote! { writer.write_bytes((*v as f32).to_le_bytes().as_slice())?; },
                                "f64" => quote! { writer.write_bytes((*v as f64).to_le_bytes().as_slice())?; },
                                "avatar_id" => quote! { writer.write(v.as_u64())?; },
                                _ => panic!("Tried to serialize unkown primitive"),
                            }
                        },
                        FieldTypeDefinition::CString { maxlen } => {
                            let maxlen_ident = if let Some(maxlen) = maxlen {
                                quote! { Some(#maxlen) }
                            } else {
                                quote! { None }
                            };
        
                            quote! { write_pkt_cstring(&mut writer, Some(v), #maxlen_ident)?; }
                        },
                        FieldTypeDefinition::WString { maxlen } => {
                            let maxlen_ident = if let Some(maxlen) = maxlen {
                                quote! { Some(#maxlen) }
                            } else {
                                quote! { None }
                            };

                            quote! { write_pkt_wstring(&mut writer, Some(v), #maxlen_ident)?; }
                        },
                        FieldTypeDefinition::Struct(_) => {
                            quote! { writer.write_bytes(v.to_bytes().as_slice())?; }
                        },
                        _ => panic!("Invalid array subtipe"),
                    };

                    if generated_field.optional {
                        quote! {
                            if let Some(arr) = &self.#generated_field_ident {
                                for v in arr {
                                    #value_writer
                                }
                            }
                        }
                    } else {
                        quote! {
                            for v in &self.#generated_field_ident {
                                #value_writer
                            }
                        }
                    }
                },
                FieldTypeDefinition::Packet => {
                    quote! { 
                        if let Some(v) = self.#generated_field_ident.as_ref() {
                            writer.write_bytes(v.to_bytes().as_slice())?;
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_writer_code(generated_struct: &GeneratedStruct) -> TokenStream {
    let id_writer = match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => 
        {
            let def = def.borrow();
            let id = def.id;
            let subid = def.sub_id;

            quote! {
                writer.write(#id)?;
                writer.write(#subid)?;
            }
        },
        GeneratedStructSource::StructDefinition(_) => quote! {},
    }; 

    let parent_field_writers: Vec<_> =  match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => {
            let def = def.borrow();
            let mut parent_ref = def.inherit.clone();
            let mut writers = Vec::new();

            while let Some(parent) = parent_ref {
                match parent {
                    PacketDefinitionReference::Resolved(packet_def) => {
                        writers = packet_def.borrow().fields.iter().map(|v| generate_field_writer_code(generated_struct, v)).chain(writers).collect();
                        parent_ref = packet_def.borrow().inherit.clone();
                    },
                    _ => panic!("Unresolved parent definition"),
                }
            }

            writers
        },
        GeneratedStructSource::StructDefinition(def) => {
            let def = def.borrow();
            if let Some(parent) = &def.inherit {
                match parent {
                    StructDefinitionReference::Resolved(packet_def) => {
                        packet_def.borrow().fields.iter().map(|v| generate_field_writer_code(generated_struct, v)).collect()
                    },
                    _ => panic!("Unresolved parent definition"),
                }
            } else {
                Vec::new()
            }
        }
    };

    let field_writers: Vec<_> =  match &generated_struct.definition {
        GeneratedStructSource::PacketDefintion(def) => {
            def.borrow().fields.iter().map(|v| generate_field_writer_code(generated_struct, v)).collect()
        },
        GeneratedStructSource::StructDefinition(def) => {
            def.borrow().fields.iter().map(|v| generate_field_writer_code(generated_struct, v)).collect()
        }
    };

    quote! {
        pub fn to_bytes(&self) -> Vec<u8> {
            let r: io::Result<Vec<u8>> = (|| {
                let mut buf = Vec::new();
                let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

                #id_writer
                #(#parent_field_writers)*
                #(#field_writers)*
                Ok(buf)
            })();
            
            r.expect("Message serialization failed")
        }
    }
}

pub fn generate_implementation_code(structs: &Vec<Rc<RefCell<GeneratedStruct>>>) -> TokenStream {
    let mut code = quote!();

    for generated_struct in structs {
        let generated_struct = generated_struct.borrow();

        if let GeneratedStructSource::PacketDefintion(pkg) = &generated_struct.definition {
            if pkg.borrow().id == 0 { continue; }
        }

        let struct_ident = format_ident!("{}", generated_struct.name);
        let parser_code = generate_parser_code(&generated_struct);
        let writer_code = generate_writer_code(&generated_struct);
        let pkt_code = match &generated_struct.definition {
            GeneratedStructSource::PacketDefintion(_) => {
                quote!{ 
                    pub fn into_pkt(self) -> CPkt {
                        CPkt::#struct_ident(Box::new(self))
                    }
                }
            },
            _ => quote!(),
        };

        code.extend(quote! {
            #[allow(dead_code)]
            impl #struct_ident {
                #parser_code
                #writer_code
                #pkt_code
            }
        });
    }

    code
}