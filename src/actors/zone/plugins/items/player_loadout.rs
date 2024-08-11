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

use std::ops::{Deref, DerefMut};

use atlas::{ItemBaseParams, Slot, Uuid};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs::{component::Component, entity::Entity};
use log::debug;

use crate::db::{get_cached_item_by_id, ItemContent};

#[derive(Component)]
pub struct Equipped;

#[derive(Component, Clone)]
pub struct PlayerLoadout {
    slots: HashMap<Slot, ItemReference>,
}

#[derive(Component, Clone)]
pub struct PlayerDisguise(pub PlayerLoadout);

impl Deref for PlayerDisguise {
    type Target = PlayerLoadout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerDisguise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum ItemReference {
    VisualOnly(i32),
    InventoryItem((Uuid, i32, Entity)),
}

impl ItemReference {
    fn content(&self) -> Option<&'static ItemContent> {
        get_cached_item_by_id(self.item_id())
    }

    fn item_id(&self) -> i32 {
        match self {
            Self::VisualOnly(id) => *id,
            Self::InventoryItem((_, id, _)) => *id,
        }
    }
}

impl PlayerLoadout {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new()
        }
    }

    fn get_item_slot(item_ref: &ItemReference) -> Option<Slot> {
        item_ref.content()
            .and_then(|c| c.data.as_ref())?
            .get_impl::<dyn ItemBaseParams>()?
            .slot_mapping()
            .parse()
            .ok()
    }

    fn equip_item_internal(&mut self, item_ref: ItemReference) -> Vec<ItemReference> {
        let mut replaced_items = Vec::new();

        if let Some(slot) = Self::get_item_slot(&item_ref) {
            // check if any of the required slots already contain an item
            let current_slots: Vec<_> = slot
                .slots()
                .iter()
                .filter_map(|slot| self.slots.get(slot))
                .cloned()
                .collect();

            // unequip each of the items
            for item_ref in current_slots {
                self.unequip_item_internal(&item_ref);
                replaced_items.push(item_ref);
            }

            // equip item
            for slot in slot.slots() {
                self.slots.insert(*slot, item_ref.clone());
            }
        } else {
            debug!("No item slot for item {} specified!", item_ref.content().unwrap().name);
        }

        replaced_items
    }

    fn unequip_item_internal(&mut self, item_ref: &ItemReference) -> bool {
        let mut removed = false;

        if let Some(slot) = Self::get_item_slot(item_ref) {
            for slot in slot.slots() {
                removed |= self.slots.remove(slot).is_some();
            }
        }

        removed
    }

    pub fn add(&mut self, item: ItemReference) -> Vec<ItemReference> {
        if item.content().is_some() {
            self.equip_item_internal(item)
        } else {
            Vec::new()
        }
    }

    pub fn remove_inventory_item(&mut self, item: Uuid) -> Option<ItemReference> {
        if let Some((_, item_ref)) = self.slots.iter().find(|(_, item_ref)| {
            if let ItemReference::InventoryItem((id, _, _)) = item_ref {
                *id == item
            } else {
                false
            }
        }) {
            let item_ref = item_ref.clone();

            if self.unequip_item_internal(&item_ref) {
                Some(item_ref)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn compile_visual_items(&self) -> Vec<i32> {
        self.slots
            .values()
            .map(ItemReference::item_id)
            .collect()
    }

    pub fn remove_all_items(&mut self) -> Vec<ItemReference> {
        let (visuals, items): (HashMap<_, _>, HashMap<_, _>) = self.slots
            .drain()
            .partition(|(_, item_ref)| matches!(item_ref, ItemReference::InventoryItem((_, _, _))));

        self.slots = visuals;

        items.into_iter()
            .map(|(_, i)| i)
            .collect::<HashSet<_>>()
            .drain()
            .collect()
    }
}
