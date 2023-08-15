use std::{fs, io, path::Path};
use yaml_rust::{YamlLoader, yaml, Yaml};
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct PacketDefintion {
    id: u8,
    sub_id: u8,
    name: String,
    inherit: Option<String>,
    fields: Vec<FieldDefinition>,
}

#[derive(Debug)]
pub struct StructDefinition {
    name: String,
    inherit: Option<String>,
    fields: Vec<FieldDefinition>,
}

#[derive(Debug)]
pub enum FieldDefinition {
    Field { name: Option<String>, r#type: FieldTypeDefinition },
    Branch { field: String, is_true: Vec<FieldDefinition>, is_false: Vec<FieldDefinition> },
}

#[derive(Debug)]
pub enum FieldLengthDefinition {
    ConstLen(usize),
    DynamicLen(String),
}

#[derive(Debug)]
pub enum FieldTypeDefinition {
    Primitive(String),
    Struct(String),
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
        let fields = yaml_defintion["fields"].as_vec()
            .ok_or(io::Error::new(io::ErrorKind::Other, "fields required"))?;

        let mut definition = Self {
            id,
            sub_id,
            name: name.to_owned(),
            inherit: inherit.map(|v| v.to_owned()), 
            fields: Vec::new(),
        };

        for yaml_field in fields {
            definition.fields.push(FieldDefinition::load_from_yaml(yaml_field)?);
        }

        Ok(definition)
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
            Ok(Self::Struct(type_name[1..].to_owned()))
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
}

pub fn load_definitions(path: &str) -> io::Result<Vec<PacketDefintion>> {
    let mut defitions = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type()?.is_file() || 
            !(Path::new(&entry.file_name()).extension().unwrap() == "yaml") { continue; }

        println!("Parsing definition {}...", entry.file_name().to_string_lossy());

        let def = PacketDefintion::load_from_file(entry.path().to_str().unwrap())?;
        println!("{:#?}", def);

        defitions.push(def);
    }

    Ok(defitions)
}