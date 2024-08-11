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

use std::any::Any;
use std::{collections::HashMap, any::type_name, io};
use std::fmt::Debug;

use bitstream_io::ByteWrite;
use log::warn;
use nom::{combinator::fail, error::{VerboseError, context}, multi, number, IResult};

use crate::{ClassId, DynParamSet, Param, ParamAttrib, ParamFlag, ParamSetBox};

#[derive(Clone)]
pub struct ParamSet<T: ParamAttrib> {
    pub(super) params: HashMap<T, Param>,
}

impl <T: ParamAttrib> Default for ParamSet<T> {
    fn default() -> Self {
        Self { params: Default::default() }
    }
}

impl <T: ParamAttrib>ParamSet<T> {
    pub fn new() -> Self {
        Self {
            params: HashMap::new()
        }
    }

    pub fn insert<P>(&mut self, key: T, value: P) -> Option<Param>
        where P: Into<Param>
    {
        self.params.insert(key, value.into())
    }

    pub fn remove(&mut self, key: &T) -> Option<Param> {
        self.params.remove(key)
    }

    pub fn get<'a>(&'a self, key: &T) -> Option<&'a Param> {
        self.params.get(key)
    }

    pub fn get_mut<'a>(&'a mut self, key: &T) -> Option<&'a mut Param> {
        self.params.get_mut(key)
    }

    pub fn extend(&mut self, other: ParamSet<T>) {
        self.params.extend(other.params);
    }

    fn read_attribute(i: &[u8]) -> IResult<&[u8], (T, Param), VerboseError<&[u8]>>
    {
        let (i, attribute_id) = context("Attribute Id", number::complete::le_u16)(i)?;
        let attribute: T = match attribute_id.try_into() {
            Ok(attribute) => attribute,
            Err(_) => {
                warn!("failed to parse attribute id {}", attribute_id);
                return fail(i);
            },
        };

        let (i, param) = Param::read(i, attribute.flags())?;
        Ok((i, (attribute, param)))
    }
    
    pub fn read<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>>
    {
        context(type_name::<T>(), |i| -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
            let (i, _) = context("Version", number::complete::le_u8)(i)?;
            let (i, count) = context("Param Count", number::complete::le_u16)(i)?;
            let (i, attribs) = context("Attributes", multi::count(Self::read_attribute, count as usize))(i)?;

            Ok((i, Self{
                params: attribs
                .into_iter()
                .collect()
            }))
        })(i)
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<(), io::Error> 
        where W: ByteWrite
    {
        writer.write(1u8)?;

        let filtered_params: Vec<_> = self.params.iter().filter(|(_, a)| !a.should_skip())
            .collect();
        writer.write(filtered_params.len() as u16)?;

        
        for (attribute, a) in filtered_params {
            writer.write(attribute.id())?;
            a.write(writer)?;
        }

        Ok(())
    }

    pub fn write_to_client<W>(&self, writer: &mut W) -> Result<(), io::Error> 
        where W: ByteWrite
    {
        let filtered_params: Vec<_> = self.params.iter()
        .filter(|(attribute, a)| !attribute.has_flag(&ParamFlag::ClientUnknown))
        .collect();

        writer.write(1u8)?;
        writer.write(filtered_params.len() as u16)?;

        for (attribute, a) in filtered_params {
            writer.write(attribute.id())?;
            a.write(writer)?;
        }

        Ok(())
    }

    pub fn into_box(self) -> ParamSetBox {
        ParamSetBox::new(self)
    }

    pub fn diff(&self, other: &ParamSet<T>) -> ParamSet<T> {
        let mut diff = ParamSet::new();

        for (key, val) in self.params.iter() {
            if let Some(other_val) = other.get(key) {
                if other_val != val {
                    diff.insert(*key, val.clone());
                }
            } else {
                diff.insert(*key, val.clone());
            }
        }

        diff
    }

    pub fn as_hash_map(&self) -> HashMap<String, Param> {
        self.params.iter()
            .map(|(a, v)| (a.name().to_string(), v.clone()))
            .collect()
    }
}

impl <T: ParamAttrib> Debug for ParamSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("ParamSet");

        for (attrib, param) in self.params.iter() {
            debug_struct.field(attrib.name(), param);
        }

        debug_struct.finish()
    }
}

impl <T: ParamAttrib> DynParamSet for ParamSet<T> {
    fn class_id(&self) -> ClassId { T::class_id() }
    fn as_any(&self) -> &dyn Any { self}
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn len(&self) -> usize {
        self.params.len()
    }

    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    fn as_hash_map(&self) -> HashMap<String, Param> {
        ParamSet::<T>::as_hash_map(self)
    }

    fn set_param(&mut self, name: &str, param: Param) -> Option<Param> {
        if let Ok(attrib) = name.parse::<T>() {
            self.params.insert(attrib, param)
        } else {
            None
        }
    }

    fn get_param(&self, name: &str) -> Option<&Param> {
        if let Ok(attrib) = name.parse::<T>() {
            self.params.get(&attrib)
        } else {
            None
        }
    }
}