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

use ::core::hash::Hasher;
use std::fmt::Debug;

use bevy::{app::AppLabel, ecs::label};
use toolkit::types::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct ZoneLabel {
    id: Uuid,
    instance: Uuid,
}

impl ZoneLabel {
    pub fn new(id: Uuid, instance: Uuid) -> Self {
        Self {
            id,
            instance
        }
    }
}

impl Debug for ZoneLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.id, self.instance))
    }
}

impl AppLabel for ZoneLabel {
    #[doc = r" Clones this `"]
    #[doc = stringify!(Self)]
    #[doc = r"`."]
    fn dyn_clone(&self) -> label::Box<dyn AppLabel> {
        label::Box::new(self.clone())
    }

    #[doc = r" Casts this value to a form where it can be compared with other type-erased values."]
    fn as_dyn_eq(&self) ->  &dyn label::DynEq {
        self
    }

    #[doc = r" Feeds this value into the given [`Hasher`]."]
    fn dyn_hash(&self, state: &mut dyn Hasher) {
        state.write(&self.id.bytes());
        state.write(&self.instance.bytes());
    }
}