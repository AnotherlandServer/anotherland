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

use std::{any::{type_name, Any}, collections::HashMap, fmt::Debug, io};

use bitstream_io::ByteWrite;
use nom::{error::{context, VerboseError}, multi, number, IResult};

use crate::{param::{GenericParam, Param}, Attribute, AttributeInfo, Class, ParamFlag, Value};

pub trait GenericParamSet: Debug + Send + Sync {
    fn class(&self) -> Class;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_hash_map(&self) -> HashMap<&'static str, Value>;

    fn set_param(&mut self, name: &str, value: Value) -> Option<Value>;
    fn get_param(&self, name: &str) -> Option<&Value>;

    fn clear_changes(&mut self);
    fn changes(&self) -> Vec<(&'static dyn AttributeInfo, Value)>;

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, &'a Value)> + 'a>;
    fn drain<'a>(&'a mut self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, Value)> + 'a>;
} 

#[derive(Clone)]
pub struct ParamSet<T: Attribute> {
    pub(super) values: HashMap<T, Param<T>>,
}

impl <T: Attribute> Default for ParamSet<T> {
    fn default() -> Self { Self::new() }
}

impl <T: Attribute> ParamSet <T> {
    pub fn new() -> Self { Self { values: HashMap::new() }}
    pub fn insert<P>(&mut self, attr: T, value: P) -> Option<Param<T>>
        where P: Into<Value>
    {
        self.values.insert(attr, Param::new(attr, value.into()))
    }
    
    pub fn remove(&mut self, key: &T) -> Option<Param<T>> {
        self.values.remove(key)
    }

    pub fn get(&self, key: &T) -> Option<&Param<T>> {
        self.values.get(key)
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Param<T>> {
        self.values.get_mut(key)
    }

    pub fn extend(&mut self, other: ParamSet<T>) {
        for (attr, param) in other.values.iter() {
            self.values.insert(
                *attr, 
                Param::new(*attr, param.value().clone())
            );
        }
    }

    pub(crate) fn read_values<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Param<T>>, VerboseError<&'a [u8]>> {
        context(type_name::<T>(), |i| -> IResult<&'a [u8], Vec<Param<T>>, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let (i, values) = context("Attributes", multi::count(Param::from_slice, count as usize))(i)?;

            Ok((i, values))
        })(i)
    }

    pub fn from_slice(i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (i, params) = Self::read_values(i)?;

        Ok((i, Self{
            values: params
            .into_iter()
            .map(|v| (*v.attribute(), v))
            .collect()
        }))
    }

    pub fn from_client_slice(i: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (i, params) = Self::read_values(i)?;

        Ok((i, Self{    	
            values: params
            .into_iter()
            .filter(|v| 
                !v.attribute().flags().contains(&ParamFlag::ExcludeFromClient) &&
                !v.attribute().flags().contains(&ParamFlag::ClientUnknown) &&
                !v.attribute().flags().contains(&ParamFlag::NodeOwn) &&
                !v.attribute().flags().contains(&ParamFlag::ServerOwn)
            )
            .map(|v| (*v.attribute(), v))
            .collect()
        }))
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<(), io::Error> 
        where W: ByteWrite
    {
        writer.write(1u8)?;

        let filtered_params: Vec<_> = self.values.iter().filter(|(_, a)| !a.should_skip())
            .collect();
        writer.write(filtered_params.len() as u16)?;

        
        for (_, v) in filtered_params {
            v.write(writer)?;
        }

        Ok(())
    }

    pub fn write_to_client<W>(&self, writer: &mut W) -> Result<(), io::Error> 
        where W: ByteWrite
    {
        let filtered_params: Vec<_> = self.values.iter()
        .filter(|(&attribute, _)| !attribute.has_flag(&ParamFlag::ClientUnknown))
        .collect();

        writer.write(1u8)?;
        writer.write(filtered_params.len() as u16)?;

        for (_, v) in filtered_params {
            v.write(writer)?;
        }

        Ok(())
    }

    pub fn as_hash_map(&self) -> HashMap<&'static str, Value> {
        self.values.iter()
            .map(|(a, v)| (a.name(), v.value().clone()))
            .collect()
    }

    pub fn changes(&self) -> Vec<Param<T>> {
        self.values.iter()
            .filter(|&(_, p)| p.is_changed())
            .map(|(_, p)| p.clone())
            .collect()
    }
}

impl <T: Attribute> Debug for ParamSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("ParamSet");

        for (attrib, param) in self.values.iter() {
            debug_struct.field(attrib.name(), param.value());
        }

        debug_struct.finish()
    }
}

impl <T: Attribute> GenericParamSet for ParamSet<T> {
    fn class(&self) -> Class {
        <T as Attribute>::class()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn as_hash_map(&self) -> HashMap<&'static str, Value> {
        self.as_hash_map()
    }

    fn set_param(&mut self, name: &str, value: Value) -> Option<Value> {
        if let Ok(attr) = name.parse() {
            self.values.insert(attr, Param::new(attr, value))
                .map(|p| p.take())
        } else {
            None
        }
    }

    fn get_param(&self, name: &str) -> Option<&Value> {
        if let Ok(attr) = name.parse() {
            self.values.get(&attr)
                .map(|p| p.value())
        } else {
            None
        }
    }
    
    fn clear_changes(&mut self) {
        for p in self.values.values_mut() {
            p.clear_state();
        }
    }

    fn changes(&self) -> Vec<(&'static dyn AttributeInfo, Value)> {
        self.changes()
            .into_iter()
            .map(|p| (p.attribute().static_info(), p.take()))
            .collect()
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, &'a Value)> + 'a> {
        Box::new(
            self.values.values()
                .map(|p| (p.attribute().static_info(), p.value()))
        )
    }

    fn drain<'a>(&'a mut self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, Value)> + 'a> {
        Box::new(
            self.values.drain()
                .map(|(a, p)| (a.static_info(), p.take()))
        )
    }
}