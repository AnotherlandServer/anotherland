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
use nom::{bytes::complete::take, combinator::{fail, map}, multi::{count, many_till}, number::complete::{le_f32, le_i32, le_u32, le_u64}, IResult};
use async_trait::async_trait;
use uuid::Uuid;
use crate::{types::StructProperty, Container, DeserializeUnrealObject, FName, LocalObjectIndexRef, ObjectRef, PackageFile, UPKResult};

use super::ScriptClass;

#[derive(Debug)]
pub struct ScriptObject {
    attributes: HashMap<FName, ObjectProperty>,
}

impl ScriptObject {
    pub fn attribs(&self) -> Iter<FName, ObjectProperty> {
        self.attributes.iter()
    }
}

#[derive(Debug)]
pub enum ObjectProperty {
    None,
    Array(Vec<ObjectProperty>),
    Bool(bool),
    Byte(u8),
    Class(ObjectRef),
    Component(ObjectRef),
    Delegate(ObjectRef),
    Float(f32),
    Interface(ObjectRef),
    Int(i32),
    Map(ObjectRef),
    Name(FName),
    Object(ObjectRef),
    Struct(Option<ObjectRef>, HashMap<FName, ObjectProperty>),
    String(String),
    Vector([f32; 3]),
    Guid(Uuid),
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
            _ => unimplemented!("unimplemented property type: {}", attribute.class().name()),
        }
    }
}

fn parse_name<'a>(file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        // attribute data size
        let (i, _) = le_u64(i)?;
        let (i, name) = le_u64(i)?;

        Ok((i, ObjectProperty::Name(file.lookup_name(name as usize))))
    }
}

fn parse_float<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        // attribute data size
        let (i, _) = le_u64(i)?;
        let (i, val) = le_f32(i)?;

        Ok((i, ObjectProperty::Float(val)))
    }
}

fn parse_string<'a>(_file: &'a Arc<PackageFile>, _container: &'a Container, _object: &'a ObjectRef, _attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        // attribute data size
        let (i, _) = le_u64(i)?;
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
        // attribute data size
        let (i, _) = le_u64(i)?;

        let (i, idx) = le_i32(i)?;
        Ok((i, ObjectProperty::Object(
            container.resolve_object(file.to_owned(), LocalObjectIndexRef::from_idx(idx)).unwrap()
        )))
    }
}

fn parse_array<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef, attribute: &'a ObjectRef)
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], ObjectProperty> 
{
    move |i: &[u8]| {
        let child = attribute.children().iter().next().unwrap();

        // attribute data size
        let (i, _) = le_u64(i)?;

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

fn parse_struct_attribute<'a>(file: &'a Arc<PackageFile>, container: &'a Container, object: &'a ObjectRef, ) 
    -> impl FnMut(&'a [u8]) 
    -> IResult<&'a [u8], (FName, ObjectProperty)> 
{
    move |i: &[u8]| {
        let (i, name) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;

        let attribute_def = object.data::<ScriptClass>()
            .unwrap()
            .attrib(&name)
            .unwrap();

        let (i, datatype) = map(le_u64, |idx| {
            file.lookup_name(idx as usize)
        })(i)?;

        let attribute_class = container.lookup_object(&format!("Core/{}", datatype)).unwrap();
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
            // attribute data size
            let (i, _) = le_u64(i)?;

            let (i, class) = map(le_u64, |idx| {
                file.lookup_name(idx as usize)
            })(i)?;

            match class.as_ref() {
                "Vector" => (i, StructClass::Intrinsic(class)),
                "Guid" => (i, StructClass::Intrinsic(class)),
                _ => {
                    // lookup struct property class
                    (i, StructClass::Class(
                        object
                        .children()
                        .iter()
                        .find(|p| p.fname() == &class)
                        .unwrap()
                    ))
                }
            }
        };

        match class {
            StructClass::Class(class) => {
                map(many_till(parse_struct_attribute(file, container, class), tag_struct_end(file)), |(properties, _)| {
                    ObjectProperty::Struct(Some(class.clone()), properties.into_iter().collect())
                })(i)
            },
            StructClass::Intrinsic(intrinsic) => {
                

                match intrinsic.as_ref() {
                    "Vector" => {
                        let (i, data) = count(le_f32, 3)(i)?;
                        Ok((i, ObjectProperty::Vector(data.try_into().unwrap())))
                    },
                    "Guid" => {
                        let (i, data) = take(16_usize)(i)?;
                        Ok((i, ObjectProperty::Guid(Uuid::from_bytes_le(data.try_into().unwrap()))))
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
        let data = &data[26..];
        let file = object.package().unwrap();
        let script_class = object.class().clone();

        let (_, res) = map(many_till(parse_struct_attribute(&file, container, &script_class), tag_struct_end(&file)), |(properties, _)| {
            Self {
                attributes: properties.into_iter().collect()
            }
        })(data)?;

        Ok(res)
    }
}