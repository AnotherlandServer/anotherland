// Copyright (C) 2025 AnotherlandServer
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

use std::{fs, io, path::Path, collections::HashMap, rc::Rc, cell::RefCell};
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};
use itertools::Itertools;

pub struct DefinitionFile {
    packets: Vec<PacketDefintion>,
    structures: Vec<StructDefinition>,
}
#[derive(Debug)]
pub struct PacketDefintion {
    pub id: u8,
    pub sub_id: u8,
    pub name: String,
    pub inherit: Option<PacketDefinitionReference>,
    pub fields: Vec<FieldDefinition>,
}

type PacketDefintionRef = Rc<RefCell<PacketDefintion>>;

#[derive(Debug, Clone)]
pub enum PacketDefinitionReference {
    Unresolved(String),
    Resolved(PacketDefintionRef),
}

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub inherit: Option<StructDefinitionReference>,
    pub fields: Vec<FieldDefinition>,
}

type StructDefintionRef = Rc<RefCell<StructDefinition>>;

#[derive(Debug, Clone)]
pub enum StructDefinitionReference {
    Unresolved(String),
    Resolved(StructDefintionRef),
}

#[derive(Debug)]
pub enum BranchTestDefinition {
    BoolValue,
    TestFlag(u32),
    TestEqual(u32),
}

#[derive(Debug)]
pub enum FieldDefinition {
    Field { name: Option<String>, r#type: FieldTypeDefinition },
    Branch { field: String, test: BranchTestDefinition, is_true: Vec<FieldDefinition>, is_false: Vec<FieldDefinition> },
}

#[derive(Debug, Clone)]
pub enum FieldLengthDefinition {
    ConstLen(usize),
    DynamicLen(String),
    Remainder,
}

#[derive(Debug, Clone)]
pub enum FieldTypeDefinition {
    Primitive(String),
    Struct(StructDefinitionReference),
    CString { maxlen: Option<usize> },
    WString { maxlen: Option<usize> },
    Array { len: FieldLengthDefinition, r#type: Box<FieldTypeDefinition> },
    Enum { primitive: Box<FieldTypeDefinition>, values: Vec<(usize, String)> },
    Packet,
}

impl DefinitionFile {
    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let yaml_contents = fs::read(file_path)?;
        let yaml_doc = YamlLoader::load_from_str(String::from_utf8_lossy(&yaml_contents).as_ref())
            .map_err(|e| io::Error::other(e.to_string()))?;
        let yaml_defintion = &yaml_doc[0];

        let mut definition = DefinitionFile {
            packets: Vec::new(),
            structures: Vec::new(),
        };

        // parse packets
        for (name, packet) in yaml_defintion["packets"].as_hash().unwrap_or(&Hash::new()) {
            definition.packets.push(PacketDefintion::load_from_yaml(
                name.as_str().ok_or(io::ErrorKind::InvalidData)?, 
                packet)?);
        }

        // parse structs
        for (name, packet) in yaml_defintion["structures"].as_hash().unwrap_or(&Hash::new()) {
            definition.structures.push(StructDefinition::load_from_yaml(
                name.as_str().ok_or(io::ErrorKind::InvalidData)?, 
                packet)?);
        }

        Ok(definition)
    }
}

impl PacketDefintion {
    pub fn load_from_yaml(name: &str, yaml: &Yaml) -> io::Result<Self> {
        let id = yaml["id"].as_i64()
            .ok_or(io::Error::other("id required"))? as u8;
        let sub_id = yaml["subId"].as_i64()
            .ok_or(io::Error::other("subId required"))? as u8;
        let inherit = yaml["inherit"].as_str();
        let fields = yaml["fields"].as_vec();

        let mut definition = Self {
            id,
            sub_id,
            name: name.to_owned(),
            inherit: inherit.map(|v| PacketDefinitionReference::Unresolved(v.to_owned())), 
            fields: Vec::new(),
        };

        if let Some(fields) = fields {
            for yaml_field in fields {
                definition.fields.push(FieldDefinition::load_from_yaml(yaml_field)?);
            }
        }

        Ok(definition)
    }
    
