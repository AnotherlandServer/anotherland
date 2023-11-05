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
        Self { x: value.x, y: value.y, z: value.z, w: value.w }
    }
}

impl Into<Vec3> for NetworkVec4 {
    fn into(self) -> Vec3 {
        //let euler = Quat::from_xyzw(self.x, self.y, self.z, self.w).to_euler(glam::EulerRot::XYZ);
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl From<&Vec3> for NetworkVec4 {
    fn from(value: &Vec3) -> Self {
        value.to_owned().into()
    }
}

impl From<Vec3> for NetworkVec4 {
    fn from(value: Vec3) -> Self {
        //let quat = Quat::from_euler(glam::EulerRot::XYZ, value.x, value.y, value.z);
        Self { x: value.x, y: value.y, z: value.z, w: 0.0 }
    }
}

/*#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::NetworkVec4;

    #[test]
    fn vec3_to_quat() {
        let vec = Vec3::new(1.0, 0.0, 0.0);
        let networkv3: NetworkVec4 = vec.into();
        let converted_vec = networkv3.into();

        assert_eq!(vec, converted_vec);
    }
}*/