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

use std::{collections::{hash_map::Iter, HashMap}, sync::Arc};
use glam::{Mat4, Vec4};
use log::debug;
use nom::{bytes::complete::take, combinator::{fail, map, map_res}, error::{context, ErrorKind, FromExternalError, VerboseError}, multi::{count, many_till}, number::complete::{le_f32, le_i32, le_u16, le_u32, le_u64, le_u8}, sequence::tuple, IResult};
use async_trait::async_trait;
use uuid::Uuid;
use crate::{types::StructProperty, Container, DeserializeUnrealObject, FName, LocalObjectIndexRef, ObjectRef, PackageFile, UPKError, UPKResult};

use super::ScriptClass;

#[derive(Debug)]
pub struct ScriptObject {
    attributes: HashMap<FName, ObjectProperty>,
}

impl ScriptObject {
    pub fn attribs(&self) -> Iter<'_, FName, ObjectProperty> {
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
    Matrix(Mat4),
    Box {
        min: [f32; 3],
        max: [f32; 3],
        is_valid: u8,
    },
    Plane([f32; 4]),
}

fn parse_attribute<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, object: ObjectRef, attribute: ObjectRef, class_override: Option<ObjectRef>, in_array: bool)
-> impl FnMut(&'_ [u8]) 
-> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let class_name = if let Some(class_override) = &class_override { 
            class_override.name() 
        } else { 
            attribute.class().name() 
        };

        debug!("Parsing attribute: {} (class: {})", attribute.fname(), class_name);

        match class_name {
            "StructProperty" => parse_struct(file.clone(), container, object.clone(), attribute.clone(), in_array)(i),
            "ArrayProperty" => parse_array(file.clone(), container, object.clone(), attribute.clone())(i),
            "ComponentProperty" => parse_component(file.clone(), container, object.clone(), attribute.clone())(i),
            "ObjectProperty" => parse_object(file.clone(), container)(i),
            "StrProperty" => parse_string(object.clone(), attribute.clone())(i),
            "FloatProperty" => parse_float(object.clone(), attribute.clone())(i),
            "NameProperty" => parse_name(&file)(i),
            "ByteProperty" => parse_bytes(file.clone(), object.clone(), attribute.clone())(i),
            "BoolProperty" => parse_bool(object.clone(), attribute.clone())(i),
            "IntProperty" => parse_int(object.clone(), attribute.clone())(i),
            "ClassProperty" => parse_class(file.clone(), container, object.clone(), attribute.clone())(i),
            _ => unimplemented!("unimplemented property type: {}", attribute.class().name()),
        }
    }
}

fn parse_class<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, _object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let (i, idx) = le_i32(i)?;
        Ok((i, ObjectProperty::Class(
            container.resolve_object(file.clone(), LocalObjectIndexRef::from_idx(idx)).unwrap()
        )))
    }
}

fn parse_int(_object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> 
{
    move |i: &'_ [u8]| {
        let (i, val) = le_i32(i)?;

        Ok((i, ObjectProperty::Int(val)))
    }
}


fn parse_bool(_object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> 
{
    move |i: &'_ [u8]| {
        let (i, val) = le_u8(i)?;

        Ok((i, ObjectProperty::Bool(val != 0)))
    }
}

fn parse_bytes(file: Arc<PackageFile>, _object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> 
{
    move |i: &'_ [u8]| {
        // todo: upk seems to encode enums as ByteProperty. how can we detect if the attribute does not decode an enum?

        let (i, enum_name) = map(tuple((le_u32, le_u32)), |(idx, _)| {
            file.lookup_name(idx as usize)
        })(i)?;
        
        if &*enum_name == "None" {
            let (i, val) = le_u8(i)?;

            Ok((i, ObjectProperty::Enum(enum_name, Box::new(ObjectProperty::Int(val as i32)))))
        } else {
            let (i, val_name) = map(tuple((le_u32, le_u32)), |(idx, _)| {
                file.lookup_name(idx as usize)
            })(i)?;

            Ok((i, ObjectProperty::Enum(enum_name, Box::new(ObjectProperty::Name(val_name)))))
        }
    }
}

pub fn parse_name(file: &Arc<PackageFile>)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + Copy 
{
    move |i: &'_ [u8]| {
        let (i, (name, _)) = tuple((le_u32, le_u32))(i)?;
        Ok((i, ObjectProperty::Name(file.lookup_name(name as usize))))
    }
}

fn parse_float(_object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> 
{
    move |i: &'_ [u8]| {
        let (i, val) = le_f32(i)?;

        Ok((i, ObjectProperty::Float(val)))
    }
}

