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

use std::{fmt::{Debug, Display}, ops::Deref, sync::Arc, hash::Hash};

#[allow(dead_code)]
struct FNameInner {
    name: String,
    flags: u64,
}

#[derive(Clone)]
pub struct FName(Arc<FNameInner>);

impl FName {
    pub fn new(name: String, flags: u64) -> Self {
        Self(Arc::new(FNameInner {
            name,
            flags
        }))
    }
}

impl Debug for FName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl Display for FName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.name.as_str())
    }
}

impl Deref for FName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0.name
    }
}

impl <'a>From<&'a FName> for &'a str {
    fn from(value: &'a FName) -> Self {
        value.0.name.as_str()
    }
}

impl Hash for FName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl PartialEq for FName {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for FName {}
