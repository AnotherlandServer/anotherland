use std::{path::{Path, PathBuf}, io, fs, collections::HashMap, env, rc::Rc, cell::RefCell};

use nom::{IResult, character::complete, error::Error};
use proc_macro2::TokenStream;
use quote::format_ident;
use regex::Regex;
use ::quote::quote;

use convert_case::{Converter, Boundary, Pattern, Case, Casing};

use crate::write_source;

#[derive(Debug, Default)]
struct Paramlist {
    data_ver: u8,

    default_client_avater_class: String,
    default_party_class: String,
    default_trade_class: String,
    default_mail_class: String,
    default_clan_class: String,

    tables: Vec<(u8, String)>,

    classes: Vec<Rc<RefCell<ParamClass>>>
}

#[derive(Debug, Clone, Default)]
struct ParamClass {
    unique_id: u16,
    name: String,
    extends: String,
    binds_to: Vec<String>,
    content_table_binding: String,
    icon: String,
    paramid: Vec<(String, u16)>,
    paramoption: Vec<(String, ParamOptions)>
}

#[derive(Debug)]
enum ParamIniLine {
    Ignore,
    DataVer(u8),
    DefaultClientAvatarClass(String),
    DefaultPartyClass(String),
    DefaultTradeClass(String),
    DefaultMailClass(String),
    DefaultClanClass(String),
    Table(u8, String),
    Class(String, String, String),
    ParamId(String, u16),
    ParamOptions(String, ParamOptions),
}

