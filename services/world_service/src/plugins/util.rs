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

use std::ops::{Deref, DerefMut};

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ForeignResource<T: Send + Sync>(pub T);

impl <T: Send + Sync> Deref for ForeignResource<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl <T: Send + Sync> DerefMut for ForeignResource<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}