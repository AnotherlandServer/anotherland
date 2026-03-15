// Copyright (C) 2026 AnotherlandServer
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

use std::sync::{OnceLock, Weak};

use bevy::{ecs::error::Result, platform::collections::HashMap};
use realm_api::{QuestTemplate, RealmApi};
use tokio::sync::RwLock;

use crate::plugins::{Cache, WeakCache};

pub struct QuestTemplateCache;

impl Cache for QuestTemplateCache {
    type CacheKey = i32;
    type CacheData = QuestTemplate;

    async fn load(key: &Self::CacheKey) -> Result<Option<Self::CacheData>> {
        Ok(
            RealmApi::get()
                .get_quest_template(*key)
                .await?
        )
    }
}

impl WeakCache for QuestTemplateCache {
    fn cache() -> &'static RwLock<HashMap<<Self as Cache>::CacheKey, Weak<<Self as Cache>::CacheData>>> {
        static CACHE: OnceLock<RwLock<HashMap<i32, Weak<QuestTemplate>>>> = OnceLock::new();
        CACHE.get_or_init(|| RwLock::new(HashMap::new()))
    }
}