#[derive(Debug, Clone)]
enum ParamType {
    Any,
    AvatarID,
    AvatarIDSet,
    AvatarIDVector,
    BitSetFilter,
    Bool,
    ClassRefPowerRangeList,
    ContentRef,
    ContentRefAndInt,
    ContentRefList,
    Float,
    FloatRange,
    FloatVector,
    Guid,
    GuidPair,
    Int,
    Int64,
    Int64Vector,
    IntVector,
    JSON,
    LocalizedString,
    String,
    StringFloatPair,
    StringStringHashmap,
    StringIntHashmap,
    StringVector,
    Vector3,
    OAInstanceGroup,
    OASetGuid,
    OAVectorGuid,
    OAVactorLocalizedString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParamFlag {
    NodeOwn,
    ServerOwn,
    ClientOwn,
    ClientUnknown,
    ClientPrivileged,
    ClientInit,
    Persistent,
    ExcludeFromClient,
    Content,
    PerInstanceSetting,
    DupeSetOk,
    Deprecated,
    Metric,
    EquipSlot,
    Uts,
}

#[derive(Debug, Clone)]
struct ParamOptions {
    param_type: ParamType,
    default: Option<String>,
    flags: Vec<ParamFlag>,
}

pub fn generate_param_code(client_path: &Path) -> io::Result<()> {
    let paramlist_path = client_path.join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/paramlist.ini");

    let file_content = {
        let data: Vec<u16> = fs::read(paramlist_path)?
            .chunks_exact(2)
            .into_iter()
            .map(|v| u16::from_ne_bytes([v[0], v[1]]))
            .collect();
        String::from_utf16(&data[1..])
    }.expect("Failed to read paramlist.ini");

    let re = Regex::new(r#""((?:\\"|[^"])*)"|([^ ,]+)"#).unwrap();

    let lines = file_content.lines();
    let parsed_lines: Vec<_> = lines.enumerate()
        .map(|(line_num, line)| -> IResult<&str, ParamIniLine> {
            //let tokens: Vec<&str> = line.split_whitespace().collect();
            let tokens: Vec<&str> = re.captures_iter(line)
            .map(|cap| {
                match cap.get(1) {
                    Some(m) => m.as_str(),
                    None =>cap.get(2).unwrap().as_str()
                }
            })
            .collect();
            if tokens.is_empty() { return Ok((line, ParamIniLine::Ignore)); }
            
            let r = match tokens[0] {
                "//" => ParamIniLine::Ignore,
                "data_ver" => ParamIniLine::DataVer(complete::u8(tokens[1])?.1),
                "defaultClientAvatarClass" => ParamIniLine::DefaultClientAvatarClass(tokens[1].to_owned()),
                "defaultPartyClass" => ParamIniLine::DefaultPartyClass(tokens[1].to_owned()),
                "defaultTradeClass" => ParamIniLine::DefaultTradeClass(tokens[1].to_owned()),
                "defaultMailClass" => ParamIniLine::DefaultMailClass(tokens[1].to_owned()),
                "defaultClanClass" => ParamIniLine::DefaultClanClass(tokens[1].to_owned()),
                "table" => ParamIniLine::Table(
                    complete::u8(tokens[1])?.1, 
                    tokens[2].to_owned()),
                "class" => ParamIniLine::Class(
                    tokens[1].to_owned(), 
                    tokens[2].to_owned(), 
                    tokens[3].to_owned()),
                "paramid" => ParamIniLine::ParamId(tokens[1].to_owned(), complete::u16(tokens[2])?.1),
                "help" => ParamIniLine::Ignore,
                _ => {
                    println!("Parsing line: {}", line_num);

                    let mut paramtype = None;
                    let mut default = None;
                    let mut flags = Vec::new();

                    let optionpairs: Vec<(&str, &str)> = tokens[1..].chunks_exact(2)
                        .map(|chunk| (chunk[0], chunk[1]))
                        .collect();

                    for (option, value) in optionpairs {
                        match option {
                            "type" => match value {
                                "Any" => paramtype = Some(ParamType::Any),
                                "AvatarID" => paramtype = Some(ParamType::AvatarID),
                                "AvatarID_set" => paramtype = Some(ParamType::AvatarIDSet),
                                "AvatarID_vector" => paramtype = Some(ParamType::AvatarIDVector),
                                "BitSetFilter" => paramtype = Some(ParamType::BitSetFilter),
                                "Bool" => paramtype = Some(ParamType::Bool),
                                "ClassRefPowerRangeList" => paramtype = Some(ParamType::ClassRefPowerRangeList),
                                "ContentRef" => paramtype = Some(ParamType::ContentRef),
                                "ContentRefAndInt" => paramtype = Some(ParamType::ContentRefAndInt),
                                "ContentRefList" => paramtype = Some(ParamType::ContentRefList),
                                "Float" => paramtype = Some(ParamType::Float),
                                "FloatRange" => paramtype = Some(ParamType::FloatRange),
                                "Float_vector" => paramtype = Some(ParamType::FloatVector),
                                "Guid" => paramtype = Some(ParamType::Guid),
                                "GuidPair" => paramtype = Some(ParamType::GuidPair),
                                "Int" => paramtype = Some(ParamType::Int),
                                "Int64" => paramtype = Some(ParamType::Int64),
                                "Int64_vector" => paramtype = Some(ParamType::Int64Vector),
                                "Int_vector" => paramtype = Some(ParamType::IntVector),
                                "JSON" => paramtype = Some(ParamType::JSON),
                                "LocalizedString" => paramtype = Some(ParamType::LocalizedString),
                                "String" => paramtype = Some(ParamType::String),
                                "StringFloatPair" => paramtype = Some(ParamType::StringFloatPair),
                                "String_String_hashmap" => paramtype = Some(ParamType::StringStringHashmap),
                                "String_int_hashmap" => paramtype = Some(ParamType::StringIntHashmap),
                                "String_vector" => paramtype = Some(ParamType::StringVector),
                                "Vector3" => paramtype = Some(ParamType::Vector3),
                                "oaInstanceGroup" => paramtype = Some(ParamType::OAInstanceGroup),
                                "oaSetGuid" => paramtype = Some(ParamType::OASetGuid),
                                "oaVectorGuid" => paramtype = Some(ParamType::OAVectorGuid),
                                "oaVectorLocalizedString" => paramtype = Some(ParamType::OAVactorLocalizedString),
                                _ => panic!("Unknown paramtype {}", value),
                            },
                            "flag" => match value {
                                "nodeOwn" => flags.push(ParamFlag::NodeOwn),
                                "serverOwn" => flags.push(ParamFlag::ServerOwn),
                                "clientOwn" => flags.push(ParamFlag::ClientOwn),
                                "persistent" => flags.push(ParamFlag::Persistent),
                                "clientUnknown" => flags.push(ParamFlag:: ClientUnknown),
                                "excludeFromClient" => flags.push(ParamFlag::ExcludeFromClient),
                                "content" => flags.push(ParamFlag::Content),
                                "perInstanceSetting" => flags.push(ParamFlag::PerInstanceSetting),
                                "dupeSetOk" => flags.push(ParamFlag::DupeSetOk),
                                "deprecated" => flags.push(ParamFlag::Deprecated),
                                "metric" => flags.push(ParamFlag::Metric),
                                "equipSlot" => flags.push(ParamFlag::EquipSlot),
                                "clientPrivileged" => flags.push(ParamFlag::ClientPrivileged),
                                "uts" => flags.push(ParamFlag::Uts),
                                "clientInit" => flags.push(ParamFlag::ClientInit),
                                _ => panic!("Unknown flag {}", value),
                            },
                            "default" => default = Some(value.to_owned()),
                            _ => println!("Skipped option {}", option),
                        }
                    }

                    ParamIniLine::ParamOptions(tokens[0].to_owned(), ParamOptions { 
                        param_type: paramtype.unwrap(), 
                        default, 
                        flags, 
                    })
                },
            };

            Ok((line, r))
        }).collect();

    let mut paramlist = Paramlist::default();
    let mut class_map: HashMap<String, Rc<RefCell<ParamClass>>> = HashMap::new();
    let mut current_class = None;

    for (line_number, line) in parsed_lines.iter().enumerate() {
        match line {
            Ok((_, ParamIniLine::DataVer(ver))) => paramlist.data_ver = *ver,
            Ok((_, ParamIniLine::DefaultClientAvatarClass(class))) => paramlist.default_client_avater_class = class.clone(),
            Ok((_, ParamIniLine::DefaultPartyClass(class))) => paramlist.default_party_class = class.clone(),
            Ok((_, ParamIniLine::DefaultTradeClass(class))) => paramlist.default_trade_class = class.clone(),
            Ok((_, ParamIniLine::DefaultMailClass(class))) => paramlist.default_mail_class = class.clone(),
            Ok((_, ParamIniLine::DefaultClanClass(class))) => paramlist.default_clan_class = class.clone(),
            Ok((_, ParamIniLine::Table(idx, name))) => paramlist.tables.push((*idx, name.clone())),
            Ok((_, ParamIniLine::Class(name, param, value))) => {
                if current_class.is_none() {
                    current_class = Some(ParamClass {
                        name: name.clone(),
                        ..Default::default()
                    });
                }

                if let Some(param_class) = &mut current_class {
                    if &param_class.name != name {
                        class_map.insert(param_class.name.to_owned(), Rc::new(RefCell::new(param_class.clone())));
                        *param_class = ParamClass {
                            name: name.clone(),
                            ..Default::default()
                        };
                    }

                    match param.as_str() {
                        "uniqueid" => param_class.unique_id = complete::u16::<&str, Error<&str>>(value.as_str()).expect("Number expected").1,
                        "bindsTo" => param_class.binds_to = value.trim_matches(['[', ']'].as_ref()).split(',').map(|v| v.to_owned()).collect(),
                        "contentTableBinding" => param_class.content_table_binding = value.clone(),
                        "extends" => param_class.extends = value.clone(),
                        "icon" => param_class.icon = value.clone(),
                        _ => panic!("Unknown parameter"),
                    }
                }
            },
            Ok((_, ParamIniLine::ParamId(name, id))) => {
                if current_class.is_some() {
                    class_map.insert(current_class.as_ref().unwrap().name.clone(), Rc::new(RefCell::new(current_class.as_ref().unwrap().clone())));
                    current_class = None;
                }

                 if let Some((class, attrib)) = name.split_once('.') {
                    if let Some(class_def) = class_map.get(class) {
                        class_def.borrow_mut().paramid.push((attrib.to_owned(), *id));
                    } else {
                        println!("Class {} not found. Line: {}", class, line_number + 1);
                    }
                } else {
                    panic!("Invalid attribute identifier");
                }
            },
            Ok((_, ParamIniLine::ParamOptions(name, options))) => {
                if current_class.is_some() {
                    class_map.insert(current_class.as_ref().unwrap().name.clone(), Rc::new(RefCell::new(current_class.as_ref().unwrap().clone())));
                    current_class = None;
                }

                if let Some((class, attrib)) = name.split_once('.') {
                    if let Some(class_def) = class_map.get(class) {
                        class_def.borrow_mut().paramoption.push((attrib.to_owned(), options.clone()));
                    } else {
                        println!("Class {} not found. Line: {}", class, line_number + 1);
                    }
                } else {
                    panic!("Invalid attribute identifier");
                }
            },
            Err(e) => {
                println!("Error in line {}", line_number + 1);
                println!("{}", e.to_string());
                panic!();
            }
            _ => (),
        }
    }

    paramlist.classes = class_map.values().map(|v| v.to_owned()).collect();

     // generate structs
     let classes_src: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        //let class_fields = generate_fields(v.to_owned(), &class_map).expect("Parameter generation failed");
        
        let params: Vec<_> = v.borrow().paramid.iter().map(|(name, id)| {
            (name.to_owned(), *id, v.borrow().paramoption.iter().find(|p| &p.0 == name).map(|s| s.1.to_owned()))
        }).collect();

        let getter_setter: Vec<_> = params.iter().map(|(name, id, options)| {
            let mut field_name = Converter::new()
            .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
            .set_pattern(Pattern::Lowercase)
            .set_delim("_")
            .convert(name);
            let param_id = *id;
    
            if field_name == "static" { 
                field_name = "r#static".to_owned();
            };
    
            if field_name == "type" { 
                field_name = "r#type".to_owned();
            };

            let field_name = format_ident!("{}", field_name);
            let set_field_name = format_ident!("set_{}", field_name);

            match options {
                Some(options) => {
                    let mut tokens = Vec::new();

                    if options.flags.contains(&ParamFlag::Deprecated) {
                        tokens.push(quote!(#[deprecated]));
                    }
                    match options.param_type {
                        ParamType::Any => tokens.push(quote! { 
                                pub fn #field_name<'a, T>(&'a self) -> Option<&T> {
                                    todo!()
                                }
                            }),
                        ParamType::AvatarID => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a AvatarId> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::AvatarIDSet => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a HashSet<AvatarId>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::AvatarIDVector => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<AvatarId>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::BitSetFilter => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a u32> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::Bool => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a bool> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::ClassRefPowerRangeList => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::ContentRef => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Uuid> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::ContentRefAndInt => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::ContentRefList => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::Float => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a f32> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::FloatRange => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a (f32, f32)> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::FloatVector => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<f32>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::Guid => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Uuid> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::GuidPair => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a (Uuid, Uuid)> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::Int64 => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a i64> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::Int64Vector => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<i64>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::IntVector => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<i32>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::JSON => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Value> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::LocalizedString => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Uuid> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::OAInstanceGroup => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::OASetGuid => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::OAVactorLocalizedString => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<()> {
                                todo!()
                            }
                        }),
                        ParamType::OAVectorGuid => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<Uuid>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::StringFloatPair => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a (String, f32)> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::StringIntHashmap => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a HashMap<String,i32>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::StringStringHashmap => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a HashMap<String, String>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::StringVector => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec<String>> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        ParamType::Vector3 => tokens.push(quote! { 
                            pub fn #field_name<'a>(&'a self) -> Option<&'a Vec3> {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        }),
                        _ => tokens.push(quote! { 
                            pub fn #field_name<'a, T>(&'a self) -> Option<&'a T> 
                                where &'a Param: TryInto<&'a T, Error = ParamError>, 
                            {
                                self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                            }
                        })
                    }

                    if options.flags.contains(&ParamFlag::NodeOwn) || options.flags.contains(&ParamFlag::ServerOwn) {
                        if options.flags.contains(&ParamFlag::Deprecated) {
                            tokens.push(quote!(#[deprecated]));
                        }
                        match options.param_type {
                            ParamType::Any => tokens.push(quote! { 
                                    pub fn #set_field_name<T>(&mut self, val: T) {
                                        todo!()
                                    }
                                }),
                            ParamType::AvatarID => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: AvatarId) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::AvatarIDSet => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: HashSet<AvatarId>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::AvatarIDVector => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<AvatarId>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::BitSetFilter => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: u32) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::Bool => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: bool) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::ClassRefPowerRangeList => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::ContentRef => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Uuid) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::ContentRefAndInt => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::ContentRefList => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::Float => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: f32) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::FloatRange => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: (f32, f32)) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::FloatVector => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<f32>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::Guid => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Uuid) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::GuidPair => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: (Uuid, Uuid)) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::Int64 => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: i64) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::Int64Vector => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<i64>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::IntVector => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<i32>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::JSON => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Value) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::LocalizedString => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<Uuid>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::OAInstanceGroup => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::OASetGuid => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::OAVactorLocalizedString => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: ()) {
                                    todo!()
                                }
                            }),
                            ParamType::OAVectorGuid => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<Uuid>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::StringFloatPair => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: (String, f32)) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::StringIntHashmap => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: HashMap<String, i32>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::StringStringHashmap => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: HashMap<String, String>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::StringVector => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec<String>) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            ParamType::Vector3 => tokens.push(quote! { 
                                pub fn #set_field_name(&mut self, val: Vec3) {
                                    self.0.set_param(#param_id, val.into())
                                }
                            }),
                            _ => tokens.push(quote! { 
                                pub fn #set_field_name<T>(&mut self, val: T) where T: Into<Param> {
                                    self.0.set_param(#param_id, val.into())
                                }
                            })
                        }
                    }

                    quote! {
                        #(#tokens)*
                    }
                },
                None => {
                    quote! { 
                        pub fn #field_name<'a, T>(&'a self) -> Option<&'a T> 
                            where &'a Param: TryInto<&'a T, Error = ParamError>,
                        {
                            self.0.get_param(#param_id).map(|v| v.try_into().ok()).flatten()
                        }

                        pub fn #set_field_name<T>(&mut self, val: T) where T: Into<Param> {
                            self.0.set_param(#param_id, val.into())
                        }
                    }
                }
            }
        }).collect();

        let attrib_names: Vec<_> = params.iter().map(|(name, id, options)| {
            let literal = format!("{}", name);
            let id = *id;

            quote!{ #id => #literal, }
        }).collect();

        /*let attrib_is_persistent: Vec<_> = params.iter().map(|(name, id, options)| {
            let literal = format!("{}", name);
            let id = *id;

            match options {
                Some(options) => {
                    if options.flags.contains(&ParamFlag::Persistent) {
                        quote!{ #id => true, }
                    } else {
                        quote!{ #id => false, }
                    }
                }
                None => quote!{ #id => true, }
            }
        }).collect();*/

        let attrib_lookup: Vec<_> = params.iter().map(|(name, id, options)| {
            let literal = format!("{}", name);
            let id = *id;

            quote!{ #literal => Some(#id), }
        }).collect();

        let attrib_flags: Vec<_> = params.iter().map(|(name, id, options)| {
            let literal = format!("{}", name);
            let id = *id;

            let flags = match options {
                Some(options) => {
                    let flag_idents: Vec<_> = options.flags.iter().map(|f| {
                        match f {
                            ParamFlag::NodeOwn => quote!(ParamFlag::NodeOwn),
                            ParamFlag::ServerOwn => quote!(ParamFlag::ServerOwn),
                            ParamFlag::ClientOwn => quote!(ParamFlag::ClientOwn),
                            ParamFlag::ClientUnknown => quote!(ParamFlag::ClientUnknown),
                            ParamFlag::ClientPrivileged =>quote!(ParamFlag::ClientPrivileged),
                            ParamFlag::ClientInit => quote!(ParamFlag::ClientInit),
                            ParamFlag::Persistent => quote!(ParamFlag::Persistent),
                            ParamFlag::ExcludeFromClient => quote!(ParamFlag::ExcludeFromClient),
                            ParamFlag::Content => quote!(ParamFlag::Content),
                            ParamFlag::PerInstanceSetting => quote!(ParamFlag::PerInstanceSetting),
                            ParamFlag::DupeSetOk => quote!(ParamFlag::DupeSetOk),
                            ParamFlag::Deprecated => quote!(ParamFlag::Deprecated),
                            ParamFlag::Metric => quote!(ParamFlag::Metric),
                            ParamFlag::EquipSlot => quote!(ParamFlag::EquipSlot),
                            ParamFlag::Uts => quote!(ParamFlag::Uts),
                        }
                    }).collect();

                    quote!{&[#(#flag_idents),*]}
                },
                None => quote!{&[]},
            };

            quote!{ #id => #flags, }
        }).collect();

        quote! {
            #[derive(Clone)]
            pub struct #class_name(AnyClass);

            impl #class_name {
                #(#getter_setter)*
            }

            impl Default for #class_name {
                fn default() -> Self {
                    Self(AnyClass::new())
                }
            }

            impl BoundParamClass for #class_name {
                const CLASS_ID: ParamClassId = ParamClassId::#unprefixed_class_name;

                fn attribute_name(id: u16) -> &'static str {
                    match id {
                        #(#attrib_names)*
                        _ => panic!(),
                    }
                }

                /*fn attribute_is_persistent(id: u16) -> bool {
                    match id {
                        #(#attrib_is_persistent)*
                        _ => panic!(),
                    }
                }*/

                fn lookup_field(name: &str) -> Option<u16> {
                    match name {
                        #(#attrib_lookup)*
                        _ => None,
                    }
                }
            }

            impl ParamClass for #class_name {
                fn as_anyclass(&self) -> &AnyClass { &self.0 }
                fn as_anyclass_mut(&mut self) -> &mut AnyClass { &mut self.0 }
                fn to_anyclass(self) -> AnyClass { self.0 }
                fn from_anyclass(anyclass: AnyClass) -> Self { 
                    Self(anyclass)
                }

                fn attribute_flags(&self, id: u16) -> &'static [ParamFlag] {
                    match id {
                        #(#attrib_flags)*
                        _ => panic!(),
                    }
                }
            }

            impl Serialize for #class_name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let json = self.clone().into_persistent_json();
                    json.serialize(s)
                }
            }
    
            impl <'de>Deserialize<'de> for #class_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                {
                    let json = Value::deserialize(deserializer)?;
                    Ok(#class_name::from_json(&json).unwrap())
                }
            }

            impl <'a>TryFrom<&'a ParamClassContainer> for &'a #class_name {
                type Error = ParamError;

                fn try_from(value: &'a ParamClassContainer) -> Result<Self, Self::Error> {
                    match value {
                        ParamClassContainer::#unprefixed_class_name(val) => Ok(val),
                        _ => Err(ParamError(()))
                    }
                }
            }

            impl TryFrom<ParamClassContainer> for #class_name {
                type Error = ParamError;

                fn try_from(value: ParamClassContainer) -> Result<Self, Self::Error> {
                    match value {
                        ParamClassContainer::#unprefixed_class_name(val) => Ok(val),
                        _ => Err(ParamError(()))
                    }
                }
            }
        }
    }).collect();

    let unique_id_enum: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        quote!(#class_name)
    }).collect();

    let unique_id_from: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_id = v.borrow().unique_id;

        quote!(#class_id => Ok(ParamClassId::#class_name))
    }).collect();

    let unique_id_into: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_id = v.borrow().unique_id;

        quote!(ParamClassId::#class_name => #class_id )
    }).collect();

    let class_container_entries: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));

        quote! { #unprefixed_class_name(#class_name) }
    }).collect();

    let class_container_id: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));

        quote! { ParamClassContainer::#unprefixed_class_name(_) => #class_name::class_id() }
    }).collect();

    let class_container_read: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_id = v.borrow().unique_id;

        quote! { 
            #class_id => {
                let (i, class) = #class_name::read(i)?;
                Ok((i, ParamClassContainer::#unprefixed_class_name(class)))
            },
        }
    }).collect();

    let class_container_write: Vec<_> = paramlist.classes.iter().map(|v| {
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));

        quote! { ParamClassContainer::#unprefixed_class_name(class) => class.write(writer)? }
    }).collect();

    let class_container_to_persistent_json: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_name_literal = v.borrow().name.to_owned();

        quote! { ParamClassContainer::#unprefixed_class_name(class) => (#class_name_literal, class.into_persistent_json()), }
    }).collect();

    let class_container_from_json: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_name_literal = v.borrow().name.to_owned();

        quote! { #class_name_literal => 
            Ok(ParamClassContainer::#unprefixed_class_name(#class_name::from_json(value)?)), }
    }).collect();

    let class_container_strip_original_data: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("{}Param", formatted_class_name(&v.borrow().name));
        let unprefixed_class_name = format_ident!("{}", formatted_class_name(&v.borrow().name));
        let class_name_literal = v.borrow().name.to_owned();

        quote! { ParamClassContainer::#unprefixed_class_name(class) => class.as_anyclass_mut().strip_original_data() }
    }).collect();

    write_source("generated_params.rs", quote! {
        pub enum ParamClassId {
            #(#unique_id_enum),*
        }

        #[derive(Clone)]
        pub enum ParamClassContainer {
            #(#class_container_entries),*
        }

        impl ParamClassContainer {
            pub fn class_id(&self) -> ParamClassId {
                match self {
                    #(#class_container_id),*
                }
            }

            pub fn read<'a>(class_id: u16, i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
                match class_id {
                    #(#class_container_read)*
                    _ => panic!("Unknown class id {}", class_id)
                }
            }

            pub fn write<T>(&self, writer: &mut T) -> Result<(), io::Error> 
            where T: ByteWrite
            {
                match self {
                    #(#class_container_write),*
                }

                Ok(())
            }

            pub fn into_persistent_json(&self) -> Value {
                let (class_name, values) = match self {
                    #(#class_container_to_persistent_json)*
                };

                json!({ class_name: values })
            }

            pub fn from_json(value: &Value) -> Result<Self, io::Error> {
                let (class_name, value) = value.as_object()
                    .map(|v| v.iter().next())
                    .flatten()
                    .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;

                match class_name.as_str() {
                    #(#class_container_from_json)*
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, ""))
                }
            }

            pub fn strip_original_data(&mut self) {
                match self {
                    #(#class_container_strip_original_data),*
                }
            }
        }

        impl Serialize for ParamClassContainer {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let json = self.clone().into_persistent_json();
                json.serialize(s)
            }
        }

        impl <'de>Deserialize<'de> for ParamClassContainer {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
            {
                let json = Value::deserialize(deserializer)?;
                Ok(ParamClassContainer::from_json(&json).unwrap())
            }
        }

        impl ParamClassId {
            pub fn as_u16(&self) -> u16 {
                self.into()
            }

            pub fn as_u32(&self) -> u32 {
                self.as_u16() as u32
            }
        }

        impl TryFrom<u16> for ParamClassId {
            type Error = ParamError;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value.into() {
                    #(#unique_id_from),*,
                    _ => Err(ParamError(())),
                }
            }
        }

        impl Into<u16> for &ParamClassId {
            fn into(self) -> u16 {
                match self {
                    #(#unique_id_into),*
                }
            }
        }

        #(#classes_src)*
    })
}

fn formatted_class_name(name: &str) -> String {
    name.to_case(Case::UpperCamel)
}