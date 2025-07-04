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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NavMeshBuilderError {
    #[error("UPK error: {0:?}")]
    UPKError(#[from] upk::UPKError),

    #[error(transparent)]
    RecastnavigationError(#[from] recastnavigation_rs::RNError),

    #[error(transparent)]
    GraphError(#[from] plexus::graph::GraphError),

    #[error(transparent)]
    PlexusBufferError(#[from] plexus::buffer::BufferError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("RealmAPI error: {0}")]
    RealmAPIError(#[from] realm_api::RealmApiError),

    #[error(transparent)]
    PepkgError(#[from] pepkg::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type NavMeshBuilderResult<T> = Result<T, NavMeshBuilderError>;