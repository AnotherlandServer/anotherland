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

use crate::{NetworkVec3, NetworkVec4, NetworkQuat};

impl From<NetworkVec3> for Vec3 {
    fn from(val: NetworkVec3) -> Vec3 {
        Vec3 { x: val.y, y: val.z, z: -val.x }
    }
}

impl From<Vec3> for NetworkVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: -value.z, y: value.x, z: value.y }
    }
}

impl From<NetworkVec4> for Vec4 {
    fn from(val: NetworkVec4) -> Vec4 {
        Vec4::new(val.y, val.z, -val.x, val.w)
    }
}

impl From<Vec4> for NetworkVec4 {
    fn from(value: Vec4) -> Self {
        Self {  x: -value.z, y: value.x, z: value.y, w: value.w }
    }
}

impl From<NetworkQuat> for Quat {
    fn from(val: NetworkQuat) -> Quat {
        // For Unreal (X forward, Y right, Z up) to Bevy (Z forward, X right, Y up)
        // We need to remap the rotation axes correctly
        // The quaternion components need to be swapped and negated correctly
        Quat::from_xyzw(val.y, val.z, -val.x, val.w)
    }
}

impl From<Quat> for NetworkQuat {
    fn from(value: Quat) -> Self {
        // Bevy to Unreal quaternion conversion
        Self { x: -value.z, y: value.x, z: value.y, w: value.w }
    }
}