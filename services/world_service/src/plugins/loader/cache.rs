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

use std::{hash::Hash, sync::{Arc, Weak}};

use bevy::{ecs::error::Result, platform::collections::HashMap};
use tokio::sync::{RwLock, RwLockWriteGuard};

pub trait Cache {
    type CacheKey: Hash + Eq + Clone + 'static;
    type CacheData: Sized + 'static;

    async fn load(key: &Self::CacheKey) -> Result<Option<Self::CacheData>>;
    async fn post_load(_data: &Arc<Self::CacheData>) -> Result<()> {
        Ok(())
    }
}

pub trait WeakCache: Cache {
    fn cache() -> &'static RwLock<HashMap<<Self as Cache>::CacheKey, Weak<<Self as Cache>::CacheData>>>;

    async fn get(key: &<Self as Cache>::CacheKey) -> Result<Option<Arc<<Self as Cache>::CacheData>>> {
        let cache = Self::cache();

        // Read from cache
        {
            let read_guard = cache.read().await;
            if 
                let Some(weak_data) = read_guard.get(key) &&
                let Some(strong_data) = weak_data.upgrade() 
            {
                return Ok(Some(strong_data));
            }
        }

        let mut write_guard = cache.write().await;

        // Double check cache in case it was populated while waiting for write lock
        if 
            let Some(weak_data) = write_guard.get(key) &&
            let Some(strong_data) = weak_data.upgrade() 
        {
            return Ok(Some(strong_data));
        }

        // Load data and insert into cache
        if let Some(data) = Self::load(key).await? {
            let arc_data = Arc::new(data);
            Self::cache_insert(&mut write_guard, key.clone(), &arc_data).await;

            drop(write_guard);
            Self::post_load(&arc_data).await?;

            Ok(Some(arc_data))
        } else {
            Ok(None)
        }
    }

    async fn cache_insert(
        cache: &mut RwLockWriteGuard<'_, HashMap<<Self as Cache>::CacheKey, Weak<<Self as Cache>::CacheData>>>, 
        key: <Self as Cache>::CacheKey, 
        data: &Arc<<Self as Cache>::CacheData>
    ) {
        cache.insert(key, Arc::downgrade(data));
    }
}

pub trait StrongCache: Cache {
    fn cache() -> &'static RwLock<HashMap<<Self as Cache>::CacheKey, Arc<<Self as Cache>::CacheData>>>;

    async fn get(key: &<Self as Cache>::CacheKey) -> Result<Option<Arc<<Self as Cache>::CacheData>>> {
        let cache = Self::cache();

        // Read from cache
        {
            let read_guard = cache.read().await;
            if 
                let Some(data) = read_guard.get(key)
            {
                return Ok(Some(data.clone()));
            }
        }

        let mut write_guard = cache.write().await;

        // Double check cache in case it was populated while waiting for write lock
        if 
            let Some(data) = write_guard.get(key)
        {
            return Ok(Some(data.clone()));
        }

        // Load data and insert into cache
        if let Some(data) = Self::load(key).await? {
            let arc_data = Arc::new(data);
            Self::cache_insert(&mut write_guard, key.clone(), &arc_data).await;

            drop(write_guard);
            Self::post_load(&arc_data).await?;

            Ok(Some(arc_data))
        } else {
            Ok(None)
        }
    }

    async fn cache_insert(
        cache: &mut RwLockWriteGuard<'_, HashMap<<Self as Cache>::CacheKey, Arc<<Self as Cache>::CacheData>>>, 
        key: <Self as Cache>::CacheKey, 
        data: &Arc<<Self as Cache>::CacheData>
    ) {
        cache.insert(key, data.clone());
    }
}
