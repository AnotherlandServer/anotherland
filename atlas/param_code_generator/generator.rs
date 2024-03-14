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

use std::{path::Path, io, fs, collections::HashMap, rc::Rc, cell::RefCell};

use nom::{IResult, character::complete, error::Error};
use proc_macro2::{Group, TokenStream};
use quote::ToTokens;
use quote::{format_ident, TokenStreamExt};
use regex::Regex;
use ::quote::quote;
use uuid::Uuid;
use proc_macro2::Punct;
use proc_macro2::Spacing;

use convert_case::{Converter, Boundary, Pattern, Case, Casing};

use crate::write_source;

static ADDITIONAL_PARAM_CLASSES: &[&str] = &[
    "structure",
    "InteractObject",
    "NonSpawnPlacement",
    "itemEdna",
    "oaCommonConfig",
    "physicsActor",
];

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
    extends_ref: Option<Rc<RefCell<ParamClass>>>,
    final_class: bool,
    binds_to: Vec<String>,
    content_table_binding: String,
    icon: String,
    paramid: Vec<(String, u16)>,
    paramoption: Vec<(String, ParamOptions)>
}

impl ParamClass {
    fn has_param(&self, param: &str) -> bool {
        self.paramid.iter().any(|(name, _)| name == param)
    }

    fn param_is_owned(&self, param: &str) -> bool {
        if let Some(extends_ref) = &self.extends_ref {
            !extends_ref.borrow().has_param(param)
        } else {
            self.has_param(param)
        }
    }
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

#[derive(Debug, Clone, Copy)]
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
    Json,
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

impl ParamType {
    fn into_rust_type(self, flags: &[ParamFlag]) -> TokenStream {
        match self {
            ParamType::Any => quote!(Param),
            ParamType::AvatarID => quote!(AvatarId),
            ParamType::AvatarIDSet => quote!(HashSet<AvatarId>),
            ParamType::AvatarIDVector => quote!([AvatarId]),
            ParamType::BitSetFilter => quote!(u32),
            ParamType::Bool => quote!(bool),
            ParamType::ClassRefPowerRangeList => quote!(str),
            ParamType::ContentRef => quote!(str),
            ParamType::ContentRefAndInt => quote!(str),
            ParamType::ContentRefList => quote!(str),
            ParamType::Float => quote!(f32),
            ParamType::FloatRange => quote!((f32, f32)),
            ParamType::FloatVector => quote!([f32]),
            ParamType::Guid => quote!(Uuid),
            ParamType::GuidPair => quote!((Uuid, Uuid)),
            ParamType::Int => quote!(i32),
            ParamType::Int64 => quote!(i64),
            ParamType::Int64Vector => quote!([i64]),
            ParamType::IntVector => quote!([i32]),
            ParamType::Json => quote!(Value),
            ParamType::LocalizedString => quote!(Uuid),
            ParamType::OAInstanceGroup => quote!(str),
            ParamType::OASetGuid => quote!(HashSet<Uuid>),
            ParamType::OAVactorLocalizedString => quote!([Uuid]),
            ParamType::OAVectorGuid => quote!([Uuid]),
            ParamType::String => quote!(str),
            ParamType::StringFloatPair => quote!((String, f32)),
            ParamType::StringIntHashmap => quote!(HashMap<String, i32>),
            ParamType::StringStringHashmap => quote!(HashMap<String, String>),
            ParamType::StringVector => quote!([String]),
            ParamType::Vector3 => if flags.contains(&ParamFlag::Uts) {
                quote!((u32, Vec3))
            } else {
                quote!(Vec3)
            },
        }
    }

    fn is_any_type(self) -> bool {
        matches!(self, ParamType::Any)
    }

    fn is_copy_type(self) -> bool {
        matches!(self, ParamType::AvatarID | 
            ParamType::Bool |
            ParamType::Float |
            ParamType::Int |
            ParamType::Int64)
    }

