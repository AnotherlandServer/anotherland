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
        Self { x: value.x, y: value.y, z: value.z, w: value.w }
    }
}

impl Into<Vec3> for NetworkVec4 {
    fn into(self) -> Vec3 {
        let euler = Quat::from_vec4(self.into()).to_euler(glam::EulerRot::XYZ);
        Vec3 { x: euler.0, y: euler.1, z: euler.2 }
    }
}

impl From<Vec3> for NetworkVec4 {
    fn from(value: Vec3) -> Self {
        let quat = Quat::from_euler(glam::EulerRot::XYZ, value.x, value.y, value.z);
        Self { x: quat.x, y: quat.y, z: quat.z, w: quat.w }
    }
}
