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
        let euler = Quat::from_xyzw(self.x, self.y, self.z, self.w).to_euler(glam::EulerRot::YXZ);
        Vec3 { x: euler.0 / PI, y: euler.1 / PI, z: euler.2 / PI }
    }
}

fn rad_to_otherland_angle(mut r: f32) -> f32 {
    while r > PI {
        r -= PI * 2.0;
    }

    while r < -PI {
        r += PI * 2.0;
    }

    r / PI
}

fn otherland_angle_to_rad(mut a: f32) -> f32 {
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

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::Vec3;
    use log::debug;

    use crate::{NetworkVec4, network_vec4::{otherland_angle_to_rad, rad_to_otherland_angle}};

    use std::{io, path::Path, env, collections::HashSet};
    use bitstream_io::{ByteWriter, LittleEndian};
    use nom::{number, multi, IResult, error::VerboseError};
    use test_case::test_case;

    use assert_float_eq::*;

    use crate::{param::{AnyClass, ParamClass}, ParamClassContainer, Param, ClassAttrib};

    #[test]
    fn instance_rot_test() -> io::Result<()>{ 
        let client_env = env::var_os("OTHERLAND_CLIENT_PATH").expect("OTHERLAND_CLIENT_PATH not set");
        let client_path = Path::new(&client_env);

        let db = sqlite::open(
            client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();
    
        let result = db
            .prepare(format!("SELECT * FROM {}", "Instance"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());
    
        // dump data
        for row in result {
            let original_data = row.read::<&[u8], _>("data");
            let guid = row.read::<&str,_>("uxInstanceGuid");
            let name: String = row.read::<&str,_>("sEditorName").chars().into_iter().filter(|c| c.is_ascii_graphic()).collect();
            let class_id = row.read::<i64,_>("ixClass") as u16;

            println!("Testing {} - {}", guid.to_string(), name);

            let data = ParamClassContainer::read(class_id, original_data).expect("Parse failed").1.to_anyclass();
            if let Some(rot) = data.get_param("rot") {
                let vector: &Vec3 = rot.try_into().unwrap();

                let nv4: NetworkVec4 = vector.into();
                let v3: Vec3 = nv4.into();

                assert_eq!(vector.clone(), v3);
            }
        }

        Ok(())
    }
}