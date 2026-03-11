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
use cynic::{http::CynicReqwestError, GraphQlError};
use thiserror::Error;
use toolkit::{GetMongoError, anyhow};

use crate::item_storage_session::ItemStorageSessionError;

#[derive(Error, Debug)]
pub enum RealmError {
    #[error(transparent)]
    Request(#[from] CynicReqwestError),

    #[error("graphql error")]
    GraphQl(Vec<GraphQlError>),

    #[error(transparent)]
    CoreApi(#[from] CoreApiError),

    #[error(transparent)]
    Cluster(#[from] cluster::Error),

    #[error(transparent)]
    Database(#[from] database::DatabaseError),

    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),

    #[error(transparent)]
    BsonError(#[from] mongodb::bson::ser::Error),

    #[error(transparent)]
    Oid(#[from] mongodb::bson::oid::Error),

    #[error(transparent)]
    Param(#[from] obj_params::ParamError),

    #[error(transparent)]
    ContentError(#[from] content::error::Error),

    #[error(transparent)]
    ItemStorageSessionError(#[from] ItemStorageSessionError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl GetMongoError for RealmError {
    fn get_mongo_error(&self) -> Option<&mongodb::error::Error> {
        match self {
            Self::Mongo(e) => Some(e),
            Self::Database(e) => e.get_mongo_error(),
            Self::ItemStorageSessionError(e) => e.get_mongo_error(),
            _ => None,
        }
    }
}

pub type RealmResult<T> = std::result::Result<T, RealmError>;