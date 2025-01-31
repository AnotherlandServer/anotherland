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

use async_graphql::scalar;
use mongodb::bson;
use obj_params::Class;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClassWrapper(Class);

impl From<Class> for ClassWrapper {
    fn from(value: Class) -> Self {
        Self(value)
    }
}

impl From<ClassWrapper> for Class {
    fn from(value: ClassWrapper) -> Self {
        value.0
    }
}

impl From<&ClassWrapper> for mongodb::bson::Bson {
    fn from(value: &ClassWrapper) -> Self {
        bson::to_bson(value).unwrap()
    }
}

scalar!(ClassWrapper, "Class");