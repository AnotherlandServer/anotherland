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

use atlas::{ParamBox, PlayerComponent};
use bevy_ecs::{entity::Entity, query::{Added, Changed, With, Without}, system::{Commands, Query, Res}};
use bson::doc;
use log::{debug, error};

use crate::{actors::{zone::{plugins::Item, resources::Tasks}, AvatarComponent, RealmDatabase}, db::{DatabaseRecord, InventoryEntry, InventoryOwner}};

use super::{CreationPending, RemovalPending};

pub fn insert_new_items(
    query: Query<(Entity, &Item, &ParamBox), Added<CreationPending>>,
    characters: Query<&AvatarComponent, With<PlayerComponent>>,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
    mut cmds: Commands,
) {
    for (entity, item, params) in query.iter() {
        if let Ok(owner) = characters.get(item.owner()) {
            let db = db.0.clone();
            let inventory_entry = InventoryEntry {
                id: *item.id(),
                owner: InventoryOwner::Character(owner.record_id.unwrap()),
                params: params.clone(),
                template: *item.template_id(),
            };

            cmds.entity(entity).remove::<CreationPending>();

            let _guard = tasks.handle.enter();
            tasks.tasks.spawn(async move {
                let collection = InventoryEntry::collection(db);
                if let Err(e) = collection.insert_one(inventory_entry, None).await {
                    error!("Database update failed: {:?}", e);
                }
            });
        }
    }
}

pub fn remove_old_items(
    query: Query<(Entity, &Item), Added<RemovalPending>>,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
    mut cmds: Commands,
) {
    for (entity, item) in query.iter() {
        let db = db.0.clone();

        cmds.entity(entity).remove::<CreationPending>();
        let id: bson::Uuid = *item.id();

        let _guard = tasks.handle.enter();
        tasks.tasks.spawn(async move {
            let collection = InventoryEntry::collection(db);
            if let Err(e) = collection.delete_one(doc! {"id": {"$eq": id}}, None).await {
                error!("Database update failed: {:?}", e);
            }
        });
    }
}


pub fn update_item_database(
    query: Query<(&Item, &ParamBox), (With<Item>, Changed<ParamBox>, Without<CreationPending>)>,
    characters: Query<&AvatarComponent, With<PlayerComponent>>,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
) {
    for (item, params) in query.iter() {
        if let Ok(owner) = characters.get(item.owner()) {
            let db = db.0.clone();
            let inventory_entry = InventoryEntry {
                id: *item.id(),
                owner: InventoryOwner::Character(owner.record_id.unwrap()),
                params: params.clone(),
                template: *item.template_id(),
            };

            let _guard = tasks.handle.enter();
            tasks.tasks.spawn(async move {
                debug!("Write item: {}", inventory_entry.id);

                let collection = InventoryEntry::collection(db);
                if let Err(e) = collection.replace_one(doc!("id": {"$eq": inventory_entry.id}), inventory_entry, None).await {
                    error!("Database update failed: {:?}", e);
                }
            });
        }
    }
}