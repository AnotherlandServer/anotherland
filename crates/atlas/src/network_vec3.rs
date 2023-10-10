use glam::Vec3;

use crate::NetworkVec3;

impl Into<Vec3> for NetworkVec3 {
    fn into(self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl From<Vec3> for NetworkVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}