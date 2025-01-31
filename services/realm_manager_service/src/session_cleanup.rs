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

use core_api::CoreApi;
use database::DatabaseRecord;
use futures_util::TryStreamExt;
use log::debug;
use mongodb::{bson::doc, Database};

use crate::db::SessionExt;

pub fn start_session_cleanup(db: Database, core_api: CoreApi) {
    tokio::spawn(async move {
        if let Ok(mut cursor) = SessionExt::collection(&db).find(doc!{}).await {
            while let Ok(Some(session_ext)) = cursor.try_next().await {
                if let Ok(None) = core_api.get_session(&session_ext.id).await {
                    debug!("Cleaning up session {}", session_ext.id);
                    let _ = SessionExt::collection(&db).delete_one(doc! { "id": session_ext.id }).await;
                }
            }
        }
    });
}