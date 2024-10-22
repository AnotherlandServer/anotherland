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

use std::io;

use cynic::http::CynicReqwestError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("raknet error")]
    RakNet(#[from] raknet::RakNetError),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("graphql error")]
    GraphQL(#[from] CynicReqwestError),

    #[error("verification failed")]
    Verification(&'static str),

    #[error("task paniced")]
    JoinError(#[from] JoinError)
}

pub type AppResult<T> = Result<T, AppError>;