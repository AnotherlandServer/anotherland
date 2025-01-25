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

use std::{fmt::Debug, sync::Arc};

use bevy::{prelude::Component, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

use crate::{Attribute, AttributeInfo, Class, GenericParamSet, ParamError, ParamFlag, ParamResult, ParamSet, ParamWriter, Value};

#[derive(Serialize, Deserialize, Component)]
#[serde(transparent)]
pub struct GameObjectData {
    #[serde(skip)]
    parent: Option<Arc<GameObjectData>>,
    instance: Box<dyn GenericParamSet>,
}

impl GameObjectData {
    pub fn new<T: Attribute + 'static>() -> Self {
        Self {
            parent: None,
            instance: Box::new(ParamSet::<T>::new())
        }
    }

    pub fn new_for_class(class: Class) -> Self {
        Self {
            parent: None,
            instance: class.create_param_set(vec![])
        }
    }

    pub fn from_set<T: Attribute + 'static>(set: ParamSet<T>) -> Self {
        Self {
            parent: None,
            instance: Box::new(set)
        }
    }

    pub fn from_generic_set(set: Box<dyn GenericParamSet>) -> Self {
        Self {
            parent: None,
            instance: set
        }
    }

    pub fn instantiate(parent: &Arc<GameObjectData>) -> Self {
        Self {
            parent: Some(parent.clone()),
            instance: parent.class().create_param_set(vec![]),
        }
    }

    pub fn set_parent(&mut self, parent: Option<Arc<GameObjectData>>) {
        self.parent = parent;
    }

    pub fn class(&self) -> Class { self.instance.class() }

    pub fn get<'a, T: Attribute, V>(&'a self, attr: T) -> ParamResult<&'a V>
        where 
            &'a V: TryFrom<&'a Value>,
            <&'a V as TryFrom<&'a Value>>::Error: Into<ParamError>
    {
        self.get_named(attr.name())
    }

    pub fn set<T: Attribute, V: Into<Value>>(&mut self, attr: T, val: V) -> Option<Value> {
        self.set_named(attr.name(), val.into())
    }

    pub fn get_named<'a, V>(&'a self, attr: &str) -> ParamResult<&'a V>
        where 
            &'a V: TryFrom<&'a Value>,
            <&'a V as TryFrom<&'a Value>>::Error: Into<ParamError>

    {
        if let Some(value) = self.instance.get_param(attr) {
            value.try_into()
                .map_err(|e: <&'a V as TryFrom<&'a Value>>::Error| e.into())
        }  else if 
            let Some(parent) = self.parent.as_ref() &&
            let Ok(value) = parent.get_named(attr)
        {
            Ok(value)
        } else {
            self.instance.class().get_attribute(attr)
                .ok_or(ParamError::UnknownAttributeName)?
                .default()
                .try_into()
                .map_err(|e: <&'a V as TryFrom<&'a Value>>::Error| e.into())
        }
    }

    pub fn set_named<V: Into<Value>>(&mut self, attr: &str, val: V) -> Option<Value> {
        let val  = val.into();

        if 
            let Some(parent) = self.parent.as_ref() &&
            parent.get_named::<Value>(attr).unwrap() == &val
        {
            self.instance.remove_param(attr)
        } else {
            self.instance.set_param(attr, val)
        }
    }

    pub fn apply(&mut self, set: &mut dyn GenericParamSet) {
        for (attr, value) in set.drain() {
            self.set_named(attr.name(), value);
        }
    }

    pub fn clear_changes(&mut self) {
        self.instance.clear_changes();
    }

    pub fn changes(&self) -> Box<dyn Iterator<Item = (&'static dyn AttributeInfo, Value)>> {
        self.instance.changes()
    }

    pub fn merge(&mut self, mut other: GameObjectData) {
        for (attr, value) in other.instance.drain() {
            self.instance.set_param(attr.name(), value);
        }
    }

    pub fn persistent_value_set(&self) -> Box<dyn GenericParamSet> {
        let values = self.instance.values()
            .filter(|&(a,_)| a.has_flag(&ParamFlag::Persistent))
            .map(|(a, v)| (a, v.clone()))
            .collect();

        self.instance.class().create_param_set(values)
    }

    pub fn as_set(&self) -> &dyn GenericParamSet {
        self.instance.as_ref()
    }

    fn write<W: bitstream_io::ByteWrite>(&self,  writer: &mut W, filter: fn(&dyn AttributeInfo, &Value) -> bool) -> Result<(), std::io::Error> {
        let mut params: HashMap<&dyn AttributeInfo, &Value> = HashMap::new();

        if let Some(parent) = self.parent.as_ref() {
            parent.as_set()
                .values()
                .filter(|&(a,v)| filter(a, v))
                .for_each(|(a,v)| { params.insert(a, v); });
        }

        self.instance
            .values()
            .filter(|&(a,v)| filter(a, v))
            .for_each(|(a,v)| { params.insert(a, v); });

        writer.write(1u8)?;
        writer.write(params.len() as u16)?;

        for (attr, value) in params {
            writer.write(attr.id())?;
            value.write(writer)?;
        }

        Ok(())
    }
}

impl Clone for GameObjectData {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            instance: self.instance.dyn_clone()
        }
    }
}

impl Debug for GameObjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.instance.fmt(f)
    }
}

impl ParamWriter for GameObjectData {
    fn write<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.write(writer, |_, v| !v.should_skip())
    }

    fn write_to_client<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.write(writer, |a, v| {
            !a.has_flag(&ParamFlag::ClientUnknown) &&
            !a.has_flag(&ParamFlag::ClientPrivileged) &&
            !v.should_skip()
        })
    }

    fn write_to_privileged_client<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.write(writer, |a, v| {
            (!a.has_flag(&ParamFlag::ClientUnknown) ||
            a.has_flag(&ParamFlag::ClientPrivileged)) &&
            !v.should_skip()
        })
    }
}

impl TryFrom<serde_json::Value> for GameObjectData {
    type Error = serde_json::Error;
    
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

impl TryFrom<GameObjectData> for serde_json::Value {
    type Error = serde_json::Error;

    fn try_from(value: GameObjectData) -> Result<Self, Self::Error> {
        serde_json::to_value(value)
    }
}

#[cfg(test)]
mod test {
    use crate::{GameObjectData, Player};

    #[test]
    fn serialization() {
        let mut obj = GameObjectData::new::<Player>();
        obj.set(Player::Alive, true);
        obj.set(Player::Lvl, 10i32);
        obj.set(Player::Zone, "test");

        println!("{:?}", obj);

        let json = serde_json::to_string(&obj)
            .expect("failed to serialize object");
        println!("{}", json);

        let new_obj = serde_json::from_str::<GameObjectData>(&json)
            .expect("failed to deserialize object");

        println!("{:?}", new_obj);

        assert_eq!(obj.instance.len(), new_obj.instance.len());
        for (attr, val) in obj.instance.values() {
            assert_eq!(Some(val), new_obj.instance.get_param(attr.name()))
        }
    }
}