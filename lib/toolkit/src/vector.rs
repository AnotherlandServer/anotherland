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
use mlua::{AnyUserData, FromLua, IntoLua, MetaMethod, UserData, UserDataRef, Value};

#[derive(Clone, Copy)]
pub struct Vec3Wrapper(pub Vec3);

impl UserData for Vec3Wrapper {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_get("x", |lua, usr| {
            usr.borrow::<Vec3Wrapper>()?
                .0.x.into_lua(lua)
        });

        fields.add_field_function_get("y", |lua, usr| {
            usr.borrow::<Vec3Wrapper>()?
                .0.y.into_lua(lua)
        });

        fields.add_field_function_get("z", |lua, usr| {
            usr.borrow::<Vec3Wrapper>()?
                .0.z.into_lua(lua)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Add, |_, this, other: Value| {
            if let Some(val) = other.as_number() {
                Ok(Vec3Wrapper(this.0 + val as f32))
            } else if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<Vec3Wrapper>().ok()) {
                Ok(Vec3Wrapper(this.0 + val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Sub, |_, this, other: Value| {
            if let Some(val) = other.as_number() {
                Ok(Vec3Wrapper(this.0 - val as f32))
            } else if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<Vec3Wrapper>().ok()) {
                Ok(Vec3Wrapper(this.0 - val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Mul, |_, this, other: Value| {
            if let Some(val) = other.as_number() {
                Ok(Vec3Wrapper(this.0 * val as f32))
            } else if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<Vec3Wrapper>().ok()) {
                Ok(Vec3Wrapper(this.0 * val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Div, |_, this, other: Value| {
            if let Some(val) = other.as_number() {
                Ok(Vec3Wrapper(this.0 / val as f32))
            } else if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<Vec3Wrapper>().ok()) {
                Ok(Vec3Wrapper(this.0 / val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Eq, |_, this, other: Vec3Wrapper| {
            Ok(this.0 == other.0)
        });

        methods.add_method("Abs", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.abs()))
        });

        methods.add_method("AbsDiffEq", |_, this, (rhs, max_abs_diff): (Vec3Wrapper, f32)| {
            Ok(this.0.abs_diff_eq(rhs.0, max_abs_diff))
        });

        methods.add_method("AngleBetween", |_, this, other: Vec3Wrapper| {
            Ok(this.0.angle_between(other.0))
        });

        methods.add_method("AnyOrthogonalVector", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.any_orthogonal_vector()))
        });

        methods.add_method("AnyOrthonormalPair", |_, this, _: ()| {
            let (a, b) = this.0.any_orthonormal_pair();
            Ok((Vec3Wrapper(a), Vec3Wrapper(b)))
        });

        methods.add_method("AnyOrthonormalVector", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.any_orthonormal_vector()))
        });

        methods.add_method("Ceil", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.ceil()))
        });

        methods.add_method("Clamp", |_, this, (min, max): (Vec3Wrapper, Vec3Wrapper)| {
            Ok(Vec3Wrapper(this.0.clamp(min.0, max.0)))
        });

        methods.add_method("ClampLength", |_, this, (min, max): (f32, f32)| {
            Ok(Vec3Wrapper(this.0.clamp_length(min, max)))
        });

        methods.add_method("ClampLengthMax", |_, this, max: f32| {
            Ok(Vec3Wrapper(this.0.clamp_length_max(max)))
        });

        methods.add_method("ClampLengthMin", |_, this, min: f32| {
            Ok(Vec3Wrapper(this.0.clamp_length_min(min)))
        });

        methods.add_method("Copysign", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.copysign(rhs.0)))
        });

        methods.add_method("Cross", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.cross(rhs.0)))
        });

        methods.add_method("Distance", |_, this, other: Vec3Wrapper| {
            Ok(this.0.distance(other.0))
        });

        methods.add_method("DistanceSquared", |_, this, other: Vec3Wrapper| {
            Ok(this.0.distance_squared(other.0))
        });

        methods.add_method("DivEuclid", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.div_euclid(rhs.0)))
        });

        methods.add_method("Dot", |_, this, rhs: Vec3Wrapper| {
            Ok(this.0.dot(rhs.0))
        });

        methods.add_method("DotIntoVec", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.dot_into_vec(rhs.0)))
        });

        methods.add_method("ElementProduct", |_, this, _: ()| {
            Ok(this.0.element_product())
        });

        methods.add_method("ElementSum", |_, this, _: ()| {
            Ok(this.0.element_sum())
        });

        methods.add_method("Exp", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.exp()))
        });

        methods.add_method("Extend", |_, this, w: f32| {
            Ok(Vec4Wrapper(this.0.extend(w)))
        });

        methods.add_method("Floor", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.floor()))
        });

        methods.add_method("Fract", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.fract()))
        });

        methods.add_method("FractGl", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.fract_gl()))
        });

        methods.add_method("IsFinite", |_, this, _: ()| {
            Ok(this.0.is_finite())
        });

        methods.add_method("IsNan", |_, this, _: ()| {
            Ok(this.0.is_nan())
        });

        methods.add_method("IsNormalized", |_, this, _: ()| {
            Ok(this.0.is_normalized())
        });

        methods.add_method("Length", |_, this, _: ()| {
            Ok(this.0.length())
        });

        methods.add_method("LengthRecip", |_, this, _: ()| {
            Ok(this.0.length_recip())
        });

        methods.add_method("LengthSquared", |_, this, _: ()| {
            Ok(this.0.length_squared())
        });

        methods.add_method("Lerp", |_, this, (rhs, t): (Vec3Wrapper, f32)| {
            Ok(Vec3Wrapper(this.0.lerp(rhs.0, t)))
        });

        methods.add_method("Max", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.max(rhs.0)))
        });

        methods.add_method("MaxElement", |_, this, _: ()| {
            Ok(this.0.max_element())
        });

        methods.add_method("Midpoint", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.midpoint(rhs.0)))
        });

        methods.add_method("Min", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.min(rhs.0)))
        });

        methods.add_method("MinElement", |_, this, _: ()| {
            Ok(this.0.min_element())
        });

        methods.add_method("MoveTowards", |_, this, (target, max_distance): (Vec3Wrapper, f32)| {
            Ok(Vec3Wrapper(this.0.move_towards(target.0, max_distance)))
        });

        methods.add_method("MulAdd", |_, this, (mul, add): (Vec3Wrapper, Vec3Wrapper)| {
            Ok(Vec3Wrapper(this.0.mul_add(mul.0, add.0)))
        });

        methods.add_method("Normalize", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.normalize()))
        });

        methods.add_method("NormalizeOr", |_, this, default: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.normalize_or(default.0)))
        });

        methods.add_method("NormalizeOrZero", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.normalize_or_zero()))
        });

        methods.add_method("Powf", |_, this, n: f32| {
            Ok(Vec3Wrapper(this.0.powf(n)))
        });

        methods.add_method("ProjectOnto", |_, this, other: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.project_onto(other.0)))
        });

        methods.add_method("ProjectOntoNormalized", |_, this, other: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.project_onto_normalized(other.0)))
        });

        methods.add_method("Recip", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.recip()))
        });

        methods.add_method("Reflect", |_, this, normal: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.reflect(normal.0)))
        });

        methods.add_method("Refract", |_, this, (normal, eta): (Vec3Wrapper, f32)| {
            Ok(Vec3Wrapper(this.0.refract(normal.0, eta)))
        });

        methods.add_method("RejectFrom", |_, this, normal: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.reject_from(normal.0)))
        });

        methods.add_method("RejectFromNormalized", |_, this, normal: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.reject_from_normalized(normal.0)))
        });

        methods.add_method("RemEuclid", |_, this, rhs: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.rem_euclid(rhs.0)))
        });

        methods.add_method("Round", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.round()))
        });

        methods.add_method("Signum", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.signum()))
        });

        methods.add_method("Trunc", |_, this, _: ()| {
            Ok(Vec3Wrapper(this.0.trunc()))
        });

        methods.add_method("WithX", |_, this, x: f32| {
            Ok(Vec3Wrapper(this.0.with_x(x)))
        });

        methods.add_method("WithY", |_, this, y: f32| {
            Ok(Vec3Wrapper(this.0.with_y(y)))
        });

        methods.add_method("WithZ", |_, this, z: f32| {
            Ok(Vec3Wrapper(this.0.with_z(z)))
        });
    }
}

