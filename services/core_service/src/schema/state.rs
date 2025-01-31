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

use async_graphql::{Context, Error, Object, SimpleObject};
use database::DatabaseRecord;
use mongodb::Database;

use crate::db;

#[derive(Default)]
pub struct StateRoot;

#[derive(Default)]
pub struct StateMutationRoot;

#[Object]
impl StateRoot {
    async fn state(&self, ctx: &Context<'_>) -> Result<ClusterStatus, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status = db::Status::get(&db, &()).await?;
        Ok(ClusterStatus::from_db(status.unwrap_or_default()))
    }
}

#[Object]
impl StateMutationRoot {
    async fn lock_auth(&self, ctx: &Context<'_>) -> Result<ClusterStatus, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status = db::Status::lock_cluster(&db).await?;
        Ok(ClusterStatus::from_db(status))
    }

    async fn unlock_auth(&self, ctx: &Context<'_>) -> Result<ClusterStatus, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status = db::Status::unlock_cluster(&db).await?;
        Ok(ClusterStatus::from_db(status))
    }
}

#[derive(SimpleObject, Clone, Debug)]
struct ClusterStatus {
    pub cluster_locked: bool,
}

impl ClusterStatus {
    fn from_db(status: db::Status) -> Self {
        Self {
            cluster_locked: status.cluster_locked.unwrap_or_default(),
        }
    }
}