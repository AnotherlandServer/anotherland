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

use std::{cell::RefCell, collections::HashMap, fs, io, path::Path, rc::Rc};

use convert_case::{Case, Casing};
use nom::{character::complete, error::Error, IResult};
use proc_macro2::{Group, Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use regex::Regex;
use uuid::Uuid;

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
    extends_ref: Option<Rc<RefCell<ParamClass>>>,
    final_class: bool,
    binds_to: Vec<String>,
    content_table_binding: String,
    icon: String,
    paramid: Vec<(String, u16)>,
    paramoption: Vec<(String, ParamOptions)>
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ParamOptions {
    param_type: ParamType,
    default: Option<String>,
    default_literal: TokenStream,
    flags: Vec<ParamFlag>,
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
            ParamType::Any => quote!([u8]),
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
            ParamType::Json => quote!(JsonValue),
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

    fn is_copy_type(self) -> bool {
        matches!(self, ParamType::AvatarID | 
            ParamType::Bool |
            ParamType::Float |
            ParamType::Int |
            ParamType::Int64)
    }

    fn is_ref_type(self) -> bool {
        matches!(self, 
            ParamType::Any |
            ParamType::AvatarIDSet |
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
                            ParamType::Any => quote! { Value::Any(vec![]) },
                            ParamType::AvatarID => {
                                let val: u64 = default_str.strip_prefix('#').unwrap().parse().expect("failed to parse avatar id");
                                quote! { Value::AvatarId(AvatarId::from_u64(#val)) }
                            },
                            ParamType::AvatarIDSet => quote! { Value::AvatarIdSet(HashSet::new()) },
                            ParamType::AvatarIDVector => quote! { Value::VectorAvatarId(vec![]) },
                            ParamType::BitSetFilter => quote! { Value::BitSetFilter(0) },
                            ParamType::Bool => if default_str == "true" { quote!{ Value::Bool(true) } } else { quote!{ Value::Bool(false) } },
                            ParamType::ClassRefPowerRangeList => quote! { Value::ClassRefPowerRangeList(#default_str.to_string()) },
                            ParamType::ContentRef => quote! { Value::ContentRef(#default_str.to_string()) },
                            ParamType::ContentRefAndInt => quote! { Value::ContentRefAndInt(#default_str.to_string()) },
                            ParamType::ContentRefList => quote! { Value::ContentRefList(#default_str.to_string()) },
                            ParamType::Float => {
                                let val: f32 = default_str.parse().expect("failed to parse float");
                                quote! { Value::Float(#val) }
                            },
                            ParamType::FloatRange => quote! { Value::FloatRange((0.0,0.0)) },
                            ParamType::FloatVector => quote! { Value::VectorFloat(vec![]) },
                            ParamType::Guid => {
                                let uuid_bytes = Uuid::parse_str(default_str).unwrap().as_bytes().to_token_stream();
                                quote!{ Value::Guid(Uuid::from_bytes(#uuid_bytes)) }
                            },
                            ParamType::GuidPair => quote! { Value::GuidPair((UUID_NIL, UUID_NIL)) },
                            ParamType::Int => {
                                let val: i32 = default_str.parse().expect("failed to parse int");
                                quote! { Value::Int(#val) }
                            },
                            ParamType::Int64 => {
                                let val: i64 = default_str.parse().expect("failed to parse int64");
                                quote! { Value::Int64(#val) }
                            },
                            ParamType::Int64Vector => quote! { Value::VectorInt64(vec![]) },
                            ParamType::IntVector => {
                                let values: Vec<_> = default_str
                                    .split(',')
                                    .map(|v| {
                                        let v = v.trim();
                                        &v[1..v.len()-1]
                                    })
                                    .filter_map(|v| v.parse::<i32>().ok())
                                    .map(|v| quote!(#v))
                                    .collect();

                                quote! { Value::VectorInt(vec![#(#values),*]) }
                            },
                            ParamType::Json => {
                                let default_str = default_str
                                    .replace("\\\"", "\"")
                                    .replace("\\\\", "\\");
                                quote! { Value::JsonValue(serde_json::from_str(#default_str).unwrap()) }
                            },
                            ParamType::LocalizedString => {
                                let uuid_bytes = Uuid::parse_str(default_str).unwrap().as_bytes().to_token_stream();
                                quote! { Value::LocalizedString(Uuid::from_bytes(#uuid_bytes)) }
                            },
                            ParamType::String => quote! { Value::String(#default_str.to_string()) },
                            ParamType::StringFloatPair => quote! { Value::StringFloatPair((String::default(), 0.0)) },
                            ParamType::StringStringHashmap => quote! { Value::HashmapStringString(HashMap::new()) },
                            ParamType::StringIntHashmap => quote! { Value::HashmapStringInt(HashMap::new()) },
                            ParamType::StringVector => quote! { Value::VectorString(vec![]) },
                            ParamType::Vector3 => {
                                let parts: Vec<_> = default_str.split(' ').collect();
                                let x: f32 = parts[0].parse().expect("failed to parse vector3");
                                let y: f32 = parts[1].parse().expect("failed to parse vector3");
                                let z: f32 = parts[2].parse().expect("failed to parse vector3");


                                if flags.contains(&ParamFlag::Uts) {
                                    quote! { Value::Vector3Uts((0, Vec3::new(#x, #y, #z))) }
                                } else {
                                    quote! { Value::Vector3(Vec3::new(#x, #y, #z)) }
                                }
                            },
                            ParamType::OAInstanceGroup => quote! { Value::InstanceGroup(String::default()) },
                            ParamType::OASetGuid => quote! { Value::GuidSet(HashSet::new()) },
                            ParamType::OAVectorGuid => quote! { Value::VectorGuid(vec![]) },
                            ParamType::OAVactorLocalizedString => quote! { Value::VectorLocalizedString(vec![]) },
                        }
                    } else {
                        match paramtype.as_ref().unwrap() {
                            ParamType::Any => quote! { Value::Any(vec![]) },
                            ParamType::AvatarID => quote! { Value::AvatarId(AvatarId::from_u64(0)) },
                            ParamType::AvatarIDSet => quote! { Value::AvatarIdSet(HashSet::new()) },
                            ParamType::AvatarIDVector => quote! { Value::VectorAvatarId(vec![]) },
                            ParamType::BitSetFilter => quote! { Value::BitSetFilter(0) },
                            ParamType::Bool =>  quote!{ Value::Bool(false) },
                            ParamType::ClassRefPowerRangeList => quote! { Value::ClassRefPowerRangeList(String::default()) },
                            ParamType::ContentRef => quote! { Value::ContentRef(String::default()) },
                            ParamType::ContentRefAndInt => quote! { Value::ContentRefAndInt(String::default()) },
                            ParamType::ContentRefList => quote! { Value::ContentRefList(String::default()) },
                            ParamType::Float => quote! { Value::Float(0.0) },
                            ParamType::FloatRange => quote! { Value::FloatRange((0.0, 0.0)) },
                            ParamType::FloatVector => quote! { Value::VectorFloat(vec![]) },
                            ParamType::Guid => quote!{ Value::Guid(UUID_NIL) },
                            ParamType::GuidPair => quote! { Value::GuidPair((UUID_NIL, UUID_NIL)) },
                            ParamType::Int => quote! { Value::Int(0) },
                            ParamType::Int64 => quote! { Value::Int64(0) },
                            ParamType::Int64Vector => quote! { Value::VectorInt64(vec![]) },
                            ParamType::IntVector => quote! { Value::VectorInt(vec![]) },
                            ParamType::Json => quote! { Value::JsonValue(JsonValue::default()) },
                            ParamType::LocalizedString => quote! { Value::LocalizedString(UUID_NIL) },
                            ParamType::String => quote! { Value::String(String::default()) },
                            ParamType::StringFloatPair => quote! { Value::StringFloatPair((String::default(), 0.0)) },
                            ParamType::StringStringHashmap => quote! { Value::HashmapStringString(HashMap::new()) },
                            ParamType::StringIntHashmap => quote! { Value::HashmapStringInt(HashMap::new()) },
                            ParamType::StringVector => quote! { Value::VectorString(vec![]) },
                            ParamType::Vector3 => {
                                if flags.contains(&ParamFlag::Uts) {
                                    quote! { Value::Vector3Uts((0, Vec3::default())) }
                                } else {
                                    quote! { Value::Vector3(Vec3::default()) }
                                }
                            },
                            ParamType::OAInstanceGroup => quote! { Value::InstanceGroup(String::default()) },
                            ParamType::OASetGuid => quote! { Value::GuidSet(HashSet::new()) },
                            ParamType::OAVectorGuid => quote! { Value::VectorGuid(vec![]) },
                            ParamType::OAVactorLocalizedString => quote! { Value::VectorLocalizedString(vec![]) },
                        }
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
    /*let final_classes: Vec<_> = paramlist.classes.iter().filter(|v| { 
        v.borrow().final_class || ADDITIONAL_PARAM_CLASSES.contains(&v.borrow().name.as_str())
    }).collect();*/

    // generate enum for fields
    let param_name_enums: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();

        let enum_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let class_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let enum_lookup_name = format_ident!("{}_ATTRIBUTES", class.name.to_case(Case::UpperSnake));
        let enum_lookup_id_name = format_ident!("{}_ATTRIBUTES_ID", class.name.to_case(Case::UpperSnake));

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
            let default_literal = &v.1.default_literal;

            if 
                matches!(v.1.param_type, ParamType::Json) || 
                matches!(v.1.param_type, ParamType::OAInstanceGroup) ||
                matches!(v.1.param_type, ParamType::String) ||
                matches!(v.1.param_type, ParamType::StringVector) ||
                matches!(v.1.param_type, ParamType::StringFloatPair) || 
                matches!(v.1.param_type, ParamType::StringIntHashmap) ||
                matches!(v.1.param_type, ParamType::StringStringHashmap) ||
                matches!(v.1.param_type, ParamType::AvatarIDSet) ||
                matches!(v.1.param_type, ParamType::OASetGuid) ||
                matches!(v.1.param_type, ParamType::IntVector) ||
                matches!(v.1.param_type, ParamType::ContentRef) ||
                matches!(v.1.param_type, ParamType::ContentRefList) ||
                matches!(v.1.param_type, ParamType::ContentRefAndInt) ||
                matches!(v.1.param_type, ParamType::ClassRefPowerRangeList)
            {
                quote!{static #entry_name: Lazy<Value> = Lazy::new(|| #default_literal);}
            } else {
                quote!{static #entry_name: Value = #default_literal;}
            }
        }).collect();

        let enum_defaults: Vec<_> = class.paramoption.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let static_name = format_ident!("{}", v.0.to_case(Case::UpperSnake));
            quote!{Self::#entry_name => &#static_name,}
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

        let static_lookup: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let name_literal = &v.0;

            quote!(#name_literal => #enum_name::#entry_name,)
        }).collect();

        let static_id_lookup: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let id_literal = &v.1;

            quote!(#id_literal => #enum_name::#entry_name,)
        }).collect();

        let static_info: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            quote!(Self::#entry_name => &Self::#entry_name,)
        }).collect();

        let enum_from_u16: Vec<_> = class.paramid.iter().map(|v| {
            let entry_name = format_ident!("{}", v.0.to_case(Case::UpperCamel));
            let id_literal = &v.1;

            quote!(#id_literal => Ok(Self::#entry_name),)
        }).collect();

        let id_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#enum_ids),*
                }
            }
        };

        let name_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#enum_names),*
                }
            }
        };

        let datatype_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#enum_types)*
                    #enum_non_option
                }
            }
        };

        let default_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#enum_defaults)*
                    #enum_non_option
                }
            }
        };

        let flags_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#enum_flags)*
                    #enum_non_option
                }
            }
        };

        let info_match = if enum_ids.is_empty() {
            quote! { unreachable!() }
        } else {
            quote! {
                match self {
                    #(#static_info)*
                }
            }
        };

        quote!{
            #[derive(PartialEq, Eq, Hash, Clone, Copy)]
            pub enum #enum_name {
                #(#enum_entries),*
            }

            static #enum_lookup_name: phf::Map<&'static str, #enum_name> = phf_map! {
                #(#static_lookup)*
            };

            static #enum_lookup_id_name: phf::Map<u16, #enum_name> = phf_map! {
                #(#static_id_lookup)*
            };

            impl Attribute for #enum_name {
                fn class() -> Class { Class::#class_name }

                fn static_info(&self) -> &'static dyn AttributeInfo {
                    #info_match
                }
            }

            impl AttributeInfo for #enum_name {
                fn class(&self) -> Class {
                    <Self as Attribute>::class()
                }

                fn id(&self) -> u16 { 
                    #id_match
                }

                fn name(&self) -> &'static str { 
                    #name_match
                }

                fn datatype(&self) -> ParamType { 
                    #datatype_match
                }

                fn default(&self) -> &'static Value { 
                    #(#static_defaults)*
                    #default_match
                }

                fn flags(&self) -> &[ParamFlag] {
                    #flags_match
                }
            }

            impl FromStr for #enum_name {
                type Err = ParamError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    #enum_lookup_name.get(s)
                        .map(|v| *v)
                        .ok_or(ParamError::UnknownAttributeName)
                }
            }

            impl TryFrom<u16> for #enum_name {
                type Error = ParamError;

                fn try_from(val: u16) -> Result<Self, Self::Error> {
                    match val {
                        #(#enum_from_u16)*
                        _ => Err(ParamError::UnknownAttributeId),
                    }
                }
            }
        }
    }).collect();

    /*let attribute_container: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));

        quote!(#class_name(#class_name))
    }).collect();*/

    let param_class_enum: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));

        quote!(#class_name)
    }).collect();

    let class_get_attribute: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let enum_lookup_name = format_ident!("{}_ATTRIBUTES", class.name.to_case(Case::UpperSnake));

        quote! {
            Self::#class_name => {
                #enum_lookup_name.get(attr)
                    .map(|a| a.static_info())
            }
        }
    }).collect();

    let class_get_attribute_by_id: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let enum_lookup_id_name = format_ident!("{}_ATTRIBUTES_ID", class.name.to_case(Case::UpperSnake));


        quote! {
            Self::#class_name => {
                #enum_lookup_id_name.get(&attr)
                    .map(|a| a.static_info())
            }
        }
    }).collect();

    let class_names: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name: syn::Ident = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let class_name_literal = &class.name;

        quote!{ Self::#class_name => #class_name_literal, }
    }).collect();

    let class_parser: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name: syn::Ident = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let class_name_literal = &class.name;

        quote!{ #class_name_literal => Ok(Self::#class_name), }
    }).collect();

    let class_create_set: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name: syn::Ident = format_ident!("{}", class.name.to_case(Case::UpperCamel));

        quote!{ Self::#class_name => Box::new(ParamSet::<#class_name>::new_from_attributes(attributes)), }
    }).collect();

    let class_from_id: Vec<_> = paramlist.classes.iter().map(|v| {
        let class = v.borrow();
        let class_name: syn::Ident = format_ident!("{}", class.name.to_case(Case::UpperCamel));
        let class_id_literal = class.unique_id;

        quote!{ #class_id_literal => Some(Self::#class_name), }
    }).collect();


    write_source("generated_params.rs", quote! {
        use glam::Vec3;
        use std::collections::HashSet;
        use std::collections::HashMap;
        use serde_json::Value as JsonValue;
        use std::str::FromStr;
        use std::io;
        use std::fmt::Display;
        use std::fmt::Formatter;
        use nom::IResult;
        use nom::error::VerboseError;
        use bitstream_io::ByteWrite;
        use serde::Serialize;
        use serde::Serializer;
        use serde::Deserialize;
        use serde::Deserializer;
        use bevy::prelude::*;
        use once_cell::sync::Lazy;
        use phf::phf_map;

        use toolkit::types::AvatarId;
        use toolkit::types::Uuid;
        use toolkit::types::UUID_NIL;
        use crate::Attribute;
        use crate::AttributeInfo;
        use crate::ParamType;
        use crate::ParamFlag;
        use crate::ParamSet;
        use crate::ParamError;
        use crate::Value;
        use crate::GameObjectData;
        use crate::GenericParamSet;

        #(#param_name_enums)*

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Class {
            #(#param_class_enum),*
        }

        impl Class {
            pub(crate) fn get_attribute(&self, attr: &str) -> Option<&'static dyn AttributeInfo> {
                match self {
                    #(#class_get_attribute),*
                }
            }

            pub(crate) fn get_attribute_from_id(&self, attr: u16) -> Option<&'static dyn AttributeInfo> {
                match self {
                    #(#class_get_attribute_by_id),*
                }
            }

            pub fn from_id(id: u16) -> Option<Self> {
                match id {
                    #(#class_from_id)*
                    _ => None,
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    #(#class_names)*
                }
            }

            pub(crate) fn create_param_set(&self, attributes: Vec<(&'static dyn AttributeInfo, Value)>) -> Box<dyn GenericParamSet> {
                match self {
                    #(#class_create_set)*
                }
            }
        }

        impl FromStr for Class {
            type Err = ParamError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#class_parser)*
                    _ => Err(ParamError::UnknownClass),
                }
            }
        }
    })
}