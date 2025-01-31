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

use glam::{Quat, Vec3, Vec4};

use crate::{NetworkVec3, NetworkVec4};

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

impl From<NetworkVec4> for Vec4 {
    fn from(val: NetworkVec4) -> Vec4 {
        Vec4::new(val.x, val.y, val.z, val.w)
    }
}

impl From<Vec4> for NetworkVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: value.w }
    }
}

impl From<NetworkVec4> for Quat {
    fn from(val: NetworkVec4) -> Quat {
        Quat::from_vec4(Vec4::new(val.x, val.y, val.z, val.w))
    }
}

impl From<Quat> for NetworkVec4 {
    fn from(value: Quat) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: value.w }
    }
}