fn parse_string(_object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> 
{
    move |i: &'_ [u8]| {
        let (i, len) = le_i32(i)?;

        let (len, is_unicode) = if len >= 0 {
            (len as usize, false)
        } else {
            (-len as usize, true)
        };

        let (i, string) = if len > 0 {
            if is_unicode {
                let (i, str_bytes) = count(le_u16, len )(i)?;
                (i, String::from_utf16_lossy(&str_bytes[..len-1]))
            } else {
                let (i, str_bytes) = take(len)(i)?;
                (i, String::from_utf8_lossy(&str_bytes[..len-1]).to_string())
            }
        } else {
            (i, String::new())
        };

        Ok((i, ObjectProperty::String(string)))
    }
}

pub fn parse_component<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, _object: ObjectRef, _attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let (i, idx) = le_i32(i)?;
        Ok((i, ObjectProperty::Component(
            container.resolve_object(file.clone(), LocalObjectIndexRef::from_idx(idx)).unwrap()
        )))
    }
}

pub fn parse_object<'ctx>(file: Arc<PackageFile>, container: &'ctx Container)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let (i, idx) = le_i32(i)?;

        debug!("Idx: {idx}");

        if idx == 0 {
            Ok((i, ObjectProperty::None))
        } else {
            Ok((i, ObjectProperty::Object(
                container.resolve_object(file.clone(), LocalObjectIndexRef::from_idx(idx)).unwrap()
            )))
        }
    }
}

fn parse_array<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, object: ObjectRef, attribute: ObjectRef)
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let child = attribute.children().iter().next().unwrap();

        // element count
        let (i, elements) = le_u32(i)?;

        map(
            count(parse_attribute(file.clone(), container, object.clone(), child.clone(), None, true), elements as usize),
            |elements| {
                ObjectProperty::Array(elements)
            }
        )(i)
    }
}

fn tag_struct_end(file: Arc<PackageFile>) 
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], (), VerboseError<&[u8]>>
{
    move |i: &'_ [u8]| {
        let (i, name) = context("attribute name", map_res(tuple((le_u32, le_u32)), |(idx, _)| {
            file.try_lookup_name(idx as usize)
                .ok_or(UPKError::Custom(format!("Failed to lookup attribute name {idx}")))
        }))(i)?;

        if &*name == "None" {
            Ok((i, ()))
        } else {
            fail(i)
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse_struct_attribute<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, object: ObjectRef) 
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], (FName, ObjectProperty), VerboseError<&[u8]>> + 'ctx
{
    move |i: &'_ [u8]| {
        let (i, name) = context("attribute name", map_res(tuple((le_u32, le_u32)), |(idx, _)| {
            file.try_lookup_name(idx as usize)
                .ok_or(UPKError::Custom(format!("Failed to lookup attribute name {idx}")))
        }))(i)?;

        debug!("{name}");

        let attribute_def = object.data::<ScriptClass>()
            .unwrap()
            .attrib(&name)
            .cloned()
            .ok_or_else(|| format!("Failed to find attribute {name} in class {}", object.name()))
            .map_err(|e| nom::Err::Error(VerboseError::from_external_error(i, ErrorKind::MapRes, e)))?;

        let (i, datatype) = context("datatype name", map_res(tuple((le_u32, le_u32)), |(idx, _)| {
            file.try_lookup_name(idx as usize)
                .ok_or(UPKError::Custom(format!("Failed to lookup datatype name {idx}")))
        }))(i)?;

        // attribute data size
        let (i, _) = le_u64(i)?;

        let attribute_class = container.lookup_object(&format!("Class:{datatype}"))
            .ok_or(UPKError::Custom(format!("Failed to lookup class {datatype}")))
            .map_err(|e| nom::Err::Error(VerboseError::from_external_error(i, ErrorKind::MapRes, e)))?;
        let (i, attribute) = context("attribute",
            parse_attribute(file.clone(), container, object.clone(), attribute_def, Some(attribute_class), false)
        )(i)?;

        Ok((i, (name, attribute)))
    }
}

