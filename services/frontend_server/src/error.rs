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
use tokio::task::JoinError;
use toolkit::anyhow;

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error(transparent)]
    RakNet(#[from] raknet::RakNetError),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    CoreApi(#[from] CoreApiError),

    #[error(transparent)]
    RealmApi(#[from] RealmApiError),

    #[error(transparent)]
    ClusterError(#[from] core_api::ClusterError),

    #[error(transparent)]
    ParamError(#[from] obj_params::ParamError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type FrontendResult<T> = Result<T, FrontendError>;