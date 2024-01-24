// Copyright (C) 2023 AnotherlandServer
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

use glam::{Vec4, Quat, Vec3};

use crate::NetworkVec4;

impl Into<Vec4> for NetworkVec4 {
    fn into(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}

impl From<Vec4> for NetworkVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: value.w }
    }
}

impl Into<Quat> for NetworkVec4 {
    fn into(self) -> Quat {
        Quat::from_vec4(Vec4::new(self.x, self.y, self.z, self.w))
    }
}

impl From<Quat> for NetworkVec4 {
    fn from(value: Quat) -> Self {
        Self { x: value.x / PI, y: value.y / PI, z: value.z / PI, w: value.w / PI }
    }
}

impl Into<Vec3> for NetworkVec4 {
    fn into(self) -> Vec3 {
        let euler = Quat::from_xyzw(self.x, self.y, self.z, self.w).to_euler(glam::EulerRot::YXZ);
        Vec3 { x: euler.0 / PI, y: euler.1 / PI, z: euler.2 / PI }
    }
}

#[allow(dead_code)]
fn rad_to_otherland_angle(mut r: f32) -> f32 {
    while r > PI {
        r -= PI * 2.0;
    }

    while r < -PI {
        r += PI * 2.0;
    }

    r / PI
}

#[allow(dead_code)]
fn otherland_angle_to_rad(a: f32) -> f32 {
    a * PI
}

impl From<&Vec3> for NetworkVec4 {
    fn from(value: &Vec3) -> Self {
        value.to_owned().into()
    }
}

impl From<Vec3> for NetworkVec4 {
    fn from(value: Vec3) -> Self {
        //  YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
        let quat = Quat::from_euler(glam::EulerRot::YXZ, value.x * PI, value.y * PI, value.z * PI);
        Self { x: quat.x, y: quat.y, z: quat.z, w: quat.w }
    }
}