    pub fn resolve_references(&mut self, packets: &HashMap<String, PacketDefintionRef>, structs: &HashMap<String, StructDefintionRef>) -> io::Result<()> {
        if let Some(inherit) = &self.inherit {
            match inherit {
                PacketDefinitionReference::Unresolved(parent_name) => {
                    if let Some(parent) = packets.get(parent_name) {
                        self.inherit = Some(PacketDefinitionReference::Resolved(parent.clone()));
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::NotFound, 
                            format!("Inherited struct {} not found for packet {}!", parent_name, self.name)
                        ))
                    }
                }
                _ => Ok(()),
            }?;
        }

        for field in &mut self.fields {
            field.resolve_references(structs)?;
        }

        Ok(())
    }

    fn count_fields(&self) -> usize {
        let mut count = 0usize;

        for field in &self.fields {
            count += field.count_fields();
        }

        count
    }

    pub fn normalize(&mut self) {
        // initialize with parent fieldcount
        let mut field_count = if let Some(PacketDefinitionReference::Resolved(parent)) = &self.inherit {
            parent.borrow().count_fields()
        } else {
            0usize
        };

        for field in &mut self.fields {
            field.normalize(&mut field_count);
        }
    }
}

impl StructDefinition {
    pub fn load_from_yaml(name: &str, yaml: &Yaml) -> io::Result<Self> {
        let inherit = yaml["inherit"].as_str();
        let fields = yaml["fields"].as_vec()
            .ok_or(io::Error::other("fields required"))?;

        let mut definition = Self {
            name: name.to_owned(),
            inherit: inherit.map(|v| StructDefinitionReference::Unresolved(v.to_owned())), 
            fields: Vec::new(),
        };

        for yaml_field in fields {
            definition.fields.push(FieldDefinition::load_from_yaml(yaml_field)?);
        }

        Ok(definition)
    }

    pub fn resolve_references(&mut self, structs: &HashMap<String, StructDefintionRef>) -> io::Result<()> {
        if let Some(inherit) = &self.inherit {
            match inherit {
                StructDefinitionReference::Unresolved(parent_name) => {
                    if let Some(parent) = structs.get(parent_name) {
                        self.inherit = Some(StructDefinitionReference::Resolved(parent.clone()));
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::NotFound, 
                            format!("Inherited struct {} not found for struct {}!", parent_name, self.name)
                        ))
                    }
                }
                _ => Ok(()),
            }?;
        }

        for field in &mut self.fields {
            field.resolve_references(structs)?;
        }

        Ok(())
    }

    
    fn count_fields(&self) -> usize {
        let mut count = 0usize;

        for field in &self.fields {
            count += field.count_fields();
        }

        count
    }

    pub fn normalize(&mut self) {
        // initialize with parent fieldcount
        let mut field_count = if let Some(StructDefinitionReference::Resolved(parent)) = &self.inherit {
            parent.borrow().count_fields()
        } else {
            0usize
        };

        for field in &mut self.fields {
            field.normalize(&mut field_count);
        }
    }
}

