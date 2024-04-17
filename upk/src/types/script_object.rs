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

use std::{collections::{hash_map::Iter, HashMap}, sync::Arc};
use log::debug;
use nom::{bytes::complete::take, combinator::{fail, map}, error::Error, multi::{count, many_till}, number::complete::{be_u32, le_f32, le_i32, le_u32, le_u64, le_u8}, IResult};
use async_trait::async_trait;
use uuid::Uuid;
use crate::{types::StructProperty, Container, DeserializeUnrealObject, FName, LocalObjectIndexRef, Object, ObjectRef, PackageFile, UPKResult};

use super::ScriptClass;

#[derive(Debug)]
pub struct ScriptObject {
    attributes: HashMap<FName, ObjectProperty>,
}

impl ScriptObject {
    pub fn attribs(&self) -> Iter<FName, ObjectProperty> {
        self.attributes.iter()
    }

    pub fn attrib(&self, name: &str) -> Option<&ObjectProperty> {
        self.attributes
            .iter()
            .find(|(key, _)| &***key == name)
            .map(|(_, attrib)| attrib)
    }
}

#[derive(Debug)]
pub enum ObjectProperty {
    None,
    Array(Vec<ObjectProperty>),
    Bool(bool),
    Bytes(Vec<u8>),
    Class(ObjectRef),
    Component(ObjectRef),
    Delegate(ObjectRef),
    Float(f32),
    Interface(ObjectRef),
    Int(i32),
    Map(ObjectRef),
    Name(FName),
    Object(ObjectRef),
    Struct(Option<ObjectRef>, ScriptObject),
    String(String),
    Vector([f32; 3]),
    Vector2D([f32; 2]),
    Rotator([f32; 3]),
    Guid(Uuid),
    Enum(FName, Box<ObjectProperty>),
    Color(u32),
    Box {
        min: [f32; 3],
        max: [f32; 3],
        is_valid: u8,
    }
}

fn parse_attribute<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef, attribute: &'a ObjectRef, class_override: Option<&'a ObjectRef>, in_array: bool)
-> impl FnMut(&'a [u8]) 
-> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let class_name = if let Some(class_override) = class_override { 
            class_override.name() 
        } else { 
            attribute.class().name() 
        };

        match class_name {
            "StructProperty" => parse_struct(file, container, object, attribute, in_array)(i),
            "ArrayProperty" => parse_array(file, container, object, attribute)(i),
            "ComponentProperty" => parse_component(file, container, object, attribute)(i),
            "ObjectProperty" => parse_object(file, container, object, attribute)(i),
            "StrProperty" => parse_string(file, container, object, attribute)(i),
            "FloatProperty" => parse_float(file, container, object, attribute)(i),
            "NameProperty" => parse_name(file, container, object, attribute)(i),
            "ByteProperty" => parse_bytes(file, container, object, attribute)(i),
            "BoolProperty" => parse_bool(file, container, object, attribute)(i),
            "IntProperty" => parse_int(file, container, object, attribute)(i),
            "ClassProperty" => parse_class(file, container, object, attribute)(i),
            _ => unimplemented!("unimplemented property type: {}", attribute.class().name()),
        }
    }
}

fn parse_class<'a>(file: &'a Arc<PackageFile>, container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, idx) = le_i32(i)?;
        Ok((i, ObjectProperty::Class(
            container.resolve_object(file.to_owned(), LocalObjectIndexRef::from_idx(idx)).unwrap()
        )))
    }
}

fn parse_int<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, val) = le_i32(i)?;

        Ok((i, ObjectProperty::Int(val)))
    }
}


fn parse_bool<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, val) = le_u8(i)?;

        Ok((i, ObjectProperty::Bool(val != 0)))
    }
}

fn parse_bytes<'a>(file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        // todo: upk seems to encode enums as ByteProperty. how can we detect if the attribute does not decode an enum?

        let (i, enum_name) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;
        
        if &*enum_name == "None" {
            let (i, val) = le_u8(i)?;

            Ok((i, ObjectProperty::Enum(enum_name, Box::new(ObjectProperty::Int(val as i32)))))
        } else {
            let (i, val_name) = map(le_u64, |idx| {
                file.lookup_name(idx as usize)
            })(i)?;

            Ok((i, ObjectProperty::Enum(enum_name, Box::new(ObjectProperty::Name(val_name)))))
        }
    }
}

fn parse_name<'a>(file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, name) = le_u64(i)?;

        Ok((i, ObjectProperty::Name(file.lookup_name(name as usize))))
    }
}

fn parse_float<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, val) = le_f32(i)?;

        Ok((i, ObjectProperty::Float(val)))
    }
}

fn parse_string<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, len) = le_u32(i)?;
        let (i, data) = take(len)(i)?;

        Ok((i, ObjectProperty::String(String::from_utf8_lossy(&data[..data.len()-1]).to_string())))
    }
}

fn parse_component<'a>(file: &'a Arc<PackageFile>, container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, idx) = le_i32(i)?;
        Ok((i, ObjectProperty::Component(
            container.resolve_object(file.to_owned(), LocalObjectIndexRef::from_idx(idx)).unwrap()
        )))
    }
}

fn parse_object<'a>(file: &'a Arc<PackageFile>, container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let (i, idx) = le_i32(i)?;
        if idx == 0 {
            Ok((i, ObjectProperty::None))
        } else {
            Ok((i, ObjectProperty::Object(
                container.resolve_object(file.to_owned(), LocalObjectIndexRef::from_idx(idx)).unwrap()
            )))
        }
    }
}

