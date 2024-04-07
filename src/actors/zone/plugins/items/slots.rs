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

use atlas::{ItemBaseParams, Slot, Uuid};
use bevy::utils::{HashMap, HashSet};
use log::debug;

use crate::db::{get_cached_item, get_cached_item_by_id, ItemContent};

pub struct LoadoutBuilder {
    slots: HashMap<Slot, i32>,
}

impl LoadoutBuilder {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new()
        }
    }

    fn get_item_slot(item: &ItemContent) -> Option<Slot> {
        item.data.as_ref()?
            .get_impl::<dyn ItemBaseParams>()?
            .slot_mapping()?
            .parse()
            .ok()
    }

    fn equip_item(&mut self, item: &ItemContent) {
        if let Some(slot) = Self::get_item_slot(item) {
            // check if any of the required slots already contain an item
            let current_slots: Vec<_> = slot
                .slots()
                .iter()
                .filter_map(|slot| self.slots.get(slot))
                .copied()
                .collect();

            // unequip each of the items
            for id in current_slots {
                self.unequip_item(id);
            }

            // equip item
            for slot in slot.slots() {
                self.slots.insert(*slot, item.id as i32);
            }
        } else {
            debug!("No item slot for item {} specified!", item.name);
        }
    }

    fn unequip_item(&mut self, id: i32) {
        if let Some(item) = get_cached_item_by_id(id) {
            if let Some(slot) = Self::get_item_slot(item) {
                for slot in slot.slots() {
                    self.slots.remove(slot);
                }
            }
        }
    }

    pub fn add(&mut self, id: i32) -> &Self {
        if let Some(item) = get_cached_item_by_id(id) {
            self.equip_item(item);
        }

        self
    }

    pub fn add_by_uuid(&mut self, id: Uuid) -> &Self {
        if let Some(item) = get_cached_item(&id) {
            self.equip_item(item);
        }

        self
    }

    pub fn build(self) -> Vec<&'static ItemContent> {
        let items: HashSet<_> = self.slots
            .into_values()
            .collect();
        
        items.into_iter()
            .filter_map(get_cached_item_by_id)
            .collect()
    }
}