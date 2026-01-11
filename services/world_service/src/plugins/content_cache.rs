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

use std::sync::{Arc, OnceLock, Weak};

use bevy::{ecs::error::Result, platform::collections::HashMap};
use futures::TryStreamExt;
use obj_params::ContentRef;
use realm_api::{ObjectTemplate, RealmApi};
use tokio::sync::{RwLock, RwLockWriteGuard};
use toolkit::types::Uuid;

use crate::plugins::{Cache, WeakCache};

pub struct ContentCache;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ContentCacheRef {
    ContentRef(ContentRef),
    Uuid(Uuid),
    Id(i32),
    Name(String),
}

impl Cache for ContentCache {
    type CacheKey = ContentCacheRef;
    type CacheData = ObjectTemplate;

    async fn load(key: &Self::CacheKey) -> Result<Option<Self::CacheData>> {
        match key {
            ContentCacheRef::ContentRef(ContentRef { id, .. }) => {
                Ok(
                    RealmApi::get()
                        .get_object_template(*id)
                        .await?
                )
            },
            ContentCacheRef::Uuid(uuid) => {
                Ok(
                    RealmApi::get()
                        .get_object_template(*uuid)
                        .await?
                )
            },
            ContentCacheRef::Id(numeric_id) => {
                Ok(
                    RealmApi::get()
                        .query_object_templates()
                        .numeric_id(*numeric_id)
                        .query()
                        .await?
                        .try_next()
                        .await?
                )
            },
            ContentCacheRef::Name(name) => {
                Ok(
                    RealmApi::get()
                        .query_object_templates()
                        .name(name.to_string())
                        .query()
                        .await?
                        .try_next()
                        .await?
                )
            }
        }
    }
}

impl WeakCache for ContentCache {
    fn cache() -> &'static RwLock<HashMap<<Self as super::Cache>::CacheKey, Weak<<Self as super::Cache>::CacheData>>> {
        static CACHE: OnceLock<RwLock<HashMap<ContentCacheRef, Weak<ObjectTemplate>>>> = OnceLock::new();
        CACHE.get_or_init(|| RwLock::new(HashMap::new()))
    }

    async fn cache_insert(
            cache: &mut RwLockWriteGuard<'_, HashMap<<Self as super::Cache>::CacheKey, Weak<<Self as super::Cache>::CacheData>>>, 
            _key: <Self as super::Cache>::CacheKey, 
            data: &Arc<<Self as super::Cache>::CacheData>
        ) {
        
        cache.insert(
            ContentCacheRef::ContentRef(ContentRef { 
                class: data.class, 
                id: data.id 
            }), 
            Arc::downgrade(data)
        );

        cache.insert(
            ContentCacheRef::Uuid(data.id), 
            Arc::downgrade(data)
        );

        cache.insert(
            ContentCacheRef::Id(data.numeric_id), 
            Arc::downgrade(data)
        );

        cache.insert(
            ContentCacheRef::Name(data.name.clone()), 
            Arc::downgrade(data)
        );
    }
}