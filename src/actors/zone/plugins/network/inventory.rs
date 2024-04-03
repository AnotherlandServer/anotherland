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

use atlas::{CPktItemNotify, CPktItemUpdate, ItemBaseComponent, ParamBox, Uuid};
use bevy::utils::hashbrown::{HashMap, HashSet};
use bevy_ecs::{entity::Entity, query::{Added, Changed, With}, removal_detection::RemovedComponents, system::{Query, ResMut, Resource}};
use bitstream_io::{ByteWriter, LittleEndian};

use crate::actors::{zone::plugins::Item, AvatarComponent};

use super::PlayerController;

#[derive(Resource)]
pub struct ItemTracker {
    owner: HashMap<Entity, (Uuid, Entity)>,
}

impl ItemTracker {
    pub fn new() -> Self {
        Self {
            owner: HashMap::new(),
        }
    }
}

pub fn track_added_items(
    query: Query<(Entity, &Item), Added<Item>>,
    mut tracker: ResMut<ItemTracker>,
) {
    for (entity, item) in query.iter() {
        tracker.owner.insert(entity, (*item.id(), item.owner()));
    }
}

pub fn send_item_updates(
    query: Query<(&Item, &ParamBox), (With<ItemBaseComponent>, Changed<ParamBox>)>,
    owner: Query<(&AvatarComponent, &PlayerController)>,
) {
    for (item, params) in query.iter() {
        if let Ok((avatar, controller)) = owner.get(item.owner()) {
            let mut param_buffer = Vec::new();
            let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

            params.write_to_client(&mut writer).expect("failed to write item params");

            controller.send_message(CPktItemUpdate {
                avatar_id: avatar.id.as_u64(),
                id: *item.id(),
                use_template: 1,
                template_id: Some(*item.template_id()),
                class_id: params.class_id().into(),
                params: param_buffer,
                ..Default::default()
            }.into_message());
        }
    }
}

pub fn send_item_removals(
    mut removals: RemovedComponents<Item>,
    owner: Query<(&AvatarComponent, &PlayerController)>,
    mut tracker: ResMut<ItemTracker>,
) {
    for removed in removals.read() {
        if let Some((item_id, item_owner)) = tracker.owner.remove(&removed) &&
            let Ok((avatar, controller)) = owner.get(item_owner)
        {
            controller.send_message(CPktItemNotify {
                avatar_id: avatar.id.as_u64(),
                id: item_id,
                ..Default::default()
            }.into_message());
        }
    }
}