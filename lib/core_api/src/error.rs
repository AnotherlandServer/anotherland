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

use cynic::{http::CynicReqwestError, GraphQlError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreApiError {
    #[error("request error")]
    Request(#[from] CynicReqwestError),

    #[error("graphql error")]
    GraphQl(Vec<GraphQlError>),
}

pub type CoreApiResult<T> = std::result::Result<T, CoreApiError>;