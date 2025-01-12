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

use std::{collections::HashMap, sync::{Arc, Weak}};

use futures_util::TryStreamExt;
use obj_params::{Class, GameObjectData};
use realm_api::{Category, ObjectTemplate, RealmApi};
use tokio::sync::Mutex;
use toolkit::types::Uuid;

use crate::error::WorldResult;

struct Inner {
    realm_api: RealmApi,
    uuid_access: HashMap<Uuid, Weak<CacheEntry>>,
    id_access: HashMap<i32, Weak<CacheEntry>>,
    name_access: HashMap<String, Weak<CacheEntry>>,
}

pub struct CacheEntry {
    pub id: Uuid,
    pub numeric_id: i32,
    pub category: Category,
    pub name: String,
    pub class: Class,
    pub data: Arc<GameObjectData>,
}

impl From<ObjectTemplate> for CacheEntry {
    fn from(item: ObjectTemplate) -> Self {
        Self {
            id: item.id,
            numeric_id: item.numeric_id,
            category: item.category,
            name: item.name,
            class: item.class,
            data: Arc::new(item.data),
        }
    }
}

#[derive(Clone)]
pub struct ObjectCache(Arc<Mutex<Inner>>);

impl ObjectCache {
    pub fn new(realm_api: RealmApi) -> Self {
        Self(Arc::new(Mutex::new(Inner {
            realm_api,
            uuid_access: HashMap::new(),
            id_access: HashMap::new(),
            name_access: HashMap::new(),
        })))
    }

    pub async fn get_object_by_id(&self, id: i32) -> WorldResult<Option<Arc<CacheEntry>>> {
        let mut v = self.0.lock().await;

        if let Some(item) = v.id_access.get(&id)
            .and_then(|weak| weak.upgrade()) {
            Ok(Some(item.clone()))
        } else if 
            let Some(item) = v.realm_api.query_object_templates()
                .numeric_id(id)
                .query().await?
                .try_next().await?
                .map(|item| Arc::<CacheEntry>::new(item.into()))
        {
            v.id_access.insert(item.numeric_id, Arc::downgrade(&item));
            v.uuid_access.insert(item.id, Arc::downgrade(&item));
            v.name_access.insert(item.name.clone(), Arc::downgrade(&item));

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub async fn get_object_by_guid(&self, guid: Uuid) -> WorldResult<Option<Arc<CacheEntry>>> {
        let mut v = self.0.lock().await;

        if let Some(item) = v.uuid_access.get(&guid)
            .and_then(|weak| weak.upgrade()) {
            Ok(Some(item.clone()))
        } else if 
            let Some(item) = v.realm_api.get_object_template(guid).await?
                .map(|item| Arc::<CacheEntry>::new(item.into()))
        {
            v.id_access.insert(item.numeric_id, Arc::downgrade(&item));
            v.uuid_access.insert(item.id, Arc::downgrade(&item));

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub async fn get_object_by_name(&self, name: &str) -> WorldResult<Option<Arc<CacheEntry>>> {
        let mut v = self.0.lock().await;

        if let Some(item) = v.name_access.get(name)
            .and_then(|weak| weak.upgrade()) {
            Ok(Some(item.clone()))
        } else if 
            let Some(item) = v.realm_api.query_object_templates()
                .name(name.to_string())
                .query().await?
                .try_next().await?
                .map(|item| Arc::<CacheEntry>::new(item.into()))
        {
            v.id_access.insert(item.numeric_id, Arc::downgrade(&item));
            v.uuid_access.insert(item.id, Arc::downgrade(&item));

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}