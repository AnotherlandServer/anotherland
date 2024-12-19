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

use std::fmt::Debug;

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::{Attribute, Class, GenericParamSet, ParamError, ParamFlag, ParamResult, ParamSet, ParamWriter, Value};

#[derive(Serialize, Deserialize, Component)]
#[serde(transparent)]
pub struct GameObjectData(Box<dyn GenericParamSet>);

impl GameObjectData {
    pub fn new<T: Attribute + 'static>() -> Self {
        Self(Box::new(ParamSet::<T>::new()))
    }

    pub fn new_for_class(class: Class) -> Self {
        Self(class.create_param_set(vec![]))
    }

    pub fn from_set<T: Attribute + 'static>(set: ParamSet<T>) -> Self {
        Self(Box::new(set))
    }

    pub fn from_generic_set(set: Box<dyn GenericParamSet>) -> Self {
        Self(set)
    }

    pub fn class(&self) -> Class { self.0.class() }

    pub fn get<'a, T: Attribute, V>(&'a self, attr: T) -> ParamResult<&'a V>
        where &'a V: TryFrom<&'a Value, Error = ParamError>
    {
        if let Some(value) = self.0.get_param(attr.name()) {
            value.try_into()
        } else if <T as Attribute>::class() == self.0.class() {
            attr.default().try_into()
        } else {
            self.0.class().get_attribute(attr.name())
                .unwrap_or_else(|| panic!("Class {:?} doesn't implement attribute {}", self.0.class(), attr.name()))
                .default()
                .try_into()
        }
    }

    pub fn set<T: Attribute, V: Into<Value>>(&mut self, attr: T, val: V) -> Option<Value> {
        self.0.set_param(attr.name(), val.into())
    }

    pub fn apply(&mut self, mut set: impl GenericParamSet) {
        for (attr, value) in set.drain() {
            self.0.set_param(attr.name(), value);
        }
    }

    pub fn clear_changes(&mut self) {
        self.0.clear_changes();
    }

    pub fn changes(&self) -> Box<dyn GenericParamSet> {
        self.0.changes()
    }

    pub fn merge(&mut self, mut other: GameObjectData) {
        for (attr, value) in other.0.drain() {
            self.0.set_param(attr.name(), value);
        }
    }

    pub fn persistent_value_set(&self) -> Box<dyn GenericParamSet> {
        let values = self.0.values()
            .filter(|&(a,_)| a.has_flag(&ParamFlag::Persistent))
            .map(|(a, v)| (a, v.clone()))
            .collect();

        self.0.class().create_param_set(values)
    }

    pub fn as_set(&self) -> &dyn GenericParamSet {
        self.0.as_ref()
    }
}

impl Clone for GameObjectData {
    fn clone(&self) -> Self {
        Self(self.0.dyn_clone())
    }
}

impl Debug for GameObjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ParamWriter for GameObjectData {
    fn write<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.0.write(writer)
    }

    fn write_to_client<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.0.write_to_client(writer)
    }

    fn write_to_privileged_client<W: bitstream_io::ByteWrite>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.0.write_to_privileged_client(writer)
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

        assert_eq!(obj.0.len(), new_obj.0.len());
        for (attr, val) in obj.0.values() {
            assert_eq!(Some(val), new_obj.0.get_param(attr.name()))
        }
    }
}