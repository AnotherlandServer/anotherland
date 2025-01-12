// Copyright (C) 2024 AnotherlandServer
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

use std::convert::Infallible;

use thiserror::Error;
use toolkit::anyhow;

#[derive(Error, Debug)]
pub enum ParamError {
    #[error("unknown attribute name")]
    UnknownAttributeName,

    #[error("unknown attribute id")]
    UnknownAttributeId,

    #[error("unknown class")]
    UnknownClass,

    #[error("type mismatch")]
    TypeMismatch,

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    Infallible(#[from] Infallible),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ParamResult<T> = Result<T, ParamError>;