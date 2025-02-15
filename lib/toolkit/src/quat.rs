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

use std::f32::consts::PI;

use glam::{Quat, Vec3};

pub trait OtherlandQuatExt {
    fn from_unit_vector(val: Vec3) -> Quat;
    fn as_unit_vector(&self) -> Vec3;
}

impl OtherlandQuatExt for Quat {
    fn from_unit_vector(val: Vec3) -> Quat {
        let a = val.x.atan2(val.y);
        let b = (-val.z).asin();
    
        Quat::from_euler(glam::EulerRot::XYZ, PI / 2.0, 0.0, 0.0)
            .mul_quat(Quat::from_euler(glam::EulerRot::XYZ, a, b, 0.0))
    }

    fn as_unit_vector(&self) -> Vec3 {
        let (a, b, _) = Quat::from_euler(glam::EulerRot::XYZ, -PI / 2.0, 0.0, 0.0)
            .mul_quat(*self)
            .to_euler(glam::EulerRot::XYZ);

        let x = a.cos() * b.cos();
        let y = a.sin() * b.cos();
        let z = b.sin();

        Vec3::new(x, y, -z)
    }
}