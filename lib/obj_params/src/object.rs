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

use serde::{ser::{SerializeMap, SerializeStruct}, Deserialize, Serialize};

use crate::{Attribute, AttributeInfo, Class, GenericParamSet, ParamSet, Value};

pub struct GameObjectData {
    class: Class,
    set: Box<dyn GenericParamSet>,
}

impl GameObjectData {
    pub fn new<T: Attribute + 'static>() -> Self {
        Self {
            class: <T as Attribute>::class(),
            set: Box::new(ParamSet::<T>::new())
        }
    }

    pub fn from_set<T: Attribute + 'static>(set: ParamSet<T>) -> Self {
        Self {
            class: <T as Attribute>::class(),
            set: Box::new(set)
        }
    }

    pub fn from_generic_set(set: impl GenericParamSet + 'static) -> Self {
        Self {
            class: set.class(),
            set: Box::new(set)
        }
    }

    pub fn class(&self) -> Class { self.class }

    pub fn get<T: Attribute>(&self, attr: T) -> &Value {
        if let Some(value) = self.set.get_param(attr.name()) {
            value
        } else if <T as Attribute>::class() == self.class {
            attr.default()
        } else {
            self.class.get_attribute(attr.name())
                .unwrap_or_else(|| panic!("Class {:?} doesn't implement attribute {}", self.class, attr.name()))
                .default()
        }
    }

    pub fn set<T: Attribute>(&mut self, attr: T, val: Value) -> Option<Value> {
        self.set.set_param(attr.name(), val)
    }

    pub fn apply(&mut self, mut set: impl GenericParamSet) {
        for (attr, value) in set.drain() {
            self.set.set_param(attr.name(), value);
        }
    }

    pub fn clear_changes(&mut self) {
        self.set.clear_changes();
    }

    pub fn changes(&self) -> Vec<(&'static dyn AttributeInfo, Value)> {
        self.set.changes()
    }
}

impl Serialize for GameObjectData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {

        let mut state = serializer.serialize_struct(self.class.name(), self.set.len())?;

        state.serialize_field("class", self.class.name())?;
        state.serialize_field("attributes", &DynSetSerializer(self.set.as_ref()))?;
        state.end()
    }
}

impl <'de> Deserialize<'de> for GameObjectData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let class = Class::Player;
        class.deserialize(deserializer)
    }
}

struct DynSetSerializer<'a>(&'a dyn GenericParamSet);

impl Serialize for DynSetSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        let mut state = serializer.serialize_map(Some(self.0.len()))?;
        
        for (attr, value) in self.0.values() {
            state.serialize_entry(attr.name(), value)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod test {
    use crate::{GameObjectData, Player, Value};

    #[test]
    fn serialization() {
        let mut obj = GameObjectData::new::<Player>();
        obj.set(Player::Alive, Value::Bool(true));
        obj.set(Player::Lvl, Value::Int(10));
        obj.set(Player::Zone, Value::String("test".to_string()));

        let json = serde_json::to_string(&obj).expect("failed to serialize object");
        println!("{}", json);

        todo!()
    }
}