impl FieldDefinition {
    pub fn load_from_yaml(yaml: &Yaml) -> io::Result<Self> {
        if !yaml["branch"].is_badvalue() && !yaml["branch"].is_null() {
            let field = yaml["branch"]["field"].as_str()
                .ok_or(io::Error::other("field required"))?;

            let mut is_true = Vec::new();
            let mut is_false = Vec::new();

            if let Some(yaml_is_true) = yaml["branch"]["isTrue"]["fields"].as_vec() {
                for yaml_field in yaml_is_true { is_true.push(Self::load_from_yaml(yaml_field)?); }
            }
 
            if let Some(yaml_is_false) = yaml["branch"]["isFalse"]["fields"].as_vec() {
                for yaml_field in yaml_is_false { is_false.push(Self::load_from_yaml(yaml_field)?); }
            }

            let branch_test = if let Some(yaml_test_flag) = yaml["branch"]["test_flag"].as_i64() {
                BranchTestDefinition::TestFlag(yaml_test_flag as u32)
            } else if let Some(yaml_test_equal) = yaml["branch"]["test_equal"].as_i64() {
                BranchTestDefinition::TestEqual(yaml_test_equal as u32)
            } else {
                BranchTestDefinition::BoolValue
            };
                
            Ok(FieldDefinition::Branch { field: field.to_owned(), test: branch_test, is_true, is_false })
        } else if !yaml["type"].is_badvalue() && !yaml["type"].is_null() {
            let name = yaml["name"].as_str().map(|v| v.to_owned());
            let yaml_type = &yaml["type"];

            Ok(FieldDefinition::Field { name, r#type: FieldTypeDefinition::load_from_yaml(yaml_type)? })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid field type"))
        }
    }

    pub fn resolve_references(&mut self, structs: &HashMap<String, StructDefintionRef>) -> io::Result<()> {
        match self {
            FieldDefinition::Branch { is_true, is_false, .. } => {
                for field in is_true {
                    field.resolve_references(structs)?;
                }

                for field in is_false {
                    field.resolve_references(structs)?;
                }

                Ok(())
            }
            FieldDefinition::Field { name, r#type } => {
                r#type.resolve_references(structs)
                    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to resolve type for field {}: {}", name.as_ref().unwrap(), e)))
            },
        }
    }

    fn count_fields(&self) -> usize {
        match self {
            FieldDefinition::Branch { is_true, is_false, .. } => {
                let mut count: usize = 0usize;
                
                for field in is_true {
                    count += field.count_fields();
                }

                for field in is_false {
                    count += field.count_fields();
                }

                count
            },
            _ => 1usize
        }
    }

    pub fn normalize(&mut self, field_count: &mut usize) {
        match self {
            FieldDefinition::Field { name, .. } => {
                if name.is_none() {
                    *name = Some(format!("field_{field_count}"));
                }

                *field_count += 1;
            },
            FieldDefinition::Branch { is_true, is_false, .. } => {
                for field in is_true { field.normalize(field_count); }
                for field in is_false { field.normalize(field_count); }
            }
        }
    }

    pub fn contained_field_names(&self) -> Vec<String> {
        match self {
            FieldDefinition::Field { name, .. } => {
                if let Some(name) = name {
                    vec![name.to_owned()]
                } else {
                    vec![]
                }
            },
            FieldDefinition::Branch { is_true, is_false, .. } => {
                let mut res = Vec::new();

                for field in is_true {
                    res.append(&mut field.contained_field_names());
                }

                for field in is_false { 
                    res.append(&mut field.contained_field_names());
                }

                res.into_iter().unique().collect()
            }
        }
    }

    pub fn has_subfield(&self, name: &String) -> bool {
        self.contained_field_names().contains(name)
    }

    pub fn owns_field(&self, name: &String) -> bool {
        let lookup_name = name;

        match self {
            FieldDefinition::Field { name, .. } => {
                if let Some(name) = name {
                    name == lookup_name
                } else {
                    false
                }
            },
            FieldDefinition::Branch { is_true, is_false, .. } => {
                for field in is_true {
                    if 
                        let FieldDefinition::Field { name: Some(name), .. } = field &&
                        name == lookup_name
                    {
                        return true;
                    }
                }

                for field in is_false { 
                    if 
                        let FieldDefinition::Field { name: Some(name), .. } = field &&
                        name == lookup_name
                    {
                        return true;
                    }
                }

                false
            }
        }
    }
}

impl FieldTypeDefinition {
    pub fn load_from_yaml(yaml: &Yaml) -> io::Result<Self> {
        let type_name = if let Some(str_type) = yaml.as_str() {
            Ok(str_type)
        } else if let Some(str_type) = yaml["name"].as_str() {
            Ok(str_type)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid type definition"))
        }?;

        if &type_name[..1] == ":" {
            Ok(Self::Struct(StructDefinitionReference::Unresolved(type_name[1..].to_owned())))
        } else if let Some(enum_map) = yaml["enum"].as_hash() {
            let mut enum_values = Vec::new();

            for (k,v) in enum_map {
                enum_values.push((
                    k.as_i64()
                        .map(|v| v as usize)
                        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "invalid enum value"))?,
                    v.as_str()
                        .map(|v| v.to_owned())
                        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "invalid enum value identifier"))?
                ));
            }

