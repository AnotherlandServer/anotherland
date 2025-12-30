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

use std::fmt::Debug;

use bevy::{app::AppLabel, ecs::label};
use toolkit::types::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InstanceLabel {
    id: Uuid,
    instance: Option<Uuid>,
}

impl InstanceLabel {
    pub fn new(id: Uuid, instance: Option<Uuid>) -> Self {
        Self {
            id,
            instance
        }
    }

    pub fn id(&self) -> Uuid { self.id }
    pub fn instance(&self) -> Option<Uuid> { self.instance }
}

impl Debug for InstanceLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instance) = self.instance {
            f.write_fmt(format_args!("({}, {})", self.id, instance))
        } else {
            f.write_fmt(format_args!("({})", self.id))
        }
    }
}

impl AppLabel for InstanceLabel {
    #[doc = r" Clones this `"]
    #[doc = stringify!(Self)]
    #[doc = r"`."]
    fn dyn_clone(&self) -> label::Box<dyn AppLabel> {
        label::Box::new(self.clone())
    }
}
