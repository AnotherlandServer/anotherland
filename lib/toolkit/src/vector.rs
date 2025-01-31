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
use mlua::{FromLua, IntoLua, UserData};

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
        
    }
}

impl FromLua for Vec3Wrapper {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("vector expected"))?;
        usr.take::<Vec3Wrapper>()
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

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        
    }
}

impl FromLua for Vec4Wrapper {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("vector expected"))?;
        usr.take::<Vec4Wrapper>()
    }
}

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
        
    }
}

impl FromLua for QuatWrapper {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("vector expected"))?;
        usr.take::<QuatWrapper>()
    }
}