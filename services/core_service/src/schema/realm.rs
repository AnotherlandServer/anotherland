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

use std::sync::Arc;

use async_graphql::{Context, Error, InputObject, Object, SimpleObject};
use database::DatabaseRecord;
use futures::TryStreamExt;
use mongodb::Database;

use crate::{db, realm_status_registry::{RealmStatus, RealmStatusRegistry}};

#[derive(Default)]
pub struct RealmRoot;

#[derive(Default)]
pub struct RealmMutationRoot;

#[Object]
impl RealmRoot {
    async fn realms(&self, ctx: &Context<'_>) -> Result<Vec<Realm>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status_registry = ctx.data::<Arc<RealmStatusRegistry>>()?;
        let mut cursor = db::Realm::list(&db).await?;
        let mut res = vec![];

        while let Some(realm) = cursor.try_next().await? {
            let status = status_registry.status(&realm.id).await;
            res.push(Realm::from_db(realm, status));
        }

        Ok(res)
    }

    async fn realm(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Realm>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status_registry = ctx.data::<Arc<RealmStatusRegistry>>()?;
        let status = status_registry.status(&id).await;
        Ok(db::Realm::get(&db, &id).await?.map(|realm| Realm::from_db(realm, status)))
    }
}

#[Object]
impl RealmMutationRoot {
    async fn create_realm(&self, ctx: &Context<'_>, input: RealmCreationInput) -> Result<Realm, Error> {
        let db = ctx.data::<Database>()?.clone();
        let realm = db::Realm::create(&db, db::Realm {
            id: input.id,
            name: input.name,
        }).await?;
        
        Ok(Realm::from_db(realm, None))
    }

    async fn delete_realm(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Realm>, Error> {
        let db = ctx.data::<Database>()?.clone();

        if let Some(realm) = db::Realm::get(&db, &id).await? {
            realm.delete(&db).await?;

            Ok(Some(Realm::from_db(realm, None)))
        } else {
            Ok(None)
        }
    }

    async fn update_realm(&self, ctx: &Context<'_>, id: i32, update: RealmUpdateInput) -> Result<Option<Realm>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let status_registry = ctx.data::<Arc<RealmStatusRegistry>>()?;

        if let Some(mut realm) = db::Realm::get(&db, &id).await? {
            if let Some(name) = update.name {
                realm.name = name;
            }

            if let Some(endpoint) = update.endpoint {
                status_registry.register_endpoint(id, endpoint.parse()?).await;
            }

            if let Some(population) = update.population {
                status_registry.update_population(&id, population).await;
            }

            Ok(Some(Realm::from_db(realm, status_registry.status(&id).await)))
        } else {
            Ok(None)
        }
    }
}

#[derive(InputObject)]
struct RealmCreationInput {
    pub id: i32,
    pub name: String,
}

#[derive(InputObject)]
struct RealmUpdateInput {
    pub name: Option<String>,
    pub population: Option<f32>,
    pub endpoint: Option<String>,
}

#[derive(SimpleObject, Clone, Debug)]
struct Realm {
    pub id: i32,
    pub name: String,
    pub population: Option<f32>,
    pub endpoint: Option<String>,
}

impl Realm {
    fn from_db(realm: db::Realm, status: Option<RealmStatus>) -> Self {
        if let Some(RealmStatus { endpoints, population }) = status {
            Self {
                id: realm.id,
                name: realm.name.clone(),
                population: Some(population),
                endpoint: endpoints.first().map(|addr| addr.to_string()),
            }
        } else {
            Self {
                id: realm.id,
                name: realm.name.clone(),
                population: None,
                endpoint: None,
            }
        }
    }
}
