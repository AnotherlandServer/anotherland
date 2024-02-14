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

use glam::Vec3;

use crate::NetworkVec3;

impl From<NetworkVec3> for Vec3 {
    fn from(val: NetworkVec3) -> Vec3 {
        Vec3 { x: val.x, y: val.y, z: val.z }
    }
}

impl From<Vec3> for NetworkVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}