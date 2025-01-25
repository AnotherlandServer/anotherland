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

use std::{hash::{Hash, Hasher}, str::FromStr};

use crate::{Class, ParamFlag, ParamType, Value};

pub trait Attribute: AttributeInfo + Clone + Copy + PartialEq + Eq + Hash + FromStr + TryFrom<u16> + Send + Sync {
    fn class() -> Class;
    fn static_info(&self) -> &'static dyn AttributeInfo;
}

pub trait AttributeInfo {
    fn class(&self) -> Class;
    fn id(&self) -> u16;
    fn name(&self) -> &'static str;
    fn default(&self) -> &'static Value;
    fn flags(&self) -> &[ParamFlag];
    fn datatype(&self) -> ParamType;
    fn has_flag(&self, flag: &ParamFlag) -> bool {
        self.flags().contains(flag)
    }
}

impl Hash for dyn AttributeInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for dyn AttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn AttributeInfo {}
