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

use std::collections::{hash_map::Iter, HashMap};
use nom::{combinator::map, error::Error, number::complete::le_u64};
use async_trait::async_trait;
use crate::{Container, DeserializeUnrealObject, FName, Flags, LocalObjectIndexRef, ObjectRef, UPKResult, CLASS};

#[derive(Debug)]
pub struct ScriptClass {
    attributes: HashMap<FName, ObjectRef>,
}

impl ScriptClass {
    pub fn attribs(&self) -> Iter<'_, FName, ObjectRef> {
        self.attributes.iter()
    }

    pub fn attrib(&self, name: &FName) -> Option<&ObjectRef> {
        self.attributes.get(name)
    }
}

#[async_trait]
impl DeserializeUnrealObject for ScriptClass {
    async fn deserialize(object: &ObjectRef, container: &Container, i: &[u8]) -> UPKResult<Self> {
        let mut attributes = HashMap::new();

        if object.flags().contains(Flags::HAS_STACK) {
            unimplemented!();
        }

        let (i, _) = le_u64::<_, Error<_>>(i)?;
        let (_, super_struct) = map(le_u64::<_, Error<_>>, |idx| LocalObjectIndexRef::from_idx(idx as i32))(i)?;

        let super_struct = container
            .resolve_object(object.package().unwrap(), super_struct)
            .unwrap_or(CLASS.clone());

        // recursively parse structs for super structs, if required
        if !super_struct.has_data() {
            let script_class = container.deserialize::<ScriptClass>(&super_struct).await?;
            super_struct.set_data(script_class);
        }

        // add all the superstructs attributes
        if super_struct.is::<ScriptClass>() {
            attributes.extend(super_struct.data::<ScriptClass>()
                .unwrap()
                .attributes
                .iter()
                .map(|(name, attrib)| (name.clone(), attrib.clone()))
            );
        }

        // add attributes of this class definition
        for child in object.children().iter() {
            attributes.insert(child.fname().clone(), child.clone());
        }

        Ok(ScriptClass {
            attributes
        })
    }
}