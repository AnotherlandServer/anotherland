use std::{fs, io, path::Path, collections::HashMap, rc::Rc, cell::RefCell};
use yaml_rust::{YamlLoader, Yaml};

#[derive(Debug)]
pub struct PacketDefintion {
    pub id: u8,
    pub sub_id: u8,
    pub name: String,
    pub inherit: Option<PacketDefinitionReference>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug)]
pub enum PacketDefinitionReference {
    Unresolved(String),
    Resolved(Rc<RefCell<PacketDefintion>>),
}

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub inherit: Option<StructDefinitionReference>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Clone)]
pub enum StructDefinitionReference {
    Unresolved(String),
    Resolved(Rc<RefCell<StructDefinition>>),
}

#[derive(Debug)]
pub enum FieldDefinition {
    Field { name: Option<String>, r#type: FieldTypeDefinition },
    Branch { field: String, is_true: Vec<FieldDefinition>, is_false: Vec<FieldDefinition> },
}

#[derive(Debug, Clone)]
pub enum FieldLengthDefinition {
    ConstLen(usize),
    DynamicLen(String),
}

#[derive(Debug, Clone)]
pub enum FieldTypeDefinition {
    Primitive(String),
    Struct(StructDefinitionReference),
    CString { maxlen: Option<usize> },
    WString { maxlen: Option<usize> },
    Array { len: FieldLengthDefinition, r#type: Box<FieldTypeDefinition> },
    Enum { primitive: Box<FieldTypeDefinition>, values: Vec<(usize, String)> },
}

impl PacketDefintion {
    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let yaml_contents = fs::read(file_path)?;
        let yaml_doc = YamlLoader::load_from_str(String::from_utf8_lossy(&yaml_contents).as_ref())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let yaml_defintion = &yaml_doc[0];

        let name = Path::new(file_path).file_stem()
            .ok_or(io::ErrorKind::InvalidInput)?
            .to_str().unwrap();

        let id = yaml_defintion["id"].as_i64()
            .ok_or(io::Error::new(io::ErrorKind::Other, "id required"))? as u8;
        let sub_id = yaml_defintion["subId"].as_i64()
            .ok_or(io::Error::new(io::ErrorKind::Other, "subId required"))? as u8;
        let inherit = yaml_defintion["inherit"].as_str();
        let fields = yaml_defintion["fields"].as_vec();;

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

    pub fn resolve_references(&mut self, packets: &HashMap<String, Rc<RefCell<PacketDefintion>>>, structs: &HashMap<String, Rc<RefCell<StructDefinition>>>) -> io::Result<()> {
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
            field.resolve_references(packets, structs)?;
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
        let mut field_count = if let Some(parent) = &self.inherit {
            match parent {
                PacketDefinitionReference::Resolved(parent) => parent.borrow().count_fields(),
                _ => 0usize
            }
        } else {
            0usize
        };

        for field in &mut self.fields {
            field.normalize(&mut field_count);
        }
    }
}

impl StructDefinition {
    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let yaml_contents = fs::read(file_path)?;
        let yaml_doc = YamlLoader::load_from_str(String::from_utf8_lossy(&yaml_contents).as_ref())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let yaml_defintion = &yaml_doc[0];

        let name = Path::new(file_path).file_stem()
            .ok_or(io::ErrorKind::InvalidInput)?
            .to_str().unwrap();

        let inherit = yaml_defintion["inherit"].as_str();
        let fields = yaml_defintion["fields"].as_vec()
            .ok_or(io::Error::new(io::ErrorKind::Other, "fields required"))?;

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

