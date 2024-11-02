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

use std::{collections::{HashMap, HashSet}, fmt::Debug};

use base64::prelude::*;
use glam::{Quat, Vec3, Vec4};
use serde::{de::{self, DeserializeSeed, SeqAccess, Visitor}, ser::{SerializeMap, SerializeStruct}, Deserialize, Serialize};
use toolkit::types::{AvatarId, Uuid};

use crate::{Attribute, AttributeInfo, Class, GenericParamSet, ParamFlag, ParamSet, ParamType, Value};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameObjectData(Box<dyn GenericParamSet>);

impl GameObjectData {
    pub fn new<T: Attribute + 'static>() -> Self {
        Self(Box::new(ParamSet::<T>::new()))
    }

    pub fn from_set<T: Attribute + 'static>(set: ParamSet<T>) -> Self {
        Self(Box::new(set))
    }

    pub fn from_generic_set(set: impl GenericParamSet + 'static) -> Self {
        Self(Box::new(set))
    }

    pub fn class(&self) -> Class { self.0.class() }

    pub fn get<'a, T: Attribute, V>(&'a self, attr: T) -> &'a V 
        where &'a V: From<&'a Value>
    {
        if let Some(value) = self.0.get_param(attr.name()) {
            value.into()
        } else if <T as Attribute>::class() == self.0.class() {
            attr.default().into()
        } else {
            self.0.class().get_attribute(attr.name())
                .unwrap_or_else(|| panic!("Class {:?} doesn't implement attribute {}", self.0.class(), attr.name()))
                .default()
                .into()
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

impl Debug for GameObjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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