            Ok(Self::Enum { primitive: Box::new(FieldTypeDefinition::load_from_yaml(&yaml["name"])?), values: enum_values })
        } else {
            match type_name {
                "bool" | 
                "u8" | 
                "u16" | 
                "u32" | 
                "u64" | 
                "i8" | 
                "i16" | 
                "i32" | 
                "i64" |
                "f32" |
                "f64" |
                "uuid" |
                "buffer" |
                "nativeparam" |
                "avatar_id" => Ok(Self::Primitive(type_name.to_owned())),
                "cstring" => {
                    let maxlen = yaml["maxlen"].as_i64().map(|v| v as usize);
                    Ok(Self::CString { maxlen })
                },
                "wstring" => {
                    let maxlen = yaml["maxlen"].as_i64().map(|v| v as usize);
                    Ok(Self::WString { maxlen })
                },
                "array" => {
                    let len = if let Some(len) = yaml["len"].as_i64() {
                        FieldLengthDefinition::ConstLen(len as usize)
                    } else if yaml["len"].as_str().ok_or(io::Error::other("len required"))? == "_eof" {
                        FieldLengthDefinition::Remainder
                    } else {
                        FieldLengthDefinition::DynamicLen(yaml["len"].as_str()
                            .ok_or(io::Error::other("len required"))?
                            .to_owned())
                    };
                    
                    let r#type = FieldTypeDefinition::load_from_yaml(&yaml["type"])?;
                    Ok(Self::Array { len, r#type: Box::new(r#type) })
                },
                "packet" => Ok(Self::Packet),
                _ => Err(io::Error::other(format!("invalid type defition: {type_name}")))
            }
        }
    }

    pub fn resolve_references(&mut self, structs: &HashMap<String, StructDefintionRef>) -> io::Result<()> {
        match self {
            FieldTypeDefinition::Struct(struct_reference) => {
                match struct_reference {
                    StructDefinitionReference::Unresolved(name) => {
                        if let Some(resolved) = structs.get(name) {
                            println!("Resolved {name}");

                            *struct_reference = StructDefinitionReference::Resolved(resolved.clone());
                            Ok(())
                        } else {
                            Err(io::Error::new(io::ErrorKind::NotFound, format!("struct {name} not found")))
                        }
                    },
                    StructDefinitionReference::Resolved(_) => Ok(()),
                }
            },
            FieldTypeDefinition::Array {  r#type, .. } => r#type.resolve_references(structs),
            _ => Ok(())
        }
    }
}

pub fn load_definitions(path: &str) -> 
    io::Result<(
        HashMap<String, PacketDefintionRef>, 
        HashMap<String, StructDefintionRef>
    )> {

    let mut packet_definitions = HashMap::new();
    let mut struct_definitions = HashMap::new();

    // parse all files
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type()?.is_file() || 
            Path::new(&entry.file_name()).extension().unwrap() != "yaml" { continue; }

        println!("Parsing definition {}...", entry.file_name().to_string_lossy());

        let def = DefinitionFile::load_from_file(entry.path().to_str().unwrap())?;
        def.packets.into_iter().for_each(|v| { packet_definitions.insert(v.name.clone(), Rc::new(RefCell::new(v))); });
        def.structures.into_iter().for_each(|v| { struct_definitions.insert(v.name.clone(), Rc::new(RefCell::new(v))); });
    }

    Ok((packet_definitions, struct_definitions))
}
