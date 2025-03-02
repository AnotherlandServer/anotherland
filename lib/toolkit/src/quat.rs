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

use glam::{Quat, Vec3};

pub trait OtherlandQuatExt {
    fn from_unit_vector(val: Vec3) -> Quat;
    fn as_unit_vector(&self) -> Vec3;
}

impl OtherlandQuatExt for Quat {
    fn from_unit_vector(val: Vec3) -> Quat {
        if val == Vec3::ZERO {
            Quat::IDENTITY
        } else {
            Quat::from_rotation_arc(Vec3::Z, val.normalize_or(Vec3::Z))
        }
    }

    fn as_unit_vector(&self) -> Vec3 {
        self.mul_vec3(Vec3::Z)
            .normalize()
    }
}