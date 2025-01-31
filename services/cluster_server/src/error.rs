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

use obj_params::ParamError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum ClusterFrontendError {
    #[error(transparent)]
    ClusterError(#[from] cluster::Error),

    #[error(transparent)]
    CoreApi(#[from] core_api::CoreApiError),

    #[error(transparent)]
    RealmApi(#[from] realm_api::RealmApiError),

    #[error(transparent)]
    RakNetError(#[from] raknet::RakNetError),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    ParamError(#[from] ParamError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ClusterFrontendResult<T> = Result<T, ClusterFrontendError>;