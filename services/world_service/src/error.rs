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

use core_api::CoreApiError;
use realm_api::RealmApiError;
use thiserror::Error;
use toolkit::{bson, NativeParamError};

#[derive(Error, Debug)]
pub enum WorldError {
    #[error(transparent)]
    CoreApi(#[from] CoreApiError),

    #[error(transparent)]
    RealmApi(#[from] RealmApiError),

    #[error(transparent)]
    ClusterError(#[from] core_api::ClusterError),

    #[error(transparent)]
    ParamError(#[from] obj_params::ParamError),

    #[error(transparent)]
    ScriptError(#[from] scripting::ScriptError),

    #[error(transparent)]
    UninitializedField(#[from] derive_builder::UninitializedFieldError),

    #[error("unknown zone type `{0}`")]
    UnknownZoneType(String),

    #[error("unknown instance type `{0}`")]
    UnknownInstanceType(i32),

    #[error(transparent)]
    LuaError(#[from] mlua::Error),

    #[error("native param error")]
    NativeParamError(#[from] NativeParamError),

    #[error(transparent)]
    UuidError(#[from] bson::uuid::Error),

    #[error(transparent)]
    RecastnavError(#[from] recastnavigation_rs::RNError),

    #[error(transparent)]
    BsonBinaryError(#[from] bson::binary::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

pub type WorldResult<T> = Result<T, WorldError>;