fn parse_array<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef, attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let child = attribute.children().iter().next().unwrap();

        // element count
        let (i, elements) = le_u32(i)?;

        map(
            count(parse_attribute(file, container, object, child, None, true), elements as usize),
            |elements| {
                ObjectProperty::Array(elements)
            }
        )(i)
    }
}

fn tag_struct_end<'a>(file: &'a Arc<PackageFile>) 
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ()>
{
    move |i: &[u8]| {
        let (i, name) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;

        if &*name == "None" {
            Ok((i, ()))
        } else {
            fail(i)
        }
    }
}

fn parse_struct_attribute<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef) 
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], (FName, ObjectProperty)> 
{
    move |i: &[u8]| {
        let (i, name) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;

        debug!("{}", name);

        let attribute_def = object.data::<ScriptClass>()
            .unwrap()
            .attrib(&name)
            .unwrap();

        let (i, datatype) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;

        // attribute data size
        let (i, _) = le_u64(i)?;

        let attribute_class = container.lookup_object(&datatype).unwrap();
        let (i, attribute) = parse_attribute(file, container, object, attribute_def, Some(attribute_class), false)(i)?;

        Ok((i, (name, attribute)))
    }
}

fn parse_struct<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef, attrib: &'a ObjectRef, in_array: bool) 
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    enum StructClass<'a> {
        Intrinsic(FName),
        Class(&'a ObjectRef)
    }

    move |i: &[u8]| {
        let (i, class) = if in_array {
            if let Some(struct_property) = attrib.data::<StructProperty>() {
                (i, StructClass::Class(struct_property.class()))
            } else {
                panic!()
            }
        } else {
            let (i, class) = map(le_u64, |idx| {
                file.lookup_name(idx as usize)
            })(i)?;

            match class.as_ref() {
                "Vector" => (i, StructClass::Intrinsic(class)),
                "Vector2D" => (i, StructClass::Intrinsic(class)),
                "Rotator" => (i, StructClass::Intrinsic(class)),
                "Guid" => (i, StructClass::Intrinsic(class)),
                "Color" => (i, StructClass::Intrinsic(class)),
                "Box" => (i, StructClass::Intrinsic(class)),
                "ViewTargetTransitionParams" => {
                    (i, StructClass::Class(
                        container.lookup_object("Camera/ViewTargetTransitionParams").unwrap()
                    ))
                },
                _ => {
                    // lookup struct property class
                    (i, StructClass::Class(
                        object
                        .children()
                        .iter()
                        .find(|p| p.fname() == &class)
                        .unwrap_or_else(|| panic!("Unable to find property class: {}", class))
                    ))
                }
            }
        };

        match class {
            StructClass::Class(class) => {
                map(many_till(parse_struct_attribute(file, container, class), tag_struct_end(file)), |(properties, _)| {
                    ObjectProperty::Struct(Some(class.clone()), ScriptObject {
                        attributes: properties.into_iter().collect()
                    })
                })(i)
            },
            StructClass::Intrinsic(intrinsic) => {
                

                match intrinsic.as_ref() {
                    "Vector" => {
                        let (i, data) = count(le_f32, 3)(i)?;
                        Ok((i, ObjectProperty::Vector(data.try_into().unwrap())))
                    },
                    "Vector2D" => {
                        let (i, data) = count(le_f32, 2)(i)?;
                        Ok((i, ObjectProperty::Vector2D(data.try_into().unwrap())))
                    },
                    "Rotator" => {
                        let (i, data) = count(le_f32, 3)(i)?;
                        Ok((i, ObjectProperty::Vector(data.try_into().unwrap())))
                    },
                    "Guid" => {
                        let (i, data) = take(16_usize)(i)?;
                        Ok((i, ObjectProperty::Guid(Uuid::from_bytes_le(data.try_into().unwrap()))))
                    },
                    "Color" => {
                        let (i, val) = le_u32(i)?;
                        Ok((i, ObjectProperty::Color(val)))
                    },
                    "Box" => {
                        let (i, min) = count(le_f32, 3)(i)?;
                        let (i, max) = count(le_f32, 3)(i)?;
                        let (i, is_valid) = le_u8(i)?;
                        Ok((i, ObjectProperty::Box {
                            min: min.try_into().unwrap(),
                            max: max.try_into().unwrap(),
                            is_valid,
                        }))
                    },
                    _ => unimplemented!("Unimplemented intrinsic: {}", intrinsic),
                }
            }
        }
    }
}

#[async_trait]
impl DeserializeUnrealObject for ScriptObject {
    async fn deserialize(object: &ObjectRef, container: &Container, data: &[u8]) -> UPKResult<Self> {
        let (data, marker) = be_u32::<_, Error<_>>(data)?;
        let data = if marker & 0xFFFFFF != 0 && marker != 0xFFFFFFFF {
            &data[22..]
        } else {
            data
        };

        let file = object.package().unwrap();
        let script_class = object.class().clone();

        println!("Deserialize object: {}", object.fully_qualified_name());
        println!("Flags: {:?}", object.flags());

        let (_, res) = map(many_till(parse_struct_attribute(&file, container, &script_class), tag_struct_end(&file)), |(properties, _)| {
            Self {
                attributes: properties.into_iter().collect()
            }
        })(data)?;

        Ok(res)
    }
}