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

use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::collections::HashMap;

use convert_case::{Converter, Boundary, Pattern};

use crate::packet_code_generator::yaml_reader::PacketDefinitionReference;

use super::PacketDefintion;
use super::yaml_reader::{StructDefinition, FieldDefinition, FieldTypeDefinition, StructDefinitionReference};

#[derive(Debug)]
pub struct GeneratedStruct {
    pub name: String,
    pub derive_default: bool,
    pub fields: Vec<Rc<RefCell<GeneratedField>>>,
    pub fields_mapped: HashMap<String, Rc<RefCell<GeneratedField>>>,
    pub definition: GeneratedStructSource,
}

#[derive(Debug)]
pub enum GeneratedStructSource {
    PacketDefintion(Rc<RefCell<PacketDefintion>>),
    StructDefinition(Rc<RefCell<StructDefinition>>)
}

#[derive(Debug)]
pub enum GeneratedStructReference {
    Unresolved(String),
    Resolved(Rc<RefCell<GeneratedStruct>>),
}

#[derive(Debug)]
pub enum GeneratedEnumReference {
    Unresolved(FieldTypeDefinition),
    Resolved(Rc<RefCell<GeneratedEnum>>),
}

#[derive(Debug)]
pub struct GeneratedEnum {
    pub name: String,
    pub values: Vec<(usize, String)>,
}

#[derive(Debug)]
pub struct GeneratedField {
    pub name: String,
    pub original_name: String,
    pub r#type: GeneratedFieldType,
    pub optional: bool,
}

#[derive(Debug)]
pub enum GeneratedFieldType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Uuid,
    NativeParam,
    Array(Box<GeneratedFieldType>),
    Struct(GeneratedStructReference),
    Enum(GeneratedEnumReference),
    Packet
}

impl GeneratedStruct {
    fn generate_fields(&mut self, fields: &[FieldDefinition], as_optional: bool, remove_optional: bool, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) {
        for field in fields {
            match field {
                FieldDefinition::Field { name, r#type } => {
                    let field_name = Converter::new()
                    .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
                    .set_pattern(Pattern::Lowercase)
                    .set_delim("_")
                    .convert(name.as_ref().unwrap());

                    if let Some(existing_field) = self.fields_mapped.get(&field_name) {
                        if remove_optional {
                            existing_field.borrow_mut().optional = false;
                        }
                    } else {
                        let field = Rc::new(RefCell::new(GeneratedField {
                            name: field_name.clone(),
                            original_name: name.as_ref().unwrap().to_owned(),
                            r#type: GeneratedFieldType::from_field_definition(r#type, struct_registry),
                            optional: as_optional,
                        }));

                        if !self.fields_mapped.contains_key(name.as_ref().unwrap()) {
                            self.fields.push(field.clone());
                            self.fields_mapped.insert(name.as_ref().unwrap().to_owned(), field.clone());
                        }
                    } 
                },
                FieldDefinition::Branch { is_true, is_false, .. } => {
                    self.generate_fields(is_true, true, false, struct_registry);
                    self.generate_fields(is_false, true, true, struct_registry);
                }
            }
        }
    }

    pub fn generate_and_resolve_enums(&mut self) -> Vec<Rc<RefCell<GeneratedEnum>>> {
        let mut enums = Vec::new();

        for field in &self.fields {
            let mut field_enums = field.borrow_mut().generate_and_resolve_enums(&self.name);
            enums.append(&mut field_enums);
        }

        enums
    }

    pub fn generate_from_packet_definition(definition: &Rc<RefCell<PacketDefintion>>, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) -> io::Result<Self> {
        let definition_ref = definition.borrow();
        
        let mut generated: GeneratedStruct = Self {
            name: definition_ref.name.clone(),
            derive_default: true,
            fields: Vec::new(),
            fields_mapped: HashMap::new(),
            definition: GeneratedStructSource::PacketDefintion(definition.clone())
        };
        
        if let Some(parent) = &definition_ref.inherit {
            if let Some(parent) = match parent {
                    PacketDefinitionReference::Resolved(parent) => {
                        Some(Self::generate_from_packet_definition(parent, struct_registry)?)
                        //generated.generate_fields(&parent.borrow().fields, false, false, struct_registry);
                    },
                    _ => panic!(),
                } {

                generated.fields = parent.fields;
                generated.fields_mapped = parent.fields_mapped;
            }
        }

        generated.generate_fields(&definition_ref.fields, false, false, struct_registry);

        Ok(generated)
    }