fn parse_struct<'ctx>(file: Arc<PackageFile>, container: &'ctx Container, object: ObjectRef, attrib: ObjectRef, in_array: bool) 
    -> impl FnMut(&'_ [u8]) 
    -> IResult<&'_ [u8], ObjectProperty, VerboseError<&[u8]>> + 'ctx
{
    enum StructClass {
        Intrinsic(FName),
        Class(ObjectRef)
    }

    move |i: &'_ [u8]| {
        let (i, class_name) = if in_array {
            if let Some(struct_property) = attrib.data::<StructProperty>() {
                debug!("Array struct class: {}", struct_property.class().name());

                (i, struct_property.class().fname().clone())
            } else {
                panic!()
            }
        } else {
            let (i, class) = map(tuple((le_u32, le_u32)), |(idx, _)| {
                file.lookup_name(idx as usize)
            })(i)?;

            debug!("Struct class: {class}");

           (i, class)
        };

        let class = match class_name.as_ref() {
            "Vector" => StructClass::Intrinsic(class_name),
            "Vector2D" => StructClass::Intrinsic(class_name),
            "Rotator" => StructClass::Intrinsic(class_name),
            "Guid" => StructClass::Intrinsic(class_name),
            "Color" => StructClass::Intrinsic(class_name),
            "Box" => StructClass::Intrinsic(class_name),
            "Plane" => StructClass::Intrinsic(class_name),
            "Matrix" => StructClass::Intrinsic(class_name), 
            "ViewTargetTransitionParams" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:Camera/ViewTargetTransitionParams").unwrap()
                )
            },
            "LightingChannelContainer" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:LightComponent/LightingChannelContainer").unwrap()
                )
            },
            "RBCollisionChannelContainer" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:PrimitiveComponent/RBCollisionChannelContainer").unwrap()
                )
            },
            "GeomSelection" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:Brush/GeomSelection").unwrap()
                )
            },
            "RUTimestamp" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:RUDecoVolume/RUTimestamp").unwrap()
                )
            },
            "KAggregateGeom" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:KMeshProps/KAggregateGeom").unwrap()
                )
            },
            "KBoxElem" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:KMeshProps/KBoxElem").unwrap()
                )
            },
            "KConvexElem" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:KMeshProps/KConvexElem").unwrap()
                )
            },
            "KSphereElem" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:KMeshProps/KSphereElem").unwrap()
                )
            },
            "KSphylElem" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:KMeshProps/KSphylElem").unwrap()
                )
            },
            "TerrainDecorationInstance" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:Terrain/TerrainDecorationInstance").unwrap()
                )
            },
            "LevelFixedBounds" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:LevelStreaming/LevelFixedBounds").unwrap()
                )
            },
            "MapViewData" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:LevelStreaming/MapViewData").unwrap()
                )
            },
            "LightmassPrimitiveSettings" => {
                StructClass::Class(
                    container.lookup_object("ScriptStruct:EngineTypes/LightmassPrimitiveSettings").unwrap()
                )
            },
            "TextureAddress" => {
                 StructClass::Class(
                    container.lookup_object("ScriptStruct:Texture/TextureAddress").unwrap()
                )
            },
            _ => {
                // lookup struct property class
                StructClass::Class(
                    object
                    .children()
                    .iter()
                    .find(|p| p.fname() == &class_name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Unable to find property class: {class_name}"))
                )
            }
        };

        match class {
            StructClass::Class(class) => {
                map(many_till(parse_struct_attribute(file.clone(), container, class.clone()), tag_struct_end(file.clone())), |(properties, _)| {
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
                    "Plane" => {
                        let (i, data) = count(le_f32, 4)(i)?;
                        Ok((i, ObjectProperty::Plane(data.try_into().unwrap())))
                    },
                    "Matrix" => {
                        let (i, data) = count(le_f32, 16)(i)?;
                        Ok((i, ObjectProperty::Matrix(Mat4 {
                            x_axis: Vec4::from_slice(&data[0..4]),
                            y_axis: Vec4::from_slice(&data[4..8]),
                            z_axis: Vec4::from_slice(&data[8..12]),
                            w_axis: Vec4::from_slice(&data[12..16]),
                        })))
                    },
                    _ => unimplemented!("Unimplemented intrinsic: {}", intrinsic),
                }
            }
        }
    }
}

#[async_trait]
impl DeserializeUnrealObject for ScriptObject {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let file = object.package().unwrap();
        let script_class = object.class().clone();

        let (data, netindex) = le_i32::<_, VerboseError<_>>(data)?;

        debug!("Deserialize object: {}", object.fully_qualified_name());
        debug!("NetIndex: {}", netindex);
        debug!("Class: {}", script_class.fully_qualified_name());
        debug!("Flags: {:?}", object.flags());

        // Skip until we read an attribute successfully 
        let mut offset = 0;
        while offset < data.len() {
            if 
                tag_struct_end(file.clone())(&data[offset..]).is_ok() || 
                parse_struct_attribute(file.clone(), container, script_class.clone())(&data[offset..]).is_ok() 
            {
                break;
            }

            offset += 1; // Skip 1 byte
        }

        debug!("Skipping {} bytes", offset);

        let (data, res) = map(many_till(parse_struct_attribute(file.clone(), container, script_class), tag_struct_end(file.clone())), |(properties, _)| {
            Self {
                attributes: properties.into_iter().collect()
            }
        })(&data[offset..])?;

        Ok((data, res))
    }
}