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

use std::collections::HashMap;

use atlas::Uuid;
use futures::TryStreamExt;
use log::info;
use tokio::sync::OnceCell;


use crate::util::{AnotherlandError, AnotherlandResult};

use super::{realm_database, DatabaseRecord, ItemContent};

static ITEM_CACHE: OnceCell<HashMap<Uuid, ItemContent>> = OnceCell::const_new();
static ITEM_ID_CACHE: OnceCell<HashMap<i32, Uuid>> = OnceCell::const_new();

pub async fn initialize_item_cache() -> AnotherlandResult<()> {
    ITEM_CACHE.get_or_try_init(|| async move {
        let collection = ItemContent::collection(realm_database().await);
        let mut cursor = collection.find(None, None).await?;
    
        let mut cache = HashMap::new();
        let mut id_cache = HashMap::new();
    
        while let Some(item) = cursor.try_next().await? {
            id_cache.insert(item.id as i32, item.guid);
            cache.insert(item.guid, item);
        }

        let _ = ITEM_ID_CACHE.set(id_cache);

        info!("Cached {} items...", cache.len());

        Ok::<_, AnotherlandError>(cache)
    }).await?;
    
    Ok(())
}

pub fn get_cached_item(id: &Uuid) -> Option<&'static ItemContent> {
    ITEM_CACHE.get()
        .expect("item cache not initialized")
        .get(id)
}

pub fn get_cached_item_by_id(id: i32) -> Option<&'static ItemContent> {
    ITEM_ID_CACHE.get()
        .expect("item cache not initialized")
        .get(&id)
        .and_then(|id| ITEM_CACHE.get().unwrap().get(id))
}