    pub fn generate_from_struct_definition(definition: &Rc<RefCell<StructDefinition>>) -> io::Result<Self>  {
        let definition_ref = definition.borrow();
        let empty_registry = HashMap::new();
        
        let mut generated: GeneratedStruct = Self {
            name: definition_ref.name.clone(),
            derive_default: true,
            fields: Vec::new(),
            fields_mapped: HashMap::new(),
            definition: GeneratedStructSource::StructDefinition(definition.clone())
        };
        
        if let Some(parent) = &definition_ref.inherit {
            match parent {
                crate::packet_code_generator::yaml_reader::StructDefinitionReference::Resolved(parent) => {
                    generated.generate_fields(&parent.borrow().fields, false, false, &empty_registry);
                },
                _ => panic!(),
            }
        }

        generated.generate_fields(&definition_ref.fields, false, false, &empty_registry);

        Ok(generated)
    }

    pub fn resolve_references(&mut self, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) -> io::Result<()> {
        for field in &self.fields {
            field.borrow_mut().resolve_references(struct_registry)?;
        }

        Ok(())
    }
}

impl GeneratedField {
    pub fn generate_and_resolve_enums(&mut self, prefix: &str) -> Vec<Rc<RefCell<GeneratedEnum>>> {
        let mut enums = Vec::new();

        if let GeneratedFieldType::Enum(enum_ref) = &mut self.r#type {
            if let GeneratedEnumReference::Unresolved(def) = enum_ref {
                let naming_case = Converter::new()
                .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
                .set_pattern(Pattern::Capital)
                .set_delim("");

                let enum_name = naming_case.convert(format!("{}_{}", prefix, self.name));

                if let FieldTypeDefinition::Enum { values, .. } = def {
                    let enum_values = values.iter()
                    .map(|v| (v.0, naming_case.convert(&v.1)))
                    .collect();
                
                    let generated = Rc::new(RefCell::new(GeneratedEnum {
                        name: enum_name,
                        values: enum_values,
                    }));

                    enums.push(generated.clone());
                    *enum_ref = GeneratedEnumReference::Resolved(generated);
                } else {
                    panic!("Enum reference did point to non enum type defintion!")
                }
            }
        }

        enums
    }

    pub fn resolve_references(&mut self, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) -> io::Result<()> {
        self.r#type.resolve_references(struct_registry)
    }
}

impl GeneratedFieldType {
    pub fn from_field_definition(definition: &FieldTypeDefinition, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) -> Self {
        match definition {
            FieldTypeDefinition::Primitive(primitive) => {
                match primitive.as_str() {
                    "bool" => GeneratedFieldType::Bool,
                    "u8" => GeneratedFieldType::U8,
                    "u16" => GeneratedFieldType::U16,
                    "u32" => GeneratedFieldType::U32,
                    "u64" => GeneratedFieldType::U64,
                    "i8" => GeneratedFieldType::I8,
                    "i16" => GeneratedFieldType::I16,
                    "i32" => GeneratedFieldType::I32,
                    "i64" => GeneratedFieldType::I64,
                    "f32" => GeneratedFieldType::F32,
                    "f64" => GeneratedFieldType::F64,
                    "uuid" => GeneratedFieldType::Uuid,
                    "nativeparam" => GeneratedFieldType::NativeParam,
                    _ => panic!()
                }
            },
            FieldTypeDefinition::CString { .. } => GeneratedFieldType::String,
            FieldTypeDefinition::WString { .. } => GeneratedFieldType::String,
            FieldTypeDefinition::Array { r#type, .. } => 
                GeneratedFieldType::Array(Box::new(GeneratedFieldType::from_field_definition(r#type, struct_registry))),
            FieldTypeDefinition::Struct(struct_ref) =>
                match struct_ref {
                    StructDefinitionReference::Resolved(struct_ref) => 
                        if let Some(resolved_struct) = struct_registry.get(&struct_ref.borrow().name) {
                            GeneratedFieldType::Struct(GeneratedStructReference::Resolved(resolved_struct.clone()))
                        } else {
                            GeneratedFieldType::Struct(GeneratedStructReference::Unresolved(struct_ref.borrow().name.to_owned()))
                        },
                    StructDefinitionReference::Unresolved(name) => panic!("Unresolved struct reference: {}", name),
                },
            FieldTypeDefinition::Enum { .. } => 
                GeneratedFieldType::Enum(GeneratedEnumReference::Unresolved(definition.clone())),
            FieldTypeDefinition::Packet => GeneratedFieldType::Packet,
        }
    }

    pub fn resolve_references(&mut self, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) -> io::Result<()> {
        match self {
            GeneratedFieldType::Struct(reference) => {
                match reference {
                    GeneratedStructReference::Unresolved(name) => {
                        println!("Resolving {}", name);

                        let resolved = struct_registry.get(name);
                        if let Some(resolved) = resolved {
                            *reference = GeneratedStructReference::Resolved(resolved.clone());
                        } else {
                            panic!("Failed to resolve previously resolved struct reference")
                        }
                        Ok(())
                    },
                    GeneratedStructReference::Resolved(_) => {
                        println!("Struct is resolved");
                        Ok(())
                    },
                }
            },
            GeneratedFieldType::Array(r#type) => {
                r#type.resolve_references(struct_registry)
            }
            _ => Ok(())
        }
    }
}