    fn is_ref_type(self) -> bool {
        matches!(self, ParamType::AvatarIDSet |
            ParamType::AvatarIDVector |
            ParamType::ContentRef |
            ParamType::ContentRefAndInt |
            ParamType::ContentRefList |
            ParamType::ClassRefPowerRangeList |
            ParamType::FloatRange |
            ParamType::FloatVector |
            ParamType::Guid |
            ParamType::GuidPair |
            ParamType::Int64Vector |
            ParamType::IntVector |
            ParamType::Json |
            ParamType::LocalizedString |
            ParamType::OAVectorGuid |
            ParamType::String |
            ParamType::StringFloatPair |
            ParamType::StringIntHashmap |
            ParamType::StringStringHashmap |
            ParamType::StringVector |
            ParamType::Vector3
        )
    }
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
#[allow(dead_code)]
struct ParamOptions {
    param_type: ParamType,
    default: Option<String>,
    default_literal: Option<TokenStream>,
    flags: Vec<ParamFlag>,
}

trait ArrayToTokenStream {
    fn to_token_stream(&self) -> TokenStream;
}

impl ArrayToTokenStream for &[u8; 16] {
    fn to_token_stream(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        stream.append_separated(self.iter(), Punct::new(',', Spacing::Joint));
        Group::new(proc_macro2::Delimiter::Bracket, stream).into_token_stream()
    }
}

pub fn generate_param_code(client_path: &Path) -> io::Result<()> {
    let paramlist_path = client_path.join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/paramlist.ini");

    let file_content = {
        let data: Vec<u16> = fs::read(paramlist_path)?
            .chunks_exact(2)
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
                                "JSON" => paramtype = Some(ParamType::Json),
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

                    let default_literal = if let Some(default_str) = default.as_ref() {
                        match paramtype.as_ref().unwrap() {
                            ParamType::Any => None,
                            ParamType::AvatarID => {
                                let val: u64 = default_str.strip_prefix('#').unwrap().parse().expect("failed to parse avatar id");
                                Some(quote! { Param::AvatarId(AvatarId::from_u64_literal(#val)) })
                            },
                            ParamType::AvatarIDSet => None,
                            ParamType::AvatarIDVector => None,
                            ParamType::BitSetFilter => None,
                            ParamType::Bool => if default_str == "true" { Some(quote!{ Param::Bool(true) }) } else { Some(quote!{ Param::Bool(false) }) },
                            ParamType::ClassRefPowerRangeList => Some(quote! { Param::StaticClassRefPowerRangeList(#default_str) }),
                            ParamType::ContentRef => Some(quote! { Param::StaticContentRef(#default_str) }),
                            ParamType::ContentRefAndInt => Some(quote! { Param::StaticContentRefAndInt(#default_str) }),
                            ParamType::ContentRefList => Some(quote! { Param::StaticContentRefList(#default_str) }),
                            ParamType::Float => {
                                let val: f32 = default_str.parse().expect("failed to parse float");
                                Some(quote! { Param::Float(#val) })
                            },
                            ParamType::FloatRange => None,
                            ParamType::FloatVector => None,
                            ParamType::Guid => {
                                let uuid_bytes = Uuid::parse_str(default_str).unwrap().as_bytes().to_token_stream();
                                Some(quote!{ Param::Guid(Uuid::from_bytes(#uuid_bytes)) })
                            },
                            ParamType::GuidPair => None,
                            ParamType::Int => {
                                let val: i32 = default_str.parse().expect("failed to parse int");
                                Some(quote! { Param::Int(#val) })
                            },
                            ParamType::Int64 => {
                                let val: i64 = default_str.parse().expect("failed to parse int64");
                                Some(quote! { Param::Int64(#val) })
                            },
                            ParamType::Int64Vector => None,
                            ParamType::IntVector => None,
                            ParamType::Json => {
                                println!("Parse: {}", default_str);
                                let default_str = default_str
                                    .replace("\\\"", "\"")
                                    .replace("\\\\", "\\");
                                Some(quote! { Param::JsonValue(serde_json::from_str(#default_str).unwrap()) })
                            },
                            ParamType::LocalizedString => {
                                let uuid_bytes = Uuid::parse_str(default_str).unwrap().as_bytes().to_token_stream();
                                Some(quote! { Param::LocalizedString(Uuid::from_bytes(#uuid_bytes)) })
                            },
                            ParamType::String => Some(quote! { Param::StaticString(#default_str) }),
                            ParamType::StringFloatPair => None,
                            ParamType::StringStringHashmap => None,
                            ParamType::StringIntHashmap => None,
                            ParamType::StringVector => None,
                            ParamType::Vector3 => {
                                let parts: Vec<_> = default_str.split(' ').collect();
                                let x: f32 = parts[0].parse().expect("failed to parse vector3");
                                let y: f32 = parts[1].parse().expect("failed to parse vector3");
                                let z: f32 = parts[2].parse().expect("failed to parse vector3");


                                if flags.contains(&ParamFlag::Uts) {
                                    Some(quote! { Param::Vector3Uts((0, Vec3::new(#x, #y, #z))) })
                                } else {
                                    Some(quote! { Param::Vector3(Vec3::new(#x, #y, #z)) })
                                }
                            },
                            ParamType::OAInstanceGroup => None,
                            ParamType::OASetGuid => None,
                            ParamType::OAVectorGuid => None,
                            ParamType::OAVactorLocalizedString => None,
                        }
                    } else {
                        None
                    };

                    ParamIniLine::ParamOptions(tokens[0].to_owned(), ParamOptions { 
                        param_type: paramtype.unwrap(), 
                        default, 
                        default_literal,
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
                        final_class: true,
                        ..Default::default()
                    });
                }

                if let Some(param_class) = &mut current_class {
                    if &param_class.name != name {
                        class_map.insert(param_class.name.to_owned(), Rc::new(RefCell::new(param_class.clone())));
                        *param_class = ParamClass {
                            name: name.clone(),
                            final_class: true,
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
                println!("{}", e);
                panic!();
            }
            _ => (),
        }
    }

    // resolve references
    for (_, class) in class_map.iter() {
        if !class.borrow().extends.is_empty() {
            let extend_class = class_map.get(&class.borrow().extends).cloned().unwrap();
            extend_class.borrow_mut().final_class = false;
            class.borrow_mut().extends_ref = Some(extend_class);
        }
    }

    // inherit types and defaults from parent classes
    for (_, class) in class_map.iter() {
        let mut class = class.borrow_mut();

        let mut options = Vec::new();

        for (param, _) in class.paramid.iter() {
            // check if we have options for this param
            let mut has_option = false;
            
            for (param_b_name, _) in class.paramoption.iter() {
                if param_b_name == param {
                    has_option = true;
                    break;
                }
            };

            // if not, search in parents for options
            if !has_option {
                let mut current_class = class.extends_ref.clone();
                'parent_search: while let Some(parent) = current_class.as_ref() {
                    let parent = parent.borrow();

                    for (param_b_name, option) in parent.paramoption.iter() {
                        if param_b_name == param {
                            options.push((param.clone(), option.clone()));
                            break 'parent_search;
                        }
                    };

                    // move to next parent
                    let parent_ref = parent.extends_ref.clone();
                    drop(parent);
                    current_class = parent_ref;
                }
            }
        }

        class.paramoption.extend(options);
    }

    paramlist.classes = class_map.values().map(|v| v.to_owned()).collect();

    // search for all final param classes. Meaning ones that are not extended
    let final_classes: Vec<_> = paramlist.classes.iter().filter(|v| { 
        v.borrow().final_class || ADDITIONAL_PARAM_CLASSES.contains(&v.borrow().name.as_str())
    }).collect();

    // generate enum for fields
    let param_name_enums: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();

        let enum_name = format_ident!("{}Attribute", class.name.to_case(Case::UpperCamel));
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));

        let enum_entries: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));

            quote!(#entry_name)
        }).collect();

        let enum_ids: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let id_literal = v.1;

            quote!(Self::#entry_name => #id_literal)
        }).collect();

        let enum_names: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let name_literal = &v.0;

            quote!(Self::#entry_name => #name_literal)
        }).collect();

        let enum_non_option = if class.paramid.len() != class.paramoption.len() {
            quote!{_ => None}
        } else {
            quote!()
        };

        let enum_types: Vec<_> = class.paramoption.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let type_ident = match v.1.param_type {
                ParamType::Any => quote!(ParamType::Any),
                ParamType::AvatarID => quote!(ParamType::AvatarId),
                ParamType::AvatarIDSet => quote!(ParamType::AvatarIdSet),
                ParamType::AvatarIDVector => quote!(ParamType::VectorAvatarId),
                ParamType::BitSetFilter => quote!(ParamType::BitSetFilter),
                ParamType::Bool => quote!(ParamType::Bool),
                ParamType::ClassRefPowerRangeList => quote!(ParamType::ClassRefPowerRangeList), 
                ParamType::ContentRef => quote!(ParamType::ContentRef),
                ParamType::ContentRefAndInt => quote!(ParamType::ContentRefAndInt), 
                ParamType::ContentRefList => quote!(ParamType::ContentRefList),
                ParamType::Float => quote!(ParamType::Float),
                ParamType::FloatRange => quote!(ParamType::FloatRange),
                ParamType::FloatVector => quote!(ParamType::VectorFloat),
                ParamType::Guid => quote!(ParamType::Guid),
                ParamType::GuidPair => quote!(ParamType::GuidPair),
                ParamType::Int => quote!(ParamType::Int),
                ParamType::Int64 => quote!(ParamType::Int64),
                ParamType::Int64Vector => quote!(ParamType::VectorInt64),
                ParamType::IntVector => quote!(ParamType::VectorInt),
                ParamType::Json => quote!(ParamType::JsonValue),
                ParamType::LocalizedString => quote!(ParamType::LocalizedString),
                ParamType::String => quote!(ParamType::String),
                ParamType::StringFloatPair => quote!(ParamType::StringFloatPair),
                ParamType::StringStringHashmap => quote!(ParamType::HashmapStringString),
                ParamType::StringIntHashmap => quote!(ParamType::HashmapStringInt),
                ParamType::StringVector => quote!(ParamType::VectorString),
                ParamType::Vector3 => quote!(ParamType::Vector3),
                ParamType::OAInstanceGroup => quote!(ParamType::InstanceGroup),
                ParamType::OASetGuid => quote!(ParamType::GuidSet),
                ParamType::OAVectorGuid => quote!(ParamType::VectorGuid),
                ParamType::OAVactorLocalizedString => quote!(ParamType::VectorLocalizedString),
            };

            quote!(Self::#entry_name => #type_ident,)
        }).collect();

        let static_defaults: Vec<_> = class.paramoption.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperSnake));
            if let Some(default_literal) = v.1.default_literal.as_ref() {
                if matches!(v.1.param_type, ParamType::Json) {
                    quote!{static #entry_name: Lazy<Param> = Lazy::new(|| #default_literal);}
                } else {
                    quote!{static #entry_name: Param = #default_literal;}
                }
            } else {
                quote!()
            }
        }).collect();

        let enum_defaults: Vec<_> = class.paramoption.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let static_name = format_ident!("{}", v.0.to_case(Case::UpperSnake));
            if let Some(default_literal) = v.1.default_literal.as_ref() {
                quote!{Self::#entry_name => Some(&#static_name),}
            } else {
                quote!{Self::#entry_name => None,}
            }
        }).collect();

        let enum_flags: Vec<_> = class.paramoption.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let flag_idents = v.1.flags.iter().map(|v| {
                match v {
                    ParamFlag::NodeOwn => quote!(ParamFlag::NodeOwn),
                    ParamFlag::ServerOwn => quote!(ParamFlag::ServerOwn),
                    ParamFlag::ClientOwn => quote!(ParamFlag::ClientOwn),
                    ParamFlag::ClientUnknown => quote!(ParamFlag::ClientUnknown),
                    ParamFlag::ClientPrivileged => quote!(ParamFlag::ClientPrivileged),
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
            });

            quote!{Self::#entry_name => &[#(#flag_idents),*],}
        }).collect();

        let enum_from_str: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let name_literal = &v.0;

            quote!(#name_literal => Ok(Self::#entry_name),)
        }).collect();

        let enum_from_u16: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let id_literal = &v.1;

            quote!(#id_literal => Ok(Self::#entry_name),)
        }).collect();

        quote!{
            #[derive(PartialEq, Eq, Hash, Clone)]
            pub enum #enum_name {
                #(#enum_entries),*
            }

            impl ParamAttrib for #enum_name {
                fn class_id() -> ClassId { ClassId::#class_name }

                fn id(&self) -> u16 { 
                    match self {
                        #(#enum_ids),*
                    }
                }

                fn name(&self) -> &'static str { 
                    match self {
                        #(#enum_names),*
                    } 
                }

                fn datatype(&self) -> ParamType { 
                    match self {
                        #(#enum_types)*
                        #enum_non_option
                    }
                }

                fn default(&self) -> Option<&Param> { 
                    #(#static_defaults)*

                    match self {
                        #(#enum_defaults)*
                        #enum_non_option
                    }
                }

                fn flags(&self) -> &[ParamFlag] {
                    match self {
                        #(#enum_flags)*
                        #enum_non_option
                    }
                }
            }

            impl FromStr for #enum_name {
                type Err = ParamError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(#enum_from_str)*
                        _ => Err(ParamError(())),
                    }
                }
            }

            impl TryFrom<u16> for #enum_name {
                type Error = ParamError;

                fn try_from(val: u16) -> Result<Self, Self::Error> {
                    match val {
                        #(#enum_from_u16)*
                        _ => Err(ParamError(())),
                    }
                }
            }
        }
    }).collect();

    // generate views & components
    let param_view_traits: Vec<_> = paramlist.classes.iter().map(|v| {
        let class: std::cell::Ref<'_, ParamClass> = v.borrow();

        let trait_name = format_ident!("{}Params", class.name.to_case(Case::UpperCamel));
        let view_name = format_ident!("{}View", class.name.to_case(Case::UpperCamel));
        let component_name = format_ident!("{}Component", class.name.to_case(Case::UpperCamel));

        let params: Vec<_> = v.borrow().paramid.iter().map(|(name, id)| {
            (name.to_owned(), *id, v.borrow().paramoption.iter().find(|p| &p.0 == name).map(|s| s.1.to_owned()))
        }).collect();

        let getter_setter: Vec<_> = params.iter()
        .filter(|(name, _, _)| v.borrow().param_is_owned(name))
        .map(|(name, _, options)| {
            let field_name = Converter::new()
            .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
            .set_pattern(Pattern::Lowercase)
            .set_delim("_")
            .convert(name.clone());

            let field_name_ident = format_ident!("{}", match field_name.as_str() {
                "static" => "r#static",
                "type" => "r#type",
                _ => field_name.as_str(),
            });
            let set_field_name = format_ident!("set_{}", field_name);

            match options {
                Some(options) => {
                    let mut tokens = Vec::new();

                    if options.flags.contains(&ParamFlag::Deprecated) {
                        tokens.push(quote!(#[deprecated]));
                    }

                    let rust_type = options.param_type.into_rust_type(&options.flags);

                    if options.param_type.is_any_type() {
                        tokens.push(quote! { 
                            fn #field_name_ident(&self) -> Option<MappedRwLockReadGuard<#rust_type>>;
                        });
                    } else if options.param_type.is_copy_type() {
                        if options.default.is_some() {
                            tokens.push(quote! { 
                                fn #field_name_ident(&self) -> #rust_type;
                            });
                        } else {
                            tokens.push(quote! { 
                                fn #field_name_ident(&self) -> Option<#rust_type>;
                            });
                        }
                    } else if options.param_type.is_ref_type() {
                        if options.default.is_some() {
                            tokens.push(quote! { 
                                fn #field_name_ident(&self) -> MappedRwLockReadGuard<#rust_type>;
                            });
                        } else {
                            tokens.push(quote! { 
                                fn #field_name_ident(&self) -> Option<MappedRwLockReadGuard<#rust_type>>;
                            });
                        }
                    } else {
                        tokens.push(quote! { 
                            fn #field_name_ident(&self) -> Option<()>;
                        });
                    }

                    //if options.flags.contains(&ParamFlag::NodeOwn) || options.flags.contains(&ParamFlag::ServerOwn) {
                    if options.flags.contains(&ParamFlag::Deprecated) {
                        tokens.push(quote!(#[deprecated]));
                    }
                    match options.param_type {
                        ParamType::Any => tokens.push(quote! { 
                                fn #set_field_name(&mut self, val: Param);
                            }),
                        ParamType::AvatarID => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: AvatarId);
                        }),
                        ParamType::AvatarIDSet => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: HashSet<AvatarId>);
                        }),
                        ParamType::AvatarIDVector => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<AvatarId>);
                        }),
                        ParamType::BitSetFilter => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: u32);
                        }),
                        ParamType::Bool => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: bool);
                        }),
                        ParamType::ClassRefPowerRangeList => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: &str);
                        }),
                        ParamType::ContentRef => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: &str);
                        }),
                        ParamType::ContentRefAndInt => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: &str);
                        }),
                        ParamType::ContentRefList => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: &str);
                        }),
                        ParamType::Float => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: f32);
                        }),
                        ParamType::FloatRange => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: (f32, f32));
                        }),
                        ParamType::FloatVector => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<f32>);
                        }),
                        ParamType::Guid => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Uuid);
                        }),
                        ParamType::GuidPair => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: (Uuid, Uuid));
                        }),
                        ParamType::Int => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: i32);
                        }),
                        ParamType::Int64 => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: i64);
                        }),
                        ParamType::Int64Vector => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<i64>);
                        }),
                        ParamType::IntVector => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<i32>);
                        }),
                        ParamType::Json => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Value);
                        }),
                        ParamType::LocalizedString => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Uuid);
                        }),
                        ParamType::OAInstanceGroup => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: &str);
                        }),
                        ParamType::OASetGuid => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: HashSet<Uuid>);
                        }),
                        ParamType::OAVactorLocalizedString => tokens.push(quote! { 
                            fn #set_field_name(&mut self, _val: Vec<Uuid>);
                        }),
                        ParamType::OAVectorGuid => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<Uuid>);
                        }),
                        ParamType::String => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: &str);
                        }),
                        ParamType::StringFloatPair => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: (String, f32));
                        }),
                        ParamType::StringIntHashmap => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: HashMap<String, i32>);
                        }),
                        ParamType::StringStringHashmap => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: HashMap<String, String>);
                        }),
                        ParamType::StringVector => tokens.push(quote! { 
                            fn #set_field_name(&mut self, val: Vec<String>);
                        }),
                        ParamType::Vector3 => {
                            if options.flags.contains(&ParamFlag::Uts) {
                                tokens.push(quote! {
                                    fn #set_field_name(&mut self, val: (u32, Vec3));
                                })
                            } else {
                                tokens.push(quote! {
                                    fn #set_field_name(&mut self, val: Vec3);
                                })
                            }
                        },
                    }

                    quote! {
                        #(#tokens)*
                    }
                },
                None => {
                    panic!("No type information for owned attribute!");
                }
            }
        }).collect();

        quote! {
            pub trait #trait_name {
                #(#getter_setter)*
            }

            pub struct #view_name<P>(P) where P: ParamClass + #trait_name;

            impl <P: ParamClass + #trait_name> #view_name<P> {
                pub fn new(params: &P) -> Self 
                where P: ParamClass + #trait_name
                {
                    Self(params.clone_ref())
                }
            }

            impl <P: ParamClass + #trait_name> Deref for #view_name<P> {
                type Target = dyn #trait_name;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            #[derive(Component)]
            pub struct #component_name(Box<dyn #trait_name>);

            unsafe impl Send for #component_name {}
            unsafe impl Sync for #component_name {}

            impl Deref for #component_name {
                type Target = dyn #trait_name;

                fn deref(&self) -> &Self::Target {
                    self.0.as_ref()
                }
            }

            impl DerefMut for #component_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.0.as_mut()
                }
            }
        }
    }).collect();

    // generate enum for fields
    let param_classes: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();

        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));
        let attrib_name = format_ident!("{}Attribute", class.name.to_case(Case::UpperCamel));

        // generate trait implementations
        let trait_implementations = {
            let mut traits = Vec::new();

            let mut current_class = Some(v.to_owned().to_owned());
            while let Some(parent_class_ref) = current_class {
                current_class = parent_class_ref.borrow().extends_ref.clone();

                let view_name = format_ident!("{}Params", parent_class_ref.borrow().name.to_case(Case::UpperCamel));

                let params: Vec<_> = parent_class_ref.borrow().paramid.iter().map(|(name, id)| {
                    (name.to_owned(), *id, parent_class_ref.borrow().paramoption.iter().find(|p| &p.0 == name).map(|s| s.1.to_owned()))
                }).collect();
    
                let getter_setter: Vec<_> = params.iter()
                .filter(|(name, _, _)| parent_class_ref.borrow().param_is_owned(name))
                .map(|(name, _, options)| {
                    let field_name = Converter::new()
                    .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
                    .set_pattern(Pattern::Lowercase)
                    .set_delim("_")
                    .convert(name.clone());
    
                    let field_name_ident = format_ident!("{}", match field_name.as_str() {
                        "static" => "r#static",
                        "type" => "r#type",
                        _ => field_name.as_str(),
                    });
                    let set_field_name = format_ident!("set_{}", field_name);

                    let attrib_enum_name = format_ident!("{}", name.to_case(Case::UpperCamel));
    
                    match options {
                        Some(options) => {
                            let mut tokens = Vec::new();

                            let rust_type = options.param_type.into_rust_type(&options.flags);

                            if options.param_type.is_any_type() {
                                tokens.push(quote! { 
                                    fn #field_name_ident(&self) -> Option<MappedRwLockReadGuard<#rust_type>> {
                                        RwLockReadGuard::try_map::<Param, _>(self.0.read(), 
                                        |v| 
                                            v.get(&#attrib_name::#attrib_enum_name)
                                        )
                                        .ok()
                                    }
                                })
                            } else if options.param_type.is_copy_type() {
                                if options.default.is_some() {
                                    tokens.push(quote!{
                                        fn #field_name_ident(&self) -> #rust_type { 
                                            self.0
                                                .read()
                                                .get(&#attrib_name::#attrib_enum_name)
                                                .and_then(|v| v.try_into().ok())
                                                .expect("param not defined")
                                        }
                                    });
                                } else {
                                    tokens.push(quote!{
                                        fn #field_name_ident(&self) -> Option<#rust_type> { 
                                            self.0
                                                .read()
                                                .get(&#attrib_name::#attrib_enum_name)
                                                .and_then(|v| v.try_into().ok())
                                        }
                                    });
                                }
                            } else if options.param_type.is_ref_type() {
                                if options.default.is_some() {
                                    tokens.push(quote!{
                                        fn #field_name_ident(&self) -> MappedRwLockReadGuard<#rust_type> {
                                            RwLockReadGuard::try_map::<#rust_type, _>(self.0.read(), 
                                            |v| 
                                                v.get(&#attrib_name::#attrib_enum_name)
                                                .and_then(|v| v.try_into().ok())
                                            )
                                            .ok()
                                            .expect("param not defined")
                                        }
                                    });
                                } else {
                                    tokens.push(quote!{
                                        fn #field_name_ident(&self) -> Option<MappedRwLockReadGuard<#rust_type>> {
                                            RwLockReadGuard::try_map::<#rust_type, _>(self.0.read(), 
                                            |v| 
                                                v.get(&#attrib_name::#attrib_enum_name)
                                                .and_then(|v| v.try_into().ok())
                                            )
                                            .ok()
                                        }
                                    });
                                }
                            } else {
                                tokens.push(quote! { 
                                    fn #field_name_ident(&self) -> Option<()> { todo!() }
                                });
                            }
    
                            match options.param_type {
                                ParamType::Any => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Param)
                                    { self.0.write().insert(#attrib_name::#attrib_enum_name, val) }
                                }),
                                ParamType::AvatarID => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: AvatarId) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::AvatarId(val)) }
                                }),
                                ParamType::AvatarIDSet => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: HashSet<AvatarId>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::AvatarIdSet(val)) }
                                }),
                                ParamType::AvatarIDVector => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<AvatarId>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorAvatarId(val)) }
                                }),
                                ParamType::BitSetFilter => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: u32) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::BitSetFilter(val)) }
                                }),
                                ParamType::Bool => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: bool) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Bool(val)) }
                                }),
                                ParamType::ClassRefPowerRangeList => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::ClassRefPowerRangeList(val.to_owned())) }
                                }),
                                ParamType::ContentRef => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::ContentRef(val.to_owned())) }
                                }),
                                ParamType::ContentRefAndInt => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::ContentRefAndInt(val.to_owned())) }
                                }),
                                ParamType::ContentRefList => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::ContentRefList(val.to_owned())) }
                                }),
                                ParamType::Float => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: f32) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Float(val)) }
                                }),
                                ParamType::FloatRange => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: (f32, f32)) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::FloatRange(val)) }
                                }),
                                ParamType::FloatVector => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<f32>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorFloat(val)) }
                                }),
                                ParamType::Guid => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Uuid) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Guid(val)) }
                                }),
                                ParamType::GuidPair => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: (Uuid, Uuid)) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::GuidPair(val)) }
                                }),
                                ParamType::Int => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: i32) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Int(val)) }
                                }),
                                ParamType::Int64 => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: i64) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Int64(val)) }
                                }),
                                ParamType::Int64Vector => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<i64>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorInt64(val)) }
                                }),
                                ParamType::IntVector => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<i32>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorInt(val)) }
                                }),
                                ParamType::Json => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Value) { self.0.write().insert(#attrib_name::#attrib_enum_name, val) }
                                }),
                                ParamType::LocalizedString => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Uuid) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::LocalizedString(val)) }
                                }),
                                ParamType::OAInstanceGroup => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::InstanceGroup(val.to_owned())) }
                                }),
                                ParamType::OASetGuid => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: HashSet<Uuid>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::GuidSet(val)) }
                                }),
                                ParamType::OAVactorLocalizedString => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<Uuid>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorLocalizedString(val)) }
                                }),
                                ParamType::OAVectorGuid => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<Uuid>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorGuid(val)) }
                                }),
                                ParamType::String => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: &str) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::String(val.to_owned())) }
                                }),
                                ParamType::StringFloatPair => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: (String, f32)) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::StringFloatPair(val)) }
                                }),
                                ParamType::StringIntHashmap => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: HashMap<String, i32>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::HashmapStringInt(val)) }
                                }),
                                ParamType::StringStringHashmap => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: HashMap<String, String>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::HashmapStringString(val)) }
                                }),
                                ParamType::StringVector => tokens.push(quote! { 
                                    fn #set_field_name(&mut self, val: Vec<String>) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::VectorString(val)) }
                                }),
                                ParamType::Vector3 => {
                                    if options.flags.contains(&ParamFlag::Uts) {
                                        tokens.push(quote! {
                                            fn #set_field_name(&mut self, val: (u32, Vec3)) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Vector3Uts(val)) }
                                        })
                                    } else {
                                        tokens.push(quote! {
                                            fn #set_field_name(&mut self, val: Vec3) { self.0.write().insert(#attrib_name::#attrib_enum_name, Param::Vector3(val)) }
                                        })
                                    }
                                },
                            }
    
                            quote! {
                                #(#tokens)*
                            }
                        },
                        None => {
                            panic!("No type information for owned attribute!");
                        }
                    }
                }).collect();

                traits.push(quote!{
                    impl #view_name for #class_name {
                        #(#getter_setter)*
                    }
                });
            }

            traits
        };

        // generate component factory
        let bundle_type = {
            let mut components: Vec<TokenStream> = Vec::new();

            components.push(quote!(ParamBox));
            components.push(quote!( #class_name ));

            let mut current_class = Some(v.to_owned().to_owned());
            while let Some(parent_class_ref) = current_class {
                current_class = parent_class_ref.borrow().extends_ref.clone();

                let component_name = format_ident!("{}Component", parent_class_ref.borrow().name.to_case(Case::UpperCamel));

                components.push(quote!( #component_name ));
            }

            quote!{(#(#components),*)}
        };

        let bundle_class = {
            let mut components: Vec<TokenStream> = Vec::new();

            components.push(quote!{ self.clone_ref().into_box() });
            components.push(quote!{ self.clone_ref() });

            let mut current_class = Some(v.to_owned().to_owned());
            while let Some(parent_class_ref) = current_class {
                current_class = parent_class_ref.borrow().extends_ref.clone();

                let component_name = format_ident!("{}Component", parent_class_ref.borrow().name.to_case(Case::UpperCamel));

                components.push(quote!{ #component_name(Box::new(self.clone_ref())) });
            }

            components
        };

        let default_params: Vec<_> = v.borrow().paramoption.iter()
        .filter_map(|(name, options)| {
            let field_name = name.to_case(Case::UpperCamel);

            let attrib_name = format_ident!("{}Attribute", class.name.to_case(Case::UpperCamel));
            let field_name_ident = format_ident!("{}", match field_name.as_str() {
                "static" => "r#static",
                "type" => "r#type",
                _ => field_name.as_str(),
            });

            if options.default_literal.is_some() {
                Some(quote!(set.insert(#attrib_name::#field_name_ident, #attrib_name::#field_name_ident.default().unwrap().clone());))
            } else {
                None
            }
        })
        .collect();

        let default_implt = if default_params.is_empty() {
            quote!(Self(Arc::new(RwLock::new(ParamSet::new()))))
        } else {
            quote!{
                let mut set = ParamSet::new();
                #(#default_params)*

                Self(Arc::new(RwLock::new(set)))
            }
        };

        quote!{
            #[derive(Component)]
            pub struct #class_name(Arc<RwLock<ParamSet<#attrib_name>>>);

            impl ParamClass for #class_name {
                type Attributes = #attrib_name;
                type EntityBundle = #bundle_type;

                fn from_set(set: ParamSet<Self::Attributes>) -> Self {
                    Self(Arc::new(RwLock::new(set)))
                }

                fn as_set(&self) -> RwLockReadGuard<ParamSet<Self::Attributes>> {
                    self.0.read()
                }

                fn into_set(self) -> ParamSet<Self::Attributes> {
                    Arc::into_inner(self.0).expect("Class must be the only reference to this instance").into_inner()
                }
            
                fn apply(&mut self, set: ParamSet<Self::Attributes>) {
                    self.0.write().extend(set)
                }

                fn clone_ref(&self) -> Self {
                    Self(self.0.clone())
                }

                fn as_bundle(&self) -> Self::EntityBundle {
                    (#(#bundle_class),*)
                }

                fn into_box(self) -> ParamBox {
                    ParamBox::new(ClassId::#class_name, Box::new(self))
                }
            }

            impl Default for #class_name {
                fn default() -> Self {
                    #default_implt
                }
            }

            impl Clone for #class_name {
                fn clone(&self) -> Self {
                    Self(Arc::new(RwLock::new(self.as_set().clone())))
                }
            }

            impl Serialize for #class_name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let json = self.as_persistent_json();
                    json.serialize(s)
                }
            }
            
            impl<'de> Deserialize<'de> for #class_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                {
                    let json = Value::deserialize(deserializer)?;
                    Ok(#class_name::from_json(&json).unwrap())
                }
            }
            

            #(#trait_implementations)*
        }
    }).collect();
    
    let param_class_enum: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));

        quote!(#class_name)
    }).collect();

    let param_class_display: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));
        let name_literal = class.name.as_str();

        quote!(ClassId::#class_name => #name_literal)
    }).collect();

    let param_class_string_map: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();
        let name_literal = class.name.as_str();
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));

        quote!(#name_literal => Ok(ClassId::#class_name),)
    }).collect();

    let param_class_int_map: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();
        let id_literal = class.unique_id;
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));

        quote!(#id_literal => Ok(ClassId::#class_name),)
    }).collect();

    let class_from_json: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => Ok(Box::new(#class_name::from_json(value)?)), }
    }).collect();

    let class_into_json: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => value.downcast_ref::<#class_name>().expect("class id mismatch").as_persistent_json(), }
    }).collect();

    let class_read: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => {
                let (i, class) = #class_name::read(i)?;
                Ok((i, Box::new(class)))
            }, 
        }
    }).collect();

    let class_write: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => value.downcast_ref::<#class_name>().expect("class id mismatch").write(writer), }
    }).collect();

    let class_write_to_client: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => value.downcast_ref::<#class_name>().expect("class id mismatch").write_to_client(writer), }
    }).collect();

    let class_from_json_set: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        let attribute_name = format_ident!("{}Attribute",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => Ok(Box::new(#attribute_name::deserialize_json_set(value)?)), }
    }).collect();

    let class_into_json_set: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        let attribute_name = format_ident!("{}Attribute",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => #attribute_name::serialize_json_set(value.downcast_ref::<ParamSet<#attribute_name>>().expect("class id mismatch")), }
    }).collect();

    let class_read_set: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        let attribute_name = format_ident!("{}Attribute",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => {
                let (i, set) = ParamSet::<#attribute_name>::read(i)?;
                Ok((i, Box::new(set)))
            }, 
        }
    }).collect();

    let class_write_set: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        let attribute_name = format_ident!("{}Attribute",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => value.downcast_ref::<ParamSet<#attribute_name>>().expect("class id mismatch").write(writer), }
    }).collect();

    let class_write_set_to_client: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        let attribute_name = format_ident!("{}Attribute",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => value.downcast_ref::<ParamSet<#attribute_name>>().expect("class id mismatch").write_to_client(writer), }
    }).collect();
    
    let class_clone: Vec<_> = final_classes.iter().map(|v| {
        let class_name = format_ident!("{}Class",&v.borrow().name.to_case(Case::UpperCamel));
        quote! { ClassId::#class_name => Box::new(value.downcast_ref::<#class_name>().expect("class id mismatch").clone()), }
    }).collect();

    let class_id_int: Vec<_> = final_classes.iter().map(|v| {
        let class = v.borrow();
        let id_literal = class.unique_id;
        let class_name = format_ident!("{}Class", class.name.to_case(Case::UpperCamel));

        quote!(ClassId::#class_name => #id_literal)
    }).collect();

    write_source("generated_params.rs", quote! {
        use glam::Vec3;
        use std::sync::Arc;
        use std::collections::HashSet;
        use std::collections::HashMap;
        use serde_json::Value;
        use std::any::Any;
        use std::str::FromStr;
        use std::io;
        use std::fmt::Display;
        use std::fmt::Formatter;
        use std::ops::Deref;
        use std::ops::DerefMut;
        use parking_lot::RwLock;
        use parking_lot::RwLockReadGuard;
        use parking_lot::MappedRwLockReadGuard;
        use nom::IResult;
        use nom::error::VerboseError;
        use bitstream_io::ByteWrite;
        use serde::Serialize;
        use serde::Serializer;
        use serde::Deserialize;
        use serde::Deserializer;
        use bevy_ecs::prelude::*;
        use uuid::uuid;
        use serde_json::json;
        use once_cell::sync::Lazy;

        use crate::AvatarId;
        use crate::Uuid;
        use crate::param::ParamAttrib;
        use crate::param::ParamBox;
        use crate::param::ParamType;
        use crate::param::ParamFlag;
        use crate::param::ParamClass;
        use crate::param::ParamSet;
        use crate::param::ParamError;
        use crate::param::Param;

        #(#param_name_enums)*

        #(#param_view_traits)*

        #(#param_classes)*
        
        #[derive(Clone, Copy)]
        pub enum ClassId {
            #(#param_class_enum),*
        }

        impl ClassId {
            pub(crate) fn class_from_json(&self, value: &Value) -> Result<Box<dyn Any>, io::Error> {
                match self {
                    #(#class_from_json)*
                }
            }

            pub(crate) fn class_into_json(&self, value: &dyn Any) -> Value {
                match self {
                    #(#class_into_json)*
                }
            }

            pub(crate) fn set_from_json(&self, value: &Value) -> Result<Box<dyn Any + Send + Sync>, io::Error> {
                match self {
                    #(#class_from_json_set)*
                }
            }

            pub(crate) fn set_into_json(&self, value: &dyn Any) -> Value {
                match self {
                    #(#class_into_json_set)*
                }
            }

            pub(crate) fn read<'a>(&self, i: &'a [u8]) -> IResult<&'a [u8], Box<dyn Any>, VerboseError<&'a [u8]>> {
                match self {
                    #(#class_read)*
                }
            }

            pub(crate) fn write<'a, T>(&self, value: &dyn Any, writer: &mut T) -> Result<(), io::Error> 
            where T: ByteWrite {
                match self {
                    #(#class_write)*
                }
            }

            pub(crate) fn write_to_client<T>(&self, value: &dyn Any, writer: &mut T) -> Result<(), io::Error> 
            where T: ByteWrite {
                match self {
                    #(#class_write_to_client)*
                }
            }

            pub(crate) fn read_set<'a>(&self, i: &'a [u8]) -> IResult<&'a [u8], Box<dyn Any + Send + Sync>, VerboseError<&'a [u8]>> {
                match self {
                    #(#class_read_set)*
                }
            }

            pub(crate) fn write_set<T>(&self, value: &dyn Any, writer: &mut T) -> Result<(), io::Error> 
            where T: ByteWrite {
                match self {
                    #(#class_write_set)*
                }
            }

            pub(crate) fn write_set_to_client<T>(&self, value: &dyn Any, writer: &mut T) -> Result<(), io::Error> 
            where T: ByteWrite {
                match self {
                    #(#class_write_set_to_client)*
                }
            }

            pub(crate) fn clone_class(&self, value: &dyn Any) -> Box<dyn Any> {
                match self {
                    #(#class_clone)*
                }
            }
        }

        impl Display for ClassId {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                let name = match self {
                    #(#param_class_display),*
                };

                write!(f, "{}", name)
            }
        }

        impl FromStr for ClassId {
            type Err = ParamError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#param_class_string_map)*
                    _ => Err(ParamError(())),
                }
            }
        }

        impl TryFrom<u16> for ClassId {
            type Error = ParamError;

            fn try_from(val: u16) -> Result<Self, Self::Error> {
                match val {
                    #(#param_class_int_map)*
                    _ => Err(ParamError(())),
                }
            }
        }

        impl From<ClassId> for u16 {
            fn from(val: ClassId) -> u16 {
                match val {
                    #(#class_id_int),*
                }
            }
        }

        impl From<ClassId> for u32 {
            fn from(val: ClassId) -> u32 {
                let id: u16 = val.into();
                id as u32
            }
        }
    })
}