    pub fn resolve_references(&mut self, packets: &HashMap<String, Rc<RefCell<PacketDefintion>>>, structs: &HashMap<String, Rc<RefCell<StructDefinition>>>) -> io::Result<()> {
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
            field.resolve_references(packets, structs)?;
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
        let mut field_count = if let Some(parent) = &self.inherit {
            match parent {
                StructDefinitionReference::Resolved(parent) => parent.borrow().count_fields(),
                _ => 0usize
            }
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
                .ok_or(io::Error::new(io::ErrorKind::Other, "field required"))?;
            let yaml_is_true = yaml["branch"]["isTrue"]["fields"].as_vec()
                .ok_or(io::Error::new(io::ErrorKind::Other, "fields required"))?;
            let yaml_is_false = yaml["branch"]["isFalse"]["fields"].as_vec()
                .ok_or(io::Error::new(io::ErrorKind::Other, "fields required"))?;

            let mut is_true = Vec::new();
            let mut is_false = Vec::new();

            for yaml_field in yaml_is_true { is_true.push(Self::load_from_yaml(yaml_field)?); }
            for yaml_field in yaml_is_false { is_false.push(Self::load_from_yaml(yaml_field)?); }

            Ok(FieldDefinition::Branch { field: field.to_owned(), is_true, is_false })
        } else if !yaml["type"].is_badvalue() && !yaml["type"].is_null() {
            let name = yaml["name"].as_str().map(|v| v.to_owned());
            let yaml_type = &yaml["type"];

            Ok(FieldDefinition::Field { name, r#type: FieldTypeDefinition::load_from_yaml(yaml_type)? })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid field type"))
        }
    }

    pub fn resolve_references(&mut self, packets: &HashMap<String, Rc<RefCell<PacketDefintion>>>, structs: &HashMap<String, Rc<RefCell<StructDefinition>>>) -> io::Result<()> {
        match self {
            FieldDefinition::Branch { is_true, is_false, .. } => {
                for field in is_true {
                    field.resolve_references(packets, structs)?;
                }

                for field in is_false {
                    field.resolve_references(packets, structs)?;
                }

                Ok(())
            }
            FieldDefinition::Field { name, r#type } => {
                r#type.resolve_references(packets, structs)
                    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to resolve type for field {}: {}", name.as_ref().unwrap(), e.to_string())))
            },
        }
    }

    fn count_fields(&self) -> usize {
        match self {
            FieldDefinition::Branch { is_true, is_false, .. } => {
                let mut count = 0usize;
                
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
                    *name = Some(format!("field_{}", field_count));
                }

                *field_count += 1;
            },
            FieldDefinition::Branch { is_true, is_false, .. } => {
                for field in is_true { field.normalize(field_count); }
                for field in is_false { field.normalize(field_count); }
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
                "i64" => Ok(Self::Primitive(type_name.to_owned())),
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
                    } else {
                        FieldLengthDefinition::DynamicLen(yaml["len"].as_str()
                            .ok_or(io::Error::new(io::ErrorKind::Other, "len required"))?
                            .to_owned())
                    };
                    
                    let r#type = FieldTypeDefinition::load_from_yaml(&yaml["type"])?;
                    Ok(Self::Array { len, r#type: Box::new(r#type) })
                },
                _ => Err(io::Error::new(io::ErrorKind::Other, format!("invalid type defition: {}", type_name)))
            }
        }
    }

    pub fn resolve_references(&mut self, packets: &HashMap<String, Rc<RefCell<PacketDefintion>>>, structs: &HashMap<String, Rc<RefCell<StructDefinition>>>) -> io::Result<()> {
        match self {
            FieldTypeDefinition::Struct(struct_reference) => {
                match struct_reference {
                    StructDefinitionReference::Unresolved(name) => {
                        if let Some(resolved) = structs.get(name) {
                            println!("Resolved {}", name);

                            *struct_reference = StructDefinitionReference::Resolved(resolved.clone());
                            Ok(())
                        } else {
                            Err(io::Error::new(io::ErrorKind::NotFound, format!("struct {} not found", name)))
                        }
                    },
                    StructDefinitionReference::Resolved(_) => Ok(()),
                }
            },
            FieldTypeDefinition::Array {  r#type, .. } => r#type.resolve_references(packets, structs),
            _ => Ok(())
        }
    }
}

pub fn load_packet_definitions(path: &str) -> io::Result<HashMap<String, Rc<RefCell<PacketDefintion>>>> {
    let mut definitions = HashMap::new();

    // parse all files
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type()?.is_file() || 
            !(Path::new(&entry.file_name()).extension().unwrap() == "yaml") { continue; }

        println!("Parsing packet definition {}...", entry.file_name().to_string_lossy());

        let def = PacketDefintion::load_from_file(entry.path().to_str().unwrap())?;
        println!("{:#?}", def);

        definitions.insert(def.name.clone(), Rc::new(RefCell::new(def)));
    }

    // resolve inheritance
    for (name, definition) in &definitions {
        let mut definition = definition.borrow_mut();

        if let Some(inherit) = &definition.inherit {
            match inherit {
                PacketDefinitionReference::Unresolved(parent_name) => {
                    if let Some(parent) = definitions.get(parent_name) {
                        definition.inherit = Some(PacketDefinitionReference::Resolved(parent.clone()));
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::NotFound, 
                            format!("Inherited struct {} not found for packet {}!", parent_name, name)
                        ))
                    }
                }
                _ => Ok(()),
            }?;
        }
    }

    Ok(definitions)
}

pub fn load_struct_definitions(path: &str) -> io::Result<HashMap<String, Rc<RefCell<StructDefinition>>>> {
    let mut definitions = HashMap::new();

    // parse all files
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type()?.is_file() || 
            !(Path::new(&entry.file_name()).extension().unwrap() == "yaml") { continue; }

        println!("Parsing struct definition {}...", entry.file_name().to_string_lossy());

        let def = StructDefinition::load_from_file(entry.path().to_str().unwrap())?;

        definitions.insert(def.name.clone(), Rc::new(RefCell::new(def)));
    }

    // resolve inheritance
    for (name, definition) in &definitions {
        let mut definition = definition.borrow_mut();

        if let Some(inherit) = &definition.inherit {
            match inherit {
                StructDefinitionReference::Unresolved(parent_name) => {
                    if let Some(parent) = definitions.get(parent_name) {
                        definition.inherit = Some(StructDefinitionReference::Resolved(parent.clone()));
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::NotFound, 
                            format!("Inherited struct {} not found for packet {}!", parent_name, name)
                        ))
                    }
                }
                _ => Ok(()),
            }?;
        }
    }

    Ok(definitions)
}