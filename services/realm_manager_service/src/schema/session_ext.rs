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

use async_graphql::{Context, Error, Object, SimpleObject};
use core_api::CoreApi;
use database::DatabaseRecord;
use mongodb::{bson::{doc, Uuid}, options::ReturnDocument, Database};

use crate::db;

#[derive(Default)]
pub struct SessionExtRoot;

#[derive(Default)]
pub struct SessionExtMutationRoot;

#[Object]
impl SessionExtRoot {
    async fn session_ext(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<SessionExt>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let core_api = ctx.data::<CoreApi>()?.clone();

        if core_api.get_session(&id).await?.is_some() {
            if let Some(session_ext) = db::SessionExt::collection(&db).find_one(doc! { "id": id }).await? {
                Ok(Some(SessionExt::from_db(session_ext)))
            } else {
                Ok(Some(SessionExt {
                    selected_world: None,
                    selected_character: None,
                }))
            }
        } else {
            Ok(None)
        }
    }
}

#[Object]
impl SessionExtMutationRoot {
    async fn session_ext_select_world(&self, ctx: &Context<'_>, id: Uuid, world_id: i32) -> Result<Option<SessionExt>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let core_api = ctx.data::<CoreApi>()?.clone();

        if core_api.get_session(&id).await?.is_some() {
            let session_ext = db::SessionExt::collection(&db).find_one_and_update(
                doc! { "id": id }, 
                doc! { "selected_world": world_id }
            )
            .return_document(ReturnDocument::After)
            .upsert(true)
            .await?
            .unwrap();

            Ok(Some(SessionExt::from_db(session_ext)))
        } else {
            Ok(None)
        }
    }

    async fn session_ext_select_character(&self, ctx: &Context<'_>, id: Uuid, character_id: i32) -> Result<Option<SessionExt>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let core_api = ctx.data::<CoreApi>()?.clone();

        if core_api.get_session(&id).await?.is_some() {
            let session_ext = db::SessionExt::collection(&db).find_one_and_update(
                doc! { "id": id }, 
                doc! { "selected_character": character_id }
            )
            .return_document(ReturnDocument::After)
            .upsert(true)
            .await?
            .unwrap();

            Ok(Some(SessionExt::from_db(session_ext)))
        } else {
            Ok(None)
        }
    }
}

#[derive(SimpleObject)]
pub struct SessionExt {
    selected_world: Option<u16>,
    selected_character: Option<i32>,
}

impl SessionExt {
    pub fn from_db(session_ext: db::SessionExt) -> Self {
        Self {
            selected_world: session_ext.selected_world,
            selected_character: session_ext.selected_character,
        }
    }
}