impl FromLua for Vec3Wrapper {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        Ok(*UserDataRef::<Vec3Wrapper>::from_lua(value, lua)?)
    }
}

pub struct Vec4Wrapper(pub Vec4);

impl UserData for Vec4Wrapper {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_get("x", |lua, usr| {
            usr.borrow::<Vec4Wrapper>()?
                .0.x.into_lua(lua)
        });

        fields.add_field_function_get("y", |lua, usr| {
            usr.borrow::<Vec4Wrapper>()?
                .0.y.into_lua(lua)
        });

        fields.add_field_function_get("z", |lua, usr| {
            usr.borrow::<Vec4Wrapper>()?
                .0.z.into_lua(lua)
        });

        fields.add_field_function_get("w", |lua, usr| {
            usr.borrow::<Vec4Wrapper>()?
                .0.w.into_lua(lua)
        });
    }
}

impl FromLua for Vec4Wrapper {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("vector expected"))?;
        usr.take::<Vec4Wrapper>()
    }
}

#[derive(Clone, Copy)]
pub struct QuatWrapper(pub Quat);

impl UserData for QuatWrapper {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_get("x", |lua, usr| {
            usr.borrow::<QuatWrapper>()?
                .0.x.into_lua(lua)
        });

        fields.add_field_function_get("y", |lua, usr| {
            usr.borrow::<QuatWrapper>()?
                .0.y.into_lua(lua)
        });

