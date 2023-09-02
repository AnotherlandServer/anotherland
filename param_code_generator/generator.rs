use std::{path::{Path, PathBuf}, io, fs, collections::HashMap, env, rc::Rc, cell::RefCell};

use nom::{IResult, character::complete, error::Error};
use proc_macro2::TokenStream;
use quote::format_ident;
use ::quote::quote;

use convert_case::{Converter, Boundary, Pattern};

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
}

fn generate_fields(class: Rc<RefCell<ParamClass>>, _: &HashMap<String, Rc<RefCell<ParamClass>>>) -> io::Result<Vec<TokenStream>> {
    let class = class.borrow();
    let fields = class.paramid.iter().map(|v| {

        let mut field_name = Converter::new()
        .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
        .set_pattern(Pattern::Lowercase)
        .set_delim("_")
        .convert(&v.0);

        if field_name == "static" { 
            field_name = "r#static".to_owned();
        };

        if field_name == "type" { 
            field_name = "r#type".to_owned();
        };

        let field_name = format_ident!("{}", field_name);
        quote! { pub #field_name: Option<CParam> }
    }).collect();

    /*if !class.extends.is_empty() {
        let parent_fields = generate_fields(
            classes.get(&class.extends)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, format!("Class {} not found!", class.extends)))?
                .to_owned(), 
                classes)?;
        Ok([parent_fields, fields].concat())
    } else {*/
        Ok(fields)
    //}
}

fn generate_field_writers(class: Rc<RefCell<ParamClass>>, _: &HashMap<String, Rc<RefCell<ParamClass>>>) -> io::Result<Vec<TokenStream>> {
    let class = class.borrow();
    let fields = class.paramid.iter().map(|v| {

        let mut field_name = Converter::new()
        .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
        .set_pattern(Pattern::Lowercase)
        .set_delim("_")
        .convert(&v.0);

        if field_name == "static" { 
            field_name = "r#static".to_owned();
        };

        if field_name == "type" { 
            field_name = "r#type".to_owned();
        };

        let field_name = format_ident!("{}", field_name);
        let field_idx = v.1;

        quote! { 
            if let Some(#field_name) = &self.#field_name {
                field_count += 1;

                let _ = writer.write(#field_idx);
                let _ = writer.write_bytes(#field_name.to_bytes().as_slice());
            } 
        }
    }).collect();

    /*if !class.extends.is_empty() {
        let parent_fields = generate_fields(
            classes.get(&class.extends)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, format!("Class {} not found!", class.extends)))?
                .to_owned(), 
                classes)?;
        Ok([parent_fields, fields].concat())
    } else {*/
        Ok(fields)
    //}
}

fn generate_field_parsers(class: Rc<RefCell<ParamClass>>, _: &HashMap<String, Rc<RefCell<ParamClass>>>) -> io::Result<Vec<TokenStream>> {
    let class = class.borrow();

    let fields = class.paramid.iter().map(|v| {
        let field_idx = v.1;
        quote! { #field_idx => tuple((success(#field_idx), CParam::from_bytes))(i), }
    }).collect();

    /*if !class.extends.is_empty() {
        let parent_fields = generate_field_parsers(
            classes.get(&class.extends)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, format!("Class {} not found!", class.extends)))?
                .to_owned(), 
                classes)?;
        Ok([parent_fields, fields].concat())
    } else {*/
        Ok(fields)
    //}
}

fn generate_field_assign(class: Rc<RefCell<ParamClass>>, _: &HashMap<String, Rc<RefCell<ParamClass>>>) -> io::Result<Vec<TokenStream>> {
    let class = class.borrow();

    let fields = class.paramid.iter().map(|v| {
        let field_idx: u16 = v.1;
        let mut field_name = Converter::new()
        .set_boundaries(&[Boundary::Hyphen, Boundary::Underscore, Boundary::Space, Boundary::LowerUpper])
        .set_pattern(Pattern::Lowercase)
        .set_delim("_")
        .convert(&v.0);

        if field_name == "static" { 
            field_name = "r#static".to_owned();
        };

        if field_name == "type" { 
            field_name = "r#type".to_owned();
        };


        let field_ident = format_ident!("{}", field_name);

        quote! { #field_idx => instance.#field_ident = Some(field.1), }
    }).collect();

    /*if !class.extends.is_empty() {
        let parent_fields = generate_field_assign(
            classes.get(&class.extends)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, format!("Class {} not found!", class.extends)))?
                .to_owned(), 
                classes)?;
        Ok([parent_fields, fields].concat())
    } else {*/
        Ok(fields)
    //}
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

    let lines = file_content.lines();
    let parsed_lines: Vec<_> = lines
        .map(|line| -> IResult<&str, ParamIniLine> {
            let tokens: Vec<&str> = line.split_whitespace().collect();
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
                _ => ParamIniLine::Ignore,
            };

            Ok((line, r))
        }).collect();

    let mut paramlist = Paramlist::default();
    let mut class_map: HashMap<String, Rc<RefCell<ParamClass>>> = HashMap::new();
    let mut current_class = None;

    for (line_number, line) in parsed_lines.iter().enumerate() {
        //println!("{:#?}", line);

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
            }
            Err(e) => {
                println!("Error in line {}", line_number + 1);
                println!("{}", e.to_string());
                panic!();
            }
            _ => (),
        }
    }

    paramlist.classes = class_map.values().map(|v| v.to_owned()).collect();

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    // generate structs
    let classes_src: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = format_ident!("CParamClass_{}", v.borrow().name);
        let class_fields = generate_fields(v.to_owned(), &class_map).expect("Parameter generation failed");
        
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Default)]
            pub struct #class_name {
                #(#class_fields),*
            }
        }
    }).collect();

    // implement structs
    let classes_impl: Vec<_> = paramlist.classes.iter().map(|v| {
        let class_name = v.borrow().name.to_owned();
        let struct_name = format_ident!("CParamClass_{}", class_name);
        let field_parser = generate_field_parsers(v.to_owned(), &class_map).expect("Parameter parse generation failed");
        let field_writer = generate_field_writers(v.to_owned(), &class_map).expect("Parameter writer generation failed");
        let field_apply: Vec<TokenStream> = generate_field_assign(v.to_owned(), &class_map).expect("Parameter assign generation failed");

        quote! {
            impl #struct_name {
                #[allow(dead_code)]
                pub fn from_bytes<'a>(i: &'a [u8]) -> IResult<&'a [u8], #struct_name, VerboseError<&'a [u8]>> { 
                    context(#class_name, |i| {
                        let (i, _) = number::complete::le_u8(i)?;
                        let (i, field_count) = number::complete::le_u16(i)?;
                        let (i, tagged_fields) = count(|i| {
                            let (i, id) = number::complete::le_u16(i)?;
                            match id {
                                #(#field_parser)*
                                _ => fail::<&'a [u8], (u16, CParam), VerboseError<&'a [u8]>>(i),
                            }
                        }, field_count as usize)(i)?;

                        let mut instance = #struct_name::default();

                        for field in tagged_fields {
                            match field.0 {
                                #(#field_apply)*
                                _ => unreachable!(),
                            }
                        }
        
                        Ok((i, instance))
                    })(i)
                }

                #[allow(dead_code)]
                pub fn to_bytes(&self) -> Vec<u8> {
                    let mut field_count = 0u16;
                    let field_buf = {
                        let mut buf = Vec::new();
                        let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
        
                        #(#field_writer)*

                        buf
                    };

                    let mut buf = Vec::new();
                    let mut writer = ByteWriter::endian(&mut buf, LittleEndian);

                    let _ = writer.write(1u8);
                    let _ = writer.write(field_count);
                    let _ = writer.write_bytes(field_buf.as_slice());

                    buf
                }
            }
        }
    }).collect();

    write_source(&out_dir_path.join("generated_params.rs"), quote! {
        #(#classes_src)*
        #(#classes_impl)*
    })
}

fn write_source(dest: &PathBuf, tokens: TokenStream) -> io::Result<()> {
    let source = if tokens.is_empty() { "".to_owned() } else {
        let item: syn::File = match syn::parse2(tokens) {
            Ok(v) => v,
            Err(e) => {
                println!("Code generation error for {}!", dest.to_str().unwrap());
                println!("Error: {}", e.to_string());
                println!("Line: {:#?}", e.span());
                panic!();
            }
        };

        prettyplease::unparse(&item)
    };

    fs::write(dest, source)
}