        fields.add_field_function_get("z", |lua, usr| {
            usr.borrow::<QuatWrapper>()?
                .0.z.into_lua(lua)
        });

        fields.add_field_function_get("w", |lua, usr| {
            usr.borrow::<QuatWrapper>()?
                .0.w.into_lua(lua)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Add, |_, this, other: Value| {
            if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<QuatWrapper>().ok()) {
                Ok(QuatWrapper(this.0 + val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Sub, |_, this, other: Value| {
            if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<QuatWrapper>().ok()) {
                Ok(QuatWrapper(this.0 - val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Mul, |_, this, other: Value| {
            if let Some(val) = other.as_userdata().and_then(|v| v.borrow::<QuatWrapper>().ok()) {
                Ok(QuatWrapper(this.0 * val.0))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Div, |_, this, other: Value| {
            if let Some(val) = other.as_number() {
                Ok(QuatWrapper(this.0 / val as f32))
            } else {
                Err(mlua::Error::RuntimeError("unsupported operand type".to_string()))
            }
        });

        methods.add_meta_method(MetaMethod::Eq, |_, this, other: QuatWrapper| {
            Ok(this.0 == other.0)
        });

        methods.add_method("AbsDiffEq", |_, this, (rhs, max_abs_diff): (QuatWrapper, f32)| {
            Ok(this.0.abs_diff_eq(rhs.0, max_abs_diff))
        });

        methods.add_method("AngleBetween", |_, this, other: QuatWrapper| {
            Ok(this.0.angle_between(other.0))
        });

        methods.add_method("Conjugate", |_, this, _: ()| {
            Ok(QuatWrapper(this.0.conjugate()))
        });

        methods.add_method("Dot", |_, this, other: QuatWrapper| {
            Ok(this.0.dot(other.0))
        });

        methods.add_method("Inverse", |_, this, _: ()| {
            Ok(QuatWrapper(this.0.inverse()))
        });

        methods.add_method("IsFinite", |_, this, _: ()| {
            Ok(this.0.is_finite())
        });

        methods.add_method("IsNan", |_, this, _: ()| {
            Ok(this.0.is_nan())
        });

        methods.add_method("IsNearIdentity", |_, this, _: ()| {
            Ok(this.0.is_near_identity())
        });

        methods.add_method("IsNormalized", |_, this, _: ()| {
            Ok(this.0.is_normalized())
        });

        methods.add_method("Length", |_, this, _: ()| {
            Ok(this.0.length())
        });

        methods.add_method("LengthRecip", |_, this, _: ()| {
            Ok(this.0.length_recip())
        });

        methods.add_method("LengthSquared", |_, this, _: ()| {
            Ok(this.0.length_squared())
        });

        methods.add_method("Lerp", |_, this, (rhs, t): (QuatWrapper, f32)| {
            Ok(QuatWrapper(this.0.lerp(rhs.0, t)))
        });

        methods.add_method("MulQuat", |_, this, other: QuatWrapper| {
            Ok(QuatWrapper(this.0.mul_quat(other.0)))
        });

        methods.add_method("MulVec", |_, this, vec: Vec3Wrapper| {
            Ok(Vec3Wrapper(this.0.mul_vec3(vec.0)))
        });

        methods.add_method("Normalize", |_, this, _: ()| {
            Ok(QuatWrapper(this.0.normalize()))
        });

        methods.add_method("RotateTowards", |_, this, (to, max_angle): (QuatWrapper, f32)| {
            Ok(QuatWrapper(this.0.rotate_towards(to.0, max_angle)))
        });

        methods.add_method("Slerp", |_, this, (rhs, t): (QuatWrapper, f32)| {
            Ok(QuatWrapper(this.0.slerp(rhs.0, t)))
        });

    }
}

impl FromLua for QuatWrapper {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        Ok(*UserDataRef::<QuatWrapper>::from_lua(value, lua)?)
    }
}

pub fn init_vector_api(lua: &mlua::Lua) -> mlua::Result<()> {
    let vector = lua.create_table()?;
    let quaternion = lua.create_table()?;

    vector.set("New", lua.create_function(|_, (x, y, z): (f32, f32, f32)| {
        Ok(Vec3Wrapper(Vec3::new(x, y, z)))
    })?)?;

    vector.set("X", Vec3Wrapper(Vec3::X))?;
    vector.set("Y", Vec3Wrapper(Vec3::Y))?;
    vector.set("Z", Vec3Wrapper(Vec3::Z))?;
    vector.set("ZERO", Vec3Wrapper(Vec3::ZERO))?;

    lua.globals().set("Vector", vector)?;

    quaternion.set("FromXYZW", lua.create_function(|_, (x, y, z, w): (f32, f32, f32, f32)| {
        Ok(QuatWrapper(Quat::from_xyzw(x, y, z, w)))
    })?)?;

    quaternion.set("FromEuler", lua.create_function(|_, (x, y, z): (f32, f32, f32)| {
        Ok(QuatWrapper(Quat::from_euler(glam::EulerRot::XYZ, x, y, z)))
    })?)?;

    quaternion.set("FromAxisAngle", lua.create_function(|_, (axis, angle): (Vec3Wrapper, f32)| {
        Ok(QuatWrapper(Quat::from_axis_angle(axis.0, angle)))
    })?)?;

    quaternion.set("FromRotationArc", lua.create_function(|_, (from, to): (Vec3Wrapper, Vec3Wrapper)| {
        Ok(QuatWrapper(Quat::from_rotation_arc(from.0, to.0)))
    })?)?;

    quaternion.set("FromRotationArcColinear", lua.create_function(|_, (from, to): (Vec3Wrapper, Vec3Wrapper)| {
        Ok(QuatWrapper(Quat::from_rotation_arc_colinear(from.0, to.0)))
    })?)?;

    quaternion.set("FromRotationX", lua.create_function(|_, angle: f32| {
        Ok(QuatWrapper(Quat::from_rotation_x(angle)))
    })?)?;

    quaternion.set("FromRotationY", lua.create_function(|_, angle: f32| {
        Ok(QuatWrapper(Quat::from_rotation_y(angle)))
    })?)?;

    quaternion.set("FromRotationZ", lua.create_function(|_, angle: f32| {
        Ok(QuatWrapper(Quat::from_rotation_z(angle)))
    })?)?;

    quaternion.set("FromScaledAxis", lua.create_function(|_, axis: Vec3Wrapper| {
        Ok(QuatWrapper(Quat::from_scaled_axis(axis.0)))
    })?)?;

    quaternion.set("IDENTITY", QuatWrapper(Quat::IDENTITY))?;
    quaternion.set("NAN", QuatWrapper(Quat::NAN))?;

    lua.globals().set("Quaternion", quaternion)?;

    Ok(())
}